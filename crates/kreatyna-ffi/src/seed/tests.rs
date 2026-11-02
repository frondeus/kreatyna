use std::path::PathBuf;

use anyhow::Context;
use kreatyna::ingredients::{
    Ingredient, NutritionBasis, ReferenceUnit, UnitName, capabilities::IngredientStoreMut,
};
use kreatyna_db::sqlite::SqliteDatabase;
use rust_decimal::Decimal;

use super::*;

// Green path

#[test]
fn seeds_names_into_new_database() -> anyhow::Result<()> {
    let database = TemporaryDatabase::new()?;

    seed_ingredients(database.path()?, r#"["Garlic", "Tomato"]"#.into())?;

    assert_eq!(stored_names(&database)?, ["Garlic", "Tomato"]);
    Ok(())
}

#[test]
fn seeds_all_fields() -> anyhow::Result<()> {
    let database = TemporaryDatabase::new()?;
    let json = r#"[{
        "name": "Garlic",
        "notes": "Peel the cloves.",
        "calories": 149, "protein": 6.4, "carbohydrates": 33, "fat": 0.5,
        "units": [{"name": "clove", "grams": 5}]
    }]"#;

    seed_ingredients(database.path()?, json.into())?;

    let garlic = only(stored(&database)?)?;
    assert_eq!(garlic.name.as_ref(), "Garlic");
    assert_eq!(garlic.notes.as_deref(), Some("Peel the cloves."));
    assert_eq!(garlic.nutrition.basis, NutritionBasis::Per100Grams);
    assert_eq!(garlic.nutrition.calories, Some(decimal("149")?.try_into()?));
    assert_eq!(garlic.nutrition.protein, Some(decimal("6.4")?.try_into()?));
    assert_eq!(
        garlic.nutrition.carbohydrates,
        Some(decimal("33")?.try_into()?)
    );
    assert_eq!(garlic.nutrition.fat, Some(decimal("0.5")?.try_into()?));
    assert_eq!(
        garlic.units,
        [ReferenceUnit {
            name: UnitName::try_new("clove")?,
            grams: Some(decimal("5")?.try_into()?),
            millilitres: None,
        }]
    );
    Ok(())
}

#[test]
fn seeds_millilitre_basis() -> anyhow::Result<()> {
    let database = TemporaryDatabase::new()?;
    let json = r#"[{
        "name": "Olive oil", "basis": "100ml", "fat": 91,
        "units": [{"name": "tablespoon", "millilitres": 15}]
    }]"#;

    seed_ingredients(database.path()?, json.into())?;

    let oil = only(stored(&database)?)?;
    assert_eq!(oil.nutrition.basis, NutritionBasis::Per100Millilitres);
    assert_eq!(oil.nutrition.calories, None);
    assert_eq!(oil.units[0].millilitres, Some(decimal("15")?.try_into()?));
    assert_eq!(oil.units[0].grams, None);
    Ok(())
}

