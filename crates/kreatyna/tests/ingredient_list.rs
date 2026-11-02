use kreatyna::capabilities::ResolveRender;
use kreatyna::ingredients::{
    Grams, Ingredient, IngredientId, IngredientList, IngredientListEvent, IngredientListOutcome,
    IngredientListRefresh, IngredientListRow, IngredientListStatus, IngredientListView,
    IngredientName, Kilocalories, Nutrition, NutritionBasis, ReferenceUnit, UnitName,
    capabilities::{IngredientEffectStream, IngredientLoadError, ResolveIngredientStore},
};
use krucyfiks::{
    Latest, Message, Widget,
    effects::{CommandTestExt, EnvelopeError},
};
use rust_decimal::Decimal;
use test_case::{test_case, test_matrix};
use uuid::Uuid;

// Green path

#[test_case(&[], &[]; "empty catalog")]
#[test_case(&[(2, "Garlic"), (1, "Basil")], &[(1, "Basil"), (2, "Garlic")]; "sort populated catalog")]
#[test_case(&[(2, "STRASSE"), (1, "Straße")], &[(1, "Straße"), (2, "STRASSE")]; "equal folded names use id order")]
#[tokio::test]
async fn initialization_renders_then_reads_and_dispatching_its_result_exposes_sorted_rows(
    records: &[(u128, &str)],
    expected: &[(u128, &str)],
) -> anyhow::Result<()> {
    let records = ingredients(records)?;
    let expected = ingredients(expected)?;
    let mut fixture = ListFixture::new();
    let mut command = fixture.list.init().driver(&mut fixture.effects);
    assert_eq!(fixture.list.view(), loading_view(""));
    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command
        .step()
        .await?
        .expect_effect()
        .expect_store()
        .resolve_list(|| async { Ok(records.clone()) })
        .await?;
    let outcome = command.step().await?.expect_outcome();
    command.step().await?.expect_done();
    drop(command);

    let IngredientListOutcome::LoadFinished(Latest { value: result, .. }) = &outcome else {
        anyhow::bail!("expected a completed load, got {outcome:?}");
    };
    assert_eq!(result, &Ok(records.clone()));

    // Reading records does not apply them. Dispatch the result to leave loading.
    let mut render = fixture
        .list
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.list.view(),
        ready_view("", &expected, records.len(), IngredientListRefresh::Idle)
    );
    render
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    render.step().await?.expect_done();
    drop(render);
    Ok(())
}

#[test_case("  STRAS  ", &[(2, "Straße")]; "unicode folded substring")]
#[test_case("GAR", &[(3, "Garlic")]; "case folded substring")]
#[test_case("  ", &[(3, "Garlic"), (2, "Straße"), (1, "Σ")]; "blank query")]
#[test_case("missing", &[]; "no matches")]
#[tokio::test]
async fn search_filters_and_sorts_cached_records_without_reading_the_store(
    query: &str,
    expected: &[(u128, &str)],
) -> anyhow::Result<()> {
    let expected = ingredients(expected)?;
    let mut fixture = ListFixture::ready(
        vec![
            ingredient(1, "Σ")?,
            ingredient(2, "Straße")?,
            ingredient(3, "Garlic")?,
        ],
        "",
    )
    .await?;
    let mut command = fixture
        .list
        .update(Message::Event(IngredientListEvent::Search(query.into())))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.list.view(),
        ready_view(query, &expected, 3, IngredientListRefresh::Idle)
    );

    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command.step().await?.expect_done();
    Ok(())
}

