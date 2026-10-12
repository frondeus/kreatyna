use std::path::PathBuf;

use kreatyna::ingredients::{
    Ingredient, NutritionBasis, ReferenceUnit, capabilities::IngredientStoreMut,
};
use kreatyna_db::sqlite::SqliteDatabase;
use rust_decimal::Decimal;

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

fn stored(database: &TemporaryDatabase) -> Vec<Ingredient> {
    RUNTIME.block_on(async {
        let mut database = SqliteDatabase::new(&database.path()).await.unwrap();
        let mut ingredients = database.list().await.unwrap();
        ingredients.sort_by(|a, b| a.name.cmp(&b.name));
        ingredients
    })
}

fn stored_names(database: &TemporaryDatabase) -> Vec<String> {
    stored(database)
        .into_iter()
        .map(|ingredient| ingredient.name)
        .collect()
}

fn decimal(value: &str) -> Decimal {
    value.parse().unwrap()
}

#[test]
fn seeds_names_into_new_database() {
    let database = TemporaryDatabase::new();

    seed_ingredients(database.path(), r#"["Garlic", "Tomato"]"#.into()).unwrap();

    assert_eq!(stored_names(&database), ["Garlic", "Tomato"]);
}

#[test]
fn seeds_all_fields() {
    let database = TemporaryDatabase::new();
    let json = r#"[{
        "name": "Garlic",
        "notes": "Peel the cloves.",
        "calories": 149, "protein": 6.4, "carbohydrates": 33, "fat": 0.5,
        "units": [{"name": "clove", "grams": 5}]
    }]"#;

    seed_ingredients(database.path(), json.into()).unwrap();

    let [garlic] = stored(&database).try_into().unwrap();
    assert_eq!(garlic.name, "Garlic");
    assert_eq!(garlic.notes.as_deref(), Some("Peel the cloves."));
    assert_eq!(garlic.nutrition.basis, NutritionBasis::Per100Grams);
    assert_eq!(garlic.nutrition.calories, Some(decimal("149").into()));
    assert_eq!(garlic.nutrition.protein, Some(decimal("6.4").into()));
    assert_eq!(garlic.nutrition.carbohydrates, Some(decimal("33").into()));
    assert_eq!(garlic.nutrition.fat, Some(decimal("0.5").into()));
    assert_eq!(
        garlic.units,
        [ReferenceUnit {
            name: "clove".into(),
            grams: Some(decimal("5").into()),
            millilitres: None,
        }]
    );
}

#[test]
fn seeds_millilitre_basis() {
    let database = TemporaryDatabase::new();
    let json = r#"[{
        "name": "Olive oil", "basis": "100ml", "fat": 91,
        "units": [{"name": "tablespoon", "millilitres": 15}]
    }]"#;

    seed_ingredients(database.path(), json.into()).unwrap();

    let [oil] = stored(&database).try_into().unwrap();
    assert_eq!(oil.nutrition.basis, NutritionBasis::Per100Millilitres);
    assert_eq!(oil.nutrition.calories, None);
    assert_eq!(oil.units[0].millilitres, Some(decimal("15").into()));
    assert_eq!(oil.units[0].grams, None);
}

#[test]
fn second_seed_skips_existing_names() {
    let database = TemporaryDatabase::new();

    seed_ingredients(database.path(), r#"["Garlic"]"#.into()).unwrap();
    seed_ingredients(database.path(), r#"["Garlic"]"#.into()).unwrap();
    seed_ingredients(database.path(), r#"["garlic", "Tomato"]"#.into()).unwrap();

    assert_eq!(stored_names(&database), ["Garlic", "Tomato"]);
}

#[test]
fn blank_name_is_rejected() {
    let database = TemporaryDatabase::new();

    let result = seed_ingredients(database.path(), r#"["  "]"#.into());

    assert!(matches!(
        result,
        Err(SeedError::InvalidIngredient { index: 0, name }) if name == "  "
    ));
}

#[test]
fn invalid_number_is_rejected_and_nothing_is_saved() {
    let database = TemporaryDatabase::new();
    let json = r#"["Tomato", {"name": "Garlic", "protein": -1}]"#;

    let result = seed_ingredients(database.path(), json.into());

    assert!(matches!(
        result,
        Err(SeedError::InvalidIngredient { index: 1, name }) if name == "Garlic"
    ));
    assert!(!std::path::Path::new(&database.path()).exists());
}

#[test_case::test_case("[" ; "unfinished text")]
#[test_case::test_case("{}" ; "object instead of array")]
#[test_case::test_case(r#"["Garlic", 5]"# ; "number item")]
fn invalid_json_is_rejected(json: &str) {
    let database = TemporaryDatabase::new();

    let result = seed_ingredients(database.path(), json.into());

    assert!(
        matches!(result, Err(SeedError::InvalidJson { .. })),
        "{result:?}"
    );
    assert!(!std::path::Path::new(&database.path()).exists());
}

#[test]
fn unknown_field_is_rejected_with_item_index() {
    let database = TemporaryDatabase::new();

    let result = seed_ingredients(
        database.path(),
        r#"[{"name": "Garlic", "calorie": 1}]"#.into(),
    );

    let Err(SeedError::InvalidJson { message }) = result else {
        panic!("expected InvalidJson, got {result:?}");
    };
    assert!(message.starts_with("item 0: "), "{message}");
    assert!(message.contains("calorie"), "{message}");
}

#[test]
fn demo_fixture_is_valid() {
    let database = TemporaryDatabase::new();
    let json = include_str!("../../../../apple/Kreatyna/Fixtures/demo-ingredients.json");

    seed_ingredients(database.path(), json.into()).unwrap();

    assert_eq!(
        stored_names(&database),
        [
            "Canned tomatoes",
            "Cucumber",
            "Garlic",
            "Olive oil",
            "Tomato"
        ]
    );
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

    let result = seed_ingredients(path, r#"["Garlic"]"#.into());

    assert!(matches!(
        result,
        Err(SeedError::Startup {
            error: AppInitializationError::DatabaseOpen
        })
    ));
}
