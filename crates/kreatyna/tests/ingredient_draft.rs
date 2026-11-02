use kreatyna::ingredients::*;
use rust_decimal::Decimal;
use test_case::test_case;

fn draft() -> IngredientDraft {
    let mut draft = IngredientDraft::new(NutritionBasis::Per100Grams);
    draft.name = "Garlic".into();
    draft
}

fn data(draft: &IngredientDraft) -> IngredientWriteData {
    match IngredientWrite::try_from(draft).unwrap() {
        IngredientWrite::Create { data } | IngredientWrite::Update { data, .. } => data,
    }
}

#[test_case("", None; "unknown")]
#[test_case(" \u{2003}", None; "whitespace unknown")]
#[test_case("0", Some("0"); "zero")]
#[test_case("-0.00", Some("0.00"); "negative zero")]
#[test_case(" 12,3400\u{2003}", Some("12.3400"); "comma and whitespace")]
#[test_case("12.345678", Some("12.345678"); "six digits")]
#[test_case("79228162514264337593543950335", Some("79228162514264337593543950335"); "maximum")]
fn nutrition_accepts_exact_values(input: &str, expected: Option<&str>) {
    let mut draft = draft();
    draft.calories = input.into();
    let result = data(&draft);
    let actual = result.nutrition().calories.map(Decimal::from);
    let expected = expected.map(|text| Decimal::from_str_exact(text).unwrap());
    assert_eq!(actual, expected);
    assert_eq!(actual.map(|v| v.scale()), expected.map(|v| v.scale()));
}

#[test_case("+1", ValidationReason::InvalidNumber; "plus")]
#[test_case("1e2", ValidationReason::InvalidNumber; "exponent")]
#[test_case("1 000", ValidationReason::InvalidNumber; "grouping")]
#[test_case("1,000.0", ValidationReason::InvalidNumber; "mixed separators")]
#[test_case(".5", ValidationReason::InvalidNumber; "missing integer")]
#[test_case("5.", ValidationReason::InvalidNumber; "missing fraction")]
#[test_case("--1", ValidationReason::InvalidNumber; "double minus")]
#[test_case("１２", ValidationReason::InvalidNumber; "non ascii digits")]
#[test_case("1.0000000", ValidationReason::TooManyDecimalPlaces; "seven trailing zeros")]
#[test_case("0.0000001", ValidationReason::TooManyDecimalPlaces; "seven digits")]
#[test_case("79228162514264337593543950336", ValidationReason::OutOfRange; "overflow")]
#[test_case("79228162514264337593543950335.1", ValidationReason::OutOfRange; "fraction overflow")]
#[test_case("-1", ValidationReason::MustBeNonNegative; "negative")]
#[test_case("-1.0000000", ValidationReason::TooManyDecimalPlaces; "precision before sign")]
#[test_case("-79228162514264337593543950336", ValidationReason::OutOfRange; "range before sign")]
#[test_case("1.000000x", ValidationReason::InvalidNumber; "syntax before precision")]
fn nutrition_rejects_invalid_values(input: &str, reason: ValidationReason) {
    let mut draft = draft();
    draft.protein = input.into();
    let before = draft.clone();
    assert_eq!(
        IngredientWrite::try_from(&draft),
        Err(vec![ValidationIssue {
            field: ValidationField::Protein,
            reason
        }])
    );
    assert_eq!(draft, before);
}

#[test_case("1", "", None; "mass only")]
#[test_case("", "2", None; "volume only")]
#[test_case("1", "2", None; "both")]
#[test_case("", "", Some((ValidationField::UnitMeasurements { index: 0 }, ValidationReason::Required)); "neither")]
#[test_case("0", "1", Some((ValidationField::UnitGrams { index: 0 }, ValidationReason::MustBePositive)); "zero mass")]
#[test_case("1", "-0.00", Some((ValidationField::UnitMillilitres { index: 0 }, ValidationReason::MustBePositive)); "negative zero volume")]
#[test_case("-1", "", Some((ValidationField::UnitGrams { index: 0 }, ValidationReason::MustBePositive)); "negative mass")]
#[test_case("bad", "1", Some((ValidationField::UnitGrams { index: 0 }, ValidationReason::InvalidNumber)); "malformed with valid alternative")]
fn reference_measurements(
    grams: &str,
    millilitres: &str,
    error: Option<(ValidationField, ValidationReason)>,
) {
    let mut draft = draft();
    draft.units.push(ReferenceUnitDraft {
        name: "clove".into(),
        grams: grams.into(),
        millilitres: millilitres.into(),
    });
    if let Some((field, reason)) = error {
        assert_eq!(
            IngredientWrite::try_from(&draft),
            Err(vec![ValidationIssue { field, reason }])
        );
    } else {
        let result = data(&draft);
        assert_eq!(result.units()[0].grams.is_some(), !grams.is_empty());
        assert_eq!(
            result.units()[0].millilitres.is_some(),
            !millilitres.is_empty()
        );
    }
}

