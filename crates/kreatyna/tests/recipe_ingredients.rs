//! The recipe ingredients of the recipe editor (REC-07): adding, naming,
//! moving and removing ingredients, and the ingredient rule that Save checks. The tests
//! drive the ingredients through the editor.
use kreatyna::{
    confirmation::ConfirmationEvent,
    recipes::{
        Recipe, RecipeEditorEvent, RecipeEditorOutcome, RecipeEditorStatus, RecipeIngredient,
        RecipeIngredientId, RecipeIngredientNameError, RecipeIngredientView,
        RecipeIngredientsEvent,
    },
};
use krucyfiks::{Message, Widget, effects::CommandTestExt};
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
                problem: None,
            },
        ]
    );
    f.render(command).await
}

#[tokio::test]
async fn naming_a_ingredient_keeps_the_raw_name_and_renders() -> anyhow::Result<()> {
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

#[test_case(RecipeIngredientsEvent::Add; "an added ingredient")]
#[test_case(RecipeIngredientsEvent::Remove { id: ingredient_id(21) }; "a removed ingredient")]
#[test_case(RecipeIngredientsEvent::Move { id: ingredient_id(22), onto: ingredient_id(21) }; "a moved ingredient")]
#[tokio::test]
async fn an_added_removed_or_moved_ingredient_makes_cancel_ask(
    event: RecipeIngredientsEvent,
) -> anyhow::Result<()> {
    let mut f = Fixture::edit_recipe(&pasta_with_ingredients()?);
    f.ingredients(event).await?;
    f.request_close().await
}

// Red cases

#[test_case(RecipeIngredientsEvent::SetName { id: unknown_ingredient(), name: "basil".into() }; "naming an unknown ingredient")]
#[test_case(RecipeIngredientsEvent::SetName { id: ingredient_id(21), name: "penne pasta".into() }; "the same name again")]
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

/// A saved ingredient as the editor shows it before the first Save.
fn shown(ingredient: &RecipeIngredient) -> RecipeIngredientView {
    RecipeIngredientView {
        id: ingredient.id,
        name: ingredient.name.as_ref().to_owned(),
        problem: None,
    }
}

fn problems(f: &Fixture) -> Vec<Option<RecipeIngredientNameError>> {
    f.editor
        .view()
        .ingredients
        .into_iter()
        .map(|ingredient| ingredient.problem)
        .collect()
}