#[test_case(&[], &[]; "refresh empties the catalog")]
#[test_case(&[(3, "Garden pea")], &[(3, "Garden pea")]; "refresh replaces matching records")]
#[test_case(&[(3, "Peas")], &[]; "refresh removes all search matches")]
#[tokio::test]
async fn refresh_keeps_cached_rows_until_dispatch_then_replaces_records_and_preserves_search(
    records: &[(u128, &str)],
    expected: &[(u128, &str)],
) -> anyhow::Result<()> {
    let records = ingredients(records)?;
    let expected = ingredients(expected)?;
    let garlic = ingredient(1, "Garlic")?;
    let mut fixture =
        ListFixture::ready(vec![garlic.clone(), ingredient(2, "Basil")?], "gar").await?;
    let mut command = fixture
        .list
        .update(Message::Event(IngredientListEvent::Refresh))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.list.view(),
        ready_view("gar", &[garlic], 2, IngredientListRefresh::Loading)
    );

    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command
        .step()
        .await?
        .expect_effect()
        .expect_store()
        .resolve_list(|| async { Ok(records.clone()) })
        .await?;
    let outcome = command.step().await?.expect_outcome();
    command.step().await?.expect_done();
    drop(command);

    let mut render = fixture
        .list
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.list.view(),
        ready_view("gar", &expected, records.len(), IngredientListRefresh::Idle)
    );
    render
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    render.step().await?.expect_done();
    drop(render);
    Ok(())
}

#[test_case(IngredientListEvent::Select(id(1)), |ingredient| IngredientListOutcome::OpenDetail { ingredient }; "open detail")]
#[test_case(IngredientListEvent::Edit(id(1)), |ingredient| IngredientListOutcome::OpenEdit { ingredient }; "open editor")]
#[tokio::test]
async fn choosing_a_visible_record_emits_the_complete_ingredient_without_changing_the_list(
    event: IngredientListEvent,
    expected: fn(Ingredient) -> IngredientListOutcome,
) -> anyhow::Result<()> {
    let expected = expected(ingredient(1, "Garlic")?);
    let mut fixture = ListFixture::ready(
        vec![ingredient(1, "Garlic")?, ingredient(2, "Basil")?],
        "gar",
    )
    .await?;
    let before = fixture.list.view();
    let mut command = fixture
        .list
        .update(Message::Event(event))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.list.view(), before);

    let outcome = command.step().await?.expect_outcome();
    assert_eq!(outcome, expected);
    command.step().await?.expect_done();
    Ok(())
}

#[test_case(&[]; "empty catalog")]
#[test_case(&[(1, "Basil")]; "populated catalog")]
#[tokio::test]
async fn requesting_creation_emits_navigation_without_changing_the_list(
    records: &[(u128, &str)],
) -> anyhow::Result<()> {
    let records = ingredients(records)?;
    let mut fixture = ListFixture::ready(records, "").await?;
    let before = fixture.list.view();
    let mut command = fixture
        .list
        .update(Message::Event(IngredientListEvent::CreateRequested))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.list.view(), before);

    let outcome = command.step().await?.expect_outcome();
    assert_eq!(outcome, IngredientListOutcome::CreateRequested);
    command.step().await?.expect_done();
    Ok(())
}

#[tokio::test]
async fn creation_is_available_while_the_initial_load_is_pending() -> anyhow::Result<()> {
    let mut fixture = ListFixture::new();
    let load = fixture.list.init();
    let mut command = fixture
        .list
        .update(Message::Event(IngredientListEvent::CreateRequested))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.list.view(), loading_view(""));

    let outcome = command.step().await?.expect_outcome();
    assert_eq!(outcome, IngredientListOutcome::CreateRequested);
    command.step().await?.expect_done();
    drop(load);
    Ok(())
}

#[tokio::test]
async fn search_entered_during_initial_loading_is_used_when_records_arrive() -> anyhow::Result<()> {
    let mut fixture = ListFixture::new();
    let load = fixture.list.init();
    let mut search = fixture
        .list
        .update(Message::Event(IngredientListEvent::Search("gar".into())))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.list.view(), loading_view("gar"));

    search
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    search.step().await?.expect_done();
    drop(search);

    let mut command = load.driver(&mut fixture.effects);
    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    let stored = ingredients(&[(1, "Garlic"), (2, "Basil")])?;
    command
        .step()
        .await?
        .expect_effect()
        .expect_store()
        .resolve_list(|| async move { Ok(stored) })
        .await?;
    let outcome = command.step().await?.expect_outcome();
    command.step().await?.expect_done();
    drop(command);

    let mut render = fixture
        .list
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.list.view(),
        ready_view(
            "gar",
            &[ingredient(1, "Garlic")?],
            2,
            IngredientListRefresh::Idle
        )
    );
    render
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    render.step().await?.expect_done();
    drop(render);
    Ok(())
}

