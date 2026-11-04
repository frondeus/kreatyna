//! Transactional SQLite implementation of the recipe capability.
use std::collections::HashMap;

use diesel::{
    Connection, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
    SqliteConnection,
    result::{DatabaseErrorKind, Error as DieselError},
};
use kreatyna::{
    ingredients::name_key,
    recipes::{
        ConnectRefusal, NewRecipe, Recipe, RecipeId, RecipeSummary, RecipeUpdate, Step, StepEdge,
        StepEdges, StepId, StepWrite,
        capabilities::{RecipeDeleteError, RecipeLoadError, RecipeSaveError, RecipeStoreMut},
    },
};

use super::{
    recipe_models::{
        LocalRecipeId, LocalStepId, NewRecipeRow, NewStepRow, RecipeRow, RemoteRecipeId,
        RemoteStepId, StepEdgeRow, StepOrdinal, StepRow, saved_step,
    },
    schema::{recipe_step_edges, recipe_steps, recipes},
    sqlite::SqliteDatabase,
};

#[derive(Debug, thiserror::Error)]
enum StoreFailure {
    #[error(transparent)]
    Database(#[from] DieselError),
    #[error("stored recipe title key is inconsistent")]
    InvalidTitleKey,
    #[error("stored step count does not fit")]
    InvalidStepCount,
    #[error("recipe title already exists")]
    DuplicateTitle,
    #[error("recipe to update was not found")]
    NotFound,
    #[error("an edge names a step that is not in the recipe")]
    EdgeOutsideRecipe,
    #[error("the stored edges break the step graph: {0}")]
    InvalidEdges(#[from] ConnectRefusal),
}

/// A summary of every recipe. One query reads the recipes, one counts the
/// steps of each.
fn summaries(connection: &mut SqliteConnection) -> Result<Vec<RecipeSummary>, StoreFailure> {
    let rows = recipes::table
        // The unique index on `title_key` gives this order without a sort step.
        .order(recipes::title_key.asc())
        .select(RecipeRow::as_select())
        .load::<RecipeRow>(connection)?;
    let counts: HashMap<LocalRecipeId, i64> = recipe_steps::table
        .group_by(recipe_steps::recipe_local_id)
        .select((recipe_steps::recipe_local_id, diesel::dsl::count_star()))
        .load::<(LocalRecipeId, i64)>(connection)?
        .into_iter()
        .collect();
    rows.into_iter()
        .map(|row| {
            let count = counts.get(&row.local_id).copied().unwrap_or(0);
            let step_count = u32::try_from(count).map_err(|_| StoreFailure::InvalidStepCount)?;
            let row = checked(row)?;
            Ok(RecipeSummary {
                id: row.remote_id.0,
                title: row.title,
                step_count,
            })
        })
        .collect()
}

fn checked(row: RecipeRow) -> Result<RecipeRow, StoreFailure> {
    if row.title_key != name_key(&row.title) {
        return Err(StoreFailure::InvalidTitleKey);
    }
    Ok(row)
}

/// The recipe with this id, its steps, and their edges. The unique index on
/// `remote_id` finds at most one.
fn record(connection: &mut SqliteConnection, id: RecipeId) -> Result<Option<Recipe>, StoreFailure> {
    let Some(row) = recipes::table
        .filter(recipes::remote_id.eq(RemoteRecipeId(id)))
        .select(RecipeRow::as_select())
        .first::<RecipeRow>(connection)
        .optional()?
    else {
        return Ok(None);
    };
    let row = checked(row)?;
    let rows = step_rows(connection, row.local_id)?;
    let step_ids: HashMap<LocalStepId, StepId> = rows
        .iter()
        .map(|step| (step.local_id, step.remote_id.0))
        .collect();
    let edges = edges(connection, &step_ids)?;
    let steps = rows.into_iter().map(StepRow::into_domain).collect();
    Ok(Some(row.into_domain(steps, edges)))
}

/// The step rows of the recipe row `recipe_local_id`, in the order added.
fn step_rows(
    connection: &mut SqliteConnection,
    recipe_local_id: LocalRecipeId,
) -> Result<Vec<StepRow>, StoreFailure> {
    // A coordinate that does not fit a `u32` fails the load (`DbCoordinate`).
    Ok(recipe_steps::table
        .filter(recipe_steps::recipe_local_id.eq(recipe_local_id))
        .order(recipe_steps::ordinal.asc())
        .select(StepRow::as_select())
        .load::<StepRow>(connection)?)
}

/// The edges that start at the steps `step_ids`. An edge that ends at a step
/// of another recipe fails with `EdgeOutsideRecipe`; a loop fails with
/// `InvalidEdges`.
fn edges(
    connection: &mut SqliteConnection,
    step_ids: &HashMap<LocalStepId, StepId>,
) -> Result<StepEdges, StoreFailure> {
    let rows = recipe_step_edges::table
        .filter(recipe_step_edges::from_local_id.eq_any(step_ids.keys()))
        .select(StepEdgeRow::as_select())
        .load::<StepEdgeRow>(connection)?;
    let edges = rows
        .into_iter()
        .map(|row| {
            let id = |local_id| step_ids.get(&local_id).copied();
            match (id(row.from_local_id), id(row.to_local_id)) {
                (Some(from), Some(to)) => Ok(StepEdge { from, to }),
                _ => Err(StoreFailure::EdgeOutsideRecipe),
            }
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(StepEdges::from_saved(edges)?)
}

/// Inserts `steps` for the recipe row `recipe_local_id`, with their order
/// added as `ordinal`. Returns them as saved.
fn insert_steps(
    connection: &mut SqliteConnection,
    recipe_local_id: LocalRecipeId,
    steps: &[StepWrite],
) -> Result<Vec<Step>, StoreFailure> {
    let rows: Vec<NewStepRow> = StepOrdinal::sequence()
        .zip(steps)
        .map(|(ordinal, step)| NewStepRow::new(recipe_local_id, ordinal, step))
        .collect();
    diesel::insert_into(recipe_steps::table)
        .values(&rows)
        .execute(connection)?;
    Ok(steps.iter().map(saved_step).collect())
}

/// Inserts `edges` between the steps of the recipe row `recipe_local_id`,
/// after `insert_steps`. An edge whose step the recipe does not have fails
/// with `EdgeOutsideRecipe`.
fn insert_edges(
    connection: &mut SqliteConnection,
    recipe_local_id: LocalRecipeId,
    edges: &StepEdges,
) -> Result<(), StoreFailure> {
    if edges.is_empty() {
        return Ok(());
    }
    let local_ids: HashMap<StepId, LocalStepId> = recipe_steps::table
        .filter(recipe_steps::recipe_local_id.eq(recipe_local_id))
        .select((recipe_steps::remote_id, recipe_steps::local_id))
        .load::<(RemoteStepId, LocalStepId)>(connection)?
        .into_iter()
        .map(|(remote_id, local_id)| (remote_id.0, local_id))
        .collect();
    let rows = edges
        .iter()
        .map(|edge| {
            let id = |step| local_ids.get(&step).copied();
            match (id(edge.from), id(edge.to)) {
                (Some(from_local_id), Some(to_local_id)) => Ok(StepEdgeRow {
                    from_local_id,
                    to_local_id,
                }),
                _ => Err(StoreFailure::EdgeOutsideRecipe),
            }
        })
        .collect::<Result<Vec<_>, _>>()?;
    diesel::insert_into(recipe_step_edges::table)
        .values(&rows)
        .execute(connection)?;
    Ok(())
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
        .first::<LocalRecipeId>(connection)
        .optional()?;
    if taken.is_some() {
        return Err(StoreFailure::DuplicateTitle);
    }
    insert(connection, &row)?;
    let local_id = recipes::table
        .filter(recipes::remote_id.eq(&row.remote_id))
        .select(recipes::local_id)
        .first::<LocalRecipeId>(connection)?;
    let steps = insert_steps(connection, local_id, &recipe.steps)?;
    insert_edges(connection, local_id, &recipe.edges)?;
    Ok(row.into_domain(steps, recipe.edges))
}

/// Runs inside an immediate transaction, like `create_in_transaction`. The
/// recipe's own title key is not a conflict, so "Shakshuka" can become
/// "shakshuka". The new steps and edges replace the old ones; a step keeps
/// its id.
fn update_in_transaction(
    connection: &mut SqliteConnection,
    id: RecipeId,
    recipe: RecipeUpdate,
) -> Result<Recipe, StoreFailure> {
    let row = NewRecipeRow::new(id, &recipe.title);
    let Some(local_id) = recipes::table
        .filter(recipes::remote_id.eq(&row.remote_id))
        .select(recipes::local_id)
        .first::<LocalRecipeId>(connection)
        .optional()?
    else {
        return Err(StoreFailure::NotFound);
    };
    let taken = recipes::table
        .filter(recipes::title_key.eq(&row.title_key))
        .filter(recipes::local_id.ne(local_id))
        .select(recipes::local_id)
        .first::<LocalRecipeId>(connection)
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
    // The cascade deletes the old steps' edges too.
    diesel::delete(recipe_steps::table.filter(recipe_steps::recipe_local_id.eq(local_id)))
        .execute(connection)?;
    let steps = insert_steps(connection, local_id, &recipe.steps)?;
    insert_edges(connection, local_id, &recipe.edges)?;
    Ok(row.into_domain(steps, recipe.edges))
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
    async fn list(&mut self) -> Result<Vec<RecipeSummary>, RecipeLoadError> {
        let result = self
            .connection
            .spawn_blocking(
                |connection| Ok(connection.transaction::<_, StoreFailure, _>(summaries)),
            )
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
    /// owns (its steps, REC-14) go with it through `ON DELETE CASCADE`.
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
