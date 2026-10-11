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
        f.app.view().ingredients.list.status,
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
    assert_eq!(f.app.view().ingredients.list.search, "gar");
    f.create().await?;
    assert!(matches!(
        f.app.view().ingredients.screen,
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

    assert_eq!(
        f.app.view().ingredients.navigation,
        IngredientNavigation::List
    );
    assert!(matches!(
        f.app.view().ingredients.list.status,
        IngredientListStatus::Ready { ref rows, .. }
            if rows.iter().any(|row| row.name == "Garlic")
    ));
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
async fn empty(command: Command<AppOutcome>) {
    assert!(command.into_stream().next().await.is_none());
}
struct Fixture {
    app: AppWidget,
    effects: AppEffectStream,
}
impl Fixture {
    fn new() -> Self {
        let (app, effects) = AppWidget::test_fixture();
        Self { app, effects }
    }
    fn ingredients(&mut self, event: IngredientCatalogEvent) -> Command<AppOutcome> {
        self.app
            .update(Message::Event(AppEvent::Ingredients(event)))
    }
    fn list(&mut self, event: IngredientListEvent) -> Command<AppOutcome> {
        self.ingredients(IngredientCatalogEvent::List(Scoped::new(
            self.app.view().ingredients.scope,
            event,
        )))
    }
    fn form(&mut self, event: IngredientFormEvent) -> Command<AppOutcome> {
        self.ingredients(IngredientCatalogEvent::Form(Scoped::new(
            self.app.view().ingredients.scope,
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
