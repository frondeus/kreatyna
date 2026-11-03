use kreatyna::recipes as core;
use krucyfiks::Scoped;

use super::{ConfirmationEvent, ConfirmationView, ScopeId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum RecipeLoadError {
    Storage,
    Transport,
}

impl From<core::capabilities::RecipeLoadError> for RecipeLoadError {
    fn from(value: core::capabilities::RecipeLoadError) -> Self {
        match value {
            core::capabilities::RecipeLoadError::Storage => Self::Storage,
            core::capabilities::RecipeLoadError::Transport => Self::Transport,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum RecipeListEvent {
    /// Load again after a failure (the Retry button).
    Refresh,
    /// The search text as typed. Rust filters the rows by its key.
    Search { query: String },
}

impl From<RecipeListEvent> for core::RecipeListEvent {
    fn from(value: RecipeListEvent) -> Self {
        match value {
            RecipeListEvent::Refresh => Self::Refresh,
            RecipeListEvent::Search { query } => Self::Search(query),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct RecipeListRow {
    pub id: core::RecipeId,
    pub title: String,
}

impl From<core::RecipeListRow> for RecipeListRow {
    fn from(value: core::RecipeListRow) -> Self {
        Self {
            id: value.id,
            title: value.title,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum RecipeListStatus {
    Loading,
    Failed {
        error: RecipeLoadError,
    },
    /// The recipes that match the search, sorted by title. Empty only when
    /// no recipe is saved: "No recipes yet".
    Ready {
        rows: Vec<RecipeListRow>,
    },
    /// Recipes are saved, but none matches the search: "No matches".
    NoMatches,
}

impl From<core::RecipeListStatus> for RecipeListStatus {
    fn from(value: core::RecipeListStatus) -> Self {
        match value {
            core::RecipeListStatus::Loading => Self::Loading,
            core::RecipeListStatus::Failed(error) => Self::Failed {
                error: error.into(),
            },
            core::RecipeListStatus::Ready { rows } => Self::Ready {
                rows: rows.into_iter().map(Into::into).collect(),
            },
            core::RecipeListStatus::NoMatches => Self::NoMatches,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct RecipeListView {
    /// The search text as typed.
    pub search: String,
    pub status: RecipeListStatus,
}

impl From<core::RecipeListView> for RecipeListView {
    fn from(value: core::RecipeListView) -> Self {
        Self {
            search: value.search,
            status: value.status.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum RecipeSaveError {
    DuplicateTitle,
    NotFound,
    Storage,
    Transport,
}

impl From<core::capabilities::RecipeSaveError> for RecipeSaveError {
    fn from(value: core::capabilities::RecipeSaveError) -> Self {
        match value {
            core::capabilities::RecipeSaveError::DuplicateTitle => Self::DuplicateTitle,
            core::capabilities::RecipeSaveError::NotFound => Self::NotFound,
            core::capabilities::RecipeSaveError::Storage => Self::Storage,
            core::capabilities::RecipeSaveError::Transport => Self::Transport,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum RecipeDeleteError {
    Storage,
    Transport,
}

impl From<core::capabilities::RecipeDeleteError> for RecipeDeleteError {
    fn from(value: core::capabilities::RecipeDeleteError) -> Self {
        match value {
            core::capabilities::RecipeDeleteError::Storage => Self::Storage,
            core::capabilities::RecipeDeleteError::Transport => Self::Transport,
        }
    }
}

/// The save rule a title breaks (nutype's error in Rust).
#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum RecipeTitleError {
    NotEmptyViolated,
}

impl From<core::RecipeTitleError> for RecipeTitleError {
    fn from(value: core::RecipeTitleError) -> Self {
        match value {
            core::RecipeTitleError::NotEmptyViolated => Self::NotEmptyViolated,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum RecipeEditorProblem {
    Invalid { error: RecipeTitleError },
    SaveFailed { error: RecipeSaveError },
}

impl From<core::RecipeEditorProblem> for RecipeEditorProblem {
    fn from(value: core::RecipeEditorProblem) -> Self {
        match value {
            core::RecipeEditorProblem::Invalid(error) => Self::Invalid {
                error: error.into(),
            },
            core::RecipeEditorProblem::SaveFailed(error) => Self::SaveFailed {
                error: error.into(),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum RecipeEditorStatus {
    Editing {
        problem: Option<RecipeEditorProblem>,
    },
    ConfirmingDiscard {
        confirmation: ConfirmationView,
    },
    Saving,
    Finished,
}

impl From<core::RecipeEditorStatus> for RecipeEditorStatus {
    fn from(value: core::RecipeEditorStatus) -> Self {
        match value {
            core::RecipeEditorStatus::Editing { problem } => Self::Editing {
                problem: problem.map(Into::into),
            },
            core::RecipeEditorStatus::ConfirmingDiscard(confirmation) => Self::ConfirmingDiscard {
                confirmation: confirmation.into(),
            },
            core::RecipeEditorStatus::Saving => Self::Saving,
            core::RecipeEditorStatus::Finished => Self::Finished,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct RecipeEditorView {
    /// The title as typed.
    pub title: String,
    pub status: RecipeEditorStatus,
}

impl From<core::RecipeEditorView> for RecipeEditorView {
    fn from(value: core::RecipeEditorView) -> Self {
        Self {
            title: value.title,
            status: value.status.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum RecipeEditorEvent {
    SetTitle {
        title: String,
    },
    Save,
    /// Cancel. A changed title asks "Discard changes?" first.
    RequestClose,
    Confirmation {
        event: ConfirmationEvent,
    },
}

impl From<RecipeEditorEvent> for core::RecipeEditorEvent {
    fn from(value: RecipeEditorEvent) -> Self {
        match value {
            RecipeEditorEvent::SetTitle { title } => Self::SetTitle(title),
            RecipeEditorEvent::Save => Self::Save,
            RecipeEditorEvent::RequestClose => Self::RequestClose,
            RecipeEditorEvent::Confirmation { event } => Self::Confirmation(event.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum RecipeDetailEvent {
    /// Load again after a failure (the Retry button).
    Retry,
    /// The Edit button: open the editor for the shown recipe.
    Edit,
    /// The Delete button: ask "Delete …?".
    RequestDelete,
    /// Delete or Cancel in the question.
    Confirmation { event: ConfirmationEvent },
}

impl From<RecipeDetailEvent> for core::RecipeDetailEvent {
    fn from(value: RecipeDetailEvent) -> Self {
        match value {
            RecipeDetailEvent::Retry => Self::Retry,
            RecipeDetailEvent::Edit => Self::Edit,
            RecipeDetailEvent::RequestDelete => Self::RequestDelete,
            RecipeDetailEvent::Confirmation { event } => Self::Confirmation(event.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum RecipeDetailStatus {
    Loading,
    Failed {
        error: RecipeLoadError,
    },
    /// No recipe has this id any more.
    Missing,
    Ready {
        title: String,
        deletion: RecipeDeletion,
    },
}

/// The delete of the shown recipe. Swift shows the question while
/// `Confirming` waits, and "Deleting…" while `Deleting` or `Deleted`.
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum RecipeDeletion {
    Idle,
    Confirming {
        confirmation: ConfirmationView,
        /// The last failed delete, if any.
        error: Option<RecipeDeleteError>,
    },
    Deleting,
    Deleted,
}

impl From<core::RecipeDeletion> for RecipeDeletion {
    fn from(value: core::RecipeDeletion) -> Self {
        match value {
            core::RecipeDeletion::Idle => Self::Idle,
            core::RecipeDeletion::Confirming {
                confirmation,
                error,
            } => Self::Confirming {
                confirmation: confirmation.into(),
                error: error.map(Into::into),
            },
            core::RecipeDeletion::Deleting => Self::Deleting,
            core::RecipeDeletion::Deleted => Self::Deleted,
        }
    }
}

impl From<core::RecipeDetailStatus> for RecipeDetailStatus {
    fn from(value: core::RecipeDetailStatus) -> Self {
        match value {
            core::RecipeDetailStatus::Loading => Self::Loading,
            core::RecipeDetailStatus::Failed(error) => Self::Failed {
                error: error.into(),
            },
            core::RecipeDetailStatus::Missing => Self::Missing,
            core::RecipeDetailStatus::Ready { title, deletion } => Self::Ready {
                title,
                deletion: deletion.into(),
            },
        }
    }
}

/// The read view of one recipe.
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct RecipeDetailView {
    pub id: core::RecipeId,
    pub status: RecipeDetailStatus,
}

impl From<core::RecipeDetailView> for RecipeDetailView {
    fn from(value: core::RecipeDetailView) -> Self {
        Self {
            id: value.id,
            status: value.status.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum RecipePaneView {
    /// "Select a recipe".
    Empty,
    /// The read view of the selected recipe.
    Detail { detail: RecipeDetailView },
    Create {
        editor: RecipeEditorView,
        /// The typed title, trimmed, for the draft row at the top of the
        /// list. Empty shows "New Recipe".
        draft_row: String,
    },
    /// The edit editor. `saved_title` is the title as stored: the heading
    /// shows it until Save.
    Edit {
        editor: RecipeEditorView,
        saved_title: String,
    },
}

impl From<core::RecipePaneView> for RecipePaneView {
    fn from(value: core::RecipePaneView) -> Self {
        match value {
            core::RecipePaneView::Empty => Self::Empty,
            core::RecipePaneView::Detail(detail) => Self::Detail {
                detail: detail.into(),
            },
            core::RecipePaneView::Create { editor, draft_row } => Self::Create {
                editor: editor.into(),
                draft_row,
            },
            core::RecipePaneView::Edit {
                editor,
                saved_title,
            } => Self::Edit {
                editor: editor.into(),
                saved_title,
            },
        }
    }
}

/// The pane view and the scope that its events must be sent with.
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct ScopedRecipePaneView {
    pub scope: ScopeId,
    pub value: RecipePaneView,
}

impl From<Scoped<core::RecipePaneView>> for ScopedRecipePaneView {
    fn from(value: Scoped<core::RecipePaneView>) -> Self {
        Self {
            scope: value.scope.into(),
            value: value.value.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct RecipeSectionView {
    pub list: RecipeListView,
    /// The highlighted row: the open recipe's id. `None` while the editor is
    /// open.
    pub selected: Option<core::RecipeId>,
    pub pane: ScopedRecipePaneView,
    /// Whether New Recipe works now.
    pub can_create: bool,
}

impl From<core::RecipeSectionView> for RecipeSectionView {
    fn from(value: core::RecipeSectionView) -> Self {
        Self {
            list: value.list.into(),
            selected: value.selected,
            pane: value.pane.into(),
            can_create: value.can_create,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum RecipeSectionEvent {
    List {
        event: RecipeListEvent,
    },
    /// New Recipe or ⌘N.
    NewRecipe,
    /// A row click.
    Select {
        id: core::RecipeId,
    },
    /// ⌘-click on the open recipe.
    ClearSelection {
        scope: ScopeId,
    },
    Detail {
        scope: ScopeId,
        event: RecipeDetailEvent,
    },
    Editor {
        scope: ScopeId,
        event: RecipeEditorEvent,
    },
}

impl From<RecipeSectionEvent> for core::RecipeSectionEvent {
    fn from(value: RecipeSectionEvent) -> Self {
        match value {
            RecipeSectionEvent::List { event } => Self::List(event.into()),
            RecipeSectionEvent::NewRecipe => Self::NewRecipe,
            RecipeSectionEvent::Select { id } => Self::Select(id),
            RecipeSectionEvent::ClearSelection { scope } => Self::ClearSelection {
                scope: scope.into(),
            },
            RecipeSectionEvent::Detail { scope, event } => {
                Self::Detail(Scoped::new(scope.into(), event.into()))
            }
            RecipeSectionEvent::Editor { scope, event } => {
                Self::Editor(Scoped::new(scope.into(), event.into()))
            }
        }
    }
}
