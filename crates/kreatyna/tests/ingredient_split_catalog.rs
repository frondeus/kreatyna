use kreatyna::{
    capabilities::ResolveRender,
    confirmation::{ConfirmationEvent, ConfirmationView},
    ingredients::{
        capabilities::{
            IngredientDeleteError, IngredientEffect, IngredientEffectStream, IngredientSaveError,
            ResolveIngredientStore,
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
async fn the_catalog_starts_with_nothing_selected() -> anyhow::Result<()> {
    let fixture = SplitFixture::ready(vec![ingredient(1, "Garlic")], "").await?;

    let view = fixture.catalog.view();
    assert_eq!(row_names(&view.list.value), ["Garlic"]);
    assert_eq!(view.selected, None);
    assert_eq!(view.pane.value, IngredientSplitPaneView::Empty);
    Ok(())
}

#[tokio::test]
async fn selecting_a_row_shows_its_detail_in_a_new_pane_scope() -> anyhow::Result<()> {
    let garlic = ingredient(1, "Garlic");
    let mut fixture = SplitFixture::ready(vec![garlic.clone()], "").await?;
    let before = fixture.catalog.view();

    let outcome = fixture
        .choose(list_event(
            before.list.scope,
            IngredientListEvent::Select(garlic.id),
        ))
        .await?;
    assert_eq!(
        outcome,
        IngredientSplitCatalogOutcome::List(Scoped::new(
            before.list.scope,
            IngredientListOutcome::OpenDetail {
                ingredient: garlic.clone()
            }
        ))
    );
    let mut command = fixture
        .catalog
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    let view = fixture.catalog.view();
    assert_eq!(view.selected, Some(garlic.id));
    assert_eq!(
        view.pane.value,
        IngredientSplitPaneView::Detail(idle_detail(&garlic))
    );
    assert_ne!(view.pane.scope, before.pane.scope);
    // The list keeps its scope and its rows.
    assert_eq!(view.list.scope, before.list.scope);
    assert_eq!(view.list.value, before.list.value);
    render_then_finish(&mut command).await
}

#[tokio::test]
async fn selecting_another_row_replaces_the_detail() -> anyhow::Result<()> {
    let garlic = ingredient(1, "Garlic");
    let basil = ingredient(2, "Basil");
    let mut fixture = SplitFixture::ready(vec![garlic.clone(), basil.clone()], "").await?;
    fixture.select(garlic.id).await?;
    let garlic_scope = fixture.catalog.view().pane.scope;

    fixture.select(basil.id).await?;

    let view = fixture.catalog.view();
    assert_eq!(view.selected, Some(basil.id));
    assert_eq!(
        view.pane.value,
        IngredientSplitPaneView::Detail(idle_detail(&basil))
    );
    assert_ne!(view.pane.scope, garlic_scope);
    Ok(())
}

#[tokio::test]
async fn selecting_the_selected_row_keeps_the_detail() -> anyhow::Result<()> {
    let garlic = ingredient(1, "Garlic");
    let mut fixture = SplitFixture::ready(vec![garlic.clone()], "").await?;
    fixture.select(garlic.id).await?;
    let before = fixture.catalog.view();

    let outcome = fixture
        .choose(list_event(
            before.list.scope,
            IngredientListEvent::Select(garlic.id),
        ))
        .await?;
    let mut command = fixture
        .catalog
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.catalog.view(), before);
    command.step().await?.expect_done();
    Ok(())
}

#[tokio::test]
async fn clearing_the_selection_empties_the_pane() -> anyhow::Result<()> {
    let garlic = ingredient(1, "Garlic");
    let mut fixture = SplitFixture::ready(vec![garlic.clone()], "").await?;
    fixture.select(garlic.id).await?;
    let detail_scope = fixture.catalog.view().pane.scope;

    fixture.apply(clear_selection(detail_scope)).await?;

    let view = fixture.catalog.view();
    assert_eq!(view.selected, None);
    assert_eq!(view.pane.value, IngredientSplitPaneView::Empty);
    assert_ne!(view.pane.scope, detail_scope);
    Ok(())
}

#[test_case(IngredientListEvent::Search("bas".into()); "search")]
#[test_case(IngredientListEvent::Refresh; "refresh")]
#[tokio::test]
async fn the_list_stays_live_while_a_detail_is_open(
    event: IngredientListEvent,
) -> anyhow::Result<()> {
    let garlic = ingredient(1, "Garlic");
    let basil = ingredient(2, "Basil");
    let mut fixture = SplitFixture::ready(vec![garlic.clone(), basil.clone()], "").await?;
    fixture.select(garlic.id).await?;
    let before = fixture.catalog.view();

    let mut command = fixture
        .catalog
        .update(Message::Event(list_event(before.list.scope, event.clone())))
        .driver(&mut fixture.effects);
    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    if event == IngredientListEvent::Refresh {
        command
            .step()
            .await?
            .expect_effect()
            .expect_store()
            .resolve_list(|| async { Ok(vec![garlic.clone(), basil.clone()]) })
            .await?;
        command.step().await?.expect_outcome();
    }
    command.step().await?.expect_done();
    drop(command);

    let view = fixture.catalog.view();
    // The detail stays, also when the search hides its row.
    assert_eq!(view.selected, Some(garlic.id));
    assert_eq!(view.pane.value, before.pane.value);
    assert_eq!(view.pane.scope, before.pane.scope);
    if let IngredientListEvent::Search(_) = event {
        assert_eq!(row_names(&view.list.value), ["Basil"]);
    }
    Ok(())
}

#[tokio::test]
async fn deleting_the_selected_ingredient_removes_its_row_and_empties_the_pane()
-> anyhow::Result<()> {
    let garlic = ingredient(1, "Garlic");
    let mut fixture = SplitFixture::ready(vec![garlic.clone(), ingredient(2, "Basil")], "").await?;
    fixture.select(garlic.id).await?;
    let scope = fixture.catalog.view().pane.scope;
    let deleted = fixture.finish_delete().await?;
    assert_eq!(
        deleted,
        IngredientSplitCatalogOutcome::Pane(Scoped::new(
            scope,
            IngredientSplitPaneOutcome::Detail(IngredientDetailOutcome::Deleted { id: garlic.id })
        ))
    );

    let mut command = fixture
        .catalog
        .update(Message::Outcome(deleted))
        .driver(&mut fixture.effects);
    let view = fixture.catalog.view();
    assert_eq!(view.selected, None);
    assert_eq!(view.pane.value, IngredientSplitPaneView::Empty);
    assert_eq!(row_names(&view.list.value), ["Basil"]);
    // Only a render follows. The cache is updated without a reload.
    render_then_finish(&mut command).await
}

// Edge cases and guards

#[tokio::test]
async fn a_row_hidden_by_the_search_cannot_be_selected() -> anyhow::Result<()> {
    let mut fixture =
        SplitFixture::ready(vec![ingredient(1, "Garlic"), ingredient(2, "Basil")], "bas").await?;
    let before = fixture.catalog.view();

    let mut command = fixture
        .catalog
        .update(Message::Event(list_event(
            before.list.scope,
            IngredientListEvent::Select(id(1)),
        )))
        .driver(&mut fixture.effects);
    command.step().await?.expect_done();
    assert_eq!(fixture.catalog.view(), before);
    Ok(())
}

#[tokio::test]
async fn a_detail_event_for_the_previous_selection_is_ignored() -> anyhow::Result<()> {
    let mut fixture =
        SplitFixture::ready(vec![ingredient(1, "Garlic"), ingredient(2, "Basil")], "").await?;
    fixture.select(id(1)).await?;
    let garlic_scope = fixture.catalog.view().pane.scope;
    fixture.select(id(2)).await?;

    let before = fixture.catalog.view();
    let mut command = fixture
        .catalog
        .update(Message::Event(detail_event(
            garlic_scope,
            IngredientDetailEvent::RequestDelete,
        )))
        .driver(&mut fixture.effects);
    command.step().await?.expect_done();
    assert_eq!(fixture.catalog.view(), before);
    Ok(())
}

#[tokio::test]
async fn a_late_selection_result_from_another_list_scope_is_ignored() -> anyhow::Result<()> {
    let mut fixture = SplitFixture::ready(vec![ingredient(1, "Garlic")], "").await?;
    let before = fixture.catalog.view();
    let foreign = CommandScopeId::from(u64::MAX);

    let mut command = fixture
        .catalog
        .update(Message::Outcome(IngredientSplitCatalogOutcome::List(
            Scoped::new(
                foreign,
                IngredientListOutcome::OpenDetail {
                    ingredient: ingredient(1, "Garlic"),
                },
            ),
        )))
        .driver(&mut fixture.effects);
    command.step().await?.expect_done();
    assert_eq!(fixture.catalog.view(), before);
    Ok(())
}

#[tokio::test]
async fn clearing_with_a_previous_pane_scope_is_ignored() -> anyhow::Result<()> {
    let mut fixture =
        SplitFixture::ready(vec![ingredient(1, "Garlic"), ingredient(2, "Basil")], "").await?;
    fixture.select(id(1)).await?;
    let garlic_scope = fixture.catalog.view().pane.scope;
    fixture.select(id(2)).await?;

    let before = fixture.catalog.view();
    let mut command = fixture
        .catalog
        .update(Message::Event(clear_selection(garlic_scope)))
        .driver(&mut fixture.effects);
    command.step().await?.expect_done();
    assert_eq!(fixture.catalog.view(), before);
    Ok(())
}

#[tokio::test]
async fn clearing_with_nothing_selected_does_nothing() -> anyhow::Result<()> {
    let mut fixture = SplitFixture::ready(vec![ingredient(1, "Garlic")], "").await?;
    let before = fixture.catalog.view();

    let mut command = fixture
        .catalog
        .update(Message::Event(clear_selection(before.pane.scope)))
        .driver(&mut fixture.effects);
    command.step().await?.expect_done();
    assert_eq!(fixture.catalog.view(), before);
    Ok(())
}

#[tokio::test]
async fn the_selection_is_kept_while_a_delete_is_being_confirmed() -> anyhow::Result<()> {
    let mut fixture =
        SplitFixture::ready(vec![ingredient(1, "Garlic"), ingredient(2, "Basil")], "").await?;
    fixture.select(id(1)).await?;
    let pane_scope = fixture.catalog.view().pane.scope;
    fixture
        .apply(detail_event(
            pane_scope,
            IngredientDetailEvent::RequestDelete,
        ))
        .await?;
    let before = fixture.catalog.view();

    let outcome = fixture
        .choose(list_event(
            before.list.scope,
            IngredientListEvent::Select(id(2)),
        ))
        .await?;
    let mut command = fixture
        .catalog
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    command.step().await?.expect_done();
    drop(command);
    let mut command = fixture
        .catalog
        .update(Message::Event(clear_selection(pane_scope)))
        .driver(&mut fixture.effects);
    command.step().await?.expect_done();
    assert_eq!(fixture.catalog.view(), before);
    Ok(())
}

#[tokio::test]
/// No Mac list action sends it: Edit is in the detail.
async fn edit_from_the_list_is_ignored() -> anyhow::Result<()> {
    let mut fixture = SplitFixture::ready(vec![ingredient(1, "Garlic")], "").await?;
    let before = fixture.catalog.view();

    let outcome = fixture
        .choose(list_event(
            before.list.scope,
            IngredientListEvent::Edit(id(1)),
        ))
        .await?;
    let mut command = fixture
        .catalog
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    command.step().await?.expect_done();
    assert_eq!(fixture.catalog.view(), before);
    Ok(())
}

#[tokio::test]
async fn a_failed_delete_keeps_the_selection() -> anyhow::Result<()> {
    let mut fixture = SplitFixture::ready(vec![ingredient(1, "Garlic")], "").await?;
    fixture.select(id(1)).await?;
    let failed = fixture.delete(Err(IngredientDeleteError::Storage)).await?;
    fixture.apply_outcome(failed).await?;

    let view = fixture.catalog.view();
    assert_eq!(view.selected, Some(id(1)));
    assert_eq!(row_names(&view.list.value), ["Garlic"]);
    Ok(())
}

// Create (MAC-05)

#[tokio::test]
async fn create_opens_the_form_in_a_new_pane_scope() -> anyhow::Result<()> {
    let mut fixture = SplitFixture::ready(vec![ingredient(1, "Garlic")], "").await?;
    fixture.select(id(1)).await?;
    let before = fixture.catalog.view();
    assert!(before.can_create);

    fixture.create().await?;

    let view = fixture.catalog.view();
    assert_ne!(view.pane.scope, before.pane.scope);
    assert_eq!(view.list.scope, before.list.scope);
    let form = fixture.form();
    assert_eq!(form.draft.name, "");
    assert_eq!(form.draft.basis, NutritionBasis::Per100Grams);
    assert_eq!(form.status, IngredientFormStatus::Editing);
    assert_eq!(view.selected, None);
    assert_eq!(view.draft_row.as_deref(), Some(""));
    assert!(!view.can_create);
    assert_eq!(row_names(&view.list.value), ["Garlic"]);
    Ok(())
}

#[tokio::test]
async fn the_draft_row_shows_the_typed_name() -> anyhow::Result<()> {
    let mut fixture = SplitFixture::ready(vec![ingredient(1, "Garlic")], "").await?;
    fixture.create().await?;

    fixture.type_name("  Paprika ").await?;
    assert_eq!(fixture.catalog.view().draft_row.as_deref(), Some("Paprika"));

    let saved = fixture.finish_save(ingredient(2, "Paprika")).await?;
    fixture.apply_outcome(saved).await?;
    assert_eq!(fixture.catalog.view().draft_row, None);
    Ok(())
}

#[tokio::test]
async fn the_draft_row_ignores_the_search() -> anyhow::Result<()> {
    let mut fixture = SplitFixture::ready(
        vec![ingredient(1, "Tomato"), ingredient(2, "Garlic")],
        "tom",
    )
    .await?;
    fixture.create().await?;
    fixture.type_name("Paprika").await?;

    let view = fixture.catalog.view();
    assert_eq!(row_names(&view.list.value), ["Tomato"]);
    assert_eq!(view.draft_row.as_deref(), Some("Paprika"));
    Ok(())
}

#[tokio::test]
async fn saving_adds_the_row_and_shows_its_detail() -> anyhow::Result<()> {
    let paprika = ingredient(3, "Paprika");
    let mut fixture =
        SplitFixture::ready(vec![ingredient(1, "Garlic"), ingredient(2, "Tomato")], "").await?;
    fixture.create().await?;
    fixture.type_name("Paprika").await?;
    let form_scope = fixture.catalog.view().pane.scope;

    let saved = fixture.finish_save(paprika.clone()).await?;
    fixture.apply_outcome(saved).await?;

    let view = fixture.catalog.view();
    assert_eq!(row_names(&view.list.value), ["Garlic", "Paprika", "Tomato"]);
    assert_eq!(view.selected, Some(paprika.id));
    assert_ne!(view.pane.scope, form_scope);
    assert_eq!(
        view.pane.value,
        IngredientSplitPaneView::Detail(idle_detail(&paprika))
    );
    assert!(view.can_create);
    Ok(())
}

#[tokio::test]
async fn a_saved_ingredient_hidden_by_the_search_still_opens() -> anyhow::Result<()> {
    let paprika = ingredient(3, "Paprika");
    let mut fixture = SplitFixture::ready(
        vec![ingredient(1, "Tomato"), ingredient(2, "Garlic")],
        "tom",
    )
    .await?;
    fixture.create().await?;
    fixture.type_name("Paprika").await?;

    let saved = fixture.finish_save(paprika.clone()).await?;
    fixture.apply_outcome(saved).await?;

    let view = fixture.catalog.view();
    assert_eq!(view.list.value.search, "tom");
    assert_eq!(row_names(&view.list.value), ["Tomato"]);
    assert_eq!(view.selected, Some(paprika.id));
    assert_eq!(
        view.pane.value,
        IngredientSplitPaneView::Detail(idle_detail(&paprika))
    );
    Ok(())
}

#[tokio::test]
async fn a_duplicate_name_keeps_the_form_with_the_error() -> anyhow::Result<()> {
    let mut fixture = SplitFixture::ready(vec![ingredient(1, "Garlic")], "").await?;
    fixture.create().await?;
    fixture.type_name("garlic").await?;
    let form_scope = fixture.catalog.view().pane.scope;

    let failed = fixture
        .save(Err(IngredientSaveError::DuplicateName))
        .await?;
    fixture.apply_outcome(failed).await?;

    let view = fixture.catalog.view();
    assert_eq!(view.pane.scope, form_scope);
    let form = fixture.form();
    assert_eq!(form.status, IngredientFormStatus::Editing);
    assert_eq!(form.save_error, Some(IngredientSaveError::DuplicateName));
    assert_eq!(view.draft_row.as_deref(), Some("garlic"));
    assert_eq!(row_names(&view.list.value), ["Garlic"]);
    Ok(())
}

#[tokio::test]
async fn cancel_on_a_clean_form_returns_to_the_previous_detail() -> anyhow::Result<()> {
    let garlic = ingredient(1, "Garlic");
    let mut fixture = SplitFixture::ready(vec![garlic.clone()], "").await?;
    fixture.select(garlic.id).await?;
    fixture.create().await?;

    let discarded = fixture.close_clean_form().await?;
    fixture.apply_outcome(discarded).await?;

    let view = fixture.catalog.view();
    assert_eq!(view.selected, Some(garlic.id));
    assert_eq!(
        view.pane.value,
        IngredientSplitPaneView::Detail(idle_detail(&garlic))
    );
    assert_eq!(view.draft_row, None);
    assert!(view.can_create);
    Ok(())
}

#[tokio::test]
async fn cancel_on_a_clean_form_with_nothing_before_empties_the_pane() -> anyhow::Result<()> {
    let mut fixture = SplitFixture::ready(vec![ingredient(1, "Garlic")], "").await?;
    fixture.create().await?;

    let discarded = fixture.close_clean_form().await?;
    fixture.apply_outcome(discarded).await?;

    let view = fixture.catalog.view();
    assert_eq!(view.selected, None);
    assert_eq!(view.pane.value, IngredientSplitPaneView::Empty);
    assert_eq!(view.draft_row, None);
    Ok(())
}

#[tokio::test]
async fn a_row_click_on_a_clean_form_opens_the_row() -> anyhow::Result<()> {
    let tomato = ingredient(2, "Tomato");
    let mut fixture =
        SplitFixture::ready(vec![ingredient(1, "Garlic"), tomato.clone()], "").await?;
    fixture.create().await?;

    fixture.select(tomato.id).await?;

    let view = fixture.catalog.view();
    assert_eq!(view.selected, Some(tomato.id));
    assert_eq!(
        view.pane.value,
        IngredientSplitPaneView::Detail(idle_detail(&tomato))
    );
    assert_eq!(view.draft_row, None);
    Ok(())
}

#[tokio::test]
async fn a_row_click_on_a_dirty_form_asks_and_discard_opens_the_row() -> anyhow::Result<()> {
    let tomato = ingredient(2, "Tomato");
    let mut fixture =
        SplitFixture::ready(vec![ingredient(1, "Garlic"), tomato.clone()], "").await?;
    fixture.select(id(1)).await?;
    fixture.create().await?;
    fixture.type_name("Pap").await?;
    let form_scope = fixture.catalog.view().pane.scope;

    fixture.select(tomato.id).await?;
    let view = fixture.catalog.view();
    assert_eq!(view.pane.scope, form_scope);
    assert_eq!(
        fixture.form().status,
        IngredientFormStatus::ConfirmingDiscard(ConfirmationView::AwaitingDecision)
    );
    assert_eq!(view.draft_row.as_deref(), Some("Pap"));

    let discarded = fixture.discard_after_question().await?;
    fixture.apply_outcome(discarded).await?;

    let view = fixture.catalog.view();
    assert_eq!(view.selected, Some(tomato.id));
    assert_eq!(
        view.pane.value,
        IngredientSplitPaneView::Detail(idle_detail(&tomato))
    );
    Ok(())
}

#[tokio::test]
async fn keep_editing_forgets_the_clicked_row() -> anyhow::Result<()> {
    let garlic = ingredient(1, "Garlic");
    let mut fixture =
        SplitFixture::ready(vec![garlic.clone(), ingredient(2, "Tomato")], "").await?;
    fixture.select(garlic.id).await?;
    fixture.create().await?;
    fixture.type_name("Pap").await?;
    fixture.select(id(2)).await?;

    let scope = fixture.catalog.view().pane.scope;
    let kept = fixture
        .choose(form_event(
            scope,
            IngredientFormEvent::Confirmation(ConfirmationEvent::Cancel),
        ))
        .await?;
    fixture.apply_outcome(kept).await?;
    assert_eq!(fixture.form().status, IngredientFormStatus::Editing);
    assert_eq!(fixture.form().draft.name, "Pap");

    // A later Cancel goes back to Garlic, not to the clicked Tomato.
    fixture
        .apply(form_event(scope, IngredientFormEvent::RequestClose))
        .await?;
    let discarded = fixture.discard_after_question().await?;
    fixture.apply_outcome(discarded).await?;
    let view = fixture.catalog.view();
    assert_eq!(view.selected, Some(garlic.id));
    assert_eq!(
        view.pane.value,
        IngredientSplitPaneView::Detail(idle_detail(&garlic))
    );
    Ok(())
}

#[test_case(Busy::FormOpen; "a form is open")]
#[test_case(Busy::DeleteRuns; "a delete is being confirmed")]
#[tokio::test]
async fn create_is_ignored_while_a_form_is_open_or_a_delete_runs(busy: Busy) -> anyhow::Result<()> {
    let mut fixture = SplitFixture::ready(vec![ingredient(1, "Garlic")], "").await?;
    fixture.select(id(1)).await?;
    match busy {
        Busy::FormOpen => {
            fixture.create().await?;
            fixture.type_name("Pap").await?;
        }
        Busy::DeleteRuns => {
            let scope = fixture.catalog.view().pane.scope;
            fixture
                .apply(detail_event(scope, IngredientDetailEvent::RequestDelete))
                .await?;
        }
    }
    let before = fixture.catalog.view();
    assert!(!before.can_create);

    let outcome = fixture
        .choose(list_event(
            before.list.scope,
            IngredientListEvent::CreateRequested,
        ))
        .await?;
    let mut command = fixture
        .catalog
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    command.step().await?.expect_done();
    assert_eq!(fixture.catalog.view(), before);
    Ok(())
}

#[tokio::test]
async fn a_late_form_result_from_an_old_pane_scope_is_ignored() -> anyhow::Result<()> {
    let mut fixture = SplitFixture::ready(vec![ingredient(1, "Garlic")], "").await?;
    fixture.create().await?;
    fixture.type_name("Paprika").await?;
    let saved = fixture.finish_save(ingredient(2, "Paprika")).await?;
    fixture.apply_outcome(saved.clone()).await?;
    let before = fixture.catalog.view();

    let mut command = fixture
        .catalog
        .update(Message::Outcome(saved))
        .driver(&mut fixture.effects);
    command.step().await?.expect_done();
    assert_eq!(fixture.catalog.view(), before);
    Ok(())
}

#[tokio::test]
async fn clearing_the_selection_is_ignored_while_a_form_is_open() -> anyhow::Result<()> {
    let mut fixture = SplitFixture::ready(vec![ingredient(1, "Garlic")], "").await?;
    fixture.select(id(1)).await?;
    fixture.create().await?;
    let before = fixture.catalog.view();

    let mut command = fixture
        .catalog
        .update(Message::Event(clear_selection(before.pane.scope)))
        .driver(&mut fixture.effects);
    command.step().await?.expect_done();
    assert_eq!(fixture.catalog.view(), before);
    Ok(())
}

// Edit (MAC-06)

#[tokio::test]
async fn edit_opens_the_form_in_a_new_pane_scope() -> anyhow::Result<()> {
    let mut fixture =
        SplitFixture::ready(vec![ingredient(1, "Garlic"), ingredient(2, "Tomato")], "").await?;
    fixture.select(id(1)).await?;
    let before = fixture.catalog.view();

    fixture.edit().await?;

    let view = fixture.catalog.view();
    assert_ne!(view.pane.scope, before.pane.scope);
    assert_eq!(view.list.scope, before.list.scope);
    assert_eq!(fixture.saved_name(), "Garlic");
    let form = fixture.form();
    assert_eq!(form.draft.name, "Garlic");
    assert_eq!(form.status, IngredientFormStatus::Editing);
    assert!(!form.dirty);
    assert_eq!(view.selected, Some(id(1)));
    assert_eq!(view.draft_row, None);
    assert!(!view.can_create);
    Ok(())
}

#[tokio::test]
async fn the_edited_row_keeps_its_saved_name_until_save() -> anyhow::Result<()> {
    let mut fixture =
        SplitFixture::ready(vec![ingredient(1, "Garlic"), ingredient(2, "Tomato")], "").await?;
    fixture.select(id(1)).await?;
    fixture.edit().await?;

    fixture.type_name("Black Garlic").await?;

    let view = fixture.catalog.view();
    assert_eq!(fixture.form().draft.name, "Black Garlic");
    assert_eq!(fixture.saved_name(), "Garlic");
    assert_eq!(row_names(&view.list.value), ["Garlic", "Tomato"]);
    assert_eq!(view.selected, Some(id(1)));
    assert_eq!(view.draft_row, None);
    Ok(())
}

#[tokio::test]
async fn saving_an_edit_updates_the_row_and_shows_its_detail() -> anyhow::Result<()> {
    let renamed = ingredient(2, "Basil");
    let mut fixture =
        SplitFixture::ready(vec![ingredient(1, "Garlic"), ingredient(2, "Tomato")], "").await?;
    fixture.select(id(2)).await?;
    fixture.edit().await?;
    fixture.type_name("Basil").await?;
    let form_scope = fixture.catalog.view().pane.scope;

    let saved = fixture.finish_save(renamed.clone()).await?;
    fixture.apply_outcome(saved).await?;

    let view = fixture.catalog.view();
    assert_eq!(row_names(&view.list.value), ["Basil", "Garlic"]);
    assert_eq!(view.selected, Some(renamed.id));
    assert_ne!(view.pane.scope, form_scope);
    assert_eq!(
        view.pane.value,
        IngredientSplitPaneView::Detail(idle_detail(&renamed))
    );
    assert!(view.can_create);
    Ok(())
}

#[tokio::test]
async fn cancel_on_a_clean_edit_returns_to_the_detail() -> anyhow::Result<()> {
    let garlic = ingredient(1, "Garlic");
    let mut fixture = SplitFixture::ready(vec![garlic.clone()], "").await?;
    fixture.select(garlic.id).await?;
    fixture.edit().await?;

    let discarded = fixture.close_clean_form().await?;
    fixture.apply_outcome(discarded).await?;

    let view = fixture.catalog.view();
    assert_eq!(view.selected, Some(garlic.id));
    assert_eq!(
        view.pane.value,
        IngredientSplitPaneView::Detail(idle_detail(&garlic))
    );
    Ok(())
}

#[tokio::test]
async fn discard_on_a_dirty_edit_returns_to_the_saved_detail() -> anyhow::Result<()> {
    let garlic = ingredient(1, "Garlic");
    let mut fixture = SplitFixture::ready(vec![garlic.clone()], "").await?;
    fixture.select(garlic.id).await?;
    fixture.edit().await?;
    fixture.type_name("Black Garlic").await?;
    let scope = fixture.catalog.view().pane.scope;

    fixture
        .apply(form_event(scope, IngredientFormEvent::RequestClose))
        .await?;
    assert_eq!(
        fixture.form().status,
        IngredientFormStatus::ConfirmingDiscard(ConfirmationView::AwaitingDecision)
    );
    let discarded = fixture.discard_after_question().await?;
    fixture.apply_outcome(discarded).await?;

    let view = fixture.catalog.view();
    assert_eq!(view.selected, Some(garlic.id));
    assert_eq!(
        view.pane.value,
        IngredientSplitPaneView::Detail(idle_detail(&garlic))
    );
    Ok(())
}

#[test_case(false; "a clean form opens the row at once")]
#[test_case(true; "a dirty form asks first")]
#[tokio::test]
async fn a_row_click_on_an_edit_form(dirty: bool) -> anyhow::Result<()> {
    let tomato = ingredient(2, "Tomato");
    let mut fixture =
        SplitFixture::ready(vec![ingredient(1, "Garlic"), tomato.clone()], "").await?;
    fixture.select(id(1)).await?;
    fixture.edit().await?;
    if dirty {
        fixture.type_name("Black Garlic").await?;
    }
    let form_scope = fixture.catalog.view().pane.scope;

    fixture.select(tomato.id).await?;

    if dirty {
        let view = fixture.catalog.view();
        assert_eq!(view.pane.scope, form_scope);
        assert_eq!(view.selected, Some(id(1)));
        assert_eq!(
            fixture.form().status,
            IngredientFormStatus::ConfirmingDiscard(ConfirmationView::AwaitingDecision)
        );
        let discarded = fixture.discard_after_question().await?;
        fixture.apply_outcome(discarded).await?;
    }
    let view = fixture.catalog.view();
    assert_eq!(view.selected, Some(tomato.id));
    assert_eq!(
        view.pane.value,
        IngredientSplitPaneView::Detail(idle_detail(&tomato))
    );
    Ok(())
}

#[tokio::test]
async fn keep_editing_on_an_edit_forgets_the_clicked_row() -> anyhow::Result<()> {
    let garlic = ingredient(1, "Garlic");
    let mut fixture =
        SplitFixture::ready(vec![garlic.clone(), ingredient(2, "Tomato")], "").await?;
    fixture.select(garlic.id).await?;
    fixture.edit().await?;
    fixture.type_name("Black Garlic").await?;
    fixture.select(id(2)).await?;

    let scope = fixture.catalog.view().pane.scope;
    let kept = fixture
        .choose(form_event(
            scope,
            IngredientFormEvent::Confirmation(ConfirmationEvent::Cancel),
        ))
        .await?;
    fixture.apply_outcome(kept).await?;
    assert_eq!(fixture.form().status, IngredientFormStatus::Editing);
    assert_eq!(fixture.form().draft.name, "Black Garlic");

    // A later Cancel goes back to Garlic, not to the clicked Tomato.
    fixture
        .apply(form_event(scope, IngredientFormEvent::RequestClose))
        .await?;
    let discarded = fixture.discard_after_question().await?;
    fixture.apply_outcome(discarded).await?;
    let view = fixture.catalog.view();
    assert_eq!(view.selected, Some(garlic.id));
    assert_eq!(
        view.pane.value,
        IngredientSplitPaneView::Detail(idle_detail(&garlic))
    );
    Ok(())
}

#[test_case(false; "clean")]
#[test_case(true; "dirty")]
#[tokio::test]
async fn clicking_the_edited_row_does_nothing(dirty: bool) -> anyhow::Result<()> {
    let mut fixture = SplitFixture::ready(vec![ingredient(1, "Garlic")], "").await?;
    fixture.select(id(1)).await?;
    fixture.edit().await?;
    if dirty {
        fixture.type_name("Black Garlic").await?;
    }
    let before = fixture.catalog.view();

    let scope = before.list.scope;
    let outcome = fixture
        .choose(list_event(scope, IngredientListEvent::Select(id(1))))
        .await?;
    let mut command = fixture
        .catalog
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    command.step().await?.expect_done();
    assert_eq!(fixture.catalog.view(), before);
    Ok(())
}

#[tokio::test]
async fn edit_is_ignored_while_a_delete_is_being_confirmed() -> anyhow::Result<()> {
    let mut fixture = SplitFixture::ready(vec![ingredient(1, "Garlic")], "").await?;
    fixture.select(id(1)).await?;
    let scope = fixture.catalog.view().pane.scope;
    fixture
        .apply(detail_event(scope, IngredientDetailEvent::RequestDelete))
        .await?;
    let before = fixture.catalog.view();

    let mut command = fixture
        .catalog
        .update(Message::Event(detail_event(
            scope,
            IngredientDetailEvent::Edit,
        )))
        .driver(&mut fixture.effects);
    command.step().await?.expect_done();
    assert_eq!(fixture.catalog.view(), before);
    Ok(())
}

#[tokio::test]
async fn a_search_that_hides_the_edited_row_keeps_the_form() -> anyhow::Result<()> {
    let mut fixture =
        SplitFixture::ready(vec![ingredient(1, "Garlic"), ingredient(2, "Tomato")], "").await?;
    fixture.select(id(1)).await?;
    fixture.edit().await?;
    let form_scope = fixture.catalog.view().pane.scope;

    let list_scope = fixture.catalog.view().list.scope;
    fixture
        .apply(list_event(
            list_scope,
            IngredientListEvent::Search("tom".into()),
        ))
        .await?;

    let view = fixture.catalog.view();
    assert_eq!(row_names(&view.list.value), ["Tomato"]);
    assert_eq!(view.pane.scope, form_scope);
    assert_eq!(fixture.saved_name(), "Garlic");
    Ok(())
}

// Support

#[derive(Debug, Clone, Copy)]
enum Busy {
    FormOpen,
    DeleteRuns,
}

struct SplitFixture {
    catalog: IngredientSplitCatalog,
    effects: IngredientEffectStream,
}

impl SplitFixture {
    /// Setup only: load records into the list and apply a search.
    async fn ready(records: Vec<Ingredient>, search: &str) -> anyhow::Result<Self> {
        let (catalog, effects) = IngredientSplitCatalog::test_fixture();
        let mut fixture = Self { catalog, effects };
        let mut command = fixture.catalog.init().driver(&mut fixture.effects);
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
            .resolve_list(|| async { Ok(records) })
            .await?;
        let loaded = command.step().await?.expect_outcome();
        command.step().await?.expect_done();
        drop(command);
        fixture.apply_outcome(loaded).await?;
        if !search.is_empty() {
            let scope = fixture.catalog.view().list.scope;
            fixture
                .apply(list_event(
                    scope,
                    IngredientListEvent::Search(search.into()),
                ))
                .await?;
        }
        Ok(fixture)
    }

    /// Setup only: dispatch an event that renders once and completes.
    async fn apply(&mut self, event: IngredientSplitCatalogEvent) -> anyhow::Result<()> {
        let mut command = self
            .catalog
            .update(Message::Event(event))
            .driver(&mut self.effects);
        render_then_finish(&mut command).await
    }

    /// Setup only: dispatch an event whose command only reports one outcome.
    async fn choose(
        &mut self,
        event: IngredientSplitCatalogEvent,
    ) -> anyhow::Result<IngredientSplitCatalogOutcome> {
        let mut command = self
            .catalog
            .update(Message::Event(event))
            .driver(&mut self.effects);
        let outcome = command.step().await?.expect_outcome();
        command.step().await?.expect_done();
        Ok(outcome)
    }

    /// Setup only: apply an outcome whose command renders once.
    async fn apply_outcome(
        &mut self,
        outcome: IngredientSplitCatalogOutcome,
    ) -> anyhow::Result<()> {
        let mut command = self
            .catalog
            .update(Message::Outcome(outcome))
            .driver(&mut self.effects);
        render_then_finish(&mut command).await
    }

    async fn select(&mut self, id: IngredientId) -> anyhow::Result<()> {
        let scope = self.catalog.view().list.scope;
        let outcome = self
            .choose(list_event(scope, IngredientListEvent::Select(id)))
            .await?;
        self.apply_outcome(outcome).await
    }

    async fn create(&mut self) -> anyhow::Result<()> {
        let scope = self.catalog.view().list.scope;
        let outcome = self
            .choose(list_event(scope, IngredientListEvent::CreateRequested))
            .await?;
        self.apply_outcome(outcome).await
    }

    async fn type_name(&mut self, name: &str) -> anyhow::Result<()> {
        let scope = self.catalog.view().pane.scope;
        self.apply(form_event(
            scope,
            IngredientFormEvent::SetField {
                field: IngredientDraftField::Name,
                value: name.into(),
            },
        ))
        .await
    }

    async fn edit(&mut self) -> anyhow::Result<()> {
        let scope = self.catalog.view().pane.scope;
        let outcome = self
            .choose(detail_event(scope, IngredientDetailEvent::Edit))
            .await?;
        self.apply_outcome(outcome).await
    }

    /// The create or the edit form.
    fn form(&self) -> IngredientFormView {
        match self.catalog.view().pane.value {
            IngredientSplitPaneView::Create(form) | IngredientSplitPaneView::Edit { form, .. } => {
                form
            }
            pane => panic!("expected a form, got {pane:?}"),
        }
    }

    fn saved_name(&self) -> String {
        match self.catalog.view().pane.value {
            IngredientSplitPaneView::Edit { saved_name, .. } => saved_name,
            pane => panic!("expected the edit form, got {pane:?}"),
        }
    }

    /// Setup only: save the form and return the store result without applying it.
    async fn save(
        &mut self,
        result: Result<Ingredient, IngredientSaveError>,
    ) -> anyhow::Result<IngredientSplitCatalogOutcome> {
        let scope = self.catalog.view().pane.scope;
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
    async fn finish_save(
        &mut self,
        saved: Ingredient,
    ) -> anyhow::Result<IngredientSplitCatalogOutcome> {
        let finished = self.save(Ok(saved)).await?;
        let mut command = self
            .catalog
            .update(Message::Outcome(finished))
            .driver(&mut self.effects);
        render_then_outcome(&mut command).await
    }

    /// Setup only: Cancel on a clean form; returns `Discarded` without applying it.
    async fn close_clean_form(&mut self) -> anyhow::Result<IngredientSplitCatalogOutcome> {
        let scope = self.catalog.view().pane.scope;
        let mut command = self
            .catalog
            .update(Message::Event(form_event(
                scope,
                IngredientFormEvent::RequestClose,
            )))
            .driver(&mut self.effects);
        render_then_outcome(&mut command).await
    }

    /// Setup only: answer "Discard changes?" with Discard; returns `Discarded`
    /// without applying it.
    async fn discard_after_question(&mut self) -> anyhow::Result<IngredientSplitCatalogOutcome> {
        let scope = self.catalog.view().pane.scope;
        let decided = self
            .choose(form_event(
                scope,
                IngredientFormEvent::Confirmation(ConfirmationEvent::Confirm),
            ))
            .await?;
        let mut command = self
            .catalog
            .update(Message::Outcome(decided))
            .driver(&mut self.effects);
        render_then_outcome(&mut command).await
    }

    /// Setup only: request and confirm deletion, then return the store result
    /// without applying it.
    async fn delete(
        &mut self,
        result: Result<(), IngredientDeleteError>,
    ) -> anyhow::Result<IngredientSplitCatalogOutcome> {
        let scope = self.catalog.view().pane.scope;
        self.apply(detail_event(scope, IngredientDetailEvent::RequestDelete))
            .await?;
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
    async fn finish_delete(&mut self) -> anyhow::Result<IngredientSplitCatalogOutcome> {
        let finished = self.delete(Ok(())).await?;
        let mut command = self
            .catalog
            .update(Message::Outcome(finished))
            .driver(&mut self.effects);
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
}

/// Setup only: resolve one render, then finish while ignoring outcomes.
async fn render_then_finish(
    command: &mut CommandDriver<'_, IngredientEffect, IngredientSplitCatalogOutcome>,
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

/// Setup only: resolve one render, then return the one outcome.
async fn render_then_outcome(
    command: &mut CommandDriver<'_, IngredientEffect, IngredientSplitCatalogOutcome>,
) -> anyhow::Result<IngredientSplitCatalogOutcome> {
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

fn list_event(scope: CommandScopeId, event: IngredientListEvent) -> IngredientSplitCatalogEvent {
    IngredientSplitCatalogEvent::List(Scoped::new(scope, event))
}

fn detail_event(
    scope: CommandScopeId,
    event: IngredientDetailEvent,
) -> IngredientSplitCatalogEvent {
    IngredientSplitCatalogEvent::Detail(Scoped::new(scope, event))
}

fn form_event(scope: CommandScopeId, event: IngredientFormEvent) -> IngredientSplitCatalogEvent {
    IngredientSplitCatalogEvent::Form(Scoped::new(scope, event))
}

fn clear_selection(scope: CommandScopeId) -> IngredientSplitCatalogEvent {
    IngredientSplitCatalogEvent::ClearSelection { scope }
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
