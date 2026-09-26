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
    interrupted_load: bool,
}

impl IngredientList {
    pub fn new(caps: IngredientCapabilities) -> Self {
        Self {
            caps,
            search: String::new(),
            state: ListState::InitialLoading,
            load: LatestCommandSlot::new(),
            interrupted_load: false,
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

    pub(crate) fn contains_visible(&self, ingredient: &Ingredient) -> bool {
        self.selected(ingredient.id).as_ref() == Some(ingredient)
    }

    fn start_load(&mut self) -> Command<IngredientListOutcome> {
        self.interrupted_load = false;
        let render = self.caps.render.clone();
        let store = self.caps.store.clone();
        Command::single(async move {
            render.render().await;
            store.list().await
        })
        .latest(&mut self.load)
        .map(|(id, result)| IngredientListOutcome::LoadFinished { id, result })
    }

    fn begin_load(&mut self) -> Command<IngredientListOutcome> {
        match &mut self.state {
            ListState::InitialFailed(_) => self.state = ListState::InitialLoading,
            ListState::Ready { refresh, .. } => *refresh = IngredientListRefresh::Loading,
            ListState::InitialLoading => {}
        }
        self.start_load()
    }

    pub(crate) fn suspend(&mut self) {
        let loading = matches!(
            self.state,
            ListState::InitialLoading
                | ListState::Ready {
                    refresh: IngredientListRefresh::Loading,
                    ..
                }
        );
        if loading {
            self.interrupted_load = true;
        }
        self.load.stop();
    }

    pub(crate) fn resume(&mut self) -> Option<Command<IngredientListOutcome>> {
        self.interrupted_load.then(|| self.begin_load())
    }

    pub(crate) fn reload(&mut self) -> Command<IngredientListOutcome> {
        self.begin_load()
    }

    /// Update a complete cached catalog. Returns false while the initial catalog
    /// is unavailable, so the coordinator can reload instead of showing a partial list.
    pub(crate) fn apply_saved(&mut self, ingredient: Ingredient) -> bool {
        let ListState::Ready {
            ingredients,
            refresh,
        } = &mut self.state
        else {
            return false;
        };
        self.load.stop();
        self.interrupted_load = false;
        if let Some(existing) = ingredients
            .iter_mut()
            .find(|existing| existing.id == ingredient.id)
        {
            *existing = ingredient;
        } else {
            ingredients.push(ingredient);
        }
        *refresh = IngredientListRefresh::Idle;
        true
    }

    pub(crate) fn apply_deleted(&mut self, id: IngredientId) -> bool {
        let ListState::Ready {
            ingredients,
            refresh,
        } = &mut self.state
        else {
            return false;
        };
        self.load.stop();
        self.interrupted_load = false;
        ingredients.retain(|ingredient| ingredient.id != id);
        *refresh = IngredientListRefresh::Idle;
        true
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
