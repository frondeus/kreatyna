use anyhow::Context;
use futures::StreamExt;
use kreatyna::{
    capabilities::ResolveRender,
    ingredients::{
        capabilities::{IngredientEffect, ResolveIngredientStore},
        *,
    },
    recipes::{
        Recipe, RecipeDetailStatus, RecipeDetailView, RecipeEditorEvent, RecipeId, RecipeListEvent,
        RecipeListRow, RecipeListStatus, RecipePaneView, RecipeSectionEvent,
        capabilities::{RecipeEffect, RecipeLoadError, RecipeStoreRequest, ResolveRecipeStore},
    },
    *,
};
use krucyfiks::{
    Command, Effect, Message, Scoped, Widget,
    effects::{CommandEffect, CommandTestExt},
};

// Green path

#[tokio::test]
async fn initialization_loads_the_catalog_once() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    let command = f.app.init();
    empty(f.app.init()).await;
    f.drive_initialization(command).await?;
    assert!(matches!(
        f.stack()?.list.status,
        IngredientListStatus::Ready { ref rows, .. }
            if rows.iter().map(|row| row.name.as_str()).eq(["Garlic"])
    ));
    empty(f.app.init()).await;
    Ok(())
}

#[tokio::test]
async fn catalog_events_and_outcomes_route_through_the_app() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    f.initialize().await?;
    let command = f.list(IngredientListEvent::Search("gar".into()))?;
    f.render(command).await?;
    assert_eq!(f.stack()?.list.search, "gar");
    f.create().await?;
    assert!(matches!(
        f.stack()?.screen,
        IngredientCatalogScreenView::Create(_)
    ));
    Ok(())
}

#[tokio::test]
async fn shutdown_hook_preserves_an_accepted_ingredient_save_and_follow_up() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    f.initialize().await?;
    f.create().await?;
    let command = f.form(IngredientFormEvent::SetField {
        field: IngredientDraftField::Name,
        value: "Garlic".into(),
    })?;
    f.render(command).await?;
    let save = f.form(IngredientFormEvent::Save)?;

    empty(f.app.begin_shutdown()).await;

    let mut save = save.driver(&mut f.effects);
    save.step()
        .await?
        .expect_effect()
        .expect_ingredients()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    let stored = garlic()?;
    save.step()
        .await?
        .expect_effect()
        .expect_ingredients()
        .expect_store()
        .resolve_save(|_| async move { Ok(stored) })
        .await?;
    let saved = save.step().await?.expect_outcome();
    save.step().await?.expect_done();
    drop(save);

    let command = f.app.update(Message::Outcome(saved));
    let mut command = command.driver(&mut f.effects);
    command
        .step()
        .await?
        .expect_effect()
        .expect_ingredients()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    let terminal = command.step().await?.expect_outcome();
    command.step().await?.expect_done();
    drop(command);
    f.apply_render(terminal).await?;

    assert_eq!(f.stack()?.navigation, IngredientNavigation::List);
    assert!(matches!(
        f.stack()?.list.status,
        IngredientListStatus::Ready { ref rows, .. }
            if rows.iter().any(|row| row.name == "Garlic")
    ));
    Ok(())
}

#[tokio::test]
async fn the_split_layout_starts_on_recipes_and_loads_both_sections() -> anyhow::Result<()> {
    let mut f = Fixture::with(AppLayout::Split);
    f.initialize().await?;
    let app = f.split()?;
    assert_eq!(app.section, AppSection::Recipes);
    assert_eq!(
        app.recipes.list.status,
        RecipeListStatus::Ready {
            rows: vec![RecipeListRow {
                id: shakshuka().id,
                title: "Shakshuka".into()
            }]
        }
    );
    assert!(matches!(
        app.ingredients.list.value.status,
        IngredientListStatus::Ready { ref rows, .. }
            if rows.iter().map(|row| row.name.as_str()).eq(["Garlic"])
    ));
    assert_eq!(app.ingredients.selected, None);
    Ok(())
}

#[tokio::test]
async fn selecting_a_section_renders_and_keeps_both_sections() -> anyhow::Result<()> {
    let mut f = Fixture::with(AppLayout::Split);
    f.initialize().await?;
    let before = f.split()?;

    let command = f.split_event(SplitAppEvent::SelectSection(AppSection::Ingredients));
    assert_eq!(
        f.split()?,
        SplitAppView {
            section: AppSection::Ingredients,
            ..before.clone()
        }
    );
    f.render(command).await?;

    let command = f.split_event(SplitAppEvent::SelectSection(AppSection::Recipes));
    assert_eq!(f.split()?, before);
    f.render(command).await?;
    Ok(())
}

