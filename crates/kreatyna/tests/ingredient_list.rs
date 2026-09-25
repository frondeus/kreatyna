use futures::StreamExt;
use kreatyna::capabilities::ResolveRender;
use kreatyna::ingredients::{
    Ingredient, IngredientId, IngredientList, IngredientListEvent, IngredientListOutcome,
    IngredientListRefresh, IngredientListStatus, Nutrition, NutritionBasis,
    capabilities::{IngredientEffectStream, IngredientLoadError, ResolveIngredientStore},
};
use krucyfiks::{Message, Widget, effects::CommandTestExt};
use test_case::test_case;

#[test_case(Vec::new(); "empty catalog")]
#[test_case(vec![ingredient("Basil"), ingredient("Garlic")]; "populated catalog")]
#[tokio::test]
async fn initial_load_renders_before_store_and_exposes_records(
    records: Vec<Ingredient>,
) -> anyhow::Result<()> {
    let mut fixture = ListFixture::new();
    let expected_count = records.len();
    fixture.assert_initial_load(Ok(records)).await?;
    let IngredientListStatus::Ready {
        rows,
        total_count,
        refresh,
    } = fixture.list.view().status
    else {
        panic!("expected ready list")
    };
    assert_eq!(rows.len(), expected_count);
    assert_eq!(total_count, expected_count);
    assert_eq!(refresh, IngredientListRefresh::Idle);
    Ok(())
}

#[test_case(IngredientLoadError::Storage; "storage")]
#[test_case(IngredientLoadError::Transport; "transport")]
#[tokio::test]
async fn initial_error_is_typed_and_retry_recovers(
    error: IngredientLoadError,
) -> anyhow::Result<()> {
    let mut fixture = ListFixture::new();
    fixture.assert_initial_load(Err(error.clone())).await?;
    assert_eq!(
        fixture.list.view().status,
        IngredientListStatus::Failed(error)
    );
    let mut retry = fixture
        .list
        .update(Message::Event(IngredientListEvent::Refresh))
        .driver(&mut fixture.effects);
    assert!(matches!(
        fixture.list.view().status,
        IngredientListStatus::Loading
    ));
    retry
        .drive()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    retry
        .drive()
        .await?
        .expect_effect()
        .expect_store()
        .resolve_list(|| async { Ok(vec![ingredient("Basil")]) })
        .await?;
    let completed = retry.drive().await?.expect_outcome();
    retry.drive().await?.expect_done();
    drop(retry);
    let mut render = fixture
        .list
        .update(Message::Outcome(completed))
        .driver(&mut fixture.effects);
    render
        .drive()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    render.drive().await?.expect_done();
    assert_eq!(rows(&fixture.list), ["Basil"]);
    Ok(())
}

#[test_case("  STRAS  ", vec!["Straße"]; "unicode folded substring")]
#[test_case("gar", vec!["Garlic"]; "ordinary substring")]
#[test_case("  ", vec!["Garlic", "Straße", "Σ"]; "blank query")]
#[test_case("missing", Vec::new(); "no matches")]
#[tokio::test]
async fn search_is_local_and_sorted(query: &str, expected: Vec<&str>) -> anyhow::Result<()> {
    let mut fixture = ListFixture::new();
    fixture
        .assert_initial_load(Ok(vec![
            ingredient("Σ"),
            ingredient("Straße"),
            ingredient("Garlic"),
        ]))
        .await?;
    let mut command = fixture
        .list
        .update(Message::Event(IngredientListEvent::Search(query.into())))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.list.view().search, query);
    assert_eq!(rows(&fixture.list), expected);
    command
        .drive()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command.drive().await?.expect_done();
    assert!(matches!(
        fixture.list.view().status,
        IngredientListStatus::Ready { total_count: 3, .. }
    ));
    Ok(())
}

#[tokio::test]
async fn equal_folded_names_use_id_tie_breaker() -> anyhow::Result<()> {
    let mut fixture = ListFixture::new();
    let mut low = ingredient("Straße");
    let mut high = ingredient("STRASSE");
    if low.id > high.id {
        std::mem::swap(&mut low, &mut high);
    }
    fixture
        .assert_initial_load(Ok(vec![high.clone(), low.clone()]))
        .await?;
    let IngredientListStatus::Ready { rows, .. } = fixture.list.view().status else {
        panic!("expected ready")
    };
    assert_eq!(
        rows.into_iter().map(|row| row.id).collect::<Vec<_>>(),
        [low.id, high.id]
    );
    Ok(())
}