#[test]
fn issues_are_complete_and_ordered() {
    let mut draft = draft();
    draft.name = " \u{2003}".into();
    draft.calories = "-1".into();
    draft.protein = "bad".into();
    draft.carbohydrates = "1.0000000".into();
    draft.fat = "+1".into();
    draft.units = vec![
        ReferenceUnitDraft::default(),
        ReferenceUnitDraft {
            name: "can".into(),
            grams: "-1".into(),
            millilitres: "bad".into(),
        },
    ];
    let pairs = [
        (ValidationField::Name, ValidationReason::Required),
        (
            ValidationField::Calories,
            ValidationReason::MustBeNonNegative,
        ),
        (ValidationField::Protein, ValidationReason::InvalidNumber),
        (
            ValidationField::Carbohydrates,
            ValidationReason::TooManyDecimalPlaces,
        ),
        (ValidationField::Fat, ValidationReason::InvalidNumber),
        (
            ValidationField::UnitName { index: 0 },
            ValidationReason::Required,
        ),
        (
            ValidationField::UnitMeasurements { index: 0 },
            ValidationReason::Required,
        ),
        (
            ValidationField::UnitGrams { index: 1 },
            ValidationReason::MustBePositive,
        ),
        (
            ValidationField::UnitMillilitres { index: 1 },
            ValidationReason::InvalidNumber,
        ),
    ];
    assert_eq!(
        IngredientWrite::try_from(&draft),
        Err(pairs
            .into_iter()
            .map(|(field, reason)| ValidationIssue { field, reason })
            .collect())
    );
}

#[test_case("Straße", " STRASSE "; "sharp s")]
#[test_case("Σ", "ς"; "sigma")]
fn duplicate_units_report_each_later_occurrence(first: &str, duplicate: &str) {
    let mut draft = draft();
    draft.units = [first, duplicate, first]
        .into_iter()
        .map(|name| ReferenceUnitDraft {
            name: name.into(),
            grams: "1".into(),
            millilitres: String::new(),
        })
        .collect();
    assert_eq!(
        IngredientWrite::try_from(&draft),
        Err(vec![
            ValidationIssue {
                field: ValidationField::UnitName { index: 1 },
                reason: ValidationReason::DuplicateName
            },
            ValidationIssue {
                field: ValidationField::UnitName { index: 2 },
                reason: ValidationReason::DuplicateName
            },
        ])
    );
}

#[test_case(" \u{2003}", None; "empty notes")]
#[test_case("  fresh\n cloves  ", Some("fresh\n cloves"); "trimmed notes")]
fn creation_trims_only_surrounding_whitespace(notes: &str, expected: Option<&str>) {
    let mut draft = draft();
    draft.name = "  Red  Pepper\u{2003}".into();
    draft.notes = notes.into();
    draft.units.push(ReferenceUnitDraft {
        name: "  Small Can ".into(),
        grams: "1".into(),
        millilitres: String::new(),
    });
    let before = draft.clone();
    let IngredientWrite::Create { data } = IngredientWrite::try_from(&draft).unwrap() else {
        panic!("expected create")
    };
    assert_eq!(draft, before);
    assert_eq!(data.name(), "Red  Pepper");
    assert_eq!(data.notes(), expected);
    assert_eq!(data.units()[0].name, "Small Can");
}

#[test_case(NutritionBasis::Per100Grams; "mass basis")]
#[test_case(NutritionBasis::Per100Millilitres; "volume basis")]
fn edit_round_trip_preserves_identity_and_exact_fields(basis: NutritionBasis) {
    let item = Ingredient {
        id: IngredientId::generate(),
        name: "Garlic".into(),
        notes: Some("Fresh".into()),
        nutrition: Nutrition {
            basis,
            calories: Some(Decimal::from_str_exact("12.3400").unwrap().into()),
            protein: None,
            carbohydrates: Some(Decimal::ZERO.into()),
            fat: None,
        },
        units: vec![
            ReferenceUnit {
                name: "clove".into(),
                grams: Some(Decimal::from_str_exact("2.500").unwrap().into()),
                millilitres: None,
            },
            ReferenceUnit {
                name: "cup".into(),
                grams: None,
                millilitres: Some(Decimal::ONE.into()),
            },
        ],
    };
    let draft = IngredientDraft::from(&item);
    assert_eq!(draft.mode, DraftMode::Edit { id: item.id });
    assert_eq!(draft.calories, "12.3400");
    assert_eq!(draft.units[0].grams, "2.500");
    assert_eq!(draft.protein, "");
    let IngredientWrite::Update { id, data } = IngredientWrite::try_from(&draft).unwrap() else {
        panic!("expected update")
    };
    assert_eq!(id, item.id);
    assert_eq!(data.nutrition(), &item.nutrition);
    assert_eq!(data.units(), item.units);
    assert_eq!(data.notes(), Some("Fresh"));
}

#[test]
fn edit_preserves_excess_precision_until_corrected() {
    let mut draft = draft();
    draft.calories = "1".into();
    let mut nutrition = data(&draft).nutrition().clone();
    nutrition.calories = Some(Decimal::from_str_exact("1.0000000").unwrap().into());
    let item = Ingredient {
        id: IngredientId::generate(),
        name: "Garlic".into(),
        notes: None,
        nutrition,
        units: vec![],
    };
    let mut draft = IngredientDraft::from(&item);
    assert_eq!(draft.calories, "1.0000000");
    assert_eq!(
        IngredientWrite::try_from(&draft),
        Err(vec![ValidationIssue {
            field: ValidationField::Calories,
            reason: ValidationReason::TooManyDecimalPlaces
        }])
    );
    draft.calories = "1.000000".into();
    assert!(IngredientWrite::try_from(&draft).is_ok());
}
