use futures::{StreamExt, stream::BoxStream};
use krucyfiks::{Command, CommandExt, CommandScope, Message, Scoped, Widget};

use crate::ingredients::{
    IngredientCatalog, IngredientCatalogEvent, IngredientCatalogOutcome, IngredientCatalogView,
    capabilities::{IngredientCapabilities, IngredientEffect},
};

#[derive(Debug)]
pub enum AppEvent {
    Ingredients(IngredientCatalogEvent),
}

#[derive(Debug)]
pub enum AppOutcome {
    Ingredients(Scoped<IngredientCatalogOutcome>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppView {
    pub ingredients: IngredientCatalogView,
}

#[derive(Clone)]
pub struct AppCapabilities {
    pub ingredients: IngredientCapabilities,
}

#[derive(Debug, krucyfiks::Effect)]
pub enum AppEffect {
    Ingredients(IngredientEffect),
}

pub type AppEffectStream = BoxStream<'static, AppEffect>;

impl AppCapabilities {
    pub fn testing() -> (Self, AppEffectStream) {
        let (ingredients, effects) = IngredientCapabilities::testing();
        (
            Self { ingredients },
            effects.map(AppEffect::Ingredients).boxed(),
        )
    }
}

pub struct AppWidget {
    ingredients: IngredientCatalog,
    ingredient_lifetime: CommandScope,
    initialized: bool,
}

impl AppWidget {
    pub fn new(caps: AppCapabilities) -> Self {
        Self {
            ingredients: IngredientCatalog::new(caps.ingredients),
            ingredient_lifetime: CommandScope::new(),
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
        self.ingredients_command(ingredients)
    }

    fn can_begin_shutdown(&self) -> bool {
        self.ingredients.can_begin_shutdown()
    }

    fn begin_shutdown(&mut self) -> Command<Self::Outcome> {
        let ingredients = self.ingredients.begin_shutdown();
        self.ingredients_command(ingredients)
    }

    fn update(&mut self, message: Message<Self>) -> Command<AppOutcome> {
        match message {
            Message::Event(AppEvent::Ingredients(event)) => {
                let command = self.ingredients.update(Message::Event(event));
                self.ingredients_command(command)
            }
            Message::Outcome(AppOutcome::Ingredients(outcome)) => {
                if !self.ingredient_lifetime.accepts(outcome.scope) {
                    return Command::empty();
                }
                let command = self.ingredients.update(Message::Outcome(outcome.value));
                self.ingredients_command(command)
            }
        }
    }

    fn view(&self) -> AppView {
        AppView {
            ingredients: self.ingredients.view(),
        }
    }
}
