use diesel::{Connection, RunQueryDsl, SqliteConnection, connection::SimpleConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use kreatyna::{
    ingredients::capabilities::IngredientStoreMut,
    recipes::{Recipe, RecipeId},
};
use test_case::test_case;

use super::*;

// Green path

#[tokio::test]
async fn an_empty_database_lists_no_recipes() {
    let mut db = SqliteDatabase::new(":memory:").await.unwrap();
    assert_eq!(RecipeStoreMut::list(&mut db).await, Ok(vec![]));
}

#[tokio::test]
async fn stored_recipes_are_listed_by_title_key_with_their_titles_as_written() {
    let mut db = SqliteDatabase::new(":memory:").await.unwrap();
    // Shakshuka is inserted first, so the order comes from the title key.
    execute(&mut db, TWO_RECIPES).await;
    assert_eq!(
        RecipeStoreMut::list(&mut db).await,
        Ok(vec![
            recipe(PASTA, "Roasted tomato pasta"),
            recipe(SHAKSHUKA, "Shakshuka"),
        ])
    );
}

#[tokio::test]
async fn a_catalog_from_before_rec_01_upgrades_and_keeps_its_ingredients() {
    let folder = tempfile::tempdir().unwrap();
    let path = folder.path().join("app.sqlite");
    let path = path.to_str().unwrap();
    {
        let mut db = SqliteConnection::establish(path).unwrap();
        // The schema before REC-01: counter, ingredients, and the counter drop.
        for _ in 0..3 {
            db.run_next_migration(MIGRATIONS).unwrap();
        }
        db.batch_execute(
            "INSERT INTO ingredients (remote_id, name, name_key, basis)
             VALUES ('6f1c1d3e-0000-4000-8000-0000000000aa', 'Garlic', 'garlic', 0)",
        )
        .unwrap();
    }
    {
        let mut db = SqliteDatabase::new(path).await.unwrap();
        let ingredients = IngredientStoreMut::list(&mut db).await.unwrap();
        assert_eq!(
            ingredients
                .iter()
                .map(|ingredient| ingredient.name.as_str())
                .collect::<Vec<_>>(),
            ["Garlic"]
        );
        assert_eq!(RecipeStoreMut::list(&mut db).await, Ok(vec![]));
    }
    // A second start finds no pending migration and changes nothing.
    let mut db = SqliteDatabase::new(path).await.unwrap();
    assert_eq!(IngredientStoreMut::list(&mut db).await.unwrap().len(), 1);
}

// Validation

#[test_case("INSERT INTO recipes (remote_id, title, title_key) VALUES ('6f1c1d3e-0000-4000-8000-000000000003', '  ', ''); "; "blank title")]
#[test_case("INSERT INTO recipes (remote_id, title, title_key) VALUES ('6f1c1d3e-0000-4000-8000-000000000003', 'shakshuka ', 'shakshuka'); "; "duplicate title key")]
#[test_case("INSERT INTO recipes (remote_id, title, title_key) VALUES ('6f1c1d3e-0000-4000-8000-000000000001', 'Pancakes', 'pancakes'); "; "duplicate id")]
fn the_table_refuses_invalid_rows(sql: &str) {
    let mut db = SqliteConnection::establish(":memory:").unwrap();
    db.run_pending_migrations(MIGRATIONS).unwrap();
    db.batch_execute(TWO_RECIPES).unwrap();
    assert!(db.batch_execute(sql).is_err());
}

// Red cases

#[test_case("UPDATE recipes SET title_key = 'wrong' WHERE title = 'Shakshuka'"; "inconsistent title key")]
#[test_case("UPDATE recipes SET remote_id = 'not-a-uuid' WHERE title = 'Shakshuka'"; "invalid id")]
#[tokio::test]
async fn a_corrupted_recipe_fails_the_whole_list(sql: &'static str) {
    let mut db = SqliteDatabase::new(":memory:").await.unwrap();
    execute(&mut db, TWO_RECIPES).await;
    execute(&mut db, sql).await;
    assert_eq!(
        RecipeStoreMut::list(&mut db).await,
        Err(RecipeLoadError::Storage)
    );
}

// Test support

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

const SHAKSHUKA: &str = "6f1c1d3e-0000-4000-8000-000000000001";
const PASTA: &str = "6f1c1d3e-0000-4000-8000-000000000002";

/// Two recipes, written as the store will write them (REC-02).
const TWO_RECIPES: &str = "INSERT INTO recipes (remote_id, title, title_key) VALUES
    ('6f1c1d3e-0000-4000-8000-000000000001', 'Shakshuka', 'shakshuka'),
    ('6f1c1d3e-0000-4000-8000-000000000002', 'Roasted tomato pasta', 'roasted tomato pasta')";

fn recipe(id: &str, title: &str) -> Recipe {
    Recipe {
        id: id.parse::<RecipeId>().unwrap(),
        title: title.into(),
    }
}

async fn execute(db: &mut SqliteDatabase, sql: &'static str) {
    db.connection
        .spawn_blocking(move |conn| diesel::sql_query(sql).execute(conn).map(|_| ()))
        .await
        .unwrap();
}
