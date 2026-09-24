use std::collections::HashSet;

use rust_decimal::Decimal;

use super::{Ingredient, IngredientId, Nutrition, NutritionBasis, ReferenceUnit, name_key};

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
}

impl From<&Ingredient> for IngredientDraft {
    fn from(value: &Ingredient) -> Self {
        fn text(value: Option<impl ToString>) -> String {
            value.map(|v| v.to_string()).unwrap_or_default()
        }
        Self {
            mode: DraftMode::Edit { id: value.id },
            name: value.name.clone(),
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
                    name: unit.name.clone(),
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
    name: String,
    notes: Option<String>,
    nutrition: Nutrition,
    units: Vec<ReferenceUnit>,
}

impl IngredientWriteData {
    pub fn name(&self) -> &str {
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
        let name = draft.name.trim();
        if name.is_empty() {
            issues.push(ValidationIssue {
                field: ValidationField::Name,
                reason: ValidationReason::Required,
            });
        }
        let nutrition = Nutrition {
            basis: draft.basis,
            calories: number(
                &draft.calories,
                ValidationField::Calories,
                AcceptsZero::Yes,
                &mut issues,
            )
            .map(Into::into),
            protein: number(
                &draft.protein,
                ValidationField::Protein,
                AcceptsZero::Yes,
                &mut issues,
            )
            .map(Into::into),
            carbohydrates: number(
                &draft.carbohydrates,
                ValidationField::Carbohydrates,
                AcceptsZero::Yes,
                &mut issues,
            )
            .map(Into::into),
            fat: number(
                &draft.fat,
                ValidationField::Fat,
                AcceptsZero::Yes,
                &mut issues,
            )
            .map(Into::into),
        };
        let mut names = HashSet::new();
        let mut units = Vec::with_capacity(draft.units.len());
        for (index, unit) in draft.units.iter().enumerate() {
            let unit_name = unit.name.trim();
            let reason = if unit_name.is_empty() {
                Some(ValidationReason::Required)
            } else if !names.insert(name_key(unit_name)) {
                Some(ValidationReason::DuplicateName)
            } else {
                None
            };
            if let Some(reason) = reason {
                issues.push(ValidationIssue {
                    field: ValidationField::UnitName { index },
                    reason,
                });
            }
            let grams = number(
                &unit.grams,
                ValidationField::UnitGrams { index },
                AcceptsZero::No,
                &mut issues,
            )
            .map(Into::into);
            let millilitres = number(
                &unit.millilitres,
                ValidationField::UnitMillilitres { index },
                AcceptsZero::No,
                &mut issues,
            )
            .map(Into::into);
            if unit.grams.trim().is_empty() && unit.millilitres.trim().is_empty() {
                issues.push(ValidationIssue {
                    field: ValidationField::UnitMeasurements { index },
                    reason: ValidationReason::Required,
                });
            }
            units.push(ReferenceUnit {
                name: unit_name.to_owned(),
                grams,
                millilitres,
            });
        }
        if !issues.is_empty() {
            return Err(issues);
        }
        let notes = draft.notes.trim();
        let data = IngredientWriteData {
            name: name.to_owned(),
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

fn number(
    input: &str,
    field: ValidationField,
    accepts_zero: AcceptsZero,
    issues: &mut Vec<ValidationIssue>,
) -> Option<Decimal> {
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

fn parse_number(input: &str, accepts_zero: AcceptsZero) -> Result<Decimal, ValidationReason> {
    let unsigned = input.strip_prefix('-').unwrap_or(input);
    let mut parts = unsigned.split(['.', ',']);
    let integer = parts.next().unwrap_or_default();
    let fractional = parts.next();
    let digits = |text: &str| !text.is_empty() && text.bytes().all(|byte| byte.is_ascii_digit());
    if !digits(integer) || fractional.is_some_and(|part| !digits(part)) || parts.next().is_some() {
        return Err(ValidationReason::InvalidNumber);
    }
    if fractional.is_some_and(|part| part.len() > 6) {
        return Err(ValidationReason::TooManyDecimalPlaces);
    }
    let value = Decimal::from_str_exact(&input.replace(',', "."))
        .map_err(|_| ValidationReason::OutOfRange)?;
    match accepts_zero {
        AcceptsZero::Yes if value < Decimal::ZERO => {
            return Err(ValidationReason::MustBeNonNegative);
        }
        AcceptsZero::No if value <= Decimal::ZERO => {
            return Err(ValidationReason::MustBePositive);
        }
        _ => {}
    }
    Ok(value)
}
