//! Ingredient catalog domain types shared by future widgets and persistence.

mod draft;
mod model;

pub use draft::{
    DraftMode, IngredientDraft, IngredientWrite, IngredientWriteData, ReferenceUnitDraft,
    ValidationField, ValidationIssue, ValidationReason,
};

pub use model::{
    Grams, Ingredient, IngredientId, Kilocalories, Millilitres, Nutrition, NutritionBasis,
    ReferenceUnit, name_key,
};
