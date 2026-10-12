//! Test data for UI tests and demos (E-17).
use kreatyna::ingredients::{
    IngredientDraft, IngredientWrite, NutritionBasis, ReferenceUnitDraft,
    capabilities::{self, IngredientStoreMut},
};
use kreatyna_db::sqlite::{SqliteDatabase, SqliteStartupError};
use serde::Deserialize;
use serde_json::{Number, Value};

use crate::{AppInitializationError, IngredientSaveError, RUNTIME};

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum SeedError {
    #[error("database startup failed: {error:?}")]
    Startup { error: AppInitializationError },
    /// The text is not a JSON array of names and ingredient objects.
    #[error("invalid seed JSON: {message}")]
    InvalidJson { message: String },
    /// Item `index` (from 0) does not pass the ingredient form's rules.
    #[error("invalid ingredient {index}: {name:?}")]
    InvalidIngredient { index: u32, name: String },
    #[error("ingredient save failed: {error:?}")]
    Save { error: IngredientSaveError },
}

impl From<SqliteStartupError> for SeedError {
    fn from(error: SqliteStartupError) -> Self {
        Self::Startup {
            error: error.into(),
        }
    }
}

/// Saves the ingredients in the JSON text `json` into the database at
/// `database_path`.
///
/// The text is an array. An item is a name, or an object with the fields of
/// the ingredient form:
///
/// ```json
/// ["Cucumber", {"name": "Garlic", "calories": 149, "units": [{"name": "clove", "grams": 5}]}]
/// ```
///
/// Rust validates every item like the ingredient form does before it opens
/// the database, so a bad item saves nothing. Names that already exist are
/// skipped, so a relaunch with the same seed succeeds. Swift calls this only
/// in Debug builds, and only for an isolated test database. It blocks the
/// calling thread, which must not be a Tokio runtime thread.
#[uniffi::export]
pub fn seed_ingredients(database_path: String, json: String) -> Result<(), SeedError> {
    let writes = parse(&json)?;
    RUNTIME.block_on(async move {
        let mut database = SqliteDatabase::new(&database_path).await?;
        for write in writes {
            match database.save(write).await {
                Ok(_) | Err(capabilities::IngredientSaveError::DuplicateName) => {}
                Err(error) => {
                    return Err(SeedError::Save {
                        error: error.into(),
                    });
                }
            }
        }
        Ok(())
    })
}

fn parse(json: &str) -> Result<Vec<IngredientWrite>, SeedError> {
    let invalid = |message: String| SeedError::InvalidJson { message };
    let items: Vec<Value> =
        serde_json::from_str(json).map_err(|error| invalid(error.to_string()))?;
    items
        .into_iter()
        .enumerate()
        .map(|(index, item)| {
            // Each item is read alone, so an error names the item.
            let item = SeedItem::read(item)
                .map_err(|message| invalid(format!("item {index}: {message}")))?;
            let draft = IngredientDraft::from(item);
            IngredientWrite::try_from(&draft).map_err(|_| SeedError::InvalidIngredient {
                index: u32::try_from(index).unwrap_or(u32::MAX),
                name: draft.name,
            })
        })
        .collect()
}

enum SeedItem {
    Name(String),
    Ingredient(SeedIngredient),
}

impl SeedItem {
    /// Reads an object directly, so its error names the bad field.
    fn read(value: Value) -> Result<Self, String> {
        match value {
            Value::String(name) => Ok(Self::Name(name)),
            Value::Object(_) => SeedIngredient::deserialize(value)
                .map(Self::Ingredient)
                .map_err(|error| error.to_string()),
            other => Err(format!(
                "expected a name or an ingredient object, found {other}"
            )),
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SeedIngredient {
    name: String,
    #[serde(default)]
    notes: String,
    #[serde(default)]
    basis: SeedBasis,
    calories: Option<Number>,
    protein: Option<Number>,
    carbohydrates: Option<Number>,
    fat: Option<Number>,
    #[serde(default)]
    units: Vec<SeedUnit>,
}

#[derive(Default, Deserialize)]
enum SeedBasis {
    #[default]
    #[serde(rename = "100g")]
    Per100Grams,
    #[serde(rename = "100ml")]
    Per100Millilitres,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SeedUnit {
    name: String,
    grams: Option<Number>,
    millilitres: Option<Number>,
}

/// Numbers become form text, so the form's rules check them.
fn text(number: Option<Number>) -> String {
    number.map(|number| number.to_string()).unwrap_or_default()
}

impl From<SeedItem> for IngredientDraft {
    fn from(item: SeedItem) -> Self {
        let item = match item {
            SeedItem::Name(name) => {
                let mut draft = IngredientDraft::new(NutritionBasis::Per100Grams);
                draft.name = name;
                return draft;
            }
            SeedItem::Ingredient(item) => item,
        };
        let mut draft = IngredientDraft::new(match item.basis {
            SeedBasis::Per100Grams => NutritionBasis::Per100Grams,
            SeedBasis::Per100Millilitres => NutritionBasis::Per100Millilitres,
        });
        draft.name = item.name;
        draft.notes = item.notes;
        draft.calories = text(item.calories);
        draft.protein = text(item.protein);
        draft.carbohydrates = text(item.carbohydrates);
        draft.fat = text(item.fat);
        draft.units = item
            .units
            .into_iter()
            .map(|unit| ReferenceUnitDraft {
                name: unit.name,
                grams: text(unit.grams),
                millilitres: text(unit.millilitres),
            })
            .collect();
        draft
    }
}

#[cfg(test)]
mod tests;