#[test]
fn second_seed_skips_existing_names() -> anyhow::Result<()> {
    let database = TemporaryDatabase::new()?;

    seed_ingredients(database.path()?, r#"["Garlic"]"#.into())?;
    seed_ingredients(database.path()?, r#"["Garlic"]"#.into())?;
    seed_ingredients(database.path()?, r#"["garlic", "Tomato"]"#.into())?;

    assert_eq!(stored_names(&database)?, ["Garlic", "Tomato"]);
    Ok(())
}

#[test]
fn demo_fixture_is_valid() -> anyhow::Result<()> {
    let database = TemporaryDatabase::new()?;
    let json = include_str!("../../../../apple/Kreatyna/Fixtures/demo-ingredients.json");

    seed_ingredients(database.path()?, json.into())?;

    assert_eq!(
        stored_names(&database)?,
        [
            "Basil",
            "Canned tomatoes",
            "Cucumber",
            "Garlic",
            "Olive oil",
            "Tomato"
        ]
    );
    Ok(())
}

// Validation

#[test]
fn blank_name_is_rejected() -> anyhow::Result<()> {
    let database = TemporaryDatabase::new()?;

    let result = seed_ingredients(database.path()?, r#"["  "]"#.into());

    assert!(matches!(
        result,
        Err(SeedError::InvalidIngredient { index: 0, name }) if name == "  "
    ));
    Ok(())
}

#[test]
fn invalid_number_is_rejected_and_nothing_is_saved() -> anyhow::Result<()> {
    let database = TemporaryDatabase::new()?;
    let json = r#"["Tomato", {"name": "Garlic", "protein": -1}]"#;

    let result = seed_ingredients(database.path()?, json.into());

    assert!(matches!(
        result,
        Err(SeedError::InvalidIngredient { index: 1, name }) if name == "Garlic"
    ));
    assert!(!std::path::Path::new(&database.path()?).exists());
    Ok(())
}

#[test_case::test_case("[" ; "unfinished text")]
#[test_case::test_case("{}" ; "object instead of array")]
#[test_case::test_case(r#"["Garlic", 5]"# ; "number item")]
fn invalid_json_is_rejected(json: &str) -> anyhow::Result<()> {
    let database = TemporaryDatabase::new()?;

    let result = seed_ingredients(database.path()?, json.into());

    assert!(
        matches!(result, Err(SeedError::InvalidJson { .. })),
        "{result:?}"
    );
    assert!(!std::path::Path::new(&database.path()?).exists());
    Ok(())
}

#[test]
fn unknown_field_is_rejected_with_item_index() -> anyhow::Result<()> {
    let database = TemporaryDatabase::new()?;

    let result = seed_ingredients(
        database.path()?,
        r#"[{"name": "Garlic", "calorie": 1}]"#.into(),
    );

    let Err(SeedError::InvalidJson { message }) = result else {
        anyhow::bail!("expected InvalidJson, got {result:?}");
    };
    assert!(message.starts_with("item 0: "), "{message}");
    assert!(message.contains("calorie"), "{message}");
    Ok(())
}

// Red cases

#[test]
fn unopenable_path_reports_startup_error() -> anyhow::Result<()> {
    let folder = tempfile::tempdir()?;
    let path = folder.path().join("missing/app.sqlite");
    let path = path
        .to_str()
        .context("the temporary path is not UTF-8")?
        .to_owned();

    let result = seed_ingredients(path, r#"["Garlic"]"#.into());

    assert!(matches!(
        result,
        Err(SeedError::Startup {
            error: AppInitializationError::DatabaseOpen
        })
    ));
    Ok(())
}

// Test support

/// A database file in its own temporary folder. The folder is deleted on drop.
struct TemporaryDatabase {
    _folder: tempfile::TempDir,
    path: PathBuf,
}

impl TemporaryDatabase {
    fn new() -> anyhow::Result<Self> {
        let folder = tempfile::tempdir()?;
        let path = folder.path().join("app.sqlite");
        Ok(Self {
            _folder: folder,
            path,
        })
    }

    fn path(&self) -> anyhow::Result<String> {
        let path = self
            .path
            .to_str()
            .context("the temporary path is not UTF-8")?;
        Ok(path.into())
    }
}

/// Every stored ingredient, sorted by name.
fn stored(database: &TemporaryDatabase) -> anyhow::Result<Vec<Ingredient>> {
    RUNTIME.block_on(async {
        let mut database = SqliteDatabase::new(&database.path()?).await?;
        let mut ingredients = database.list().await?;
        ingredients.sort_by(|a, b| a.name.as_ref().cmp(b.name.as_ref()));
        Ok(ingredients)
    })
}

fn stored_names(database: &TemporaryDatabase) -> anyhow::Result<Vec<String>> {
    let names = stored(database)?
        .into_iter()
        .map(|ingredient| ingredient.name.into_inner())
        .collect();
    Ok(names)
}

fn only(ingredients: Vec<Ingredient>) -> anyhow::Result<Ingredient> {
    let [ingredient] = ingredients.try_into().map_err(|rest: Vec<Ingredient>| {
        anyhow::anyhow!("expected one ingredient, got {}", rest.len())
    })?;
    Ok(ingredient)
}

fn decimal(value: &str) -> anyhow::Result<Decimal> {
    Ok(value.parse()?)
}
