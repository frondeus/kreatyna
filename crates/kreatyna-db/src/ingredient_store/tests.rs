use std::path::PathBuf;

use anyhow::Context;
use diesel::RunQueryDsl;
use kreatyna::ingredients::{
    DraftMode, IngredientDraft, IngredientId, IngredientWrite, NutritionBasis, ReferenceUnitDraft,
    capabilities::{IngredientLoadError, IngredientSaveError, IngredientStoreMut},
};
use test_case::test_case;

use super::*;
use crate::schema::ingredient_units;

// Green path

#[test_case(NutritionBasis::Per100Grams; "mass basis")]
#[test_case(NutritionBasis::Per100Millilitres; "volume basis")]
#[tokio::test]
async fn create_update_list_and_reopen_preserve_values(
    basis: NutritionBasis,
) -> anyhow::Result<()> {
    let file = TemporaryDatabase::new()?;
    let mut db = SqliteDatabase::new(file.path()?).await?;
    assert!(db.list().await?.is_empty());
    let saved = db.save(write(&draft("  Tomatoes  ", basis))?).await?;
    assert_eq!(saved.name, "Tomatoes");
    assert_eq!(saved.nutrition.basis, basis);
    let calories = saved.nutrition.calories.context("no calories")?;
    assert_eq!(calories.to_string(), "12.3400");
    let protein = saved.nutrition.protein.context("no protein")?;
    assert_eq!(protein.to_string(), "0");
    assert!(saved.nutrition.fat.is_none());
    assert_eq!(unit_names(&saved), ["can", "cup"]);
    let can = saved.units[0].grams.context("no can grams")?;
    assert_eq!(can.to_string(), "400.000");
    let cup = saved.units[1].millilitres.context("no cup millilitres")?;
    assert_eq!(cup.to_string(), "250.00");
    assert_eq!(db.list().await?.as_slice(), std::slice::from_ref(&saved));

    let mut edit = IngredientDraft::from(&saved);
    edit.name = "  Red  tomatoes ".into();
    edit.notes = " fresh ".into();
    edit.calories.clear();
    edit.protein = "0.000000".into();
    edit.units.remove(0);
    edit.units.push(ReferenceUnitDraft {
        name: "bottle".into(),
        grams: "100.5".into(),
        millilitres: "125".into(),
    });
    let updated = db.save(write(&edit)?).await?;
    assert_eq!(updated.id, saved.id);
    assert_eq!(updated.name, "Red  tomatoes");
    assert_eq!(updated.notes.as_deref(), Some("fresh"));
    assert!(updated.nutrition.calories.is_none());
    let protein = updated.nutrition.protein.context("no protein")?;
    assert_eq!(protein.to_string(), "0.000000");
    assert_eq!(unit_names(&updated), ["cup", "bottle"]);
    let bottle = updated.units[1].grams.context("no bottle grams")?;
    assert_eq!(bottle.to_string(), "100.5");

    let mut clear_notes = IngredientDraft::from(&updated);
    clear_notes.notes = " \u{2003} ".into();
    clear_notes.protein.clear();
    let updated = db.save(write(&clear_notes)?).await?;
    assert_eq!(updated.id, saved.id);
    assert!(updated.notes.is_none());
    assert!(updated.nutrition.protein.is_none());

    drop(db);
    let mut reopened = SqliteDatabase::new(file.path()?).await?;
    assert_eq!(
        reopened.list().await?.as_slice(),
        std::slice::from_ref(&updated)
    );
    reopened.delete(updated.id).await?;
    reopened.delete(updated.id).await?;
    assert!(reopened.list().await?.is_empty());
    let remaining: i64 = reopened
        .connection
        .spawn_blocking(|conn| ingredient_units::table.count().get_result(conn))
        .await?;
    assert_eq!(remaining, 0);
    Ok(())
}

// Validation

#[tokio::test]
async fn missing_update_does_not_insert() -> anyhow::Result<()> {
    let mut db = SqliteDatabase::new(":memory:").await?;
    let mut edit = draft("Absent", NutritionBasis::Per100Grams);
    edit.mode = DraftMode::Edit {
        id: IngredientId::generate(),
    };
    assert_eq!(
        db.save(write(&edit)?).await,
        Err(IngredientSaveError::NotFound)
    );
    assert!(db.list().await?.is_empty());
    Ok(())
}

#[test_case("Straße", " STRASSE "; "sharp s")]
#[test_case("Σ", "ς"; "sigma")]
#[tokio::test]
async fn names_are_unique_after_unicode_folding(
    first: &str,
    duplicate: &str,
) -> anyhow::Result<()> {
    let mut db = SqliteDatabase::new(":memory:").await?;
    let original = db
        .save(write(&draft(first, NutritionBasis::Per100Grams))?)
        .await?;
    assert_eq!(
        db.save(write(&draft(duplicate, NutritionBasis::Per100Grams))?)
            .await,
        Err(IngredientSaveError::DuplicateName)
    );
    let other = db
        .save(write(&draft("Other", NutritionBasis::Per100Grams))?)
        .await?;
    let unchanged = IngredientDraft::from(&original);
    assert_eq!(db.save(write(&unchanged)?).await?, original);
    let mut rename = IngredientDraft::from(&other);
    rename.name = duplicate.into();
    assert_eq!(
        db.save(write(&rename)?).await,
        Err(IngredientSaveError::DuplicateName)
    );
    assert_eq!(db.list().await?.len(), 2);
    Ok(())
}