// Validation

#[test_case(IngredientListEvent::Select(id(2)); "select hidden record")]
#[test_case(IngredientListEvent::Edit(id(2)); "edit hidden record")]
#[test_case(IngredientListEvent::Select(id(3)); "select missing record")]
#[test_case(IngredientListEvent::Edit(id(3)); "edit missing record")]
#[tokio::test]
async fn choosing_a_hidden_or_missing_record_does_nothing(
    event: IngredientListEvent,
) -> anyhow::Result<()> {
    let mut fixture = ListFixture::ready(
        vec![ingredient(1, "Garlic")?, ingredient(2, "Basil")?],
        "gar",
    )
    .await?;
    let before = fixture.list.view();
    let mut command = fixture
        .list
        .update(Message::Event(event))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.list.view(), before);
    command.step().await?.expect_done();
    Ok(())
}

#[test_case(IngredientListEvent::Select(id(1)); "select during loading")]
#[test_case(IngredientListEvent::Edit(id(1)); "edit during loading")]
#[tokio::test]
async fn choosing_a_record_before_the_catalog_arrives_does_nothing(
    event: IngredientListEvent,
) -> anyhow::Result<()> {
    let mut fixture = ListFixture::new();
    let load = fixture.list.init();
    let mut command = fixture
        .list
        .update(Message::Event(event))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.list.view(), loading_view(""));
    command.step().await?.expect_done();
    drop(load);
    Ok(())
}

#[test_case(IngredientListEvent::Select(id(1)), |ingredient| IngredientListOutcome::OpenDetail { ingredient }; "open detail during refresh")]
#[test_case(IngredientListEvent::Edit(id(1)), |ingredient| IngredientListOutcome::OpenEdit { ingredient }; "open editor during refresh")]
#[tokio::test]
async fn cached_records_remain_available_while_refresh_is_pending(
    event: IngredientListEvent,
    expected: fn(Ingredient) -> IngredientListOutcome,
) -> anyhow::Result<()> {
    let expected = expected(ingredient(1, "Garlic")?);
    let mut fixture = ListFixture::ready(vec![ingredient(1, "Garlic")?], "gar").await?;
    let refresh = fixture
        .list
        .update(Message::Event(IngredientListEvent::Refresh));
    assert_eq!(
        fixture.list.view(),
        ready_view(
            "gar",
            &[ingredient(1, "Garlic")?],
            1,
            IngredientListRefresh::Loading
        )
    );
    let before = fixture.list.view();
    let mut command = fixture
        .list
        .update(Message::Event(event))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.list.view(), before);
    let outcome = command.step().await?.expect_outcome();
    assert_eq!(outcome, expected);
    command.step().await?.expect_done();
    drop(refresh);
    Ok(())
}

#[tokio::test]
async fn refresh_cancels_an_initial_load_that_has_not_started() -> anyhow::Result<()> {
    let mut fixture = ListFixture::new();
    let initial = fixture.list.init();
    let refresh = fixture
        .list
        .update(Message::Event(IngredientListEvent::Refresh));
    assert_eq!(fixture.list.view(), loading_view(""));
    let mut cancelled = initial.driver(&mut fixture.effects);
    cancelled.step().await?.expect_done();
    drop(cancelled);

    let mut command = refresh.driver(&mut fixture.effects);
    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    let stored = ingredients(&[(1, "Basil")])?;
    command
        .step()
        .await?
        .expect_effect()
        .expect_store()
        .resolve_list(|| async move { Ok(stored) })
        .await?;
    let outcome = command.step().await?.expect_outcome();
    command.step().await?.expect_done();
    drop(command);

    let mut render = fixture
        .list
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.list.view(),
        ready_view(
            "",
            &[ingredient(1, "Basil")?],
            1,
            IngredientListRefresh::Idle
        )
    );
    render
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    render.step().await?.expect_done();
    drop(render);
    Ok(())
}

