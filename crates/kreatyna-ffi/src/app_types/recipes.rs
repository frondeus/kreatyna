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
}

impl From<RecipeListEvent> for core::RecipeListEvent {
    fn from(value: RecipeListEvent) -> Self {
        match value {
            RecipeListEvent::Refresh => Self::Refresh,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct RecipeListRow {
    pub id: String,
    pub title: String,
}

impl From<core::RecipeListRow> for RecipeListRow {
    fn from(value: core::RecipeListRow) -> Self {
        Self {
            id: value.id.to_string(),
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
    /// Rows sorted by title. Empty means "No recipes yet".
    Ready {
        rows: Vec<RecipeListRow>,
    },
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
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct RecipeListView {
    pub status: RecipeListStatus,
}

impl From<core::RecipeListView> for RecipeListView {
    fn from(value: core::RecipeListView) -> Self {
        Self {
            status: value.status.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum RecipeSaveError {
    DuplicateTitle,
    Storage,
    Transport,
}

impl From<core::capabilities::RecipeSaveError> for RecipeSaveError {
    fn from(value: core::capabilities::RecipeSaveError) -> Self {
        match value {
            core::capabilities::RecipeSaveError::DuplicateTitle => Self::DuplicateTitle,
            core::capabilities::RecipeSaveError::Storage => Self::Storage,
            core::capabilities::RecipeSaveError::Transport => Self::Transport,
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
pub enum RecipePaneView {
    /// "Select a recipe".
    Empty,
    Create {
        editor: RecipeEditorView,
        /// The typed title, trimmed, for the draft row at the top of the
        /// list. Empty shows "New Recipe".
        draft_row: String,
    },
}

impl From<core::RecipePaneView> for RecipePaneView {
    fn from(value: core::RecipePaneView) -> Self {
        match value {
            core::RecipePaneView::Empty => Self::Empty,
            core::RecipePaneView::Create { editor, draft_row } => Self::Create {
                editor: editor.into(),
                draft_row,
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
    pub pane: ScopedRecipePaneView,
    /// Whether New Recipe works now.
    pub can_create: bool,
}

impl From<core::RecipeSectionView> for RecipeSectionView {
    fn from(value: core::RecipeSectionView) -> Self {
        Self {
            list: value.list.into(),
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
            RecipeSectionEvent::Editor { scope, event } => {
                Self::Editor(Scoped::new(scope.into(), event.into()))
            }
        }
    }
}
