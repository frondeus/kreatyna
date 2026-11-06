//! The recipe ingredients of the recipe editor: adding, naming, moving and
//! removing ingredients (REC-07), their amount, unit and note (REC-08), and
//! the rules that Save checks. The tests drive the ingredients through the
//! editor.
use kreatyna::{
    confirmation::ConfirmationEvent,
    recipes::{
        KnownUnit, Recipe, RecipeEditorEvent, RecipeEditorOutcome, RecipeEditorStatus,
        RecipeIngredient, RecipeIngredientAmount, RecipeIngredientAmountProblem,
        RecipeIngredientId, RecipeIngredientLabel, RecipeIngredientNameError, RecipeIngredientNote,
        RecipeIngredientProblems, RecipeIngredientQuantity, RecipeIngredientSummary,
        RecipeIngredientView, RecipeIngredientsEvent,
    },
};
use krucyfiks::{Message, Widget, effects::CommandTestExt};
use rust_decimal::Decimal;
use test_case::test_case;
use uuid::Uuid;

// Green path

#[tokio::test]
async fn an_edit_editor_opens_with_the_saved_ingredients_in_list_order() -> anyhow::Result<()> {
    let f = Fixture::edit_recipe(&pasta_with_ingredients()?);
    assert_eq!(
        f.editor.view().ingredients,
        vec![shown(&penne()?), shown(&olive_oil()?)]
    );
    Ok(())
}

#[tokio::test]
async fn add_ingredient_adds_an_empty_ingredient_at_the_bottom_with_a_new_id() -> anyhow::Result<()>
{
    let mut f = Fixture::edit_recipe(&pasta_with_ingredients()?);
    let command = f.send(RecipeEditorEvent::Ingredients(RecipeIngredientsEvent::Add));
    let ingredients = f.editor.view().ingredients;
    let added = ingredients.last().map(|ingredient| ingredient.id);
    let Some(id) = added else {
        anyhow::bail!("＋ Line added no ingredient");
    };
    assert_ne!(id, penne()?.id);
    assert_ne!(id, olive_oil()?.id);
    assert_eq!(
        ingredients,
        vec![
            shown(&penne()?),
            shown(&olive_oil()?),
            RecipeIngredientView {
                id,
                name: String::new(),
                amount: String::new(),
                unit: None,
                note: String::new(),
                summary: RecipeIngredientSummary {
                    amount: None,
                    name: RecipeIngredientLabel::Unnamed,
                    note: None,
                },
                problems: no_problems(),
            },
        ]
    );
    f.render(command).await
}

#[tokio::test]
async fn the_unit_picker_offers_the_known_units_in_order() -> anyhow::Result<()> {
    let f = Fixture::new();
    assert_eq!(f.editor.view().units, units());
    Ok(())
}

#[tokio::test]
async fn amount_unit_and_note_keep_the_text_as_typed_and_render() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    let id = f.add_ingredient().await?;
    f.name_ingredient(id, " tomatoes ").await?;
    f.fill_ingredient(id, " 1,50 ", Some(KnownUnit::Kilogram), " chopped ")
        .await?;
    let shown = f.editor.view().ingredients;
    assert_eq!(
        shown,
        [RecipeIngredientView {
            id,
            name: " tomatoes ".into(),
            amount: " 1,50 ".into(),
            unit: Some(KnownUnit::Kilogram),
            note: " chopped ".into(),
            summary: RecipeIngredientSummary {
                amount: Some("1,50 kg".into()),
                name: named("tomatoes"),
                note: Some("chopped".into()),
            },
            // No Save yet, so no problem shows.
            problems: no_problems(),
        }]
    );
    Ok(())
}

#[test_case("2", None, "eggs", "", Some("2"), named("eggs"), None; "a count without a unit")]
#[test_case("", Some(KnownUnit::Gram), "flour", "", Some("g"), named("flour"), None; "a unit without an amount")]
#[test_case("", None, "", " to taste ", None, RecipeIngredientLabel::Unnamed, Some("to taste"); "only a note")]
#[test_case("1", Some(KnownUnit::Cup), "milk", "", Some("1 cup"), named("milk"), None; "one cup")]
#[test_case("2", Some(KnownUnit::Cup), "milk", "", Some("2 cups"), named("milk"), None; "two cups")]
#[test_case("lots", Some(KnownUnit::Piece), "garlic", "", Some("lots piece"), named("garlic"), None; "a unit after text that is not a number")]
#[tokio::test]
async fn the_summary_joins_the_typed_amount_the_unit_the_name_and_the_note(
    amount: &str,
    unit: Option<KnownUnit>,
    name: &str,
    note: &str,
    summary_amount: Option<&str>,
    summary_name: RecipeIngredientLabel,
    summary_note: Option<&str>,
) -> anyhow::Result<()> {
    let mut f = Fixture::new();
    let id = f.add_ingredient().await?;
    if !name.is_empty() {
        f.name_ingredient(id, name).await?;
    }
    f.fill_ingredient(id, amount, unit, note).await?;
    assert_eq!(
        summaries(&f),
        [RecipeIngredientSummary {
            amount: summary_amount.map(Into::into),
            name: summary_name,
            note: summary_note.map(Into::into),
        }]
    );
    Ok(())
}

