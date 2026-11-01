use kreatyna::recipes::{RecipeTitle, RecipeTitleError};
use test_case::test_case;

// Green path

#[test_case("Shakshuka", "Shakshuka"; "plain")]
#[test_case("  Shakshuka \n", "Shakshuka"; "surrounding whitespace")]
#[test_case("Shakshuka  (spicy)", "Shakshuka  (spicy)"; "inner spaces stay")]
fn a_title_is_trimmed(text: &str, title: &str) -> anyhow::Result<()> {
    assert_eq!(RecipeTitle::try_new(text)?.as_ref() as &str, title);
    Ok(())
}

// Validation

#[test_case(""; "empty")]
#[test_case("   \t"; "whitespace only")]
fn a_blank_title_is_empty(text: &str) {
    assert_eq!(
        RecipeTitle::try_new(text),
        Err(RecipeTitleError::NotEmptyViolated)
    );
}
