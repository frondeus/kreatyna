use std::collections::HashSet;

use rust_decimal::Decimal;

use crate::decimal_input::{DecimalInputError, parse_decimal};

use super::{
    Ingredient, IngredientId, IngredientName, IngredientNameError, Nutrition, NutritionBasis,
    ReferenceUnit, UnitName, UnitNameError, name_key,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DraftMode {
    Create,
    Edit { id: IngredientId },
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ReferenceUnitDraft {
    pub name: String,
    pub grams: String,
    pub millilitres: String,
}

/// Editable text field in an ingredient draft. Unit indices use the current unit order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IngredientDraftField {
    Name,
    Notes,
    Calories,
    Protein,
    Carbohydrates,
    Fat,
    UnitName { index: usize },
    UnitGrams { index: usize },
    UnitMillilitres { index: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

impl IngredientDraft {
    pub fn new(basis: NutritionBasis) -> Self {
        Self {
            mode: DraftMode::Create,
            name: String::new(),
            notes: String::new(),
            basis,
            calories: String::new(),
            protein: String::new(),
            carbohydrates: String::new(),
            fat: String::new(),
            units: Vec::new(),
        }
    }

    /// Change one raw field without normalizing its text. Returns false for an
    /// unchanged value or an out-of-range unit index.
    pub fn set_field(&mut self, field: IngredientDraftField, value: String) -> bool {
        let text = match field {
            IngredientDraftField::Name => Some(&mut self.name),
            IngredientDraftField::Notes => Some(&mut self.notes),
            IngredientDraftField::Calories => Some(&mut self.calories),
            IngredientDraftField::Protein => Some(&mut self.protein),
            IngredientDraftField::Carbohydrates => Some(&mut self.carbohydrates),
            IngredientDraftField::Fat => Some(&mut self.fat),
            IngredientDraftField::UnitName { index } => {
                self.units.get_mut(index).map(|unit| &mut unit.name)
            }
            IngredientDraftField::UnitGrams { index } => {
                self.units.get_mut(index).map(|unit| &mut unit.grams)
            }
            IngredientDraftField::UnitMillilitres { index } => {
                self.units.get_mut(index).map(|unit| &mut unit.millilitres)
            }
        };
        let Some(text) = text else { return false };
        if *text == value {
            return false;
        }
        *text = value;
        true
    }

    pub fn set_basis(&mut self, basis: NutritionBasis) -> bool {
        if self.basis == basis {
            return false;
        }
        self.basis = basis;
        true
    }

    pub fn add_unit(&mut self) {
        self.units.push(ReferenceUnitDraft::default());
    }

    pub fn remove_unit(&mut self, index: usize) -> bool {
        if index >= self.units.len() {
            return false;
        }
        self.units.remove(index);
        true
    }

    /// Move a unit to its final index. Invalid or unchanged moves do nothing.
    pub fn move_unit(&mut self, from: usize, to: usize) -> bool {
        if from >= self.units.len() || to >= self.units.len() || from == to {
            return false;
        }
        let unit = self.units.remove(from);
        self.units.insert(to, unit);
        true
    }
}

impl From<&Ingredient> for IngredientDraft {
    fn from(value: &Ingredient) -> Self {
        fn text(value: Option<impl ToString>) -> String {
            value.map(|v| v.to_string()).unwrap_or_default()
        }
        Self {
            mode: DraftMode::Edit { id: value.id },
            name: value.name.to_string(),
            notes: value.notes.clone().unwrap_or_default(),
            basis: value.nutrition.basis,
            calories: text(value.nutrition.calories),
            protein: text(value.nutrition.protein),
            carbohydrates: text(value.nutrition.carbohydrates),
            fat: text(value.nutrition.fat),
            units: value
                .units
                .iter()
                .map(|unit| ReferenceUnitDraft {
                    name: unit.name.to_string(),
                    grams: text(unit.grams),
                    millilitres: text(unit.millilitres),
                })
                .collect(),
        }
    }
}

/// Validated data; construction is restricted to draft validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IngredientWriteData {
    name: IngredientName,
    notes: Option<String>,
    nutrition: Nutrition,
    units: Vec<ReferenceUnit>,
}

impl IngredientWriteData {
    pub fn name(&self) -> &IngredientName {
        &self.name
    }
    pub fn notes(&self) -> Option<&str> {
        self.notes.as_deref()
    }
    pub fn nutrition(&self) -> &Nutrition {
        &self.nutrition
    }
    pub fn units(&self) -> &[ReferenceUnit] {
        &self.units
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngredientWrite {
    Create {
        data: IngredientWriteData,
    },
    Update {
        id: IngredientId,
        data: IngredientWriteData,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationField {
    Name,
    Calories,
    Protein,
    Carbohydrates,
    Fat,
    UnitName { index: usize },
    UnitGrams { index: usize },
    UnitMillilitres { index: usize },
    UnitMeasurements { index: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationReason {
    Required,
    InvalidNumber,
    TooManyDecimalPlaces,
    OutOfRange,
    MustBeNonNegative,
    MustBePositive,
    DuplicateName,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidationIssue {
    pub field: ValidationField,
    pub reason: ValidationReason,
}

#[derive(Clone, Copy)]
enum AcceptsZero {
    Yes,
    No,
}

impl TryFrom<&IngredientDraft> for IngredientWrite {
    type Error = Vec<ValidationIssue>;

    fn try_from(draft: &IngredientDraft) -> Result<Self, Self::Error> {
        let mut issues = Vec::new();
        // The type owns the "trimmed and not empty" rule.
        let name = IngredientName::try_new(draft.name.as_str())
            .map_err(|IngredientNameError::NotEmptyViolated| {
                issues.push(ValidationIssue {
                    field: ValidationField::Name,
                    reason: ValidationReason::Required,
                });
            })
            .ok();
        let nutrition = Nutrition {
            basis: draft.basis,
            calories: number(
                &draft.calories,
                ValidationField::Calories,
                AcceptsZero::Yes,
                &mut issues,
            ),
            protein: number(
                &draft.protein,
                ValidationField::Protein,
                AcceptsZero::Yes,
                &mut issues,
            ),
            carbohydrates: number(
                &draft.carbohydrates,
                ValidationField::Carbohydrates,
                AcceptsZero::Yes,
                &mut issues,
            ),
            fat: number(
                &draft.fat,
                ValidationField::Fat,
                AcceptsZero::Yes,
                &mut issues,
            ),
        };
        let mut names = HashSet::new();
        let mut units = Vec::with_capacity(draft.units.len());
        for (index, unit) in draft.units.iter().enumerate() {
            let unit_name = match UnitName::try_new(unit.name.as_str()) {
                Err(UnitNameError::NotEmptyViolated) => Err(ValidationReason::Required),
                Ok(name) if !names.insert(name_key(name.as_ref())) => {
                    Err(ValidationReason::DuplicateName)
                }
                Ok(name) => Ok(name),
            };
            let unit_name = unit_name
                .map_err(|reason| {
                    issues.push(ValidationIssue {
                        field: ValidationField::UnitName { index },
                        reason,
                    });
                })
                .ok();
            let grams = number(
                &unit.grams,
                ValidationField::UnitGrams { index },
                AcceptsZero::No,
                &mut issues,
            );
            let millilitres = number(
                &unit.millilitres,
                ValidationField::UnitMillilitres { index },
                AcceptsZero::No,
                &mut issues,
            );
            if unit.grams.trim().is_empty() && unit.millilitres.trim().is_empty() {
                issues.push(ValidationIssue {
                    field: ValidationField::UnitMeasurements { index },
                    reason: ValidationReason::Required,
                });
            }
            if let Some(name) = unit_name {
                units.push(ReferenceUnit {
                    name,
                    grams,
                    millilitres,
                });
            }
        }
        let (Some(name), true) = (name, issues.is_empty()) else {
            return Err(issues);
        };
        let notes = draft.notes.trim();
        let data = IngredientWriteData {
            name,
            notes: (!notes.is_empty()).then(|| notes.to_owned()),
            nutrition,
            units,
        };
        Ok(match draft.mode {
            DraftMode::Create => Self::Create { data },
            DraftMode::Edit { id } => Self::Update { id, data },
        })
    }
}

fn number<Q: TryFrom<Decimal>>(
    input: &str,
    field: ValidationField,
    accepts_zero: AcceptsZero,
    issues: &mut Vec<ValidationIssue>,
) -> Option<Q> {
    let input = input.trim();
    if input.is_empty() {
        return None;
    }
    match parse_number(input, accepts_zero) {
        Ok(value) => Some(value),
        Err(reason) => {
            issues.push(ValidationIssue { field, reason });
            None
        }
    }
}

fn parse_number<Q: TryFrom<Decimal>>(
    input: &str,
    accepts_zero: AcceptsZero,
) -> Result<Q, ValidationReason> {
    let value = parse_decimal(input).map_err(|error| match error {
        DecimalInputError::NotANumber => ValidationReason::InvalidNumber,
        DecimalInputError::TooManyDecimalPlaces => ValidationReason::TooManyDecimalPlaces,
        DecimalInputError::OutOfRange => ValidationReason::OutOfRange,
    })?;
    if matches!(accepts_zero, AcceptsZero::No) && value <= Decimal::ZERO {
        return Err(ValidationReason::MustBePositive);
    }
    // The quantity type owns the "not negative" rule. Its only error is
    // `GreaterOrEqualViolated`, so every error means a negative number.
    Q::try_from(value).map_err(|_| ValidationReason::MustBeNonNegative)
}
