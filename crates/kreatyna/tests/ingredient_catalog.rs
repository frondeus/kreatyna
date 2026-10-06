use kreatyna::{
    capabilities::ResolveRender,
    confirmation::{ConfirmationEvent, ConfirmationOutcome, ConfirmationView},
    ingredients::{
        capabilities::{
            IngredientDeleteError, IngredientEffect, IngredientEffectStream, IngredientLoadError,
            IngredientSaveError, ResolveIngredientStore,
        },
        *,
    },
};
use krucyfiks::{
    CommandScopeId, Message, Scoped, Widget,
    effects::{CommandDriver, CommandTestExt},
};
use test_case::test_case;
use uuid::Uuid;

// Ordinary behavior

#[tokio::test]
async fn initialization_loads_the_list_in_the_current_scope() -> anyhow::Result<()> {
    let mut fixture = CatalogFixture::new();
    let scope = fixture.scope();
    let mut command = fixture.catalog.init().driver(&mut fixture.effects);
    assert_eq!(
        fixture.catalog.view(),
        IngredientCatalogView {
            navigation: IngredientNavigation::List,
            list: IngredientListView {
                search: String::new(),
                status: IngredientListStatus::Loading,
            },
            scope,
            screen: IngredientCatalogScreenView::List,
        }
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
        .resolve_list(|| async { Ok(vec![ingredient(1, "Garlic")]) })
        .await?;
    let outcome = command.step().await?.expect_outcome();
    let IngredientCatalogOutcome::List(Scoped {
        scope: outcome_scope,
        value: IngredientListOutcome::LoadFinished { .. },
    }) = &outcome
    else {
        panic!("expected a list load result, got {outcome:?}");
    };
    assert_eq!(*outcome_scope, scope);
    command.step().await?.expect_done();
    drop(command);

    let mut command = fixture
        .catalog
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    let view = fixture.catalog.view();
    assert_eq!(row_names(&view.list), ["Garlic"]);
    assert_eq!(view.scope, scope);
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

#[test_case(
    IngredientListEvent::Select(id(1)),
    IngredientListOutcome::OpenDetail { ingredient: ingredient(1, "Garlic") },
    IngredientNavigation::Detail { id: id(1) };
    "select opens detail"
)]
#[test_case(
    IngredientListEvent::Edit(id(1)),
    IngredientListOutcome::OpenEdit { ingredient: ingredient(1, "Garlic") },
    IngredientNavigation::Edit { id: id(1), origin: IngredientEditOrigin::List };
    "edit opens the form from the list"
)]
#[test_case(
    IngredientListEvent::CreateRequested,
    IngredientListOutcome::CreateRequested,
    IngredientNavigation::Create;
    "create opens an empty form"
)]
#[tokio::test]
async fn choosing_from_the_list_opens_the_screen_in_a_new_scope(
    event: IngredientListEvent,
    expected_outcome: IngredientListOutcome,
    expected_navigation: IngredientNavigation,
) -> anyhow::Result<()> {
    let mut fixture = CatalogFixture::ready(vec![ingredient(1, "Garlic")], "").await?;
    let list_scope = fixture.scope();
    let before = fixture.catalog.view();
    let mut command = fixture
        .catalog
        .update(Message::Event(list_event(list_scope, event)))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.catalog.view(), before);
    let outcome = command.step().await?.expect_outcome();
    assert_eq!(
        outcome,
        IngredientCatalogOutcome::List(Scoped::new(list_scope, expected_outcome))
    );
    command.step().await?.expect_done();
    drop(command);

    let mut command = fixture
        .catalog
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    let view = fixture.catalog.view();
    assert_eq!(view.navigation, expected_navigation);
    assert_ne!(view.scope, list_scope);
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

