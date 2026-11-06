//! The recipe ingredients in the recipe editor: ＋ Line adds one, you type
//! its name (REC-07), its amount, unit and note (REC-08), drag it to another
//! place, or remove it. The ingredients have no effects, so they are a plain
//! struct, not a widget: the editor renders after each change.
use super::{
    KnownUnit, RecipeIngredient, RecipeIngredientAmount, RecipeIngredientId, RecipeIngredientName,
    RecipeIngredientNameError, RecipeIngredientNote, RecipeIngredientQuantity,
};
use crate::{
    decimal_input::{DecimalInputError, parse_decimal},
    draft_change::DraftChange,
    problem_display::ProblemDisplay,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecipeIngredientsEvent {
    /// ＋ Line: an empty ingredient at the bottom, with a new id.
    Add,
    /// The raw text of the Name field. Rust trims it only on Save. Ignored
    /// for an id that names no ingredient.
    SetName {
        id: RecipeIngredientId,
        name: String,
    },
    /// The raw text of the Amount field. Rust reads it only on Save. Ignored
    /// for an id that names no ingredient.
    SetAmount {
        id: RecipeIngredientId,
        amount: String,
    },
    /// The Unit picker. `None` is "No unit". Ignored for an id that names no
    /// ingredient.
    SetUnit {
        id: RecipeIngredientId,
        unit: Option<KnownUnit>,
    },
    /// The raw text of the Note field. Rust trims it only on Save. Ignored
    /// for an id that names no ingredient.
    SetNote {
        id: RecipeIngredientId,
        note: String,
    },
    /// The − button. Ignored for an id that names no ingredient.
    Remove { id: RecipeIngredientId },
    /// A drop of the ingredient `id` onto the ingredient `onto`: `id` takes `onto`'s
    /// place, and the ingredients between move one place (REC-07/Q1). So the
    /// dragged ingredient ends up before `onto` when it moves up, and after `onto`
    /// when it moves down. With the ingredients penne, oil, basil:
    ///
    /// | Drag              | Result            | The dragged ingredient ends up |
    /// |-------------------|-------------------|--------------------------------|
    /// | basil onto penne  | basil, penne, oil | before penne                   |
    /// | penne onto basil  | oil, basil, penne | after basil                    |
    ///
    /// Ignored when either id names no ingredient, or when both are the same ingredient.
    Move {
        id: RecipeIngredientId,
        onto: RecipeIngredientId,
    },
}

/// Why Save refuses an ingredient's amount (REC-08).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecipeIngredientAmountProblem {
    /// "lots" or "1.2.3".
    NotANumber,
    /// More than 6 decimal places: "0.1234567".
    TooManyDecimalPlaces,
    /// A number too large to keep.
    OutOfRange,
    /// "0" or "-1".
    NotPositive,
    /// A unit is picked, and the amount is empty: "g flour".
    MissingForUnit,
}

/// An ingredient's problems. Both are `None` until the first Save.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecipeIngredientProblems {
    pub name: Option<RecipeIngredientNameError>,
    pub amount: Option<RecipeIngredientAmountProblem>,
}

/// One ingredient as the editor shows it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecipeIngredientView {
    pub id: RecipeIngredientId,
    /// The popover's fields, as typed.
    pub name: String,
    pub amount: String,
    pub unit: Option<KnownUnit>,
    pub note: String,
    /// What the row shows.
    pub summary: RecipeIngredientSummary,
    pub problems: RecipeIngredientProblems,
}

/// The row's text, in three parts that Swift joins:
///
/// | Fields                              | `amount`  | `name`              | `note`      |
/// |-------------------------------------|-----------|---------------------|-------------|
/// | "1,50", kg, "tomatoes", " chopped " | "1,50 kg" | Named("tomatoes")   | "chopped"   |
/// | "2", no unit, "eggs", ""            | "2"       | Named("eggs")       | `None`      |
/// | "", g, "flour", ""                  | "g"       | Named("flour")      | `None`      |
/// | "", no unit, "", "to taste"         | `None`    | Unnamed             | "to taste"  |
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecipeIngredientSummary {
    /// The amount as typed and trimmed, then the unit's name. `None` with
    /// neither.
    pub amount: Option<String>,
    pub name: RecipeIngredientLabel,
    /// The note, trimmed. `None` when it is empty.
    pub note: Option<String>,
}

/// The name in the row's summary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecipeIngredientLabel {
    /// The name, trimmed: "penne pasta".
    Named { name: String },
    /// The name is empty. Swift shows "No name" in grey.
    Unnamed,
}

/// One choice of the Unit picker, after "No unit": `Gram` with "g".
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KnownUnitOption {
    pub unit: KnownUnit,
    pub label: String,
}

