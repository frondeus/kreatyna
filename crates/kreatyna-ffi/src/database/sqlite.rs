use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, SqliteConnection};
use diesel_async::{AsyncConnection, sync_connection_wrapper::SyncConnectionWrapper};
use diesel_async::{RunQueryDsl, SimpleAsyncConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use futures::StreamExt;
use kreatyna::capabilities::{
    DatabaseError, DatabaseMut, DatabaseRequest, DatabaseResponse, ResolveDatabase,
};
use krucyfiks::effects::RequestInbox;

use super::schema::counter;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub struct SqliteDatabase {
    pub(super) connection: SyncConnectionWrapper<SqliteConnection>,
}

#[async_trait::async_trait]
impl DatabaseMut for SqliteDatabase {
    #[tracing::instrument(skip_all)]
    async fn count(&mut self) -> Result<Option<usize>, DatabaseError> {
        let value = counter::table
            .find(1_i32)
            .select(counter::value)
            .first::<i64>(&mut self.connection)
            .await
            .optional()
            .map_err(|error| {
                tracing::error!(?error, "Storage error");
                DatabaseError::Storage
            })?;

        value
            .map(|value| usize::try_from(value).map_err(|_| DatabaseError::OutOfRange))
            .transpose()
    }

    #[tracing::instrument(skip(self))]
    async fn set_count(&mut self, count: usize) -> Result<(), DatabaseError> {
        let value = i64::try_from(count).map_err(|error| {
            tracing::error!(?error, "Out of range");
            DatabaseError::OutOfRange
        })?;

        diesel::insert_into(counter::table)
            .values((counter::id.eq(1_i32), counter::value.eq(value)))
            .on_conflict(counter::id)
            .do_update()
            .set(counter::value.eq(value))
            .execute(&mut self.connection)
            .await
            .map_err(|error| {
                tracing::error!(?error, "Storage error");
                DatabaseError::Storage
            })?;
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

    pub async fn worker(&mut self, rx: RequestInbox<DatabaseRequest, DatabaseResponse>) {
        let mut requests = rx.into_stream();

        while let Some(envelope) = requests.next().await {
            if let Err(error) = envelope.resolve_mut(self).await {
                tracing::error!(?error, "Sending database response");
            }
        }
    }
}
