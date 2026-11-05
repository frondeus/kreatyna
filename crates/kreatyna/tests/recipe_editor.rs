//! The recipe editor (REC-02, REC-04, REC-14): the title, Save, and
//! "Discard changes?". The step canvas has its own file, `step_canvas.rs`.
use kreatyna::{
    confirmation::ConfirmationEvent,
    recipes::{
        DurationInput, IngredientLinesEvent, Recipe, RecipeEditorEvent, RecipeEditorOutcome,
        RecipeEditorProblem, RecipeEditorStatus, RecipeEditorView, StepCanvasEvent, StepCanvasView,
        StepEdge, StepKind, StepNodesView, capabilities::RecipeSaveError,
    },
};
use krucyfiks::{Message, Widget, effects::CommandTestExt, test_utils::EmptyInitTest};
use test_case::test_case;

// Green path

#[tokio::test]
async fn a_new_editor_has_an_empty_title_and_starts_nothing() {
    let mut f = Fixture::new();
    assert_eq!(f.editor.view(), editing("", None));
    f.editor.expect_noop_init().await;
}

#[tokio::test]
async fn typing_keeps_the_raw_title_and_renders() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    let command = f.send(RecipeEditorEvent::SetTitle("  Shakshuka ".into()));
    assert_eq!(f.editor.view(), editing("  Shakshuka ", None));
    f.render(command).await
}

#[tokio::test]
async fn save_creates_the_trimmed_title_then_reports_saved() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    f.type_title("  Shakshuka ").await?;

    let save = f.send(RecipeEditorEvent::Save);
    assert_eq!(f.editor.view().status, RecipeEditorStatus::Saving);
    let finished = f.save(save, Ok(shakshuka())).await?;
    assert_eq!(f.writes, [Write::Created("Shakshuka".into())]);

    let command = f.editor.update(Message::Outcome(finished));
    assert_eq!(f.editor.view().status, RecipeEditorStatus::Finished);
    assert_eq!(
        f.render_then_outcome(command).await?,
        RecipeEditorOutcome::Saved {
            recipe: shakshuka()
        }
    );
    Ok(())
}

#[tokio::test]
async fn cancel_on_an_untouched_editor_discards_at_once() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    let command = f.send(RecipeEditorEvent::RequestClose);
    assert_eq!(f.editor.view().status, RecipeEditorStatus::Finished);
    assert_eq!(
        f.render_then_outcome(command).await?,
        RecipeEditorOutcome::Discarded
    );
    Ok(())
}

#[tokio::test]
async fn cancel_after_typing_asks_and_confirming_discards() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    f.type_title("Shakshuka").await?;
    f.request_close().await?;

    let decided = f.decide(ConfirmationEvent::Confirm).await?;
    let command = f.editor.update(Message::Outcome(decided));
    assert_eq!(f.editor.view().status, RecipeEditorStatus::Finished);
    assert_eq!(
        f.render_then_outcome(command).await?,
        RecipeEditorOutcome::Discarded
    );
    Ok(())
}

#[tokio::test]
async fn an_edit_editor_opens_with_the_saved_title_and_starts_nothing() {
    let mut f = Fixture::edit();
    assert_eq!(f.editor.view(), editing("Shakshuka", None));
    f.editor.expect_noop_init().await;
}

#[test_case(Some(" Spicy shakshuka "), "Spicy shakshuka"; "a new title")]
#[test_case(None, "Shakshuka"; "the saved title")]
#[tokio::test]
async fn save_in_an_edit_editor_updates_the_recipe_by_id_then_reports_saved(
    typed: Option<&str>,
    title: &str,
) -> anyhow::Result<()> {
    let mut f = Fixture::edit();
    if let Some(text) = typed {
        f.type_title(text).await?;
    }
    let saved = Recipe {
        title: title.into(),
        ..shakshuka()
    };

    let save = f.send(RecipeEditorEvent::Save);
    let finished = f.save(save, Ok(saved.clone())).await?;
    assert_eq!(f.writes, [Write::Updated(shakshuka().id, title.into())]);

    let command = f.editor.update(Message::Outcome(finished));
    assert_eq!(
        f.render_then_outcome(command).await?,
        RecipeEditorOutcome::Saved { recipe: saved }
    );
    Ok(())
}

#[tokio::test]
async fn cancel_on_an_unchanged_edit_editor_discards_at_once() -> anyhow::Result<()> {
    let mut f = Fixture::edit();
    let command = f.send(RecipeEditorEvent::RequestClose);
    assert_eq!(
        f.render_then_outcome(command).await?,
        RecipeEditorOutcome::Discarded
    );
    Ok(())
}

