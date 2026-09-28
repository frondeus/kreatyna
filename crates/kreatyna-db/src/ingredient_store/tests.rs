use std::{future::Future, path::PathBuf};

use diesel::RunQueryDsl;
use kreatyna::capabilities::CounterDatabaseMut;
use kreatyna::ingredients::{
    DraftMode, IngredientDraft, IngredientId, IngredientWrite, NutritionBasis, ReferenceUnitDraft,
    capabilities::{IngredientLoadError, IngredientSaveError, IngredientStoreMut},
};
use test_case::test_case;

use super::*;
use crate::schema::ingredient_units;

fn run<F: Future>(future: F) -> F::Output {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap()
        .block_on(future)
}

struct TemporaryDatabase(PathBuf);

impl TemporaryDatabase {
    fn new() -> Self {
        Self(std::env::temp_dir().join(format!("ingredient-store-{}.sqlite", IngredientId::new())))
    }

    fn path(&self) -> &str {
        self.0.to_str().unwrap()
    }
}

impl Drop for TemporaryDatabase {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
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

fn write(draft: &IngredientDraft) -> IngredientWrite {
    IngredientWrite::try_from(draft).unwrap()
}

async fn execute(db: &mut SqliteDatabase, sql: &'static str) {
    db.connection
        .spawn_blocking(move |conn| diesel::sql_query(sql).execute(conn).map(|_| ()))
        .await
        .unwrap();
}

#[test_case(NutritionBasis::Per100Grams; "mass basis")]
#[test_case(NutritionBasis::Per100Millilitres; "volume basis")]
fn create_update_list_and_reopen_preserve_values(basis: NutritionBasis) {
    run(async {
        let file = TemporaryDatabase::new();
        let mut db = SqliteDatabase::new(file.path()).await.unwrap();
        assert!(db.list().await.unwrap().is_empty());
        db.set_count(27).await.unwrap();
        let saved = db.save(write(&draft("  Tomatoes  ", basis))).await.unwrap();
        assert_eq!(saved.name, "Tomatoes");
        assert_eq!(saved.nutrition.basis, basis);
        assert_eq!(saved.nutrition.calories.unwrap().to_string(), "12.3400");
        assert_eq!(saved.nutrition.protein.unwrap().to_string(), "0");
        assert!(saved.nutrition.fat.is_none());
        assert_eq!(
            saved
                .units
                .iter()
                .map(|u| u.name.as_str())
                .collect::<Vec<_>>(),
            ["can", "cup"]
        );
        assert_eq!(saved.units[0].grams.unwrap().to_string(), "400.000");
        assert_eq!(saved.units[1].millilitres.unwrap().to_string(), "250.00");
        assert_eq!(
            db.list().await.unwrap().as_slice(),
            std::slice::from_ref(&saved)
        );

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
        let updated = db.save(write(&edit)).await.unwrap();
        assert_eq!(updated.id, saved.id);
        assert_eq!(updated.name, "Red  tomatoes");
        assert_eq!(updated.notes.as_deref(), Some("fresh"));
        assert!(updated.nutrition.calories.is_none());
        assert_eq!(updated.nutrition.protein.unwrap().to_string(), "0.000000");
        assert_eq!(
            updated
                .units
                .iter()
                .map(|u| u.name.as_str())
                .collect::<Vec<_>>(),
            ["cup", "bottle"]
        );
        assert_eq!(updated.units[1].grams.unwrap().to_string(), "100.5");

        let mut clear_notes = IngredientDraft::from(&updated);
        clear_notes.notes = " \u{2003} ".into();
        clear_notes.protein.clear();
        let updated = db.save(write(&clear_notes)).await.unwrap();
        assert_eq!(updated.id, saved.id);
        assert!(updated.notes.is_none());
        assert!(updated.nutrition.protein.is_none());

        drop(db);
        let mut reopened = SqliteDatabase::new(file.path()).await.unwrap();
        assert_eq!(reopened.count().await.unwrap(), Some(27));
        assert_eq!(
            reopened.list().await.unwrap().as_slice(),
            std::slice::from_ref(&updated)
        );
        reopened.delete(updated.id).await.unwrap();
        reopened.delete(updated.id).await.unwrap();
        assert!(reopened.list().await.unwrap().is_empty());
        let remaining: i64 = reopened
            .connection
            .spawn_blocking(|conn| ingredient_units::table.count().get_result(conn))
            .await
            .unwrap();
        assert_eq!(remaining, 0);
        assert_eq!(reopened.count().await.unwrap(), Some(27));
    });
}

#[test]
fn missing_update_does_not_insert() {
    run(async {
        let mut db = SqliteDatabase::new(":memory:").await.unwrap();
        let mut edit = draft("Absent", NutritionBasis::Per100Grams);
        edit.mode = DraftMode::Edit {
            id: IngredientId::new(),
        };
        assert_eq!(
            db.save(write(&edit)).await,
            Err(IngredientSaveError::NotFound)
        );
        assert!(db.list().await.unwrap().is_empty());
    });
}

#[test_case("Straße", " STRASSE "; "sharp s")]
#[test_case("Σ", "ς"; "sigma")]
fn names_are_unique_after_unicode_folding(first: &str, duplicate: &str) {
    run(async {
        let mut db = SqliteDatabase::new(":memory:").await.unwrap();
        let original = db
            .save(write(&draft(first, NutritionBasis::Per100Grams)))
            .await
            .unwrap();
        assert_eq!(
            db.save(write(&draft(duplicate, NutritionBasis::Per100Grams)))
                .await,
            Err(IngredientSaveError::DuplicateName)
        );
        let other = db
            .save(write(&draft("Other", NutritionBasis::Per100Grams)))
            .await
            .unwrap();
        let unchanged = IngredientDraft::from(&original);
        assert_eq!(db.save(write(&unchanged)).await.unwrap(), original);
        let mut rename = IngredientDraft::from(&other);
        rename.name = duplicate.into();
        assert_eq!(
            db.save(write(&rename)).await,
            Err(IngredientSaveError::DuplicateName)
        );
        assert_eq!(db.list().await.unwrap().len(), 2);
    });
}

#[test]
fn concurrent_creates_serialize_name_conflict() {
    run(async {
        let file = TemporaryDatabase::new();
        let mut first = SqliteDatabase::new(file.path()).await.unwrap();
        let mut second = SqliteDatabase::new(file.path()).await.unwrap();
        let (left, right) = futures::join!(
            first.save(write(&draft("Straße", NutritionBasis::Per100Grams))),
            second.save(write(&draft("STRASSE", NutritionBasis::Per100Grams)))
        );
        assert_eq!(left.is_ok() as u8 + right.is_ok() as u8, 1);
        assert!(
            left == Err(IngredientSaveError::DuplicateName)
                || right == Err(IngredientSaveError::DuplicateName)
        );
        assert_eq!(first.list().await.unwrap().len(), 1);
    });
}

#[test_case(false; "create rollback")]
#[test_case(true; "update rollback")]
fn unit_failure_rolls_back_whole_save(update: bool) {
    run(async {
        let mut db = SqliteDatabase::new(":memory:").await.unwrap();
        let before = db
            .save(write(&draft("Original", NutritionBasis::Per100Grams)))
            .await
            .unwrap();
        execute(&mut db, "CREATE TRIGGER reject_boom BEFORE INSERT ON ingredient_units WHEN NEW.name = 'boom' BEGIN SELECT RAISE(FAIL, 'blocked by test'); END").await;
        let mut attempted = if update {
            IngredientDraft::from(&before)
        } else {
            draft("New", NutritionBasis::Per100Grams)
        };
        attempted.name = if update { "Changed" } else { "New" }.into();
        attempted.units.push(ReferenceUnitDraft {
            name: "boom".into(),
            grams: "1".into(),
            millilitres: String::new(),
        });
        assert_eq!(
            db.save(write(&attempted)).await,
            Err(IngredientSaveError::Storage)
        );
        assert_eq!(db.list().await.unwrap(), [before]);
    });
}

#[test_case("UPDATE ingredients SET calories = 'bad'"; "malformed decimal")]
#[test_case("UPDATE ingredients SET calories = '-1'"; "negative decimal")]
#[test_case("UPDATE ingredient_units SET grams = '0' WHERE position = 0"; "nonpositive unit")]
#[test_case("UPDATE ingredients SET name_key = 'wrong' WHERE name = 'Corrupt'"; "inconsistent key")]
#[test_case("UPDATE ingredient_units SET name_key = 'wrong' WHERE name = 'can'"; "inconsistent unit key")]
fn corrupted_record_fails_whole_list(sql: &'static str) {
    run(async {
        let mut db = SqliteDatabase::new(":memory:").await.unwrap();
        db.save(write(&draft("Healthy", NutritionBasis::Per100Grams)))
            .await
            .unwrap();
        db.save(write(&draft("Corrupt", NutritionBasis::Per100Grams)))
            .await
            .unwrap();
        execute(&mut db, sql).await;
        assert_eq!(db.list().await, Err(IngredientLoadError::Storage));
    });
}