#[tokio::test]
async fn a_second_refresh_cancels_the_first_before_it_reads_the_store() -> anyhow::Result<()> {
    let mut fixture = ListFixture::ready(vec![ingredient(1, "Basil")?], "").await?;
    let mut first = fixture
        .list
        .update(Message::Event(IngredientListEvent::Refresh))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.list.view(),
        ready_view(
            "",
            &[ingredient(1, "Basil")?],
            1,
            IngredientListRefresh::Loading
        )
    );

    first
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    let replacement = fixture
        .list
        .update(Message::Event(IngredientListEvent::Refresh));
    assert_eq!(
        fixture.list.view(),
        ready_view(
            "",
            &[ingredient(1, "Basil")?],
            1,
            IngredientListRefresh::Loading
        )
    );
    first.step().await?.expect_done();
    drop(first);

    let mut command = replacement.driver(&mut fixture.effects);
    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    let stored = ingredients(&[(2, "Peas")])?;
    command
        .step()
        .await?
        .expect_effect()
        .expect_store()
        .resolve_list(|| async move { Ok(stored) })
        .await?;
    let outcome = command.step().await?.expect_outcome();
    command.step().await?.expect_done();
    drop(command);

    let mut render = fixture
        .list
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.list.view(),
        ready_view(
            "",
            &[ingredient(2, "Peas")?],
            1,
            IngredientListRefresh::Idle
        )
    );
    render
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    render.step().await?.expect_done();
    drop(render);
    Ok(())
}

#[test_case(Ok(&[(2, "Garlic")]); "stale success")]
#[test_case(Err(IngredientLoadError::Storage); "stale storage failure")]
#[test_case(Err(IngredientLoadError::Transport); "stale transport failure")]
#[tokio::test]
async fn a_queued_refresh_result_is_ignored_after_a_newer_refresh_starts(
    result: Result<&[(u128, &str)], IngredientLoadError>,
) -> anyhow::Result<()> {
    let result = match result {
        Ok(records) => Ok(ingredients(records)?),
        Err(error) => Err(error),
    };
    let mut fixture = ListFixture::ready(vec![ingredient(1, "Basil")?], "").await?;
    let mut first = fixture
        .list
        .update(Message::Event(IngredientListEvent::Refresh))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.list.view(),
        ready_view(
            "",
            &[ingredient(1, "Basil")?],
            1,
            IngredientListRefresh::Loading
        )
    );

    first
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    first
        .step()
        .await?
        .expect_effect()
        .expect_store()
        .resolve_list(|| async { result })
        .await?;
    let queued = first.step().await?.expect_outcome();
    first.step().await?.expect_done();
    drop(first);

    let newest = fixture
        .list
        .update(Message::Event(IngredientListEvent::Refresh));
    let refreshing = ready_view(
        "",
        &[ingredient(1, "Basil")?],
        1,
        IngredientListRefresh::Loading,
    );
    assert_eq!(fixture.list.view(), refreshing);
    let mut stale = fixture
        .list
        .update(Message::Outcome(queued))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.list.view(), refreshing);
    stale.step().await?.expect_done();
    drop(stale);

    let mut command = newest.driver(&mut fixture.effects);
    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    let stored = ingredients(&[(3, "Peas")])?;
    command
        .step()
        .await?
        .expect_effect()
        .expect_store()
        .resolve_list(|| async move { Ok(stored) })
        .await?;
    let outcome = command.step().await?.expect_outcome();
    command.step().await?.expect_done();
    drop(command);

    let mut render = fixture
        .list
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.list.view(),
        ready_view(
            "",
            &[ingredient(3, "Peas")?],
            1,
            IngredientListRefresh::Idle
        )
    );
    render
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    render.step().await?.expect_done();
    drop(render);
    Ok(())
}

