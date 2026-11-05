//! Test support that `recipe_editor.rs` and `step_canvas.rs` share: the
//! editor fixture and the recipes it opens. `recipe_section.rs` uses the
//! chips. Each test file compiles this module on its own and uses only part
//! of it.
#![expect(
    dead_code,
    reason = "each test file compiles this module on its own and uses only part of it"
)]

use kreatyna::{
    capabilities::ResolveRender,
    confirmation::{ConfirmationEvent, ConfirmationView},
    recipes::{
        CanvasPoint, DurationChip, DurationEntry, DurationInput, DurationParts, IngredientLine,
        IngredientLineId, IngredientLineName, IngredientLinesEvent, Recipe, RecipeEditor,
        RecipeEditorEvent, RecipeEditorOutcome, RecipeEditorProblem, RecipeEditorStatus, RecipeId,
        RecipeTitleError, Step, StepCanvasEvent, StepCanvasView, StepDurationS, StepDurationView,
        StepEdge, StepEdges, StepId, StepInstructions, StepKind, StepLabel, StepNodeView,
        StepNodesView, StepProblems, StepTitle, StepTitleError, StepWrite,
        capabilities::{
            RecipeEffectStream, RecipeSaveError, RecipeStoreRequest, ResolveRecipeStore,
        },
    },
};
use krucyfiks::{Command, Message, Widget, effects::CommandTestExt};
use uuid::Uuid;

pub(crate) fn shakshuka() -> Recipe {
    Recipe {
        id: RecipeId::from(Uuid::from_u128(1)),
        title: "Shakshuka".into(),
        lines: vec![],
        steps: vec![],
        edges: StepEdges::default(),
    }
}

/// A saved recipe with two steps.
pub(crate) fn pasta() -> Recipe {
    Recipe {
        id: RecipeId::from(Uuid::from_u128(2)),
        title: "Pasta".into(),
        lines: vec![],
        steps: vec![boil_water(), cook_pasta()],
        edges: StepEdges::default(),
    }
}

/// `pasta()` with two saved lines: penne pasta, then olive oil.
pub(crate) fn pasta_with_lines() -> anyhow::Result<Recipe> {
    Ok(Recipe {
        lines: vec![penne()?, olive_oil()?],
        ..pasta()
    })
}

pub(crate) fn penne() -> anyhow::Result<IngredientLine> {
    saved_line(21, "penne pasta")
}

pub(crate) fn olive_oil() -> anyhow::Result<IngredientLine> {
    saved_line(22, "olive oil")
}

pub(crate) fn saved_line(id: u128, name: &str) -> anyhow::Result<IngredientLine> {
    Ok(IngredientLine {
        id: IngredientLineId::from(Uuid::from_u128(id)),
        name: IngredientLineName::try_new(name)?,
    })
}

/// `pasta()` with an arrow: Cook pasta comes after Boil water.
/// `from_saved` cannot refuse this one edge.
pub(crate) fn connected_pasta() -> Recipe {
    Recipe {
        edges: StepEdges::from_saved([boil_then_cook()]).unwrap_or_default(),
        ..pasta()
    }
}

pub(crate) fn boil_then_cook() -> StepEdge {
    StepEdge {
        from: boil_water().id,
        to: cook_pasta().id,
    }
}

pub(crate) fn boil_water() -> Step {
    Step {
        id: StepId::from(Uuid::from_u128(11)),
        title: "Boil water".into(),
        position: point(40, 40),
        duration: None,
        kind: StepKind::Active,
        instructions: None,
    }
}

pub(crate) fn cook_pasta() -> Step {
    Step {
        id: StepId::from(Uuid::from_u128(12)),
        title: "Cook pasta".into(),
        position: point(240, 40),
        duration: None,
        kind: StepKind::Active,
        instructions: None,
    }
}

/// A saved step with a duration, Waiting, and instructions.
pub(crate) fn bake_sauce() -> Step {
    Step {
        id: StepId::from(Uuid::from_u128(13)),
        title: "Bake sauce".into(),
        position: point(440, 40),
        duration: StepDurationS::try_new(1_200).ok(),
        kind: StepKind::Waiting,
        instructions: StepInstructions::try_new("Cover the dish.").ok(),
    }
}

/// A saved recipe with `bake_sauce()` only.
pub(crate) fn sauce() -> Recipe {
    Recipe {
        id: RecipeId::from(Uuid::from_u128(3)),
        title: "Sauce".into(),
        lines: vec![],
        steps: vec![bake_sauce()],
        edges: StepEdges::default(),
    }
}

pub(crate) fn point(x: u32, y: u32) -> CanvasPoint {
    CanvasPoint { x, y }
}

pub(crate) fn node(
    id: StepId,
    title: &str,
    label: StepLabel,
    position: CanvasPoint,
    title_problem: Option<StepTitleError>,
) -> StepNodeView {
    StepNodeView {
        id,
        title: title.into(),
        label,
        position,
        duration: StepDurationView::NotSet,
        kind: StepKind::Active,
        instructions: String::new(),
        problems: StepProblems {
            title: title_problem,
            duration: None,
        },
        after: vec![],
    }
}