#[tokio::test]
async fn cancel_after_changing_the_saved_title_asks() -> anyhow::Result<()> {
    let mut f = Fixture::edit();
    f.type_title("Spicy shakshuka").await?;
    f.request_close().await
}

// Validation

#[tokio::test]
async fn typing_the_saved_title_again_makes_the_edit_editor_unchanged() -> anyhow::Result<()> {
    let mut f = Fixture::edit();
    f.type_title("Pasta").await?;
    f.type_title("Shakshuka").await?;
    let command = f.send(RecipeEditorEvent::RequestClose);
    assert_eq!(
        f.render_then_outcome(command).await?,
        RecipeEditorOutcome::Discarded
    );
    Ok(())
}

#[test_case(&[], ""; "nothing typed")]
#[test_case(&["   "], "   "; "spaces only")]
#[tokio::test]
async fn save_with_a_blank_title_shows_the_issue_and_does_not_call_the_store(
    typed: &[&str],
    title: &str,
) -> anyhow::Result<()> {
    let mut f = Fixture::new();
    for text in typed {
        f.type_title(text).await?;
    }
    let command = f.send(RecipeEditorEvent::Save);
    assert_eq!(f.editor.view(), editing(title, Some(empty_title())));
    // One render and nothing else: no store call.
    f.render(command).await
}

#[tokio::test]
async fn before_the_first_save_a_blank_title_shows_no_issue() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    f.type_title("S").await?;
    f.type_title("").await?;
    assert_eq!(f.editor.view(), editing("", None));
    Ok(())
}

#[tokio::test]
async fn after_the_first_save_every_change_checks_the_title_again() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    let command = f.send(RecipeEditorEvent::Save);
    f.render(command).await?;

    f.type_title("S").await?;
    assert_eq!(f.editor.view(), editing("S", None));
    f.type_title(" ").await?;
    assert_eq!(f.editor.view(), editing(" ", Some(empty_title())));
    Ok(())
}

#[tokio::test]
async fn typing_the_same_title_again_does_nothing() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    f.type_title("Shakshuka").await?;
    let command = f.send(RecipeEditorEvent::SetTitle("Shakshuka".into()));
    command.driver(&mut f.effects).step().await?.expect_done();
    Ok(())
}

#[tokio::test]
async fn keep_editing_returns_to_the_editor_with_its_problem() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    f.type_title(" ").await?;
    let command = f.send(RecipeEditorEvent::Save);
    f.render(command).await?;
    f.request_close().await?;

    let decided = f.decide(ConfirmationEvent::Cancel).await?;
    let command = f.editor.update(Message::Outcome(decided));
    assert_eq!(f.editor.view(), editing(" ", Some(empty_title())));
    f.render(command).await
}

#[tokio::test]
async fn edits_steps_lines_save_and_cancel_are_ignored_while_saving() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    f.type_title("Shakshuka").await?;
    let onion = f.add_step(point(40, 40)).await?;
    f.name_step(onion, "Dice onion").await?;
    let _save = f.send(RecipeEditorEvent::Save);
    for event in [
        RecipeEditorEvent::SetTitle("Pasta".into()),
        RecipeEditorEvent::Canvas(StepCanvasEvent::Add { at: point(240, 40) }),
        RecipeEditorEvent::Canvas(StepCanvasEvent::Move {
            id: onion,
            to: point(320, 200),
        }),
        RecipeEditorEvent::Canvas(StepCanvasEvent::Delete(onion)),
        RecipeEditorEvent::Canvas(StepCanvasEvent::SetDuration {
            id: onion,
            input: DurationInput::Minutes(5),
        }),
        RecipeEditorEvent::Canvas(StepCanvasEvent::SetKind {
            id: onion,
            kind: StepKind::Waiting,
        }),
        RecipeEditorEvent::Canvas(StepCanvasEvent::SetInstructions {
            id: onion,
            text: "Fine.".into(),
        }),
        RecipeEditorEvent::Canvas(StepCanvasEvent::Connect {
            from: onion,
            to: onion,
        }),
        RecipeEditorEvent::Canvas(StepCanvasEvent::SelectConnection(StepEdge {
            from: onion,
            to: onion,
        })),
        RecipeEditorEvent::Canvas(StepCanvasEvent::DeleteConnection(StepEdge {
            from: onion,
            to: onion,
        })),
        RecipeEditorEvent::Lines(IngredientLinesEvent::Add),
        RecipeEditorEvent::Save,
        RecipeEditorEvent::RequestClose,
    ] {
        let command = f.send(event);
        command.driver(&mut f.effects).step().await?.expect_done();
    }
    assert_eq!(
        f.editor.view(),
        RecipeEditorView {
            title: "Shakshuka".into(),
            lines: vec![],
            canvas: canvas(StepNodesView::Selected {
                selected: node(
                    onion,
                    "Dice onion",
                    titled("Dice onion"),
                    point(40, 40),
                    None
                ),
                others: vec![],
            }),
            status: RecipeEditorStatus::Saving,
        }
    );
    Ok(())
}

