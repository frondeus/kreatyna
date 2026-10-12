use std::path::PathBuf;

use kreatyna::ingredients::capabilities::IngredientStoreMut;
use kreatyna_db::sqlite::SqliteDatabase;

use super::*;

/// A database file in its own temporary folder. The folder is deleted on drop.
struct TemporaryDatabase {
    _folder: tempfile::TempDir,
    path: PathBuf,
}

impl TemporaryDatabase {
    fn new() -> Self {
        let folder = tempfile::tempdir().unwrap();
        let path = folder.path().join("app.sqlite");
        Self {
            _folder: folder,
            path,
        }
    }

    fn path(&self) -> String {
        self.path.to_str().unwrap().into()
    }
}

fn names(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).into()).collect()
}

fn stored_names(database: &TemporaryDatabase) -> Vec<String> {
    RUNTIME.block_on(async {
        let mut database = SqliteDatabase::new(&database.path()).await.unwrap();
        let mut names: Vec<_> = database
            .list()
            .await
            .unwrap()
            .into_iter()
            .map(|ingredient| ingredient.name)
            .collect();
        names.sort();
        names
    })
}

#[test]
fn seeds_names_into_new_database() {
    let database = TemporaryDatabase::new();

    seed_ingredients(database.path(), names(&["Garlic", "Tomato"])).unwrap();

    assert_eq!(stored_names(&database), ["Garlic", "Tomato"]);
}

#[test]
fn second_seed_skips_existing_names() {
    let database = TemporaryDatabase::new();

    seed_ingredients(database.path(), names(&["Garlic"])).unwrap();
    seed_ingredients(database.path(), names(&["Garlic"])).unwrap();
    seed_ingredients(database.path(), names(&["garlic", "Tomato"])).unwrap();

    assert_eq!(stored_names(&database), ["Garlic", "Tomato"]);
}

#[test]
fn blank_name_is_rejected() {
    let database = TemporaryDatabase::new();

    let result = seed_ingredients(database.path(), names(&["  "]));

    assert!(matches!(result, Err(SeedError::InvalidName { name }) if name == "  "));
}

#[test]
fn unopenable_path_reports_startup_error() {
    let folder = tempfile::tempdir().unwrap();
    let path = folder
        .path()
        .join("missing/app.sqlite")
        .to_str()
        .unwrap()
        .to_owned();

    let result = seed_ingredients(path, names(&["Garlic"]));

    assert!(matches!(
        result,
        Err(SeedError::Startup {
            error: AppInitializationError::DatabaseOpen
        })
    ));
}