/// The Unit picker's choices after "No unit", in the order of D-25.
pub(crate) fn unit_options() -> Vec<KnownUnitOption> {
    KnownUnit::ALL
        .into_iter()
        .map(|unit| KnownUnitOption {
            unit,
            label: unit.picker_label().to_owned(),
        })
        .collect()
}

/// Save found an ingredient with a problem. Each such ingredient shows it
/// (`RecipeIngredientView::problems`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct InvalidRecipeIngredient;

/// An ingredient as the editor keeps it: each field as typed.
#[derive(Debug, Clone, PartialEq, Eq)]
struct DraftRecipeIngredient {
    id: RecipeIngredientId,
    name: String,
    amount: String,
    unit: Option<KnownUnit>,
    note: String,
}

impl DraftRecipeIngredient {
    fn empty() -> Self {
        Self {
            id: RecipeIngredientId::generate(),
            name: String::new(),
            amount: String::new(),
            unit: None,
            note: String::new(),
        }
    }

    /// The fields of a saved ingredient, as the popover shows them. The
    /// amount keeps its stored decimal places: "1.50".
    fn saved(ingredient: &RecipeIngredient) -> Self {
        let name = ingredient.name.as_ref().to_owned();
        let note = ingredient
            .note
            .as_ref()
            .map(|note| note.as_ref().to_owned())
            .unwrap_or_default();
        let amount = match ingredient.quantity {
            RecipeIngredientQuantity::Unmeasured => String::new(),
            RecipeIngredientQuantity::Count(amount)
            | RecipeIngredientQuantity::Measured { amount, .. } => amount.to_string(),
        };
        let unit = match ingredient.quantity {
            RecipeIngredientQuantity::Measured { unit, .. } => Some(unit),
            RecipeIngredientQuantity::Unmeasured | RecipeIngredientQuantity::Count(_) => None,
        };
        Self {
            id: ingredient.id,
            name,
            amount,
            unit,
            note,
        }
    }

    /// The amount and the unit as Save writes them, or the amount's problem.
    /// An empty amount is no amount.
    fn quantity(&self) -> Result<RecipeIngredientQuantity, RecipeIngredientAmountProblem> {
        let amount = typed(&self.amount).map(amount).transpose()?;
        match (amount, self.unit) {
            (None, None) => Ok(RecipeIngredientQuantity::Unmeasured),
            (None, Some(_)) => Err(RecipeIngredientAmountProblem::MissingForUnit),
            (Some(amount), None) => Ok(RecipeIngredientQuantity::Count(amount)),
            (Some(amount), Some(unit)) => Ok(RecipeIngredientQuantity::Measured { amount, unit }),
        }
    }

    /// The ingredient as Save writes it: the name and the note trimmed, an
    /// empty note as no note.
    fn write(&self) -> Result<RecipeIngredient, InvalidRecipeIngredient> {
        let name = RecipeIngredientName::try_new(self.name.as_str());
        match (name, self.quantity()) {
            (Ok(name), Ok(quantity)) => Ok(RecipeIngredient {
                id: self.id,
                name,
                quantity,
                note: RecipeIngredientNote::try_new(self.note.as_str()).ok(),
            }),
            (Err(_), _) | (_, Err(_)) => Err(InvalidRecipeIngredient),
        }
    }

    fn problems(&self, display: ProblemDisplay) -> RecipeIngredientProblems {
        match display {
            ProblemDisplay::Hidden => RecipeIngredientProblems {
                name: None,
                amount: None,
            },
            ProblemDisplay::Shown => RecipeIngredientProblems {
                name: RecipeIngredientName::try_new(self.name.as_str()).err(),
                amount: self.quantity().err(),
            },
        }
    }

    fn summary(&self) -> RecipeIngredientSummary {
        let amount = typed(&self.amount);
        let unit = self.unit.map(|unit| match amount.map(parse_decimal) {
            Some(Ok(value)) => unit.label_for(value),
            Some(Err(_)) | None => unit.picker_label(),
        });
        RecipeIngredientSummary {
            amount: match (amount, unit) {
                (None, None) => None,
                (Some(amount), None) => Some(amount.to_owned()),
                (None, Some(unit)) => Some(unit.to_owned()),
                (Some(amount), Some(unit)) => Some(format!("{amount} {unit}")),
            },
            name: match typed(&self.name) {
                Some(name) => RecipeIngredientLabel::Named {
                    name: name.to_owned(),
                },
                None => RecipeIngredientLabel::Unnamed,
            },
            note: typed(&self.note).map(str::to_owned),
        }
    }

