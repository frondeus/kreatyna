use futures::StreamExt;
use kreatyna::{
    capabilities::ResolveRender,
    ingredients::{capabilities::ResolveIngredientStore, *},
    *,
};
use krucyfiks::{
    Command, Effect, Message, Scoped, Widget,
    effects::{CommandEffect, CommandTestExt},
};

#[tokio::test]
async fn initialization_loads_the_catalog_once() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    let command = f.app.init();
    empty(f.app.init()).await;
    f.drive_initialization(command).await?;
    assert!(matches!(
        f.stack().list.status,
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
    let command = f.list(IngredientListEvent::Search("gar".into()));
    f.render(command).await?;
    assert_eq!(f.stack().list.search, "gar");
    f.create().await?;
    assert!(matches!(
        f.stack().screen,
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
    });
    f.render(command).await?;
    let save = f.form(IngredientFormEvent::Save);

    empty(f.app.begin_shutdown()).await;

    let mut save = save.driver(&mut f.effects);
    save.step()
        .await?
        .expect_effect()
        .expect_ingredients()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    save.step()
        .await?
        .expect_effect()
        .expect_ingredients()
        .expect_store()
        .resolve_save(|_| async { Ok(record()) })
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

    assert_eq!(f.stack().navigation, IngredientNavigation::List);
    assert!(matches!(
        f.stack().list.status,
        IngredientListStatus::Ready { ref rows, .. }
            if rows.iter().any(|row| row.name == "Garlic")
    ));
    Ok(())
}

#[tokio::test]
async fn the_split_layout_runs_the_split_catalog() -> anyhow::Result<()> {
    let mut f = Fixture::with(AppLayout::Split);
    f.initialize().await?;
    let catalog = f.split();
    assert!(matches!(
        catalog.list.value.status,
        IngredientListStatus::Ready { ref rows, .. }
            if rows.iter().map(|row| row.name.as_str()).eq(["Garlic"])
    ));
    assert_eq!(catalog.selected, None);

    let row = record_id(&catalog.list.value);
    let command = f.app.update(Message::Event(AppEvent::SplitIngredients(
        IngredientSplitCatalogEvent::List(Scoped::new(
            catalog.list.scope,
            IngredientListEvent::Select(row),
        )),
    )));
    let outcome = command.into_stream().next().await.unwrap();
    f.apply_render(outcome).await?;
    assert_eq!(f.split().selected, Some(row));
    Ok(())
}

#[tokio::test]
async fn an_event_for_the_other_layout_is_ignored() -> anyhow::Result<()> {
    let mut stack = Fixture::new();
    stack.initialize().await?;
    let before = stack.app.view();
    let scope = stack.stack().scope;
    empty(stack.app.update(Message::Event(AppEvent::SplitIngredients(
        IngredientSplitCatalogEvent::ClearSelection { scope },
    ))))
    .await;
    assert_eq!(stack.app.view(), before);

    let mut split = Fixture::with(AppLayout::Split);
    split.initialize().await?;
    let before = split.app.view();
    let scope = split.split().list.scope;
    empty(split.app.update(Message::Event(AppEvent::Ingredients(
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

fn record() -> Ingredient {
    Ingredient {
        id: IngredientId::new(),
        name: "Garlic".into(),
        notes: None,
        nutrition: Nutrition {
            basis: NutritionBasis::Per100Grams,
            calories: None,
            protein: None,
            carbohydrates: None,
            fat: None,
        },
        units: vec![],
    }
}
fn record_id(list: &IngredientListView) -> IngredientId {
    let IngredientListStatus::Ready { rows, .. } = &list.status else {
        panic!("expected a ready list");
    };
    rows[0].id
}
async fn empty(command: Command<AppOutcome>) {
    assert!(command.into_stream().next().await.is_none());
}
struct Fixture {
    app: AppWidget,
    effects: AppEffectStream,
}
impl Fixture {
    fn new() -> Self {
        Self::with(AppLayout::Stack)
    }
    fn with(layout: AppLayout) -> Self {
        let (app, effects) = AppWidget::test_fixture_with(layout);
        Self { app, effects }
    }
    fn stack(&self) -> IngredientCatalogView {
        let IngredientsView::Stack(catalog) = self.app.view().ingredients else {
            panic!("expected the stack layout");
        };
        catalog
    }
    fn split(&self) -> IngredientSplitCatalogView {
        let IngredientsView::Split(catalog) = self.app.view().ingredients else {
            panic!("expected the split layout");
        };
        catalog
    }
    fn ingredients(&mut self, event: IngredientCatalogEvent) -> Command<AppOutcome> {
        self.app
            .update(Message::Event(AppEvent::Ingredients(event)))
    }
    fn list(&mut self, event: IngredientListEvent) -> Command<AppOutcome> {
        self.ingredients(IngredientCatalogEvent::List(Scoped::new(
            self.stack().scope,
            event,
        )))
    }
    fn form(&mut self, event: IngredientFormEvent) -> Command<AppOutcome> {
        self.ingredients(IngredientCatalogEvent::Form(Scoped::new(
            self.stack().scope,
            event,
        )))
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
        let command = self.list(IngredientListEvent::CreateRequested);
        let outcome = command.into_stream().next().await.unwrap();
        self.apply_render(outcome).await
    }
    async fn initialize(&mut self) -> anyhow::Result<()> {
        let command = self.app.init();
        self.drive_initialization(command).await
    }
    async fn drive_initialization(&mut self, command: Command<AppOutcome>) -> anyhow::Result<()> {
        let mut driver = command.driver(&mut self.effects);
        // The catalog renders its loading state and loads the list in either order.
        let mut outcomes = vec![];
        for _ in 0..3 {
            match driver.step().await? {
                CommandEffect::Effect(AppEffect::Ingredients(effect)) => match effect {
                    ingredients::capabilities::IngredientEffect::Render(e) => {
                        e.resolve_render(|| async {}).await?;
                    }
                    effect => {
                        effect
                            .expect_store()
                            .resolve_list(|| async { Ok(vec![record()]) })
                            .await?;
                    }
                },
                CommandEffect::Outcome(outcome) => outcomes.push(outcome),
                _ => panic!("unexpected initialization step"),
            }
        }
        driver.step().await?.expect_done();
        drop(driver);
        for outcome in outcomes {
            self.apply_render(outcome).await?;
        }
        Ok(())
    }
}
