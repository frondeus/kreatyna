use futures::StreamExt;
use kreatyna::{
    capabilities::{ResolveClock, ResolveCounterDatabase, ResolveRender},
    confirmation::ConfirmationEvent,
    ingredients::{capabilities::ResolveIngredientStore, *},
    *,
};
use krucyfiks::{
    Command, Effect, Message, Scoped, Widget,
    effects::{CommandEffect, CommandTestExt},
};
use test_case::test_case;

#[test_case(true; "counter finishes first")]
#[test_case(false; "ingredients finish first")]
#[tokio::test]
async fn initialization_is_concurrent_and_runs_once(counter_first: bool) -> anyhow::Result<()> {
    let mut f = Fixture::new();
    assert_eq!(f.app.view().tab, AppTab::Ingredients);
    let command = f.app.init();
    empty(f.app.init()).await;
    let mut driver = command.driver(&mut f.effects);
    let mut counter_request = None;
    let mut ingredient_request = None;
    for _ in 0..3 {
        match driver.step().await?.expect_effect() {
            AppEffect::Counter(effect) => counter_request = Some(effect),
            AppEffect::Ingredients(effect) => match effect {
                ingredients::capabilities::IngredientEffect::Render(envelope) => {
                    envelope.resolve_render(|| async {}).await?;
                }
                effect => ingredient_request = Some(effect),
            },
            _ => panic!("unexpected app render"),
        }
    }
    if counter_first {
        counter_request
            .take()
            .unwrap()
            .expect_database()
            .resolve_count(|| async { Ok(Some(7)) })
            .await?;
    } else {
        ingredient_request
            .take()
            .unwrap()
            .expect_store()
            .resolve_list(|| async { Ok(vec![]) })
            .await?;
    }
    let first = driver.step().await?.expect_outcome();
    assert_eq!(matches!(first, AppOutcome::Counter(_)), counter_first);
    if counter_first {
        ingredient_request
            .unwrap()
            .expect_store()
            .resolve_list(|| async { Ok(vec![]) })
            .await?;
    } else {
        counter_request
            .unwrap()
            .expect_database()
            .resolve_count(|| async { Ok(Some(7)) })
            .await?;
    }
    let second = driver.step().await?.expect_outcome();
    driver.step().await?.expect_done();
    drop(driver);
    for outcome in [first, second] {
        f.apply_render(outcome).await?;
    }
    assert_eq!(f.app.view().counter.count, 7);
    assert_eq!(f.app.view().counter.status, CounterStatus::Ready);
    empty(f.app.init()).await;
    Ok(())
}

#[tokio::test]
async fn tab_events_require_the_current_activation_and_ready_counter() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    let old = f.app.view().scope;
    f.switch(AppTab::Counter).await?;
    empty(f.counter(CounterEvent::Increase)).await;
    assert_eq!(f.app.view().counter.count, 0);
    f.initialize().await?;
    empty(f.app.update(Message::Event(AppEvent::Counter(Scoped::new(
        old,
        CounterEvent::Increase,
    )))))
    .await;
    empty(
        f.app
            .update(Message::Event(AppEvent::SelectTab(AppTab::Counter))),
    )
    .await;
    f.switch(AppTab::Ingredients).await?;
    empty(f.counter(CounterEvent::Increase)).await;
    let old = f.app.view().scope;
    f.switch(AppTab::Counter).await?;
    f.switch(AppTab::Ingredients).await?;
    let event = IngredientCatalogEvent::List(Scoped::new(
        f.app.view().ingredients.scope,
        IngredientListEvent::CreateRequested,
    ));
    empty(
        f.app
            .update(Message::Event(AppEvent::Ingredients(Scoped::new(
                old, event,
            )))),
    )
    .await;
    assert_eq!(
        f.app.view().ingredients.navigation,
        IngredientNavigation::List
    );
    Ok(())
}

#[tokio::test]
async fn navigation_draft_and_confirmation_survive_tab_switches() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    f.initialize().await?;
    let command = f.list(IngredientListEvent::Search("gar".into()));
    f.render(command).await?;
    f.create().await?;
    let IngredientCatalogScreenView::Create(form) = f.app.view().ingredients.screen else {
        panic!("expected create form")
    };
    assert_eq!(form.draft.basis, NutritionBasis::Per100Grams);
    let command = f.form(IngredientFormEvent::SetField {
        field: IngredientDraftField::Name,
        value: "Garlic".into(),
    });
    f.render(command).await?;
    let command = f.form(IngredientFormEvent::RequestClose);
    f.render(command).await?;
    let before = f.app.view().ingredients;
    f.switch(AppTab::Counter).await?;
    f.switch(AppTab::Ingredients).await?;
    assert_eq!(f.app.view().ingredients, before);
    let command = f.form(IngredientFormEvent::Confirmation(ConfirmationEvent::Cancel));
    let outcome = command.into_stream().next().await.unwrap();
    f.apply_render(outcome).await?;
    assert_eq!(f.app.view().ingredients.list.search, "gar");
    Ok(())
}

