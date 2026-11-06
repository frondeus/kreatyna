use nutype::nutype;
use rust_decimal::Decimal;
use uuid::Uuid;

use super::StepEdges;

/// Stable recipe identity. Creation is explicit and does not use a nil sentinel:
/// `generate` makes a version 4 identifier, and `new` wraps a known one.
#[nutype(derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Display, FromStr, From, Into
))]
pub struct RecipeId(Uuid);

impl RecipeId {
    /// Generate a new version 4 identifier.
    pub fn generate() -> Self {
        Self::new(Uuid::new_v4())
    }
}

/// Stable step identity. The editor generates it when you add a step
/// (REC-14), and the step keeps it through every save.
#[nutype(derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Display, FromStr, From, Into
))]
pub struct StepId(Uuid);

impl StepId {
    /// Generate a new version 4 identifier.
    pub fn generate() -> Self {
        Self::new(Uuid::new_v4())
    }
}

/// Stable recipe ingredient identity. The editor generates it when you add a
/// ingredient (REC-07), and the ingredient keeps it through every save.
#[nutype(derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Display, FromStr, From, Into
))]
pub struct RecipeIngredientId(Uuid);

impl RecipeIngredientId {
    /// Generate a new version 4 identifier.
    pub fn generate() -> Self {
        Self::new(Uuid::new_v4())
    }
}

/// An ingredient's name: trimmed and not empty (REC-07). `try_new` keeps the
/// spaces inside the name and fails with
/// `RecipeIngredientNameError::NotEmptyViolated`.
#[nutype(
    sanitize(trim),
    validate(not_empty),
    derive(Debug, Clone, PartialEq, Eq, AsRef)
)]
pub struct RecipeIngredientName(String);

/// An ingredient's amount, such as 300 in "300 g penne pasta" (REC-08): an exact
/// decimal, more than 0. `try_new` fails with
/// `RecipeIngredientAmountError::GreaterViolated`.
#[nutype(
    validate(greater = Decimal::ZERO),
    derive(Debug, Clone, Copy, PartialEq, Eq, Display, TryFrom, Into)
)]
pub struct RecipeIngredientAmount(Decimal);

/// The units that the app knows (D-25), in the picker's order. Only these
/// units can convert later. REC-09 adds custom units.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KnownUnit {
    Gram,
    Kilogram,
    Millilitre,
    Litre,
    Teaspoon,
    Tablespoon,
    Cup,
    Piece,
}

impl KnownUnit {
    /// Every unit, in the picker's order (D-25).
    pub const ALL: [Self; 8] = [
        Self::Gram,
        Self::Kilogram,
        Self::Millilitre,
        Self::Litre,
        Self::Teaspoon,
        Self::Tablespoon,
        Self::Cup,
        Self::Piece,
    ];

    /// The unit's name in the picker: "g", "tsp", "cup".
    pub fn picker_label(self) -> &'static str {
        self.label_for(Decimal::ONE)
    }

    /// The unit's name after `amount`. Only "cup" and "piece" are words, so
    /// only they get a plural:
    ///
    /// | Amount | Gram | Cup  | Piece  |
    /// |--------|------|------|--------|
    /// | 1      | g    | cup  | piece  |
    /// | 1.5    | g    | cups | pieces |
    /// | 2      | g    | cups | pieces |
    pub fn label_for(self, amount: Decimal) -> &'static str {
        let one = amount == Decimal::ONE;
        match self {
            Self::Gram => "g",
            Self::Kilogram => "kg",
            Self::Millilitre => "ml",
            Self::Litre => "l",
            Self::Teaspoon => "tsp",
            Self::Tablespoon => "tbsp",
            Self::Cup if one => "cup",
            Self::Cup => "cups",
            Self::Piece if one => "piece",
            Self::Piece => "pieces",
        }
    }
}

/// How much of the ingredient the recipe needs (REC-08). A unit without an amount
/// cannot be built.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecipeIngredientQuantity {
    /// "salt, to taste"
    Unmeasured,
    /// "2 eggs"
    Count(RecipeIngredientAmount),
    /// "300 g penne pasta"
    Measured {
        amount: RecipeIngredientAmount,
        unit: KnownUnit,
    },
}

impl RecipeIngredientQuantity {
    /// The read view's amount column (REC-08), with no trailing zeros:
    ///
    /// | Quantity                   | Text     |
    /// |----------------------------|----------|
    /// | `Unmeasured`               | `None`   |
    /// | `Count(2)`                 | "2"      |
    /// | `Measured(1.50, Kilogram)` | "1.5 kg" |
    /// | `Measured(2, Cup)`         | "2 cups" |
    pub fn text(self) -> Option<String> {
        match self {
            Self::Unmeasured => None,
            Self::Count(amount) => Some(Decimal::from(amount).normalize().to_string()),
            Self::Measured { amount, unit } => {
                let amount = Decimal::from(amount).normalize();
                Some(format!("{amount} {}", unit.label_for(amount)))
            }
        }
    }
}

