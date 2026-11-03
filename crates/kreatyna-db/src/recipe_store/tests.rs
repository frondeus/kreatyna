use anyhow::{Context, anyhow};
use diesel::{Connection, RunQueryDsl, SqliteConnection, connection::SimpleConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use kreatyna::{
    ingredients::capabilities::IngredientStoreMut,
    recipes::{
        CanvasPoint, NewRecipe, Recipe, RecipeId, RecipeSummary, RecipeTitle, RecipeUpdate, Step,
        StepId, StepTitle, StepWrite,
    },
};
use test_case::test_case;

use super::*;

// Green path

#[tokio::test]
async fn an_empty_database_lists_no_recipes() -> anyhow::Result<()> {
    let mut db = SqliteDatabase::new(":memory:").await?;
    assert_eq!(RecipeStoreMut::list(&mut db).await, Ok(vec![]));
    Ok(())
}

#[tokio::test]
async fn stored_recipes_are_listed_by_title_key_with_their_titles_as_written() -> anyhow::Result<()>
{
    let mut db = SqliteDatabase::new(":memory:").await?;
    // Shakshuka is inserted first, so the order comes from the title key.
    execute(&mut db, TWO_RECIPES).await?;
    assert_eq!(
        RecipeStoreMut::list(&mut db).await,
        Ok(vec![
            summary(PASTA, "Roasted tomato pasta", 0)?,
            summary(SHAKSHUKA, "Shakshuka", 0)?,
        ])
    );
    Ok(())
}

#[tokio::test]
async fn a_created_recipe_has_a_new_id_and_its_trimmed_title() -> anyhow::Result<()> {
    let mut db = SqliteDatabase::new(":memory:").await?;
    let created = RecipeStoreMut::create(&mut db, new_recipe("  Shakshuka ")?).await?;
    assert_eq!(created.title, "Shakshuka");
    assert_eq!(
        RecipeStoreMut::list(&mut db).await,
        Ok(vec![summary_of(&created)])
    );
    Ok(())
}

#[tokio::test]
async fn created_recipes_are_listed_by_title_key() -> anyhow::Result<()> {
    let mut db = SqliteDatabase::new(":memory:").await?;
    let shakshuka = RecipeStoreMut::create(&mut db, new_recipe("Shakshuka")?).await?;
    let pasta = RecipeStoreMut::create(&mut db, new_recipe("roasted tomato pasta")?).await?;
    assert_ne!(shakshuka.id, pasta.id);
    assert_eq!(
        RecipeStoreMut::list(&mut db).await,
        Ok(vec![summary_of(&pasta), summary_of(&shakshuka)])
    );
    Ok(())
}

#[tokio::test]
async fn get_returns_a_created_recipe() -> anyhow::Result<()> {
    let mut db = SqliteDatabase::new(":memory:").await?;
    let created = RecipeStoreMut::create(&mut db, new_recipe("Shakshuka")?).await?;
    assert_eq!(
        RecipeStoreMut::get(&mut db, created.id).await,
        Ok(Some(created))
    );
    Ok(())
}

#[tokio::test]
async fn get_finds_the_recipe_by_id_among_others() -> anyhow::Result<()> {
    let mut db = SqliteDatabase::new(":memory:").await?;
    execute(&mut db, TWO_RECIPES).await?;
    assert_eq!(
        RecipeStoreMut::get(&mut db, PASTA.parse()?).await,
        Ok(Some(recipe(PASTA, "Roasted tomato pasta")?))
    );
    Ok(())
}

#[tokio::test]
async fn an_update_renames_the_recipe_and_keeps_its_id() -> anyhow::Result<()> {
    let mut db = SqliteDatabase::new(":memory:").await?;
    execute(&mut db, TWO_RECIPES).await?;
    let updated =
        RecipeStoreMut::update(&mut db, SHAKSHUKA.parse()?, update(" Apple shakshuka ")?).await?;
    assert_eq!(updated, recipe(SHAKSHUKA, "Apple shakshuka")?);
    assert_eq!(
        RecipeStoreMut::get(&mut db, SHAKSHUKA.parse()?).await,
        Ok(Some(updated.clone()))
    );
    // The new title key sorts it first.
    assert_eq!(
        RecipeStoreMut::list(&mut db).await,
        Ok(vec![
            summary_of(&updated),
            summary(PASTA, "Roasted tomato pasta", 0)?
        ])
    );
    Ok(())
}

#[test_case("Shakshuka"; "same title")]
#[test_case("SHAKSHUKA"; "other case")]
#[test_case("shakshuka "; "same title key")]
#[tokio::test]
async fn an_update_may_keep_the_recipes_own_title_key(title: &str) -> anyhow::Result<()> {
    let mut db = SqliteDatabase::new(":memory:").await?;
    execute(&mut db, TWO_RECIPES).await?;
    let updated = RecipeStoreMut::update(&mut db, SHAKSHUKA.parse()?, update(title)?).await?;
    assert_eq!(updated, recipe(SHAKSHUKA, title.trim())?);
    assert_eq!(RecipeStoreMut::list(&mut db).await?.len(), 2);
    Ok(())
}

#[tokio::test]
async fn a_created_recipe_keeps_its_steps_in_the_order_added() -> anyhow::Result<()> {
    let mut db = SqliteDatabase::new(":memory:").await?;
    let steps = vec![
        step(1, "Fry onion", 200, 40)?,
        step(2, "Dice onion", 10, 20)?,
    ];
    let created = RecipeStoreMut::create(&mut db, with_steps("Shakshuka", steps.clone())?).await?;
    assert_eq!(created.steps, saved(&steps));
    assert_eq!(
        RecipeStoreMut::get(&mut db, created.id).await,
        Ok(Some(created.clone()))
    );
    assert_eq!(
        RecipeStoreMut::list(&mut db).await,
        Ok(vec![summary_of(&created)])
    );
    Ok(())
}

#[tokio::test]
async fn the_list_counts_the_steps_of_each_recipe() -> anyhow::Result<()> {
    let mut db = SqliteDatabase::new(":memory:").await?;
    let three = vec![
        step(1, "Boil water", 0, 0)?,
        step(2, "Cook pasta", 0, 100)?,
        step(3, "Drain", 0, 200)?,
    ];
    RecipeStoreMut::create(&mut db, with_steps("Pasta", three)?).await?;
    RecipeStoreMut::create(
        &mut db,
        with_steps("Shakshuka", vec![step(4, "Fry", 0, 0)?])?,
    )
    .await?;
    RecipeStoreMut::create(&mut db, new_recipe("Toast")?).await?;
    let counts: Vec<_> = RecipeStoreMut::list(&mut db)
        .await?
        .into_iter()
        .map(|summary| (summary.title, summary.step_count))
        .collect();
    assert_eq!(
        counts,
        [
            ("Pasta".to_owned(), 3),
            ("Shakshuka".to_owned(), 1),
            ("Toast".to_owned(), 0)
        ]
    );
    Ok(())
}

#[tokio::test]
async fn an_update_replaces_the_steps_and_a_kept_step_keeps_its_id() -> anyhow::Result<()> {
    let mut db = SqliteDatabase::new(":memory:").await?;
    let dice = step(1, "Dice onion", 10, 20)?;
    let fry = step(2, "Fry onion", 200, 40)?;
    let created =
        RecipeStoreMut::create(&mut db, with_steps("Shakshuka", vec![dice, fry])?).await?;

    // Dice goes, Fry gets a new title and place, Serve is new.
    let fry = step(2, "Fry onion and garlic", 220, 60)?;
    let serve = step(3, "Serve", 400, 60)?;
    let steps = vec![fry, serve];
    let update = RecipeUpdate {
        title: RecipeTitle::try_new("Shakshuka")?,
        steps: steps.clone(),
    };
    let updated = RecipeStoreMut::update(&mut db, created.id, update).await?;
    assert_eq!(updated.steps, saved(&steps));
    assert_eq!(
        RecipeStoreMut::get(&mut db, created.id).await,
        Ok(Some(updated))
    );
    assert_eq!(step_rows(&mut db).await?, 2);
    Ok(())
}

#[tokio::test]
async fn a_deleted_recipe_takes_its_steps_with_it() -> anyhow::Result<()> {
    let mut db = SqliteDatabase::new(":memory:").await?;
    let shakshuka = RecipeStoreMut::create(
        &mut db,
        with_steps("Shakshuka", vec![step(1, "Fry", 0, 0)?])?,
    )
    .await?;
    RecipeStoreMut::create(&mut db, with_steps("Pasta", vec![step(2, "Boil", 0, 0)?])?).await?;
    RecipeStoreMut::delete(&mut db, shakshuka.id).await?;
    assert_eq!(step_rows(&mut db).await?, 1);
    Ok(())
}

#[tokio::test]
async fn a_recipe_from_before_rec_14_upgrades_with_no_steps() -> anyhow::Result<()> {
    let folder = tempfile::tempdir()?;
    let path = folder.path().join("app.sqlite");
    let path = path.to_str().context("the temporary path is not UTF-8")?;
    {
        let mut db = SqliteConnection::establish(path)?;
        // The schema before REC-14: up to and with the recipes table.
        for _ in 0..4 {
            db.run_next_migration(MIGRATIONS)
                .map_err(|error| anyhow!(error))?;
        }
        db.batch_execute(TWO_RECIPES)?;
    }
    let mut db = SqliteDatabase::new(path).await?;
    assert_eq!(
        RecipeStoreMut::list(&mut db).await,
        Ok(vec![
            summary(PASTA, "Roasted tomato pasta", 0)?,
            summary(SHAKSHUKA, "Shakshuka", 0)?,
        ])
    );
    assert_eq!(
        RecipeStoreMut::get(&mut db, SHAKSHUKA.parse()?).await,
        Ok(Some(recipe(SHAKSHUKA, "Shakshuka")?))
    );
    Ok(())
}

#[tokio::test]
async fn a_catalog_from_before_rec_01_upgrades_and_keeps_its_ingredients() -> anyhow::Result<()> {
    let folder = tempfile::tempdir()?;
    let path = folder.path().join("app.sqlite");
    let path = path.to_str().context("the temporary path is not UTF-8")?;
    {
        let mut db = SqliteConnection::establish(path)?;
        // The schema before REC-01: counter, ingredients, and the counter drop.
        for _ in 0..3 {
            db.run_next_migration(MIGRATIONS)
                .map_err(|error| anyhow!(error))?;
        }
        db.batch_execute(
            "INSERT INTO ingredients (remote_id, name, name_key, basis)
             VALUES ('6f1c1d3e-0000-4000-8000-0000000000aa', 'Garlic', 'garlic', 0)",
        )?;
    }
    {
        let mut db = SqliteDatabase::new(path).await?;
        let ingredients = IngredientStoreMut::list(&mut db).await?;
        assert_eq!(
            ingredients
                .iter()
                .map(|ingredient| ingredient.name.as_ref())
                .collect::<Vec<_>>(),
            ["Garlic"]
        );
        assert_eq!(RecipeStoreMut::list(&mut db).await, Ok(vec![]));
    }
    // A second start finds no pending migration and changes nothing.
    let mut db = SqliteDatabase::new(path).await?;
    assert_eq!(IngredientStoreMut::list(&mut db).await?.len(), 1);
    Ok(())
}

// Validation

#[test_case("INSERT INTO recipes (remote_id, title, title_key) VALUES ('6f1c1d3e-0000-4000-8000-000000000003', '  ', ''); "; "blank title")]
#[test_case("INSERT INTO recipes (remote_id, title, title_key) VALUES ('6f1c1d3e-0000-4000-8000-000000000003', 'shakshuka ', 'shakshuka'); "; "duplicate title key")]
#[test_case("INSERT INTO recipes (remote_id, title, title_key) VALUES ('6f1c1d3e-0000-4000-8000-000000000001', 'Pancakes', 'pancakes'); "; "duplicate id")]
fn the_table_refuses_invalid_rows(sql: &str) -> anyhow::Result<()> {
    let mut db = migrated_connection()?;
    assert!(db.batch_execute(sql).is_err());
    Ok(())
}

#[test_case("'  ', 0, 0, 0"; "blank title")]
#[test_case("'Fry', 0, -1, 0"; "negative x")]
#[test_case("'Fry', 0, 0, -1"; "negative y")]
#[test_case("'Fry', -1, 0, 0"; "negative ordinal")]
fn the_step_table_refuses_invalid_rows(values: &str) -> anyhow::Result<()> {
    let mut db = migrated_connection()?;
    let sql = format!(
        "INSERT INTO recipe_steps (remote_id, recipe_local_id, title, ordinal, x, y)
         VALUES ('{STEP}', (SELECT local_id FROM recipes WHERE title = 'Shakshuka'), {values})"
    );
    assert!(db.batch_execute(&sql).is_err());
    Ok(())
}

#[test_case("'6f1c1d3e-0000-4000-8000-0000000000ff', 0"; "same ordinal")]
#[test_case("'6f1c1d3e-0000-4000-8000-0000000000f1', 1"; "same id")]
fn the_step_table_refuses_a_second_step_with_the_same(values: &str) -> anyhow::Result<()> {
    let mut db = migrated_connection()?;
    let insert = |id_and_ordinal: &str| {
        format!(
            "INSERT INTO recipe_steps (remote_id, ordinal, recipe_local_id, title, x, y)
             VALUES ({id_and_ordinal},
                     (SELECT local_id FROM recipes WHERE title = 'Shakshuka'), 'Fry', 0, 0)"
        )
    };
    db.batch_execute(&insert("'6f1c1d3e-0000-4000-8000-0000000000f1', 0"))?;
    assert!(db.batch_execute(&insert(values)).is_err());
    Ok(())
}

#[tokio::test]
async fn a_step_id_of_another_recipe_fails_the_save_and_changes_nothing() -> anyhow::Result<()> {
    let mut db = SqliteDatabase::new(":memory:").await?;
    RecipeStoreMut::create(
        &mut db,
        with_steps("Shakshuka", vec![step(1, "Fry", 0, 0)?])?,
    )
    .await?;
    assert_eq!(
        RecipeStoreMut::create(&mut db, with_steps("Pasta", vec![step(1, "Boil", 0, 0)?])?).await,
        Err(RecipeSaveError::Storage)
    );
    // The transaction rolled back the recipe row too.
    assert_eq!(RecipeStoreMut::list(&mut db).await?.len(), 1);
    assert_eq!(step_rows(&mut db).await?, 1);
    Ok(())
}

#[test_case("Shakshuka"; "same title")]
#[test_case("shakshuka "; "same title key")]
#[test_case("SHAKSHUKA"; "other case")]
#[tokio::test]
async fn a_taken_title_is_refused_and_changes_nothing(title: &str) -> anyhow::Result<()> {
    let mut db = SqliteDatabase::new(":memory:").await?;
    execute(&mut db, TWO_RECIPES).await?;
    assert_eq!(
        RecipeStoreMut::create(&mut db, new_recipe(title)?).await,
        Err(RecipeSaveError::DuplicateTitle)
    );
    assert_eq!(RecipeStoreMut::list(&mut db).await?.len(), 2);
    Ok(())
}

/// Another writer can insert between a check and an insert when they are not
/// in one immediate transaction. The unique index catches it.
#[test]
fn a_title_key_caught_by_the_unique_index_is_a_duplicate() -> anyhow::Result<()> {
    let mut db = migrated_connection()?;
    let row = NewRecipeRow::new(RecipeId::generate(), &RecipeTitle::try_new("shakshuka")?);
    assert!(matches!(
        insert(&mut db, &row),
        Err(StoreFailure::DuplicateTitle)
    ));
    Ok(())
}

#[test]
fn a_taken_id_is_a_storage_failure_not_a_duplicate() -> anyhow::Result<()> {
    let mut db = migrated_connection()?;
    let row = NewRecipeRow::new(SHAKSHUKA.parse()?, &RecipeTitle::try_new("Pancakes")?);
    assert!(matches!(
        insert(&mut db, &row),
        Err(StoreFailure::Database(_))
    ));
    Ok(())
}

#[test_case("Roasted tomato pasta"; "same title")]
#[test_case("roasted TOMATO pasta "; "same title key")]
#[tokio::test]
async fn an_update_to_another_recipes_title_is_refused_and_changes_nothing(
    title: &str,
) -> anyhow::Result<()> {
    let mut db = SqliteDatabase::new(":memory:").await?;
    execute(&mut db, TWO_RECIPES).await?;
    assert_eq!(
        RecipeStoreMut::update(&mut db, SHAKSHUKA.parse()?, update(title)?).await,
        Err(RecipeSaveError::DuplicateTitle)
    );
    assert_eq!(
        RecipeStoreMut::list(&mut db).await,
        Ok(vec![
            summary(PASTA, "Roasted tomato pasta", 0)?,
            summary(SHAKSHUKA, "Shakshuka", 0)?,
        ])
    );
    Ok(())
}

#[tokio::test]
async fn an_update_of_an_unknown_id_is_not_found_and_inserts_nothing() -> anyhow::Result<()> {
    let mut db = SqliteDatabase::new(":memory:").await?;
    execute(&mut db, TWO_RECIPES).await?;
    let unknown = RecipeId::generate();
    assert_eq!(
        RecipeStoreMut::update(&mut db, unknown, update("Pancakes")?).await,
        Err(RecipeSaveError::NotFound)
    );
    assert_eq!(RecipeStoreMut::get(&mut db, unknown).await, Ok(None));
    assert_eq!(RecipeStoreMut::list(&mut db).await?.len(), 2);
    Ok(())
}

#[tokio::test]
async fn a_deleted_recipe_is_gone_and_the_others_stay() -> anyhow::Result<()> {
    let mut db = SqliteDatabase::new(":memory:").await?;
    execute(&mut db, TWO_RECIPES).await?;
    RecipeStoreMut::delete(&mut db, SHAKSHUKA.parse()?).await?;
    assert_eq!(
        RecipeStoreMut::get(&mut db, SHAKSHUKA.parse()?).await,
        Ok(None)
    );
    assert_eq!(
        RecipeStoreMut::list(&mut db).await,
        Ok(vec![summary(PASTA, "Roasted tomato pasta", 0)?])
    );
    Ok(())
}

#[tokio::test]
async fn a_delete_of_an_unknown_id_succeeds_and_changes_nothing() -> anyhow::Result<()> {
    let mut db = SqliteDatabase::new(":memory:").await?;
    execute(&mut db, TWO_RECIPES).await?;
    assert_eq!(
        RecipeStoreMut::delete(&mut db, RecipeId::generate()).await,
        Ok(())
    );
    assert_eq!(RecipeStoreMut::list(&mut db).await?.len(), 2);
    Ok(())
}

#[tokio::test]
async fn a_deleted_recipes_title_is_free_for_a_new_recipe() -> anyhow::Result<()> {
    let mut db = SqliteDatabase::new(":memory:").await?;
    execute(&mut db, TWO_RECIPES).await?;
    RecipeStoreMut::delete(&mut db, SHAKSHUKA.parse()?).await?;
    let created = RecipeStoreMut::create(&mut db, new_recipe("shakshuka")?).await?;
    assert_ne!(created.id, SHAKSHUKA.parse()?);
    assert_eq!(created.title, "shakshuka");
    Ok(())
}

#[tokio::test]
async fn get_of_an_unknown_id_is_none() -> anyhow::Result<()> {
    let mut db = SqliteDatabase::new(":memory:").await?;
    execute(&mut db, TWO_RECIPES).await?;
    assert_eq!(
        RecipeStoreMut::get(&mut db, RecipeId::generate()).await,
        Ok(None)
    );
    Ok(())
}

// Red cases

#[test_case("UPDATE recipes SET title_key = 'wrong' WHERE title = 'Shakshuka'"; "inconsistent title key")]
#[test_case("UPDATE recipes SET remote_id = 'not-a-uuid' WHERE title = 'Shakshuka'"; "invalid id")]
#[tokio::test]
async fn a_corrupted_recipe_fails_the_whole_list(sql: &'static str) -> anyhow::Result<()> {
    let mut db = SqliteDatabase::new(":memory:").await?;
    execute(&mut db, TWO_RECIPES).await?;
    execute(&mut db, sql).await?;
    assert_eq!(
        RecipeStoreMut::list(&mut db).await,
        Err(RecipeLoadError::Storage)
    );
    Ok(())
}

#[tokio::test]
async fn get_of_a_recipe_with_an_inconsistent_title_key_is_a_storage_failure() -> anyhow::Result<()>
{
    let mut db = SqliteDatabase::new(":memory:").await?;
    execute(&mut db, TWO_RECIPES).await?;
    execute(
        &mut db,
        "UPDATE recipes SET title_key = 'wrong' WHERE title = 'Shakshuka'",
    )
    .await?;
    assert_eq!(
        RecipeStoreMut::get(&mut db, SHAKSHUKA.parse()?).await,
        Err(RecipeLoadError::Storage)
    );
    // The other recipe still opens.
    assert!(
        RecipeStoreMut::get(&mut db, PASTA.parse()?)
            .await?
            .is_some()
    );
    Ok(())
}

#[test_case("x", 4_294_967_296; "x over u32")]
#[test_case("y", 4_294_967_296; "y over u32")]
#[tokio::test]
async fn a_step_position_that_does_not_fit_the_canvas_fails_get(
    axis: &str,
    value: i64,
) -> anyhow::Result<()> {
    let mut db = SqliteDatabase::new(":memory:").await?;
    let created = RecipeStoreMut::create(
        &mut db,
        with_steps("Shakshuka", vec![step(1, "Fry", 0, 0)?])?,
    )
    .await?;
    let sql = format!("UPDATE recipe_steps SET {axis} = {value}");
    db.connection
        .spawn_blocking(move |conn| diesel::sql_query(sql).execute(conn).map(|_| ()))
        .await?;
    assert_eq!(
        RecipeStoreMut::get(&mut db, created.id).await,
        Err(RecipeLoadError::Storage)
    );
    Ok(())
}

// Test support

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

const SHAKSHUKA: &str = "6f1c1d3e-0000-4000-8000-000000000001";
const PASTA: &str = "6f1c1d3e-0000-4000-8000-000000000002";

/// Two recipes, written as the store writes them.
const TWO_RECIPES: &str = "INSERT INTO recipes (remote_id, title, title_key) VALUES
    ('6f1c1d3e-0000-4000-8000-000000000001', 'Shakshuka', 'shakshuka'),
    ('6f1c1d3e-0000-4000-8000-000000000002', 'Roasted tomato pasta', 'roasted tomato pasta')";

/// A step id no test row uses.
const STEP: &str = "6f1c1d3e-0000-4000-8000-0000000000ee";

fn summary(id: &str, title: &str, step_count: u32) -> anyhow::Result<RecipeSummary> {
    Ok(RecipeSummary {
        id: id.parse()?,
        title: title.into(),
        step_count,
    })
}

fn summary_of(recipe: &Recipe) -> RecipeSummary {
    RecipeSummary {
        id: recipe.id,
        title: recipe.title.clone(),
        step_count: u32::try_from(recipe.steps.len()).unwrap_or(u32::MAX),
    }
}

fn step(id: u128, title: &str, x: u32, y: u32) -> anyhow::Result<StepWrite> {
    Ok(StepWrite {
        id: format!("6f1c1d3e-0000-4000-8000-{id:012x}").parse::<StepId>()?,
        title: StepTitle::try_new(title)?,
        position: CanvasPoint { x, y },
    })
}

/// The steps as `get` returns them after a save of `steps`.
fn saved(steps: &[StepWrite]) -> Vec<Step> {
    steps
        .iter()
        .map(|step| Step {
            id: step.id,
            title: step.title.as_ref().to_owned(),
            position: step.position,
        })
        .collect()
}

fn with_steps(title: &str, steps: Vec<StepWrite>) -> anyhow::Result<NewRecipe> {
    Ok(NewRecipe {
        title: RecipeTitle::try_new(title)?,
        steps,
    })
}

/// How many step rows the database has, over all recipes.
async fn step_rows(db: &mut SqliteDatabase) -> anyhow::Result<i64> {
    Ok(db
        .connection
        .spawn_blocking(|conn| recipe_steps::table.count().get_result::<i64>(conn))
        .await?)
}

fn recipe(id: &str, title: &str) -> anyhow::Result<Recipe> {
    Ok(Recipe {
        id: id.parse()?,
        title: title.into(),
        steps: vec![],
    })
}

fn new_recipe(title: &str) -> anyhow::Result<NewRecipe> {
    with_steps(title, vec![])
}

fn update(title: &str) -> anyhow::Result<RecipeUpdate> {
    Ok(RecipeUpdate {
        title: RecipeTitle::try_new(title)?,
        steps: vec![],
    })
}

/// A synchronous connection with every migration and `TWO_RECIPES`.
fn migrated_connection() -> anyhow::Result<SqliteConnection> {
    let mut db = SqliteConnection::establish(":memory:")?;
    db.run_pending_migrations(MIGRATIONS)
        .map_err(|error| anyhow!(error))?;
    db.batch_execute(TWO_RECIPES)?;
    Ok(db)
}

async fn execute(db: &mut SqliteDatabase, sql: &'static str) -> anyhow::Result<()> {
    db.connection
        .spawn_blocking(move |conn| diesel::sql_query(sql).execute(conn).map(|_| ()))
        .await?;
    Ok(())
}
