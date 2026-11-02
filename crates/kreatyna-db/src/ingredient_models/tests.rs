use diesel::{
    Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection,
    connection::SimpleConnection, result::Error as DieselError,
};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use kreatyna::ingredients::{
    Ingredient, IngredientDraft, IngredientId, IngredientWrite, NutritionBasis, ReferenceUnitDraft,
};
use rust_decimal::Decimal;
use test_case::test_case;

use super::*;
use crate::schema::{ingredient_units as u, ingredients as i};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

fn connection() -> SqliteConnection {
    let mut db = SqliteConnection::establish(":memory:").unwrap();
    db.batch_execute("PRAGMA foreign_keys = ON;").unwrap();
    db.run_pending_migrations(MIGRATIONS).unwrap();
    db
}

fn write_data(basis: NutritionBasis, calories: &str, grams: &str) -> IngredientWriteData {
    let mut draft = IngredientDraft::new(basis);
    draft.name = "  Canned tomatoes  ".into();
    draft.notes = "Whole".into();
    draft.calories = calories.into();
    draft.units.push(ReferenceUnitDraft {
        name: "  can  ".into(),
        grams: grams.into(),
        millilitres: String::new(),
    });
    let IngredientWrite::Create { data } = IngredientWrite::try_from(&draft).unwrap() else {
        panic!("expected create")
    };
    data
}

fn insert_parent(db: &mut SqliteConnection, id: IngredientId, name: &str) -> LocalIngredientId {
    let data = write_data(NutritionBasis::Per100Grams, "0", "1");
    let mut row = NewIngredientRow::from_write(id, &data).unwrap();
    row.name = name.into();
    row.name_key = name_key(name);
    diesel::insert_into(i::table)
        .values(&row)
        .execute(db)
        .unwrap();
    i::table
        .select(i::local_id)
        .filter(i::remote_id.eq(RemoteIngredientId(id)))
        .first(db)
        .unwrap()
}

fn counter_table_exists(db: &mut SqliteConnection) -> bool {
    diesel::select(diesel::dsl::sql::<diesel::sql_types::Bool>(
        "EXISTS (SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = 'counter')",
    ))
    .get_result(db)
    .unwrap()
}

#[test]
fn empty_database_migrates_and_rerun_is_harmless() {
    let mut db = SqliteConnection::establish(":memory:").unwrap();
    db.batch_execute("PRAGMA foreign_keys = ON;").unwrap();
    assert_eq!(db.run_pending_migrations(MIGRATIONS).unwrap().len(), 4);
    assert!(db.run_pending_migrations(MIGRATIONS).unwrap().is_empty());
    assert_eq!(i::table.count().get_result::<i64>(&mut db).unwrap(), 0);
    assert_eq!(u::table.count().get_result::<i64>(&mut db).unwrap(), 0);
    assert!(!counter_table_exists(&mut db));
}

