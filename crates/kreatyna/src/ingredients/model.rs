use nutype::nutype;
use unicode_casefold::UnicodeCaseFold;
use uuid::Uuid;

/// Stable ingredient identity. Creation is explicit and does not use a nil sentinel:
/// `generate` makes a version 4 identifier, and `new` wraps a known one.
#[nutype(derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Display, FromStr, From, Into
))]
pub struct IngredientId(Uuid);

impl IngredientId {
    /// Generate a new version 4 identifier.
    pub fn generate() -> Self {
        Self::new(Uuid::new_v4())
    }
}

macro_rules! quantity {
    ($(#[$doc:meta])* $name:ident) => {
        $(#[$doc])*
        #[nutype(derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Display, From, Into))]
        pub struct $name(Decimal);
    };
}

quantity!(
    /// Exact quantity in grams. The draft layer checks the sign: nutrition
    /// accepts zero, and a reference unit needs more than zero.
    ///
    /// Grams and millilitres do not mix:
    ///
    /// ```compile_fail
    /// use kreatyna::ingredients::{Grams, Millilitres};
    /// use rust_decimal::Decimal;
    /// let mass = Grams::new(Decimal::ONE);
    /// let volume: Millilitres = mass;
    /// ```
    Grams
);
quantity!(
    /// Exact quantity in millilitres. The draft layer checks the sign.
    Millilitres
);
quantity!(
    /// Exact energy in kilocalories. The draft layer checks the sign.
    Kilocalories
);

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
