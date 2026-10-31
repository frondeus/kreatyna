//! Transactional SQLite implementation of the recipe capability.
use diesel::{
    Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection,
};
use kreatyna::{
    ingredients::name_key,
    recipes::{
        Recipe,
        capabilities::{RecipeLoadError, RecipeStoreMut},
    },
};

use super::{recipe_models::RecipeRow, schema::recipes, sqlite::SqliteDatabase};

#[derive(Debug, thiserror::Error)]
enum StoreFailure {
    #[error(transparent)]
    Database(#[from] diesel::result::Error),
    #[error("stored recipe title key is inconsistent")]
    InvalidTitleKey,
}

fn records(connection: &mut SqliteConnection) -> Result<Vec<Recipe>, StoreFailure> {
    recipes::table
        // The unique index on `title_key` gives this order without a sort step.
        .order(recipes::title_key.asc())
        .select(RecipeRow::as_select())
        .load::<RecipeRow>(connection)?
        .into_iter()
        .map(|row| {
            if row.title_key != name_key(&row.title) {
                return Err(StoreFailure::InvalidTitleKey);
            }
            Ok(row.into_domain())
        })
        .collect()
}

#[async_trait::async_trait]
impl RecipeStoreMut for SqliteDatabase {
    #[tracing::instrument(skip_all)]
    async fn list(&mut self) -> Result<Vec<Recipe>, RecipeLoadError> {
        let result = self
            .connection
            .spawn_blocking(|connection| Ok(connection.transaction::<_, StoreFailure, _>(records)))
            .await;
        match result {
            Ok(Ok(records)) => Ok(records),
            error => {
                tracing::error!(?error, "Recipe list failed");
                Err(RecipeLoadError::Storage)
            }
        }
    }
}

#[cfg(test)]
mod tests;
