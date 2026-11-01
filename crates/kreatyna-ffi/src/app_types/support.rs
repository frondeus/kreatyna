use kreatyna::{ingredients::IngredientId, recipes::RecipeId};

use crate::AppFfiError;
use krucyfiks::CommandScopeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Record)]
pub struct ScopeId {
    pub value: u64,
}

impl From<CommandScopeId> for ScopeId {
    fn from(value: CommandScopeId) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl From<ScopeId> for CommandScopeId {
    fn from(value: ScopeId) -> Self {
        value.value.into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error, uniffi::Error)]
pub enum AppTypeConversionError {
    #[error("invalid ingredient ID: {value}")]
    InvalidIngredientId { value: String },
    #[error("invalid recipe ID: {value}")]
    InvalidRecipeId { value: String },
    #[error("integer cannot be represented on this platform")]
    IntegerOutOfRange,
}

pub(super) fn index(value: u64) -> Result<usize, AppTypeConversionError> {
    value
        .try_into()
        .map_err(|_| AppTypeConversionError::IntegerOutOfRange)
}

pub(super) fn count(value: usize) -> Result<u64, AppTypeConversionError> {
    value
        .try_into()
        .map_err(|_| AppTypeConversionError::IntegerOutOfRange)
}

pub(super) fn ingredient_id(value: String) -> Result<IngredientId, AppTypeConversionError> {
    value
        .parse()
        .map_err(|_| AppTypeConversionError::InvalidIngredientId { value })
}

// A recipe id crosses the FFI as its UUID text. Swift sees `RecipeId`, a
// type alias of `String`.
uniffi::custom_type!(RecipeId, String, {
    remote,
    lower: |id| id.to_string(),
    try_lift: |value| lift_recipe_id(value),
});

/// Swift's id text as a `RecipeId`. Invalid text fails the call that carries
/// it, for example `dispatch`, with `AppFfiError::Conversion`.
pub(crate) fn lift_recipe_id(value: String) -> uniffi::Result<RecipeId> {
    value.parse().map_err(|_| {
        uniffi::deps::anyhow::Error::new(AppFfiError::from(
            AppTypeConversionError::InvalidRecipeId { value },
        ))
    })
}
