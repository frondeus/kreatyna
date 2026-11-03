use krucyfiks::{Command, CommandExt, Latest, LatestCommandSlot, Message, Widget};

use crate::ingredients::name_key;

use super::{
    Recipe, RecipeId,
    capabilities::{RecipeCapabilities, RecipeEffectStream, RecipeLoadError},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecipeListEvent {
    /// Load again after a failure (the Retry button). Ignored in other states.
    Refresh,
    /// The search text as typed. Rust keeps it and filters the rows by its
    /// key. It works in every state, also while the list loads.
    Search(String),
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
    /// The recipes that match the search, in store order: by title key.
    /// Empty only when no recipe is saved: "No recipes yet".
    Ready {
        rows: Vec<RecipeListRow>,
    },
    /// Recipes are saved, but none matches the search: "No matches".
    NoMatches,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecipeListView {
    /// The search text as typed.
    pub search: String,
    pub status: RecipeListStatus,
}

/// The recipe list of the Recipes section. It keeps the search and shows
/// the recipes whose title key contains the search key. The section owns
/// the selection: it is the open read view (REC-03).
pub struct RecipeList {
    caps: RecipeCapabilities,
    search: String,
    state: ListState,
    load: LatestCommandSlot,
}

enum ListState {
    Loading,
    Failed(RecipeLoadError),
    /// Every loaded recipe, in store order. The search does not change it.
    Ready(Vec<Entry>),
}

/// A loaded recipe and its title key, computed once per load.
struct Entry {
    key: String,
    row: RecipeListRow,
}

impl RecipeList {
    pub fn new(caps: RecipeCapabilities) -> Self {
        Self {
            caps,
            search: String::new(),
            state: ListState::Loading,
            load: LatestCommandSlot::new(),
        }
    }

    pub fn test_fixture() -> (Self, RecipeEffectStream) {
        let (caps, effects) = RecipeCapabilities::testing();
        (Self::new(caps), effects)
    }

    /// Shows `Loading`, then reads the store.
    fn load(&mut self) -> Command<RecipeListOutcome> {
        self.state = ListState::Loading;
        self.start_load()
    }

    /// Loads again after a save. A ready list keeps its rows until the
    /// result, so it does not flash "Loading". A running load is replaced.
    /// The search stays.
    pub(crate) fn reload(&mut self) -> Command<RecipeListOutcome> {
        match self.state {
            ListState::Ready(_) => self.start_load(),
            ListState::Loading | ListState::Failed(_) => self.load(),
        }
    }

    /// Whether the list shows a row with this id: the search does not hide it.
    pub(crate) fn contains(&self, id: RecipeId) -> bool {
        self.visible().any(|row| row.id == id)
    }

    /// The rows that match the search. No rows while the list is not ready.
    fn visible(&self) -> impl Iterator<Item = &RecipeListRow> {
        let key = name_key(&self.search);
        let entries = match &self.state {
            ListState::Ready(entries) => entries.as_slice(),
            ListState::Loading | ListState::Failed(_) => &[],
        };
        entries
            .iter()
            .filter(move |entry| entry.key.contains(&key))
            .map(|entry| &entry.row)
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
            Message::Event(RecipeListEvent::Refresh) => match self.state {
                ListState::Failed(_) => self.load(),
                ListState::Loading | ListState::Ready(_) => Command::empty(),
            },
            Message::Event(RecipeListEvent::Search(search)) => {
                self.search = search;
                self.render()
            }
            Message::Outcome(RecipeListOutcome::LoadFinished(finished)) => {
                let Some(result) = self.load.accept(finished) else {
                    return Command::empty();
                };
                self.load.stop();
                self.state = match result {
                    // The store returns the recipes sorted by title key.
                    Ok(recipes) => ListState::Ready(
                        recipes
                            .into_iter()
                            .map(|recipe| Entry {
                                key: name_key(&recipe.title),
                                row: RecipeListRow {
                                    id: recipe.id,
                                    title: recipe.title,
                                },
                            })
                            .collect(),
                    ),
                    Err(error) => ListState::Failed(error),
                };
                self.render()
            }
        }
    }

    fn view(&self) -> Self::View {
        let status = match &self.state {
            ListState::Loading => RecipeListStatus::Loading,
            ListState::Failed(error) => RecipeListStatus::Failed(error.clone()),
            ListState::Ready(entries) => {
                let rows: Vec<_> = self.visible().cloned().collect();
                if rows.is_empty() && !entries.is_empty() {
                    RecipeListStatus::NoMatches
                } else {
                    RecipeListStatus::Ready { rows }
                }
            }
        };
        RecipeListView {
            search: self.search.clone(),
            status,
        }
    }
}
