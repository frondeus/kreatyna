use std::str::FromStr;

use kreatyna::ingredients::{
    Grams, Ingredient, IngredientId, Kilocalories, Millilitres, Nutrition, NutritionBasis,
    ReferenceUnit, name_key,
};
use rust_decimal::Decimal;
use test_case::test_case;
use uuid::{Variant, Version};

#[test]
fn ingredient_identity_is_stable_and_explicit() {
    let first = IngredientId::new();
    let second = IngredientId::new();
    assert_ne!(first, second);
    let uuid: uuid::Uuid = first.into();
    assert_eq!(uuid.get_version(), Some(Version::Random));
    assert_eq!(uuid.get_variant(), Variant::RFC4122);
    assert_eq!(IngredientId::from(uuid), first);
    assert_eq!(IngredientId::from_str(&first.to_string()).unwrap(), first);
    assert_eq!(first.to_string().len(), 36);
    assert!(IngredientId::from_str("not-an-id").is_err());
}

#[test_case("0"; "zero")]
#[test_case("0.00"; "scaled zero")]
#[test_case("12.3400"; "fraction with trailing zeros")]
#[test_case("0.0000000000000000000000000001"; "smallest increment")]
#[test_case("79228162514264337593543950335"; "maximum magnitude")]
fn quantities_preserve_decimal_value_and_scale(text: &str) {
    let decimal = Decimal::from_str_exact(text).unwrap();
    let grams: Grams = decimal.into();
    let millilitres: Millilitres = decimal.into();
    let calories: Kilocalories = decimal.into();
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
}

#[test]
fn ingredient_keeps_unknown_and_known_zero_distinct_and_units_ordered() {
    let zero = Decimal::ZERO;
    let item = Ingredient {
        id: IngredientId::new(),
        name: "Canned tomatoes".into(),
        notes: None,
        nutrition: Nutrition {
            basis: NutritionBasis::Per100Grams,
            calories: Some(zero.into()),
            protein: None,
            carbohydrates: Some(zero.into()),
            fat: None,
        },
        units: vec![
            ReferenceUnit {
                name: "clove".into(),
                grams: Some(Decimal::ONE.into()),
                millilitres: None,
            },
            ReferenceUnit {
                name: "scoop".into(),
                grams: None,
                millilitres: Some(Decimal::ONE.into()),
            },
            ReferenceUnit {
                name: "can".into(),
                grams: Some(Decimal::ONE.into()),
                millilitres: Some(Decimal::ONE.into()),
            },
        ],
    };
    let copy = item.clone();
    assert_eq!(copy.id, item.id);
    assert_eq!(Decimal::from(copy.nutrition.calories.unwrap()), zero);
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
}

#[test_case("  GARLIC\u{2003}", "garlic"; "trims and folds")]
#[test_case(" \u{2003} ", ""; "empty after trimming")]
fn name_key_produces_expected_key(input: &str, expected: &str) {
    assert_eq!(name_key(input), expected);
}

#[test_case("Straße", "STRASSE", true; "sharp s")]
#[test_case("Σ", "σ", true; "capital sigma")]
#[test_case("ς", "σ", true; "final sigma")]
#[test_case("red  pepper", "red pepper", false; "internal spacing")]
#[test_case("café", "cafe", false; "accents")]
#[test_case("a-b", "ab", false; "punctuation")]
fn name_key_compares_names(left: &str, right: &str, equal: bool) {
    assert_eq!(name_key(left) == name_key(right), equal);
}

#[test]
fn name_key_is_idempotent() {
    assert_eq!(name_key(&name_key("Straße")), name_key("Straße"));
}
