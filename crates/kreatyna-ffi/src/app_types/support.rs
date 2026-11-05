use kreatyna::{
    ingredients::IngredientId,
    recipes::{IngredientLineId, RecipeId, StepId},
};

use crate::AppFfiError;
use krucyfiks::CommandScopeId;

/// Whether the New button opens a create form now.
#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum CreateAvailability {
    Available,
    Unavailable,
}

impl From<kreatyna::CreateAvailability> for CreateAvailability {
    fn from(value: kreatyna::CreateAvailability) -> Self {
        match value {
            kreatyna::CreateAvailability::Available => Self::Available,
            kreatyna::CreateAvailability::Unavailable => Self::Unavailable,
        }
    }
}

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
    #[error("invalid step ID: {value}")]
    InvalidStepId { value: String },
    #[error("invalid ingredient line ID: {value}")]
    InvalidIngredientLineId { value: String },
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

/// Swift's id text as the id `T`. Invalid text fails the call that carries
/// it, for example `dispatch`, with `AppFfiError::Conversion` and the error
/// that `invalid` makes from the text.
fn lift_id<T: std::str::FromStr>(
    value: String,
    invalid: fn(String) -> AppTypeConversionError,
) -> uniffi::Result<T> {
    value
        .parse()
        .map_err(|_| uniffi::deps::anyhow::Error::new(AppFfiError::from(invalid(value))))
}

// An id crosses the FFI as its UUID text. `uniffi.toml` maps each one to a
// Swift struct of the same name, so Swift cannot mix the two ids up.
uniffi::custom_type!(IngredientId, String, {
    remote,
    lower: |id| id.to_string(),
    try_lift: |value| lift_ingredient_id(value),
});

/// Swift's id text as an `IngredientId`. Invalid text fails the call that
/// carries it, for example `dispatch`, with `AppFfiError::Conversion`.
pub(crate) fn lift_ingredient_id(value: String) -> uniffi::Result<IngredientId> {
    lift_id(value, |value| AppTypeConversionError::InvalidIngredientId {
        value,
    })
}

uniffi::custom_type!(RecipeId, String, {
    remote,
    lower: |id| id.to_string(),
    try_lift: |value| lift_recipe_id(value),
});

/// Swift's id text as a `RecipeId`. Invalid text fails the call that carries
/// it, for example `dispatch`, with `AppFfiError::Conversion`.
pub(crate) fn lift_recipe_id(value: String) -> uniffi::Result<RecipeId> {
    lift_id(value, |value| AppTypeConversionError::InvalidRecipeId {
        value,
    })
}

uniffi::custom_type!(StepId, String, {
    remote,
    lower: |id| id.to_string(),
    try_lift: |value| lift_step_id(value),
});

/// Swift's id text as a `StepId`. Invalid text fails the call that carries
/// it, for example `dispatch`, with `AppFfiError::Conversion`.
pub(crate) fn lift_step_id(value: String) -> uniffi::Result<StepId> {
    lift_id(value, |value| AppTypeConversionError::InvalidStepId {
        value,
    })
}

uniffi::custom_type!(IngredientLineId, String, {
    remote,
    lower: |id| id.to_string(),
    try_lift: |value| lift_ingredient_line_id(value),
});

/// Swift's id text as an `IngredientLineId`. Invalid text fails the call
/// that carries it, for example `dispatch`, with `AppFfiError::Conversion`.
pub(crate) fn lift_ingredient_line_id(value: String) -> uniffi::Result<IngredientLineId> {
    lift_id(value, |value| {
        AppTypeConversionError::InvalidIngredientLineId { value }
    })
}