#[test_case(IngredientLoadError::Storage; "storage refresh error")]
#[test_case(IngredientLoadError::Transport; "transport refresh error")]
#[tokio::test]
async fn refresh_failure_keeps_cached_rows_and_retry_replaces_them(
    error: IngredientLoadError,
) -> anyhow::Result<()> {
    let mut fixture = ListFixture::new();
    let original = ingredient("Garlic");
    let old_completion = fixture
        .assert_initial_load(Ok(vec![original.clone()]))
        .await?;
    let mut search = fixture
        .list
        .update(Message::Event(IngredientListEvent::Search("gar".into())))
        .driver(&mut fixture.effects);
    search
        .drive()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    search.drive().await?.expect_done();
    drop(search);

    let mut refresh = fixture
        .list
        .update(Message::Event(IngredientListEvent::Refresh))
        .driver(&mut fixture.effects);
    assert_eq!(rows(&fixture.list), ["Garlic"]);
    assert!(matches!(
        fixture.list.view().status,
        IngredientListStatus::Ready {
            refresh: IngredientListRefresh::Loading,
            ..
        }
    ));
    let during_refresh = fixture
        .list
        .update(Message::Event(IngredientListEvent::Select(original.id)))
        .into_stream()
        .next()
        .await
        .unwrap();
    assert_eq!(
        during_refresh,
        IngredientListOutcome::OpenDetail {
            ingredient: original.clone()
        }
    );
    assert!(
        fixture
            .list
            .update(Message::Outcome(old_completion))
            .into_stream()
            .next()
            .await
            .is_none()
    );
    refresh
        .drive()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    refresh
        .drive()
        .await?
        .expect_effect()
        .expect_store()
        .resolve_list(|| async { Err(error.clone()) })
        .await?;
    let failure = refresh.drive().await?.expect_outcome();
    refresh.drive().await?.expect_done();
    drop(refresh);
    let mut render = fixture
        .list
        .update(Message::Outcome(failure))
        .driver(&mut fixture.effects);
    render
        .drive()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    render.drive().await?.expect_done();
    drop(render);
    assert_eq!(rows(&fixture.list), ["Garlic"]);
    assert!(matches!(
        fixture.list.view().status,
        IngredientListStatus::Ready {
            refresh: IngredientListRefresh::Failed(_),
            ..
        }
    ));
    let after_failure = fixture
        .list
        .update(Message::Event(IngredientListEvent::Select(original.id)))
        .into_stream()
        .next()
        .await
        .unwrap();
    assert_eq!(
        after_failure,
        IngredientListOutcome::OpenDetail {
            ingredient: original
        }
    );

    let mut retry = fixture
        .list
        .update(Message::Event(IngredientListEvent::Refresh))
        .driver(&mut fixture.effects);
    retry
        .drive()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    retry
        .drive()
        .await?
        .expect_effect()
        .expect_store()
        .resolve_list(|| async { Ok(vec![ingredient("Garden pea")]) })
        .await?;
    let success = retry.drive().await?.expect_outcome();
    retry.drive().await?.expect_done();
    drop(retry);
    let mut render = fixture
        .list
        .update(Message::Outcome(success))
        .driver(&mut fixture.effects);
    render
        .drive()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    render.drive().await?.expect_done();
    assert_eq!(fixture.list.view().search, "gar");
    assert_eq!(rows(&fixture.list), ["Garden pea"]);
    Ok(())
}

