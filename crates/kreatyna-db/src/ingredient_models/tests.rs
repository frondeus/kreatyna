use anyhow::Context;
use diesel::{
    Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection,
    connection::SimpleConnection, result::Error as DieselError,
};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use kreatyna::ingredients::{
    Ingredient, IngredientDraft, IngredientId, IngredientWrite, NutritionBasis, ReferenceUnit,
    ReferenceUnitDraft,
};
use rust_decimal::Decimal;
use test_case::test_case;

use super::*;
use crate::schema::{ingredient_units as u, ingredients as i};

// Green path

#[test]
fn empty_database_migrates_and_rerun_is_harmless() -> anyhow::Result<()> {
    let mut db = SqliteConnection::establish(":memory:")?;
    db.batch_execute("PRAGMA foreign_keys = ON;")?;
    assert_eq!(migrate(&mut db)?, 5);
    assert_eq!(migrate(&mut db)?, 0);
    assert_eq!(i::table.count().get_result::<i64>(&mut db)?, 0);
    assert_eq!(u::table.count().get_result::<i64>(&mut db)?, 0);
    assert!(!counter_table_exists(&mut db)?);
    Ok(())
}

#[test]
fn counter_database_upgrades_and_keeps_ingredients() -> anyhow::Result<()> {
    let folder = tempfile::tempdir()?;
    let path = folder.path().join("kreatyna.sqlite");
    let path = path.to_str().context("the temporary path is not UTF-8")?;
    {
        let mut db = SqliteConnection::establish(path)?;
        db.batch_execute("PRAGMA foreign_keys = ON;")?;
        // The schema before MAC-03: the counter and ingredient migrations only.
        db.run_next_migration(MIGRATIONS)
            .map_err(|error| anyhow::anyhow!(error))?;
        db.run_next_migration(MIGRATIONS)
            .map_err(|error| anyhow::anyhow!(error))?;
        db.batch_execute("INSERT INTO counter (id, value) VALUES (1, 27);")?;
        let data = write_data(NutritionBasis::Per100Grams, "1.2300", "2")?;
        diesel::insert_into(i::table)
            .values(NewIngredientRow::from_write(
                IngredientId::generate(),
                &data,
            ))
            .execute(&mut db)?;

        // MAC-03 drops the counter; REC-01 adds recipes; REC-14 adds steps.
        assert_eq!(migrate(&mut db)?, 3);
        assert!(!counter_table_exists(&mut db)?);
    }
    let mut db = SqliteConnection::establish(path)?;
    db.batch_execute("PRAGMA foreign_keys = ON;")?;
    assert_eq!(migrate(&mut db)?, 0);
    let row = i::table
        .select(IngredientRow::as_select())
        .first::<IngredientRow>(&mut db)?;
    assert_eq!(row.name.0.as_ref(), "Canned tomatoes");
    let calories = row.calories.context("the calories were not kept")?;
    assert_eq!(Decimal::from(calories.0).to_string(), "1.2300");
    Ok(())
}

