//! The recipe ingredients of the recipe editor (REC-07): adding, naming,
//! moving and removing lines, and the line rule that Save checks. The tests
//! drive the lines through the editor.
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
async fn an_edit_editor_opens_with_the_saved_lines_in_list_order() -> anyhow::Result<()> {
    let f = Fixture::edit_recipe(&pasta_with_lines()?);
    assert_eq!(
        f.editor.view().lines,
        vec![shown(&penne()?), shown(&olive_oil()?)]
    );
    Ok(())
}

#[tokio::test]
async fn add_line_adds_an_empty_line_at_the_bottom_with_a_new_id() -> anyhow::Result<()> {
    let mut f = Fixture::edit_recipe(&pasta_with_lines()?);
    let command = f.send(RecipeEditorEvent::Lines(RecipeIngredientsEvent::Add));
    let lines = f.editor.view().lines;
    let added = lines.last().map(|line| line.id);
    let Some(id) = added else {
        anyhow::bail!("＋ Line added no line");
    };
    assert_ne!(id, penne()?.id);
    assert_ne!(id, olive_oil()?.id);
    assert_eq!(
        lines,
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
async fn naming_a_line_keeps_the_raw_name_and_renders() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    let id = f.add_line().await?;
    f.name_line(id, " basil ").await?;
    assert_eq!(f.line_names(), [" basil "]);
    Ok(())
}

#[tokio::test]
async fn save_writes_the_lines_with_trimmed_names_in_list_order() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    f.type_title("Pasta").await?;
    let penne = f.add_line().await?;
    f.name_line(penne, " penne pasta ").await?;
    let oil = f.add_line().await?;
    f.name_line(oil, "olive oil").await?;

    let save = f.send(RecipeEditorEvent::Save);
    assert_eq!(f.editor.view().status, RecipeEditorStatus::Saving);
    f.save(save, Ok(pasta())).await?;
    assert_eq!(
        f.written_lines,
        [vec![line(penne, "penne pasta")?, line(oil, "olive oil")?]]
    );
    Ok(())
}

#[tokio::test]
async fn remove_removes_only_that_line() -> anyhow::Result<()> {
    let mut f = Fixture::edit_recipe(&pasta_with_lines()?);
    f.lines(RecipeIngredientsEvent::Remove { id: penne()?.id })
        .await?;
    assert_eq!(f.editor.view().lines, vec![shown(&olive_oil()?)]);
    Ok(())
}

#[test_case(23, 21, ["basil", "penne pasta", "olive oil"]; "the last line onto the first goes to the top")]
#[test_case(21, 23, ["olive oil", "basil", "penne pasta"]; "the first line onto the last goes to the bottom")]
#[test_case(22, 23, ["penne pasta", "basil", "olive oil"]; "a line onto the next one swaps them")]
#[test_case(22, 21, ["olive oil", "penne pasta", "basil"]; "a line onto the one above swaps them")]
#[tokio::test]
async fn a_dropped_line_takes_the_place_of_the_line_it_lands_on(
    id: u128,
    onto: u128,
    expected: [&str; 3],
) -> anyhow::Result<()> {
    let mut f = Fixture::edit_recipe(&three_lines()?);
    f.lines(RecipeIngredientsEvent::Move {
        id: line_id(id),
        onto: line_id(onto),
    })
    .await?;
    assert_eq!(f.line_names(), expected);
    Ok(())
}

#[tokio::test]
async fn a_moved_line_keeps_its_id_and_saves_in_its_new_place() -> anyhow::Result<()> {
    let recipe = three_lines()?;
    let mut f = Fixture::edit_recipe(&recipe);
    f.lines(RecipeIngredientsEvent::Move {
        id: line_id(23),
        onto: line_id(21),
    })
    .await?;
    let save = f.send(RecipeEditorEvent::Save);
    f.save(save, Ok(recipe)).await?;
    assert_eq!(f.written_lines, [vec![basil()?, penne()?, olive_oil()?]]);
    Ok(())
}

// Validation