#[test_case("300", Some(KnownUnit::Gram), " extra virgin ", measured("300", KnownUnit::Gram), Some("extra virgin"); "an amount with a unit and a note")]
#[test_case(" 1,5 ", Some(KnownUnit::Kilogram), "", measured("1.5", KnownUnit::Kilogram), None; "a comma and spaces")]
#[test_case("2", None, "", count("2"), None; "a count without a unit")]
#[test_case("", None, "to taste", Ok(RecipeIngredientQuantity::Unmeasured), Some("to taste"); "no amount")]
#[tokio::test]
async fn save_writes_the_amount_the_unit_and_the_trimmed_note(
    amount: &str,
    unit: Option<KnownUnit>,
    note: &str,
    quantity: anyhow::Result<RecipeIngredientQuantity>,
    written_note: Option<&str>,
) -> anyhow::Result<()> {
    let mut f = Fixture::new();
    f.type_title("Pasta").await?;
    let id = f.add_ingredient().await?;
    f.name_ingredient(id, "olive oil").await?;
    f.fill_ingredient(id, amount, unit, note).await?;

    let save = f.send(RecipeEditorEvent::Save);
    f.save(save, Ok(pasta())).await?;
    assert_eq!(
        f.written_ingredients,
        [vec![RecipeIngredient {
            quantity: quantity?,
            note: written_note
                .map(RecipeIngredientNote::try_new)
                .transpose()?,
            ..ingredient(id, "olive oil")?
        }]]
    );
    Ok(())
}

#[tokio::test]
async fn an_edit_editor_opens_with_the_saved_amount_unit_and_note() -> anyhow::Result<()> {
    let tomatoes = RecipeIngredient {
        quantity: measured("1.50", KnownUnit::Kilogram)?,
        note: Some(RecipeIngredientNote::try_new("chopped")?),
        ..saved_ingredient(21, "tomatoes")?
    };
    let f = Fixture::edit_recipe(&Recipe {
        ingredients: vec![tomatoes.clone()],
        ..pasta()
    });
    let shown = f.editor.view().ingredients;
    assert_eq!(
        shown,
        [RecipeIngredientView {
            id: tomatoes.id,
            name: "tomatoes".into(),
            // The stored decimal keeps its places.
            amount: "1.50".into(),
            unit: Some(KnownUnit::Kilogram),
            note: "chopped".into(),
            summary: RecipeIngredientSummary {
                amount: Some("1.50 kg".into()),
                name: named("tomatoes"),
                note: Some("chopped".into()),
            },
            problems: no_problems(),
        }]
    );
    Ok(())
}

#[tokio::test]
async fn naming_an_ingredient_keeps_the_raw_name_and_renders() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    let id = f.add_ingredient().await?;
    f.name_ingredient(id, " basil ").await?;
    assert_eq!(f.ingredient_names(), [" basil "]);
    Ok(())
}

#[tokio::test]
async fn save_writes_the_ingredients_with_trimmed_names_in_list_order() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    f.type_title("Pasta").await?;
    let penne = f.add_ingredient().await?;
    f.name_ingredient(penne, " penne pasta ").await?;
    let oil = f.add_ingredient().await?;
    f.name_ingredient(oil, "olive oil").await?;

    let save = f.send(RecipeEditorEvent::Save);
    assert_eq!(f.editor.view().status, RecipeEditorStatus::Saving);
    f.save(save, Ok(pasta())).await?;
    assert_eq!(
        f.written_ingredients,
        [vec![
            ingredient(penne, "penne pasta")?,
            ingredient(oil, "olive oil")?
        ]]
    );
    Ok(())
}

#[tokio::test]
async fn remove_removes_only_that_ingredient() -> anyhow::Result<()> {
    let mut f = Fixture::edit_recipe(&pasta_with_ingredients()?);
    f.ingredients(RecipeIngredientsEvent::Remove { id: penne()?.id })
        .await?;
    assert_eq!(f.editor.view().ingredients, vec![shown(&olive_oil()?)]);
    Ok(())
}

