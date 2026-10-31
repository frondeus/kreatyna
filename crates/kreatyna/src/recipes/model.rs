use std::{fmt, str::FromStr};

use uuid::Uuid;

/// Stable recipe identity. Creation is explicit and does not use a nil sentinel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RecipeId(Uuid);

#[allow(clippy::new_without_default)] // Identity generation must be an explicit action.
impl RecipeId {
    /// Generate a new version 4 identifier.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl From<Uuid> for RecipeId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<RecipeId> for Uuid {
    fn from(value: RecipeId) -> Self {
        value.0
    }
}

impl FromStr for RecipeId {
    type Err = uuid::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        value.parse::<Uuid>().map(Self::from)
    }
}

impl fmt::Display for RecipeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// A stored recipe. Until REC-02 a recipe is only its title (E-02).
///
/// Titles compare by `ingredients::name_key`, so "Shakshuka" and
/// "shakshuka " are the same title (D-39).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recipe {
    pub id: RecipeId,
    pub title: String,
}
