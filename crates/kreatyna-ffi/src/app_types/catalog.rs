use kreatyna::ingredients as core;
use krucyfiks::Scoped;

use super::{
    AppTypeConversionError, IngredientDetailEvent, IngredientDetailView, IngredientFormEvent,
    IngredientFormView, IngredientListEvent, IngredientListView, ScopeId,
};

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum IngredientCatalogEvent {
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
    Back {
        scope: ScopeId,
    },
}

impl TryFrom<IngredientCatalogEvent> for core::IngredientCatalogEvent {
    type Error = AppTypeConversionError;

    fn try_from(value: IngredientCatalogEvent) -> Result<Self, Self::Error> {
        Ok(match value {
            IngredientCatalogEvent::List { scope, event } => {
                Self::List(Scoped::new(scope.into(), event.into()))
            }
            IngredientCatalogEvent::Detail { scope, event } => {
                Self::Detail(Scoped::new(scope.into(), event.into()))
            }
            IngredientCatalogEvent::Form { scope, event } => {
                Self::Form(Scoped::new(scope.into(), event.try_into()?))
            }
            IngredientCatalogEvent::Back { scope } => Self::Back {
                scope: scope.into(),
            },
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct IngredientCatalogView {
    pub navigation: IngredientNavigation,
    pub list: IngredientListView,
    pub scope: ScopeId,
    pub screen: IngredientCatalogScreenView,
}

impl TryFrom<core::IngredientCatalogView> for IngredientCatalogView {
    type Error = AppTypeConversionError;

    fn try_from(value: core::IngredientCatalogView) -> Result<Self, Self::Error> {
        Ok(Self {
            navigation: value.navigation.into(),
            list: value.list.try_into()?,
            scope: value.scope.into(),
            screen: value.screen.try_into()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum IngredientCatalogScreenView {
    List,
    Detail {
        detail: IngredientDetailView,
    },
    Create {
        form: IngredientFormView,
    },
    EditFromList {
        form: IngredientFormView,
    },
    EditFromDetail {
        detail: IngredientDetailView,
        form: IngredientFormView,
    },
}

impl TryFrom<core::IngredientCatalogScreenView> for IngredientCatalogScreenView {
    type Error = AppTypeConversionError;

    fn try_from(value: core::IngredientCatalogScreenView) -> Result<Self, Self::Error> {
        Ok(match value {
            core::IngredientCatalogScreenView::List => Self::List,
            core::IngredientCatalogScreenView::Detail(detail) => Self::Detail {
                detail: detail.into(),
            },
            core::IngredientCatalogScreenView::Create(form) => Self::Create {
                form: form.try_into()?,
            },
            core::IngredientCatalogScreenView::EditFromList(form) => Self::EditFromList {
                form: form.try_into()?,
            },
            core::IngredientCatalogScreenView::EditFromDetail { detail, form } => {
                Self::EditFromDetail {
                    detail: detail.into(),
                    form: form.try_into()?,
                }
            }
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum IngredientEditOrigin {
    List,
    Detail,
}

impl From<core::IngredientEditOrigin> for IngredientEditOrigin {
    fn from(value: core::IngredientEditOrigin) -> Self {
        match value {
            core::IngredientEditOrigin::List => Self::List,
            core::IngredientEditOrigin::Detail => Self::Detail,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum IngredientNavigation {
    List,
    Detail {
        id: core::IngredientId,
    },
    Create,
    Edit {
        id: core::IngredientId,
        origin: IngredientEditOrigin,
    },
}

impl From<core::IngredientNavigation> for IngredientNavigation {
    fn from(value: core::IngredientNavigation) -> Self {
        match value {
            core::IngredientNavigation::List => Self::List,
            core::IngredientNavigation::Detail { id } => Self::Detail { id },
            core::IngredientNavigation::Create => Self::Create,
            core::IngredientNavigation::Edit { id, origin } => Self::Edit {
                id,
                origin: origin.into(),
            },
        }
    }
}
