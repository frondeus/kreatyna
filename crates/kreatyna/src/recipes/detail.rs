//! The read view of one saved recipe in the detail pane (REC-03). It loads
//! the recipe by id each time it opens (E-08). REC-07 adds the ingredient
//! lines, REC-14 the steps, and REC-19 shows the steps as stages (D-16).
use krucyfiks::{Command, CommandExt, Latest, LatestCommandSlot, Message, Widget};

use super::{
    Recipe, RecipeId,
    capabilities::{RecipeCapabilities, RecipeEffectStream, RecipeLoadError},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecipeDetailEvent {
    /// Load again after a failure (the Retry button). Ignored in other states.
    Retry,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecipeDetailOutcome {
    /// `None`: no recipe has this id.
    LoadFinished(Latest<Result<Option<Recipe>, RecipeLoadError>>),
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
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecipeDetailView {
    pub id: RecipeId,
    pub status: RecipeDetailStatus,
}

pub struct RecipeDetail {
    caps: RecipeCapabilities,
    id: RecipeId,
    status: RecipeDetailStatus,
    load: LatestCommandSlot,
}

impl RecipeDetail {
    /// A read view of the recipe with this id. `init` loads it.
    pub fn new(caps: RecipeCapabilities, id: RecipeId) -> Self {
        Self {
            caps,
            id,
            status: RecipeDetailStatus::Loading,
            load: LatestCommandSlot::new(),
        }
    }

    pub fn test_fixture(id: RecipeId) -> (Self, RecipeEffectStream) {
        let (caps, effects) = RecipeCapabilities::testing();
        (Self::new(caps, id), effects)
    }

    /// Shows `Loading`, renders, then reads the store. A new load replaces a
    /// running one.
    fn load(&mut self) -> Command<RecipeDetailOutcome> {
        self.status = RecipeDetailStatus::Loading;
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
            Message::Event(RecipeDetailEvent::Retry) => match self.status {
                RecipeDetailStatus::Failed(_) => self.load(),
                RecipeDetailStatus::Loading
                | RecipeDetailStatus::Missing
                | RecipeDetailStatus::Ready { .. } => Command::empty(),
            },
            Message::Outcome(RecipeDetailOutcome::LoadFinished(finished)) => {
                let Some(result) = self.load.accept(finished) else {
                    return Command::empty();
                };
                self.load.stop();
                self.status = match result {
                    Ok(Some(recipe)) => RecipeDetailStatus::Ready {
                        title: recipe.title,
                    },
                    Ok(None) => RecipeDetailStatus::Missing,
                    Err(error) => RecipeDetailStatus::Failed(error),
                };
                self.render()
            }
        }
    }

    fn view(&self) -> Self::View {
        RecipeDetailView {
            id: self.id,
            status: self.status.clone(),
        }
    }
}