#[tokio::test]
async fn dispatching_a_load_result_twice_does_not_render_again() -> anyhow::Result<()> {
    let mut fixture = ListFixture::new();
    let mut command = fixture.list.init().driver(&mut fixture.effects);
    assert_eq!(fixture.list.view(), loading_view(""));
    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    let stored = ingredients(&[(1, "Basil")])?;
    command
        .step()
        .await?
        .expect_effect()
        .expect_store()
        .resolve_list(|| async move { Ok(stored) })
        .await?;
    let outcome = command.step().await?.expect_outcome();
    command.step().await?.expect_done();
    drop(command);

    let mut render = fixture
        .list
        .update(Message::Outcome(outcome.clone()))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.list.view(),
        ready_view(
            "",
            &[ingredient(1, "Basil")?],
            1,
            IngredientListRefresh::Idle
        )
    );
    render
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    render.step().await?.expect_done();
    drop(render);

    let before = fixture.list.view();
    let mut duplicate = fixture
        .list
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.list.view(), before);
    duplicate.step().await?.expect_done();
    Ok(())
}

#[tokio::test]
async fn dropping_the_widget_cancels_the_load_before_the_store_request() -> anyhow::Result<()> {
    let mut fixture = ListFixture::new();
    let mut command = fixture.list.init().driver(&mut fixture.effects);
    assert_eq!(fixture.list.view(), loading_view(""));
    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    drop(fixture.list);
    command.step().await?.expect_done();
    Ok(())
}

#[tokio::test]
async fn refresh_cancels_an_in_flight_store_request_and_accepts_only_the_replacement()
-> anyhow::Result<()> {
    let mut fixture = ListFixture::ready(vec![ingredient(1, "Basil")?], "").await?;
    let mut first = fixture
        .list
        .update(Message::Event(IngredientListEvent::Refresh))
        .driver(&mut fixture.effects);
    let refreshing = ready_view(
        "",
        &[ingredient(1, "Basil")?],
        1,
        IngredientListRefresh::Loading,
    );
    assert_eq!(fixture.list.view(), refreshing);
    first
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    let request = first.step().await?.expect_effect().expect_store();

    let replacement = fixture
        .list
        .update(Message::Event(IngredientListEvent::Refresh));
    assert_eq!(fixture.list.view(), refreshing);
    first.step().await?.expect_done();
    drop(first);
    let stored = ingredients(&[(2, "Garlic")])?;
    assert!(matches!(
        request.resolve_list(|| async move { Ok(stored) }).await,
        Err(EnvelopeError::Send(_))
    ));

    let mut command = replacement.driver(&mut fixture.effects);
    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    let stored = ingredients(&[(3, "Peas")])?;
    command
        .step()
        .await?
        .expect_effect()
        .expect_store()
        .resolve_list(|| async move { Ok(stored) })
        .await?;
    let outcome = command.step().await?.expect_outcome();
    command.step().await?.expect_done();
    drop(command);

    let mut render = fixture
        .list
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.list.view(),
        ready_view(
            "",
            &[ingredient(3, "Peas")?],
            1,
            IngredientListRefresh::Idle
        )
    );
    render
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    render.step().await?.expect_done();
    Ok(())
}

// Red cases