#[tokio::test]
async fn editing_from_the_detail_keeps_the_detail_behind_the_form() -> anyhow::Result<()> {
    let garlic = ingredient(1, "Garlic");
    let mut fixture = CatalogFixture::ready(vec![garlic.clone()], "").await?;
    fixture.open_detail(garlic.id).await?;
    let detail_scope = fixture.scope();
    let mut command = fixture
        .catalog
        .update(Message::Event(detail_event(
            detail_scope,
            IngredientDetailEvent::Edit,
        )))
        .driver(&mut fixture.effects);
    let outcome = command.step().await?.expect_outcome();
    assert_eq!(
        outcome,
        IngredientCatalogOutcome::Detail(Scoped::new(
            detail_scope,
            IngredientDetailOutcome::OpenEdit {
                ingredient: garlic.clone()
            }
        ))
    );
    command.step().await?.expect_done();
    drop(command);

    let mut command = fixture
        .catalog
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    let view = fixture.catalog.view();
    assert_eq!(
        view.navigation,
        IngredientNavigation::Edit {
            id: garlic.id,
            origin: IngredientEditOrigin::Detail
        }
    );
    assert_ne!(view.scope, detail_scope);
    let IngredientCatalogScreenView::EditFromDetail { detail, form } = view.screen else {
        panic!("expected the form above the detail, got {:?}", view.screen);
    };
    assert_eq!(detail, idle_detail(&garlic));
    assert_eq!(form.draft, IngredientDraft::from(&garlic));
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

#[tokio::test]
async fn back_from_the_detail_returns_to_the_list_with_its_search_and_without_reloading()
-> anyhow::Result<()> {
    let mut fixture =
        CatalogFixture::ready(vec![ingredient(1, "Garlic"), ingredient(2, "Basil")], "gar").await?;
    let list = fixture.catalog.view().list;
    fixture.open_detail(id(1)).await?;
    let detail_scope = fixture.scope();

    let mut command = fixture
        .catalog
        .update(Message::Event(back(detail_scope)))
        .driver(&mut fixture.effects);
    let view = fixture.catalog.view();
    assert_eq!(view.navigation, IngredientNavigation::List);
    assert_eq!(view.screen, IngredientCatalogScreenView::List);
    assert_eq!(view.list, list);
    assert_ne!(view.scope, detail_scope);

    // Only a render follows. The cached list is not loaded again.
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

#[test_case(Origin::Create, IngredientNavigation::List, IngredientCatalogScreenView::List; "create returns to the list")]
#[test_case(Origin::List, IngredientNavigation::List, IngredientCatalogScreenView::List; "list edit returns to the list")]
#[test_case(Origin::Detail, IngredientNavigation::Detail { id: id(1) }, IngredientCatalogScreenView::Detail(idle_detail(&ingredient(1, "Garlic"))); "detail edit returns to the detail")]
#[tokio::test]
async fn closing_a_clean_form_returns_to_its_origin(
    origin: Origin,
    expected_navigation: IngredientNavigation,
    expected_screen: IngredientCatalogScreenView,
) -> anyhow::Result<()> {
    let mut fixture = CatalogFixture::ready(vec![ingredient(1, "Garlic")], "").await?;
    fixture.open_form(origin, id(1)).await?;
    let form_scope = fixture.scope();

    let mut command = fixture
        .catalog
        .update(Message::Event(back(form_scope)))
        .driver(&mut fixture.effects);
    assert_eq!(
        active_form(&fixture.catalog.view().screen).status,
        IngredientFormStatus::Finished
    );
    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    let outcome = command.step().await?.expect_outcome();
    assert_eq!(
        outcome,
        IngredientCatalogOutcome::Form(Scoped::new(form_scope, IngredientFormOutcome::Discarded))
    );
    command.step().await?.expect_done();
    drop(command);

    let mut command = fixture
        .catalog
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    let view = fixture.catalog.view();
    assert_eq!(view.navigation, expected_navigation);
    assert_eq!(view.screen, expected_screen);
    assert_ne!(view.scope, form_scope);
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

#[tokio::test]
async fn back_on_a_dirty_form_asks_for_confirmation_and_discarding_returns_to_the_list()
-> anyhow::Result<()> {
    let mut fixture = CatalogFixture::ready(vec![], "").await?;
    fixture.open_create().await?;
    let scope = fixture.scope();
    fixture
        .apply(form_event(
            scope,
            field(IngredientDraftField::Name, "Garlic"),
        ))
        .await?;

    let mut command = fixture
        .catalog
        .update(Message::Event(back(scope)))
        .driver(&mut fixture.effects);
    let view = fixture.catalog.view();
    assert_eq!(view.navigation, IngredientNavigation::Create);
    assert_eq!(
        active_form(&view.screen).status,
        IngredientFormStatus::ConfirmingDiscard(ConfirmationView::AwaitingDecision)
    );
    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command.step().await?.expect_done();
    drop(command);

    let decision = fixture
        .choose(form_event(
            scope,
            IngredientFormEvent::Confirmation(ConfirmationEvent::Confirm),
        ))
        .await?;
    let mut command = fixture
        .catalog
        .update(Message::Outcome(decision))
        .driver(&mut fixture.effects);
    assert_eq!(
        active_form(&fixture.catalog.view().screen).status,
        IngredientFormStatus::Finished
    );
    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    let discarded = command.step().await?.expect_outcome();
    assert_eq!(
        discarded,
        IngredientCatalogOutcome::Form(Scoped::new(scope, IngredientFormOutcome::Discarded))
    );
    command.step().await?.expect_done();
    drop(command);

    let mut command = fixture
        .catalog
        .update(Message::Outcome(discarded))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.catalog.view().navigation,
        IngredientNavigation::List
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

#[tokio::test]
async fn back_during_delete_confirmation_cancels_it_and_stays_on_the_detail() -> anyhow::Result<()>
{
    let garlic = ingredient(1, "Garlic");
    let mut fixture = CatalogFixture::ready(vec![garlic.clone()], "").await?;
    fixture.open_detail(garlic.id).await?;
    let scope = fixture.scope();
    fixture
        .apply(detail_event(scope, IngredientDetailEvent::RequestDelete))
        .await?;

    let mut command = fixture
        .catalog
        .update(Message::Event(back(scope)))
        .driver(&mut fixture.effects);
    let view = fixture.catalog.view();
    assert_eq!(
        view.navigation,
        IngredientNavigation::Detail { id: garlic.id }
    );
    assert_eq!(
        view.screen,
        IngredientCatalogScreenView::Detail(IngredientDetailView {
            ingredient: garlic.clone(),
            deletion: IngredientDetailDeletion::Confirming {
                confirmation: ConfirmationView::Resolved {
                    decision: ConfirmationOutcome::Cancelled
                },
                error: None,
            },
        })
    );
    let decision = command.step().await?.expect_outcome();
    command.step().await?.expect_done();
    drop(command);

    let mut command = fixture
        .catalog
        .update(Message::Outcome(decision))
        .driver(&mut fixture.effects);
    let view = fixture.catalog.view();
    assert_eq!(
        view.navigation,
        IngredientNavigation::Detail { id: garlic.id }
    );
    assert_eq!(
        view.screen,
        IngredientCatalogScreenView::Detail(idle_detail(&garlic))
    );
    assert_eq!(view.scope, scope);
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

#[tokio::test]
async fn saving_a_new_ingredient_adds_it_to_the_cached_list_and_returns_to_the_list()
-> anyhow::Result<()> {
    let mut fixture = CatalogFixture::ready(vec![ingredient(1, "Basil")], "").await?;
    fixture.open_create().await?;
    let scope = fixture.scope();
    fixture
        .apply(form_event(
            scope,
            field(IngredientDraftField::Name, "Garlic"),
        ))
        .await?;
    let saved = fixture.finish_save(ingredient(2, "Garlic")).await?;

    let mut command = fixture
        .catalog
        .update(Message::Outcome(saved))
        .driver(&mut fixture.effects);
    let view = fixture.catalog.view();
    assert_eq!(view.navigation, IngredientNavigation::List);
    assert_eq!(view.screen, IngredientCatalogScreenView::List);
    assert_ne!(view.scope, scope);
    assert_eq!(row_names(&view.list), ["Basil", "Garlic"]);

    // Only a render follows. The cache is updated without a reload.
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

#[tokio::test]
async fn saving_an_edit_from_the_list_replaces_the_cached_record_and_returns_to_the_list()
-> anyhow::Result<()> {
    let garlic = ingredient(1, "Garlic");
    let mut fixture =
        CatalogFixture::ready(vec![garlic.clone(), ingredient(2, "Basil")], "").await?;
    fixture.open_edit_from_list(garlic.id).await?;
    let scope = fixture.scope();
    fixture
        .apply(form_event(
            scope,
            field(IngredientDraftField::Name, "Roasted garlic"),
        ))
        .await?;
    let saved = fixture.finish_save(ingredient(1, "Roasted garlic")).await?;

    let mut command = fixture
        .catalog
        .update(Message::Outcome(saved))
        .driver(&mut fixture.effects);
    let view = fixture.catalog.view();
    assert_eq!(view.navigation, IngredientNavigation::List);
    assert_eq!(view.screen, IngredientCatalogScreenView::List);
    assert_eq!(row_names(&view.list), ["Basil", "Roasted garlic"]);

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

#[tokio::test]
async fn saving_an_edit_from_the_detail_opens_a_fresh_detail_with_the_saved_record()
-> anyhow::Result<()> {
    let garlic = ingredient(1, "Garlic");
    let mut fixture = CatalogFixture::ready(vec![garlic.clone()], "").await?;
    fixture.open_detail(garlic.id).await?;
    fixture.open_edit_from_detail().await?;
    let scope = fixture.scope();
    fixture
        .apply(form_event(
            scope,
            field(IngredientDraftField::Name, "Roasted garlic"),
        ))
        .await?;
    let roasted = ingredient(1, "Roasted garlic");
    let saved = fixture.finish_save(roasted.clone()).await?;

    let mut command = fixture
        .catalog
        .update(Message::Outcome(saved))
        .driver(&mut fixture.effects);
    let view = fixture.catalog.view();
    assert_eq!(
        view.navigation,
        IngredientNavigation::Detail { id: garlic.id }
    );
    assert_eq!(
        view.screen,
        IngredientCatalogScreenView::Detail(idle_detail(&roasted))
    );
    assert_ne!(view.scope, scope);
    // The list behind the detail is updated too.
    assert_eq!(row_names(&view.list), ["Roasted garlic"]);

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

#[tokio::test]
async fn deleting_from_the_detail_removes_the_cached_record_and_returns_to_the_list()
-> anyhow::Result<()> {
    let garlic = ingredient(1, "Garlic");
    let mut fixture =
        CatalogFixture::ready(vec![garlic.clone(), ingredient(2, "Basil")], "").await?;
    fixture.open_detail(garlic.id).await?;
    let scope = fixture.scope();
    let deleted = fixture.finish_delete().await?;
    assert_eq!(
        deleted,
        IngredientCatalogOutcome::Detail(Scoped::new(
            scope,
            IngredientDetailOutcome::Deleted { id: garlic.id }
        ))
    );

    let mut command = fixture
        .catalog
        .update(Message::Outcome(deleted))
        .driver(&mut fixture.effects);
    let view = fixture.catalog.view();
    assert_eq!(view.navigation, IngredientNavigation::List);
    assert_eq!(view.screen, IngredientCatalogScreenView::List);
    assert_ne!(view.scope, scope);
    assert_eq!(row_names(&view.list), ["Basil"]);

    // Only a render follows. The cache is updated without a reload.
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

// Edge cases and guards

#[tokio::test]
async fn a_list_event_from_before_the_detail_opened_is_ignored() -> anyhow::Result<()> {
    let mut fixture = CatalogFixture::ready(vec![ingredient(1, "Garlic")], "").await?;
    let list_scope = fixture.scope();
    fixture.open_detail(id(1)).await?;

    let before = fixture.catalog.view();
    let mut command = fixture
        .catalog
        .update(Message::Event(list_event(
            list_scope,
            IngredientListEvent::CreateRequested,
        )))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.catalog.view(), before);
    command.step().await?.expect_done();
    Ok(())
}

#[tokio::test]
async fn a_detail_event_after_returning_to_the_list_is_ignored() -> anyhow::Result<()> {
    let mut fixture = CatalogFixture::ready(vec![ingredient(1, "Garlic")], "").await?;
    fixture.open_detail(id(1)).await?;
    let detail_scope = fixture.scope();
    fixture.apply(back(detail_scope)).await?;

    let before = fixture.catalog.view();
    let mut command = fixture
        .catalog
        .update(Message::Event(detail_event(
            detail_scope,
            IngredientDetailEvent::Edit,
        )))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.catalog.view(), before);
    command.step().await?.expect_done();
    Ok(())
}

#[tokio::test]
async fn back_from_a_previous_screen_is_ignored() -> anyhow::Result<()> {
    let mut fixture = CatalogFixture::ready(vec![ingredient(1, "Garlic")], "").await?;
    let list_scope = fixture.scope();
    fixture.open_detail(id(1)).await?;

    let before = fixture.catalog.view();
    let mut command = fixture
        .catalog
        .update(Message::Event(back(list_scope)))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.catalog.view(), before);
    command.step().await?.expect_done();
    Ok(())
}

#[tokio::test]
async fn a_load_result_arriving_after_another_screen_opened_is_ignored() -> anyhow::Result<()> {
    let mut fixture = CatalogFixture::new();
    let mut load = fixture.catalog.init().driver(&mut fixture.effects);
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
        .resolve_list(|| async { Ok(vec![ingredient(1, "Garlic")]) })
        .await?;
    let stale = load.step().await?.expect_outcome();
    load.step().await?.expect_done();
    drop(load);
    fixture.open_create().await?;

    let before = fixture.catalog.view();
    let mut command = fixture
        .catalog
        .update(Message::Outcome(stale))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.catalog.view(), before);
    command.step().await?.expect_done();
    Ok(())
}

#[tokio::test]
async fn a_list_choice_arriving_after_another_screen_opened_is_ignored() -> anyhow::Result<()> {
    let mut fixture = CatalogFixture::ready(vec![ingredient(1, "Garlic")], "").await?;
    let list_scope = fixture.scope();
    let stale = fixture
        .choose(list_event(list_scope, IngredientListEvent::Select(id(1))))
        .await?;
    fixture.open_create().await?;

    let before = fixture.catalog.view();
    let mut command = fixture
        .catalog
        .update(Message::Outcome(stale))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.catalog.view(), before);
    command.step().await?.expect_done();
    Ok(())
}

#[tokio::test]
async fn back_on_the_list_does_nothing() -> anyhow::Result<()> {
    let mut fixture = CatalogFixture::ready(vec![ingredient(1, "Garlic")], "").await?;
    let before = fixture.catalog.view();
    let mut command = fixture
        .catalog
        .update(Message::Event(back(before.scope)))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.catalog.view(), before);
    command.step().await?.expect_done();
    Ok(())
}

#[tokio::test]
async fn back_is_ignored_while_saving() -> anyhow::Result<()> {
    let mut fixture = CatalogFixture::ready(vec![], "").await?;
    fixture.open_create().await?;
    let scope = fixture.scope();
    fixture
        .apply(form_event(
            scope,
            field(IngredientDraftField::Name, "Garlic"),
        ))
        .await?;
    let save = fixture
        .catalog
        .update(Message::Event(form_event(scope, IngredientFormEvent::Save)));
    assert_eq!(
        active_form(&fixture.catalog.view().screen).status,
        IngredientFormStatus::Saving
    );

    let before = fixture.catalog.view();
    let mut command = fixture
        .catalog
        .update(Message::Event(back(scope)))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.catalog.view(), before);
    command.step().await?.expect_done();
    drop(save);
    Ok(())
}

#[tokio::test]
async fn back_is_ignored_while_deleting() -> anyhow::Result<()> {
    let mut fixture = CatalogFixture::ready(vec![ingredient(1, "Garlic")], "").await?;
    fixture.open_detail(id(1)).await?;
    let scope = fixture.scope();
    fixture
        .apply(detail_event(scope, IngredientDetailEvent::RequestDelete))
        .await?;
    let confirmed = fixture
        .choose(detail_event(
            scope,
            IngredientDetailEvent::Confirmation(ConfirmationEvent::Confirm),
        ))
        .await?;
    let deletion = fixture.catalog.update(Message::Outcome(confirmed));
    assert!(matches!(
        fixture.catalog.view().screen,
        IngredientCatalogScreenView::Detail(IngredientDetailView {
            deletion: IngredientDetailDeletion::Deleting,
            ..
        })
    ));

    let before = fixture.catalog.view();
    let mut command = fixture
        .catalog
        .update(Message::Event(back(scope)))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.catalog.view(), before);
    command.step().await?.expect_done();
    drop(deletion);
    Ok(())
}

#[tokio::test]
async fn opening_a_screen_cancels_a_pending_refresh_and_back_restarts_it() -> anyhow::Result<()> {
    let mut fixture = CatalogFixture::ready(vec![ingredient(1, "Garlic")], "").await?;
    let list_scope = fixture.scope();
    let refresh = fixture.catalog.update(Message::Event(list_event(
        list_scope,
        IngredientListEvent::Refresh,
    )));
    fixture.open_detail(id(1)).await?;

    // The refresh stops before it reads the store.
    let mut refresh = refresh.driver(&mut fixture.effects);
    refresh.step().await?.expect_done();
    drop(refresh);

    let detail_scope = fixture.scope();
    let mut command = fixture
        .catalog
        .update(Message::Event(back(detail_scope)))
        .driver(&mut fixture.effects);
    let view = fixture.catalog.view();
    assert_eq!(view.navigation, IngredientNavigation::List);
    assert!(matches!(
        view.list.status,
        IngredientListStatus::Ready {
            refresh: IngredientListRefresh::Loading,
            ..
        }
    ));
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
        .resolve_list(|| async { Ok(vec![ingredient(1, "Garlic"), ingredient(2, "Basil")]) })
        .await?;
    let outcome = command.step().await?.expect_outcome();
    command.step().await?.expect_done();
    drop(command);

    fixture.apply_outcome(outcome).await?;
    assert_eq!(row_names(&fixture.catalog.view().list), ["Basil", "Garlic"]);
    Ok(())
}

#[tokio::test]
async fn opening_creation_during_the_initial_load_cancels_it_and_closing_resumes_it()
-> anyhow::Result<()> {
    let mut fixture = CatalogFixture::new();
    let initial = fixture.catalog.init();
    fixture.open_create().await?;

    // The initial load stops before it reads the store.
    let mut initial = initial.driver(&mut fixture.effects);
    initial.step().await?.expect_done();
    drop(initial);

    let form_scope = fixture.scope();
    let discarded = fixture.choose_after_render(back(form_scope)).await?;
    let mut command = fixture
        .catalog
        .update(Message::Outcome(discarded))
        .driver(&mut fixture.effects);
    let view = fixture.catalog.view();
    assert_eq!(view.navigation, IngredientNavigation::List);
    assert_eq!(view.list.status, IngredientListStatus::Loading);
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
        .resolve_list(|| async { Ok(vec![ingredient(1, "Garlic")]) })
        .await?;
    let outcome = command.step().await?.expect_outcome();
    command.step().await?.expect_done();
    drop(command);

    fixture.apply_outcome(outcome).await?;
    assert_eq!(row_names(&fixture.catalog.view().list), ["Garlic"]);
    Ok(())
}

#[test_case(Unavailable::Pending; "initial load still pending")]
#[test_case(Unavailable::Failed; "initial load failed")]
#[tokio::test]
async fn saving_without_a_complete_cache_reloads_the_list(
    unavailable: Unavailable,
) -> anyhow::Result<()> {
    let mut fixture = CatalogFixture::without_cache(unavailable).await?;
    fixture.open_create().await?;
    let scope = fixture.scope();
    fixture
        .apply(form_event(
            scope,
            field(IngredientDraftField::Name, "Garlic"),
        ))
        .await?;
    let saved = fixture.finish_save(ingredient(2, "Garlic")).await?;

    let mut command = fixture
        .catalog
        .update(Message::Outcome(saved))
        .driver(&mut fixture.effects);
    let view = fixture.catalog.view();
    assert_eq!(view.navigation, IngredientNavigation::List);
    // A partial list is never shown. The catalog loads it again.
    assert_eq!(view.list.status, IngredientListStatus::Loading);
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
        .resolve_list(|| async { Ok(vec![ingredient(1, "Basil"), ingredient(2, "Garlic")]) })
        .await?;
    let outcome = command.step().await?.expect_outcome();
    command.step().await?.expect_done();
    drop(command);

    fixture.apply_outcome(outcome).await?;
    assert_eq!(row_names(&fixture.catalog.view().list), ["Basil", "Garlic"]);
    Ok(())
}

#[tokio::test]
async fn dropping_the_catalog_cancels_a_pending_save() -> anyhow::Result<()> {
    let mut fixture = CatalogFixture::ready(vec![], "").await?;
    fixture.open_create().await?;
    let scope = fixture.scope();
    fixture
        .apply(form_event(
            scope,
            field(IngredientDraftField::Name, "Garlic"),
        ))
        .await?;
    let mut save = fixture
        .catalog
        .update(Message::Event(form_event(scope, IngredientFormEvent::Save)))
        .driver(&mut fixture.effects);
    assert_eq!(
        active_form(&fixture.catalog.view().screen).status,
        IngredientFormStatus::Saving
    );
    save.step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;

    drop(fixture.catalog);
    save.step().await?.expect_done();
    Ok(())
}

// Failures and recovery

#[test_case(IngredientSaveError::DuplicateName; "duplicate name")]
#[test_case(IngredientSaveError::NotFound; "missing record")]
#[test_case(IngredientSaveError::Storage; "storage failure")]
#[test_case(IngredientSaveError::Transport; "transport failure")]
#[tokio::test]
async fn a_failed_save_keeps_the_form_and_does_not_change_the_cached_list(
    error: IngredientSaveError,
) -> anyhow::Result<()> {
    let mut fixture = CatalogFixture::ready(vec![ingredient(1, "Basil")], "").await?;
    fixture.open_create().await?;
    let scope = fixture.scope();
    fixture
        .apply(form_event(
            scope,
            field(IngredientDraftField::Name, "  Garlic  "),
        ))
        .await?;
    let failed = fixture.save(Err(error.clone())).await?;

    let mut command = fixture
        .catalog
        .update(Message::Outcome(failed))
        .driver(&mut fixture.effects);
    let view = fixture.catalog.view();
    assert_eq!(view.navigation, IngredientNavigation::Create);
    assert_eq!(view.scope, scope);
    let form = active_form(&view.screen);
    assert_eq!(form.status, IngredientFormStatus::Editing);
    assert_eq!(form.save_error, Some(error));
    assert_eq!(form.draft.name, "  Garlic  ");
    assert_eq!(row_names(&view.list), ["Basil"]);
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

#[test_case(IngredientDeleteError::Storage; "storage failure")]
#[test_case(IngredientDeleteError::Transport; "transport failure")]
#[tokio::test]
async fn a_failed_delete_keeps_the_detail_and_the_cached_record(
    error: IngredientDeleteError,
) -> anyhow::Result<()> {
    let garlic = ingredient(1, "Garlic");
    let mut fixture = CatalogFixture::ready(vec![garlic.clone()], "").await?;
    fixture.open_detail(garlic.id).await?;
    let scope = fixture.scope();
    let failed = fixture.delete(Err(error.clone())).await?;

    let mut command = fixture
        .catalog
        .update(Message::Outcome(failed))
        .driver(&mut fixture.effects);
    let view = fixture.catalog.view();
    assert_eq!(
        view.navigation,
        IngredientNavigation::Detail { id: garlic.id }
    );
    assert_eq!(view.scope, scope);
    assert_eq!(
        view.screen,
        IngredientCatalogScreenView::Detail(IngredientDetailView {
            ingredient: garlic,
            deletion: IngredientDetailDeletion::Confirming {
                confirmation: ConfirmationView::AwaitingDecision,
                error: Some(error),
            },
        })
    );
    assert_eq!(row_names(&view.list), ["Garlic"]);
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

#[tokio::test]
async fn saving_after_a_failed_save_navigates_like_a_first_save() -> anyhow::Result<()> {
    let mut fixture = CatalogFixture::ready(vec![ingredient(1, "Basil")], "").await?;
    fixture.open_create().await?;
    let scope = fixture.scope();
    fixture
        .apply(form_event(
            scope,
            field(IngredientDraftField::Name, "Garlic"),
        ))
        .await?;
    let failed = fixture.save(Err(IngredientSaveError::Storage)).await?;
    fixture.apply_outcome(failed).await?;
    let saved = fixture.finish_save(ingredient(2, "Garlic")).await?;

    let mut command = fixture
        .catalog
        .update(Message::Outcome(saved))
        .driver(&mut fixture.effects);
    let view = fixture.catalog.view();
    assert_eq!(view.navigation, IngredientNavigation::List);
    assert_eq!(row_names(&view.list), ["Basil", "Garlic"]);
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

#[tokio::test]
async fn deleting_after_a_failed_delete_navigates_like_a_first_delete() -> anyhow::Result<()> {
    let mut fixture =
        CatalogFixture::ready(vec![ingredient(1, "Garlic"), ingredient(2, "Basil")], "").await?;
    fixture.open_detail(id(1)).await?;
    let failed = fixture.delete(Err(IngredientDeleteError::Storage)).await?;
    fixture.apply_outcome(failed).await?;
    // The confirmation is already open again after the failure.
    let deleted = fixture.finish_delete_from_confirmation().await?;

    let mut command = fixture
        .catalog
        .update(Message::Outcome(deleted))
        .driver(&mut fixture.effects);
    let view = fixture.catalog.view();
    assert_eq!(view.navigation, IngredientNavigation::List);
    assert_eq!(row_names(&view.list), ["Basil"]);
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

// Test support

#[derive(Debug, Clone, Copy)]
enum Origin {
    Create,
    List,
    Detail,
}

#[derive(Debug, Clone, Copy)]
enum Unavailable {
    Pending,
    Failed,
}

struct CatalogFixture {
    catalog: IngredientCatalog,
    effects: IngredientEffectStream,
}

impl CatalogFixture {
    fn new() -> Self {
        let (catalog, effects) = IngredientCatalog::test_fixture();
        Self { catalog, effects }
    }

    fn scope(&self) -> CommandScopeId {
        self.catalog.view().scope
    }

    /// Setup only: load records into the list and apply a search.
    async fn ready(records: Vec<Ingredient>, search: &str) -> anyhow::Result<Self> {
        let mut fixture = Self::new();
        let outcome = fixture.load(Ok(records)).await?;
        fixture.apply_outcome(outcome).await?;
        if !search.is_empty() {
            let scope = fixture.scope();
            fixture
                .apply(list_event(
                    scope,
                    IngredientListEvent::Search(search.into()),
                ))
                .await?;
        }
        Ok(fixture)
    }

    /// Setup only: leave the list without a complete catalog.
    async fn without_cache(unavailable: Unavailable) -> anyhow::Result<Self> {
        let mut fixture = Self::new();
        match unavailable {
            // The initial load starts but never delivers records.
            Unavailable::Pending => drop(fixture.catalog.init()),
            Unavailable::Failed => {
                let outcome = fixture.load(Err(IngredientLoadError::Storage)).await?;
                fixture.apply_outcome(outcome).await?;
            }
        }
        Ok(fixture)
    }

    /// Setup only: run the initial load and return its result without applying it.
    async fn load(
        &mut self,
        result: Result<Vec<Ingredient>, IngredientLoadError>,
    ) -> anyhow::Result<IngredientCatalogOutcome> {
        let mut command = self.catalog.init().driver(&mut self.effects);
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
            .resolve_list(|| async { result })
            .await?;
        let outcome = command.step().await?.expect_outcome();
        command.step().await?.expect_done();
        Ok(outcome)
    }

    /// Setup only: dispatch an event that renders once and completes.
    async fn apply(&mut self, event: IngredientCatalogEvent) -> anyhow::Result<()> {
        let mut command = self
            .catalog
            .update(Message::Event(event))
            .driver(&mut self.effects);
        render_then_finish(&mut command).await
    }

    /// Setup only: dispatch an event whose command only reports one outcome.
    async fn choose(
        &mut self,
        event: IngredientCatalogEvent,
    ) -> anyhow::Result<IngredientCatalogOutcome> {
        let mut command = self
            .catalog
            .update(Message::Event(event))
            .driver(&mut self.effects);
        let outcome = command.step().await?.expect_outcome();
        command.step().await?.expect_done();
        Ok(outcome)
    }

    /// Setup only: dispatch an event whose command renders and reports one outcome.
    async fn choose_after_render(
        &mut self,
        event: IngredientCatalogEvent,
    ) -> anyhow::Result<IngredientCatalogOutcome> {
        let mut command = self
            .catalog
            .update(Message::Event(event))
            .driver(&mut self.effects);
        render_then_outcome(&mut command).await
    }

    /// Setup only: apply an outcome whose command renders once.
    async fn apply_outcome(&mut self, outcome: IngredientCatalogOutcome) -> anyhow::Result<()> {
        let mut command = self
            .catalog
            .update(Message::Outcome(outcome))
            .driver(&mut self.effects);
        render_then_finish(&mut command).await
    }

    async fn open_from_list(&mut self, event: IngredientListEvent) -> anyhow::Result<()> {
        let scope = self.scope();
        let outcome = self.choose(list_event(scope, event)).await?;
        self.apply_outcome(outcome).await
    }

    async fn open_create(&mut self) -> anyhow::Result<()> {
        self.open_from_list(IngredientListEvent::CreateRequested)
            .await
    }

    async fn open_detail(&mut self, id: IngredientId) -> anyhow::Result<()> {
        self.open_from_list(IngredientListEvent::Select(id)).await
    }

    async fn open_edit_from_list(&mut self, id: IngredientId) -> anyhow::Result<()> {
        self.open_from_list(IngredientListEvent::Edit(id)).await
    }

    async fn open_edit_from_detail(&mut self) -> anyhow::Result<()> {
        let scope = self.scope();
        let outcome = self
            .choose(detail_event(scope, IngredientDetailEvent::Edit))
            .await?;
        self.apply_outcome(outcome).await
    }

    async fn open_form(&mut self, origin: Origin, id: IngredientId) -> anyhow::Result<()> {
        match origin {
            Origin::Create => self.open_create().await,
            Origin::List => self.open_edit_from_list(id).await,
            Origin::Detail => {
                self.open_detail(id).await?;
                self.open_edit_from_detail().await
            }
        }
    }

    /// Setup only: save the active form and return the store result without applying it.
    async fn save(
        &mut self,
        result: Result<Ingredient, IngredientSaveError>,
    ) -> anyhow::Result<IngredientCatalogOutcome> {
        let scope = self.scope();
        let mut command = self
            .catalog
            .update(Message::Event(form_event(scope, IngredientFormEvent::Save)))
            .driver(&mut self.effects);
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
            .resolve_save(|_| async { result })
            .await?;
        let outcome = command.step().await?.expect_outcome();
        command.step().await?.expect_done();
        Ok(outcome)
    }

    /// Setup only: save successfully and return the form's `Saved` outcome
    /// without applying it.
    async fn finish_save(&mut self, saved: Ingredient) -> anyhow::Result<IngredientCatalogOutcome> {
        let finished = self.save(Ok(saved)).await?;
        let mut command = self
            .catalog
            .update(Message::Outcome(finished))
            .driver(&mut self.effects);
        render_then_outcome(&mut command).await
    }

    /// Setup only: request and confirm deletion, then return the store result
    /// without applying it.
    async fn delete(
        &mut self,
        result: Result<(), IngredientDeleteError>,
    ) -> anyhow::Result<IngredientCatalogOutcome> {
        let scope = self.scope();
        self.apply(detail_event(scope, IngredientDetailEvent::RequestDelete))
            .await?;
        self.confirm_delete(result).await
    }

    /// Setup only: confirm an open deletion and return the store result
    /// without applying it.
    async fn confirm_delete(
        &mut self,
        result: Result<(), IngredientDeleteError>,
    ) -> anyhow::Result<IngredientCatalogOutcome> {
        let scope = self.scope();
        let confirmed = self
            .choose(detail_event(
                scope,
                IngredientDetailEvent::Confirmation(ConfirmationEvent::Confirm),
            ))
            .await?;
        let mut command = self
            .catalog
            .update(Message::Outcome(confirmed))
            .driver(&mut self.effects);
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
            .resolve_delete(|_| async { result })
            .await?;
        let outcome = command.step().await?.expect_outcome();
        command.step().await?.expect_done();
        Ok(outcome)
    }

    /// Setup only: delete successfully and return the detail's `Deleted`
    /// outcome without applying it.
    async fn finish_delete(&mut self) -> anyhow::Result<IngredientCatalogOutcome> {
        let finished = self.delete(Ok(())).await?;
        self.finish_deleted(finished).await
    }

    /// Setup only: like `finish_delete`, for a confirmation that is already open.
    async fn finish_delete_from_confirmation(
        &mut self,
    ) -> anyhow::Result<IngredientCatalogOutcome> {
        let finished = self.confirm_delete(Ok(())).await?;
        self.finish_deleted(finished).await
    }

    async fn finish_deleted(
        &mut self,
        finished: IngredientCatalogOutcome,
    ) -> anyhow::Result<IngredientCatalogOutcome> {
        let mut command = self
            .catalog
            .update(Message::Outcome(finished))
            .driver(&mut self.effects);
        render_then_outcome(&mut command).await
    }
}

/// Setup only: resolve one render, then finish while ignoring outcomes.
async fn render_then_finish(
    command: &mut CommandDriver<'_, IngredientEffect, IngredientCatalogOutcome>,
) -> anyhow::Result<()> {
    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command.until_done_ignoring_outcomes().await?;
    Ok(())
}

/// Setup only: resolve one render and return the single outcome that follows.
async fn render_then_outcome(
    command: &mut CommandDriver<'_, IngredientEffect, IngredientCatalogOutcome>,
) -> anyhow::Result<IngredientCatalogOutcome> {
    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    let outcome = command.step().await?.expect_outcome();
    command.step().await?.expect_done();
    Ok(outcome)
}

fn list_event(scope: CommandScopeId, event: IngredientListEvent) -> IngredientCatalogEvent {
    IngredientCatalogEvent::List(Scoped::new(scope, event))
}

fn detail_event(scope: CommandScopeId, event: IngredientDetailEvent) -> IngredientCatalogEvent {
    IngredientCatalogEvent::Detail(Scoped::new(scope, event))
}

fn form_event(scope: CommandScopeId, event: IngredientFormEvent) -> IngredientCatalogEvent {
    IngredientCatalogEvent::Form(Scoped::new(scope, event))
}

fn back(scope: CommandScopeId) -> IngredientCatalogEvent {
    IngredientCatalogEvent::Back { scope }
}

fn field(field: IngredientDraftField, value: &str) -> IngredientFormEvent {
    IngredientFormEvent::SetField {
        field,
        value: value.into(),
    }
}

fn active_form(screen: &IngredientCatalogScreenView) -> &IngredientFormView {
    match screen {
        IngredientCatalogScreenView::Create(form)
        | IngredientCatalogScreenView::EditFromList(form)
        | IngredientCatalogScreenView::EditFromDetail { form, .. } => form,
        other => panic!("expected a form screen, got {other:?}"),
    }
}

fn idle_detail(ingredient: &Ingredient) -> IngredientDetailView {
    IngredientDetailView {
        ingredient: ingredient.clone(),
        deletion: IngredientDetailDeletion::Idle,
    }
}

fn row_names(view: &IngredientListView) -> Vec<&str> {
    let IngredientListStatus::Ready { rows, .. } = &view.status else {
        panic!("expected a ready list, got {:?}", view.status);
    };
    rows.iter().map(|row| row.name.as_str()).collect()
}

fn id(value: u128) -> IngredientId {
    Uuid::from_u128(value).into()
}

fn ingredient(value: u128, name: &str) -> Ingredient {
    Ingredient {
        id: id(value),
        name: name.into(),
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