#[test_case(23, 21, ["basil", "penne pasta", "olive oil"]; "the last ingredient onto the first goes to the top")]
#[test_case(21, 23, ["olive oil", "basil", "penne pasta"]; "the first ingredient onto the last goes to the bottom")]
#[test_case(22, 23, ["penne pasta", "basil", "olive oil"]; "an ingredient onto the next one swaps them")]
#[test_case(22, 21, ["olive oil", "penne pasta", "basil"]; "an ingredient onto the one above swaps them")]
#[tokio::test]
async fn a_dropped_ingredient_takes_the_place_of_the_ingredient_it_lands_on(
    id: u128,
    onto: u128,
    expected: [&str; 3],
) -> anyhow::Result<()> {
    let mut f = Fixture::edit_recipe(&three_ingredients()?);
    f.ingredients(RecipeIngredientsEvent::Move {
        id: ingredient_id(id),
        onto: ingredient_id(onto),
    })
    .await?;
    assert_eq!(f.ingredient_names(), expected);
    Ok(())
}

#[tokio::test]
async fn a_moved_ingredient_keeps_its_id_and_saves_in_its_new_place() -> anyhow::Result<()> {
    let recipe = three_ingredients()?;
    let mut f = Fixture::edit_recipe(&recipe);
    f.ingredients(RecipeIngredientsEvent::Move {
        id: ingredient_id(23),
        onto: ingredient_id(21),
    })
    .await?;
    let save = f.send(RecipeEditorEvent::Save);
    f.save(save, Ok(recipe)).await?;
    assert_eq!(
        f.written_ingredients,
        [vec![basil()?, penne()?, olive_oil()?]]
    );
    Ok(())
}

// Validation

#[test_case(None; "an ingredient left empty")]
#[test_case(Some("   "); "a name of spaces")]
#[tokio::test]
async fn save_with_an_unnamed_ingredient_shows_name_required_on_that_ingredient_only(
    typed: Option<&str>,
) -> anyhow::Result<()> {
    let mut f = Fixture::edit_recipe(&pasta_with_ingredients()?);
    let id = f.add_ingredient().await?;
    if let Some(name) = typed {
        f.name_ingredient(id, name).await?;
    }
    // Before the first Save, the empty ingredient shows no problem.
    assert_eq!(problems(&f), [None, None, None]);

    let save = f.send(RecipeEditorEvent::Save);
    assert_eq!(
        f.editor.view().status,
        RecipeEditorStatus::Editing { problem: None }
    );
    assert_eq!(
        problems(&f),
        [
            None,
            None,
            Some(RecipeIngredientNameError::NotEmptyViolated)
        ]
    );
    f.render(save).await?;
    assert!(f.writes.is_empty());
    Ok(())
}

#[tokio::test]
async fn after_the_first_save_each_name_change_checks_again() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    f.type_title("Pasta").await?;
    let id = f.add_ingredient().await?;
    let save = f.send(RecipeEditorEvent::Save);
    f.render(save).await?;

    f.name_ingredient(id, "basil").await?;
    assert_eq!(problems(&f), [None]);
    f.name_ingredient(id, " ").await?;
    assert_eq!(
        problems(&f),
        [Some(RecipeIngredientNameError::NotEmptyViolated)]
    );
    Ok(())
}

#[tokio::test]
async fn an_ingredient_change_makes_cancel_ask_and_changing_it_back_does_not() -> anyhow::Result<()>
{
    let mut f = Fixture::edit_recipe(&pasta_with_ingredients()?);
    let penne = penne()?.id;
    f.name_ingredient(penne, "penne rigate").await?;
    f.request_close().await?;
    let decided = f.decide(ConfirmationEvent::Cancel).await?;
    let command = f.editor.update(Message::Outcome(decided));
    f.render(command).await?;

    f.name_ingredient(penne, "penne pasta").await?;
    let command = f.send(RecipeEditorEvent::RequestClose);
    assert_eq!(
        f.render_then_outcome(command).await?,
        RecipeEditorOutcome::Discarded
    );
    Ok(())
}

