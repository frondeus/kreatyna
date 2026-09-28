use std::path::PathBuf;

use futures::StreamExt;
use kreatyna::{
    CLIENT_CAPABILITY,
    capabilities::{CounterDatabase, CounterDatabaseClient, DatabaseError},
    ingredients::capabilities::{IngredientLoadError, IngredientStore, IngredientStoreClient},
};
use tokio::sync::oneshot;

use super::{DatabaseEffect, DatabaseWorker, RUNTIME, SqliteDatabase};

#[test]
fn close_and_wait_finishes_while_clients_are_retained() {
    RUNTIME.block_on(async {
        let database = TemporaryDatabase::new();
        let (mut worker, counter, ingredients) = start_worker(database.path()).await;

        worker.request_close();
        worker.wait().await.expect("database worker should finish");

        assert!(matches!(
            counter.count().await,
            Err(DatabaseError::Transport)
        ));
        assert!(matches!(
            ingredients.list().await,
            Err(IngredientLoadError::Transport)
        ));
    });
}

#[test]
fn admitted_requests_from_both_capabilities_finish_after_close() {
    RUNTIME.block_on(async {
        let database = TemporaryDatabase::new();
        let mut database_connection = SqliteDatabase::new(database.path())
            .await
            .expect("database should open");
        let (counter, counter_rx) = CounterDatabaseClient::new(CLIENT_CAPABILITY);
        let (ingredients, ingredient_rx) = IngredientStoreClient::new(CLIENT_CAPABILITY);
        let close_handles = vec![counter_rx.close_handle(), ingredient_rx.close_handle()];
        let (admitted_tx, admitted_rx) = oneshot::channel();
        let (release_tx, release_rx) = oneshot::channel();

        let task = RUNTIME.spawn(async move {
            let mut inbox = Box::pin(futures::stream::select(
                counter_rx.into_stream().map(DatabaseEffect::Counter),
                ingredient_rx.into_stream().map(DatabaseEffect::Ingredient),
            ));
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

        let count = RUNTIME.spawn(async move { counter.count().await });
        let list = RUNTIME.spawn(async move { ingredients.list().await });
        admitted_rx
            .await
            .expect("worker should observe both admitted requests");

        worker.request_close();
        release_tx.send(()).expect("worker should still be waiting");

        assert_eq!(
            count.await.expect("count task should finish").unwrap(),
            None
        );
        assert!(
            list.await
                .expect("list task should finish")
                .unwrap()
                .is_empty()
        );
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
        let (counter, counter_rx) = CounterDatabaseClient::new(CLIENT_CAPABILITY);
        let close_handles = vec![counter_rx.close_handle()];
        let (finished_tx, finished_rx) = oneshot::channel();
        let task = RUNTIME.spawn(async move {
            database_connection
                .worker(counter_rx.into_stream().map(DatabaseEffect::Counter))
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
            counter.count().await,
            Err(DatabaseError::Transport)
        ));
    });
}

async fn start_worker(
    path: &str,
) -> (DatabaseWorker, CounterDatabaseClient, IngredientStoreClient) {
    let mut database = SqliteDatabase::new(path)
        .await
        .expect("database should open");
    let (counter, counter_rx) = CounterDatabaseClient::new(CLIENT_CAPABILITY);
    let (ingredients, ingredient_rx) = IngredientStoreClient::new(CLIENT_CAPABILITY);
    let worker = DatabaseWorker {
        close_handles: vec![counter_rx.close_handle(), ingredient_rx.close_handle()],
        task: RUNTIME.spawn(async move {
            let inbox = futures::stream::select(
                counter_rx.into_stream().map(DatabaseEffect::Counter),
                ingredient_rx.into_stream().map(DatabaseEffect::Ingredient),
            );
            database.worker(inbox).await;
        }),
    };
    (worker, counter, ingredients)
}

struct TemporaryDatabase(PathBuf);

impl TemporaryDatabase {
    fn new() -> Self {
        Self(std::env::temp_dir().join(format!(
            "database-worker-{}.sqlite",
            kreatyna::ingredients::IngredientId::new()
        )))
    }

    fn path(&self) -> &str {
        self.0.to_str().expect("temporary path should be UTF-8")
    }
}

impl Drop for TemporaryDatabase {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}
