use kreatyna::ingredients::IngredientId;
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
