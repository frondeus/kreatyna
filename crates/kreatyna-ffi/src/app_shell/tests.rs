use std::{path::PathBuf, sync::Arc};

use anyhow::Context;
use futures::{StreamExt, stream::BoxStream};
use kreatyna::{
    CLIENT_CAPABILITY,
    ingredients::capabilities::{IngredientLoadError, IngredientStore, IngredientStoreClient},
};
use krucyfiks::{Message, Widget};
use tokio::sync::oneshot;

use super::{
    AppSetup, AppShell, AppShellEvent, AppShellOutcome, DatabaseEffect, DatabaseHandle,
    DatabaseWorker, RUNTIME, SqliteDatabase,
};
use crate::{AppInitializationError, AppView};

// The shell and the database worker spawn their tasks on the app's own
// `RUNTIME`, so these tests run on it too instead of on `#[tokio::test]`.

// Green path

#[test]
fn close_and_wait_finishes_while_clients_are_retained() -> anyhow::Result<()> {
    RUNTIME.block_on(async {
        let database = TemporaryDatabase::new()?;
        let (mut worker, ingredients) = start_worker(database.path()?).await?;

        worker.request_close();
        worker.wait().await?;

        assert!(matches!(
            ingredients.list().await,
            Err(IngredientLoadError::Transport)
        ));
        Ok(())
    })
}

#[test]
fn admitted_requests_finish_after_close() -> anyhow::Result<()> {
    RUNTIME.block_on(async {
        let database = TemporaryDatabase::new()?;
        let mut database_connection = SqliteDatabase::new(database.path()?).await?;
        let (ingredients, ingredient_rx) = IngredientStoreClient::new(CLIENT_CAPABILITY);
        let close_handles = vec![ingredient_rx.close_handle()];
        let (admitted_tx, admitted_rx) = oneshot::channel();
        let (release_tx, release_rx) = oneshot::channel();

        // On a failure the task returns early. That drops its channel ends,
        // and the test reports the step that did not happen.
        let task = RUNTIME.spawn(async move {
            let mut inbox = Box::pin(ingredient_rx.into_stream().map(DatabaseEffect::Ingredient));
            let mut admitted = Vec::with_capacity(2);
            for _ in 0..2 {
                let Some(request) = inbox.next().await else {
                    return;
                };
                admitted.push(request);
            }
            if admitted_tx.send(()).is_err() || release_rx.await.is_err() {
                return;
            }
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
            .context("the worker did not admit both requests")?;

        worker.request_close();
        release_tx
            .send(())
            .map_err(|()| anyhow::anyhow!("the worker stopped waiting for the release"))?;

        let (first, second) = lists.await?;
        assert_eq!(first, Ok(vec![]));
        assert_eq!(second, Ok(vec![]));
        worker.wait().await?;
        Ok(())
    })
}

#[test]
fn drop_requests_close_without_aborting_the_worker() -> anyhow::Result<()> {
    RUNTIME.block_on(async {
        let database = TemporaryDatabase::new()?;
        let mut database_connection = SqliteDatabase::new(database.path()?).await?;
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
            .context("the detached worker did not drain and finish")?;
        assert!(matches!(
            ingredients.list().await,
            Err(IngredientLoadError::Transport)
        ));
        Ok(())
    })
}

// Validation

#[test]
fn a_database_stop_after_shutdown_begins_is_ignored() -> anyhow::Result<()> {
    RUNTIME.block_on(async {
        let database = TemporaryDatabase::new()?;
        let handle = DatabaseHandle::default();
        let mut shell = AppShell::new(AppSetup {
            database_path: database.path()?.into(),
            database: handle.clone(),
            render: Arc::new(NoRender),
            layout: kreatyna::AppLayout::Split,
        });
        let mut running = start(&mut shell).await?;
        drop(shell.begin_shutdown());

        handle.close();
        // The watch was stopped: its stream ends without a report.
        while let Some(outcome) = running.next().await {
            assert!(!matches!(outcome, AppShellOutcome::DatabaseStopped(_)));
        }
        assert!(matches!(shell.view(), Ok(AppView::Running { .. })));
        Ok(())
    })
}

// Red cases

#[test]
fn a_stopped_database_shows_the_failure_and_retry_starts_it_again() -> anyhow::Result<()> {
    RUNTIME.block_on(async {
        let database = TemporaryDatabase::new()?;
        let handle = DatabaseHandle::default();
        let mut shell = AppShell::new(AppSetup {
            database_path: database.path()?.into(),
            database: handle.clone(),
            render: Arc::new(NoRender),
            layout: kreatyna::AppLayout::Split,
        });
        let mut running = start(&mut shell).await?;
        assert!(matches!(shell.view(), Ok(AppView::Running { .. })));

        handle.close();
        let stopped = loop {
            let outcome = running
                .next()
                .await
                .context("the watch ended without reporting the stop")?;
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
        let finished = first_initialization(retry.into_stream()).await?;
        drop(shell.update(Message::Outcome(finished)));
        assert!(matches!(shell.view(), Ok(AppView::Running { .. })));
        Ok(())
    })
}

// Test support

struct NoRender;

#[async_trait::async_trait]
impl kreatyna::capabilities::Render for NoRender {
    async fn render(&self) {}
}

/// A database file in its own temporary folder. The folder is deleted on drop.
struct TemporaryDatabase {
    _folder: tempfile::TempDir,
    path: PathBuf,
}

impl TemporaryDatabase {
    fn new() -> anyhow::Result<Self> {
        let folder = tempfile::tempdir()?;
        let path = folder.path().join("app.sqlite");
        Ok(Self {
            _folder: folder,
            path,
        })
    }

    fn path(&self) -> anyhow::Result<&str> {
        self.path
            .to_str()
            .context("the temporary path is not UTF-8")
    }
}

/// Initializes the shell and returns the running app's command, unapplied.
async fn start(shell: &mut AppShell) -> anyhow::Result<BoxStream<'static, AppShellOutcome>> {
    let finished = first_initialization(shell.init().into_stream()).await?;
    Ok(shell.update(Message::Outcome(finished)).into_stream())
}

async fn first_initialization(
    mut outcomes: BoxStream<'static, AppShellOutcome>,
) -> anyhow::Result<AppShellOutcome> {
    loop {
        let outcome = outcomes
            .next()
            .await
            .context("initialization ended without finishing")?;
        if let AppShellOutcome::InitializationFinished(_) = outcome {
            return Ok(outcome);
        }
    }
}

async fn start_worker(path: &str) -> anyhow::Result<(DatabaseWorker, IngredientStoreClient)> {
    let mut database = SqliteDatabase::new(path).await?;
    let (ingredients, ingredient_rx) = IngredientStoreClient::new(CLIENT_CAPABILITY);
    let worker = DatabaseWorker {
        close_handles: vec![ingredient_rx.close_handle()],
        task: RUNTIME.spawn(async move {
            database
                .worker(ingredient_rx.into_stream().map(DatabaseEffect::Ingredient))
                .await;
        }),
    };
    Ok((worker, ingredients))
}
