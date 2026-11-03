use nutype::nutype;
use uuid::Uuid;

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

/// A position on the step canvas in whole points, from the top left corner
/// (D-19). A node cannot go left of or above the canvas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CanvasPoint {
    pub x: u32,
    pub y: u32,
}

/// A stored step of a recipe (REC-14).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Step {
    pub id: StepId,
    pub title: String,
    pub position: CanvasPoint,
}

/// A stored recipe. Until REC-22 a recipe needs only its title (E-02).
///
/// Titles compare by `ingredients::name_key`, so "Shakshuka" and
/// "shakshuka " are the same title (D-39).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recipe {
    pub id: RecipeId,
    pub title: String,
    /// In the order added.
    pub steps: Vec<Step>,
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
}

/// What `RecipeStore::create` writes. REC-07 adds the lines.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewRecipe {
    pub title: RecipeTitle,
    /// In the order added.
    pub steps: Vec<StepWrite>,
}

/// What `RecipeStore::update` writes: every field of the recipe, not a
/// patch. It has the same fields as `NewRecipe` for now, as its own type so
/// the two can differ later (REC-04/Q1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecipeUpdate {
    pub title: RecipeTitle,
    /// The whole new step list, in the order added. Steps keep their ids.
    pub steps: Vec<StepWrite>,
}
