//! Catalog coordinator for a split layout: the list and the detail side by side.
//!
//! The list never leaves the window, so it keeps one scope for its whole life
//! and never pauses. The pane gets a new scope for each selection change, so
//! late events and results for the previous ingredient are ignored (E-19).
use krucyfiks::{Command, CommandExt, CommandScopeId, Message, Scoped, Widget};

use crate::scoped_child::ScopedChild;

use super::{
    Ingredient, IngredientDetail, IngredientDetailDeletion, IngredientDetailEvent,
    IngredientDetailOutcome, IngredientDetailView, IngredientId, IngredientList,
    IngredientListEvent, IngredientListOutcome, IngredientListView,
    capabilities::{IngredientCapabilities, IngredientEffectStream},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngredientSplitCatalogEvent {
    List(Scoped<IngredientListEvent>),
    Detail(Scoped<IngredientDetailEvent>),
    /// Deselects the row and empties the pane. The scope is the pane's.
    ClearSelection {
        scope: CommandScopeId,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngredientSplitCatalogOutcome {
    List(Scoped<IngredientListOutcome>),
    Detail(Scoped<IngredientDetailOutcome>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngredientSplitPaneView {
    /// Nothing is selected.
    Empty,
    Detail(IngredientDetailView),
}

/// Each part carries the scope that its events must be sent with.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IngredientSplitCatalogView {
    pub list: Scoped<IngredientListView>,
    /// The highlighted row.
    pub selected: Option<IngredientId>,
    pub pane: Scoped<IngredientSplitPaneView>,
}

#[allow(clippy::large_enum_variant)] // one per catalog
enum Pane {
    Empty,
    Detail(IngredientDetail),
}

/// The detail column. An empty pane ignores every message.
impl Widget for Pane {
    type View = IngredientSplitPaneView;
    type Event = IngredientDetailEvent;
    type Outcome = IngredientDetailOutcome;

    fn init(&mut self) -> Command<Self::Outcome> {
        match self {
            Self::Empty => Command::empty(),
            Self::Detail(detail) => detail.init(),
        }
    }

    fn can_begin_shutdown(&self) -> bool {
        match self {
            Self::Empty => true,
            Self::Detail(detail) => detail.can_begin_shutdown(),
        }
    }

    fn begin_shutdown(&mut self) -> Command<Self::Outcome> {
        match self {
            Self::Empty => Command::empty(),
            Self::Detail(detail) => detail.begin_shutdown(),
        }
    }

    fn update(&mut self, message: Message<Self>) -> Command<Self::Outcome> {
        let Self::Detail(detail) = self else {
            return Command::empty();
        };
        match message {
            Message::Event(event) => detail.update(Message::Event(event)),
            Message::Outcome(outcome) => detail.update(Message::Outcome(outcome)),
        }
    }

    fn view(&self) -> Self::View {
        match self {
            Self::Empty => IngredientSplitPaneView::Empty,
            Self::Detail(detail) => IngredientSplitPaneView::Detail(detail.view()),
        }
    }
}

pub struct IngredientSplitCatalog {
    caps: IngredientCapabilities,
    list: ScopedChild<IngredientList>,
    pane: ScopedChild<Pane>,
}

impl IngredientSplitCatalog {
    pub fn new(caps: IngredientCapabilities) -> Self {
        Self {
            list: ScopedChild::new(IngredientList::new(caps.clone())),
            caps,
            pane: ScopedChild::new(Pane::Empty),
        }
    }

    pub fn test_fixture() -> (Self, IngredientEffectStream) {
        let (caps, effects) = IngredientCapabilities::testing();
        (Self::new(caps), effects)
    }

    fn selected(&self) -> Option<IngredientId> {
        match &self.pane.widget {
            Pane::Empty => None,
            Pane::Detail(detail) => Some(detail.view().ingredient.id),
        }
    }

    /// A running or confirmed delete keeps its detail until it finishes.
    fn pane_is_locked(&self) -> bool {
        match &self.pane.widget {
            Pane::Empty => false,
            Pane::Detail(detail) => detail.view().deletion != IngredientDetailDeletion::Idle,
        }
    }

    /// Renders belong to the list's scope, which lives as long as the catalog.
    fn render(&mut self) -> Command<IngredientSplitCatalogOutcome> {
        let render = self.caps.render.clone();
        self.list
            .command(Command::<()>::without_outcome(async move {
                render.render().await
            }))
            .map(|_| unreachable!("a render command has no outcome"))
    }

    fn list_command(
        &mut self,
        command: Command<IngredientListOutcome>,
    ) -> Command<IngredientSplitCatalogOutcome> {
        self.list
            .command(command)
            .map(IngredientSplitCatalogOutcome::List)
    }

    fn detail_command(
        &mut self,
        command: Command<IngredientDetailOutcome>,
    ) -> Command<IngredientSplitCatalogOutcome> {
        self.pane
            .command(command)
            .map(IngredientSplitCatalogOutcome::Detail)
    }

    /// Replaces the pane in a new scope, which cancels the old pane's commands.
    fn show(&mut self, pane: Pane) -> Command<IngredientSplitCatalogOutcome> {
        self.pane.replace(pane);
        let init = self.pane.widget.init();
        Command::select_all(vec![self.detail_command(init), self.render()])
    }

    fn select(&mut self, ingredient: Ingredient) -> Command<IngredientSplitCatalogOutcome> {
        if self.selected() == Some(ingredient.id)
            || self.pane_is_locked()
            || !self.list.widget.contains_visible(&ingredient)
        {
            return Command::empty();
        }
        let detail = IngredientDetail::new(self.caps.clone(), ingredient);
        self.show(Pane::Detail(detail))
    }

    fn handle_list_outcome(
        &mut self,
        outcome: IngredientListOutcome,
    ) -> Command<IngredientSplitCatalogOutcome> {
        match outcome {
            IngredientListOutcome::OpenDetail { ingredient } => self.select(ingredient),
            // Create and edit arrive with MAC-05 and MAC-06.
            IngredientListOutcome::CreateRequested | IngredientListOutcome::OpenEdit { .. } => {
                Command::empty()
            }
            outcome @ IngredientListOutcome::LoadFinished { .. } => {
                let command = self.list.widget.update(Message::Outcome(outcome));
                self.list_command(command)
            }
        }
    }

    fn handle_detail_outcome(
        &mut self,
        outcome: IngredientDetailOutcome,
    ) -> Command<IngredientSplitCatalogOutcome> {
        let Pane::Detail(detail) = &mut self.pane.widget else {
            return Command::empty();
        };
        match outcome {
            // Edit arrives with MAC-06.
            IngredientDetailOutcome::OpenEdit { .. } => Command::empty(),
            IngredientDetailOutcome::Deleted { id } => {
                if detail.view().deletion != IngredientDetailDeletion::Deleted
                    || detail.view().ingredient.id != id
                {
                    return Command::empty();
                }
                let complete = self.list.widget.apply_deleted(id);
                let reload = if complete {
                    None
                } else {
                    Some(self.list.widget.reload())
                };
                let show = self.show(Pane::Empty);
                match reload {
                    Some(reload) => Command::select_all(vec![self.list_command(reload), show]),
                    None => show,
                }
            }
            outcome => {
                let command = self.pane.widget.update(Message::Outcome(outcome));
                self.detail_command(command)
            }
        }
    }
}

impl Widget for IngredientSplitCatalog {
    type View = IngredientSplitCatalogView;
    type Event = IngredientSplitCatalogEvent;
    type Outcome = IngredientSplitCatalogOutcome;

    fn init(&mut self) -> Command<Self::Outcome> {
        let command = self.list.widget.init();
        self.list_command(command)
    }

    fn can_begin_shutdown(&self) -> bool {
        self.list.widget.can_begin_shutdown() && self.pane.widget.can_begin_shutdown()
    }

    fn begin_shutdown(&mut self) -> Command<Self::Outcome> {
        let list = self.list.widget.begin_shutdown();
        let pane = self.pane.widget.begin_shutdown();
        Command::select_all(vec![self.list_command(list), self.detail_command(pane)])
    }

    fn update(&mut self, message: Message<Self>) -> Command<Self::Outcome> {
        match message {
            Message::Event(IngredientSplitCatalogEvent::List(scoped)) => self
                .list
                .forward(scoped)
                .map(IngredientSplitCatalogOutcome::List),
            Message::Event(IngredientSplitCatalogEvent::Detail(scoped)) => self
                .pane
                .forward(scoped)
                .map(IngredientSplitCatalogOutcome::Detail),
            Message::Event(IngredientSplitCatalogEvent::ClearSelection { scope }) => {
                if scope != self.pane.scope()
                    || matches!(self.pane.widget, Pane::Empty)
                    || self.pane_is_locked()
                {
                    return Command::empty();
                }
                self.show(Pane::Empty)
            }
            Message::Outcome(IngredientSplitCatalogOutcome::List(scoped)) => {
                match self.list.accept(scoped) {
                    Some(outcome) => self.handle_list_outcome(outcome),
                    None => Command::empty(),
                }
            }
            Message::Outcome(IngredientSplitCatalogOutcome::Detail(scoped)) => {
                match self.pane.accept(scoped) {
                    Some(outcome) => self.handle_detail_outcome(outcome),
                    None => Command::empty(),
                }
            }
        }
    }

    fn view(&self) -> Self::View {
        IngredientSplitCatalogView {
            list: self.list.view(IngredientList::view),
            selected: self.selected(),
            pane: self.pane.view(Pane::view),
        }
    }
}
