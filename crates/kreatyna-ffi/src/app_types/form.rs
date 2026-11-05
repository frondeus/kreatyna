use kreatyna::ingredients as core;

use super::support::index;
use super::{
    AppTypeConversionError, ConfirmationEvent, ConfirmationView, IngredientDraft,
    IngredientDraftField, IngredientSaveError, NutritionBasis, ValidationIssue,
};

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum IngredientFormEvent {
    SetField {
        field: IngredientDraftField,
        value: String,
    },
    SetBasis {
        basis: NutritionBasis,
    },
    AddUnit,
    RemoveUnit {
        index: u64,
    },
    MoveUnit {
        from: u64,
        to: u64,
    },
    Save,
    RequestClose,
    Confirmation {
        event: ConfirmationEvent,
    },
}

impl TryFrom<IngredientFormEvent> for core::IngredientFormEvent {
    type Error = AppTypeConversionError;

    fn try_from(value: IngredientFormEvent) -> Result<Self, Self::Error> {
        Ok(match value {
            IngredientFormEvent::SetField { field, value } => Self::SetField {
                field: field.try_into()?,
                value,
            },
            IngredientFormEvent::SetBasis { basis } => Self::SetBasis(basis.into()),
            IngredientFormEvent::AddUnit => Self::AddUnit,
            IngredientFormEvent::RemoveUnit { index: value } => Self::RemoveUnit {
                index: index(value)?,
            },
            IngredientFormEvent::MoveUnit { from, to } => Self::MoveUnit {
                from: index(from)?,
                to: index(to)?,
            },
            IngredientFormEvent::Save => Self::Save,
            IngredientFormEvent::RequestClose => Self::RequestClose,
            IngredientFormEvent::Confirmation { event } => Self::Confirmation(event.into()),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum IngredientFormStatus {
    Editing,
    ConfirmingDiscard { confirmation: ConfirmationView },
    Saving,
    Finished,
}

impl From<core::IngredientFormStatus> for IngredientFormStatus {
    fn from(value: core::IngredientFormStatus) -> Self {
        match value {
            core::IngredientFormStatus::Editing => Self::Editing,
            core::IngredientFormStatus::ConfirmingDiscard(confirmation) => {
                Self::ConfirmingDiscard {
                    confirmation: confirmation.into(),
                }
            }
            core::IngredientFormStatus::Saving => Self::Saving,
            core::IngredientFormStatus::Finished => Self::Finished,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct IngredientFormView {
    pub draft: IngredientDraft,
    pub issues: Vec<ValidationIssue>,
    pub save_error: Option<IngredientSaveError>,
    pub status: IngredientFormStatus,
}

impl TryFrom<core::IngredientFormView> for IngredientFormView {
    type Error = AppTypeConversionError;

    fn try_from(value: core::IngredientFormView) -> Result<Self, Self::Error> {
        Ok(Self {
            draft: value.draft.into(),
            issues: value
                .issues
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
            save_error: value.save_error.map(Into::into),
            status: value.status.into(),
        })
    }
}