#[test]
fn counter_database_upgrades_and_keeps_ingredients() {
    let path = std::env::temp_dir().join(format!(
        "kreatyna-mac03-{}-{}.sqlite",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    {
        let mut db = SqliteConnection::establish(path.to_str().unwrap()).unwrap();
        db.batch_execute("PRAGMA foreign_keys = ON;").unwrap();
        // The schema before MAC-03: the counter and ingredient migrations only.
        db.run_next_migration(MIGRATIONS).unwrap();
        db.run_next_migration(MIGRATIONS).unwrap();
        db.batch_execute("INSERT INTO counter (id, value) VALUES (1, 27);")
            .unwrap();
        let data = write_data(NutritionBasis::Per100Grams, "1.2300", "2");
        diesel::insert_into(i::table)
            .values(NewIngredientRow::from_write(IngredientId::generate(), &data).unwrap())
            .execute(&mut db)
            .unwrap();

        // MAC-03 drops the counter; REC-01 adds recipes.
        assert_eq!(db.run_pending_migrations(MIGRATIONS).unwrap().len(), 2);
        assert!(!counter_table_exists(&mut db));
    }
    {
        let mut db = SqliteConnection::establish(path.to_str().unwrap()).unwrap();
        db.batch_execute("PRAGMA foreign_keys = ON;").unwrap();
        assert!(db.run_pending_migrations(MIGRATIONS).unwrap().is_empty());
        let row = i::table
            .select(IngredientRow::as_select())
            .first::<IngredientRow>(&mut db)
            .unwrap();
        assert_eq!(row.name, "Canned tomatoes");
        assert_eq!(Decimal::from(row.calories.unwrap().0).to_string(), "1.2300");
    }
    std::fs::remove_file(path).unwrap();
}

#[test_case(NutritionBasis::Per100Grams, "0", "1.2300"; "mass basis and zero")]
#[test_case(NutritionBasis::Per100Millilitres, "12.345678", "2.500"; "volume basis and fractions")]
fn typed_rows_round_trip_domain_and_order(basis: NutritionBasis, calories: &str, grams: &str) {
    let mut db = connection();
    let id = IngredientId::generate();
    let data = write_data(basis, calories, grams);
    let row = NewIngredientRow::from_write(id, &data).unwrap();
    assert_eq!(row.name, "Canned tomatoes");
    assert_eq!(row.name_key, "canned tomatoes");
    diesel::insert_into(i::table)
        .values(row)
        .execute(&mut db)
        .unwrap();
    let parent = i::table
        .select(IngredientRow::as_select())
        .first::<IngredientRow>(&mut db)
        .unwrap();
    let first = UnitRow::from_domain(parent.local_id, 1, &data.units()[0]).unwrap();
    let second = UnitRow::from_domain(
        parent.local_id,
        0,
        &kreatyna::ingredients::ReferenceUnit {
            name: "cup".into(),
            grams: None,
            millilitres: Some(Decimal::from_str_exact("250.00").unwrap().into()),
        },
    )
    .unwrap();
    diesel::insert_into(u::table)
        .values(&first)
        .execute(&mut db)
        .unwrap();
    diesel::insert_into(u::table)
        .values(&second)
        .execute(&mut db)
        .unwrap();
    let units = u::table
        .order(u::position)
        .select(UnitRow::as_select())
        .load::<UnitRow>(&mut db)
        .unwrap();
    let positions_and_units: Vec<_> = units
        .into_iter()
        .map(UnitRow::into_domain)
        .collect::<Result<_, _>>()
        .unwrap();
    assert_eq!(
        positions_and_units
            .iter()
            .map(|(position, unit)| (*position, unit.name.as_str()))
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
    assert_eq!(
        Decimal::from(domain.nutrition.calories.unwrap()).to_string(),
        calories
    );
    assert_eq!(
        Decimal::from(domain.units[0].millilitres.unwrap()).to_string(),
        "250.00"
    );
    assert_eq!(
        Decimal::from(domain.units[1].grams.unwrap()).to_string(),
        grams
    );
    assert!(domain.nutrition.protein.is_none());
    assert_eq!(
        Decimal::from(domain.nutrition.fat.unwrap_or(Decimal::ZERO.into())),
        Decimal::ZERO
    );
}

#[test]
fn full_decimal_range_and_nullable_values_round_trip() {
    let mut db = connection();
    let id = IngredientId::generate();
    let row = NewIngredientRow {
        remote_id: id.into(),
        name: "Garlic".into(),
        name_key: "garlic".into(),
        notes: None,
        basis: NutritionBasis::Per100Grams.into(),
        calories: Some(DbCalories(Decimal::MAX.into())),
        protein: Some(DbGrams(
            Decimal::from_str_exact("0.0000000000000000000000000001")
                .unwrap()
                .into(),
        )),
        carbohydrates: Some(DbGrams(Decimal::ZERO.into())),
        fat: None,
    };
    diesel::insert_into(i::table)
        .values(row)
        .execute(&mut db)
        .unwrap();
    let row = i::table
        .select(IngredientRow::as_select())
        .first::<IngredientRow>(&mut db)
        .unwrap();
    let item: Ingredient = row.into_domain(vec![]);
    assert_eq!(item.id, id);
    assert_eq!(item.notes, None);
    assert_eq!(
        Decimal::from(item.nutrition.calories.unwrap()),
        Decimal::MAX
    );
    assert_eq!(Decimal::from(item.nutrition.protein.unwrap()).scale(), 28);
    assert_eq!(
        Decimal::from(item.nutrition.carbohydrates.unwrap()),
        Decimal::ZERO
    );
    assert!(item.nutrition.fat.is_none());
}

#[test_case("0", false; "zero measurement")]
#[test_case("-1", true; "negative measurement")]
#[test_case("bad", true; "malformed measurement")]
fn stored_reference_measurements_are_checked(raw: &str, rejects_during_read: bool) {
    let mut db = connection();
    let parent = insert_parent(&mut db, IngredientId::generate(), "Garlic");
    diesel::insert_into(u::table)
        .values((
            u::ingredient_local_id.eq(parent),
            u::position.eq(UnitPosition(0)),
            u::name.eq("clove"),
            u::name_key.eq("clove"),
            u::grams.eq(Some(raw)),
        ))
        .execute(&mut db)
        .unwrap();
    let result = u::table
        .select(UnitRow::as_select())
        .first::<UnitRow>(&mut db);
    if rejects_during_read {
        assert!(matches!(result, Err(DieselError::DeserializationError(_))));
    } else {
        assert_eq!(
            result.unwrap().into_domain(),
            Err(ModelError::NonPositiveMeasurement)
        );
    }
}

#[test_case("bad", "0", false; "malformed uuid")]
#[test_case("00000000-0000-0000-0000-000000000001", "oops", true; "malformed decimal")]
#[test_case("00000000-0000-0000-0000-000000000001", "79228162514264337593543950336", true; "decimal overflow")]
#[test_case("00000000-0000-0000-0000-000000000001", "-1", true; "negative nutrition")]
fn malformed_storage_is_rejected_by_typed_row(remote_id: &str, calories: &str, valid_id: bool) {
    let mut db = connection();
    diesel::insert_into(i::table)
        .values((
            i::remote_id.eq(remote_id),
            i::name.eq("garlic"),
            i::name_key.eq("garlic"),
            i::basis.eq(0),
            i::calories.eq(Some(calories)),
        ))
        .execute(&mut db)
        .unwrap();
    let result = i::table
        .select(IngredientRow::as_select())
        .first::<IngredientRow>(&mut db);
    assert!(matches!(result, Err(DieselError::DeserializationError(_))));
    assert_eq!(remote_id.parse::<IngredientId>().is_ok(), valid_id);
}

#[test_case(2; "unknown basis")]
#[test_case(-1; "negative basis")]
fn invalid_basis_is_rejected(value: i32) {
    assert_eq!(
        DbNutritionBasis::try_from(value),
        Err(ModelError::Basis(value))
    );
    let mut db = connection();
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
}

#[test]
fn invalid_unit_values_are_rejected_by_row_mapping() {
    let parent = LocalIngredientId(1);
    let unit = kreatyna::ingredients::ReferenceUnit {
        name: "cup".into(),
        grams: Some(Decimal::ZERO.into()),
        millilitres: None,
    };
    assert_eq!(
        UnitRow::from_domain(parent, 0, &unit),
        Err(ModelError::NonPositiveMeasurement)
    );
    let unit = kreatyna::ingredients::ReferenceUnit {
        name: "cup".into(),
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
        name: "cup".into(),
        name_key: "cup".into(),
        grams: Some(DbGrams(Decimal::ZERO.into())),
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
}

#[test]
fn constraints_and_cascade_work_in_sqlite() {
    let mut db = connection();
    let id = IngredientId::generate();
    let parent = insert_parent(&mut db, id, "Garlic");
    let duplicate_id =
        NewIngredientRow::from_write(id, &write_data(NutritionBasis::Per100Grams, "0", "1"))
            .unwrap();
    assert!(
        diesel::insert_into(i::table)
            .values(duplicate_id)
            .execute(&mut db)
            .is_err()
    );
    let other_id = IngredientId::generate();
    let mut duplicate_key =
        NewIngredientRow::from_write(other_id, &write_data(NutritionBasis::Per100Grams, "0", "1"))
            .unwrap();
    duplicate_key.name = " GARLIC ".into();
    duplicate_key.name_key = name_key(&duplicate_key.name);
    assert!(
        diesel::insert_into(i::table)
            .values(duplicate_key)
            .execute(&mut db)
            .is_err()
    );
    let unit = UnitRow {
        ingredient_local_id: parent,
        position: UnitPosition(0),
        name: "can".into(),
        name_key: "can".into(),
        grams: Some(DbGrams(Decimal::ONE.into())),
        millilitres: None,
    };
    diesel::insert_into(u::table)
        .values(&unit)
        .execute(&mut db)
        .unwrap();
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
    diesel::delete(i::table.filter(i::local_id.eq(parent)))
        .execute(&mut db)
        .unwrap();
    assert_eq!(u::table.count().get_result::<i64>(&mut db).unwrap(), 0);
}
