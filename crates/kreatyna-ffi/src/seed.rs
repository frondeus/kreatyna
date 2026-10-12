//! Test data for UI tests and demos (E-17).
use kreatyna::ingredients::{
    IngredientDraft, IngredientWrite, NutritionBasis,
    capabilities::{self, IngredientStoreMut},
};
use kreatyna_db::sqlite::{SqliteDatabase, SqliteStartupError};

use crate::{AppInitializationError, IngredientSaveError, RUNTIME};

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum SeedError {
    #[error("database startup failed: {error:?}")]
    Startup { error: AppInitializationError },
    #[error("invalid ingredient name: {name:?}")]
    InvalidName { name: String },
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

/// Saves name-only ingredients into the database at `database_path`.
///
/// Rust opens and migrates the database and validates each name like the
/// ingredient form does. Names that already exist are skipped, so a relaunch
/// with the same seed succeeds. Swift calls this only in Debug builds, and
/// only for an isolated test database. It blocks the calling thread, which
/// must not be a Tokio runtime thread.
#[uniffi::export]
pub fn seed_ingredients(database_path: String, names: Vec<String>) -> Result<(), SeedError> {
    RUNTIME.block_on(async move {
        let mut database = SqliteDatabase::new(&database_path).await?;
        for name in names {
            let mut draft = IngredientDraft::new(NutritionBasis::Per100Grams);
            draft.name.clone_from(&name);
            let write =
                IngredientWrite::try_from(&draft).map_err(|_| SeedError::InvalidName { name })?;
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

#[cfg(test)]
mod tests;
