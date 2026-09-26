use kreatyna::{confirmation as core_confirmation, ingredients as core};

use super::{Ingredient, IngredientDeleteError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum ConfirmationEvent {
    Confirm,
    Cancel,
}

impl From<ConfirmationEvent> for core_confirmation::ConfirmationEvent {
    fn from(value: ConfirmationEvent) -> Self {
        match value {
            ConfirmationEvent::Confirm => Self::Confirm,
            ConfirmationEvent::Cancel => Self::Cancel,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum ConfirmationOutcome {
    Confirmed,
    Cancelled,
}

impl From<core_confirmation::ConfirmationOutcome> for ConfirmationOutcome {
    fn from(value: core_confirmation::ConfirmationOutcome) -> Self {
        match value {
            core_confirmation::ConfirmationOutcome::Confirmed => Self::Confirmed,
            core_confirmation::ConfirmationOutcome::Cancelled => Self::Cancelled,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum ConfirmationView {
    AwaitingDecision,
    Resolved { decision: ConfirmationOutcome },
}

impl From<core_confirmation::ConfirmationView> for ConfirmationView {
    fn from(value: core_confirmation::ConfirmationView) -> Self {
        match value {
            core_confirmation::ConfirmationView::AwaitingDecision => Self::AwaitingDecision,
            core_confirmation::ConfirmationView::Resolved { decision } => Self::Resolved {
                decision: decision.into(),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum IngredientDetailEvent {
    Edit,
    RequestDelete,
    Confirmation { event: ConfirmationEvent },
}

impl From<IngredientDetailEvent> for core::IngredientDetailEvent {
    fn from(value: IngredientDetailEvent) -> Self {
        match value {
            IngredientDetailEvent::Edit => Self::Edit,
            IngredientDetailEvent::RequestDelete => Self::RequestDelete,
            IngredientDetailEvent::Confirmation { event } => Self::Confirmation(event.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum IngredientDetailDeletion {
    Idle,
    Confirming {
        confirmation: ConfirmationView,
        error: Option<IngredientDeleteError>,
    },
    Deleting,
    Deleted,
}

impl From<core::IngredientDetailDeletion> for IngredientDetailDeletion {
    fn from(value: core::IngredientDetailDeletion) -> Self {
        match value {
            core::IngredientDetailDeletion::Idle => Self::Idle,
            core::IngredientDetailDeletion::Confirming {
                confirmation,
                error,
            } => Self::Confirming {
                confirmation: confirmation.into(),
                error: error.map(Into::into),
            },
            core::IngredientDetailDeletion::Deleting => Self::Deleting,
            core::IngredientDetailDeletion::Deleted => Self::Deleted,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct IngredientDetailView {
    pub ingredient: Ingredient,
    pub deletion: IngredientDetailDeletion,
}

impl From<core::IngredientDetailView> for IngredientDetailView {
    fn from(value: core::IngredientDetailView) -> Self {
        Self {
            ingredient: value.ingredient.into(),
            deletion: value.deletion.into(),
        }
    }
}
