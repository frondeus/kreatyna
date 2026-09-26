use kreatyna as core;
use krucyfiks::Scoped;

use super::{
    AppTypeConversionError, CounterEvent, CounterView, IngredientCatalogEvent,
    IngredientCatalogView, ScopeId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum AppTab {
    Ingredients,
    Counter,
}

impl From<AppTab> for core::AppTab {
    fn from(value: AppTab) -> Self {
        match value {
            AppTab::Ingredients => Self::Ingredients,
            AppTab::Counter => Self::Counter,
        }
    }
}

impl From<core::AppTab> for AppTab {
    fn from(value: core::AppTab) -> Self {
        match value {
            core::AppTab::Ingredients => Self::Ingredients,
            core::AppTab::Counter => Self::Counter,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum AppEvent {
    SelectTab {
        tab: AppTab,
    },
    Ingredients {
        scope: ScopeId,
        event: IngredientCatalogEvent,
    },
    Counter {
        scope: ScopeId,
        event: CounterEvent,
    },
}

impl TryFrom<AppEvent> for core::AppEvent {
    type Error = AppTypeConversionError;

    fn try_from(value: AppEvent) -> Result<Self, Self::Error> {
        Ok(match value {
            AppEvent::SelectTab { tab } => Self::SelectTab(tab.into()),
            AppEvent::Ingredients { scope, event } => {
                Self::Ingredients(Scoped::new(scope.into(), event.try_into()?))
            }
            AppEvent::Counter { scope, event } => {
                Self::Counter(Scoped::new(scope.into(), event.into()))
            }
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct AppView {
    pub tab: AppTab,
    pub scope: ScopeId,
    pub ingredients: IngredientCatalogView,
    pub counter: CounterView,
}

impl TryFrom<core::AppView> for AppView {
    type Error = AppTypeConversionError;

    fn try_from(value: core::AppView) -> Result<Self, Self::Error> {
        Ok(Self {
            tab: value.tab.into(),
            scope: value.scope.into(),
            ingredients: value.ingredients.try_into()?,
            counter: value.counter.try_into()?,
        })
    }
}
