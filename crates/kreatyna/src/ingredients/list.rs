use futures::stream;
use krucyfiks::{Command, CommandExt, CommandId, LatestCommandSlot, Message, Widget};

use super::{
    Ingredient, IngredientId,
    capabilities::{IngredientCapabilities, IngredientEffectStream, IngredientLoadError},
    name_key,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngredientListEvent {
    Search(String),
    Refresh,
    CreateRequested,
    Select(IngredientId),
    Edit(IngredientId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngredientListOutcome {
    LoadFinished {
        id: CommandId,
        result: Result<Vec<Ingredient>, IngredientLoadError>,
    },
    CreateRequested,
    OpenDetail {
        ingredient: Ingredient,
    },
    OpenEdit {
        ingredient: Ingredient,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IngredientListRow {
    pub id: IngredientId,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngredientListRefresh {
    Idle,
    Loading,
    Failed(IngredientLoadError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngredientListStatus {
    Loading,
    Failed(IngredientLoadError),
    Ready {
        rows: Vec<IngredientListRow>,
        total_count: usize,
        refresh: IngredientListRefresh,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IngredientListView {
    pub search: String,
    pub status: IngredientListStatus,
}

enum ListState {
    InitialLoading,
    InitialFailed(IngredientLoadError),
    Ready {
        ingredients: Vec<Ingredient>,
        refresh: IngredientListRefresh,
    },
}

pub struct IngredientList {
    caps: IngredientCapabilities,
    search: String,
    state: ListState,
    load: LatestCommandSlot,
}

impl IngredientList {
    pub fn new(caps: IngredientCapabilities) -> Self {
        Self {
            caps,
            search: String::new(),
            state: ListState::InitialLoading,
            load: LatestCommandSlot::new(),
        }
    }

    pub fn test_fixture() -> (Self, IngredientEffectStream) {
        let (caps, effects) = IngredientCapabilities::testing();
        (Self::new(caps), effects)
    }

    fn visible(&self) -> Vec<&Ingredient> {
        let ListState::Ready { ingredients, .. } = &self.state else {
            return Vec::new();
        };
        let query = name_key(&self.search);
        let mut result: Vec<_> = ingredients
            .iter()
            .filter(|ingredient| name_key(&ingredient.name).contains(&query))
            .collect();
        result.sort_by(|left, right| {
            name_key(&left.name)
                .cmp(&name_key(&right.name))
                .then_with(|| left.id.cmp(&right.id))
        });
        result
    }

    fn selected(&self, id: IngredientId) -> Option<Ingredient> {
        self.visible()
            .into_iter()
            .find(|ingredient| ingredient.id == id)
            .cloned()
    }

    fn start_load(&mut self) -> Command<IngredientListOutcome> {
        let render = self.caps.render.clone();
        let store = self.caps.store.clone();
        Command::latest(&mut self.load, |id| {
            stream::once(async move {
                render.render().await;
                IngredientListOutcome::LoadFinished {
                    id,
                    result: store.list().await,
                }
            })
        })
    }

    fn begin_load(&mut self) -> Command<IngredientListOutcome> {
        match &mut self.state {
            ListState::InitialFailed(_) => self.state = ListState::InitialLoading,
            ListState::Ready { refresh, .. } => *refresh = IngredientListRefresh::Loading,
            ListState::InitialLoading => {}
        }
        self.start_load()
    }
}

impl Widget for IngredientList {
    type View = IngredientListView;
    type Event = IngredientListEvent;
    type Outcome = IngredientListOutcome;

    fn init(&mut self) -> Command<Self::Outcome> {
        self.start_load()
    }

    fn update(&mut self, message: Message<Self>) -> Command<Self::Outcome> {
        match message {
            Message::Event(IngredientListEvent::Refresh) => self.begin_load(),
            Message::Event(IngredientListEvent::Search(search)) => {
                self.search = search;
                let render = self.caps.render.clone();
                Command::without_outcome(async move { render.render().await })
            }
            Message::Event(IngredientListEvent::CreateRequested) => {
                Command::single(async { IngredientListOutcome::CreateRequested })
            }
            Message::Event(IngredientListEvent::Select(id)) => {
                if let Some(ingredient) = self.selected(id) {
                    Command::single(async move { IngredientListOutcome::OpenDetail { ingredient } })
                } else {
                    Command::empty()
                }
            }
            Message::Event(IngredientListEvent::Edit(id)) => {
                if let Some(ingredient) = self.selected(id) {
                    Command::single(async move { IngredientListOutcome::OpenEdit { ingredient } })
                } else {
                    Command::empty()
                }
            }
            Message::Outcome(IngredientListOutcome::LoadFinished { id, result }) => {
                if !self.load.accepts(id) {
                    return Command::empty();
                }
                self.load.stop();
                match result {
                    Ok(ingredients) => {
                        self.state = ListState::Ready {
                            ingredients,
                            refresh: IngredientListRefresh::Idle,
                        };
                    }
                    Err(error) => match &mut self.state {
                        ListState::Ready { refresh, .. } => {
                            *refresh = IngredientListRefresh::Failed(error);
                        }
                        _ => self.state = ListState::InitialFailed(error),
                    },
                }
                let render = self.caps.render.clone();
                Command::without_outcome(async move { render.render().await })
            }
            Message::Outcome(
                IngredientListOutcome::CreateRequested
                | IngredientListOutcome::OpenDetail { .. }
                | IngredientListOutcome::OpenEdit { .. },
            ) => Command::empty(),
        }
    }

    fn view(&self) -> Self::View {
        let status = match &self.state {
            ListState::InitialLoading => IngredientListStatus::Loading,
            ListState::InitialFailed(error) => IngredientListStatus::Failed(error.clone()),
            ListState::Ready {
                ingredients,
                refresh,
            } => IngredientListStatus::Ready {
                rows: self
                    .visible()
                    .into_iter()
                    .map(|ingredient| IngredientListRow {
                        id: ingredient.id,
                        name: ingredient.name.clone(),
                    })
                    .collect(),
                total_count: ingredients.len(),
                refresh: refresh.clone(),
            },
        };
        IngredientListView {
            search: self.search.clone(),
            status,
        }
    }
}
