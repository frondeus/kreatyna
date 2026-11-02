use anyhow::Context;
use kreatyna::ingredients::{
    Grams, GramsError, Ingredient, IngredientId, Kilocalories, KilocaloriesError, Millilitres,
    MillilitresError, Nutrition, NutritionBasis, ReferenceUnit, name_key,
};
use rust_decimal::Decimal;
use test_case::test_case;
use uuid::{Variant, Version};

// Green path

#[test]
fn ingredient_identity_is_stable_and_explicit() -> anyhow::Result<()> {
    let first = IngredientId::generate();
    let second = IngredientId::generate();
    assert_ne!(first, second);
    let uuid: uuid::Uuid = first.into();
    assert_eq!(uuid.get_version(), Some(Version::Random));
    assert_eq!(uuid.get_variant(), Variant::RFC4122);
    assert_eq!(IngredientId::from(uuid), first);
    assert_eq!(first.to_string().parse::<IngredientId>()?, first);
    assert_eq!(first.to_string().len(), 36);
    Ok(())
}

#[test_case("0"; "zero")]
#[test_case("0.00"; "scaled zero")]
#[test_case("12.3400"; "fraction with trailing zeros")]
#[test_case("0.0000000000000000000000000001"; "smallest increment")]
#[test_case("79228162514264337593543950335"; "maximum magnitude")]
fn quantities_preserve_decimal_value_and_scale(text: &str) -> anyhow::Result<()> {
    let decimal = Decimal::from_str_exact(text)?;
    let grams: Grams = decimal.try_into()?;
    let millilitres: Millilitres = decimal.try_into()?;
    let calories: Kilocalories = decimal.try_into()?;
    for result in [
        Decimal::from(grams),
        Decimal::from(millilitres),
        Decimal::from(calories),
    ] {
        assert_eq!(result, decimal);
        assert_eq!(result.scale(), decimal.scale());
        assert_eq!(result.to_string(), text);
    }
    assert_eq!(grams.to_string(), text);
    Ok(())
}

#[test]
fn ingredient_keeps_unknown_and_known_zero_distinct_and_units_ordered() -> anyhow::Result<()> {
    let zero = Decimal::ZERO;
    let item = Ingredient {
        id: IngredientId::generate(),
        name: "Canned tomatoes".into(),
        notes: None,
        nutrition: Nutrition {
            basis: NutritionBasis::Per100Grams,
            calories: Some(zero.try_into()?),
            protein: None,
            carbohydrates: Some(zero.try_into()?),
            fat: None,
        },
        units: vec![
            ReferenceUnit {
                name: "clove".into(),
                grams: Some(Decimal::ONE.try_into()?),
                millilitres: None,
            },
            ReferenceUnit {
                name: "scoop".into(),
                grams: None,
                millilitres: Some(Decimal::ONE.try_into()?),
            },
            ReferenceUnit {
                name: "can".into(),
                grams: Some(Decimal::ONE.try_into()?),
                millilitres: Some(Decimal::ONE.try_into()?),
            },
        ],
    };
    let copy = item.clone();
    assert_eq!(copy.id, item.id);
    let calories = copy.nutrition.calories.context("no calories")?;
    assert_eq!(Decimal::from(calories), zero);
    assert!(copy.nutrition.protein.is_none());
    assert_eq!(
        copy.units
            .iter()
            .map(|unit| unit.name.as_str())
            .collect::<Vec<_>>(),
        vec!["clove", "scoop", "can"]
    );
    let volume_based = Nutrition {
        basis: NutritionBasis::Per100Millilitres,
        ..copy.nutrition.clone()
    };
    assert_eq!(volume_based.calories, copy.nutrition.calories);
    Ok(())
}

#[test_case("  GARLIC\u{2003}", "garlic"; "trims and folds")]
#[test_case(" \u{2003} ", ""; "empty after trimming")]
fn name_key_produces_expected_key(input: &str, expected: &str) {
    assert_eq!(name_key(input), expected);
}

#[test_case("Straße", "STRASSE"; "sharp s")]
#[test_case("Σ", "σ"; "capital sigma")]
#[test_case("ς", "σ"; "final sigma")]
fn name_key_folds_case_variants_to_one_key(left: &str, right: &str) {
    assert_eq!(name_key(left), name_key(right));
}

#[test_case("red  pepper", "red pepper"; "internal spacing")]
#[test_case("café", "cafe"; "accents")]
#[test_case("a-b", "ab"; "punctuation")]
fn name_key_keeps_spacing_accents_and_punctuation(left: &str, right: &str) {
    assert_ne!(name_key(left), name_key(right));
}

#[test]
fn name_key_is_idempotent() {
    assert_eq!(name_key(&name_key("Straße")), name_key("Straße"));
}

// Validation

#[test]
fn an_invalid_id_text_does_not_parse() {
    assert!("not-an-id".parse::<IngredientId>().is_err());
}

#[test]
fn a_quantity_is_never_negative() {
    let minus_one = Decimal::NEGATIVE_ONE;
    assert_eq!(
        Grams::try_new(minus_one),
        Err(GramsError::GreaterOrEqualViolated)
    );
    assert_eq!(
        Millilitres::try_new(minus_one),
        Err(MillilitresError::GreaterOrEqualViolated)
    );
    assert_eq!(
        Kilocalories::try_new(minus_one),
        Err(KilocaloriesError::GreaterOrEqualViolated)
    );
}