#[test_case(NutritionBasis::Per100Grams, "0", "1.2300"; "mass basis and zero")]
#[test_case(NutritionBasis::Per100Millilitres, "12.345678", "2.500"; "volume basis and fractions")]
fn typed_rows_round_trip_domain_and_order(
    basis: NutritionBasis,
    calories: &str,
    grams: &str,
) -> anyhow::Result<()> {
    let mut db = connection()?;
    let id = IngredientId::generate();
    let data = write_data(basis, calories, grams)?;
    let row = NewIngredientRow::from_write(id, &data);
    assert_eq!(row.name.0.as_ref(), "Canned tomatoes");
    assert_eq!(row.name_key, "canned tomatoes");
    diesel::insert_into(i::table).values(row).execute(&mut db)?;
    let parent = i::table
        .select(IngredientRow::as_select())
        .first::<IngredientRow>(&mut db)?;
    let first = UnitRow::from_domain(parent.local_id, 1, &data.units()[0])?;
    let second = UnitRow::from_domain(
        parent.local_id,
        0,
        &ReferenceUnit {
            name: UnitName::try_new("cup")?,
            grams: None,
            millilitres: Some(Decimal::from_str_exact("250.00")?.try_into()?),
        },
    )?;
    diesel::insert_into(u::table)
        .values(&first)
        .execute(&mut db)?;
    diesel::insert_into(u::table)
        .values(&second)
        .execute(&mut db)?;
    let units = u::table
        .order(u::position)
        .select(UnitRow::as_select())
        .load::<UnitRow>(&mut db)?;
    let positions_and_units: Vec<_> = units
        .into_iter()
        .map(UnitRow::into_domain)
        .collect::<Result<_, _>>()?;
    assert_eq!(
        positions_and_units
            .iter()
            .map(|(position, unit)| (*position, unit.name.as_ref()))
            .collect::<Vec<_>>(),
        [(0, "cup"), (1, "can")]
    );
    let domain = parent.into_domain(
        positions_and_units
            .into_iter()
            .map(|(_, unit)| unit)
            .collect(),
    );
    assert_eq!(domain.id, id);
    assert_eq!(domain.nutrition.basis, basis);
    let stored_calories = domain.nutrition.calories.context("no calories")?;
    assert_eq!(Decimal::from(stored_calories).to_string(), calories);
    let cup = domain.units[0].millilitres.context("no cup millilitres")?;
    assert_eq!(Decimal::from(cup).to_string(), "250.00");
    let can = domain.units[1].grams.context("no can grams")?;
    assert_eq!(Decimal::from(can).to_string(), grams);
    assert!(domain.nutrition.protein.is_none());
    assert!(domain.nutrition.fat.is_none());
    Ok(())
}

#[test]
fn full_decimal_range_and_nullable_values_round_trip() -> anyhow::Result<()> {
    let mut db = connection()?;
    let id = IngredientId::generate();
    let row = NewIngredientRow {
        remote_id: id.into(),
        name: DbIngredientName(IngredientName::try_new("Garlic")?),
        name_key: "garlic".into(),
        notes: None,
        basis: NutritionBasis::Per100Grams.into(),
        calories: Some(DbCalories(Decimal::MAX.try_into()?)),
        protein: Some(DbGrams(
            Decimal::from_str_exact("0.0000000000000000000000000001")?.try_into()?,
        )),
        carbohydrates: Some(DbGrams(Decimal::ZERO.try_into()?)),
        fat: None,
    };
    diesel::insert_into(i::table).values(row).execute(&mut db)?;
    let row = i::table
        .select(IngredientRow::as_select())
        .first::<IngredientRow>(&mut db)?;
    let item: Ingredient = row.into_domain(vec![]);
    assert_eq!(item.id, id);
    assert_eq!(item.notes, None);
    let calories = item.nutrition.calories.context("no calories")?;
    assert_eq!(Decimal::from(calories), Decimal::MAX);
    let protein = item.nutrition.protein.context("no protein")?;
    assert_eq!(Decimal::from(protein).scale(), 28);
    let carbohydrates = item.nutrition.carbohydrates.context("no carbohydrates")?;
    assert_eq!(Decimal::from(carbohydrates), Decimal::ZERO);
    assert!(item.nutrition.fat.is_none());
    Ok(())
}

// Validation

#[test_case(2; "unknown basis")]
#[test_case(-1; "negative basis")]
fn invalid_basis_is_rejected(value: i32) -> anyhow::Result<()> {
    assert_eq!(
        DbNutritionBasis::try_from(value),
        Err(ModelError::Basis(value))
    );
    let mut db = connection()?;
    let id = IngredientId::generate().to_string();
    assert!(
        diesel::insert_into(i::table)
            .values((
                i::remote_id.eq(id),
                i::name.eq("garlic"),
                i::name_key.eq("garlic"),
                i::basis.eq(value)
            ))
            .execute(&mut db)
            .is_err()
    );
    Ok(())
}