#[tokio::test]
async fn stale_results_are_ignored() -> anyhow::Result<()> {
    let mut fixture = ListFixture::new();
    let mut first = fixture.list.init().driver(&mut fixture.effects);
    first
        .drive()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    first
        .drive()
        .await?
        .expect_effect()
        .expect_store()
        .resolve_list(|| async { Ok(vec![ingredient("Basil")]) })
        .await?;
    let completed = first.drive().await?.expect_outcome();
    first.drive().await?.expect_done();
    drop(first);
    let mut render = fixture
        .list
        .update(Message::Outcome(completed.clone()))
        .driver(&mut fixture.effects);
    render
        .drive()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    render.drive().await?.expect_done();
    drop(render);
    let mut stale = fixture
        .list
        .update(Message::Outcome(completed))
        .driver(&mut fixture.effects);
    stale.drive().await?.expect_done();
    assert_eq!(rows(&fixture.list), ["Basil"]);
    Ok(())
}

#[tokio::test]
async fn selection_uses_visible_complete_records_and_create_always_works() -> anyhow::Result<()> {
    let mut fixture = ListFixture::new();
    let garlic = ingredient("Garlic");
    let basil = ingredient("Basil");
    fixture
        .assert_initial_load(Ok(vec![garlic.clone(), basil.clone()]))
        .await?;
    let mut search = fixture
        .list
        .update(Message::Event(IngredientListEvent::Search("gar".into())))
        .driver(&mut fixture.effects);
    search
        .drive()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    search.drive().await?.expect_done();
    drop(search);
    for event in [
        IngredientListEvent::Select(basil.id),
        IngredientListEvent::Edit(IngredientId::new()),
    ] {
        let mut command = fixture
            .list
            .update(Message::Event(event))
            .driver(&mut fixture.effects);
        command.drive().await?.expect_done();
    }
    let mut detail = fixture
        .list
        .update(Message::Event(IngredientListEvent::Select(garlic.id)))
        .driver(&mut fixture.effects);
    let outcome = detail.drive().await?.expect_outcome();
    assert_eq!(
        outcome,
        IngredientListOutcome::OpenDetail {
            ingredient: garlic.clone()
        }
    );
    detail.drive().await?.expect_done();
    drop(detail);
    let mut ignored = fixture
        .list
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    ignored.drive().await?.expect_done();
    drop(ignored);
    let mut edit = fixture
        .list
        .update(Message::Event(IngredientListEvent::Edit(garlic.id)))
        .driver(&mut fixture.effects);
    assert_eq!(
        edit.drive().await?.expect_outcome(),
        IngredientListOutcome::OpenEdit { ingredient: garlic }
    );
    edit.drive().await?.expect_done();
    drop(edit);
    let mut create = fixture
        .list
        .update(Message::Event(IngredientListEvent::CreateRequested))
        .driver(&mut fixture.effects);
    assert_eq!(
        create.drive().await?.expect_outcome(),
        IngredientListOutcome::CreateRequested
    );
    create.drive().await?.expect_done();
    Ok(())
}

#[tokio::test]
async fn create_is_available_during_loading_and_errors() -> anyhow::Result<()> {
    let mut fixture = ListFixture::new();
    let mut create = fixture
        .list
        .update(Message::Event(IngredientListEvent::CreateRequested))
        .driver(&mut fixture.effects);
    assert_eq!(
        create.drive().await?.expect_outcome(),
        IngredientListOutcome::CreateRequested
    );
    create.drive().await?.expect_done();
    drop(create);
    fixture
        .assert_initial_load(Err(IngredientLoadError::Storage))
        .await?;
    let mut create = fixture
        .list
        .update(Message::Event(IngredientListEvent::CreateRequested))
        .driver(&mut fixture.effects);
    assert_eq!(
        create.drive().await?.expect_outcome(),
        IngredientListOutcome::CreateRequested
    );
    create.drive().await?.expect_done();
    Ok(())
}

#[tokio::test]
async fn refresh_replaces_initial_load() -> anyhow::Result<()> {
    let mut fixture = ListFixture::new();
    let initial = fixture.list.init();
    let mut refresh = fixture
        .list
        .update(Message::Event(IngredientListEvent::Refresh))
        .driver(&mut fixture.effects);
    assert!(initial.into_stream().next().await.is_none());
    refresh
        .drive()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    refresh
        .drive()
        .await?
        .expect_effect()
        .expect_store()
        .resolve_list(|| async { Ok(vec![ingredient("Basil")]) })
        .await?;
    let outcome = refresh.drive().await?.expect_outcome();
    refresh.drive().await?.expect_done();
    drop(refresh);
    let mut render = fixture
        .list
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    render
        .drive()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    render.drive().await?.expect_done();
    assert_eq!(rows(&fixture.list), ["Basil"]);
    Ok(())
}