#[test_case(RecipeIngredientsEvent::SetAmount { id: ingredient_id(21), amount: "300".into() }; "a typed amount")]
#[test_case(RecipeIngredientsEvent::SetUnit { id: ingredient_id(21), unit: Some(KnownUnit::Gram) }; "a picked unit")]
#[test_case(RecipeIngredientsEvent::SetNote { id: ingredient_id(21), note: "rigate".into() }; "a typed note")]
#[test_case(RecipeIngredientsEvent::Add; "an added ingredient")]
#[test_case(RecipeIngredientsEvent::Remove { id: ingredient_id(21) }; "a removed ingredient")]
#[test_case(RecipeIngredientsEvent::Move { id: ingredient_id(22), onto: ingredient_id(21) }; "a moved ingredient")]
#[tokio::test]
async fn each_ingredient_change_makes_cancel_ask(
    event: RecipeIngredientsEvent,
) -> anyhow::Result<()> {
    let mut f = Fixture::edit_recipe(&pasta_with_ingredients()?);
    f.ingredients(event).await?;
    f.request_close().await
}

// Validation

#[test_case("lots", None, RecipeIngredientAmountProblem::NotANumber; "a word")]
#[test_case("1.2.3", None, RecipeIngredientAmountProblem::NotANumber; "two decimal points")]
#[test_case("½", None, RecipeIngredientAmountProblem::NotANumber; "a fraction")]
#[test_case("0.1234567", None, RecipeIngredientAmountProblem::TooManyDecimalPlaces; "seven decimal places")]
#[test_case("99999999999999999999999999999", None, RecipeIngredientAmountProblem::OutOfRange; "a number too large")]
#[test_case("0", None, RecipeIngredientAmountProblem::NotPositive; "zero")]
#[test_case("-1", Some(KnownUnit::Gram), RecipeIngredientAmountProblem::NotPositive; "a negative amount")]
#[test_case("", Some(KnownUnit::Gram), RecipeIngredientAmountProblem::MissingForUnit; "a unit without an amount")]
#[test_case(" ", Some(KnownUnit::Cup), RecipeIngredientAmountProblem::MissingForUnit; "a unit with spaces as the amount")]
#[tokio::test]
async fn save_refuses_a_bad_amount_and_shows_why_on_that_ingredient_only(
    amount: &str,
    unit: Option<KnownUnit>,
    problem: RecipeIngredientAmountProblem,
) -> anyhow::Result<()> {
    let mut f = Fixture::edit_recipe(&pasta_with_ingredients()?);
    let flour = f.add_ingredient().await?;
    f.name_ingredient(flour, "flour").await?;
    f.fill_ingredient(flour, amount, unit, "").await?;
    // Before the first Save, the amount shows no problem.
    assert_eq!(amount_problems(&f), [None, None, None]);

    let save = f.send(RecipeEditorEvent::Save);
    assert_eq!(
        f.editor.view().status,
        RecipeEditorStatus::Editing { problem: None }
    );
    assert_eq!(amount_problems(&f), [None, None, Some(problem)]);
    f.render(save).await?;
    assert!(f.writes.is_empty());
    Ok(())
}

#[tokio::test]
async fn after_the_first_save_each_amount_or_unit_change_checks_again() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    f.type_title("Pasta").await?;
    let id = f.add_ingredient().await?;
    f.name_ingredient(id, "flour").await?;
    f.fill_ingredient(id, "", Some(KnownUnit::Gram), "").await?;
    let save = f.send(RecipeEditorEvent::Save);
    f.render(save).await?;
    assert_eq!(
        amount_problems(&f),
        [Some(RecipeIngredientAmountProblem::MissingForUnit)]
    );

    f.fill_ingredient(id, "200", None, "").await?;
    assert_eq!(amount_problems(&f), [None]);
    f.ingredients(RecipeIngredientsEvent::SetUnit { id, unit: None })
        .await?;
    f.fill_ingredient(id, "0", None, "").await?;
    assert_eq!(
        amount_problems(&f),
        [Some(RecipeIngredientAmountProblem::NotPositive)]
    );
    Ok(())
}

// Red cases