    fn view(&self, display: ProblemDisplay) -> RecipeIngredientView {
        RecipeIngredientView {
            id: self.id,
            name: self.name.clone(),
            amount: self.amount.clone(),
            unit: self.unit,
            note: self.note.clone(),
            summary: self.summary(),
            problems: self.problems(display),
        }
    }
}

/// The text without the spaces around it, or `None` when nothing is left.
fn typed(text: &str) -> Option<&str> {
    Some(text.trim()).filter(|text| !text.is_empty())
}

/// A typed, trimmed amount as a decimal of more than 0.
fn amount(text: &str) -> Result<RecipeIngredientAmount, RecipeIngredientAmountProblem> {
    let value = parse_decimal(text).map_err(|error| match error {
        DecimalInputError::NotANumber => RecipeIngredientAmountProblem::NotANumber,
        DecimalInputError::TooManyDecimalPlaces => {
            RecipeIngredientAmountProblem::TooManyDecimalPlaces
        }
        DecimalInputError::OutOfRange => RecipeIngredientAmountProblem::OutOfRange,
    })?;
    RecipeIngredientAmount::try_new(value).map_err(|_| RecipeIngredientAmountProblem::NotPositive)
}

/// The editor's ingredients, in list order.
pub(crate) struct DraftRecipeIngredients(Vec<DraftRecipeIngredient>);

impl DraftRecipeIngredients {
    /// The ingredients that the editor opens with: none for a new recipe, the
    /// stored ones for a saved one.
    pub(crate) fn new(ingredients: &[RecipeIngredient]) -> Self {
        Self(
            ingredients
                .iter()
                .map(DraftRecipeIngredient::saved)
                .collect(),
        )
    }

    pub(crate) fn apply(&mut self, event: RecipeIngredientsEvent) -> DraftChange {
        match event {
            RecipeIngredientsEvent::Add => {
                self.0.push(DraftRecipeIngredient::empty());
                DraftChange::Changed
            }
            RecipeIngredientsEvent::SetName { id, name } => {
                self.edit(id, |ingredient| &mut ingredient.name, name)
            }
            RecipeIngredientsEvent::SetAmount { id, amount } => {
                self.edit(id, |ingredient| &mut ingredient.amount, amount)
            }
            RecipeIngredientsEvent::SetUnit { id, unit } => {
                self.edit(id, |ingredient| &mut ingredient.unit, unit)
            }
            RecipeIngredientsEvent::SetNote { id, note } => {
                self.edit(id, |ingredient| &mut ingredient.note, note)
            }
            RecipeIngredientsEvent::Remove { id } => match self.index(id) {
                Some(index) => {
                    self.0.remove(index);
                    DraftChange::Changed
                }
                None => DraftChange::Unchanged,
            },
            RecipeIngredientsEvent::Move { id, onto } => match (self.index(id), self.index(onto)) {
                (Some(from), Some(to)) if from != to => {
                    let ingredient = self.0.remove(from);
                    self.0.insert(to, ingredient);
                    DraftChange::Changed
                }
                _ => DraftChange::Unchanged,
            },
        }
    }

    pub(crate) fn view(&self, display: ProblemDisplay) -> Vec<RecipeIngredientView> {
        self.0
            .iter()
            .map(|ingredient| ingredient.view(display))
            .collect()
    }

    /// Every ingredient as Save writes it, or `InvalidRecipeIngredient` when
    /// one has a problem.
    pub(crate) fn writes(&self) -> Result<Vec<RecipeIngredient>, InvalidRecipeIngredient> {
        self.0.iter().map(DraftRecipeIngredient::write).collect()
    }

    /// Whether the ingredients differ from `saved`: in order, ids or any
    /// field as typed.
    pub(crate) fn differs_from(&self, saved: &[RecipeIngredient]) -> bool {
        self.0.len() != saved.len()
            || self
                .0
                .iter()
                .zip(saved)
                .any(|(ingredient, saved)| *ingredient != DraftRecipeIngredient::saved(saved))
    }

    /// Sets one `field` of the ingredient `id` to `value`, as the step
    /// canvas's `edit` does. An unknown id changes nothing.
    fn edit<T: PartialEq>(
        &mut self,
        id: RecipeIngredientId,
        field: impl FnOnce(&mut DraftRecipeIngredient) -> &mut T,
        value: T,
    ) -> DraftChange {
        match self.0.iter_mut().find(|ingredient| ingredient.id == id) {
            Some(ingredient) => DraftChange::set(field(ingredient), value),
            None => DraftChange::Unchanged,
        }
    }

    fn index(&self, id: RecipeIngredientId) -> Option<usize> {
        self.0.iter().position(|ingredient| ingredient.id == id)
    }
}