#[tokio::test]
async fn latest_refresh_cancels_previous_and_rejects_queued_result() -> anyhow::Result<()> {
    let mut fixture = ListFixture::new();
    let old = fixture
        .assert_initial_load(Ok(vec![ingredient("Basil")]))
        .await?;
    let cancelled = fixture
        .list
        .update(Message::Event(IngredientListEvent::Refresh));
    let replacement = fixture
        .list
        .update(Message::Event(IngredientListEvent::Refresh));
    assert!(cancelled.into_stream().next().await.is_none());
    assert!(
        fixture
            .list
            .update(Message::Outcome(old))
            .into_stream()
            .next()
            .await
            .is_none()
    );
    let mut command = replacement.driver(&mut fixture.effects);
    command
        .drive()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command
        .drive()
        .await?
        .expect_effect()
        .expect_store()
        .resolve_list(|| async { Ok(vec![ingredient("Garlic")]) })
        .await?;
    let queued = command.drive().await?.expect_outcome();
    command.drive().await?.expect_done();
    drop(command);
    let newest = fixture
        .list
        .update(Message::Event(IngredientListEvent::Refresh));
    assert!(
        fixture
            .list
            .update(Message::Outcome(queued))
            .into_stream()
            .next()
            .await
            .is_none()
    );
    assert_eq!(rows(&fixture.list), ["Basil"]);
    let mut command = newest.driver(&mut fixture.effects);
    command
        .drive()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command
        .drive()
        .await?
        .expect_effect()
        .expect_store()
        .resolve_list(|| async { Ok(vec![ingredient("Peas")]) })
        .await?;
    let outcome = command.drive().await?.expect_outcome();
    command.drive().await?.expect_done();
    drop(command);
    let mut render = fixture
        .list
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    render
        .drive()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    render.drive().await?.expect_done();
    assert_eq!(rows(&fixture.list), ["Peas"]);
    Ok(())
}

#[tokio::test]
async fn dropping_widget_cancels_pending_load() -> anyhow::Result<()> {
    let mut fixture = ListFixture::new();
    let mut command = fixture.list.init().driver(&mut fixture.effects);
    command
        .drive()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    drop(fixture.list);
    command.drive().await?.expect_done();
    Ok(())
}

fn ingredient(name: &str) -> Ingredient {
    Ingredient {
        id: IngredientId::new(),
        name: name.into(),
        notes: Some("complete record".into()),
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

struct ListFixture {
    list: IngredientList,
    effects: IngredientEffectStream,
}

impl ListFixture {
    fn new() -> Self {
        let (list, effects) = IngredientList::test_fixture();
        Self { list, effects }
    }

    /// Check the initial loading view, render/store sequence, and completion render.
    async fn assert_initial_load(
        &mut self,
        result: Result<Vec<Ingredient>, IngredientLoadError>,
    ) -> anyhow::Result<IngredientListOutcome> {
        let mut command = self.list.init().driver(&mut self.effects);
        assert!(matches!(
            self.list.view().status,
            IngredientListStatus::Loading
        ));
        command
            .drive()
            .await?
            .expect_effect()
            .expect_render()
            .resolve_render(|| async {})
            .await?;
        command
            .drive()
            .await?
            .expect_effect()
            .expect_store()
            .resolve_list(|| async move { result })
            .await?;
        let outcome = command.drive().await?.expect_outcome();
        command.drive().await?.expect_done();
        drop(command);
        let mut render = self
            .list
            .update(Message::Outcome(outcome.clone()))
            .driver(&mut self.effects);
        render
            .drive()
            .await?
            .expect_effect()
            .expect_render()
            .resolve_render(|| async {})
            .await?;
        render.drive().await?.expect_done();
        Ok(outcome)
    }
}

fn rows(list: &IngredientList) -> Vec<String> {
    let IngredientListStatus::Ready { rows, .. } = list.view().status else {
        panic!("expected ready list")
    };
    rows.into_iter().map(|row| row.name).collect()
}
