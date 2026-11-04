use kreatyna::{
    capabilities::ResolveRender,
    confirmation::{ConfirmationEvent, ConfirmationView},
    recipes::{
        CanvasPoint, Recipe, RecipeEditor, RecipeEditorEvent, RecipeEditorOutcome,
        RecipeEditorProblem, RecipeEditorStatus, RecipeEditorView, RecipeId, RecipeTitleError,
        Step, StepCanvasEvent, StepCanvasView, StepId, StepLabel, StepNodeView, StepTitle,
        StepTitleError, StepWrite,
        capabilities::{
            RecipeEffectStream, RecipeSaveError, RecipeStoreRequest, ResolveRecipeStore,
        },
    },
};
use krucyfiks::{Command, Message, Widget, effects::CommandTestExt, test_utils::EmptyInitTest};
use test_case::test_case;
use uuid::Uuid;

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

#[tokio::test]
async fn add_step_adds_an_untitled_step_at_the_point_and_selects_it() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    let command = f.send(RecipeEditorEvent::Canvas(StepCanvasEvent::Add {
        at: point(120, 80),
    }));
    let id = f.selected()?;
    assert_eq!(
        f.editor.view().canvas,
        StepCanvasView::Selected {
            selected: node(id, "", StepLabel::Untitled, point(120, 80), None),
            others: vec![],
        }
    );
    f.render(command).await
}

#[test_case(" Dice onion ", titled("Dice onion") ; "a title shows trimmed")]
#[test_case("   ", StepLabel::Untitled ; "a title of spaces shows untitled")]
#[tokio::test]
async fn naming_a_step_keeps_the_raw_title_and_renders(
    title: &str,
    label: StepLabel,
) -> anyhow::Result<()> {
    let mut f = Fixture::new();
    let id = f.add_step(point(120, 80)).await?;
    let command = f.send(RecipeEditorEvent::Canvas(StepCanvasEvent::SetTitle {
        id,
        title: title.into(),
    }));
    assert_eq!(
        f.editor.view().canvas,
        StepCanvasView::Selected {
            selected: node(id, title, label, point(120, 80), None),
            others: vec![],
        }
    );
    f.render(command).await
}

#[tokio::test]
async fn a_click_on_a_node_selects_it_and_a_click_on_empty_canvas_clears_it() -> anyhow::Result<()>
{
    let mut f = Fixture::edit_recipe(&pasta());
    f.canvas(StepCanvasEvent::Select(cook_pasta().id)).await?;
    assert_eq!(
        f.editor.view().canvas,
        StepCanvasView::Selected {
            selected: saved_node(&cook_pasta()),
            others: vec![saved_node(&boil_water())],
        }
    );

    f.canvas(StepCanvasEvent::ClearSelection).await?;
    assert_eq!(f.editor.view().canvas, saved_canvas(&pasta()));
    Ok(())
}

#[tokio::test]
async fn save_writes_the_steps_with_trimmed_titles_in_the_order_added() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    f.type_title("Shakshuka").await?;
    let onion = f.add_step(point(40, 40)).await?;
    f.name_step(onion, " Dice onion ").await?;
    let eggs = f.add_step(point(240, 40)).await?;
    f.name_step(eggs, "Crack eggs").await?;

    let save = f.send(RecipeEditorEvent::Save);
    assert_eq!(f.editor.view().status, RecipeEditorStatus::Saving);
    f.save(save, Ok(shakshuka())).await?;
    assert_eq!(f.writes, [Write::Created("Shakshuka".into())]);
    assert_eq!(
        f.written_steps,
        [vec![
            write(onion, "Dice onion", point(40, 40))?,
            write(eggs, "Crack eggs", point(240, 40))?,
        ]]
    );
    Ok(())
}

#[tokio::test]
async fn an_edit_editor_opens_with_the_saved_steps_and_no_selection() {
    let f = Fixture::edit_recipe(&pasta());
    assert_eq!(f.editor.view().canvas, saved_canvas(&pasta()));
}

