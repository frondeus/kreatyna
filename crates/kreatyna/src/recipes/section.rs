//! The Recipes section of the split layout: the recipe list and the detail
//! pane beside it.
//!
//! The list lives as long as the section. The pane gets a new scope each time
//! its content changes, so late events and results of old content are
//! ignored (E-19). The pane is empty, a recipe's read view (REC-03), the
//! create editor (REC-02), or the edit editor (REC-04). A delete in the read
//! view keeps the pane until it ends, then empties it (REC-05).
use krucyfiks::{Command, CommandExt, CommandScopeId, Message, Scoped, Widget};

use crate::{CreateAvailability, scoped_child::ScopedChild};

use super::{
    Recipe, RecipeDeletion, RecipeDetail, RecipeDetailEvent, RecipeDetailOutcome,
    RecipeDetailStatus, RecipeDetailView, RecipeEditor, RecipeEditorEvent, RecipeEditorOutcome,
    RecipeEditorStatus, RecipeEditorView, RecipeId, RecipeList, RecipeListEvent, RecipeListOutcome,
    RecipeListView,
    capabilities::{RecipeCapabilities, RecipeEffectStream},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecipeSectionEvent {
    List(RecipeListEvent),
    /// New Recipe or ⌘N. Ignored while the editor is open.
    NewRecipe,
    /// A row click. Ignored for a row that the list does not show.
    Select(RecipeId),
    /// ⌘-click on the open recipe. The scope is the pane's.
    ClearSelection {
        scope: CommandScopeId,
    },
    /// The scope is the pane's.
    Detail(Scoped<RecipeDetailEvent>),
    /// The scope is the pane's.
    Editor(Scoped<RecipeEditorEvent>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecipeSectionOutcome {
    List(RecipeListOutcome),
    Pane(Scoped<RecipePaneOutcome>),
}

/// An outcome of the pane's content: a read view or the editor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecipePaneOutcome {
    Detail(RecipeDetailOutcome),
    Editor(RecipeEditorOutcome),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecipePaneView {
    /// "Select a recipe".
    Empty,
    /// The read view of the selected recipe.
    Detail(RecipeDetailView),
    Create {
        editor: RecipeEditorView,
        /// The typed title, trimmed, for the draft row at the top of the
        /// list. Empty shows "New Recipe" (REC-02/Q1).
        draft_row: String,
    },
    /// The edit editor. `saved_title` is the title as stored: the heading
    /// and the row show it until Save (MAC-06/Q1).
    Edit {
        editor: RecipeEditorView,
        saved_title: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecipeSectionView {
    pub list: RecipeListView,
    /// The highlighted row: the open or the edited recipe. `None` while the
    /// create editor is open; the draft row is highlighted then.
    pub selected: Option<RecipeId>,
    /// The scope goes with read view and editor events.
    pub pane: Scoped<RecipePaneView>,
    pub create: CreateAvailability,
}

/// An editor, and where the pane goes when it is discarded. The ingredient
/// catalog's `FormPane` explains the two fields with a table (REC-03/Q2).
struct EditorPane {
    editor: RecipeEditor,
    /// The recipe that was open when New Recipe was pressed, or the edited
    /// recipe. Cancel goes back to it.
    return_to: Option<RecipeId>,
    /// The row clicked while "Discard changes?" is open (E-10). Keep Editing
    /// clears it.
    clicked_row: Option<RecipeId>,
}

impl EditorPane {
    /// Where Discard goes: the clicked row wins, then `return_to`, then nothing.
    fn discard_destination(&mut self) -> Option<RecipeId> {
        self.clicked_row.take().or_else(|| self.return_to.take())
    }
}

enum Pane {
    Empty,
    Detail(RecipeDetail),
    Create(EditorPane),
    /// `saved` is the edited recipe as stored. The row and the heading show
    /// it until Save (MAC-06/Q1).
    Edit {
        saved: Recipe,
        pane: EditorPane,
    },
}

impl Pane {
    fn editor_pane(&mut self) -> Option<&mut EditorPane> {
        match self {
            Self::Create(pane) | Self::Edit { pane, .. } => Some(pane),
            Self::Empty | Self::Detail(_) => None,
        }
    }

    fn view(&self) -> RecipePaneView {
        match self {
            Self::Empty => RecipePaneView::Empty,
            Self::Detail(detail) => RecipePaneView::Detail(detail.view()),
            Self::Create(pane) => {
                let editor = pane.editor.view();
                RecipePaneView::Create {
                    draft_row: editor.title.trim().to_owned(),
                    editor,
                }
            }
            Self::Edit { saved, pane } => RecipePaneView::Edit {
                editor: pane.editor.view(),
                saved_title: saved.title.clone(),
            },
        }
    }
}

pub struct RecipeSection {
    caps: RecipeCapabilities,
    list: RecipeList,
    pane: ScopedChild<Pane>,
}

impl RecipeSection {
    pub fn new(caps: RecipeCapabilities) -> Self {
        Self {
            list: RecipeList::new(caps.clone()),
            caps,
            pane: ScopedChild::new(Pane::Empty),
        }
    }

    pub fn test_fixture() -> (Self, RecipeEffectStream) {
        let (caps, effects) = RecipeCapabilities::testing();
        (Self::new(caps), effects)
    }

    /// The delete question is open, or a delete runs or has finished. The
    /// read view keeps the pane until the delete ends (MAC-07).
    fn detail_is_locked(detail: &RecipeDetail) -> bool {
        match detail.view().status {
            RecipeDetailStatus::Ready { deletion, .. } => deletion != RecipeDeletion::Idle,
            RecipeDetailStatus::Loading
            | RecipeDetailStatus::Failed(_)
            | RecipeDetailStatus::Missing => false,
        }
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

    fn selected(&self) -> Option<RecipeId> {
        match &self.pane.widget {
            Pane::Detail(detail) => Some(detail.view().id),
            Pane::Edit { saved, .. } => Some(saved.id),
            Pane::Empty | Pane::Create(_) => None,
        }
    }

    fn render(&self) -> Command<RecipeSectionOutcome> {
        let render = self.caps.render.clone();
        Command::without_outcome(async move { render.render().await })
    }

    fn list_command(command: Command<RecipeListOutcome>) -> Command<RecipeSectionOutcome> {
        command.map(RecipeSectionOutcome::List)
    }

    fn pane_command(
        &mut self,
        command: Command<RecipePaneOutcome>,
    ) -> Command<RecipeSectionOutcome> {
        self.pane.command(command).map(RecipeSectionOutcome::Pane)
    }

    fn detail_command(
        &mut self,
        command: Command<RecipeDetailOutcome>,
    ) -> Command<RecipeSectionOutcome> {
        self.pane_command(command.map(RecipePaneOutcome::Detail))
    }

    fn editor_command(
        &mut self,
        command: Command<RecipeEditorOutcome>,
    ) -> Command<RecipeSectionOutcome> {
        self.pane_command(command.map(RecipePaneOutcome::Editor))
    }

    /// Replaces the pane in a new scope, which cancels the old pane's
    /// commands. A read view renders `Loading` and loads its recipe; the
    /// other panes start nothing, so only a render runs.
    fn show(&mut self, pane: Pane) -> Command<RecipeSectionOutcome> {
        self.pane.replace(pane);
        match &mut self.pane.widget {
            Pane::Detail(detail) => {
                let init = detail.init();
                self.detail_command(init)
            }
            Pane::Empty | Pane::Create(_) | Pane::Edit { .. } => self.render(),
        }
    }

    /// The read view of `id`, or the empty pane.
    fn show_recipe(&mut self, id: Option<RecipeId>) -> Command<RecipeSectionOutcome> {
        let pane = match id {
            Some(id) => Pane::Detail(RecipeDetail::new(self.caps.clone(), id)),
            None => Pane::Empty,
        };
        self.show(pane)
    }

    fn new_recipe(&mut self) -> Command<RecipeSectionOutcome> {
        let return_to = match &self.pane.widget {
            Pane::Empty => None,
            Pane::Detail(detail) if !Self::detail_is_locked(detail) => Some(detail.view().id),
            Pane::Detail(_) | Pane::Create(_) | Pane::Edit { .. } => return Command::empty(),
        };
        let editor = RecipeEditor::new(self.caps.clone());
        self.show(Pane::Create(EditorPane {
            editor,
            return_to,
            clicked_row: None,
        }))
    }

    /// Edit in the read view (REC-04). The read view must still show that
    /// recipe, with no delete asked or running.
    fn open_edit(&mut self, recipe: Recipe) -> Command<RecipeSectionOutcome> {
        let Pane::Detail(detail) = &self.pane.widget else {
            return Command::empty();
        };
        if detail.view().id != recipe.id || Self::detail_is_locked(detail) {
            return Command::empty();
        }
        let editor = RecipeEditor::edit(self.caps.clone(), &recipe);
        self.show(Pane::Edit {
            pane: EditorPane {
                editor,
                return_to: Some(recipe.id),
                clicked_row: None,
            },
            saved: recipe,
        })
    }

    /// A row click. A changed editor asks "Discard changes?" first (E-10,
    /// REC-03/Q2).
    fn select(&mut self, id: RecipeId) -> Command<RecipeSectionOutcome> {
        if !self.list.contains(id) {
            return Command::empty();
        }
        match &mut self.pane.widget {
            Pane::Empty => {}
            Pane::Detail(detail) => {
                if detail.view().id == id || Self::detail_is_locked(detail) {
                    return Command::empty();
                }
            }
            Pane::Edit { saved, .. } if saved.id == id => return Command::empty(),
            Pane::Create(pane) | Pane::Edit { pane, .. } => {
                if !matches!(
                    pane.editor.view().status,
                    RecipeEditorStatus::Editing { .. }
                ) {
                    return Command::empty();
                }
                if pane.editor.is_dirty() {
                    pane.clicked_row = Some(id);
                    let command = pane
                        .editor
                        .update(Message::Event(RecipeEditorEvent::RequestClose));
                    return self.editor_command(command);
                }
            }
        }
        self.show_recipe(Some(id))
    }

    fn clear_selection(&mut self, scope: CommandScopeId) -> Command<RecipeSectionOutcome> {
        let Pane::Detail(detail) = &self.pane.widget else {
            return Command::empty();
        };
        if scope != self.pane.scope() || Self::detail_is_locked(detail) {
            return Command::empty();
        }
        self.show(Pane::Empty)
    }

    fn handle_detail_outcome(
        &mut self,
        outcome: RecipeDetailOutcome,
    ) -> Command<RecipeSectionOutcome> {
        let Pane::Detail(detail) = &mut self.pane.widget else {
            return Command::empty();
        };
        match outcome {
            RecipeDetailOutcome::OpenEdit { recipe } => return self.open_edit(recipe),
            RecipeDetailOutcome::Deleted { id } => {
                let view = detail.view();
                let deleted = matches!(
                    view.status,
                    RecipeDetailStatus::Ready {
                        deletion: RecipeDeletion::Deleted,
                        ..
                    }
                );
                if !deleted || view.id != id {
                    return Command::empty();
                }
                let reload = Self::list_command(self.list.reload());
                let show = self.show(Pane::Empty);
                return Command::select_all(vec![show, reload]);
            }
            RecipeDetailOutcome::LoadFinished(_)
            | RecipeDetailOutcome::ConfirmationDecided(_)
            | RecipeDetailOutcome::DeleteFinished(_) => {}
        }
        let was_missing = detail.view().status == RecipeDetailStatus::Missing;
        let command = detail.update(Message::Outcome(outcome));
        let missing = detail.view().status == RecipeDetailStatus::Missing;
        let command = self.detail_command(command);
        if missing && !was_missing {
            // The list still shows the recipe that no longer exists.
            let reload = Self::list_command(self.list.reload());
            return Command::select_all(vec![command, reload]);
        }
        command
    }

    fn handle_editor_outcome(
        &mut self,
        outcome: RecipeEditorOutcome,
    ) -> Command<RecipeSectionOutcome> {
        let Some(pane) = self.pane.widget.editor_pane() else {
            return Command::empty();
        };
        match outcome {
            // The editor reports Saved and Discarded only once it is finished.
            RecipeEditorOutcome::Saved { .. } | RecipeEditorOutcome::Discarded
                if pane.editor.view().status != RecipeEditorStatus::Finished =>
            {
                Command::empty()
            }
            RecipeEditorOutcome::Saved { recipe } => {
                let reload = Self::list_command(self.list.reload());
                let show = self.show_recipe(Some(recipe.id));
                Command::select_all(vec![show, reload])
            }
            RecipeEditorOutcome::Discarded => {
                let next = pane.discard_destination();
                self.show_recipe(next)
            }
            outcome @ (RecipeEditorOutcome::ConfirmationDecided(_)
            | RecipeEditorOutcome::SaveFinished(_)) => {
                let command = pane.editor.update(Message::Outcome(outcome));
                // Keep Editing (or a failed save) forgets the clicked row.
                if matches!(
                    pane.editor.view().status,
                    RecipeEditorStatus::Editing { .. }
                ) {
                    pane.clicked_row = None;
                }
                self.editor_command(command)
            }
        }
    }
}

impl Widget for RecipeSection {
    type View = RecipeSectionView;
    type Event = RecipeSectionEvent;
    type Outcome = RecipeSectionOutcome;

    fn init(&mut self) -> Command<Self::Outcome> {
        let command = self.list.init();
        Self::list_command(command)
    }

    fn update(&mut self, message: Message<Self>) -> Command<Self::Outcome> {
        match message {
            Message::Event(RecipeSectionEvent::List(event)) => {
                let command = self.list.update(Message::Event(event));
                Self::list_command(command)
            }
            Message::Event(RecipeSectionEvent::NewRecipe) => self.new_recipe(),
            Message::Event(RecipeSectionEvent::Select(id)) => self.select(id),
            Message::Event(RecipeSectionEvent::ClearSelection { scope }) => {
                self.clear_selection(scope)
            }
            Message::Event(RecipeSectionEvent::Detail(scoped)) => {
                let Some(event) = self.pane.accept(scoped) else {
                    return Command::empty();
                };
                let Pane::Detail(detail) = &mut self.pane.widget else {
                    return Command::empty();
                };
                let command = detail.update(Message::Event(event));
                self.detail_command(command)
            }
            Message::Event(RecipeSectionEvent::Editor(scoped)) => {
                let Some(event) = self.pane.accept(scoped) else {
                    return Command::empty();
                };
                let Some(pane) = self.pane.widget.editor_pane() else {
                    return Command::empty();
                };
                let command = pane.editor.update(Message::Event(event));
                self.editor_command(command)
            }
            Message::Outcome(RecipeSectionOutcome::List(outcome)) => {
                let command = self.list.update(Message::Outcome(outcome));
                Self::list_command(command)
            }
            Message::Outcome(RecipeSectionOutcome::Pane(scoped)) => {
                match self.pane.accept(scoped) {
                    Some(RecipePaneOutcome::Detail(outcome)) => self.handle_detail_outcome(outcome),
                    Some(RecipePaneOutcome::Editor(outcome)) => self.handle_editor_outcome(outcome),
                    None => Command::empty(),
                }
            }
        }
    }

    fn view(&self) -> Self::View {
        RecipeSectionView {
            list: self.list.view(),
            selected: self.selected(),
            pane: self.pane.view(Pane::view),
            create: self.create(),
        }
    }
}