#[test_case(true; "saved while hidden")]
#[test_case(false; "save failed while hidden")]
#[tokio::test]
async fn pending_save_finishes_while_hidden(success: bool) -> anyhow::Result<()> {
    let mut f = Fixture::new();
    f.initialize().await?;
    f.create().await?;
    let command = f.form(IngredientFormEvent::SetField {
        field: IngredientDraftField::Name,
        value: "Garlic".into(),
    });
    f.render(command).await?;
    let save = f.form(IngredientFormEvent::Save);
    f.switch(AppTab::Counter).await?;
    let mut driver = save.driver(&mut f.effects);
    driver
        .step()
        .await?
        .expect_effect()
        .expect_ingredients()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    driver
        .step()
        .await?
        .expect_effect()
        .expect_ingredients()
        .expect_store()
        .resolve_save(|_| async move {
            if success {
                Ok(record())
            } else {
                Err(ingredients::capabilities::IngredientSaveError::Storage)
            }
        })
        .await?;
    let outcome = driver.step().await?.expect_outcome();
    driver.step().await?.expect_done();
    drop(driver);
    let command = f.app.update(Message::Outcome(outcome));
    let mut driver = command.driver(&mut f.effects);
    driver
        .step()
        .await?
        .expect_effect()
        .expect_ingredients()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    if success {
        let terminal = driver.step().await?.expect_outcome();
        driver.step().await?.expect_done();
        drop(driver);
        f.apply_render(terminal).await?;
        assert_eq!(
            f.app.view().ingredients.navigation,
            IngredientNavigation::List
        );
    } else {
        driver.step().await?.expect_done();
        drop(driver);
        let IngredientCatalogScreenView::Create(form) = f.app.view().ingredients.screen else {
            panic!()
        };
        assert_eq!(form.draft.name, "Garlic");
        assert!(form.save_error.is_some());
    }
    assert_eq!(f.app.view().tab, AppTab::Counter);
    Ok(())
}

#[test_case(true; "deleted while hidden")]
#[test_case(false; "delete failed while hidden")]
#[tokio::test]
async fn pending_delete_finishes_while_hidden(success: bool) -> anyhow::Result<()> {
    let mut f = Fixture::new();
    f.initialize().await?;
    let IngredientListStatus::Ready { rows, .. } = f.app.view().ingredients.list.status else {
        panic!("expected loaded catalog")
    };
    f.open_detail(rows[0].id).await?;
    let command = f.detail(IngredientDetailEvent::RequestDelete);
    f.render(command).await?;
    let decision = f
        .detail(IngredientDetailEvent::Confirmation(
            ConfirmationEvent::Confirm,
        ))
        .into_stream()
        .next()
        .await
        .unwrap();
    let delete = f.app.update(Message::Outcome(decision));
    f.switch(AppTab::Counter).await?;

    let mut driver = delete.driver(&mut f.effects);
    driver
        .step()
        .await?
        .expect_effect()
        .expect_ingredients()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    driver
        .step()
        .await?
        .expect_effect()
        .expect_ingredients()
        .expect_store()
        .resolve_delete(move |_| async move {
            if success {
                Ok(())
            } else {
                Err(ingredients::capabilities::IngredientDeleteError::Storage)
            }
        })
        .await?;
    let finished = driver.step().await?.expect_outcome();
    driver.step().await?.expect_done();
    drop(driver);

    let command = f.app.update(Message::Outcome(finished));
    let mut driver = command.driver(&mut f.effects);
    driver
        .step()
        .await?
        .expect_effect()
        .expect_ingredients()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    if success {
        let deleted = driver.step().await?.expect_outcome();
        driver.step().await?.expect_done();
        drop(driver);
        f.apply_render(deleted).await?;
        assert_eq!(
            f.app.view().ingredients.navigation,
            IngredientNavigation::List
        );
        assert!(matches!(
            f.app.view().ingredients.list.status,
            IngredientListStatus::Ready { ref rows, .. } if rows.is_empty()
        ));
    } else {
        driver.step().await?.expect_done();
        drop(driver);
        let IngredientCatalogScreenView::Detail(detail) = f.app.view().ingredients.screen else {
            panic!("expected retained detail")
        };
        assert!(matches!(
            detail.deletion,
            IngredientDetailDeletion::Confirming { error: Some(_), .. }
        ));
    }
    assert_eq!(f.app.view().tab, AppTab::Counter);
    Ok(())
}

