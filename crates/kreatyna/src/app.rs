use futures::{
    StreamExt,
    stream::{BoxStream, select},
};
use krucyfiks::{Command, CommandExt, Message, Scoped, Widget};
use std::sync::Arc;

use crate::capabilities::Render;

use crate::ingredients::{
    IngredientCatalog, IngredientCatalogEvent, IngredientCatalogOutcome, IngredientCatalogView,
    IngredientSplitCatalog, IngredientSplitCatalogEvent, IngredientSplitCatalogOutcome,
    IngredientSplitCatalogView,
    capabilities::{IngredientCapabilities, IngredientEffect},
};
use crate::recipes::{
    RecipeSection, RecipeSectionEvent, RecipeSectionOutcome, RecipeSectionView,
    capabilities::{RecipeCapabilities, RecipeEffect},
};
use crate::scoped_child::ScopedChild;

/// How the platform shows the app. Swift chooses it once, at startup (E-19).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppLayout {
    /// Screens on a stack, one at a time (iPhone).
    Stack,
    /// A sidebar with sections, and each section's list and detail (Mac).
    Split,
}

/// A sidebar section of the split layout. Swift writes the names, the icons,
/// and the order: Recipes, then Ingredients.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppSection {
    Recipes,
    Ingredients,
}

/// Each layout has its own events, so an event cannot reach the wrong one.
#[derive(Debug)]
pub enum AppEvent {
    Stack(IngredientCatalogEvent),
    Split(SplitAppEvent),
}

#[derive(Debug)]
pub enum SplitAppEvent {
    SelectSection(AppSection),
    Recipes(RecipeSectionEvent),
    Ingredients(IngredientSplitCatalogEvent),
}

#[derive(Debug)]
pub enum AppOutcome {
    Stack(Scoped<IngredientCatalogOutcome>),
    SplitRecipes(Scoped<RecipeSectionOutcome>),
    SplitIngredients(Scoped<IngredientSplitCatalogOutcome>),
}

