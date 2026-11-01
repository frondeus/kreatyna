use kreatyna as core;
use kreatyna_db::sqlite::SqliteStartupError;

use super::{
    AppTypeConversionError, IngredientCatalogEvent, IngredientCatalogView,
    IngredientSplitCatalogEvent, IngredientSplitCatalogView, RecipeSectionEvent, RecipeSectionView,
};

/// How the platform shows the app. Swift passes it once, to `app` (E-19).
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

/// A sidebar section of the split layout. Swift writes the names, the icons,
/// and the order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum AppSection {
    Recipes,
    Ingredients,
}

impl From<AppSection> for core::AppSection {
    fn from(value: AppSection) -> Self {
        match value {
            AppSection::Recipes => Self::Recipes,
            AppSection::Ingredients => Self::Ingredients,
        }
    }
}

impl From<core::AppSection> for AppSection {
    fn from(value: core::AppSection) -> Self {
        match value {
            core::AppSection::Recipes => Self::Recipes,
            core::AppSection::Ingredients => Self::Ingredients,
        }
    }
}

/// Each layout has its own events, as in Rust.
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum AppEvent {
    Stack { event: IngredientCatalogEvent },
    Split { event: SplitAppEvent },
}

impl TryFrom<AppEvent> for core::AppEvent {
    type Error = AppTypeConversionError;

    fn try_from(value: AppEvent) -> Result<Self, Self::Error> {
        Ok(match value {
            AppEvent::Stack { event } => Self::Stack(event.try_into()?),
            AppEvent::Split { event } => Self::Split(event.try_into()?),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum SplitAppEvent {
    SelectSection { section: AppSection },
    Recipes { event: RecipeSectionEvent },
    Ingredients { event: IngredientSplitCatalogEvent },
}

impl TryFrom<SplitAppEvent> for core::SplitAppEvent {
    type Error = AppTypeConversionError;

    fn try_from(value: SplitAppEvent) -> Result<Self, Self::Error> {
        Ok(match value {
            SplitAppEvent::SelectSection { section } => Self::SelectSection(section.into()),
            SplitAppEvent::Recipes { event } => Self::Recipes(event.into()),
            SplitAppEvent::Ingredients { event } => Self::Ingredients(event.try_into()?),
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
    InitializationFailed {
        error: AppInitializationError,
    },
    Running {
        app: RunningAppView,
    },
    /// The Rust core stopped on a bug (a panic) and refuses every event.
    /// Only quitting helps (`vibetyna-dm7`).
    Crashed,
}

/// The running app in its layout, the same shape as Rust's `AppView`.
#[allow(clippy::large_enum_variant)] // one per app
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum RunningAppView {
    /// iOS: the ingredient catalog on a stack. No sections (E-04).
    Stack { catalog: IngredientCatalogView },
    /// Mac: a sidebar with the Recipes and Ingredients sections.
    Split { app: SplitAppView },
}

impl TryFrom<core::AppView> for RunningAppView {
    type Error = AppTypeConversionError;

    fn try_from(value: core::AppView) -> Result<Self, Self::Error> {
        Ok(match value {
            core::AppView::Stack(catalog) => Self::Stack {
                catalog: catalog.try_into()?,
            },
            core::AppView::Split(app) => Self::Split {
                app: app.try_into()?,
            },
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct SplitAppView {
    /// The selected section. Both sections always exist.
    pub section: AppSection,
    pub recipes: RecipeSectionView,
    pub ingredients: IngredientSplitCatalogView,
}

impl TryFrom<core::SplitAppView> for SplitAppView {
    type Error = AppTypeConversionError;

    fn try_from(value: core::SplitAppView) -> Result<Self, Self::Error> {
        Ok(Self {
            section: value.section.into(),
            recipes: value.recipes.into(),
            ingredients: value.ingredients.try_into()?,
        })
    }
}
