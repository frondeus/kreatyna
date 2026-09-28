use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, SqliteConnection};
use diesel_async::{AsyncConnection, sync_connection_wrapper::SyncConnectionWrapper};
use diesel_async::{RunQueryDsl, SimpleAsyncConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use futures::{Stream, StreamExt, pin_mut};
use kreatyna::capabilities::{
    CounterDatabaseMut, CounterDatabaseRequest, CounterDatabaseResponse, DatabaseError,
    ResolveCounterDatabase,
};
use kreatyna::ingredients::capabilities::{
    IngredientStoreRequest, IngredientStoreResponse, ResolveIngredientStore,
};
use krucyfiks::effects::Envelope;

use super::schema::counter;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub struct SqliteDatabase {
    pub(super) connection: SyncConnectionWrapper<SqliteConnection>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum SqliteStartupError {
    #[error("could not open the database")]
    Open,
    #[error("could not configure the database")]
    Configure,
    #[error("could not migrate the database")]
    Migrate,
}

pub enum DatabaseEffect {
    Counter(Envelope<CounterDatabaseRequest, CounterDatabaseResponse>),
    Ingredient(Envelope<IngredientStoreRequest, IngredientStoreResponse>),
}

#[async_trait::async_trait]
impl CounterDatabaseMut for SqliteDatabase {
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
    pub async fn new(path: &str) -> Result<Self, SqliteStartupError> {
        let mut connection = SyncConnectionWrapper::<SqliteConnection>::establish(path)
            .await
            .map_err(|error| {
                tracing::error!(?error, "Could not open database");
                SqliteStartupError::Open
            })?;

        connection
            .batch_execute(
                "PRAGMA busy_timeout = 5000;
                PRAGMA foreign_keys = ON;",
            )
            .await
            .map_err(|error| {
                tracing::error!(?error, "Could not configure database");
                SqliteStartupError::Configure
            })?;

        let migration_result = connection
            .spawn_blocking(|connection| {
                Ok(connection.run_pending_migrations(MIGRATIONS).map(|_| ()))
            })
            .await
            .map_err(|error| {
                tracing::error!(?error, "Could not run database migration task");
                SqliteStartupError::Migrate
            })?;

        migration_result.map_err(|error| {
            tracing::error!(?error, "Could not migrate database");
            SqliteStartupError::Migrate
        })?;
        Ok(Self { connection })
    }

    pub async fn worker(&mut self, requests: impl Stream<Item = DatabaseEffect>) {
        pin_mut!(requests);

        while let Some(request) = requests.next().await {
            match request {
                DatabaseEffect::Counter(envelope) => {
                    if let Err(error) = envelope.resolve_mut(self).await {
                        tracing::error!(?error, "Sending counter database response");
                    }
                }
                DatabaseEffect::Ingredient(envelope) => {
                    if let Err(error) = envelope.resolve_mut(self).await {
                        tracing::error!(?error, "Sending ingredient database response");
                    }
                }
            }
        }
    }
}
