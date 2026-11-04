//! The step canvas of the recipe editor (REC-14 to REC-18): adding,
//! selecting, naming, moving, deleting, timing, and connecting steps, and
//! the step rules that Save checks. The tests drive it through the editor.
use kreatyna::recipes::{
    RecipeEditorEvent, RecipeEditorStatus, StepAfterView, StepCanvasEvent, StepCanvasView,
    StepConnectionView, StepEdges, StepLabel, StepNodeView, StepNodesView,
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
