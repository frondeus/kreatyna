//! Transactional SQLite implementation of the recipe capability.
use diesel::{
    Connection, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
    SqliteConnection,
    result::{DatabaseErrorKind, Error as DieselError},
};
use kreatyna::{
    ingredients::name_key,
    recipes::{
        NewRecipe, Recipe, RecipeId,
        capabilities::{RecipeLoadError, RecipeSaveError, RecipeStoreMut},
    },
};

use super::{
    recipe_models::{NewRecipeRow, RecipeRow, RemoteRecipeId},
    schema::recipes,
    sqlite::SqliteDatabase,
};

#[derive(Debug, thiserror::Error)]
enum StoreFailure {
    #[error(transparent)]
    Database(#[from] DieselError),
    #[error("stored recipe title key is inconsistent")]
    InvalidTitleKey,
    #[error("recipe title already exists")]
    DuplicateTitle,
}

fn records(connection: &mut SqliteConnection) -> Result<Vec<Recipe>, StoreFailure> {
    recipes::table
        // The unique index on `title_key` gives this order without a sort step.
        .order(recipes::title_key.asc())
        .select(RecipeRow::as_select())
        .load::<RecipeRow>(connection)?
        .into_iter()
        .map(checked)
        .collect()
}

fn checked(row: RecipeRow) -> Result<Recipe, StoreFailure> {
    if row.title_key != name_key(&row.title) {
        return Err(StoreFailure::InvalidTitleKey);
    }
    Ok(row.into_domain())
}

/// The recipe with this id. The unique index on `remote_id` finds at most one.
fn record(connection: &mut SqliteConnection, id: RecipeId) -> Result<Option<Recipe>, StoreFailure> {
    recipes::table
        .filter(recipes::remote_id.eq(RemoteRecipeId(id)))
        .select(RecipeRow::as_select())
        .first::<RecipeRow>(connection)
        .optional()?
        .map(checked)
        .transpose()
}

/// Runs inside an immediate transaction, so the check and the insert see the
/// same rows. The unique index on `title_key` still guards the insert.
fn create_in_transaction(
    connection: &mut SqliteConnection,
    recipe: NewRecipe,
) -> Result<Recipe, StoreFailure> {
    let row = NewRecipeRow::new(RecipeId::new(), &recipe.title);
    let taken = recipes::table
        .filter(recipes::title_key.eq(&row.title_key))
        .select(recipes::local_id)
        .first::<i64>(connection)
        .optional()?;
    if taken.is_some() {
        return Err(StoreFailure::DuplicateTitle);
    }
    insert(connection, &row)?;
    Ok(row.into_domain())
}

/// A taken title key fails with `DuplicateTitle`, also when the unique index
/// finds it and not the check before.
fn insert(connection: &mut SqliteConnection, row: &NewRecipeRow) -> Result<(), StoreFailure> {
    match diesel::insert_into(recipes::table)
        .values(row)
        .execute(connection)
    {
        Ok(_) => Ok(()),
        Err(DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, info))
            if info.message().contains("recipes.title_key") =>
        {
            Err(StoreFailure::DuplicateTitle)
        }
        Err(error) => Err(error.into()),
    }
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

    #[tracing::instrument(skip_all)]
    async fn get(&mut self, id: RecipeId) -> Result<Option<Recipe>, RecipeLoadError> {
        let result = self
            .connection
            .spawn_blocking(move |connection| {
                Ok(connection
                    .transaction::<_, StoreFailure, _>(|connection| record(connection, id)))
            })
            .await;
        match result {
            Ok(Ok(recipe)) => Ok(recipe),
            error => {
                tracing::error!(?error, "Recipe get failed");
                Err(RecipeLoadError::Storage)
            }
        }
    }

    #[tracing::instrument(skip_all)]
    async fn create(&mut self, recipe: NewRecipe) -> Result<Recipe, RecipeSaveError> {
        let result = self
            .connection
            .spawn_blocking(move |connection| {
                Ok(connection
                    .immediate_transaction(|connection| create_in_transaction(connection, recipe)))
            })
            .await;
        match result {
            Ok(Ok(recipe)) => Ok(recipe),
            Ok(Err(StoreFailure::DuplicateTitle)) => Err(RecipeSaveError::DuplicateTitle),
            error => {
                tracing::error!(?error, "Recipe create failed");
                Err(RecipeSaveError::Storage)
            }
        }
    }
}

#[cfg(test)]
mod tests;
