//! Transactional SQLite implementation of the recipe capability.
use diesel::{
    Connection, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
    SqliteConnection,
    result::{DatabaseErrorKind, Error as DieselError},
};
use kreatyna::{
    ingredients::name_key,
    recipes::{
        NewRecipe, Recipe, RecipeId, RecipeUpdate,
        capabilities::{RecipeDeleteError, RecipeLoadError, RecipeSaveError, RecipeStoreMut},
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
    #[error("recipe to update was not found")]
    NotFound,
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
    let row = NewRecipeRow::new(RecipeId::generate(), &recipe.title);
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

/// Runs inside an immediate transaction, like `create_in_transaction`. The
/// recipe's own title key is not a conflict, so "Shakshuka" can become
/// "shakshuka".
fn update_in_transaction(
    connection: &mut SqliteConnection,
    id: RecipeId,
    recipe: RecipeUpdate,
) -> Result<Recipe, StoreFailure> {
    let row = NewRecipeRow::new(id, &recipe.title);
    let Some(local_id) = recipes::table
        .filter(recipes::remote_id.eq(&row.remote_id))
        .select(recipes::local_id)
        .first::<i64>(connection)
        .optional()?
    else {
        return Err(StoreFailure::NotFound);
    };
    let taken = recipes::table
        .filter(recipes::title_key.eq(&row.title_key))
        .filter(recipes::local_id.ne(local_id))
        .select(recipes::local_id)
        .first::<i64>(connection)
        .optional()?;
    if taken.is_some() {
        return Err(StoreFailure::DuplicateTitle);
    }
    let write = diesel::update(recipes::table.filter(recipes::local_id.eq(local_id)))
        .set((
            recipes::title.eq(&row.title),
            recipes::title_key.eq(&row.title_key),
        ))
        .execute(connection);
    duplicate_title(write)?;
    Ok(row.into_domain())
}

/// A taken title key fails with `DuplicateTitle`, also when the unique index
/// finds it and not the check before.
fn insert(connection: &mut SqliteConnection, row: &NewRecipeRow) -> Result<(), StoreFailure> {
    let write = diesel::insert_into(recipes::table)
        .values(row)
        .execute(connection);
    duplicate_title(write)
}

/// Maps a unique-index failure on `title_key` to `DuplicateTitle`.
fn duplicate_title(write: Result<usize, DieselError>) -> Result<(), StoreFailure> {
    match write {
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

    #[tracing::instrument(skip_all)]
    async fn update(
        &mut self,
        id: RecipeId,
        recipe: RecipeUpdate,
    ) -> Result<Recipe, RecipeSaveError> {
        let result = self
            .connection
            .spawn_blocking(move |connection| {
                Ok(connection.immediate_transaction(|connection| {
                    update_in_transaction(connection, id, recipe)
                }))
            })
            .await;
        match result {
            Ok(Ok(recipe)) => Ok(recipe),
            Ok(Err(StoreFailure::DuplicateTitle)) => Err(RecipeSaveError::DuplicateTitle),
            Ok(Err(StoreFailure::NotFound)) => Err(RecipeSaveError::NotFound),
            error => {
                tracing::error!(?error, "Recipe update failed");
                Err(RecipeSaveError::Storage)
            }
        }
    }

    /// One `DELETE` by id. Zero deleted rows is a success. Rows that a recipe
    /// owns (REC-07, REC-14) go with it through `ON DELETE CASCADE`.
    #[tracing::instrument(skip_all)]
    async fn delete(&mut self, id: RecipeId) -> Result<(), RecipeDeleteError> {
        let result = self
            .connection
            .spawn_blocking(move |connection| {
                Ok(
                    connection.immediate_transaction::<_, DieselError, _>(|connection| {
                        diesel::delete(
                            recipes::table.filter(recipes::remote_id.eq(RemoteRecipeId(id))),
                        )
                        .execute(connection)?;
                        Ok(())
                    }),
                )
            })
            .await;
        match result {
            Ok(Ok(())) => Ok(()),
            error => {
                tracing::error!(?error, "Recipe delete failed");
                Err(RecipeDeleteError::Storage)
            }
        }
    }
}

#[cfg(test)]
mod tests;
