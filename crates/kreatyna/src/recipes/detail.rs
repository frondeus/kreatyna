//! The read view of one saved recipe in the detail pane (REC-03). It loads
//! the recipe by id each time it opens (E-08). REC-07 adds the ingredient
//! lines, REC-14 the steps, and REC-19 shows the steps as stages (D-16).
//! REC-05 deletes the shown recipe after a question (MAC-07).
use krucyfiks::{Command, CommandExt, Latest, LatestCommandSlot, Message, Widget};

use super::{
    Recipe, RecipeId,
    capabilities::{RecipeCapabilities, RecipeDeleteError, RecipeEffectStream, RecipeLoadError},
};
use crate::confirmation::{Confirmation, ConfirmationEvent, ConfirmationOutcome, ConfirmationView};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecipeDetailEvent {
    /// Load again after a failure (the Retry button). Ignored in other states.
    Retry,
    /// The Edit button (REC-04). Ignored unless the recipe is shown and no
    /// delete is asked or runs.
    Edit,
    /// The Delete button (REC-05). Ignored unless the recipe is shown and no
    /// delete is asked or runs.
    RequestDelete,
    /// Delete or Cancel in the question. Ignored unless the question waits.
    Confirmation(ConfirmationEvent),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecipeDetailOutcome {
    /// `None`: no recipe has this id.
    LoadFinished(Latest<Result<Option<Recipe>, RecipeLoadError>>),
    /// The section opens the editor for this recipe, as shown.
    OpenEdit {
        recipe: Recipe,
    },
    ConfirmationDecided(Latest<ConfirmationOutcome>),
    DeleteFinished(Latest<Result<(), RecipeDeleteError>>),
    /// The store deleted the recipe. The section empties the pane.
    Deleted {
        id: RecipeId,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecipeDetailStatus {
    Loading,
    Failed(RecipeLoadError),
    /// No recipe has this id any more, for example after a delete in another
    /// process.
    Missing,
    Ready {
        title: String,
        deletion: RecipeDeletion,
    },
}

/// The delete of the shown recipe (MAC-07).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecipeDeletion {
    Idle,
    /// The question is open. `error` is the last failed delete, if any.
    Confirming {
        confirmation: ConfirmationView,
        error: Option<RecipeDeleteError>,
    },
    Deleting,
    /// The store deleted the recipe. The section replaces the pane next.
    Deleted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecipeDetailView {
    pub id: RecipeId,
    pub status: RecipeDetailStatus,
}

enum State {
    Loading,
    Failed(RecipeLoadError),
    Missing,
    Ready { title: String, deletion: Deletion },
}

enum Deletion {
    Idle,
    Confirming {
        confirmation: Confirmation,
        error: Option<RecipeDeleteError>,
    },
    Deleting,
    Deleted,
}

pub struct RecipeDetail {
    caps: RecipeCapabilities,
    id: RecipeId,
    state: State,
    load: LatestCommandSlot,
    confirmation: LatestCommandSlot,
    deletion: LatestCommandSlot,
}

impl RecipeDetail {
    /// A read view of the recipe with this id. `init` loads it.
    pub fn new(caps: RecipeCapabilities, id: RecipeId) -> Self {
        Self {
            caps,
            id,
            state: State::Loading,
            load: LatestCommandSlot::new(),
            confirmation: LatestCommandSlot::new(),
            deletion: LatestCommandSlot::new(),
        }
    }

    pub fn test_fixture(id: RecipeId) -> (Self, RecipeEffectStream) {
        let (caps, effects) = RecipeCapabilities::testing();
        (Self::new(caps, id), effects)
    }

    /// Shows `Loading`, renders, then reads the store. A new load replaces a
    /// running one.
    fn load(&mut self) -> Command<RecipeDetailOutcome> {
        self.state = State::Loading;
        let render = self.caps.render.clone();
        let store = self.caps.store.clone();
        let id = self.id;
        Command::single(async move {
            render.render().await;
            store.get(id).await
        })
        .latest(&mut self.load)
        .map(RecipeDetailOutcome::LoadFinished)
    }

    fn render(&self) -> Command<RecipeDetailOutcome> {
        let render = self.caps.render.clone();
        Command::without_outcome(async move { render.render().await })
    }

    /// The shown title, when the recipe is shown and no delete is asked or
    /// runs.
    fn idle_title(&self) -> Option<&str> {
        match &self.state {
            State::Ready {
                title,
                deletion: Deletion::Idle,
            } => Some(title),
            State::Ready { .. } | State::Loading | State::Failed(_) | State::Missing => None,
        }
    }

    /// Sets the delete state of the shown recipe. Only called while Ready.
    fn set_deletion(&mut self, next: Deletion) {
        if let State::Ready { deletion, .. } = &mut self.state {
            *deletion = next;
        }
    }

    fn open_confirmation(&mut self, error: Option<RecipeDeleteError>) {
        self.confirmation.stop();
        self.set_deletion(Deletion::Confirming {
            confirmation: Confirmation::new(),
            error,
        });
    }

    fn delete(&mut self) -> Command<RecipeDetailOutcome> {
        self.set_deletion(Deletion::Deleting);
        let render = self.caps.render.clone();
        let store = self.caps.store.clone();
        let id = self.id;
        Command::single(async move {
            render.render().await;
            store.delete(id).await
        })
        .latest(&mut self.deletion)
        .map(RecipeDetailOutcome::DeleteFinished)
    }

    fn decide(&mut self, decided: Latest<ConfirmationOutcome>) -> Command<RecipeDetailOutcome> {
        let State::Ready {
            deletion: Deletion::Confirming { confirmation, .. },
            ..
        } = &self.state
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
            ConfirmationOutcome::Confirmed => self.delete(),
            ConfirmationOutcome::Cancelled => {
                self.set_deletion(Deletion::Idle);
                self.render()
            }
        }
    }

    fn delete_finished(
        &mut self,
        finished: Latest<Result<(), RecipeDeleteError>>,
    ) -> Command<RecipeDetailOutcome> {
        if !matches!(
            self.state,
            State::Ready {
                deletion: Deletion::Deleting,
                ..
            }
        ) {
            return Command::empty();
        }
        let Some(result) = self.deletion.accept(finished) else {
            return Command::empty();
        };
        self.deletion.stop();
        match result {
            Err(error) => {
                self.open_confirmation(Some(error));
                self.render()
            }
            Ok(()) => {
                self.set_deletion(Deletion::Deleted);
                let render = self.caps.render.clone();
                let id = self.id;
                Command::single(async move {
                    render.render().await;
                    RecipeDetailOutcome::Deleted { id }
                })
            }
        }
    }
}

impl Widget for RecipeDetail {
    type View = RecipeDetailView;
    type Event = RecipeDetailEvent;
    type Outcome = RecipeDetailOutcome;

    fn init(&mut self) -> Command<Self::Outcome> {
        self.load()
    }

    fn update(&mut self, message: Message<Self>) -> Command<Self::Outcome> {
        match message {
            Message::Event(RecipeDetailEvent::Retry) => match self.state {
                State::Failed(_) => self.load(),
                State::Loading | State::Missing | State::Ready { .. } => Command::empty(),
            },
            Message::Event(RecipeDetailEvent::Edit) => {
                let Some(title) = self.idle_title() else {
                    return Command::empty();
                };
                // No step can exist before the editor adds them (REC-14
                // change 4). Change 3 keeps the loaded steps here.
                let recipe = Recipe {
                    id: self.id,
                    title: title.to_owned(),
                    steps: vec![],
                };
                Command::single(async move { RecipeDetailOutcome::OpenEdit { recipe } })
            }
            Message::Event(RecipeDetailEvent::RequestDelete) => {
                if self.idle_title().is_none() {
                    return Command::empty();
                }
                self.open_confirmation(None);
                self.render()
            }
            Message::Event(RecipeDetailEvent::Confirmation(event)) => {
                let State::Ready {
                    deletion: Deletion::Confirming { confirmation, .. },
                    ..
                } = &mut self.state
                else {
                    return Command::empty();
                };
                if confirmation.view() != ConfirmationView::AwaitingDecision {
                    return Command::empty();
                }
                confirmation
                    .update(Message::Event(event))
                    .latest(&mut self.confirmation)
                    .map(RecipeDetailOutcome::ConfirmationDecided)
            }
            Message::Outcome(RecipeDetailOutcome::LoadFinished(finished)) => {
                let Some(result) = self.load.accept(finished) else {
                    return Command::empty();
                };
                self.load.stop();
                self.state = match result {
                    Ok(Some(recipe)) => State::Ready {
                        title: recipe.title,
                        deletion: Deletion::Idle,
                    },
                    Ok(None) => State::Missing,
                    Err(error) => State::Failed(error),
                };
                self.render()
            }
            Message::Outcome(RecipeDetailOutcome::ConfirmationDecided(decided)) => {
                self.decide(decided)
            }
            Message::Outcome(RecipeDetailOutcome::DeleteFinished(finished)) => {
                self.delete_finished(finished)
            }
            // The section handles them; they come back here only by mistake.
            Message::Outcome(
                RecipeDetailOutcome::OpenEdit { .. } | RecipeDetailOutcome::Deleted { .. },
            ) => Command::empty(),
        }
    }

    fn view(&self) -> Self::View {
        let status = match &self.state {
            State::Loading => RecipeDetailStatus::Loading,
            State::Failed(error) => RecipeDetailStatus::Failed(error.clone()),
            State::Missing => RecipeDetailStatus::Missing,
            State::Ready { title, deletion } => RecipeDetailStatus::Ready {
                title: title.clone(),
                deletion: match deletion {
                    Deletion::Idle => RecipeDeletion::Idle,
                    Deletion::Confirming {
                        confirmation,
                        error,
                    } => RecipeDeletion::Confirming {
                        confirmation: confirmation.view(),
                        error: error.clone(),
                    },
                    Deletion::Deleting => RecipeDeletion::Deleting,
                    Deletion::Deleted => RecipeDeletion::Deleted,
                },
            },
        };
        RecipeDetailView {
            id: self.id,
            status,
        }
    }
}
