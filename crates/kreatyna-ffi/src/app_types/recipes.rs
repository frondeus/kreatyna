use kreatyna::recipes as core;

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