#[tokio::test]
async fn counter_departure_cancels_ticks_but_preserves_accepted_writes() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    f.initialize().await?;
    f.switch(AppTab::Counter).await?;
    let background = f.counter(CounterEvent::StartBackground);
    let mut driver = background.driver(&mut f.effects);
    driver
        .step()
        .await?
        .expect_effect()
        .expect_counter()
        .expect_clock()
        .resolve_sleep_ms(|ms| async move {
            assert_eq!(ms, 1000);
        })
        .await?;
    let queued = driver.step().await?.expect_outcome();
    drop(driver);
    let write = f.counter(CounterEvent::Increase);
    f.switch(AppTab::Ingredients).await?;
    empty(f.app.update(Message::Outcome(queued))).await;
    let mut driver = write.driver(&mut f.effects);
    driver
        .step()
        .await?
        .expect_effect()
        .expect_counter()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    driver
        .step()
        .await?
        .expect_effect()
        .expect_counter()
        .expect_database()
        .resolve_set_count(|count| async move {
            assert_eq!(count, 8);
            Ok(())
        })
        .await?;
    driver.step().await?.expect_done();
    drop(driver);
    f.switch(AppTab::Counter).await?;
    let background = f.counter(CounterEvent::StartBackground);
    f.switch(AppTab::Ingredients).await?;
    empty(background).await;
    assert_eq!(f.app.view().counter.count, 8);
    Ok(())
}

#[tokio::test]
async fn shutdown_hook_stops_counter_background_but_preserves_an_accepted_write()
-> anyhow::Result<()> {
    let mut f = Fixture::new();
    f.initialize().await?;
    f.switch(AppTab::Counter).await?;
    let background = f.counter(CounterEvent::StartBackground);
    let write = f.counter(CounterEvent::Increase);
    let mut background = background.driver(&mut f.effects);
    let pending_clock = background
        .step()
        .await?
        .expect_effect()
        .expect_counter()
        .expect_clock();

    empty(f.app.begin_shutdown()).await;
    background.step().await?.expect_done();
    drop(pending_clock);
    drop(background);

    let mut write = write.driver(&mut f.effects);
    write
        .step()
        .await?
        .expect_effect()
        .expect_counter()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    write
        .step()
        .await?
        .expect_effect()
        .expect_counter()
        .expect_database()
        .resolve_set_count(|count| async move {
            assert_eq!(count, 8);
            Ok(())
        })
        .await?;
    write.step().await?.expect_done();

    assert_eq!(f.app.view().counter.count, 8);
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
    fn counter(&mut self, event: CounterEvent) -> Command<AppOutcome> {
        self.app
            .update(Message::Event(AppEvent::Counter(Scoped::new(
                self.app.view().scope,
                event,
            ))))
    }
    fn ingredients(&mut self, event: IngredientCatalogEvent) -> Command<AppOutcome> {
        self.app
            .update(Message::Event(AppEvent::Ingredients(Scoped::new(
                self.app.view().scope,
                event,
            ))))
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
    fn detail(&mut self, event: IngredientDetailEvent) -> Command<AppOutcome> {
        self.ingredients(IngredientCatalogEvent::Detail(Scoped::new(
            self.app.view().ingredients.scope,
            event,
        )))
    }
    async fn switch(&mut self, tab: AppTab) -> anyhow::Result<()> {
        let command = self.app.update(Message::Event(AppEvent::SelectTab(tab)));
        self.render(command).await
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
    async fn open_detail(&mut self, id: IngredientId) -> anyhow::Result<()> {
        let command = self.list(IngredientListEvent::Select(id));
        let outcome = command.into_stream().next().await.unwrap();
        self.apply_render(outcome).await
    }
    async fn initialize(&mut self) -> anyhow::Result<()> {
        let mut driver = self.app.init().driver(&mut self.effects);
        // Both branches can finish in any order; drive effects until both outcomes arrive.
        let mut outcomes = vec![];
        for _ in 0..5 {
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
                CommandEffect::Effect(AppEffect::Counter(effect)) => {
                    effect
                        .expect_database()
                        .resolve_count(|| async { Ok(Some(7)) })
                        .await?;
                }
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