#[test_case(IngredientLoadError::Storage; "storage failure")]
#[test_case(IngredientLoadError::Transport; "transport failure")]
#[tokio::test]
async fn an_initial_load_failure_is_typed_and_refresh_can_recover(
    error: IngredientLoadError,
) -> anyhow::Result<()> {
    let mut fixture = ListFixture::new();
    let mut command = fixture.list.init().driver(&mut fixture.effects);
    assert_eq!(fixture.list.view(), loading_view(""));
    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command
        .step()
        .await?
        .expect_effect()
        .expect_store()
        .resolve_list(|| async { Err(error.clone()) })
        .await?;
    let outcome = command.step().await?.expect_outcome();
    command.step().await?.expect_done();
    drop(command);

    let mut render = fixture
        .list
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.list.view(),
        IngredientListView {
            search: String::new(),
            status: IngredientListStatus::Failed(error)
        }
    );
    render
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    render.step().await?.expect_done();
    drop(render);

    let mut retry = fixture
        .list
        .update(Message::Event(IngredientListEvent::Refresh))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.list.view(), loading_view(""));

    retry
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    let stored = ingredients(&[(1, "Basil")])?;
    retry
        .step()
        .await?
        .expect_effect()
        .expect_store()
        .resolve_list(|| async move { Ok(stored) })
        .await?;
    let outcome = retry.step().await?.expect_outcome();
    retry.step().await?.expect_done();
    drop(retry);

    let mut render = fixture
        .list
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.list.view(),
        ready_view(
            "",
            &[ingredient(1, "Basil")?],
            1,
            IngredientListRefresh::Idle
        )
    );
    render
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    render.step().await?.expect_done();
    drop(render);
    Ok(())
}

#[test_case(IngredientLoadError::Storage; "storage failure")]
#[test_case(IngredientLoadError::Transport; "transport failure")]
#[tokio::test]
async fn a_refresh_failure_keeps_cached_rows_and_retry_replaces_them(
    error: IngredientLoadError,
) -> anyhow::Result<()> {
    let mut fixture = ListFixture::ready(
        vec![ingredient(1, "Garlic")?, ingredient(2, "Basil")?],
        "gar",
    )
    .await?;
    let mut command = fixture
        .list
        .update(Message::Event(IngredientListEvent::Refresh))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.list.view(),
        ready_view(
            "gar",
            &[ingredient(1, "Garlic")?],
            2,
            IngredientListRefresh::Loading
        )
    );

    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command
        .step()
        .await?
        .expect_effect()
        .expect_store()
        .resolve_list(|| async { Err(error.clone()) })
        .await?;
    let outcome = command.step().await?.expect_outcome();
    command.step().await?.expect_done();
    drop(command);

    let mut render = fixture
        .list
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.list.view(),
        ready_view(
            "gar",
            &[ingredient(1, "Garlic")?],
            2,
            IngredientListRefresh::Failed(error)
        )
    );
    render
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    render.step().await?.expect_done();
    drop(render);

    let mut retry = fixture
        .list
        .update(Message::Event(IngredientListEvent::Refresh))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.list.view(),
        ready_view(
            "gar",
            &[ingredient(1, "Garlic")?],
            2,
            IngredientListRefresh::Loading
        )
    );

    retry
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    let stored = ingredients(&[(3, "Garden pea")])?;
    retry
        .step()
        .await?
        .expect_effect()
        .expect_store()
        .resolve_list(|| async move { Ok(stored) })
        .await?;
    let outcome = retry.step().await?.expect_outcome();
    retry.step().await?.expect_done();
    drop(retry);

    let mut render = fixture
        .list
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.list.view(),
        ready_view(
            "gar",
            &[ingredient(3, "Garden pea")?],
            1,
            IngredientListRefresh::Idle
        )
    );
    render
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    render.step().await?.expect_done();
    drop(render);
    Ok(())
}