pub(crate) fn set(text: &str, parts: DurationParts, entry: DurationEntry) -> StepDurationView {
    StepDurationView::Set {
        text: text.into(),
        parts,
        entry,
    }
}

pub(crate) fn minutes(minutes: u32) -> DurationEntry {
    DurationEntry::Minutes { minutes }
}

pub(crate) fn parts(days: u32, hours: u32, minutes: u32, seconds: u32) -> DurationParts {
    DurationParts {
        days,
        hours,
        minutes,
        seconds,
    }
}

pub(crate) fn titled(title: &str) -> StepLabel {
    StepLabel::Titled {
        title: title.into(),
    }
}

/// A saved step's node: saved steps always pass the rules.
pub(crate) fn saved_node(step: &Step) -> StepNodeView {
    let duration = match step.duration {
        None => StepDurationView::NotSet,
        Some(duration) => set(&duration.text(), duration.parts(), duration.entry()),
    };
    StepNodeView {
        duration,
        kind: step.kind,
        instructions: step
            .instructions
            .as_ref()
            .map_or_else(String::new, |text| text.as_ref().to_owned()),
        ..node(
            step.id,
            &step.title,
            titled(&step.title),
            step.position,
            None,
        )
    }
}

/// The canvas of `recipe` as the edit editor opens it: no selection.
pub(crate) fn saved_canvas(recipe: &Recipe) -> StepCanvasView {
    canvas(StepNodesView::Unselected {
        steps: recipe.steps.iter().map(saved_node).collect(),
    })
}

/// `nodes` with no arrows and no refusal.
pub(crate) fn canvas(nodes: StepNodesView) -> StepCanvasView {
    StepCanvasView {
        nodes,
        connections: vec![],
        refusal: None,
        duration_chips: chips(),
    }
}

/// The Duration field's chips, as every canvas shows them (REC-17/Q1).
pub(crate) fn chips() -> Vec<DurationChip> {
    [
        (1, "1 min"),
        (2, "2 min"),
        (5, "5 min"),
        (10, "10 min"),
        (15, "15 min"),
        (20, "20 min"),
        (30, "30 min"),
        (45, "45 min"),
        (60, "1 h"),
    ]
    .map(|(minutes, text)| DurationChip {
        minutes,
        text: text.into(),
    })
    .into()
}

pub(crate) fn write(id: StepId, title: &str, position: CanvasPoint) -> anyhow::Result<StepWrite> {
    Ok(StepWrite {
        id,
        title: StepTitle::try_new(title)?,
        position,
        duration: None,
        kind: StepKind::Active,
        instructions: None,
    })
}

pub(crate) fn empty_title() -> RecipeEditorProblem {
    RecipeEditorProblem::Invalid(RecipeTitleError::NotEmptyViolated)
}

/// A store call that Save made, with the title it wrote.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Write {
    Created(String),
    Updated(RecipeId, String),
}

pub(crate) struct Fixture {
    pub(crate) editor: RecipeEditor,
    pub(crate) effects: RecipeEffectStream,
    /// The store calls of the saves, in order.
    pub(crate) writes: Vec<Write>,
    /// The steps that each store call wrote, in the order of `writes`.
    pub(crate) written_steps: Vec<Vec<StepWrite>>,
    /// The edges that each store call wrote, in the order of `writes`.
    pub(crate) written_edges: Vec<StepEdges>,
    /// The lines that each store call wrote, in the order of `writes`.
    pub(crate) written_lines: Vec<Vec<IngredientLine>>,
}

impl Fixture {
    /// An editor for a new recipe.
    pub(crate) fn new() -> Self {
        let (editor, effects) = RecipeEditor::test_fixture();
        Self::with(editor, effects)
    }

    /// An editor for the saved `shakshuka()`.
    pub(crate) fn edit() -> Self {
        Self::edit_recipe(&shakshuka())
    }

    /// An editor for the saved `recipe`.
    pub(crate) fn edit_recipe(recipe: &Recipe) -> Self {
        let (editor, effects) = RecipeEditor::edit_test_fixture(recipe);
        Self::with(editor, effects)
    }

    pub(crate) fn with(editor: RecipeEditor, effects: RecipeEffectStream) -> Self {
        Self {
            editor,
            effects,
            writes: vec![],
            written_steps: vec![],
            written_edges: vec![],
            written_lines: vec![],
        }
    }

    pub(crate) fn send(&mut self, event: RecipeEditorEvent) -> Command<RecipeEditorOutcome> {
        self.editor.update(Message::Event(event))
    }