#[test]
fn invalid_unit_values_are_rejected_by_row_mapping() -> anyhow::Result<()> {
    let parent = LocalIngredientId(1);
    let unit = ReferenceUnit {
        name: UnitName::try_new("cup")?,
        grams: Some(Decimal::ZERO.try_into()?),
        millilitres: None,
    };
    assert_eq!(
        UnitRow::from_domain(parent, 0, &unit),
        Err(ModelError::NonPositiveMeasurement)
    );
    let unit = ReferenceUnit {
        name: UnitName::try_new("cup")?,
        grams: None,
        millilitres: None,
    };
    assert_eq!(
        UnitRow::from_domain(parent, 0, &unit),
        Err(ModelError::MissingMeasurement)
    );
    let row = UnitRow {
        ingredient_local_id: parent,
        position: UnitPosition(0),
        name: DbUnitName(UnitName::try_new("cup")?),
        name_key: "cup".into(),
        grams: Some(DbGrams(Decimal::ZERO.try_into()?)),
        millilitres: None,
    };
    assert_eq!(row.into_domain(), Err(ModelError::NonPositiveMeasurement));
    assert_eq!(
        usize::try_from(UnitPosition(-1)),
        Err(ModelError::NegativePosition)
    );
    assert_eq!(
        UnitPosition::try_from(usize::MAX),
        Err(ModelError::PositionOverflow)
    );
    Ok(())
}

#[test]
fn constraints_and_cascade_work_in_sqlite() -> anyhow::Result<()> {
    let mut db = connection()?;
    let id = IngredientId::generate();
    let parent = insert_parent(&mut db, id, "Garlic")?;
    let duplicate_id =
        NewIngredientRow::from_write(id, &write_data(NutritionBasis::Per100Grams, "0", "1")?);
    assert!(
        diesel::insert_into(i::table)
            .values(duplicate_id)
            .execute(&mut db)
            .is_err()
    );
    let other_id = IngredientId::generate();
    let mut duplicate_key = NewIngredientRow::from_write(
        other_id,
        &write_data(NutritionBasis::Per100Grams, "0", "1")?,
    );
    duplicate_key.name = DbIngredientName(IngredientName::try_new(" GARLIC ")?);
    duplicate_key.name_key = name_key(duplicate_key.name.0.as_ref());
    assert!(
        diesel::insert_into(i::table)
            .values(duplicate_key)
            .execute(&mut db)
            .is_err()
    );
    let unit = UnitRow {
        ingredient_local_id: parent,
        position: UnitPosition(0),
        name: DbUnitName(UnitName::try_new("can")?),
        name_key: "can".into(),
        grams: Some(DbGrams(Decimal::ONE.try_into()?)),
        millilitres: None,
    };
    diesel::insert_into(u::table)
        .values(&unit)
        .execute(&mut db)?;
    let mut duplicate_position = unit.clone();
    duplicate_position.name_key = "other".into();
    assert!(
        diesel::insert_into(u::table)
            .values(duplicate_position)
            .execute(&mut db)
            .is_err()
    );
    let mut duplicate_unit_name = unit.clone();
    duplicate_unit_name.position = UnitPosition(1);
    assert!(
        diesel::insert_into(u::table)
            .values(duplicate_unit_name)
            .execute(&mut db)
            .is_err()
    );
    let mut orphan = unit.clone();
    orphan.ingredient_local_id = LocalIngredientId(parent.0 + 100);
    assert!(
        diesel::insert_into(u::table)
            .values(orphan)
            .execute(&mut db)
            .is_err()
    );
    let missing = diesel::insert_into(u::table)
        .values((
            u::ingredient_local_id.eq(parent),
            u::position.eq(UnitPosition(2)),
            u::name.eq("empty"),
            u::name_key.eq("empty"),
        ))
        .execute(&mut db);
    assert!(missing.is_err());
    let negative_position = diesel::insert_into(u::table)
        .values((
            u::ingredient_local_id.eq(parent),
            u::position.eq(UnitPosition(-1)),
            u::name.eq("bad"),
            u::name_key.eq("bad"),
            u::grams.eq(Some("1")),
        ))
        .execute(&mut db);
    assert!(negative_position.is_err());
    diesel::delete(i::table.filter(i::local_id.eq(parent))).execute(&mut db)?;
    assert_eq!(u::table.count().get_result::<i64>(&mut db)?, 0);
    Ok(())
}

// Red cases

#[test]
fn a_stored_zero_measurement_fails_the_domain_mapping() -> anyhow::Result<()> {
    let mut db = connection()?;
    insert_raw_unit_grams(&mut db, "0")?;
    let row = u::table
        .select(UnitRow::as_select())
        .first::<UnitRow>(&mut db)?;
    assert_eq!(row.into_domain(), Err(ModelError::NonPositiveMeasurement));
    Ok(())
}

