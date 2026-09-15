use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, SqliteConnection};
use diesel_async::{AsyncConnection, sync_connection_wrapper::SyncConnectionWrapper};
use diesel_async::{RunQueryDsl, SimpleAsyncConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use flume::Receiver;
use futures::StreamExt;
use mausritter::capabilities::DatabaseError;

use super::channel::{DatabaseEnvelope, DatabaseRequest, DatabaseResponse};
use super::schema::counter;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub struct SqliteDatabase {
    connection: SyncConnectionWrapper<SqliteConnection>,
}

impl SqliteDatabase {
    async fn count(&mut self) -> Result<Option<usize>, DatabaseError> {
        let value = counter::table
            .find(1_i32)
            .select(counter::value)
            .first::<i64>(&mut self.connection)
            .await
            .optional()
            .map_err(|_| DatabaseError::Storage)?;

        value
            .map(|value| usize::try_from(value).map_err(|_| DatabaseError::OutOfRange))
            .transpose()
    }

    async fn set_count(&mut self, count: usize) -> Result<(), DatabaseError> {
        let value = i64::try_from(count).map_err(|_| DatabaseError::OutOfRange)?;

        diesel::insert_into(counter::table)
            .values((counter::id.eq(1_i32), counter::value.eq(value)))
            .on_conflict(counter::id)
            .do_update()
            .set(counter::value.eq(value))
            .execute(&mut self.connection)
            .await
            .map_err(|_| DatabaseError::Storage)?;
        Ok(())
    }
}

impl SqliteDatabase {
    pub async fn new(path: &str) -> anyhow::Result<Self> {
        let mut connection = SyncConnectionWrapper::<SqliteConnection>::establish(path).await?;

        connection
            .batch_execute(
                "PRAGMA busy_timeout = 5000;
                PRAGMA foreign_keys = ON;",
            )
            .await?;

        let migration_result = connection
            .spawn_blocking(|connection| {
                Ok(connection.run_pending_migrations(MIGRATIONS).map(|_| ()))
            })
            .await?;

        migration_result.map_err(|e| anyhow::anyhow!("MigrationError: {e:?}"))?;
        Ok(Self { connection })
    }

    pub async fn worker(&mut self, rx: Receiver<DatabaseEnvelope>) {
        let mut rx = rx.into_stream();
        while let Some(envelope) = rx.next().await {
            match envelope.request {
                DatabaseRequest::Count => {
                    let res = self.count().await;
                    _ = envelope
                        .response
                        .send_async(DatabaseResponse::Count(res))
                        .await;
                }
                DatabaseRequest::SetCount { count } => {
                    let res = self.set_count(count).await;
                    _ = envelope
                        .response
                        .send_async(DatabaseResponse::SetCount(res))
                        .await;
                }
            }
        }
    }
}