#[tokio::test]
async fn save_in_an_edit_editor_writes_the_changed_and_the_added_steps() -> anyhow::Result<()> {
    let mut f = Fixture::edit_recipe(&pasta());
    f.name_step(boil_water().id, "Boil salted water").await?;
    let drain = f.add_step(point(440, 40)).await?;
    f.name_step(drain, "Drain").await?;

    let save = f.send(RecipeEditorEvent::Save);
    f.save(save, Ok(pasta())).await?;
    assert_eq!(f.writes, [Write::Updated(pasta().id, "Pasta".into())]);
    assert_eq!(
        f.written_steps,
        [vec![
            write(boil_water().id, "Boil salted water", point(40, 40))?,
            write(cook_pasta().id, "Cook pasta", point(240, 40))?,
            write(drain, "Drain", point(440, 40))?,
        ]]
    );
    Ok(())
}

#[test_case(None; "nothing selected")]
#[test_case(Some(cook_pasta().id); "another step selected")]
#[test_case(Some(boil_water().id); "the same step selected")]
#[tokio::test]
async fn a_move_puts_the_step_at_the_point_and_selects_it(
    selected: Option<StepId>,
) -> anyhow::Result<()> {
    let mut f = Fixture::edit_recipe(&pasta());
    if let Some(id) = selected {
        f.canvas(StepCanvasEvent::Select(id)).await?;
    }
    let command = f.send(RecipeEditorEvent::Canvas(StepCanvasEvent::Move {
        id: boil_water().id,
        to: point(320, 200),
    }));
    assert_eq!(
        f.editor.view().canvas,
        StepCanvasView::Selected {
            selected: node(
                boil_water().id,
                "Boil water",
                titled("Boil water"),
                point(320, 200),
                None
            ),
            others: vec![saved_node(&cook_pasta())],
        }
    );
    f.render(command).await
}

#[tokio::test]
async fn a_move_to_the_same_point_selects_an_unselected_step() -> anyhow::Result<()> {
    let mut f = Fixture::edit_recipe(&pasta());
    let command = f.send(RecipeEditorEvent::Canvas(StepCanvasEvent::Move {
        id: boil_water().id,
        to: boil_water().position,
    }));
    assert_eq!(
        f.editor.view().canvas,
        StepCanvasView::Selected {
            selected: saved_node(&boil_water()),
            others: vec![saved_node(&cook_pasta())],
        }
    );
    f.render(command).await
}

#[tokio::test]
async fn save_writes_the_moved_position() -> anyhow::Result<()> {
    let mut f = Fixture::edit_recipe(&pasta());
    f.move_step(cook_pasta().id, point(320, 200)).await?;

    let save = f.send(RecipeEditorEvent::Save);
    f.save(save, Ok(pasta())).await?;
    assert_eq!(
        f.written_steps,
        [vec![
            write(boil_water().id, "Boil water", point(40, 40))?,
            write(cook_pasta().id, "Cook pasta", point(320, 200))?,
        ]]
    );
    Ok(())
}

#[tokio::test]
async fn deleting_the_selected_step_removes_it_and_clears_the_selection() -> anyhow::Result<()> {
    let mut f = Fixture::edit_recipe(&pasta());
    f.canvas(StepCanvasEvent::Select(boil_water().id)).await?;
    let command = f.send(RecipeEditorEvent::Canvas(StepCanvasEvent::Delete(
        boil_water().id,
    )));
    assert_eq!(
        f.editor.view().canvas,
        StepCanvasView::Unselected {
            steps: vec![saved_node(&cook_pasta())],
        }
    );
    f.render(command).await
}

