use std::{fmt, str::FromStr};

use rust_decimal::Decimal;
use unicode_casefold::UnicodeCaseFold;
use uuid::Uuid;

/// Stable ingredient identity. Creation is explicit and does not use a nil sentinel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IngredientId(Uuid);

#[allow(clippy::new_without_default)] // Identity generation must be an explicit action.
impl IngredientId {
    /// Generate a new version 4 identifier.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl From<Uuid> for IngredientId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<IngredientId> for Uuid {
    fn from(value: IngredientId) -> Self {
        value.0
    }
}

impl FromStr for IngredientId {
    type Err = uuid::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        value.parse::<Uuid>().map(Self::from)
    }
}

impl fmt::Display for IngredientId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

macro_rules! quantity {
    ($name:ident, $unit:literal, $example:literal) => {
        #[doc = concat!("Exact quantity measured in ", $unit, ". Semantic validation belongs to the draft layer.\n", $example)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name(Decimal);

        impl From<Decimal> for $name {
            fn from(value: Decimal) -> Self {
                Self(value)
            }
        }

        impl From<$name> for Decimal {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

quantity!(
    Grams,
    "grams",
    "\n```compile_fail\nuse kreatyna::ingredients::{Grams, Millilitres};\nuse rust_decimal::Decimal;\nlet mass: Grams = Decimal::ONE.into();\nlet volume: Millilitres = mass;\n```"
);
quantity!(Millilitres, "millilitres", "");
quantity!(Kilocalories, "kilocalories", "");

/// Interpretation of nutrition values. Switching basis does not convert them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NutritionBasis {
    Per100Grams,
    Per100Millilitres,
}

/// Nutrition for the selected basis. `None` is unknown; `Some(0)` is known zero.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Nutrition {
    pub basis: NutritionBasis,
    pub calories: Option<Kilocalories>,
    pub protein: Option<Grams>,
    pub carbohydrates: Option<Grams>,
    pub fat: Option<Grams>,
}

/// Named approximate measurement, such as one clove or one can.
///
/// The draft layer validates that the name and at least one measurement exist.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReferenceUnit {
    pub name: String,
    pub grams: Option<Grams>,
    pub millilitres: Option<Millilitres>,
}

/// Catalog record. Fields here do not imply a validated write operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ingredient {
    pub id: IngredientId,
    pub name: String,
    pub notes: Option<String>,
    pub nutrition: Nutrition,
    pub units: Vec<ReferenceUnit>,
}

/// Shared search, sort, and uniqueness key for ingredient and unit names.
///
/// Trims surrounding Unicode whitespace and applies full, non-Turkic case folding.
/// Internal whitespace, accents, and punctuation remain significant.
pub fn name_key(name: &str) -> String {
    name.trim().case_fold().collect()
}
