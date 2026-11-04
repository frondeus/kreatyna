//! The step canvas of the recipe editor (REC-14 to REC-18): adding,
//! selecting, naming, moving, deleting, timing, and connecting steps, and
//! the step rules that Save checks. The tests drive it through the editor.
use kreatyna::recipes::{
    DurationInput, RecipeEditorEvent, RecipeEditorStatus, StepAfterView, StepCanvasEvent,
    StepCanvasView, StepConnectionView, StepDurationS, StepDurationView, StepEdges, StepId,
    StepInstructions, StepKind, StepLabel, StepNodeView, StepNodesView, StepTitle, StepWrite,
};
use krucyfiks::Widget;
use test_case::test_case;

// Green path

#[tokio::test]
async fn add_step_adds_an_untitled_step_at_the_point_and_selects_it() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    let command = f.send(RecipeEditorEvent::Canvas(StepCanvasEvent::Add {
        at: point(120, 80),
    }));
    let id = f.selected()?;
    assert_eq!(
        f.editor.view().canvas.nodes,
        StepNodesView::Selected {
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
        f.editor.view().canvas.nodes,
        StepNodesView::Selected {
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
        f.editor.view().canvas.nodes,
        StepNodesView::Selected {
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
async fn an_edit_editor_shows_the_saved_arrows_and_each_steps_after() {
    let f = Fixture::edit_recipe(&connected_pasta());
    assert_eq!(
        f.editor.view().canvas,
        StepCanvasView {
            nodes: StepNodesView::Unselected {
                steps: vec![saved_node(&boil_water()), cook_after(titled("Boil water"))],
            },
            connections: vec![boil_then_cook()],
            refusal: None,
        }
    );
}

#[tokio::test]
async fn after_shows_the_earlier_steps_label_as_typed() -> anyhow::Result<()> {
    let mut f = Fixture::edit_recipe(&connected_pasta());
    f.name_step(boil_water().id, "  ").await?;
    assert_eq!(
        f.editor.view().canvas.nodes,
        StepNodesView::Unselected {
            steps: vec![
                StepNodeView {
                    title: "  ".into(),
                    label: StepLabel::Untitled,
                    ..saved_node(&boil_water())
                },
                cook_after(StepLabel::Untitled),
            ],
        }
    );
    Ok(())
}

#[tokio::test]
async fn connect_adds_the_arrow_and_the_after_and_keeps_the_selection() -> anyhow::Result<()> {
    let mut f = Fixture::edit_recipe(&pasta());
    f.canvas(StepCanvasEvent::Select(boil_water().id)).await?;
    f.connect(boil_water().id, cook_pasta().id).await?;
    assert_eq!(
        f.editor.view().canvas,
        StepCanvasView {
            nodes: StepNodesView::Selected {
                selected: saved_node(&boil_water()),
                others: vec![cook_after(titled("Boil water"))],
            },
            connections: vec![boil_then_cook()],
            refusal: None,
        }
    );
    Ok(())
}

#[tokio::test]
async fn save_writes_the_connected_arrows() -> anyhow::Result<()> {
    let mut f = Fixture::edit_recipe(&pasta());
    f.connect(boil_water().id, cook_pasta().id).await?;
    let save = f.send(RecipeEditorEvent::Save);
    f.save(save, Ok(connected_pasta())).await?;
    assert_eq!(
        f.written_edges,
        [StepEdges::from_saved([boil_then_cook()])?]
    );
    Ok(())
}

#[tokio::test]
async fn a_click_on_an_arrow_selects_it_and_a_click_on_a_step_selects_the_step()
-> anyhow::Result<()> {
    let mut f = Fixture::edit_recipe(&connected_pasta());
    f.canvas(StepCanvasEvent::SelectConnection(boil_then_cook()))
        .await?;
    assert_eq!(
        f.editor.view().canvas.nodes,
        StepNodesView::ConnectionSelected {
            connection: StepConnectionView {
                edge: boil_then_cook(),
                from: titled("Boil water"),
                to: titled("Cook pasta"),
            },
            steps: vec![saved_node(&boil_water()), cook_after(titled("Boil water"))],
        }
    );
    f.canvas(StepCanvasEvent::Select(cook_pasta().id)).await?;
    assert_eq!(f.selected()?, cook_pasta().id);
    Ok(())
}

#[tokio::test]
async fn delete_connection_removes_the_arrow_and_its_selection() -> anyhow::Result<()> {
    let mut f = Fixture::edit_recipe(&connected_pasta());
    f.canvas(StepCanvasEvent::SelectConnection(boil_then_cook()))
        .await?;
    f.canvas(StepCanvasEvent::DeleteConnection(boil_then_cook()))
        .await?;
    assert_eq!(f.editor.view().canvas, saved_canvas(&pasta()));
    f.request_close().await
}

#[test_case(StepCanvasEvent::Select(boil_water().id); "a step selected")]
#[test_case(StepCanvasEvent::SelectConnection(boil_then_cook()); "its arrow selected")]
#[tokio::test]
async fn deleting_a_step_removes_its_arrows_and_the_after_they_made(
    select: StepCanvasEvent,
) -> anyhow::Result<()> {
    let mut f = Fixture::edit_recipe(&connected_pasta());
    f.canvas(select).await?;
    f.delete_step(boil_water().id).await?;
    assert_eq!(
        f.editor.view().canvas,
        canvas(StepNodesView::Unselected {
            steps: vec![saved_node(&cook_pasta())],
        })
    );
    Ok(())
}

#[tokio::test]
async fn save_in_an_edit_editor_writes_the_changed_and_the_added_steps() -> anyhow::Result<()> {
    let mut f = Fixture::edit_recipe(&connected_pasta());
    f.name_step(boil_water().id, "Boil salted water").await?;
    let drain = f.add_step(point(440, 40)).await?;
    f.name_step(drain, "Drain").await?;

    let save = f.send(RecipeEditorEvent::Save);
    f.save(save, Ok(connected_pasta())).await?;
    assert_eq!(f.writes, [Write::Updated(pasta().id, "Pasta".into())]);
    assert_eq!(
        f.written_steps,
        [vec![
            write(boil_water().id, "Boil salted water", point(40, 40))?,
            write(cook_pasta().id, "Cook pasta", point(240, 40))?,
            write(drain, "Drain", point(440, 40))?,
        ]]
    );
    // REC-18 change 2: the editor cannot change edges yet, so it writes
    // back the saved ones.
    assert_eq!(
        f.written_edges,
        [StepEdges::from_saved([boil_then_cook()])?]
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
        f.editor.view().canvas.nodes,
        StepNodesView::Selected {
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
        f.editor.view().canvas.nodes,
        StepNodesView::Selected {
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
        f.editor.view().canvas.nodes,
        StepNodesView::Unselected {
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
        f.editor.view().canvas.nodes,
        StepNodesView::Selected {
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

#[tokio::test]
async fn save_keeps_a_steps_duration_kind_and_instructions() -> anyhow::Result<()> {
    let bake = bake_sauce();
    let mut f = Fixture::edit_recipe(&sauce());
    f.name_step(bake.id, "Bake the sauce").await?;

    let save = f.send(RecipeEditorEvent::Save);
    f.save(save, Ok(sauce())).await?;
    assert_eq!(
        f.written_steps,
        [vec![StepWrite {
            id: bake.id,
            title: StepTitle::try_new("Bake the sauce")?,
            position: bake.position,
            duration: bake.duration,
            kind: StepKind::Waiting,
            instructions: bake.instructions,
        }]]
    );
    Ok(())
}

#[tokio::test]
async fn an_edit_editor_shows_a_saved_steps_duration_kind_and_instructions() -> anyhow::Result<()> {
    let mut f = Fixture::edit_recipe(&sauce());
    f.canvas(StepCanvasEvent::Select(bake_sauce().id)).await?;
    assert_eq!(
        f.editor.view().canvas.nodes,
        StepNodesView::Selected {
            selected: StepNodeView {
                duration: set("20 min", parts(0, 0, 20, 0), Some(20)),
                kind: StepKind::Waiting,
                instructions: "Cover the dish.".into(),
                ..saved_node(&bake_sauce())
            },
            others: vec![],
        }
    );
    Ok(())
}

#[test_case(DurationInput::Minutes(10), set("10 min", parts(0, 0, 10, 0), Some(10)); "a chip")]
#[test_case(DurationInput::Minutes(90), set("1 h 30 min", parts(0, 1, 30, 0), Some(90)); "the minutes field past an hour")]
#[test_case(DurationInput::Parts(parts(0, 0, 1, 30)), set("1 min 30 s", parts(0, 0, 1, 30), None); "seconds leave no whole minutes")]
#[test_case(DurationInput::Parts(parts(0, 1, 75, 0)), set("2 h 15 min", parts(0, 2, 15, 0), Some(135)); "a unit field past its unit")]
#[test_case(DurationInput::Parts(parts(2, 0, 0, 0)), set("2 d", parts(2, 0, 0, 0), Some(2_880)); "days")]
#[test_case(DurationInput::Cleared, StepDurationView::NotSet; "clearing it")]
#[test_case(DurationInput::Minutes(0), StepDurationView::NotSet; "0 minutes")]
#[test_case(DurationInput::Parts(parts(400, 0, 0, 0)), StepDurationView::TooLong { parts: parts(400, 0, 0, 0) }; "past 365 days keeps what you set")]
#[tokio::test]
async fn setting_the_duration_renders_the_step_with_it(
    input: DurationInput,
    expected: StepDurationView,
) -> anyhow::Result<()> {
    let mut f = Fixture::edit_recipe(&sauce());
    let id = bake_sauce().id;
    let command = f.send(RecipeEditorEvent::Canvas(StepCanvasEvent::SetDuration {
        id,
        input,
    }));
    // No problem before the first Save, even past 365 days.
    assert_eq!(
        f.editor.view().canvas.nodes,
        StepNodesView::Unselected {
            steps: vec![StepNodeView {
                duration: expected,
                ..saved_node(&bake_sauce())
            }],
        }
    );
    f.render(command).await
}

#[tokio::test]
async fn the_kind_and_the_raw_instructions_render() -> anyhow::Result<()> {
    let mut f = Fixture::edit_recipe(&pasta());
    let id = cook_pasta().id;
    f.canvas(StepCanvasEvent::SetKind {
        id,
        kind: StepKind::Waiting,
    })
    .await?;
    f.canvas(StepCanvasEvent::SetInstructions {
        id,
        text: " Salt it well. ".into(),
    })
    .await?;
    assert_eq!(
        f.editor.view().canvas.nodes,
        StepNodesView::Unselected {
            steps: vec![
                saved_node(&boil_water()),
                StepNodeView {
                    kind: StepKind::Waiting,
                    instructions: " Salt it well. ".into(),
                    ..saved_node(&cook_pasta())
                },
            ],
        }
    );
    Ok(())
}

#[test_case(" Salt it well. ", StepInstructions::try_new("Salt it well.").ok(); "trimmed instructions")]
#[test_case("   ", None; "instructions of spaces are none")]
#[tokio::test]
async fn save_writes_the_duration_the_kind_and_the_instructions(
    text: &str,
    instructions: Option<StepInstructions>,
) -> anyhow::Result<()> {
    let mut f = Fixture::new();
    f.type_title("Pasta").await?;
    let id = f.add_step(point(40, 40)).await?;
    f.name_step(id, "Boil water").await?;
    f.set_duration(id, DurationInput::Minutes(10)).await?;
    f.canvas(StepCanvasEvent::SetKind {
        id,
        kind: StepKind::Waiting,
    })
    .await?;
    f.canvas(StepCanvasEvent::SetInstructions {
        id,
        text: text.into(),
    })
    .await?;

    let save = f.send(RecipeEditorEvent::Save);
    f.save(save, Ok(pasta())).await?;
    assert_eq!(
        f.written_steps,
        [vec![StepWrite {
            duration: Some(StepDurationS::try_new(600)?),
            kind: StepKind::Waiting,
            instructions,
            ..write(id, "Boil water", point(40, 40))?
        }]]
    );
    Ok(())
}

// Test support

mod recipe_editor_support;

use recipe_editor_support::*;

/// The saved Cook pasta node, after Boil water shown as `label`.
fn cook_after(label: StepLabel) -> StepNodeView {
    StepNodeView {
        after: vec![StepAfterView {
            id: boil_water().id,
            label,
        }],
        ..saved_node(&cook_pasta())
    }
}