#[tokio::test]
async fn deleting_an_unselected_step_keeps_the_selection() -> anyhow::Result<()> {
    let mut f = Fixture::edit_recipe(&pasta());
    f.canvas(StepCanvasEvent::Select(cook_pasta().id)).await?;
    f.delete_step(boil_water().id).await?;
    assert_eq!(
        f.editor.view().canvas,
        StepCanvasView::Selected {
            selected: saved_node(&cook_pasta()),
            others: vec![],
        }
    );
    Ok(())
}

#[tokio::test]
async fn save_writes_the_steps_without_the_deleted_one() -> anyhow::Result<()> {
    let mut f = Fixture::edit_recipe(&pasta());
    f.delete_step(boil_water().id).await?;

    let save = f.send(RecipeEditorEvent::Save);
    f.save(save, Ok(pasta())).await?;
    assert_eq!(
        f.written_steps,
        [vec![write(cook_pasta().id, "Cook pasta", point(240, 40))?]]
    );
    Ok(())
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
async fn deleting_the_untitled_step_lets_save_pass() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    f.type_title("Shakshuka").await?;
    let onion = f.add_step(point(40, 40)).await?;
    f.name_step(onion, "Dice onion").await?;
    let untitled = f.add_step(point(240, 40)).await?;
    let refused = f.send(RecipeEditorEvent::Save);
    f.render(refused).await?;

    f.delete_step(untitled).await?;
    let save = f.send(RecipeEditorEvent::Save);
    f.save(save, Ok(shakshuka())).await?;
    assert_eq!(
        f.written_steps,
        [vec![write(onion, "Dice onion", point(40, 40))?]]
    );
    Ok(())
}

