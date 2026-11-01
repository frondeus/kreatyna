use krucyfiks::{Command, CommandExt, Latest, LatestCommandSlot, Message, Widget};

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
    LoadFinished(Latest<Result<Vec<Ingredient>, IngredientLoadError>>),
    CreateRequested,
    OpenDetail { ingredient: Ingredient },
    OpenEdit { ingredient: Ingredient },
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
        catalog: Catalog,
        refresh: IngredientListRefresh,
    },
}

/// The loaded ingredients, kept in list order: by name key, then by id.
/// Each entry keeps its name key, so a render or a lookup sorts nothing.
struct Catalog(Vec<CatalogEntry>);

struct CatalogEntry {
    key: String,
    ingredient: Ingredient,
}

impl CatalogEntry {
    fn new(ingredient: Ingredient) -> Self {
        Self {
            key: name_key(&ingredient.name),
            ingredient,
        }
    }

    fn order(&self) -> (&str, IngredientId) {
        (&self.key, self.ingredient.id)
    }
}

impl Catalog {
    fn new(ingredients: Vec<Ingredient>) -> Self {
        let mut entries: Vec<_> = ingredients.into_iter().map(CatalogEntry::new).collect();
        entries.sort_by(|left, right| left.order().cmp(&right.order()));
        Self(entries)
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn get(&self, id: IngredientId) -> Option<&CatalogEntry> {
        self.0.iter().find(|entry| entry.ingredient.id == id)
    }

    fn remove(&mut self, id: IngredientId) {
        self.0.retain(|entry| entry.ingredient.id != id);
    }

    /// Adds the ingredient, or replaces the one with its id, in list order.
    fn upsert(&mut self, ingredient: Ingredient) {
        self.remove(ingredient.id);
        let entry = CatalogEntry::new(ingredient);
        let index = self
            .0
            .partition_point(|existing| existing.order() < entry.order());
        self.0.insert(index, entry);
    }
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
        let ListState::Ready { catalog, .. } = &self.state else {
            return Vec::new();
        };
        let query = name_key(&self.search);
        catalog
            .0
            .iter()
            .filter(|entry| entry.key.contains(&query))
            .map(|entry| &entry.ingredient)
            .collect()
    }

    /// The ingredient with this id, if the search shows it.
    fn selected(&self, id: IngredientId) -> Option<Ingredient> {
        let ListState::Ready { catalog, .. } = &self.state else {
            return None;
        };
        catalog
            .get(id)
            .filter(|entry| entry.key.contains(&name_key(&self.search)))
            .map(|entry| entry.ingredient.clone())
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
        .map(IngredientListOutcome::LoadFinished)
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
        let ListState::Ready { catalog, refresh } = &mut self.state else {
            return false;
        };
        self.load.stop();
        self.interrupted_load = false;
        catalog.upsert(ingredient);
        *refresh = IngredientListRefresh::Idle;
        true
    }

    pub(crate) fn apply_deleted(&mut self, id: IngredientId) -> bool {
        let ListState::Ready { catalog, refresh } = &mut self.state else {
            return false;
        };
        self.load.stop();
        self.interrupted_load = false;
        catalog.remove(id);
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
            Message::Outcome(IngredientListOutcome::LoadFinished(finished)) => {
                let Some(result) = self.load.accept(finished) else {
                    return Command::empty();
                };
                self.load.stop();
                match result {
                    Ok(ingredients) => {
                        self.state = ListState::Ready {
                            catalog: Catalog::new(ingredients),
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
            ListState::Ready { catalog, refresh } => IngredientListStatus::Ready {
                rows: self
                    .visible()
                    .into_iter()
                    .map(|ingredient| IngredientListRow {
                        id: ingredient.id,
                        name: ingredient.name.clone(),
                    })
                    .collect(),
                total_count: catalog.len(),
                refresh: refresh.clone(),
            },
        };
        IngredientListView {
            search: self.search.clone(),
            status,
        }
    }
}
