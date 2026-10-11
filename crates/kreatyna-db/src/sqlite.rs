use diesel::SqliteConnection;
use diesel_async::{
    AsyncConnection, SimpleAsyncConnection, sync_connection_wrapper::SyncConnectionWrapper,
};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use futures::{Stream, StreamExt, pin_mut};
use kreatyna::ingredients::capabilities::{IngredientStoreRequest, ResolveIngredientStore};
use krucyfiks::effects::Envelope;

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
    Ingredient(Envelope<IngredientStoreRequest>),
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
                DatabaseEffect::Ingredient(envelope) => {
                    if let Err(error) = envelope.resolve_mut(self).await {
                        tracing::error!(?error, "Sending ingredient database response");
                    }
                }
            }
        }
    }
}