#[tokio::test]
async fn edits_steps_save_and_cancel_are_ignored_while_saving() -> anyhow::Result<()> {
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
            canvas: StepCanvasView::Selected {
                selected: node(
                    onion,
                    "Dice onion",
                    titled("Dice onion"),
                    point(40, 40),
                    None
                ),
                others: vec![],
            },
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

#[test_case("Pasta", None; "a valid recipe title")]
#[test_case("", Some(empty_title()); "a blank recipe title")]
#[tokio::test]
async fn save_with_an_untitled_step_selects_the_first_one_and_does_not_call_the_store(
    title: &str,
    problem: Option<RecipeEditorProblem>,
) -> anyhow::Result<()> {
    let mut f = Fixture::new();
    if !title.is_empty() {
        f.type_title(title).await?;
    }
    let boil = f.add_step(point(40, 40)).await?;
    f.name_step(boil, "Boil water").await?;
    let first = f.add_step(point(240, 40)).await?;
    let second = f.add_step(point(440, 40)).await?;
    f.canvas(StepCanvasEvent::Select(boil)).await?;

    let command = f.send(RecipeEditorEvent::Save);
    let untitled = Some(StepTitleError::NotEmptyViolated);
    assert_eq!(
        f.editor.view(),
        RecipeEditorView {
            title: title.into(),
            canvas: StepCanvasView::Selected {
                selected: node(
                    first,
                    "",
                    StepLabel::Untitled,
                    point(240, 40),
                    untitled.clone()
                ),
                others: vec![
                    node(
                        boil,
                        "Boil water",
                        titled("Boil water"),
                        point(40, 40),
                        None
                    ),
                    node(second, "", StepLabel::Untitled, point(440, 40), untitled),
                ],
            },
            status: RecipeEditorStatus::Editing { problem },
        }
    );
    // One render and nothing else: no store call.
    f.render(command).await
}

#[tokio::test]
async fn after_the_first_save_naming_a_step_checks_its_title_again() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    f.type_title("Shakshuka").await?;
    let id = f.add_step(point(40, 40)).await?;
    let command = f.send(RecipeEditorEvent::Save);
    f.render(command).await?;

    f.name_step(id, "Dice onion").await?;
    assert_eq!(
        f.editor.view().canvas,
        StepCanvasView::Selected {
            selected: node(id, "Dice onion", titled("Dice onion"), point(40, 40), None),
            others: vec![],
        }
    );
    f.name_step(id, "  ").await?;
    assert_eq!(
        f.editor.view().canvas,
        StepCanvasView::Selected {
            selected: node(
                id,
                "  ",
                StepLabel::Untitled,
                point(40, 40),
                Some(StepTitleError::NotEmptyViolated)
            ),
            others: vec![],
        }
    );
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
#[tokio::test]
async fn changing_a_step_back_to_its_saved_state_keeps_the_editor_unchanged(
    changes: Vec<StepCanvasEvent>,
) -> anyhow::Result<()> {
    let mut f = Fixture::edit_recipe(&pasta());
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

#[test_case(StepCanvasEvent::Select(unknown_step()); "select an unknown step")]
#[test_case(StepCanvasEvent::SetTitle { id: unknown_step(), title: "Drain".into() }; "name an unknown step")]
#[test_case(StepCanvasEvent::Select(cook_pasta().id); "select the selected step")]
#[test_case(StepCanvasEvent::SetTitle { id: cook_pasta().id, title: "Cook pasta".into() }; "type the same title")]
#[test_case(StepCanvasEvent::Move { id: unknown_step(), to: point(320, 200) }; "move an unknown step")]
#[test_case(StepCanvasEvent::Move { id: cook_pasta().id, to: cook_pasta().position }; "drop the selected step where it is")]
#[test_case(StepCanvasEvent::Delete(unknown_step()); "delete an unknown step")]
#[tokio::test]
async fn a_canvas_event_that_changes_nothing_does_nothing(
    event: StepCanvasEvent,
) -> anyhow::Result<()> {
    let mut f = Fixture::edit_recipe(&pasta());
    f.canvas(StepCanvasEvent::Select(cook_pasta().id)).await?;
    let before = f.editor.view();
    let command = f.send(RecipeEditorEvent::Canvas(event));
    command.driver(&mut f.effects).step().await?.expect_done();
    assert_eq!(f.editor.view(), before);
    Ok(())
}

#[tokio::test]
async fn clearing_an_empty_selection_does_nothing() -> anyhow::Result<()> {
    let mut f = Fixture::edit_recipe(&pasta());
    let command = f.send(RecipeEditorEvent::Canvas(StepCanvasEvent::ClearSelection));
    command.driver(&mut f.effects).step().await?.expect_done();
    assert_eq!(f.editor.view().canvas, saved_canvas(&pasta()));
    Ok(())
}

#[test_case(StepCanvasEvent::Add { at: point(440, 40) }; "add a step")]
#[test_case(StepCanvasEvent::Move { id: cook_pasta().id, to: point(320, 200) }; "move a step")]
#[test_case(StepCanvasEvent::Delete(cook_pasta().id); "delete a step")]
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

fn shakshuka() -> Recipe {
    Recipe {
        id: RecipeId::from(Uuid::from_u128(1)),
        title: "Shakshuka".into(),
        steps: vec![],
    }
}

/// A saved recipe with two steps.
fn pasta() -> Recipe {
    Recipe {
        id: RecipeId::from(Uuid::from_u128(2)),
        title: "Pasta".into(),
        steps: vec![boil_water(), cook_pasta()],
    }
}

fn boil_water() -> Step {
    Step {
        id: StepId::from(Uuid::from_u128(11)),
        title: "Boil water".into(),
        position: point(40, 40),
    }
}

fn cook_pasta() -> Step {
    Step {
        id: StepId::from(Uuid::from_u128(12)),
        title: "Cook pasta".into(),
        position: point(240, 40),
    }
}

/// An id that names no step.
fn unknown_step() -> StepId {
    StepId::from(Uuid::from_u128(99))
}

fn point(x: u32, y: u32) -> CanvasPoint {
    CanvasPoint { x, y }
}

fn node(
    id: StepId,
    title: &str,
    label: StepLabel,
    position: CanvasPoint,
    problem: Option<StepTitleError>,
) -> StepNodeView {
    StepNodeView {
        id,
        title: title.into(),
        label,
        position,
        problem,
    }
}

fn titled(title: &str) -> StepLabel {
    StepLabel::Titled {
        title: title.into(),
    }
}

/// A saved step's node: saved titles always pass the rule.
fn saved_node(step: &Step) -> StepNodeView {
    node(
        step.id,
        &step.title,
        titled(&step.title),
        step.position,
        None,
    )
}

/// The canvas of `recipe` as the edit editor opens it: no selection.
fn saved_canvas(recipe: &Recipe) -> StepCanvasView {
    StepCanvasView::Unselected {
        steps: recipe.steps.iter().map(saved_node).collect(),
    }
}

fn no_steps() -> StepCanvasView {
    StepCanvasView::Unselected { steps: vec![] }
}

fn write(id: StepId, title: &str, position: CanvasPoint) -> anyhow::Result<StepWrite> {
    Ok(StepWrite {
        id,
        title: StepTitle::try_new(title)?,
        position,
    })
}

fn empty_title() -> RecipeEditorProblem {
    RecipeEditorProblem::Invalid(RecipeTitleError::NotEmptyViolated)
}

fn editing(title: &str, problem: Option<RecipeEditorProblem>) -> RecipeEditorView {
    RecipeEditorView {
        title: title.into(),
        canvas: no_steps(),
        status: RecipeEditorStatus::Editing { problem },
    }
}

/// A store call that Save made, with the title it wrote.
#[derive(Debug, PartialEq, Eq)]
enum Write {
    Created(String),
    Updated(RecipeId, String),
}

struct Fixture {
    editor: RecipeEditor,
    effects: RecipeEffectStream,
    /// The store calls of the saves, in order.
    writes: Vec<Write>,
    /// The steps that each store call wrote, in the order of `writes`.
    written_steps: Vec<Vec<StepWrite>>,
}

impl Fixture {
    /// An editor for a new recipe.
    fn new() -> Self {
        let (editor, effects) = RecipeEditor::test_fixture();
        Self::with(editor, effects)
    }

    /// An editor for the saved `shakshuka()`.
    fn edit() -> Self {
        Self::edit_recipe(&shakshuka())
    }

    /// An editor for the saved `recipe`.
    fn edit_recipe(recipe: &Recipe) -> Self {
        let (editor, effects) = RecipeEditor::edit_test_fixture(recipe);
        Self::with(editor, effects)
    }

    fn with(editor: RecipeEditor, effects: RecipeEffectStream) -> Self {
        Self {
            editor,
            effects,
            writes: vec![],
            written_steps: vec![],
        }
    }

    fn send(&mut self, event: RecipeEditorEvent) -> Command<RecipeEditorOutcome> {
        self.editor.update(Message::Event(event))
    }

    /// Runs a command that only renders.
    async fn render(&mut self, command: Command<RecipeEditorOutcome>) -> anyhow::Result<()> {
        let mut command = command.driver(&mut self.effects);
        command
            .step()
            .await?
            .expect_effect()
            .expect_render()
            .resolve_render(|| async {})
            .await?;
        command.step().await?.expect_done();
        Ok(())
    }

    /// Runs a command that renders, then reports one outcome.
    async fn render_then_outcome(
        &mut self,
        command: Command<RecipeEditorOutcome>,
    ) -> anyhow::Result<RecipeEditorOutcome> {
        let mut command = command.driver(&mut self.effects);
        command
            .step()
            .await?
            .expect_effect()
            .expect_render()
            .resolve_render(|| async {})
            .await?;
        let outcome = command.step().await?.expect_outcome();
        command.step().await?.expect_done();
        Ok(outcome)
    }

    async fn type_title(&mut self, title: &str) -> anyhow::Result<()> {
        let command = self.send(RecipeEditorEvent::SetTitle(title.into()));
        self.render(command).await
    }

    /// Sends a canvas event that changes the canvas, and runs its render.
    async fn canvas(&mut self, event: StepCanvasEvent) -> anyhow::Result<()> {
        let command = self.send(RecipeEditorEvent::Canvas(event));
        self.render(command).await
    }

    /// Right-click ▸ Add Step at `at`. Returns the new step's id.
    async fn add_step(&mut self, at: CanvasPoint) -> anyhow::Result<StepId> {
        self.canvas(StepCanvasEvent::Add { at }).await?;
        self.selected()
    }

    async fn name_step(&mut self, id: StepId, title: &str) -> anyhow::Result<()> {
        let title = title.into();
        self.canvas(StepCanvasEvent::SetTitle { id, title }).await
    }

    /// A drop of the step `id` at `to`.
    async fn move_step(&mut self, id: StepId, to: CanvasPoint) -> anyhow::Result<()> {
        self.canvas(StepCanvasEvent::Move { id, to }).await
    }

    async fn delete_step(&mut self, id: StepId) -> anyhow::Result<()> {
        self.canvas(StepCanvasEvent::Delete(id)).await
    }

    /// The id of the selected step.
    fn selected(&self) -> anyhow::Result<StepId> {
        match self.editor.view().canvas {
            StepCanvasView::Selected { selected, .. } => Ok(selected.id),
            StepCanvasView::Unselected { .. } => anyhow::bail!("no step is selected"),
        }
    }

    /// Runs a save command: its render, then the store call (create or
    /// update) answered with `result`. Returns `SaveFinished`, not yet
    /// dispatched.
    async fn save(
        &mut self,
        save: Command<RecipeEditorOutcome>,
        result: Result<Recipe, RecipeSaveError>,
    ) -> anyhow::Result<RecipeEditorOutcome> {
        let mut command = save.driver(&mut self.effects);
        command
            .step()
            .await?
            .expect_effect()
            .expect_render()
            .resolve_render(|| async {})
            .await?;
        let mut write = None;
        let mut steps = None;
        let store = command.step().await?.expect_effect().expect_store();
        match store.request {
            RecipeStoreRequest::Create { .. } => {
                store
                    .resolve_create(|recipe| {
                        write = Some(Write::Created(recipe.title.as_ref().to_owned()));
                        steps = Some(recipe.steps);
                        async { result }
                    })
                    .await?
            }
            RecipeStoreRequest::Update { .. } => {
                store
                    .resolve_update(|id, recipe| {
                        write = Some(Write::Updated(id, recipe.title.as_ref().to_owned()));
                        steps = Some(recipe.steps);
                        async { result }
                    })
                    .await?
            }
            _ => anyhow::bail!("expected a create or an update"),
        }
        let finished = command.step().await?.expect_outcome();
        command.step().await?.expect_done();
        drop(command);
        self.writes.extend(write);
        self.written_steps.extend(steps);
        Ok(finished)
    }

    /// Cancel on a changed title or step list: the editor asks "Discard
    /// changes?".
    async fn request_close(&mut self) -> anyhow::Result<()> {
        let command = self.send(RecipeEditorEvent::RequestClose);
        assert_eq!(
            self.editor.view().status,
            RecipeEditorStatus::ConfirmingDiscard(ConfirmationView::AwaitingDecision)
        );
        self.render(command).await
    }

    /// Answers "Discard changes?". Returns `ConfirmationDecided`, not yet
    /// dispatched.
    async fn decide(&mut self, event: ConfirmationEvent) -> anyhow::Result<RecipeEditorOutcome> {
        let command = self.send(RecipeEditorEvent::Confirmation(event));
        let mut command = command.driver(&mut self.effects);
        let decided = command.step().await?.expect_outcome();
        command.step().await?.expect_done();
        Ok(decided)
    }
}
