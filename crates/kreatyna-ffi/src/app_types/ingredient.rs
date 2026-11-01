use kreatyna::ingredients as core;

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum NutritionBasis {
    Per100Grams,
    Per100Millilitres,
}

impl From<NutritionBasis> for core::NutritionBasis {
    fn from(value: NutritionBasis) -> Self {
        match value {
            NutritionBasis::Per100Grams => Self::Per100Grams,
            NutritionBasis::Per100Millilitres => Self::Per100Millilitres,
        }
    }
}

impl From<core::NutritionBasis> for NutritionBasis {
    fn from(value: core::NutritionBasis) -> Self {
        match value {
            core::NutritionBasis::Per100Grams => Self::Per100Grams,
            core::NutritionBasis::Per100Millilitres => Self::Per100Millilitres,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct Ingredient {
    pub id: core::IngredientId,
    pub name: String,
    pub notes: Option<String>,
    pub nutrition: Nutrition,
    pub units: Vec<ReferenceUnit>,
}

impl From<core::Ingredient> for Ingredient {
    fn from(value: core::Ingredient) -> Self {
        Self {
            id: value.id,
            name: value.name,
            notes: value.notes,
            nutrition: value.nutrition.into(),
            units: value.units.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct Nutrition {
    pub basis: NutritionBasis,
    pub calories: Option<String>,
    pub protein: Option<String>,
    pub carbohydrates: Option<String>,
    pub fat: Option<String>,
}

impl From<core::Nutrition> for Nutrition {
    fn from(value: core::Nutrition) -> Self {
        Self {
            basis: value.basis.into(),
            calories: value.calories.map(|quantity| quantity.to_string()),
            protein: value.protein.map(|quantity| quantity.to_string()),
            carbohydrates: value.carbohydrates.map(|quantity| quantity.to_string()),
            fat: value.fat.map(|quantity| quantity.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct ReferenceUnit {
    pub name: String,
    pub grams: Option<String>,
    pub millilitres: Option<String>,
}

impl From<core::ReferenceUnit> for ReferenceUnit {
    fn from(value: core::ReferenceUnit) -> Self {
        Self {
            name: value.name,
            grams: value.grams.map(|quantity| quantity.to_string()),
            millilitres: value.millilitres.map(|quantity| quantity.to_string()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum IngredientLoadError {
    Storage,
    Transport,
}

impl From<core::capabilities::IngredientLoadError> for IngredientLoadError {
    fn from(value: core::capabilities::IngredientLoadError) -> Self {
        match value {
            core::capabilities::IngredientLoadError::Storage => Self::Storage,
            core::capabilities::IngredientLoadError::Transport => Self::Transport,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum IngredientSaveError {
    DuplicateName,
    NotFound,
    Storage,
    Transport,
}

impl From<core::capabilities::IngredientSaveError> for IngredientSaveError {
    fn from(value: core::capabilities::IngredientSaveError) -> Self {
        match value {
            core::capabilities::IngredientSaveError::DuplicateName => Self::DuplicateName,
            core::capabilities::IngredientSaveError::NotFound => Self::NotFound,
            core::capabilities::IngredientSaveError::Storage => Self::Storage,
            core::capabilities::IngredientSaveError::Transport => Self::Transport,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum IngredientDeleteError {
    Storage,
    Transport,
}

impl From<core::capabilities::IngredientDeleteError> for IngredientDeleteError {
    fn from(value: core::capabilities::IngredientDeleteError) -> Self {
        match value {
            core::capabilities::IngredientDeleteError::Storage => Self::Storage,
            core::capabilities::IngredientDeleteError::Transport => Self::Transport,
        }
    }
}
