use kreatyna::ingredients as core;
use krucyfiks::Scoped;

use super::{
    AppTypeConversionError, IngredientDetailEvent, IngredientDetailView, IngredientListEvent,
    IngredientListView, ScopeId,
};

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum IngredientSplitCatalogEvent {
    List {
        scope: ScopeId,
        event: IngredientListEvent,
    },
    Detail {
        scope: ScopeId,
        event: IngredientDetailEvent,
    },
    ClearSelection {
        scope: ScopeId,
    },
}

impl TryFrom<IngredientSplitCatalogEvent> for core::IngredientSplitCatalogEvent {
    type Error = AppTypeConversionError;

    fn try_from(value: IngredientSplitCatalogEvent) -> Result<Self, Self::Error> {
        Ok(match value {
            IngredientSplitCatalogEvent::List { scope, event } => {
                Self::List(Scoped::new(scope.into(), event.try_into()?))
            }
            IngredientSplitCatalogEvent::Detail { scope, event } => {
                Self::Detail(Scoped::new(scope.into(), event.into()))
            }
            IngredientSplitCatalogEvent::ClearSelection { scope } => Self::ClearSelection {
                scope: scope.into(),
            },
        })
    }
}

// UniFFI exports this as a Swift enum; boxing would not shrink the generated
// value (see `IngredientCatalogScreenView`).
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum IngredientSplitPaneView {
    Empty,
    Detail { detail: IngredientDetailView },
}

impl From<core::IngredientSplitPaneView> for IngredientSplitPaneView {
    fn from(value: core::IngredientSplitPaneView) -> Self {
        match value {
            core::IngredientSplitPaneView::Empty => Self::Empty,
            core::IngredientSplitPaneView::Detail(detail) => Self::Detail {
                detail: detail.into(),
            },
        }
    }
}

/// The list view and the scope that its events must be sent with.
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct ScopedIngredientListView {
    pub scope: ScopeId,
    pub value: IngredientListView,
}

impl TryFrom<Scoped<core::IngredientListView>> for ScopedIngredientListView {
    type Error = AppTypeConversionError;

    fn try_from(value: Scoped<core::IngredientListView>) -> Result<Self, Self::Error> {
        Ok(Self {
            scope: value.scope.into(),
            value: value.value.try_into()?,
        })
    }
}

/// The pane view and the scope that its events must be sent with.
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct ScopedIngredientSplitPaneView {
    pub scope: ScopeId,
    pub value: IngredientSplitPaneView,
}

impl From<Scoped<core::IngredientSplitPaneView>> for ScopedIngredientSplitPaneView {
    fn from(value: Scoped<core::IngredientSplitPaneView>) -> Self {
        Self {
            scope: value.scope.into(),
            value: value.value.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct IngredientSplitCatalogView {
    pub list: ScopedIngredientListView,
    /// The highlighted row's ingredient ID.
    pub selected: Option<String>,
    pub pane: ScopedIngredientSplitPaneView,
}

impl TryFrom<core::IngredientSplitCatalogView> for IngredientSplitCatalogView {
    type Error = AppTypeConversionError;

    fn try_from(value: core::IngredientSplitCatalogView) -> Result<Self, Self::Error> {
        Ok(Self {
            list: value.list.try_into()?,
            selected: value.selected.map(|id| id.to_string()),
            pane: value.pane.into(),
        })
    }
}
