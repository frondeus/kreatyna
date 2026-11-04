//! The recipe editor in the detail pane. It creates a recipe from a title
//! (REC-02) or edits a saved recipe's title (REC-04). On a canvas it adds
//! and names the recipe's steps (REC-14), moves them (REC-15), deletes them
//! (REC-16), sets their details (REC-17), and connects them (REC-18).
use krucyfiks::{Command, CommandExt, Latest, LatestCommandSlot, Message, Widget};

use crate::confirmation::{Confirmation, ConfirmationEvent, ConfirmationOutcome, ConfirmationView};

use super::{
    NewRecipe, Recipe, RecipeTitle, RecipeTitleError, RecipeUpdate, Step, StepEdges, StepWrite,
    capabilities::{RecipeCapabilities, RecipeEffectStream, RecipeSaveError},
    step_canvas::{CanvasChange, ProblemDisplay, StepCanvas, StepCanvasEvent, StepCanvasView},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecipeEditorEvent {
    /// The raw text of the title field. Rust trims it only on Save.
    SetTitle(String),
    /// The steps on the canvas and in the inspector (REC-14).
    Canvas(StepCanvasEvent),
    /// Checks the recipe title and every step title, then saves.
    Save,
    /// Cancel. A changed title, step, or arrow asks "Discard changes?"
    /// first (E-10).
    RequestClose,
    Confirmation(ConfirmationEvent),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecipeEditorOutcome {
    /// The answer to "Discard changes?".
    ConfirmationDecided(Latest<ConfirmationOutcome>),
    SaveFinished(Latest<Result<Recipe, RecipeSaveError>>),
    /// The editor is finished. The parent shows what comes next.
    Saved {
        recipe: Recipe,
    },
    /// The editor is finished without a save.
    Discarded,
}

/// Why the last Save did not save. A step's problems live on its node
/// (`StepNodeView::problems`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecipeEditorProblem {
    Invalid(RecipeTitleError),
    SaveFailed(RecipeSaveError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecipeEditorStatus {
    /// Only an editable editor shows a problem.
    Editing {
        problem: Option<RecipeEditorProblem>,
    },
    ConfirmingDiscard(ConfirmationView),
    Saving,
    /// Saved or discarded. The parent replaces the editor next.
    Finished,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecipeEditorView {
    /// The title as typed.
    pub title: String,
    pub canvas: StepCanvasView,
    pub status: RecipeEditorStatus,
}

enum EditorState {
    /// The problem stays while "Discard changes?" is open, so Keep Editing
    /// shows it again.
    Editing {
        problem: Option<RecipeEditorProblem>,
    },
    ConfirmingDiscard {
        confirmation: Confirmation,
        problem: Option<RecipeEditorProblem>,
    },
    Saving,
    Finished,
}

/// What the editor edits. `new` and `edit` choose it.
enum EditorTarget {
    /// Save creates a recipe. The editor opens with an empty title.
    New,
    /// Save updates this recipe. `recipe` is the recipe as stored: the editor
    /// opens with its title and steps, and compares the edits with them.
    Saved { recipe: Recipe },
}

/// The title, the steps, and the edges the editor opens with. `is_dirty`
/// compares the edits with them.
struct OpenedWith<'a> {
    title: &'a str,
    steps: &'a [Step],
    edges: &'a StepEdges,
}

/// What a new recipe opens with.
static NO_EDGES: StepEdges = StepEdges::new();

impl EditorTarget {
    fn opened_with(&self) -> OpenedWith<'_> {
        match self {
            EditorTarget::New => OpenedWith {
                title: "",
                steps: &[],
                edges: &NO_EDGES,
            },
            EditorTarget::Saved { recipe } => OpenedWith {
                title: &recipe.title,
                steps: &recipe.steps,
                edges: &recipe.edges,
            },
        }
    }
}

pub struct RecipeEditor {
    caps: RecipeCapabilities,
    target: EditorTarget,
    title: String,
    canvas: StepCanvas,
    problems: ProblemDisplay,
    state: EditorState,
    confirmation: LatestCommandSlot,
    save: LatestCommandSlot,
}

impl RecipeEditor {
    /// An editor for a new recipe, with an empty title and no steps.
    pub fn new(caps: RecipeCapabilities) -> Self {
        Self::open(caps, EditorTarget::New)
    }

    /// An editor for a saved recipe, with its title (REC-04) and steps
    /// (REC-14).
    pub fn edit(caps: RecipeCapabilities, recipe: &Recipe) -> Self {
        let target = EditorTarget::Saved {
            recipe: recipe.clone(),
        };
        Self::open(caps, target)
    }

    fn open(caps: RecipeCapabilities, target: EditorTarget) -> Self {
        let opened_with = target.opened_with();
        let title = opened_with.title.to_owned();
        let canvas = StepCanvas::new(opened_with.steps, opened_with.edges);
        Self {
            caps,
            target,
            title,
            canvas,
            problems: ProblemDisplay::Hidden,
            state: EditorState::Editing { problem: None },
            confirmation: LatestCommandSlot::new(),
            save: LatestCommandSlot::new(),
        }
    }

    pub fn test_fixture() -> (Self, RecipeEffectStream) {
        let (caps, effects) = RecipeCapabilities::testing();
        (Self::new(caps), effects)
    }

    pub fn edit_test_fixture(recipe: &Recipe) -> (Self, RecipeEffectStream) {
        let (caps, effects) = RecipeCapabilities::testing();
        (Self::edit(caps, recipe), effects)
    }

    /// Whether the title, the steps, or the arrows differ from the ones the
    /// editor opened with: an empty title, no steps, and no arrows for a new
    /// recipe, the stored ones for a saved one.
    pub(crate) fn is_dirty(&self) -> bool {
        let opened_with = self.target.opened_with();
        self.title != opened_with.title
            || self
                .canvas
                .differs_from(opened_with.steps, opened_with.edges)
    }

    fn render(&self) -> Command<RecipeEditorOutcome> {
        let render = self.caps.render.clone();
        Command::without_outcome(async move { render.render().await })
    }

    /// Renders the finished state, then reports `outcome` to the parent.
    fn finish(&mut self, outcome: RecipeEditorOutcome) -> Command<RecipeEditorOutcome> {
        self.state = EditorState::Finished;
        let render = self.caps.render.clone();
        Command::single(async move {
            render.render().await;
            outcome
        })
    }

    /// Save. A recipe title that breaks the rule becomes the editor's
    /// problem. A step title that breaks the rule selects the first such
    /// step, so the inspector shows its problem (REC-14/Q2).
    fn save(&mut self) -> Command<RecipeEditorOutcome> {
        self.problems = ProblemDisplay::Shown;
        let title = RecipeTitle::try_new(self.title.as_str());
        let steps = self.canvas.writes();
        if let Err(untitled) = steps {
            // The view shows the selection after the render below, changed
            // or not.
            let _ = self.canvas.select(untitled);
        }
        match (title, steps) {
            (Ok(title), Ok(steps)) => self.start_save(title, steps),
            (title, _) => {
                self.state = EditorState::Editing {
                    problem: title.err().map(RecipeEditorProblem::Invalid),
                };
                self.render()
            }
        }
    }

    /// Starts the save: render `Saving`, then create a new recipe or update
    /// the saved one.
    fn start_save(
        &mut self,
        title: RecipeTitle,
        steps: Vec<StepWrite>,
    ) -> Command<RecipeEditorOutcome> {
        let edges = self.canvas.edges().clone();
        self.state = EditorState::Saving;
        let render = self.caps.render.clone();
        let store = self.caps.store.clone();
        let saved_id = match &self.target {
            EditorTarget::New => None,
            EditorTarget::Saved { recipe } => Some(recipe.id),
        };
        Command::single(async move {
            render.render().await;
            match saved_id {
                None => {
                    let recipe = NewRecipe {
                        title,
                        steps,
                        edges,
                    };
                    store.create(recipe).await
                }
                Some(id) => {
                    let recipe = RecipeUpdate {
                        title,
                        steps,
                        edges,
                    };
                    store.update(id, recipe).await
                }
            }
        })
        .latest(&mut self.save)
        .map(RecipeEditorOutcome::SaveFinished)
    }

    fn set_title(&mut self, title: String) -> Command<RecipeEditorOutcome> {
        if self.title == title {
            return Command::empty();
        }
        self.title = title;
        // A save error belongs to the old title. The title rule is checked
        // again only after the first Save.
        let problem = match self.problems {
            ProblemDisplay::Hidden => None,
            ProblemDisplay::Shown => RecipeTitle::try_new(self.title.as_str())
                .err()
                .map(RecipeEditorProblem::Invalid),
        };
        self.state = EditorState::Editing { problem };
        self.render()
    }

    /// A step change keeps the problem: a save error or a bad recipe title
    /// does not depend on the steps. A step's own problem is on its node.
    fn change_canvas(&mut self, event: StepCanvasEvent) -> Command<RecipeEditorOutcome> {
        match self.canvas.apply(event) {
            CanvasChange::Changed => self.render(),
            CanvasChange::Unchanged => Command::empty(),
        }
    }

    fn request_close(&mut self) -> Command<RecipeEditorOutcome> {
        if !self.is_dirty() {
            return self.finish(RecipeEditorOutcome::Discarded);
        }
        let EditorState::Editing { problem } = &mut self.state else {
            return Command::empty();
        };
        let problem = problem.take();
        self.confirmation.stop();
        self.state = EditorState::ConfirmingDiscard {
            confirmation: Confirmation::new(),
            problem,
        };
        self.render()
    }
}

impl Widget for RecipeEditor {
    type View = RecipeEditorView;
    type Event = RecipeEditorEvent;
    type Outcome = RecipeEditorOutcome;

    fn update(&mut self, message: Message<Self>) -> Command<Self::Outcome> {
        match message {
            Message::Event(event) => match (&mut self.state, event) {
                (EditorState::Editing { .. }, RecipeEditorEvent::SetTitle(title)) => {
                    self.set_title(title)
                }
                (EditorState::Editing { .. }, RecipeEditorEvent::Canvas(event)) => {
                    self.change_canvas(event)
                }
                (EditorState::Editing { .. }, RecipeEditorEvent::Save) => self.save(),
                (EditorState::Editing { .. }, RecipeEditorEvent::RequestClose) => {
                    self.request_close()
                }
                (
                    EditorState::ConfirmingDiscard { confirmation, .. },
                    RecipeEditorEvent::Confirmation(event),
                ) => confirmation
                    .update(Message::Event(event))
                    .latest(&mut self.confirmation)
                    .map(RecipeEditorOutcome::ConfirmationDecided),
                // Every other event does not apply in the current state.
                _ => Command::empty(),
            },
            Message::Outcome(RecipeEditorOutcome::ConfirmationDecided(decided)) => {
                let EditorState::ConfirmingDiscard {
                    confirmation,
                    problem,
                } = &mut self.state
                else {
                    return Command::empty();
                };
                let Some(decision) = self.confirmation.accept(decided) else {
                    return Command::empty();
                };
                if confirmation.view() != (ConfirmationView::Resolved { decision }) {
                    return Command::empty();
                }
                self.confirmation.stop();
                match decision {
                    ConfirmationOutcome::Confirmed => self.finish(RecipeEditorOutcome::Discarded),
                    ConfirmationOutcome::Cancelled => {
                        let problem = problem.take();
                        self.state = EditorState::Editing { problem };
                        self.render()
                    }
                }
            }
            Message::Outcome(RecipeEditorOutcome::SaveFinished(finished)) => {
                if !matches!(self.state, EditorState::Saving) {
                    return Command::empty();
                }
                let Some(result) = self.save.accept(finished) else {
                    return Command::empty();
                };
                self.save.stop();
                match result {
                    Ok(recipe) => self.finish(RecipeEditorOutcome::Saved { recipe }),
                    Err(error) => {
                        self.state = EditorState::Editing {
                            problem: Some(RecipeEditorProblem::SaveFailed(error)),
                        };
                        self.render()
                    }
                }
            }
            // The parent handles them; they come back here only by mistake.
            Message::Outcome(
                RecipeEditorOutcome::Saved { .. } | RecipeEditorOutcome::Discarded,
            ) => Command::empty(),
        }
    }

    fn view(&self) -> Self::View {
        let status = match &self.state {
            EditorState::Editing { problem } => RecipeEditorStatus::Editing {
                problem: problem.clone(),
            },
            EditorState::ConfirmingDiscard { confirmation, .. } => {
                RecipeEditorStatus::ConfirmingDiscard(confirmation.view())
            }
            EditorState::Saving => RecipeEditorStatus::Saving,
            EditorState::Finished => RecipeEditorStatus::Finished,
        };
        RecipeEditorView {
            title: self.title.clone(),
            canvas: self.canvas.view(self.problems),
            status,
        }
    }
}