#[test_case(None; "a line left empty")]
#[test_case(Some("   "); "a name of spaces")]
#[tokio::test]
async fn save_with_an_unnamed_line_shows_name_required_on_that_line_only(
    typed: Option<&str>,
) -> anyhow::Result<()> {
    let mut f = Fixture::edit_recipe(&pasta_with_lines()?);
    let id = f.add_line().await?;
    if let Some(name) = typed {
        f.name_line(id, name).await?;
    }
    // Before the first Save, the empty line shows no problem.
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
    let id = f.add_line().await?;
    let save = f.send(RecipeEditorEvent::Save);
    f.render(save).await?;

    f.name_line(id, "basil").await?;
    assert_eq!(problems(&f), [None]);
    f.name_line(id, " ").await?;
    assert_eq!(
        problems(&f),
        [Some(RecipeIngredientNameError::NotEmptyViolated)]
    );
    Ok(())
}

#[tokio::test]
async fn a_line_change_makes_cancel_ask_and_changing_it_back_does_not() -> anyhow::Result<()> {
    let mut f = Fixture::edit_recipe(&pasta_with_lines()?);
    let penne = penne()?.id;
    f.name_line(penne, "penne rigate").await?;
    f.request_close().await?;
    let decided = f.decide(ConfirmationEvent::Cancel).await?;
    let command = f.editor.update(Message::Outcome(decided));
    f.render(command).await?;

    f.name_line(penne, "penne pasta").await?;
    let command = f.send(RecipeEditorEvent::RequestClose);
    assert_eq!(
        f.render_then_outcome(command).await?,
        RecipeEditorOutcome::Discarded
    );
    Ok(())
}

#[test_case(RecipeIngredientsEvent::Add; "an added line")]
#[test_case(RecipeIngredientsEvent::Remove { id: line_id(21) }; "a removed line")]
#[test_case(RecipeIngredientsEvent::Move { id: line_id(22), onto: line_id(21) }; "a moved line")]
#[tokio::test]
async fn an_added_removed_or_moved_line_makes_cancel_ask(
    event: RecipeIngredientsEvent,
) -> anyhow::Result<()> {
    let mut f = Fixture::edit_recipe(&pasta_with_lines()?);
    f.lines(event).await?;
    f.request_close().await
}

// Red cases

#[test_case(RecipeIngredientsEvent::SetName { id: unknown_line(), name: "basil".into() }; "naming an unknown line")]
#[test_case(RecipeIngredientsEvent::SetName { id: line_id(21), name: "penne pasta".into() }; "the same name again")]
#[test_case(RecipeIngredientsEvent::Remove { id: unknown_line() }; "removing an unknown line")]
#[test_case(RecipeIngredientsEvent::Move { id: unknown_line(), onto: line_id(21) }; "moving an unknown line")]
#[test_case(RecipeIngredientsEvent::Move { id: line_id(21), onto: unknown_line() }; "moving onto an unknown line")]
#[test_case(RecipeIngredientsEvent::Move { id: line_id(21), onto: line_id(21) }; "moving a line onto itself")]
#[tokio::test]
async fn a_line_event_that_changes_nothing_does_not_render(
    event: RecipeIngredientsEvent,
) -> anyhow::Result<()> {
    let mut f = Fixture::edit_recipe(&pasta_with_lines()?);
    let before = f.editor.view();
    let command = f.send(RecipeEditorEvent::Lines(event));
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
fn three_lines() -> anyhow::Result<Recipe> {
    Ok(Recipe {
        lines: vec![penne()?, olive_oil()?, basil()?],
        ..pasta()
    })
}

fn basil() -> anyhow::Result<RecipeIngredient> {
    saved_line(23, "basil")
}

fn line_id(id: u128) -> RecipeIngredientId {
    RecipeIngredientId::from(Uuid::from_u128(id))
}

fn unknown_line() -> RecipeIngredientId {
    line_id(99)
}

fn line(id: RecipeIngredientId, name: &str) -> anyhow::Result<RecipeIngredient> {
    Ok(RecipeIngredient {
        id,
        ..saved_line(0, name)?
    })
}

/// A saved line as the editor shows it before the first Save.
fn shown(line: &RecipeIngredient) -> RecipeIngredientView {
    RecipeIngredientView {
        id: line.id,
        name: line.name.as_ref().to_owned(),
        problem: None,
    }
}

fn problems(f: &Fixture) -> Vec<Option<RecipeIngredientNameError>> {
    f.editor
        .view()
        .lines
        .into_iter()
        .map(|line| line.problem)
        .collect()
}
