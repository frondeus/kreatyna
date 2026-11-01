use kreatyna::ingredients as core;

use super::support::{count, index};
use super::{AppTypeConversionError, NutritionBasis};

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum DraftMode {
    Create,
    Edit { id: core::IngredientId },
}

impl From<core::DraftMode> for DraftMode {
    fn from(value: core::DraftMode) -> Self {
        match value {
            core::DraftMode::Create => Self::Create,
            core::DraftMode::Edit { id } => Self::Edit { id },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct ReferenceUnitDraft {
    pub name: String,
    pub grams: String,
    pub millilitres: String,
}

impl From<core::ReferenceUnitDraft> for ReferenceUnitDraft {
    fn from(value: core::ReferenceUnitDraft) -> Self {
        Self {
            name: value.name,
            grams: value.grams,
            millilitres: value.millilitres,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct IngredientDraft {
    pub mode: DraftMode,
    pub name: String,
    pub notes: String,
    pub basis: NutritionBasis,
    pub calories: String,
    pub protein: String,
    pub carbohydrates: String,
    pub fat: String,
    pub units: Vec<ReferenceUnitDraft>,
}

impl From<core::IngredientDraft> for IngredientDraft {
    fn from(value: core::IngredientDraft) -> Self {
        Self {
            mode: value.mode.into(),
            name: value.name,
            notes: value.notes,
            basis: value.basis.into(),
            calories: value.calories,
            protein: value.protein,
            carbohydrates: value.carbohydrates,
            fat: value.fat,
            units: value.units.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum IngredientDraftField {
    Name,
    Notes,
    Calories,
    Protein,
    Carbohydrates,
    Fat,
    UnitName { index: u64 },
    UnitGrams { index: u64 },
    UnitMillilitres { index: u64 },
}

impl TryFrom<IngredientDraftField> for core::IngredientDraftField {
    type Error = AppTypeConversionError;

    fn try_from(value: IngredientDraftField) -> Result<Self, Self::Error> {
        Ok(match value {
            IngredientDraftField::Name => Self::Name,
            IngredientDraftField::Notes => Self::Notes,
            IngredientDraftField::Calories => Self::Calories,
            IngredientDraftField::Protein => Self::Protein,
            IngredientDraftField::Carbohydrates => Self::Carbohydrates,
            IngredientDraftField::Fat => Self::Fat,
            IngredientDraftField::UnitName { index: value } => Self::UnitName {
                index: index(value)?,
            },
            IngredientDraftField::UnitGrams { index: value } => Self::UnitGrams {
                index: index(value)?,
            },
            IngredientDraftField::UnitMillilitres { index: value } => Self::UnitMillilitres {
                index: index(value)?,
            },
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum ValidationField {
    Name,
    Calories,
    Protein,
    Carbohydrates,
    Fat,
    UnitName { index: u64 },
    UnitGrams { index: u64 },
    UnitMillilitres { index: u64 },
    UnitMeasurements { index: u64 },
}

impl TryFrom<core::ValidationField> for ValidationField {
    type Error = AppTypeConversionError;

    fn try_from(value: core::ValidationField) -> Result<Self, Self::Error> {
        Ok(match value {
            core::ValidationField::Name => Self::Name,
            core::ValidationField::Calories => Self::Calories,
            core::ValidationField::Protein => Self::Protein,
            core::ValidationField::Carbohydrates => Self::Carbohydrates,
            core::ValidationField::Fat => Self::Fat,
            core::ValidationField::UnitName { index } => Self::UnitName {
                index: count(index)?,
            },
            core::ValidationField::UnitGrams { index } => Self::UnitGrams {
                index: count(index)?,
            },
            core::ValidationField::UnitMillilitres { index } => Self::UnitMillilitres {
                index: count(index)?,
            },
            core::ValidationField::UnitMeasurements { index } => Self::UnitMeasurements {
                index: count(index)?,
            },
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum ValidationReason {
    Required,
    InvalidNumber,
    TooManyDecimalPlaces,
    OutOfRange,
    MustBeNonNegative,
    MustBePositive,
    DuplicateName,
}

impl From<core::ValidationReason> for ValidationReason {
    fn from(value: core::ValidationReason) -> Self {
        match value {
            core::ValidationReason::Required => Self::Required,
            core::ValidationReason::InvalidNumber => Self::InvalidNumber,
            core::ValidationReason::TooManyDecimalPlaces => Self::TooManyDecimalPlaces,
            core::ValidationReason::OutOfRange => Self::OutOfRange,
            core::ValidationReason::MustBeNonNegative => Self::MustBeNonNegative,
            core::ValidationReason::MustBePositive => Self::MustBePositive,
            core::ValidationReason::DuplicateName => Self::DuplicateName,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct ValidationIssue {
    pub field: ValidationField,
    pub reason: ValidationReason,
}

impl TryFrom<core::ValidationIssue> for ValidationIssue {
    type Error = AppTypeConversionError;

    fn try_from(value: core::ValidationIssue) -> Result<Self, Self::Error> {
        Ok(Self {
            field: value.field.try_into()?,
            reason: value.reason.into(),
        })
    }
}
