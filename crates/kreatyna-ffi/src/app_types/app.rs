use kreatyna as core;
use kreatyna_db::sqlite::SqliteStartupError;

use super::{AppTypeConversionError, IngredientCatalogEvent, IngredientCatalogView};

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum AppEvent {
    Ingredients { event: IngredientCatalogEvent },
}

impl TryFrom<AppEvent> for core::AppEvent {
    type Error = AppTypeConversionError;

    fn try_from(value: AppEvent) -> Result<Self, Self::Error> {
        Ok(match value {
            AppEvent::Ingredients { event } => Self::Ingredients(event.try_into()?),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum AppInitializationError {
    DatabaseOpen,
    DatabaseConfiguration,
    DatabaseMigration,
}

impl From<SqliteStartupError> for AppInitializationError {
    fn from(value: SqliteStartupError) -> Self {
        match value {
            SqliteStartupError::Open => Self::DatabaseOpen,
            SqliteStartupError::Configure => Self::DatabaseConfiguration,
            SqliteStartupError::Migrate => Self::DatabaseMigration,
        }
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum AppView {
    Initializing,
    InitializationFailed { error: AppInitializationError },
    Running { app: RunningAppView },
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct RunningAppView {
    pub ingredients: IngredientCatalogView,
}

impl TryFrom<core::AppView> for RunningAppView {
    type Error = AppTypeConversionError;

    fn try_from(value: core::AppView) -> Result<Self, Self::Error> {
        Ok(Self {
            ingredients: value.ingredients.try_into()?,
        })
    }
}
