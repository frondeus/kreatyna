use krucyfiks::{Command, CommandExt, Message, Widget};

use super::{
    Recipe, RecipeId,
    capabilities::{RecipeCapabilities, RecipeEffectStream, RecipeLoadError},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecipeListEvent {
    /// Load again after a failure (the Retry button). Ignored in other states.
    Refresh,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecipeListOutcome {
    /// Only one load runs at a time, so a result needs no command id.
    LoadFinished {
        result: Result<Vec<Recipe>, RecipeLoadError>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecipeListRow {
    pub id: RecipeId,
    pub title: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecipeListStatus {
    Loading,
    Failed(RecipeLoadError),
    /// Rows in store order: by title key. Empty means "No recipes yet".
    Ready {
        rows: Vec<RecipeListRow>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecipeListView {
    pub status: RecipeListStatus,
}

/// The recipe list of the Recipes section. It has no search (REC-06) and no
/// selection (REC-03).
pub struct RecipeList {
    caps: RecipeCapabilities,
    status: RecipeListStatus,
}

impl RecipeList {
    pub fn new(caps: RecipeCapabilities) -> Self {
        Self {
            caps,
            status: RecipeListStatus::Loading,
        }
    }

    pub fn test_fixture() -> (Self, RecipeEffectStream) {
        let (caps, effects) = RecipeCapabilities::testing();
        (Self::new(caps), effects)
    }

    /// Renders `Loading`, then reads the store.
    fn load(&mut self) -> Command<RecipeListOutcome> {
        self.status = RecipeListStatus::Loading;
        let render = self.caps.render.clone();
        let store = self.caps.store.clone();
        Command::single(async move {
            render.render().await;
            RecipeListOutcome::LoadFinished {
                result: store.list().await,
            }
        })
    }

    fn render(&self) -> Command<RecipeListOutcome> {
        let render = self.caps.render.clone();
        Command::without_outcome(async move { render.render().await })
    }
}

impl Widget for RecipeList {
    type View = RecipeListView;
    type Event = RecipeListEvent;
    type Outcome = RecipeListOutcome;

    fn init(&mut self) -> Command<Self::Outcome> {
        self.load()
    }

    fn update(&mut self, message: Message<Self>) -> Command<Self::Outcome> {
        match message {
            Message::Event(RecipeListEvent::Refresh) => match self.status {
                RecipeListStatus::Failed(_) => self.load(),
                RecipeListStatus::Loading | RecipeListStatus::Ready { .. } => Command::empty(),
            },
            Message::Outcome(RecipeListOutcome::LoadFinished { result }) => {
                self.status = match result {
                    // The store returns the recipes sorted by title key.
                    Ok(recipes) => RecipeListStatus::Ready {
                        rows: recipes
                            .into_iter()
                            .map(|recipe| RecipeListRow {
                                id: recipe.id,
                                title: recipe.title,
                            })
                            .collect(),
                    },
                    Err(error) => RecipeListStatus::Failed(error),
                };
                self.render()
            }
        }
    }

    fn view(&self) -> Self::View {
        RecipeListView {
            status: self.status.clone(),
        }
    }
}