#[tokio::test]
async fn the_split_layout_routes_ingredient_events_to_its_catalog() -> anyhow::Result<()> {
    let mut f = Fixture::with(AppLayout::Split);
    f.initialize().await?;
    let catalog = f.split()?.ingredients;
    let row = first_ingredient(&catalog.list.value)?;

    let command = f.split_event(SplitAppEvent::Ingredients(
        IngredientSplitCatalogEvent::List(Scoped::new(
            catalog.list.scope,
            IngredientListEvent::Select(row),
        )),
    ));
    let outcome = command
        .into_stream()
        .next()
        .await
        .context("the selection has no outcome")?;
    f.apply_render(outcome).await?;
    assert_eq!(f.split()?.ingredients.selected, Some(row));
    // Opening an ingredient does not change the selected section.
    assert_eq!(f.split()?.section, AppSection::Recipes);
    Ok(())
}

#[tokio::test]
async fn recipe_section_events_route_through_the_app() -> anyhow::Result<()> {
    let mut f = Fixture::with(AppLayout::Split);
    f.initialize().await?;
    let command = f.recipes(RecipeSectionEvent::NewRecipe);
    f.render(command).await?;
    f.recipe_editor(RecipeEditorEvent::SetTitle("Pasta".into()))
        .await?;
    assert!(matches!(
        f.split()?.recipes.pane.value,
        RecipePaneView::Create { ref draft_row, .. } if draft_row == "Pasta"
    ));
    Ok(())
}

#[tokio::test]
async fn a_recipe_row_click_opens_its_read_view_and_a_section_switch_keeps_it() -> anyhow::Result<()>
{
    let mut f = Fixture::with(AppLayout::Split);
    f.initialize().await?;
    let command = f.recipes(RecipeSectionEvent::Select(shakshuka().id));
    f.drive_initialization(command).await?;
    let recipes = f.split()?.recipes;
    assert_eq!(recipes.selected, Some(shakshuka().id));
    assert_eq!(
        recipes.pane.value,
        RecipePaneView::Detail(RecipeDetailView {
            id: shakshuka().id,
            status: RecipeDetailStatus::Ready {
                title: "Shakshuka".into()
            },
        })
    );

    let command = f.split_event(SplitAppEvent::SelectSection(AppSection::Ingredients));
    f.render(command).await?;
    let command = f.split_event(SplitAppEvent::SelectSection(AppSection::Recipes));
    f.render(command).await?;
    assert_eq!(f.split()?.recipes, recipes);
    Ok(())
}

#[tokio::test]
async fn switching_sections_keeps_an_open_recipe_editor() -> anyhow::Result<()> {
    let mut f = Fixture::with(AppLayout::Split);
    f.initialize().await?;
    let command = f.recipes(RecipeSectionEvent::NewRecipe);
    f.render(command).await?;
    f.recipe_editor(RecipeEditorEvent::SetTitle("Pasta".into()))
        .await?;
    let before = f.split()?.recipes;

    let command = f.split_event(SplitAppEvent::SelectSection(AppSection::Ingredients));
    f.render(command).await?;
    let command = f.split_event(SplitAppEvent::SelectSection(AppSection::Recipes));
    f.render(command).await?;
    assert_eq!(f.split()?.recipes, before);
    Ok(())
}

#[tokio::test]
async fn shutdown_keeps_an_accepted_recipe_save() -> anyhow::Result<()> {
    let mut f = Fixture::with(AppLayout::Split);
    f.initialize().await?;
    let command = f.recipes(RecipeSectionEvent::NewRecipe);
    f.render(command).await?;
    f.recipe_editor(RecipeEditorEvent::SetTitle("Pasta".into()))
        .await?;
    let scope = f.split()?.recipes.pane.scope;
    let save = f.recipes(RecipeSectionEvent::Editor(Scoped::new(
        scope,
        RecipeEditorEvent::Save,
    )));

    assert!(f.app.can_begin_shutdown());
    empty(f.app.begin_shutdown()).await;

    let mut save = save.driver(&mut f.effects);
    save.step()
        .await?
        .expect_effect()
        .expect_recipes()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    save.step()
        .await?
        .expect_effect()
        .expect_recipes()
        .expect_store()
        .resolve_create(|_| async { Ok(pasta()) })
        .await?;
    let finished = save.step().await?.expect_outcome();
    save.step().await?.expect_done();
    drop(save);
    let command = f.app.update(Message::Outcome(finished));
    let saved = f.render_then_outcome(command).await?;
    let command = f.app.update(Message::Outcome(saved));
    f.recipes = Ok(vec![pasta(), shakshuka()]);
    f.drive_initialization(command).await?;
    // The saved recipe's read view opens (REC-03).
    assert_eq!(
        f.split()?.recipes.pane.value,
        RecipePaneView::Detail(RecipeDetailView {
            id: pasta().id,
            status: RecipeDetailStatus::Ready {
                title: "Pasta".into()
            },
        })
    );
    assert!(matches!(
        f.split()?.recipes.list.status,
        RecipeListStatus::Ready { ref rows } if rows.len() == 2
    ));
    Ok(())
}