/// An ingredient's note, such as "extra virgin" or "to taste" (D-12): trimmed and
/// not empty. `try_new` fails with `RecipeIngredientNoteError::NotEmptyViolated`.
#[nutype(
    sanitize(trim),
    validate(not_empty),
    derive(Debug, Clone, PartialEq, Eq, AsRef)
)]
pub struct RecipeIngredientNote(String);

/// An ingredient of a recipe's ingredient list, such as "300 g penne pasta" (D-10,
/// D-12). The store reads and writes this type, so a stored ingredient has a valid
/// name, and a unit only with an amount.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecipeIngredient {
    pub id: RecipeIngredientId,
    pub name: RecipeIngredientName,
    pub quantity: RecipeIngredientQuantity,
    pub note: Option<RecipeIngredientNote>,
}

/// A position on the step canvas in whole points, from the top left corner
/// (D-19). A node cannot go left of or above the canvas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CanvasPoint {
    pub x: u32,
    pub y: u32,
}

/// A stored step of a recipe (REC-14, REC-17).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Step {
    pub id: StepId,
    pub title: String,
    pub position: CanvasPoint,
    /// `None` until you set one (Q-R5).
    pub duration: Option<StepDurationS>,
    pub kind: StepKind,
    pub instructions: Option<StepInstructions>,
}

/// A step's duration in whole seconds; the `S` in the name stands for
/// seconds. It holds 1 second to 365 days (REC-17/Q3, Q4). `try_new` fails with `StepDurationSError::GreaterViolated` for 0 and
/// `StepDurationSError::LessOrEqualViolated` above 365 days.
#[nutype(
    validate(greater = 0, less_or_equal = 31_536_000),
    derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Into)
)]
pub struct StepDurationS(u32);

/// Whether your hands are busy during a step (D-09, D-30). A new step is
/// active.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StepKind {
    /// Hands busy, such as "Dice onion".
    #[default]
    Active,
    /// Hands free, such as "Bake 20 min".
    Waiting,
}

/// A step's longer text (D-09): trimmed and not empty. `try_new` fails
/// with `StepInstructionsError::NotEmptyViolated`; the editor reads that as
/// "no instructions".
#[nutype(
    sanitize(trim),
    validate(not_empty),
    derive(Debug, Clone, PartialEq, Eq, AsRef)
)]
pub struct StepInstructions(String);

/// A stored recipe. Until REC-22 a recipe needs only its title (E-02).
///
/// Titles compare by `ingredients::name_key`, so "Shakshuka" and
/// "shakshuka " are the same title (D-39).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recipe {
    pub id: RecipeId,
    pub title: String,
    /// In list order: the order added, then moved by drag (REC-07/Q1).
    pub ingredients: Vec<RecipeIngredient>,
    /// In the order added.
    pub steps: Vec<Step>,
    /// The arrows between `steps` (REC-18).
    pub edges: StepEdges,
}

/// A row of `RecipeStore::list`: what the recipe list shows, without the
/// steps themselves.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecipeSummary {
    pub id: RecipeId,
    pub title: String,
    pub step_count: u32,
}

/// A recipe title that passed the save rule: trimmed and not empty (E-02).
/// Uniqueness is the store's rule (D-39). `try_new` keeps the spaces inside
/// the title and fails with `RecipeTitleError::NotEmptyViolated`.
#[nutype(
    sanitize(trim),
    validate(not_empty),
    derive(Debug, Clone, PartialEq, Eq, AsRef)
)]
pub struct RecipeTitle(String);

/// A step title that passed the save rule: trimmed and not empty
/// (REC-14/Q2). `try_new` fails with `StepTitleError::NotEmptyViolated`.
#[nutype(
    sanitize(trim),
    validate(not_empty),
    derive(Debug, Clone, PartialEq, Eq, AsRef)
)]
pub struct StepTitle(String);

/// A step that passed the save rule, as `RecipeStore` writes it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StepWrite {
    pub id: StepId,
    pub title: StepTitle,
    pub position: CanvasPoint,
    pub duration: Option<StepDurationS>,
    pub kind: StepKind,
    pub instructions: Option<StepInstructions>,
}

/// What `RecipeStore::create` writes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewRecipe {
    pub title: RecipeTitle,
    /// In list order (REC-07/Q1).
    pub ingredients: Vec<RecipeIngredient>,
    /// In the order added.
    pub steps: Vec<StepWrite>,
    /// The arrows between `steps` (REC-18).
    pub edges: StepEdges,
}

/// What `RecipeStore::update` writes: every field of the recipe, not a
/// patch. It has the same fields as `NewRecipe` for now, as its own type so
/// the two can differ later (REC-04/Q1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecipeUpdate {
    pub title: RecipeTitle,
    /// The whole new ingredient list, in list order. Ingredients keep their ids.
    pub ingredients: Vec<RecipeIngredient>,
    /// The whole new step list, in the order added. Steps keep their ids.
    pub steps: Vec<StepWrite>,
    /// The whole new edge set between `steps` (REC-18).
    pub edges: StepEdges,
}
