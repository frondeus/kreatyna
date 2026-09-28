use std::sync::Arc;

use futures::{
    StreamExt,
    stream::{BoxStream, select_all},
};
use krucyfiks::{
    Command, CommandExt, CommandScope, CommandScopeId, Message, Scoped, Widget, effects::Envelope,
};

use crate::{
    CLIENT_CAPABILITY, Counter, CounterEvent, CounterOutcome, CounterStatus, CounterView,
    capabilities::{CounterCapabilities, CounterEffect, Render, RenderClient, RenderRequest},
    ingredients::{
        IngredientCatalog, IngredientCatalogEvent, IngredientCatalogOutcome, IngredientCatalogView,
        capabilities::{IngredientCapabilities, IngredientEffect},
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AppTab {
    #[default]
    Ingredients,
    Counter,
}

#[derive(Debug)]
pub enum AppEvent {
    SelectTab(AppTab),
    Ingredients(Scoped<IngredientCatalogEvent>),
    Counter(Scoped<CounterEvent>),
}

#[derive(Debug)]
pub enum AppOutcome {
    Ingredients(Scoped<IngredientCatalogOutcome>),
    Counter(Scoped<CounterOutcome>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppView {
    pub tab: AppTab,
    pub scope: CommandScopeId,
    pub ingredients: IngredientCatalogView,
    pub counter: CounterView,
}

#[derive(Clone)]
pub struct AppCapabilities {
    pub ingredients: IngredientCapabilities,
    pub counter: CounterCapabilities,
    pub render: Arc<dyn Render>,
}

#[derive(Debug, krucyfiks::Effect)]
pub enum AppEffect {
    Ingredients(IngredientEffect),
    Counter(CounterEffect),
    Render(Envelope<RenderRequest>),
}

pub type AppEffectStream = BoxStream<'static, AppEffect>;

impl AppCapabilities {
    pub fn testing() -> (Self, AppEffectStream) {
        let (ingredients, ingredient_effects) = IngredientCapabilities::testing();
        let (counter, counter_effects) = CounterCapabilities::testing();
        let (render, inbox) = RenderClient::new(CLIENT_CAPABILITY);
        let effects = select_all(vec![
            ingredient_effects.map(AppEffect::Ingredients).boxed(),
            counter_effects.map(AppEffect::Counter).boxed(),
            inbox.into_stream().map(AppEffect::Render).boxed(),
        ])
        .boxed();
        (
            Self {
                ingredients,
                counter,
                render: Arc::new(render),
            },
            effects,
        )
    }
}

pub struct AppWidget {
    ingredients: IngredientCatalog,
    counter: Counter,
    render: Arc<dyn Render>,
    tab: AppTab,
    activation: CommandScope,
    app_lifetime: CommandScope,
    ingredient_lifetime: CommandScope,
    counter_lifetime: CommandScope,
    initialized: bool,
}

impl AppWidget {
    pub fn new(caps: AppCapabilities) -> Self {
        Self {
            ingredients: IngredientCatalog::new(caps.ingredients),
            counter: Counter::new(caps.counter),
            render: caps.render,
            tab: AppTab::Ingredients,
            activation: CommandScope::new(),
            app_lifetime: CommandScope::new(),
            ingredient_lifetime: CommandScope::new(),
            counter_lifetime: CommandScope::new(),
            initialized: false,
        }
    }

    pub fn test_fixture() -> (Self, AppEffectStream) {
        let (caps, effects) = AppCapabilities::testing();
        (Self::new(caps), effects)
    }

    fn ingredients_command(
        &mut self,
        command: Command<IngredientCatalogOutcome>,
    ) -> Command<AppOutcome> {
        command
            .scoped(&mut self.ingredient_lifetime)
            .map(AppOutcome::Ingredients)
    }

    fn counter_command(&mut self, command: Command<CounterOutcome>) -> Command<AppOutcome> {
        command
            .scoped(&mut self.counter_lifetime)
            .map(AppOutcome::Counter)
    }

    fn render(&mut self) -> Command<AppOutcome> {
        let render = self.render.clone();
        Command::<AppOutcome>::without_outcome(async move { render.render().await })
            .scoped(&mut self.app_lifetime)
            .map(|_| unreachable!("a render command has no outcome"))
    }
}

impl Widget for AppWidget {
    type View = AppView;
    type Event = AppEvent;
    type Outcome = AppOutcome;

    fn init(&mut self) -> Command<AppOutcome> {
        if self.initialized {
            return Command::empty();
        }
        self.initialized = true;
        let ingredients = self.ingredients.init();
        let counter = self.counter.init();
        Command::select_all(vec![
            self.ingredients_command(ingredients),
            self.counter_command(counter),
        ])
    }

    fn can_begin_shutdown(&self) -> bool {
        self.ingredients.can_begin_shutdown() && self.counter.can_begin_shutdown()
    }

    fn begin_shutdown(&mut self) -> Command<Self::Outcome> {
        let ingredients = self.ingredients.begin_shutdown();
        let counter = self.counter.begin_shutdown();
        Command::select_all(vec![
            self.ingredients_command(ingredients),
            self.counter_command(counter),
        ])
    }

    fn update(&mut self, message: Message<Self>) -> Command<AppOutcome> {
        match message {
            Message::Event(AppEvent::SelectTab(tab)) => {
                if self.tab == tab {
                    return Command::empty();
                }
                let stop = if self.tab == AppTab::Counter {
                    let command = self
                        .counter
                        .update(Message::Event(CounterEvent::StopBackground));
                    self.counter_command(command)
                } else {
                    Command::empty()
                };
                self.tab = tab;
                self.activation = CommandScope::new();
                Command::select_all(vec![stop, self.render()])
            }
            Message::Event(AppEvent::Ingredients(event)) => {
                if self.tab != AppTab::Ingredients || !self.activation.accepts(event.scope) {
                    return Command::empty();
                }
                let command = self.ingredients.update(Message::Event(event.value));
                self.ingredients_command(command)
            }
            Message::Event(AppEvent::Counter(event)) => {
                if self.tab != AppTab::Counter
                    || !self.activation.accepts(event.scope)
                    || self.counter.view().status != CounterStatus::Ready
                {
                    return Command::empty();
                }
                let command = self.counter.update(Message::Event(event.value));
                self.counter_command(command)
            }
            Message::Outcome(AppOutcome::Ingredients(outcome)) => {
                if !self.ingredient_lifetime.accepts(outcome.scope) {
                    return Command::empty();
                }
                let command = self.ingredients.update(Message::Outcome(outcome.value));
                self.ingredients_command(command)
            }
            Message::Outcome(AppOutcome::Counter(outcome)) => {
                if !self.counter_lifetime.accepts(outcome.scope) {
                    return Command::empty();
                }
                if matches!(outcome.value, CounterOutcome::Loaded { .. })
                    && self.counter.view().status == CounterStatus::Ready
                {
                    return Command::empty();
                }
                let command = self.counter.update(Message::Outcome(outcome.value));
                self.counter_command(command)
            }
        }
    }

    fn view(&self) -> AppView {
        AppView {
            tab: self.tab,
            scope: self.activation.id(),
            ingredients: self.ingredients.view(),
            counter: self.counter.view(),
        }
    }
}