// Validation

#[tokio::test]
async fn an_event_for_the_other_layout_is_ignored() -> anyhow::Result<()> {
    let mut stack = Fixture::new();
    stack.initialize().await?;
    let before = stack.app.view();
    empty(stack.app.update(Message::Event(AppEvent::Split(
        SplitAppEvent::SelectSection(AppSection::Ingredients),
    ))))
    .await;
    assert_eq!(stack.app.view(), before);

    let mut split = Fixture::with(AppLayout::Split);
    split.initialize().await?;
    let before = split.app.view();
    let scope = split.split()?.ingredients.list.scope;
    empty(split.app.update(Message::Event(AppEvent::Stack(
        IngredientCatalogEvent::List(Scoped::new(scope, IngredientListEvent::CreateRequested)),
    ))))
    .await;
    assert_eq!(split.app.view(), before);
    Ok(())
}

#[tokio::test]
async fn app_drop_cancels_unpolled_initialization() {
    let mut f = Fixture::new();
    let command = f.app.init();
    drop(f.app);
    empty(command).await;
}

// Red cases

#[tokio::test]
async fn a_failed_recipe_load_can_be_retried_through_the_app() -> anyhow::Result<()> {
    let mut f = Fixture::with(AppLayout::Split);
    f.recipes = Err(RecipeLoadError::Storage);
    f.initialize().await?;
    assert_eq!(
        f.split()?.recipes.list.status,
        RecipeListStatus::Failed(RecipeLoadError::Storage)
    );

    f.recipes = Ok(vec![shakshuka()]);
    let command = f.recipes(RecipeSectionEvent::List(RecipeListEvent::Refresh));
    assert_eq!(f.split()?.recipes.list.status, RecipeListStatus::Loading);
    f.drive_initialization(command).await?;
    assert!(matches!(
        f.split()?.recipes.list.status,
        RecipeListStatus::Ready { ref rows } if rows.len() == 1
    ));
    Ok(())
}

// Test support

fn garlic() -> anyhow::Result<Ingredient> {
    Ok(Ingredient {
        id: IngredientId::generate(),
        name: IngredientName::try_new("Garlic")?,
        notes: None,
        nutrition: Nutrition {
            basis: NutritionBasis::Per100Grams,
            calories: None,
            protein: None,
            carbohydrates: None,
            fat: None,
        },
        units: vec![],
    })
}

fn shakshuka() -> Recipe {
    Recipe {
        id: RecipeId::from(uuid::Uuid::from_u128(1)),
        title: "Shakshuka".into(),
    }
}

fn pasta() -> Recipe {
    Recipe {
        id: RecipeId::from(uuid::Uuid::from_u128(2)),
        title: "Pasta".into(),
    }
}

fn first_ingredient(list: &IngredientListView) -> anyhow::Result<IngredientId> {
    let IngredientListStatus::Ready { rows, .. } = &list.status else {
        anyhow::bail!("expected a ready list, got {:?}", list.status);
    };
    let first = rows.first().context("the list has no rows")?;
    Ok(first.id)
}

async fn empty(command: Command<AppOutcome>) {
    assert!(command.into_stream().next().await.is_none());
}

struct Fixture {
    app: AppWidget,
    effects: AppEffectStream,
    /// What the recipe store returns to the next list request.
    recipes: Result<Vec<Recipe>, RecipeLoadError>,
}

impl Fixture {
    fn new() -> Self {
        Self::with(AppLayout::Stack)
    }

    fn with(layout: AppLayout) -> Self {
        let (app, effects) = AppWidget::test_fixture_with(layout);
        Self {
            app,
            effects,
            recipes: Ok(vec![shakshuka()]),
        }
    }

    fn stack(&self) -> anyhow::Result<IngredientCatalogView> {
        let view = self.app.view();
        let AppView::Stack(catalog) = view else {
            anyhow::bail!("expected the stack layout, got {view:?}");
        };
        Ok(catalog)
    }

