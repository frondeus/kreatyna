use kreatyna as core;
use kreatyna_db::sqlite::SqliteStartupError;

use super::{
    AppTypeConversionError, IngredientCatalogEvent, IngredientCatalogView,
    IngredientSplitCatalogEvent, IngredientSplitCatalogView,
};

/// How the platform shows the catalog. Swift passes it once, to `app` (E-19).
#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum AppLayout {
    Stack,
    Split,
}

impl From<AppLayout> for core::AppLayout {
    fn from(value: AppLayout) -> Self {
        match value {
            AppLayout::Stack => Self::Stack,
            AppLayout::Split => Self::Split,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum AppEvent {
    Ingredients { event: IngredientCatalogEvent },
    SplitIngredients { event: IngredientSplitCatalogEvent },
}

impl TryFrom<AppEvent> for core::AppEvent {
    type Error = AppTypeConversionError;

    fn try_from(value: AppEvent) -> Result<Self, Self::Error> {
        Ok(match value {
            AppEvent::Ingredients { event } => Self::Ingredients(event.try_into()?),
            AppEvent::SplitIngredients { event } => Self::SplitIngredients(event.try_into()?),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum AppInitializationError {
    DatabaseOpen,
    DatabaseConfiguration,
    DatabaseMigration,
    /// The database task ended while the app was running. Retry starts the
    /// database again (P06).
    DatabaseStopped,
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
    pub ingredients: IngredientsView,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum IngredientsView {
    Stack { catalog: IngredientCatalogView },
    Split { catalog: IngredientSplitCatalogView },
}

impl TryFrom<core::IngredientsView> for IngredientsView {
    type Error = AppTypeConversionError;

    fn try_from(value: core::IngredientsView) -> Result<Self, Self::Error> {
        Ok(match value {
            core::IngredientsView::Stack(catalog) => Self::Stack {
                catalog: catalog.try_into()?,
            },
            core::IngredientsView::Split(catalog) => Self::Split {
                catalog: catalog.try_into()?,
            },
        })
    }
}

impl TryFrom<core::AppView> for RunningAppView {
    type Error = AppTypeConversionError;

    fn try_from(value: core::AppView) -> Result<Self, Self::Error> {
        Ok(Self {
            ingredients: value.ingredients.try_into()?,
        })
    }
}
