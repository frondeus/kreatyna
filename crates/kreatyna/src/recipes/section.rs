//! The Recipes section of the split layout: the recipe list and the detail
//! pane beside it.
//!
//! The list lives as long as the section. The pane gets a new scope each time
//! its content changes, so late events and results of an old editor are
//! ignored (E-19). In REC-02 the pane is empty or shows the create editor;
//! REC-03 adds the read view.
use krucyfiks::{Command, CommandExt, Message, Scoped, Widget};

use crate::scoped_child::ScopedChild;

use super::{
    RecipeEditor, RecipeEditorEvent, RecipeEditorOutcome, RecipeEditorStatus, RecipeEditorView,
    RecipeList, RecipeListEvent, RecipeListOutcome, RecipeListView,
    capabilities::{RecipeCapabilities, RecipeEffectStream},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecipeSectionEvent {
    List(RecipeListEvent),
    /// New Recipe or ⌘N. Ignored while the pane is not empty.
    NewRecipe,
    /// The scope is the pane's.
    Editor(Scoped<RecipeEditorEvent>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecipeSectionOutcome {
    List(RecipeListOutcome),
    Pane(Scoped<RecipeEditorOutcome>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecipePaneView {
    /// "Select a recipe". REC-03 adds the read view.
    Empty,
    Create {
        editor: RecipeEditorView,
        /// The typed title, trimmed, for the draft row at the top of the
        /// list. Empty shows "New Recipe" (REC-02/Q1).
        draft_row: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecipeSectionView {
    pub list: RecipeListView,
    /// The scope goes with editor events.
    pub pane: Scoped<RecipePaneView>,
    /// Whether New Recipe works now. Rust decides: REC-03 and REC-05 add
    /// panes where it does and where it does not.
    pub can_create: bool,
}

enum Pane {
    Empty,
    Create(RecipeEditor),
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

    fn can_create(&self) -> bool {
        match self.pane.widget {
            Pane::Empty => true,
            Pane::Create(_) => false,
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
        command: Command<RecipeEditorOutcome>,
    ) -> Command<RecipeSectionOutcome> {
        self.pane.command(command).map(RecipeSectionOutcome::Pane)
    }

    /// Replaces the pane in a new scope, which cancels the old pane's
    /// commands. An editor starts nothing, so only a render runs.
    fn show(&mut self, pane: Pane) -> Command<RecipeSectionOutcome> {
        self.pane.replace(pane);
        self.render()
    }

    fn new_recipe(&mut self) -> Command<RecipeSectionOutcome> {
        if !self.can_create() {
            return Command::empty();
        }
        let editor = RecipeEditor::new(self.caps.clone());
        self.show(Pane::Create(editor))
    }

    fn handle_editor_outcome(
        &mut self,
        outcome: RecipeEditorOutcome,
    ) -> Command<RecipeSectionOutcome> {
        let Pane::Create(editor) = &mut self.pane.widget else {
            return Command::empty();
        };
        match outcome {
            // The editor reports Saved and Discarded only once it is finished.
            RecipeEditorOutcome::Saved { .. } | RecipeEditorOutcome::Discarded
                if editor.view().status != RecipeEditorStatus::Finished =>
            {
                Command::empty()
            }
            // REC-03 opens the saved recipe's read view here.
            RecipeEditorOutcome::Saved { recipe: _ } => {
                let reload = self.list.reload();
                let reload = Self::list_command(reload);
                let show = self.show(Pane::Empty);
                Command::select_all(vec![show, reload])
            }
            RecipeEditorOutcome::Discarded => self.show(Pane::Empty),
            outcome @ (RecipeEditorOutcome::ConfirmationDecided(_)
            | RecipeEditorOutcome::SaveFinished(_)) => {
                let command = editor.update(Message::Outcome(outcome));
                self.pane_command(command)
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
            Message::Event(RecipeSectionEvent::Editor(scoped)) => {
                let Some(event) = self.pane.accept(scoped) else {
                    return Command::empty();
                };
                let Pane::Create(editor) = &mut self.pane.widget else {
                    return Command::empty();
                };
                let command = editor.update(Message::Event(event));
                self.pane_command(command)
            }
            Message::Outcome(RecipeSectionOutcome::List(outcome)) => {
                let command = self.list.update(Message::Outcome(outcome));
                Self::list_command(command)
            }
            Message::Outcome(RecipeSectionOutcome::Pane(scoped)) => {
                match self.pane.accept(scoped) {
                    Some(outcome) => self.handle_editor_outcome(outcome),
                    None => Command::empty(),
                }
            }
        }
    }

    fn view(&self) -> Self::View {
        RecipeSectionView {
            list: self.list.view(),
            pane: self.pane.view(|pane| match pane {
                Pane::Empty => RecipePaneView::Empty,
                Pane::Create(editor) => {
                    let editor = editor.view();
                    RecipePaneView::Create {
                        draft_row: editor.title.trim().to_owned(),
                        editor,
                    }
                }
            }),
            can_create: self.can_create(),
        }
    }
}