#[tokio::test]
async fn a_result_of_an_earlier_save_is_ignored() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    f.type_title("Shakshuka").await?;
    let first = f.send(RecipeEditorEvent::Save);
    let failed = f.save(first, Err(RecipeSaveError::Storage)).await?;
    let command = f.editor.update(Message::Outcome(failed.clone()));
    f.render(command).await?;

    let _second = f.send(RecipeEditorEvent::Save);
    let command = f.editor.update(Message::Outcome(failed));
    command.driver(&mut f.effects).step().await?.expect_done();
    assert_eq!(f.editor.view().status, RecipeEditorStatus::Saving);
    Ok(())
}

#[test_case(None; "a new editor")]
#[test_case(Some(pasta()); "an edit editor")]
#[tokio::test]
async fn cancel_after_adding_a_step_asks(recipe: Option<Recipe>) -> anyhow::Result<()> {
    let mut f = match &recipe {
        None => Fixture::new(),
        Some(recipe) => Fixture::edit_recipe(recipe),
    };
    f.add_step(point(40, 40)).await?;
    f.request_close().await
}

#[tokio::test]
async fn deleting_an_added_step_keeps_the_editor_unchanged() -> anyhow::Result<()> {
    let mut f = Fixture::edit_recipe(&pasta());
    let drain = f.add_step(point(440, 40)).await?;
    f.delete_step(drain).await?;
    let command = f.send(RecipeEditorEvent::RequestClose);
    assert_eq!(
        f.render_then_outcome(command).await?,
        RecipeEditorOutcome::Discarded
    );
    Ok(())
}

#[test_case(StepCanvasEvent::SetTitle { id: cook_pasta().id, title: "Cook pasta al dente".into() }; "rename it")]
#[test_case(StepCanvasEvent::Move { id: cook_pasta().id, to: point(320, 200) }; "move it")]
#[test_case(StepCanvasEvent::Delete(cook_pasta().id); "delete it")]
#[test_case(StepCanvasEvent::SetDuration { id: cook_pasta().id, input: DurationInput::Minutes(10) }; "give it a duration")]
#[test_case(StepCanvasEvent::SetKind { id: cook_pasta().id, kind: StepKind::Waiting }; "make it waiting")]
#[test_case(StepCanvasEvent::SetInstructions { id: cook_pasta().id, text: "Stir.".into() }; "give it instructions")]
#[test_case(StepCanvasEvent::Connect { from: boil_water().id, to: cook_pasta().id }; "connect it")]
#[tokio::test]
async fn cancel_after_changing_a_saved_step_asks(change: StepCanvasEvent) -> anyhow::Result<()> {
    let mut f = Fixture::edit_recipe(&pasta());
    f.canvas(change).await?;
    f.request_close().await
}

#[test_case(
    vec![
        StepCanvasEvent::Select(cook_pasta().id),
        StepCanvasEvent::SetTitle { id: cook_pasta().id, title: "Cook".into() },
        StepCanvasEvent::SetTitle { id: cook_pasta().id, title: "Cook pasta".into() },
    ];
    "select it and type its saved title again"
)]
#[test_case(
    vec![
        StepCanvasEvent::Move { id: cook_pasta().id, to: point(320, 200) },
        StepCanvasEvent::Move { id: cook_pasta().id, to: cook_pasta().position },
    ];
    "move it back to its saved point"
)]
#[test_case(
    vec![
        StepCanvasEvent::SetDuration { id: bake_sauce().id, input: DurationInput::Parts(parts(400, 0, 0, 0)) },
        StepCanvasEvent::SetDuration { id: bake_sauce().id, input: DurationInput::Minutes(20) },
    ];
    "set it past 365 days, then its saved 20 minutes"
)]
#[test_case(
    vec![
        StepCanvasEvent::SetDuration { id: bake_sauce().id, input: DurationInput::Minutes(5) },
        StepCanvasEvent::SetDuration { id: bake_sauce().id, input: DurationInput::Parts(parts(0, 0, 0, 1_200)) },
    ];
    "set 5 minutes, then its saved 20 minutes as seconds"
)]
#[test_case(
    vec![
        StepCanvasEvent::SetKind { id: bake_sauce().id, kind: StepKind::Active },
        StepCanvasEvent::SetKind { id: bake_sauce().id, kind: StepKind::Waiting },
    ];
    "switch it to active and back"
)]
#[test_case(
    vec![StepCanvasEvent::SetInstructions { id: bake_sauce().id, text: " Cover the dish. ".into() }];
    "type its saved instructions with spaces around"
)]
#[test_case(
    vec![
        StepCanvasEvent::Connect { from: boil_water().id, to: cook_pasta().id },
        StepCanvasEvent::DeleteConnection(boil_then_cook()),
    ];
    "connect two steps, then delete the arrow"
)]
#[tokio::test]
async fn changing_a_step_back_to_its_saved_state_keeps_the_editor_unchanged(
    changes: Vec<StepCanvasEvent>,
) -> anyhow::Result<()> {
    let mut f = Fixture::edit_recipe(&Recipe {
        steps: vec![boil_water(), cook_pasta(), bake_sauce()],
        ..pasta()
    });
    for change in changes {
        f.canvas(change).await?;
    }
    let command = f.send(RecipeEditorEvent::RequestClose);
    assert_eq!(
        f.render_then_outcome(command).await?,
        RecipeEditorOutcome::Discarded
    );
    Ok(())
}

