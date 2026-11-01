use krucyfiks::{Command, CommandExt, Latest, LatestCommandSlot, Message, Widget};

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
    /// A new load replaces a running one, so only the newest result counts.
    LoadFinished(Latest<Result<Vec<Recipe>, RecipeLoadError>>),
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
    load: LatestCommandSlot,
}

impl RecipeList {
    pub fn new(caps: RecipeCapabilities) -> Self {
        Self {
            caps,
            status: RecipeListStatus::Loading,
            load: LatestCommandSlot::new(),
        }
    }

    pub fn test_fixture() -> (Self, RecipeEffectStream) {
        let (caps, effects) = RecipeCapabilities::testing();
        (Self::new(caps), effects)
    }

    /// Shows `Loading`, then reads the store.
    fn load(&mut self) -> Command<RecipeListOutcome> {
        self.status = RecipeListStatus::Loading;
        self.start_load()
    }

    /// Loads again after a save. A ready list keeps its rows until the
    /// result, so it does not flash "Loading". A running load is replaced.
    pub(crate) fn reload(&mut self) -> Command<RecipeListOutcome> {
        match self.status {
            RecipeListStatus::Ready { .. } => self.start_load(),
            RecipeListStatus::Loading | RecipeListStatus::Failed(_) => self.load(),
        }
    }

    /// Renders, then reads the store. The newest load wins.
    fn start_load(&mut self) -> Command<RecipeListOutcome> {
        let render = self.caps.render.clone();
        let store = self.caps.store.clone();
        Command::single(async move {
            render.render().await;
            store.list().await
        })
        .latest(&mut self.load)
        .map(RecipeListOutcome::LoadFinished)
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
            Message::Outcome(RecipeListOutcome::LoadFinished(finished)) => {
                let Some(result) = self.load.accept(finished) else {
                    return Command::empty();
                };
                self.load.stop();
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