#[expect(
    clippy::large_enum_variant,
    reason = "an app has one layout, so one value of each exists at a time; boxing would only add an allocation per render"
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppView {
    /// iOS: the ingredient catalog on a stack. No sections (E-04).
    Stack(IngredientCatalogView),
    /// Mac: a sidebar with the Recipes and Ingredients sections.
    Split(SplitAppView),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SplitAppView {
    /// The selected section. Both sections always exist.
    pub section: AppSection,
    pub recipes: RecipeSectionView,
    pub ingredients: IngredientSplitCatalogView,
}

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

/// The children of the chosen layout. Each variant holds exactly its own.
#[expect(clippy::large_enum_variant, reason = "one per app, see `AppView`")]
enum Layout {
    Stack(ScopedChild<IngredientCatalog>),
    Split(SplitLayout),
}

struct SplitLayout {
    section: AppSection,
    render: Arc<dyn Render>,
    recipes: ScopedChild<RecipeSection>,
    ingredients: ScopedChild<IngredientSplitCatalog>,
}

/// Whether `init` ran. A second `init` starts nothing.
enum Startup {
    NotStarted,
    Started,
}

pub struct AppWidget {
    layout: Layout,
    startup: Startup,
}

impl AppWidget {
    pub fn new(caps: AppCapabilities, layout: AppLayout) -> Self {
        let layout = match layout {
            AppLayout::Stack => {
                Layout::Stack(ScopedChild::new(IngredientCatalog::new(caps.ingredients)))
            }
            AppLayout::Split => Layout::Split(SplitLayout {
                // REC-01/Q1: the Mac app opens on Recipes.
                section: AppSection::Recipes,
                render: caps.recipes.render.clone(),
                recipes: ScopedChild::new(RecipeSection::new(caps.recipes)),
                ingredients: ScopedChild::new(IngredientSplitCatalog::new(caps.ingredients)),
            }),
        };
        Self {
            layout,
            startup: Startup::NotStarted,
        }
    }

    pub fn test_fixture() -> (Self, AppEffectStream) {
        Self::test_fixture_with(AppLayout::Stack)
    }

    pub fn test_fixture_with(layout: AppLayout) -> (Self, AppEffectStream) {
        let (caps, effects) = AppCapabilities::testing();
        (Self::new(caps, layout), effects)
    }
}

impl SplitLayout {
    // The scopes live as long as the app, so dropping the app cancels the
    // children's commands.
    fn recipes(&mut self, command: Command<RecipeSectionOutcome>) -> Command<AppOutcome> {
        self.recipes.command(command).map(AppOutcome::SplitRecipes)
    }

    fn ingredients(
        &mut self,
        command: Command<IngredientSplitCatalogOutcome>,
    ) -> Command<AppOutcome> {
        self.ingredients
            .command(command)
            .map(AppOutcome::SplitIngredients)
    }

    fn update(&mut self, event: SplitAppEvent) -> Command<AppOutcome> {
        match event {
            SplitAppEvent::SelectSection(section) => {
                self.section = section;
                // Both sections keep their state; only the selection changes.
                // The render runs in the recipe list's scope, so dropping the
                // app cancels it.
                let render = self.render.clone();
                self.recipes(Command::without_outcome(
                    async move { render.render().await },
                ))
            }
            SplitAppEvent::Recipes(event) => {
                let command = self.recipes.widget.update(Message::Event(event));
                self.recipes(command)
            }
            SplitAppEvent::Ingredients(event) => {
                let command = self.ingredients.widget.update(Message::Event(event));
                self.ingredients(command)
            }
        }
    }
}

impl Widget for AppWidget {
    type View = AppView;
    type Event = AppEvent;
    type Outcome = AppOutcome;

    fn init(&mut self) -> Command<AppOutcome> {
        match self.startup {
            Startup::Started => return Command::empty(),
            Startup::NotStarted => self.startup = Startup::Started,
        }
        match &mut self.layout {
            Layout::Stack(catalog) => {
                let command = catalog.widget.init();
                catalog.command(command).map(AppOutcome::Stack)
            }
            Layout::Split(split) => {
                let recipes = split.recipes.widget.init();
                let ingredients = split.ingredients.widget.init();
                Command::select_all(vec![split.recipes(recipes), split.ingredients(ingredients)])
            }
        }
    }

    /// Only the ingredient catalog can delay shutdown. The recipe section
    /// does not: shutdown does not cancel a recipe save that already runs.
    fn can_begin_shutdown(&self) -> bool {
        match &self.layout {
            Layout::Stack(catalog) => catalog.widget.can_begin_shutdown(),
            Layout::Split(split) => split.ingredients.widget.can_begin_shutdown(),
        }
    }

    fn begin_shutdown(&mut self) -> Command<Self::Outcome> {
        match &mut self.layout {
            Layout::Stack(catalog) => {
                let command = catalog.widget.begin_shutdown();
                catalog.command(command).map(AppOutcome::Stack)
            }
            Layout::Split(split) => {
                let command = split.ingredients.widget.begin_shutdown();
                split.ingredients(command)
            }
        }
    }

    /// An event or outcome for the other layout is ignored.
    fn update(&mut self, message: Message<Self>) -> Command<AppOutcome> {
        match (message, &mut self.layout) {
            (Message::Event(AppEvent::Stack(event)), Layout::Stack(catalog)) => {
                let command = catalog.widget.update(Message::Event(event));
                catalog.command(command).map(AppOutcome::Stack)
            }
            (Message::Event(AppEvent::Split(event)), Layout::Split(split)) => split.update(event),
            (Message::Outcome(AppOutcome::Stack(scoped)), Layout::Stack(catalog)) => {
                match catalog.accept(scoped) {
                    Some(outcome) => {
                        let command = catalog.widget.update(Message::Outcome(outcome));
                        catalog.command(command).map(AppOutcome::Stack)
                    }
                    None => Command::empty(),
                }
            }
            (Message::Outcome(AppOutcome::SplitRecipes(scoped)), Layout::Split(split)) => {
                match split.recipes.accept(scoped) {
                    Some(outcome) => {
                        let command = split.recipes.widget.update(Message::Outcome(outcome));
                        split.recipes(command)
                    }
                    None => Command::empty(),
                }
            }
            (Message::Outcome(AppOutcome::SplitIngredients(scoped)), Layout::Split(split)) => {
                match split.ingredients.accept(scoped) {
                    Some(outcome) => {
                        let command = split.ingredients.widget.update(Message::Outcome(outcome));
                        split.ingredients(command)
                    }
                    None => Command::empty(),
                }
            }
            // A value for the other layout: ignore it.
            (
                Message::Event(AppEvent::Split(_))
                | Message::Outcome(AppOutcome::SplitRecipes(_) | AppOutcome::SplitIngredients(_)),
                Layout::Stack(_),
            )
            | (
                Message::Event(AppEvent::Stack(_)) | Message::Outcome(AppOutcome::Stack(_)),
                Layout::Split(_),
            ) => Command::empty(),
        }
    }

    fn view(&self) -> AppView {
        match &self.layout {
            Layout::Stack(catalog) => AppView::Stack(catalog.widget.view()),
            Layout::Split(split) => AppView::Split(SplitAppView {
                section: split.section,
                recipes: split.recipes.widget.view(),
                ingredients: split.ingredients.widget.view(),
            }),
        }
    }
}