#[test_case(StepCanvasEvent::Add { at: point(440, 40) }; "add a step")]
#[test_case(StepCanvasEvent::Move { id: cook_pasta().id, to: point(320, 200) }; "move a step")]
#[test_case(StepCanvasEvent::Delete(cook_pasta().id); "delete a step")]
#[test_case(StepCanvasEvent::SetDuration { id: cook_pasta().id, input: DurationInput::Minutes(10) }; "time a step")]
#[test_case(StepCanvasEvent::Connect { from: boil_water().id, to: cook_pasta().id }; "connect two steps")]
#[tokio::test]
async fn canvas_events_are_ignored_while_discard_is_asked(
    event: StepCanvasEvent,
) -> anyhow::Result<()> {
    let mut f = Fixture::edit_recipe(&pasta());
    f.type_title("Spicy pasta").await?;
    f.request_close().await?;
    let command = f.send(RecipeEditorEvent::Canvas(event));
    command.driver(&mut f.effects).step().await?.expect_done();
    assert_eq!(f.editor.view().canvas, saved_canvas(&pasta()));
    Ok(())
}

// Red cases

#[test_case(RecipeSaveError::DuplicateTitle; "duplicate title")]
#[test_case(RecipeSaveError::Storage; "storage")]
#[test_case(RecipeSaveError::Transport; "transport")]
#[tokio::test]
async fn a_failed_save_shows_its_error_until_the_title_changes(
    error: RecipeSaveError,
) -> anyhow::Result<()> {
    let mut f = Fixture::new();
    f.type_title("shakshuka").await?;
    let save = f.send(RecipeEditorEvent::Save);
    let failed = f.save(save, Err(error.clone())).await?;

    let command = f.editor.update(Message::Outcome(failed));
    assert_eq!(
        f.editor.view(),
        editing("shakshuka", Some(RecipeEditorProblem::SaveFailed(error)))
    );
    f.render(command).await?;

    f.type_title("Shakshuka (spicy)").await?;
    assert_eq!(f.editor.view(), editing("Shakshuka (spicy)", None));
    Ok(())
}

#[test_case(RecipeSaveError::NotFound; "recipe no longer exists")]
#[test_case(RecipeSaveError::DuplicateTitle; "duplicate title")]
#[tokio::test]
async fn a_failed_update_keeps_the_edit_editor_open_with_its_error(
    error: RecipeSaveError,
) -> anyhow::Result<()> {
    let mut f = Fixture::edit();
    f.type_title("Pasta").await?;
    let save = f.send(RecipeEditorEvent::Save);
    let failed = f.save(save, Err(error.clone())).await?;

    let command = f.editor.update(Message::Outcome(failed));
    assert_eq!(
        f.editor.view(),
        editing("Pasta", Some(RecipeEditorProblem::SaveFailed(error)))
    );
    f.render(command).await
}

// Test support

mod recipe_editor_support;

use recipe_editor_support::*;

fn no_steps() -> StepCanvasView {
    canvas(StepNodesView::Unselected { steps: vec![] })
}

fn editing(title: &str, problem: Option<RecipeEditorProblem>) -> RecipeEditorView {
    RecipeEditorView {
        title: title.into(),
        lines: vec![],
        canvas: no_steps(),
        status: RecipeEditorStatus::Editing { problem },
    }
}
