use std::path::PathBuf;

use futures::StreamExt;
use kreatyna::{
    CLIENT_CAPABILITY,
    ingredients::capabilities::{IngredientLoadError, IngredientStore, IngredientStoreClient},
};
use tokio::sync::oneshot;

use std::sync::Arc;

use futures::stream::BoxStream;
use krucyfiks::{Message, Widget};

use super::{
    AppSetup, AppShell, AppShellEvent, AppShellOutcome, AppShellState, DatabaseEffect,
    DatabaseWorker, RUNTIME, SqliteDatabase,
};
use crate::{AppInitializationError, AppView};

#[test]
fn a_stopped_database_shows_the_failure_and_retry_starts_it_again() {
    RUNTIME.block_on(async {
        let database = TemporaryDatabase::new();
        let mut shell = AppShell::new(AppSetup {
            database_path: database.path().into(),
            render: Arc::new(NoRender),
            layout: kreatyna::AppLayout::Split,
        });
        let mut running = start(&mut shell).await;
        assert!(matches!(shell.view(), Ok(AppView::Running { .. })));

        let AppShellState::Running(prepared) = &shell.state else {
            panic!("expected a running app")
        };
        prepared._database_worker.request_close();
        let stopped = loop {
            let outcome = running.next().await.expect("the watch reports the stop");
            if let AppShellOutcome::DatabaseStopped(_) = outcome {
                break outcome;
            }
        };
        drop(shell.update(Message::Outcome(stopped)));

        assert!(matches!(
            shell.view(),
            Ok(AppView::InitializationFailed {
                error: AppInitializationError::DatabaseStopped
            })
        ));

        let retry = shell.update(Message::Event(AppShellEvent::RetryInitialization));
        let finished = first_initialization(retry.into_stream()).await;
        drop(shell.update(Message::Outcome(finished)));
        assert!(matches!(shell.view(), Ok(AppView::Running { .. })));
    });
}

#[test]
fn a_database_stop_after_shutdown_begins_is_ignored() {
    RUNTIME.block_on(async {
        let database = TemporaryDatabase::new();
        let mut shell = AppShell::new(AppSetup {
            database_path: database.path().into(),
            render: Arc::new(NoRender),
            layout: kreatyna::AppLayout::Split,
        });
        let mut running = start(&mut shell).await;
        drop(shell.begin_shutdown());

        let AppShellState::Running(prepared) = &shell.state else {
            panic!("expected a running app")
        };
        prepared._database_worker.request_close();
        // The watch was stopped: its stream ends without a report.
        while let Some(outcome) = running.next().await {
            assert!(!matches!(outcome, AppShellOutcome::DatabaseStopped(_)));
        }
        assert!(matches!(shell.view(), Ok(AppView::Running { .. })));
    });
}

/// Initializes the shell and returns the running app's command, unapplied.
async fn start(shell: &mut AppShell) -> BoxStream<'static, AppShellOutcome> {
    let finished = first_initialization(shell.init().into_stream()).await;
    shell.update(Message::Outcome(finished)).into_stream()
}

async fn first_initialization(
    mut outcomes: BoxStream<'static, AppShellOutcome>,
) -> AppShellOutcome {
    loop {
        let outcome = outcomes.next().await.expect("initialization finishes");
        if let AppShellOutcome::InitializationFinished(_) = outcome {
            return outcome;
        }
    }
}

struct NoRender;

#[async_trait::async_trait]
impl kreatyna::capabilities::Render for NoRender {
    async fn render(&self) {}
}

#[test]
fn close_and_wait_finishes_while_clients_are_retained() {
    RUNTIME.block_on(async {
        let database = TemporaryDatabase::new();
        let (mut worker, ingredients) = start_worker(database.path()).await;

        worker.request_close();
        worker.wait().await.expect("database worker should finish");

        assert!(matches!(
            ingredients.list().await,
            Err(IngredientLoadError::Transport)
        ));
    });
}

#[test]
fn admitted_requests_finish_after_close() {
    RUNTIME.block_on(async {
        let database = TemporaryDatabase::new();
        let mut database_connection = SqliteDatabase::new(database.path())
            .await
            .expect("database should open");
        let (ingredients, ingredient_rx) = IngredientStoreClient::new(CLIENT_CAPABILITY);
        let close_handles = vec![ingredient_rx.close_handle()];
        let (admitted_tx, admitted_rx) = oneshot::channel();
        let (release_tx, release_rx) = oneshot::channel();

        let task = RUNTIME.spawn(async move {
            let mut inbox = Box::pin(ingredient_rx.into_stream().map(DatabaseEffect::Ingredient));
            let mut admitted = Vec::with_capacity(2);
            for _ in 0..2 {
                admitted.push(inbox.next().await.expect("request should be admitted"));
            }
            admitted_tx.send(()).expect("test should still be waiting");
            release_rx.await.expect("test should release the worker");
            database_connection
                .worker(futures::stream::iter(admitted).chain(inbox))
                .await;
        });
        let mut worker = DatabaseWorker {
            task,
            close_handles,
        };

        let lists =
            RUNTIME.spawn(async move { futures::join!(ingredients.list(), ingredients.list()) });
        admitted_rx
            .await
            .expect("worker should observe both admitted requests");

        worker.request_close();
        release_tx.send(()).expect("worker should still be waiting");

        let (first, second) = lists.await.expect("list task should finish");
        assert!(first.unwrap().is_empty());
        assert!(second.unwrap().is_empty());
        worker.wait().await.expect("database worker should finish");
    });
}

#[test]
fn drop_requests_close_without_aborting_the_worker() {
    RUNTIME.block_on(async {
        let database = TemporaryDatabase::new();
        let mut database_connection = SqliteDatabase::new(database.path())
            .await
            .expect("database should open");
        let (ingredients, ingredient_rx) = IngredientStoreClient::new(CLIENT_CAPABILITY);
        let close_handles = vec![ingredient_rx.close_handle()];
        let (finished_tx, finished_rx) = oneshot::channel();
        let task = RUNTIME.spawn(async move {
            database_connection
                .worker(ingredient_rx.into_stream().map(DatabaseEffect::Ingredient))
                .await;
            let _ = finished_tx.send(());
        });
        let worker = DatabaseWorker {
            task,
            close_handles,
        };

        drop(worker);

        finished_rx
            .await
            .expect("detached worker should drain and finish");
        assert!(matches!(
            ingredients.list().await,
            Err(IngredientLoadError::Transport)
        ));
    });
}

async fn start_worker(path: &str) -> (DatabaseWorker, IngredientStoreClient) {
    let mut database = SqliteDatabase::new(path)
        .await
        .expect("database should open");
    let (ingredients, ingredient_rx) = IngredientStoreClient::new(CLIENT_CAPABILITY);
    let worker = DatabaseWorker {
        close_handles: vec![ingredient_rx.close_handle()],
        task: RUNTIME.spawn(async move {
            database
                .worker(ingredient_rx.into_stream().map(DatabaseEffect::Ingredient))
                .await;
        }),
    };
    (worker, ingredients)
}

/// A database file in its own temporary folder. The folder is deleted on drop.
struct TemporaryDatabase {
    _folder: tempfile::TempDir,
    path: PathBuf,
}

impl TemporaryDatabase {
    fn new() -> Self {
        let folder = tempfile::tempdir().unwrap();
        let path = folder.path().join("app.sqlite");
        Self {
            _folder: folder,
            path,
        }
    }

    fn path(&self) -> &str {
        self.path.to_str().unwrap()
    }
}
