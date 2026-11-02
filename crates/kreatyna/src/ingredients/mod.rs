//! Ingredient catalog domain types shared by future widgets and persistence.

pub mod capabilities;
mod catalog;
mod detail;
mod draft;
mod form;
mod list;
mod model;
mod navigation;
mod split_catalog;

pub use draft::{
    DraftMode, IngredientDraft, IngredientDraftField, IngredientWrite, IngredientWriteData,
    ReferenceUnitDraft, ValidationField, ValidationIssue, ValidationReason,
};

pub use catalog::{
    IngredientCatalog, IngredientCatalogEvent, IngredientCatalogOutcome,
    IngredientCatalogScreenView, IngredientCatalogView,
};

pub use list::{
    IngredientList, IngredientListEvent, IngredientListOutcome, IngredientListRefresh,
    IngredientListRow, IngredientListStatus, IngredientListView,
};

pub use model::{
    Grams, Ingredient, IngredientId, IngredientIdParseError, Kilocalories, Millilitres, Nutrition,
    NutritionBasis, ReferenceUnit, name_key,
};

pub use detail::{
    IngredientDetail, IngredientDetailDeletion, IngredientDetailEvent, IngredientDetailOutcome,
    IngredientDetailView,
};

pub use form::{
    IngredientForm, IngredientFormEvent, IngredientFormOutcome, IngredientFormStatus,
    IngredientFormView,
};

pub use split_catalog::{
    IngredientSplitCatalog, IngredientSplitCatalogEvent, IngredientSplitCatalogOutcome,
    IngredientSplitCatalogView, IngredientSplitPaneOutcome, IngredientSplitPaneView,
};

pub use navigation::{
    IngredientEditOrigin, IngredientNavigation, IngredientNavigationAction,
    IngredientNavigationError,
};