    /// Runs a command that only renders.
    pub(crate) async fn render(
        &mut self,
        command: Command<RecipeEditorOutcome>,
    ) -> anyhow::Result<()> {
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
    pub(crate) async fn render_then_outcome(
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

    pub(crate) async fn type_title(&mut self, title: &str) -> anyhow::Result<()> {
        let command = self.send(RecipeEditorEvent::SetTitle(title.into()));
        self.render(command).await
    }

    /// Sends a canvas event that changes the canvas, and runs its render.
    pub(crate) async fn canvas(&mut self, event: StepCanvasEvent) -> anyhow::Result<()> {
        let command = self.send(RecipeEditorEvent::Canvas(event));
        self.render(command).await
    }

    /// Right-click ▸ Add Step at `at`. Returns the new step's id.
    pub(crate) async fn add_step(&mut self, at: CanvasPoint) -> anyhow::Result<StepId> {
        self.canvas(StepCanvasEvent::Add { at }).await?;
        self.selected()
    }

    pub(crate) async fn name_step(&mut self, id: StepId, title: &str) -> anyhow::Result<()> {
        let title = title.into();
        self.canvas(StepCanvasEvent::SetTitle { id, title }).await
    }

    /// A drop of the step `id` at `to`.
    pub(crate) async fn move_step(&mut self, id: StepId, to: CanvasPoint) -> anyhow::Result<()> {
        self.canvas(StepCanvasEvent::Move { id, to }).await
    }

    pub(crate) async fn set_duration(
        &mut self,
        id: StepId,
        input: DurationInput,
    ) -> anyhow::Result<()> {
        self.canvas(StepCanvasEvent::SetDuration { id, input })
            .await
    }

    /// A drop of a ● drag from `from` on the node `to`.
    pub(crate) async fn connect(&mut self, from: StepId, to: StepId) -> anyhow::Result<()> {
        self.canvas(StepCanvasEvent::Connect { from, to }).await
    }

    pub(crate) async fn delete_step(&mut self, id: StepId) -> anyhow::Result<()> {
        self.canvas(StepCanvasEvent::Delete(id)).await
    }

    /// Sends a line event that changes the lines, and runs its render.
    pub(crate) async fn lines(&mut self, event: IngredientLinesEvent) -> anyhow::Result<()> {
        let command = self.send(RecipeEditorEvent::Lines(event));
        self.render(command).await
    }

    /// ＋ Line. Returns the new line's id.
    pub(crate) async fn add_line(&mut self) -> anyhow::Result<IngredientLineId> {
        self.lines(IngredientLinesEvent::Add).await?;
        self.editor
            .view()
            .lines
            .last()
            .map(|line| line.id)
            .ok_or_else(|| anyhow::anyhow!("＋ Line added no line"))
    }

    pub(crate) async fn name_line(
        &mut self,
        id: IngredientLineId,
        name: &str,
    ) -> anyhow::Result<()> {
        let name = name.into();
        self.lines(IngredientLinesEvent::SetName { id, name }).await
    }

    /// The names of the lines as the editor shows them, in list order.
    pub(crate) fn line_names(&self) -> Vec<String> {
        self.editor
            .view()
            .lines
            .into_iter()
            .map(|line| line.name)
            .collect()
    }

    /// The id of the selected step.
    pub(crate) fn selected(&self) -> anyhow::Result<StepId> {
        match self.editor.view().canvas.nodes {
            StepNodesView::Selected { selected, .. } => Ok(selected.id),
            StepNodesView::Unselected { .. } | StepNodesView::ConnectionSelected { .. } => {
                anyhow::bail!("no step is selected")
            }
        }
    }

    /// Runs a save command: its render, then the store call (create or
    /// update) answered with `result`. Returns `SaveFinished`, not yet
    /// dispatched.
    pub(crate) async fn save(
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
        let mut edges = None;
        let mut lines = None;
        let store = command.step().await?.expect_effect().expect_store();
        match store.request {
            RecipeStoreRequest::Create { .. } => {
                store
                    .resolve_create(|recipe| {
                        write = Some(Write::Created(recipe.title.as_ref().to_owned()));
                        lines = Some(recipe.lines);
                        steps = Some(recipe.steps);
                        edges = Some(recipe.edges);
                        async { result }
                    })
                    .await?
            }
            RecipeStoreRequest::Update { .. } => {
                store
                    .resolve_update(|id, recipe| {
                        write = Some(Write::Updated(id, recipe.title.as_ref().to_owned()));
                        lines = Some(recipe.lines);
                        steps = Some(recipe.steps);
                        edges = Some(recipe.edges);
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
        self.written_edges.extend(edges);
        self.written_lines.extend(lines);
        Ok(finished)
    }

    /// Cancel on a changed title or step list: the editor asks "Discard
    /// changes?".
    pub(crate) async fn request_close(&mut self) -> anyhow::Result<()> {
        let command = self.send(RecipeEditorEvent::RequestClose);
        assert_eq!(
            self.editor.view().status,
            RecipeEditorStatus::ConfirmingDiscard(ConfirmationView::AwaitingDecision)
        );
        self.render(command).await
    }

    /// Answers "Discard changes?". Returns `ConfirmationDecided`, not yet
    /// dispatched.
    pub(crate) async fn decide(
        &mut self,
        event: ConfirmationEvent,
    ) -> anyhow::Result<RecipeEditorOutcome> {
        let command = self.send(RecipeEditorEvent::Confirmation(event));
        let mut command = command.driver(&mut self.effects);
        let decided = command.step().await?.expect_outcome();
        command.step().await?.expect_done();
        Ok(decided)
    }
}
