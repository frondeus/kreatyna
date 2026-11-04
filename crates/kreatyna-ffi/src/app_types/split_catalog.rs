use kreatyna::ingredients as core;
use krucyfiks::Scoped;

use super::{
    AppTypeConversionError, IngredientDetailEvent, IngredientDetailView, IngredientFormEvent,
    IngredientFormView, IngredientListEvent, IngredientListView, ScopeId,
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
    Form {
        scope: ScopeId,
        event: IngredientFormEvent,
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
                Self::List(Scoped::new(scope.into(), event.into()))
            }
            IngredientSplitCatalogEvent::Detail { scope, event } => {
                Self::Detail(Scoped::new(scope.into(), event.into()))
            }
            IngredientSplitCatalogEvent::Form { scope, event } => {
                Self::Form(Scoped::new(scope.into(), event.try_into()?))
            }
            IngredientSplitCatalogEvent::ClearSelection { scope } => Self::ClearSelection {
                scope: scope.into(),
            },
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum IngredientSplitPaneView {
    Empty,
    Detail {
        detail: IngredientDetailView,
    },
    Create {
        form: IngredientFormView,
    },
    /// The edit form. The pane title shows `saved_name`, the name as stored,
    /// until Save.
    Edit {
        saved_name: String,
        form: IngredientFormView,
    },
}

impl TryFrom<core::IngredientSplitPaneView> for IngredientSplitPaneView {
    type Error = AppTypeConversionError;

    fn try_from(value: core::IngredientSplitPaneView) -> Result<Self, Self::Error> {
        Ok(match value {
            core::IngredientSplitPaneView::Empty => Self::Empty,
            core::IngredientSplitPaneView::Detail(detail) => Self::Detail {
                detail: detail.into(),
            },
            core::IngredientSplitPaneView::Create(form) => Self::Create {
                form: form.try_into()?,
            },
            core::IngredientSplitPaneView::Edit { saved_name, form } => Self::Edit {
                saved_name,
                form: form.try_into()?,
            },
        })
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

impl TryFrom<Scoped<core::IngredientSplitPaneView>> for ScopedIngredientSplitPaneView {
    type Error = AppTypeConversionError;

    fn try_from(value: Scoped<core::IngredientSplitPaneView>) -> Result<Self, Self::Error> {
        Ok(Self {
            scope: value.scope.into(),
            value: value.value.try_into()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct IngredientSplitCatalogView {
    pub list: ScopedIngredientListView,
    /// The highlighted row's ingredient ID.
    pub selected: Option<core::IngredientId>,
    pub pane: ScopedIngredientSplitPaneView,
    /// Whether **＋** opens the create form now.
    pub can_create: bool,
    /// The draft row's text while the create form is open: the typed name,
    /// trimmed. Empty until a name is typed.
    pub draft_row: Option<String>,
}

impl TryFrom<core::IngredientSplitCatalogView> for IngredientSplitCatalogView {
    type Error = AppTypeConversionError;

    fn try_from(value: core::IngredientSplitCatalogView) -> Result<Self, Self::Error> {
        Ok(Self {
            list: value.list.try_into()?,
            selected: value.selected,
            pane: value.pane.try_into()?,
            can_create: value.can_create,
            draft_row: value.draft_row,
        })
    }
}
