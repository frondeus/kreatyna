//! Catalog coordinator for a split layout: the list and the detail side by side.
//!
//! The list never leaves the window, so it keeps one scope for its whole life
//! and never pauses. The pane gets a new scope each time its content changes,
//! so late events and results for the previous content are ignored (E-19).
//! The pane shows nothing, an ingredient's detail, the create form, or the
//! edit form.
use krucyfiks::{Command, CommandExt, CommandScopeId, Message, Scoped, Widget};

use crate::{CreateAvailability, scoped_child::ScopedChild};

use super::{
    Ingredient, IngredientDetail, IngredientDetailDeletion, IngredientDetailEvent,
    IngredientDetailOutcome, IngredientDetailView, IngredientDraft, IngredientForm,
    IngredientFormEvent, IngredientFormOutcome, IngredientFormStatus, IngredientFormView,
    IngredientId, IngredientList, IngredientListEvent, IngredientListOutcome, IngredientListView,
    NutritionBasis,
    capabilities::{IngredientCapabilities, IngredientEffectStream},
    list::ListRefresh,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngredientSplitCatalogEvent {
    List(Scoped<IngredientListEvent>),
    Detail(Scoped<IngredientDetailEvent>),
    Form(Scoped<IngredientFormEvent>),
    /// Deselects the row and empties the pane. The scope is the pane's.
    ClearSelection {
        scope: CommandScopeId,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngredientSplitCatalogOutcome {
    List(Scoped<IngredientListOutcome>),
    Pane(Scoped<IngredientSplitPaneOutcome>),
}

/// An outcome of the detail pane's content: a detail or a form.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngredientSplitPaneOutcome {
    Detail(IngredientDetailOutcome),
    Form(IngredientFormOutcome),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngredientSplitPaneView {
    /// Nothing is selected.
    Empty,
    Detail(IngredientDetailView),
    Create(IngredientFormView),
    /// The edit form. `saved_name` is the name as stored: the pane title
    /// shows it until Save (MAC-06/Q1).
    Edit {
        saved_name: String,
        form: IngredientFormView,
    },
}

/// Each part carries the scope that its events must be sent with.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IngredientSplitCatalogView {
    pub list: Scoped<IngredientListView>,
    /// The highlighted row: the open detail or the edited ingredient. `None`
    /// while the create form is open.
    pub selected: Option<IngredientId>,
    pub pane: Scoped<IngredientSplitPaneView>,
    pub create: CreateAvailability,
    /// The draft row at the top of the list while the create form is open:
    /// the typed name, trimmed. An empty name has no text yet (MAC-05/Q2).
    /// The search never hides it.
    pub draft_row: Option<String>,
}

/// A form, and where the pane goes when the form is discarded.
///
/// Two fields, because the two destinations are set at different moments and
/// Keep Editing must clear only one of them:
///
/// | You do | `return_to` | `clicked_row` | After Discard |
/// |---|---|---|---|
/// | Garlic open, ＋, type "Pap", Cancel → Discard | Garlic | none | Garlic |
/// | Garlic open, ＋, type "Pap", click Tomato → Discard | Garlic | Tomato | Tomato |
/// | Same click on Tomato, Keep Editing, then Cancel → Discard | Garlic | cleared at Keep Editing | Garlic |
/// | Nothing open, ＋, Cancel | none | none | empty pane |
///
/// For the edit form, `return_to` is the edited ingredient as stored.
struct FormPane {
    form: IngredientForm,
    /// The ingredient that was open when ＋ was pressed, or the edited
    /// ingredient as stored. Cancel goes back to it.
    return_to: Option<Ingredient>,
    /// The row clicked while "Discard changes?" is open (E-10). Keep Editing
    /// clears it.
    clicked_row: Option<Ingredient>,
}

impl FormPane {
    /// Where Discard goes: the clicked row wins, then `return_to`, then nothing.
    fn discard_destination(&mut self) -> Option<Ingredient> {
        self.clicked_row.take().or_else(|| self.return_to.take())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PaneEvent {
    Detail(IngredientDetailEvent),
    Form(IngredientFormEvent),
}

enum Pane {
    Empty,
    Detail(IngredientDetail),
    Create(FormPane),
    /// `saved` is the edited ingredient as stored. The row and the title show
    /// it until Save (MAC-06/Q1).
    Edit {
        saved: Ingredient,
        form: FormPane,
    },
}

impl Pane {
    fn form_pane(&mut self) -> Option<&mut FormPane> {
        match self {
            Self::Create(form) | Self::Edit { form, .. } => Some(form),
            Self::Empty | Self::Detail(_) => None,
        }
    }
}

/// The detail column. A message for another kind of content is ignored.
impl Widget for Pane {
    type View = IngredientSplitPaneView;
    type Event = PaneEvent;
    type Outcome = IngredientSplitPaneOutcome;

    fn init(&mut self) -> Command<Self::Outcome> {
        match self {
            Self::Empty => Command::empty(),
            Self::Detail(detail) => detail.init().map(IngredientSplitPaneOutcome::Detail),
            Self::Create(pane) | Self::Edit { form: pane, .. } => {
                pane.form.init().map(IngredientSplitPaneOutcome::Form)
            }
        }
    }

    fn can_begin_shutdown(&self) -> bool {
        match self {
            Self::Empty => true,
            Self::Detail(detail) => detail.can_begin_shutdown(),
            Self::Create(pane) | Self::Edit { form: pane, .. } => pane.form.can_begin_shutdown(),
        }
    }

    fn begin_shutdown(&mut self) -> Command<Self::Outcome> {
        match self {
            Self::Empty => Command::empty(),
            Self::Detail(detail) => detail
                .begin_shutdown()
                .map(IngredientSplitPaneOutcome::Detail),
            Self::Create(pane) | Self::Edit { form: pane, .. } => pane
                .form
                .begin_shutdown()
                .map(IngredientSplitPaneOutcome::Form),
        }
    }

    fn update(&mut self, message: Message<Self>) -> Command<Self::Outcome> {
        match (self, message) {
            (Self::Detail(detail), Message::Event(PaneEvent::Detail(event))) => detail
                .update(Message::Event(event))
                .map(IngredientSplitPaneOutcome::Detail),
            (
                Self::Detail(detail),
                Message::Outcome(IngredientSplitPaneOutcome::Detail(outcome)),
            ) => detail
                .update(Message::Outcome(outcome))
                .map(IngredientSplitPaneOutcome::Detail),
            (pane, Message::Event(PaneEvent::Form(event))) => match pane.form_pane() {
                Some(pane) => pane
                    .form
                    .update(Message::Event(event))
                    .map(IngredientSplitPaneOutcome::Form),
                None => Command::empty(),
            },
            (pane, Message::Outcome(IngredientSplitPaneOutcome::Form(outcome))) => {
                match pane.form_pane() {
                    Some(pane) => pane
                        .form
                        .update(Message::Outcome(outcome))
                        .map(IngredientSplitPaneOutcome::Form),
                    None => Command::empty(),
                }
            }
            _ => Command::empty(),
        }
    }

    fn view(&self) -> Self::View {
        match self {
            Self::Empty => IngredientSplitPaneView::Empty,
            Self::Detail(detail) => IngredientSplitPaneView::Detail(detail.view()),
            Self::Create(pane) => IngredientSplitPaneView::Create(pane.form.view()),
            Self::Edit { saved, form } => IngredientSplitPaneView::Edit {
                saved_name: saved.name.to_string(),
                form: form.form.view(),
            },
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
            Pane::Detail(detail) => Some(detail.view().ingredient.id),
            Pane::Edit { saved, .. } => Some(saved.id),
            Pane::Empty | Pane::Create(_) => None,
        }
    }

    fn draft_row(&self) -> Option<String> {
        match &self.pane.widget {
            Pane::Create(pane) => Some(pane.form.view().draft.name.trim().to_owned()),
            Pane::Empty | Pane::Detail(_) | Pane::Edit { .. } => None,
        }
    }

    /// A running or confirmed delete keeps its detail until it finishes.
    fn detail_is_locked(detail: &IngredientDetail) -> bool {
        detail.view().deletion != IngredientDetailDeletion::Idle
    }

    fn create(&self) -> CreateAvailability {
        match &self.pane.widget {
            Pane::Empty => CreateAvailability::Available,
            Pane::Detail(detail) if Self::detail_is_locked(detail) => {
                CreateAvailability::Unavailable
            }
            Pane::Detail(_) => CreateAvailability::Available,
            Pane::Create(_) | Pane::Edit { .. } => CreateAvailability::Unavailable,
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

    fn pane_command(
        &mut self,
        command: Command<IngredientSplitPaneOutcome>,
    ) -> Command<IngredientSplitCatalogOutcome> {
        self.pane
            .command(command)
            .map(IngredientSplitCatalogOutcome::Pane)
    }

    fn forward_to_pane(
        &mut self,
        event: Scoped<PaneEvent>,
    ) -> Command<IngredientSplitCatalogOutcome> {
        match self.pane.accept(event) {
            Some(event) => {
                let command = self.pane.widget.update(Message::Event(event));
                self.pane_command(command)
            }
            None => Command::empty(),
        }
    }

    /// Replaces the pane in a new scope, which cancels the old pane's commands.
    fn show(&mut self, pane: Pane) -> Command<IngredientSplitCatalogOutcome> {
        self.pane.replace(pane);
        let init = self.pane.widget.init();
        Command::select_all(vec![self.pane_command(init), self.render()])
    }

    fn show_detail(
        &mut self,
        ingredient: Option<Ingredient>,
    ) -> Command<IngredientSplitCatalogOutcome> {
        let pane = match ingredient {
            Some(ingredient) => Pane::Detail(IngredientDetail::new(self.caps.clone(), ingredient)),
            None => Pane::Empty,
        };
        self.show(pane)
    }

    fn open_create(&mut self) -> Command<IngredientSplitCatalogOutcome> {
        let return_to = match &self.pane.widget {
            Pane::Empty => None,
            Pane::Detail(detail) if !Self::detail_is_locked(detail) => {
                Some(detail.view().ingredient)
            }
            Pane::Detail(_) | Pane::Create(_) | Pane::Edit { .. } => return Command::empty(),
        };
        let form = IngredientForm::new(
            self.caps.clone(),
            IngredientDraft::new(NutritionBasis::Per100Grams),
        );
        self.show(Pane::Create(FormPane {
            form,
            return_to,
            clicked_row: None,
        }))
    }

    /// Edit in the detail. The detail must still show that ingredient, with
    /// no delete running.
    fn open_edit(&mut self, ingredient: Ingredient) -> Command<IngredientSplitCatalogOutcome> {
        let Pane::Detail(detail) = &self.pane.widget else {
            return Command::empty();
        };
        if detail.view().ingredient.id != ingredient.id || Self::detail_is_locked(detail) {
            return Command::empty();
        }
        let form = IngredientForm::new(self.caps.clone(), IngredientDraft::from(&ingredient));
        self.show(Pane::Edit {
            saved: ingredient.clone(),
            form: FormPane {
                form,
                return_to: Some(ingredient),
                clicked_row: None,
            },
        })
    }

    /// A row click. A dirty form asks "Discard changes?" first (E-10).
    fn select(&mut self, ingredient: Ingredient) -> Command<IngredientSplitCatalogOutcome> {
        if !self.list.widget.contains_visible(&ingredient) {
            return Command::empty();
        }
        match &mut self.pane.widget {
            Pane::Empty => {}
            Pane::Detail(detail) => {
                if detail.view().ingredient.id == ingredient.id || Self::detail_is_locked(detail) {
                    return Command::empty();
                }
            }
            Pane::Edit { saved, .. } if saved.id == ingredient.id => {
                return Command::empty();
            }
            Pane::Create(pane) | Pane::Edit { form: pane, .. } => {
                if pane.form.view().status != IngredientFormStatus::Editing {
                    return Command::empty();
                }
                if pane.form.is_dirty() {
                    pane.clicked_row = Some(ingredient);
                    let command = pane
                        .form
                        .update(Message::Event(IngredientFormEvent::RequestClose))
                        .map(IngredientSplitPaneOutcome::Form);
                    return self.pane_command(command);
                }
            }
        }
        self.show_detail(Some(ingredient))
    }

    fn handle_list_outcome(
        &mut self,
        outcome: IngredientListOutcome,
    ) -> Command<IngredientSplitCatalogOutcome> {
        match outcome {
            IngredientListOutcome::OpenDetail { ingredient } => self.select(ingredient),
            IngredientListOutcome::CreateRequested => self.open_create(),
            // No Mac list action sends it: Edit is in the detail.
            IngredientListOutcome::OpenEdit { .. } => Command::empty(),
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
            IngredientDetailOutcome::OpenEdit { ingredient } => self.open_edit(ingredient),
            IngredientDetailOutcome::Deleted { id } => {
                if detail.view().deletion != IngredientDetailDeletion::Deleted
                    || detail.view().ingredient.id != id
                {
                    return Command::empty();
                }
                let refresh = self.list.widget.apply_deleted(id);
                self.refresh_list_and_show(refresh, None)
            }
            outcome => {
                let command = detail
                    .update(Message::Outcome(outcome))
                    .map(IngredientSplitPaneOutcome::Detail);
                self.pane_command(command)
            }
        }
    }

    fn handle_form_outcome(
        &mut self,
        outcome: IngredientFormOutcome,
    ) -> Command<IngredientSplitCatalogOutcome> {
        let Some(pane) = self.pane.widget.form_pane() else {
            return Command::empty();
        };
        match outcome {
            IngredientFormOutcome::Saved { ingredient } => {
                if pane.form.view().status != IngredientFormStatus::Finished {
                    return Command::empty();
                }
                let refresh = self.list.widget.apply_saved(ingredient.clone());
                self.refresh_list_and_show(refresh, Some(ingredient))
            }
            IngredientFormOutcome::Discarded => {
                if pane.form.view().status != IngredientFormStatus::Finished {
                    return Command::empty();
                }
                let next = pane.discard_destination();
                self.show_detail(next)
            }
            outcome => {
                let command = pane
                    .form
                    .update(Message::Outcome(outcome))
                    .map(IngredientSplitPaneOutcome::Form);
                // Keep Editing (or a failed save) forgets the clicked row.
                if pane.form.view().status == IngredientFormStatus::Editing {
                    pane.clicked_row = None;
                }
                self.pane_command(command)
            }
        }
    }

    /// Shows a detail (or the empty pane) after a write. The list reloads when
    /// it could not apply the write to a complete catalog.
    fn refresh_list_and_show(
        &mut self,
        refresh: ListRefresh,
        ingredient: Option<Ingredient>,
    ) -> Command<IngredientSplitCatalogOutcome> {
        let reload = match refresh {
            ListRefresh::Resume => None,
            ListRefresh::Reload => Some(self.list.widget.reload()),
        };
        let show = self.show_detail(ingredient);
        match reload {
            Some(reload) => Command::select_all(vec![self.list_command(reload), show]),
            None => show,
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
        Command::select_all(vec![self.list_command(list), self.pane_command(pane)])
    }

    fn update(&mut self, message: Message<Self>) -> Command<Self::Outcome> {
        match message {
            Message::Event(IngredientSplitCatalogEvent::List(scoped)) => self
                .list
                .forward(scoped)
                .map(IngredientSplitCatalogOutcome::List),
            Message::Event(IngredientSplitCatalogEvent::Detail(scoped)) => {
                self.forward_to_pane(scoped.map(PaneEvent::Detail))
            }
            Message::Event(IngredientSplitCatalogEvent::Form(scoped)) => {
                self.forward_to_pane(scoped.map(PaneEvent::Form))
            }
            Message::Event(IngredientSplitCatalogEvent::ClearSelection { scope }) => {
                let Pane::Detail(detail) = &self.pane.widget else {
                    return Command::empty();
                };
                if scope != self.pane.scope() || Self::detail_is_locked(detail) {
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
            Message::Outcome(IngredientSplitCatalogOutcome::Pane(scoped)) => {
                match self.pane.accept(scoped) {
                    Some(IngredientSplitPaneOutcome::Detail(outcome)) => {
                        self.handle_detail_outcome(outcome)
                    }
                    Some(IngredientSplitPaneOutcome::Form(outcome)) => {
                        self.handle_form_outcome(outcome)
                    }
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
            create: self.create(),
            draft_row: self.draft_row(),
        }
    }
}
