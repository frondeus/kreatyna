//! Ingredient catalog domain types shared by future widgets and persistence.

mod model;

pub use model::{
    Grams, Ingredient, IngredientId, Kilocalories, Millilitres, Nutrition, NutritionBasis,
    ReferenceUnit, name_key,
};