    fn split(&self) -> anyhow::Result<SplitAppView> {
        let view = self.app.view();
        let AppView::Split(app) = view else {
            anyhow::bail!("expected the split layout, got {view:?}");
        };
        Ok(app)
    }

    fn split_event(&mut self, event: SplitAppEvent) -> Command<AppOutcome> {
        self.app.update(Message::Event(AppEvent::Split(event)))
    }

    fn recipes(&mut self, event: RecipeSectionEvent) -> Command<AppOutcome> {
        self.split_event(SplitAppEvent::Recipes(event))
    }

    /// Sends an editor event with the pane's scope and runs its render.
    async fn recipe_editor(&mut self, event: RecipeEditorEvent) -> anyhow::Result<()> {
        let scope = self.split()?.recipes.pane.scope;
        let command = self.recipes(RecipeSectionEvent::Editor(Scoped::new(scope, event)));
        self.render(command).await
    }

    async fn render_then_outcome(
        &mut self,
        command: Command<AppOutcome>,
    ) -> anyhow::Result<AppOutcome> {
        let mut driver = command.driver(&mut self.effects);
        driver
            .step()
            .await?
            .expect_effect()
            .any_route()
            .resolve_render(|| async {})
            .await?;
        let outcome = driver.step().await?.expect_outcome();
        driver.step().await?.expect_done();
        Ok(outcome)
    }

    fn stack_event(&mut self, event: IngredientCatalogEvent) -> Command<AppOutcome> {
        self.app.update(Message::Event(AppEvent::Stack(event)))
    }

    fn list(&mut self, event: IngredientListEvent) -> anyhow::Result<Command<AppOutcome>> {
        let scope = self.stack()?.scope;
        Ok(self.stack_event(IngredientCatalogEvent::List(Scoped::new(scope, event))))
    }

    fn form(&mut self, event: IngredientFormEvent) -> anyhow::Result<Command<AppOutcome>> {
        let scope = self.stack()?.scope;
        Ok(self.stack_event(IngredientCatalogEvent::Form(Scoped::new(scope, event))))
    }

    async fn render(&mut self, command: Command<AppOutcome>) -> anyhow::Result<()> {
        let mut driver = command.driver(&mut self.effects);
        driver
            .step()
            .await?
            .expect_effect()
            .any_route()
            .resolve_render(|| async {})
            .await?;
        driver.step().await?.expect_done();
        Ok(())
    }

    async fn apply_render(&mut self, outcome: AppOutcome) -> anyhow::Result<()> {
        let command = self.app.update(Message::Outcome(outcome));
        self.render(command).await
    }

    async fn create(&mut self) -> anyhow::Result<()> {
        let command = self.list(IngredientListEvent::CreateRequested)?;
        let outcome = command
            .into_stream()
            .next()
            .await
            .context("the create request has no outcome")?;
        self.apply_render(outcome).await
    }

    async fn initialize(&mut self) -> anyhow::Result<()> {
        let command = self.app.init();
        self.drive_initialization(command).await
    }

    /// Runs the loads of a command in any order: each list renders its
    /// loading state, then reads its store. Then it applies every outcome.
    async fn drive_initialization(&mut self, command: Command<AppOutcome>) -> anyhow::Result<()> {
        let mut driver = command.driver(&mut self.effects);
        let mut outcomes = vec![];
        loop {
            match driver.step().await? {
                CommandEffect::Effect(AppEffect::Ingredients(IngredientEffect::Render(e)))
                | CommandEffect::Effect(AppEffect::Recipes(RecipeEffect::Render(e))) => {
                    e.resolve_render(|| async {}).await?;
                }
                CommandEffect::Effect(AppEffect::Ingredients(IngredientEffect::Store(e))) => {
                    let stored = vec![garlic()?];
                    e.resolve_list(|| async move { Ok(stored) }).await?;
                }
                CommandEffect::Effect(AppEffect::Recipes(RecipeEffect::Store(e))) => {
                    let recipes = self.recipes.clone();
                    match e.request {
                        RecipeStoreRequest::Get { id } => {
                            let found = recipes
                                .map(|recipes| recipes.into_iter().find(|recipe| recipe.id == id));
                            e.resolve_get(|_| async { found }).await?;
                        }
                        _ => e.resolve_list(|| async { recipes }).await?,
                    }
                }
                CommandEffect::Outcome(outcome) => outcomes.push(outcome),
                CommandEffect::Done => break,
            }
        }
        drop(driver);
        for outcome in outcomes {
            self.apply_render(outcome).await?;
        }
        Ok(())
    }
}