#[test_matrix(
    [IngredientLoadError::Storage, IngredientLoadError::Transport],
    [
        (IngredientListEvent::Select(id(1)), |ingredient| IngredientListOutcome::OpenDetail { ingredient }),
        (IngredientListEvent::Edit(id(1)), |ingredient| IngredientListOutcome::OpenEdit { ingredient }),
        (IngredientListEvent::CreateRequested, |_| IngredientListOutcome::CreateRequested)
    ]
)]
#[tokio::test]
async fn a_refresh_failure_keeps_cached_navigation_and_creation_available(
    error: IngredientLoadError,
    (event, expected): (IngredientListEvent, fn(Ingredient) -> IngredientListOutcome),
) -> anyhow::Result<()> {
    let expected = expected(ingredient(1, "Garlic")?);
    let mut fixture = ListFixture::ready(vec![ingredient(1, "Garlic")?], "").await?;
    let mut refresh = fixture
        .list
        .update(Message::Event(IngredientListEvent::Refresh))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.list.view(),
        ready_view(
            "",
            &[ingredient(1, "Garlic")?],
            1,
            IngredientListRefresh::Loading
        )
    );

    refresh
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    refresh
        .step()
        .await?
        .expect_effect()
        .expect_store()
        .resolve_list(|| async { Err(error.clone()) })
        .await?;
    let outcome = refresh.step().await?.expect_outcome();
    refresh.step().await?.expect_done();
    drop(refresh);

    let mut render = fixture
        .list
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.list.view(),
        ready_view(
            "",
            &[ingredient(1, "Garlic")?],
            1,
            IngredientListRefresh::Failed(error)
        )
    );
    render
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    render.step().await?.expect_done();
    drop(render);

    let before = fixture.list.view();
    let mut command = fixture
        .list
        .update(Message::Event(event))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.list.view(), before);
    let outcome = command.step().await?.expect_outcome();
    assert_eq!(outcome, expected);
    command.step().await?.expect_done();
    Ok(())
}

#[test_case(IngredientLoadError::Storage; "storage failure")]
#[test_case(IngredientLoadError::Transport; "transport failure")]
#[tokio::test]
async fn creation_is_available_after_the_initial_load_fails(
    error: IngredientLoadError,
) -> anyhow::Result<()> {
    let mut fixture = ListFixture::new();
    let mut load = fixture.list.init().driver(&mut fixture.effects);
    assert_eq!(fixture.list.view(), loading_view(""));
    load.step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    load.step()
        .await?
        .expect_effect()
        .expect_store()
        .resolve_list(|| async { Err(error.clone()) })
        .await?;
    let outcome = load.step().await?.expect_outcome();
    load.step().await?.expect_done();
    drop(load);

    let mut render = fixture
        .list
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.list.view(),
        IngredientListView {
            search: String::new(),
            status: IngredientListStatus::Failed(error)
        }
    );
    render
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    render.step().await?.expect_done();
    drop(render);

    let before = fixture.list.view();
    let mut command = fixture
        .list
        .update(Message::Event(IngredientListEvent::CreateRequested))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.list.view(), before);
    let outcome = command.step().await?.expect_outcome();
    assert_eq!(outcome, IngredientListOutcome::CreateRequested);
    command.step().await?.expect_done();
    Ok(())
}

#[tokio::test]
async fn a_dropped_initial_store_response_exposes_a_transport_failure() -> anyhow::Result<()> {
    let mut fixture = ListFixture::new();
    let mut command = fixture.list.init().driver(&mut fixture.effects);
    assert_eq!(fixture.list.view(), loading_view(""));
    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    drop(command.step().await?.expect_effect().expect_store());
    let outcome = command.step().await?.expect_outcome();
    let IngredientListOutcome::LoadFinished(Latest { value: result, .. }) = &outcome else {
        anyhow::bail!("expected a completed load, got {outcome:?}");
    };
    assert_eq!(result, &Err(IngredientLoadError::Transport));
    command.step().await?.expect_done();
    drop(command);

    let mut render = fixture
        .list
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.list.view(),
        IngredientListView {
            search: String::new(),
            status: IngredientListStatus::Failed(IngredientLoadError::Transport)
        }
    );
    render
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    render.step().await?.expect_done();
    Ok(())
}

