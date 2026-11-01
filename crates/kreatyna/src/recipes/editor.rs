//! The recipe editor in the detail pane. In REC-02 it creates a recipe from a
//! title; REC-04 adds editing a saved recipe.
use krucyfiks::{Command, CommandExt, Latest, LatestCommandSlot, Message, Widget};

use crate::confirmation::{Confirmation, ConfirmationEvent, ConfirmationOutcome, ConfirmationView};

use super::{
    NewRecipe, Recipe, RecipeTitle, RecipeTitleError,
    capabilities::{RecipeCapabilities, RecipeEffectStream, RecipeSaveError},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecipeEditorEvent {
    /// The raw text of the title field. Rust trims it only on Save.
    SetTitle(String),
    Save,
    /// Cancel. A changed title asks "Discard changes?" first (E-10).
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

/// Why the last Save did not save.
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

pub struct RecipeEditor {
    caps: RecipeCapabilities,
    title: String,
    /// After the first Save, every change checks the title again.
    save_attempted: bool,
    state: EditorState,
    confirmation: LatestCommandSlot,
    save: LatestCommandSlot,
}

impl RecipeEditor {
    /// An editor for a new recipe, with an empty title.
    pub fn new(caps: RecipeCapabilities) -> Self {
        Self {
            caps,
            title: String::new(),
            save_attempted: false,
            state: EditorState::Editing { problem: None },
            confirmation: LatestCommandSlot::new(),
            save: LatestCommandSlot::new(),
        }
    }

    pub fn test_fixture() -> (Self, RecipeEffectStream) {
        let (caps, effects) = RecipeCapabilities::testing();
        (Self::new(caps), effects)
    }

    /// Whether the title differs from the one the editor opened with.
    fn is_dirty(&self) -> bool {
        !self.title.is_empty()
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

    /// Save. A title that breaks the rule becomes the editor's problem.
    fn save(&mut self) -> Command<RecipeEditorOutcome> {
        self.save_attempted = true;
        match self.try_save() {
            Ok(save) => save,
            Err(error) => {
                self.state = EditorState::Editing {
                    problem: Some(RecipeEditorProblem::Invalid(error)),
                };
                self.render()
            }
        }
    }

    /// Checks the title, then starts the save: render `Saving`, then create.
    fn try_save(&mut self) -> Result<Command<RecipeEditorOutcome>, RecipeTitleError> {
        let title = RecipeTitle::try_new(self.title.as_str())?;
        self.state = EditorState::Saving;
        let render = self.caps.render.clone();
        let store = self.caps.store.clone();
        Ok(Command::single(async move {
            render.render().await;
            store.create(NewRecipe { title }).await
        })
        .latest(&mut self.save)
        .map(RecipeEditorOutcome::SaveFinished))
    }

    fn set_title(&mut self, title: String) -> Command<RecipeEditorOutcome> {
        if self.title == title {
            return Command::empty();
        }
        self.title = title;
        // A save error belongs to the old title. The title rule is checked
        // again only after the first Save.
        let problem = if self.save_attempted {
            RecipeTitle::try_new(self.title.as_str())
                .err()
                .map(RecipeEditorProblem::Invalid)
        } else {
            None
        };
        self.state = EditorState::Editing { problem };
        self.render()
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
            status,
        }
    }
}
