use kreatyna::ingredients as core;

use super::support::{count, ingredient_id};
use super::{AppTypeConversionError, IngredientLoadError};

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum IngredientListEvent {
    Search { query: String },
    Refresh,
    CreateRequested,
    Select { id: String },
    Edit { id: String },
}

impl TryFrom<IngredientListEvent> for core::IngredientListEvent {
    type Error = AppTypeConversionError;

    fn try_from(value: IngredientListEvent) -> Result<Self, Self::Error> {
        Ok(match value {
            IngredientListEvent::Search { query } => Self::Search(query),
            IngredientListEvent::Refresh => Self::Refresh,
            IngredientListEvent::CreateRequested => Self::CreateRequested,
            IngredientListEvent::Select { id } => Self::Select(ingredient_id(id)?),
            IngredientListEvent::Edit { id } => Self::Edit(ingredient_id(id)?),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct IngredientListRow {
    pub id: String,
    pub name: String,
}

impl From<core::IngredientListRow> for IngredientListRow {
    fn from(value: core::IngredientListRow) -> Self {
        Self {
            id: value.id.to_string(),
            name: value.name,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum IngredientListRefresh {
    Idle,
    Loading,
    Failed { error: IngredientLoadError },
}

impl From<core::IngredientListRefresh> for IngredientListRefresh {
    fn from(value: core::IngredientListRefresh) -> Self {
        match value {
            core::IngredientListRefresh::Idle => Self::Idle,
            core::IngredientListRefresh::Loading => Self::Loading,
            core::IngredientListRefresh::Failed(error) => Self::Failed {
                error: error.into(),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum IngredientListStatus {
    Loading,
    Failed {
        error: IngredientLoadError,
    },
    Ready {
        rows: Vec<IngredientListRow>,
        total_count: u64,
        refresh: IngredientListRefresh,
    },
}

impl TryFrom<core::IngredientListStatus> for IngredientListStatus {
    type Error = AppTypeConversionError;

    fn try_from(value: core::IngredientListStatus) -> Result<Self, Self::Error> {
        Ok(match value {
            core::IngredientListStatus::Loading => Self::Loading,
            core::IngredientListStatus::Failed(error) => Self::Failed {
                error: error.into(),
            },
            core::IngredientListStatus::Ready {
                rows,
                total_count,
                refresh,
            } => Self::Ready {
                rows: rows.into_iter().map(Into::into).collect(),
                total_count: count(total_count)?,
                refresh: refresh.into(),
            },
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct IngredientListView {
    pub search: String,
    pub status: IngredientListStatus,
}

impl TryFrom<core::IngredientListView> for IngredientListView {
    type Error = AppTypeConversionError;

    fn try_from(value: core::IngredientListView) -> Result<Self, Self::Error> {
        Ok(Self {
            search: value.search,
            status: value.status.try_into()?,
        })
    }
}