#[tokio::test]
async fn a_dropped_refresh_response_keeps_cached_rows_and_exposes_a_transport_failure()
-> anyhow::Result<()> {
    let mut fixture = ListFixture::ready(vec![ingredient(1, "Garlic")?], "gar").await?;
    let mut command = fixture
        .list
        .update(Message::Event(IngredientListEvent::Refresh))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.list.view(),
        ready_view(
            "gar",
            &[ingredient(1, "Garlic")?],
            1,
            IngredientListRefresh::Loading
        )
    );
    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    drop(command.step().await?.expect_effect().expect_store());
    let outcome = command.step().await?.expect_outcome();
    command.step().await?.expect_done();
    drop(command);

    let mut render = fixture
        .list
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.list.view(),
        ready_view(
            "gar",
            &[ingredient(1, "Garlic")?],
            1,
            IngredientListRefresh::Failed(IngredientLoadError::Transport)
        )
    );
    render
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    render.step().await?.expect_done();
    Ok(())
}

// Test support

fn id(value: u128) -> IngredientId {
    Uuid::from_u128(value).into()
}

/// A complete record with nutrition and one unit, so tests see that rows
/// and outcomes carry every field.
fn ingredient(value: u128, name: &str) -> anyhow::Result<Ingredient> {
    Ok(Ingredient {
        id: id(value),
        name: IngredientName::try_new(name)?,
        notes: Some("complete record".into()),
        nutrition: Nutrition {
            basis: NutritionBasis::Per100Grams,
            calories: Some(Kilocalories::try_new(Decimal::from(149))?),
            protein: Some(Grams::try_new(Decimal::from(6))?),
            carbohydrates: None,
            fat: Some(Grams::try_new(Decimal::ZERO)?),
        },
        units: vec![ReferenceUnit {
            name: UnitName::try_new("clove")?,
            grams: Some(Grams::try_new(Decimal::from(3))?),
            millilitres: None,
        }],
    })
}

/// `ingredient` for each `(id, name)` pair, in order.
fn ingredients(records: &[(u128, &str)]) -> anyhow::Result<Vec<Ingredient>> {
    records
        .iter()
        .map(|&(value, name)| ingredient(value, name))
        .collect()
}

fn loading_view(search: &str) -> IngredientListView {
    IngredientListView {
        search: search.into(),
        status: IngredientListStatus::Loading,
    }
}

fn ready_view(
    search: &str,
    visible: &[Ingredient],
    total_count: usize,
    refresh: IngredientListRefresh,
) -> IngredientListView {
    IngredientListView {
        search: search.into(),
        status: IngredientListStatus::Ready {
            rows: visible
                .iter()
                .map(|ingredient| IngredientListRow {
                    id: ingredient.id,
                    name: ingredient.name.to_string(),
                })
                .collect(),
            total_count,
            refresh,
        },
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

    async fn ready(records: Vec<Ingredient>, search: &str) -> anyhow::Result<Self> {
        let mut fixture = Self::new();
        let mut load = fixture.list.init().driver(&mut fixture.effects);
        load.step()
            .await?
            .expect_effect()
            .expect_render()
            .resolve_render(|| async {})
            .await?;
        load.step()
            .await?
            .expect_effect()
            .expect_store()
            .resolve_list(|| async { Ok(records) })
            .await?;
        let outcome = load.step().await?.expect_outcome();
        load.step().await?.expect_done();
        drop(load);

        let mut render = fixture
            .list
            .update(Message::Outcome(outcome))
            .driver(&mut fixture.effects);
        render
            .step()
            .await?
            .expect_effect()
            .expect_render()
            .resolve_render(|| async {})
            .await?;
        render.step().await?.expect_done();
        drop(render);
        let mut command = fixture
            .list
            .update(Message::Event(IngredientListEvent::Search(search.into())))
            .driver(&mut fixture.effects);
        command
            .step()
            .await?
            .expect_effect()
            .expect_render()
            .resolve_render(|| async {})
            .await?;
        command.step().await?.expect_done();
        drop(command);
        Ok(fixture)
    }
}
