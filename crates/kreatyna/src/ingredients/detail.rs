use krucyfiks::{Command, CommandExt, CommandId, Latest, LatestCommandSlot, Message, Widget};

use crate::confirmation::{Confirmation, ConfirmationEvent, ConfirmationOutcome, ConfirmationView};

use super::{
    Ingredient, IngredientId,
    capabilities::{IngredientCapabilities, IngredientDeleteError, IngredientEffectStream},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngredientDetailEvent {
    Edit,
    RequestDelete,
    Confirmation(ConfirmationEvent),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngredientDetailOutcome {
    OpenEdit {
        ingredient: Ingredient,
    },
    ConfirmationDecided {
        id: CommandId,
        decision: ConfirmationOutcome,
    },
    DeleteFinished {
        id: CommandId,
        result: Result<(), IngredientDeleteError>,
    },
    Deleted {
        id: IngredientId,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngredientDetailDeletion {
    Idle,
    Confirming {
        confirmation: ConfirmationView,
        error: Option<IngredientDeleteError>,
    },
    Deleting,
    Deleted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IngredientDetailView {
    pub ingredient: Ingredient,
    pub deletion: IngredientDetailDeletion,
}

enum DetailState {
    Idle,
    Confirming {
        confirmation: Confirmation,
        error: Option<IngredientDeleteError>,
    },
    Deleting,
    Deleted,
}

pub struct IngredientDetail {
    caps: IngredientCapabilities,
    ingredient: Ingredient,
    state: DetailState,
    confirmation: LatestCommandSlot,
    deletion: LatestCommandSlot,
}

impl IngredientDetail {
    pub fn new(caps: IngredientCapabilities, ingredient: Ingredient) -> Self {
        Self {
            caps,
            ingredient,
            state: DetailState::Idle,
            confirmation: LatestCommandSlot::new(),
            deletion: LatestCommandSlot::new(),
        }
    }

    pub fn test_fixture(ingredient: Ingredient) -> (Self, IngredientEffectStream) {
        let (caps, effects) = IngredientCapabilities::testing();
        (Self::new(caps, ingredient), effects)
    }

    fn render(&self) -> Command<IngredientDetailOutcome> {
        let render = self.caps.render.clone();
        Command::without_outcome(async move { render.render().await })
    }

    fn open_confirmation(&mut self, error: Option<IngredientDeleteError>) {
        self.confirmation.stop();
        self.state = DetailState::Confirming {
            confirmation: Confirmation::new(),
            error,
        };
    }

    fn delete(&mut self) -> Command<IngredientDetailOutcome> {
        self.state = DetailState::Deleting;
        let render = self.caps.render.clone();
        let store = self.caps.store.clone();
        let ingredient_id = self.ingredient.id;
        Command::single(async move {
            render.render().await;
            store.delete(ingredient_id).await
        })
        .latest(&mut self.deletion)
        .map(|Latest { id, value: result }| IngredientDetailOutcome::DeleteFinished { id, result })
    }
}

impl Widget for IngredientDetail {
    type View = IngredientDetailView;
    type Event = IngredientDetailEvent;
    type Outcome = IngredientDetailOutcome;

    fn update(&mut self, message: Message<Self>) -> Command<Self::Outcome> {
        match message {
            Message::Event(IngredientDetailEvent::Edit) => {
                if !matches!(self.state, DetailState::Idle) {
                    return Command::empty();
                }
                let ingredient = self.ingredient.clone();
                Command::single(async move { IngredientDetailOutcome::OpenEdit { ingredient } })
            }
            Message::Event(IngredientDetailEvent::RequestDelete) => {
                if !matches!(self.state, DetailState::Idle) {
                    return Command::empty();
                }
                self.open_confirmation(None);
                self.render()
            }
            Message::Event(IngredientDetailEvent::Confirmation(event)) => {
                let DetailState::Confirming { confirmation, .. } = &mut self.state else {
                    return Command::empty();
                };
                if !matches!(confirmation.view(), ConfirmationView::AwaitingDecision) {
                    return Command::empty();
                }
                confirmation
                    .update(Message::Event(event))
                    .latest(&mut self.confirmation)
                    .map(
                        |Latest {
                             id,
                             value: decision,
                         }| IngredientDetailOutcome::ConfirmationDecided {
                            id,
                            decision,
                        },
                    )
            }
            Message::Outcome(IngredientDetailOutcome::ConfirmationDecided { id, decision }) => {
                let DetailState::Confirming { confirmation, .. } = &self.state else {
                    return Command::empty();
                };
                if !self.confirmation.accepts(id)
                    || confirmation.view() != (ConfirmationView::Resolved { decision })
                {
                    return Command::empty();
                }
                self.confirmation.stop();
                match decision {
                    ConfirmationOutcome::Confirmed => self.delete(),
                    ConfirmationOutcome::Cancelled => {
                        self.state = DetailState::Idle;
                        self.render()
                    }
                }
            }
            Message::Outcome(IngredientDetailOutcome::DeleteFinished { id, result }) => {
                if !self.deletion.accepts(id) {
                    return Command::empty();
                }
                self.deletion.stop();
                match result {
                    Err(error) => {
                        self.open_confirmation(Some(error));
                        self.render()
                    }
                    Ok(()) => {
                        self.state = DetailState::Deleted;
                        let render = self.caps.render.clone();
                        let id = self.ingredient.id;
                        Command::single(async move {
                            render.render().await;
                            IngredientDetailOutcome::Deleted { id }
                        })
                    }
                }
            }
            Message::Outcome(
                IngredientDetailOutcome::OpenEdit { .. } | IngredientDetailOutcome::Deleted { .. },
            ) => Command::empty(),
        }
    }

    fn view(&self) -> Self::View {
        let deletion = match &self.state {
            DetailState::Idle => IngredientDetailDeletion::Idle,
            DetailState::Confirming {
                confirmation,
                error,
                ..
            } => IngredientDetailDeletion::Confirming {
                confirmation: confirmation.view(),
                error: error.clone(),
            },
            DetailState::Deleting => IngredientDetailDeletion::Deleting,
            DetailState::Deleted => IngredientDetailDeletion::Deleted,
        };
        IngredientDetailView {
            ingredient: self.ingredient.clone(),
            deletion,
        }
    }
}
