use std::path::PathBuf;

use futures::StreamExt;
use kreatyna::{
    CLIENT_CAPABILITY,
    ingredients::capabilities::{IngredientLoadError, IngredientStore, IngredientStoreClient},
};
use tokio::sync::oneshot;

use super::{DatabaseEffect, DatabaseWorker, RUNTIME, SqliteDatabase};

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