#[test_case("-1"; "negative measurement")]
#[test_case("bad"; "malformed measurement")]
fn a_stored_unreadable_measurement_fails_the_row_read(raw: &str) -> anyhow::Result<()> {
    let mut db = connection()?;
    insert_raw_unit_grams(&mut db, raw)?;
    let result = u::table
        .select(UnitRow::as_select())
        .first::<UnitRow>(&mut db);
    assert!(matches!(result, Err(DieselError::DeserializationError(_))));
    Ok(())
}

#[test_case("bad", "0"; "malformed uuid")]
#[test_case("00000000-0000-0000-0000-000000000001", "oops"; "malformed decimal")]
#[test_case("00000000-0000-0000-0000-000000000001", "79228162514264337593543950336"; "decimal overflow")]
#[test_case("00000000-0000-0000-0000-000000000001", "-1"; "negative nutrition")]
fn malformed_storage_is_rejected_by_typed_row(
    remote_id: &str,
    calories: &str,
) -> anyhow::Result<()> {
    let mut db = connection()?;
    diesel::insert_into(i::table)
        .values((
            i::remote_id.eq(remote_id),
            i::name.eq("garlic"),
            i::name_key.eq("garlic"),
            i::basis.eq(0),
            i::calories.eq(Some(calories)),
        ))
        .execute(&mut db)?;
    let result = i::table
        .select(IngredientRow::as_select())
        .first::<IngredientRow>(&mut db);
    assert!(matches!(result, Err(DieselError::DeserializationError(_))));
    Ok(())
}

// Test support

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

fn connection() -> anyhow::Result<SqliteConnection> {
    let mut db = SqliteConnection::establish(":memory:")?;
    db.batch_execute("PRAGMA foreign_keys = ON;")?;
    migrate(&mut db)?;
    Ok(db)
}

/// Runs the pending migrations and returns how many ran.
fn migrate(db: &mut SqliteConnection) -> anyhow::Result<usize> {
    let applied = db
        .run_pending_migrations(MIGRATIONS)
        .map_err(|error| anyhow::anyhow!(error))?;
    Ok(applied.len())
}

fn write_data(
    basis: NutritionBasis,
    calories: &str,
    grams: &str,
) -> anyhow::Result<IngredientWriteData> {
    let mut draft = IngredientDraft::new(basis);
    draft.name = "  Canned tomatoes  ".into();
    draft.notes = "Whole".into();
    draft.calories = calories.into();
    draft.units.push(ReferenceUnitDraft {
        name: "  can  ".into(),
        grams: grams.into(),
        millilitres: String::new(),
    });
    let write = IngredientWrite::try_from(&draft)
        .map_err(|issues| anyhow::anyhow!("the draft is invalid: {issues:?}"))?;
    let IngredientWrite::Create { data } = write else {
        anyhow::bail!("expected a create write, got {write:?}");
    };
    Ok(data)
}

fn insert_parent(
    db: &mut SqliteConnection,
    id: IngredientId,
    name: &str,
) -> anyhow::Result<LocalIngredientId> {
    let data = write_data(NutritionBasis::Per100Grams, "0", "1")?;
    let mut row = NewIngredientRow::from_write(id, &data);
    row.name = DbIngredientName(IngredientName::try_new(name)?);
    row.name_key = name_key(name);
    diesel::insert_into(i::table).values(&row).execute(db)?;
    let local_id = i::table
        .select(i::local_id)
        .filter(i::remote_id.eq(RemoteIngredientId(id)))
        .first(db)?;
    Ok(local_id)
}

/// Writes a unit row whose `grams` column holds `raw` without any check.
fn insert_raw_unit_grams(db: &mut SqliteConnection, raw: &str) -> anyhow::Result<()> {
    let parent = insert_parent(db, IngredientId::generate(), "Garlic")?;
    diesel::insert_into(u::table)
        .values((
            u::ingredient_local_id.eq(parent),
            u::position.eq(UnitPosition(0)),
            u::name.eq("clove"),
            u::name_key.eq("clove"),
            u::grams.eq(Some(raw)),
        ))
        .execute(db)?;
    Ok(())
}

fn counter_table_exists(db: &mut SqliteConnection) -> anyhow::Result<bool> {
    let exists = diesel::select(diesel::dsl::sql::<diesel::sql_types::Bool>(
        "EXISTS (SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = 'counter')",
    ))
    .get_result(db)?;
    Ok(exists)
}