#[test_case(RecipeIngredientsEvent::SetName { id: unknown_ingredient(), name: "basil".into() }; "naming an unknown ingredient")]
#[test_case(RecipeIngredientsEvent::SetName { id: ingredient_id(21), name: "penne pasta".into() }; "the same name again")]
#[test_case(RecipeIngredientsEvent::SetAmount { id: unknown_ingredient(), amount: "300".into() }; "an amount for an unknown ingredient")]
#[test_case(RecipeIngredientsEvent::SetAmount { id: ingredient_id(21), amount: String::new() }; "the same amount again")]
#[test_case(RecipeIngredientsEvent::SetUnit { id: unknown_ingredient(), unit: Some(KnownUnit::Gram) }; "a unit for an unknown ingredient")]
#[test_case(RecipeIngredientsEvent::SetUnit { id: ingredient_id(21), unit: None }; "the same unit again")]
#[test_case(RecipeIngredientsEvent::SetNote { id: unknown_ingredient(), note: "rigate".into() }; "a note for an unknown ingredient")]
#[test_case(RecipeIngredientsEvent::SetNote { id: ingredient_id(21), note: String::new() }; "the same note again")]
#[test_case(RecipeIngredientsEvent::Remove { id: unknown_ingredient() }; "removing an unknown ingredient")]
#[test_case(RecipeIngredientsEvent::Move { id: unknown_ingredient(), onto: ingredient_id(21) }; "moving an unknown ingredient")]
#[test_case(RecipeIngredientsEvent::Move { id: ingredient_id(21), onto: unknown_ingredient() }; "moving onto an unknown ingredient")]
#[test_case(RecipeIngredientsEvent::Move { id: ingredient_id(21), onto: ingredient_id(21) }; "moving an ingredient onto itself")]
#[tokio::test]
async fn an_ingredient_event_that_changes_nothing_does_not_render(
    event: RecipeIngredientsEvent,
) -> anyhow::Result<()> {
    let mut f = Fixture::edit_recipe(&pasta_with_ingredients()?);
    let before = f.editor.view();
    let command = f.send(RecipeEditorEvent::Ingredients(event));
    command.driver(&mut f.effects).step().await?.expect_done();
    assert_eq!(f.editor.view(), before);
    // Nothing changed, so Cancel closes without asking.
    let command = f.send(RecipeEditorEvent::RequestClose);
    assert_eq!(
        f.render_then_outcome(command).await?,
        RecipeEditorOutcome::Discarded
    );
    Ok(())
}

// Test support

mod recipe_editor_support;

use recipe_editor_support::*;

/// `pasta()` with penne pasta, olive oil and basil, in that order.
fn three_ingredients() -> anyhow::Result<Recipe> {
    Ok(Recipe {
        ingredients: vec![penne()?, olive_oil()?, basil()?],
        ..pasta()
    })
}

fn basil() -> anyhow::Result<RecipeIngredient> {
    saved_ingredient(23, "basil")
}

fn ingredient_id(id: u128) -> RecipeIngredientId {
    RecipeIngredientId::from(Uuid::from_u128(id))
}

fn unknown_ingredient() -> RecipeIngredientId {
    ingredient_id(99)
}

fn ingredient(id: RecipeIngredientId, name: &str) -> anyhow::Result<RecipeIngredient> {
    Ok(RecipeIngredient {
        id,
        ..saved_ingredient(0, name)?
    })
}

/// A saved ingredient with no amount and no note, as the editor shows it
/// before the first Save.
fn shown(ingredient: &RecipeIngredient) -> RecipeIngredientView {
    let name = ingredient.name.as_ref();
    RecipeIngredientView {
        id: ingredient.id,
        name: name.to_owned(),
        amount: String::new(),
        unit: None,
        note: String::new(),
        summary: RecipeIngredientSummary {
            amount: None,
            name: named(name),
            note: None,
        },
        problems: no_problems(),
    }
}

fn named(name: &str) -> RecipeIngredientLabel {
    RecipeIngredientLabel::Named { name: name.into() }
}

fn no_problems() -> RecipeIngredientProblems {
    RecipeIngredientProblems {
        name: None,
        amount: None,
    }
}

/// `amount` as exact decimal text.
fn amount(text: &str) -> anyhow::Result<RecipeIngredientAmount> {
    Ok(RecipeIngredientAmount::try_new(Decimal::from_str_exact(
        text,
    )?)?)
}

fn count(text: &str) -> anyhow::Result<RecipeIngredientQuantity> {
    Ok(RecipeIngredientQuantity::Count(amount(text)?))
}

fn measured(text: &str, unit: KnownUnit) -> anyhow::Result<RecipeIngredientQuantity> {
    Ok(RecipeIngredientQuantity::Measured {
        amount: amount(text)?,
        unit,
    })
}

fn problems(f: &Fixture) -> Vec<Option<RecipeIngredientNameError>> {
    f.editor
        .view()
        .ingredients
        .into_iter()
        .map(|ingredient| ingredient.problems.name)
        .collect()
}

fn amount_problems(f: &Fixture) -> Vec<Option<RecipeIngredientAmountProblem>> {
    f.editor
        .view()
        .ingredients
        .into_iter()
        .map(|ingredient| ingredient.problems.amount)
        .collect()
}

fn summaries(f: &Fixture) -> Vec<RecipeIngredientSummary> {
    f.editor
        .view()
        .ingredients
        .into_iter()
        .map(|ingredient| ingredient.summary)
        .collect()
}
