use futures::{
    StreamExt,
    stream::{BoxStream, select},
};
use krucyfiks::{Command, CommandExt, Message, Scoped, Widget};

use crate::ingredients::{
    IngredientCatalog, IngredientCatalogEvent, IngredientCatalogOutcome, IngredientCatalogView,
    IngredientSplitCatalog, IngredientSplitCatalogEvent, IngredientSplitCatalogOutcome,
    IngredientSplitCatalogView,
    capabilities::{IngredientCapabilities, IngredientEffect},
};
use crate::recipes::capabilities::{RecipeCapabilities, RecipeEffect};
use crate::scoped_child::ScopedChild;

/// How the platform shows the catalog. Swift chooses it once, at startup (E-19).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppLayout {
    /// Screens on a stack, one at a time (iPhone).
    Stack,
    /// The list and the detail side by side (Mac).
    Split,
}

#[derive(Debug)]
pub enum AppEvent {
    Ingredients(IngredientCatalogEvent),
    SplitIngredients(IngredientSplitCatalogEvent),
}

#[derive(Debug)]
pub enum AppOutcome {
    Ingredients(Scoped<IngredientsOutcome>),
}

/// An outcome of the catalog coordinator of the app's layout.
#[derive(Debug)]
pub enum IngredientsOutcome {
    Stack(IngredientCatalogOutcome),
    Split(IngredientSplitCatalogOutcome),
}

// An app has one layout, so one value of each exists at a time. Boxing would
// only add an allocation per render.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngredientsView {
    Stack(IngredientCatalogView),
    Split(IngredientSplitCatalogView),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppView {
    pub ingredients: IngredientsView,
}

/// The recipe capabilities wait for the Recipes section (REC-01).
#[derive(Clone)]
pub struct AppCapabilities {
    pub ingredients: IngredientCapabilities,
    pub recipes: RecipeCapabilities,
}

#[derive(Debug, krucyfiks::Effect)]
pub enum AppEffect {
    Ingredients(IngredientEffect),
    Recipes(RecipeEffect),
}

pub type AppEffectStream = BoxStream<'static, AppEffect>;

impl AppCapabilities {
    pub fn testing() -> (Self, AppEffectStream) {
        let (ingredients, ingredient_effects) = IngredientCapabilities::testing();
        let (recipes, recipe_effects) = RecipeCapabilities::testing();
        (
            Self {
                ingredients,
                recipes,
            },
            select(
                ingredient_effects.map(AppEffect::Ingredients),
                recipe_effects.map(AppEffect::Recipes),
            )
            .boxed(),
        )
    }
}

/// The app's one child: the catalog coordinator of the chosen layout. The app
/// has no other children, so app events go to it unchanged.
#[allow(clippy::large_enum_variant)] // one per app, see `IngredientsView`
enum Ingredients {
    Stack(IngredientCatalog),
    Split(IngredientSplitCatalog),
}

impl Widget for Ingredients {
    type View = IngredientsView;
    type Event = AppEvent;
    type Outcome = IngredientsOutcome;

    fn init(&mut self) -> Command<Self::Outcome> {
        match self {
            Self::Stack(catalog) => catalog.init().map(IngredientsOutcome::Stack),
            Self::Split(catalog) => catalog.init().map(IngredientsOutcome::Split),
        }
    }

    fn can_begin_shutdown(&self) -> bool {
        match self {
            Self::Stack(catalog) => catalog.can_begin_shutdown(),
            Self::Split(catalog) => catalog.can_begin_shutdown(),
        }
    }

    fn begin_shutdown(&mut self) -> Command<Self::Outcome> {
        match self {
            Self::Stack(catalog) => catalog.begin_shutdown().map(IngredientsOutcome::Stack),
            Self::Split(catalog) => catalog.begin_shutdown().map(IngredientsOutcome::Split),
        }
    }

    /// An event or outcome for the other layout is ignored.
    fn update(&mut self, message: Message<Self>) -> Command<Self::Outcome> {
        use IngredientsOutcome as Outcome;
        match (message, self) {
            (Message::Event(AppEvent::Ingredients(event)), Self::Stack(catalog)) => {
                catalog.update(Message::Event(event)).map(Outcome::Stack)
            }
            (Message::Event(AppEvent::SplitIngredients(event)), Self::Split(catalog)) => {
                catalog.update(Message::Event(event)).map(Outcome::Split)
            }
            (Message::Outcome(Outcome::Stack(outcome)), Self::Stack(catalog)) => catalog
                .update(Message::Outcome(outcome))
                .map(Outcome::Stack),
            (Message::Outcome(Outcome::Split(outcome)), Self::Split(catalog)) => catalog
                .update(Message::Outcome(outcome))
                .map(Outcome::Split),
            _ => Command::empty(),
        }
    }

    fn view(&self) -> IngredientsView {
        match self {
            Self::Stack(catalog) => IngredientsView::Stack(catalog.view()),
            Self::Split(catalog) => IngredientsView::Split(catalog.view()),
        }
    }
}

pub struct AppWidget {
    ingredients: ScopedChild<Ingredients>,
    initialized: bool,
}

impl AppWidget {
    pub fn new(caps: AppCapabilities, layout: AppLayout) -> Self {
        let ingredients = match layout {
            AppLayout::Stack => Ingredients::Stack(IngredientCatalog::new(caps.ingredients)),
            AppLayout::Split => Ingredients::Split(IngredientSplitCatalog::new(caps.ingredients)),
        };
        Self {
            ingredients: ScopedChild::new(ingredients),
            initialized: false,
        }
    }

    pub fn test_fixture() -> (Self, AppEffectStream) {
        Self::test_fixture_with(AppLayout::Stack)
    }

    pub fn test_fixture_with(layout: AppLayout) -> (Self, AppEffectStream) {
        let (caps, effects) = AppCapabilities::testing();
        (Self::new(caps, layout), effects)
    }

    /// The scope lives as long as the app, so dropping the app cancels the
    /// catalog's commands.
    fn ingredients_command(&mut self, command: Command<IngredientsOutcome>) -> Command<AppOutcome> {
        self.ingredients
            .command(command)
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
        let command = self.ingredients.widget.init();
        self.ingredients_command(command)
    }

    fn can_begin_shutdown(&self) -> bool {
        self.ingredients.widget.can_begin_shutdown()
    }

    fn begin_shutdown(&mut self) -> Command<Self::Outcome> {
        let command = self.ingredients.widget.begin_shutdown();
        self.ingredients_command(command)
    }

    fn update(&mut self, message: Message<Self>) -> Command<AppOutcome> {
        let command = match message {
            Message::Event(event) => self.ingredients.widget.update(Message::Event(event)),
            Message::Outcome(AppOutcome::Ingredients(scoped)) => {
                match self.ingredients.accept(scoped) {
                    Some(outcome) => self.ingredients.widget.update(Message::Outcome(outcome)),
                    None => return Command::empty(),
                }
            }
        };
        self.ingredients_command(command)
    }

    fn view(&self) -> AppView {
        AppView {
            ingredients: self.ingredients.widget.view(),
        }
    }
}