// Two connections save at the same time, so their SQLite transactions
// really overlap; the multi-thread runtime lets both run at once.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn concurrent_creates_serialize_name_conflict() -> anyhow::Result<()> {
    let file = TemporaryDatabase::new()?;
    let mut first = SqliteDatabase::new(file.path()?).await?;
    let mut second = SqliteDatabase::new(file.path()?).await?;
    let (left, right) = futures::join!(
        first.save(write(&draft("Straße", NutritionBasis::Per100Grams))?),
        second.save(write(&draft("STRASSE", NutritionBasis::Per100Grams))?)
    );
    assert_eq!(left.is_ok() as u8 + right.is_ok() as u8, 1);
    assert!(
        left == Err(IngredientSaveError::DuplicateName)
            || right == Err(IngredientSaveError::DuplicateName)
    );
    assert_eq!(first.list().await?.len(), 1);
    Ok(())
}

// Red cases

#[tokio::test]
async fn a_unit_failure_rolls_back_the_whole_create() -> anyhow::Result<()> {
    let mut db = SqliteDatabase::new(":memory:").await?;
    let before = db
        .save(write(&draft("Original", NutritionBasis::Per100Grams))?)
        .await?;
    execute(&mut db, REJECT_BOOM_UNITS).await?;
    let mut attempted = draft("New", NutritionBasis::Per100Grams);
    attempted.units.push(boom_unit());
    assert_eq!(
        db.save(write(&attempted)?).await,
        Err(IngredientSaveError::Storage)
    );
    assert_eq!(db.list().await?, [before]);
    Ok(())
}

#[tokio::test]
async fn a_unit_failure_rolls_back_the_whole_update() -> anyhow::Result<()> {
    let mut db = SqliteDatabase::new(":memory:").await?;
    let before = db
        .save(write(&draft("Original", NutritionBasis::Per100Grams))?)
        .await?;
    execute(&mut db, REJECT_BOOM_UNITS).await?;
    let mut attempted = IngredientDraft::from(&before);
    attempted.name = "Changed".into();
    attempted.units.push(boom_unit());
    assert_eq!(
        db.save(write(&attempted)?).await,
        Err(IngredientSaveError::Storage)
    );
    assert_eq!(db.list().await?, [before]);
    Ok(())
}

#[test_case("UPDATE ingredients SET calories = 'bad'"; "malformed decimal")]
#[test_case("UPDATE ingredients SET calories = '-1'"; "negative decimal")]
#[test_case("UPDATE ingredient_units SET grams = '0' WHERE position = 0"; "nonpositive unit")]
#[test_case("UPDATE ingredients SET name_key = 'wrong' WHERE name = 'Corrupt'"; "inconsistent key")]
#[test_case("UPDATE ingredient_units SET name_key = 'wrong' WHERE name = 'can'"; "inconsistent unit key")]
#[tokio::test]
async fn corrupted_record_fails_whole_list(sql: &'static str) -> anyhow::Result<()> {
    let mut db = SqliteDatabase::new(":memory:").await?;
    db.save(write(&draft("Healthy", NutritionBasis::Per100Grams))?)
        .await?;
    db.save(write(&draft("Corrupt", NutritionBasis::Per100Grams))?)
        .await?;
    execute(&mut db, sql).await?;
    assert_eq!(db.list().await, Err(IngredientLoadError::Storage));
    Ok(())
}

// Test support

/// A trigger that fails every insert of a unit named "boom".
const REJECT_BOOM_UNITS: &str = "CREATE TRIGGER reject_boom BEFORE INSERT ON ingredient_units \
    WHEN NEW.name = 'boom' BEGIN SELECT RAISE(FAIL, 'blocked by test'); END";

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

    fn path(&self) -> anyhow::Result<&str> {
        self.path
            .to_str()
            .context("the temporary path is not UTF-8")
    }
}

fn draft(name: &str, basis: NutritionBasis) -> IngredientDraft {
    let mut draft = IngredientDraft::new(basis);
    draft.name = name.into();
    draft.calories = "12.3400".into();
    draft.protein = "0".into();
    draft.units = vec![
        ReferenceUnitDraft {
            name: "can".into(),
            grams: "400.000".into(),
            millilitres: String::new(),
        },
        ReferenceUnitDraft {
            name: "cup".into(),
            grams: String::new(),
            millilitres: "250.00".into(),
        },
    ];
    draft
}

fn boom_unit() -> ReferenceUnitDraft {
    ReferenceUnitDraft {
        name: "boom".into(),
        grams: "1".into(),
        millilitres: String::new(),
    }
}

fn write(draft: &IngredientDraft) -> anyhow::Result<IngredientWrite> {
    IngredientWrite::try_from(draft)
        .map_err(|issues| anyhow::anyhow!("the draft is invalid: {issues:?}"))
}

fn unit_names(ingredient: &kreatyna::ingredients::Ingredient) -> Vec<&str> {
    ingredient
        .units
        .iter()
        .map(|unit| unit.name.as_str())
        .collect()
}

async fn execute(db: &mut SqliteDatabase, sql: &'static str) -> anyhow::Result<()> {
    db.connection
        .spawn_blocking(move |conn| diesel::sql_query(sql).execute(conn).map(|_| ()))
        .await?;
    Ok(())
}
