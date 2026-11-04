use kreatyna::{
    capabilities::ResolveRender,
    confirmation::{ConfirmationEvent, ConfirmationView},
    recipes::{
        CanvasPoint, Recipe, RecipeDeletion, RecipeDetailEvent, RecipeDetailStatus,
        RecipeDetailStep, RecipeDetailView, RecipeEditorEvent, RecipeEditorProblem,
        RecipeEditorStatus, RecipeEditorView, RecipeId, RecipeListEvent, RecipeListRow,
        RecipeListStatus, RecipePaneView, RecipeSection, RecipeSectionEvent, RecipeSectionOutcome,
        RecipeSectionView, RecipeSummary, Step, StepCanvasView, StepId, StepKind, StepLabel,
        StepNodeView,
        capabilities::{
            RecipeDeleteError, RecipeEffect, RecipeEffectStream, RecipeLoadError, RecipeSaveError,
            RecipeStoreRequest, ResolveRecipeStore,
        },
    },
};
use krucyfiks::{
    Command, Message, Scoped, Widget,
    effects::{CommandEffect, CommandTestExt, Envelope},
};
use uuid::Uuid;

// Green path

#[tokio::test]
async fn the_section_starts_with_the_list_and_an_empty_pane() -> anyhow::Result<()> {
    let mut f = Fixture::new(vec![shakshuka()]);
    let load = f.section.init();
    f.settle(load).await?;
    let view = f.section.view();
    assert_eq!(view.list.status, ready(&[shakshuka()]));
    assert_eq!(view.pane.value, RecipePaneView::Empty);
    assert_eq!(view.selected, None);
    assert!(view.can_create);
    Ok(())
}

#[tokio::test]
async fn selecting_a_row_highlights_it_and_opens_its_read_view() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    let before = f.section.view().pane.scope;
    let command = f.send(RecipeSectionEvent::Select(shakshuka().id));
    let view = f.section.view();
    assert_eq!(view.selected, Some(shakshuka().id));
    assert_eq!(
        view.pane.value,
        detail(&shakshuka(), RecipeDetailStatus::Loading)
    );
    assert_ne!(view.pane.scope, before);
    assert!(view.can_create);

    f.settle(command).await?;
    assert_eq!(f.section.view().pane.value, read(&shakshuka()));
    Ok(())
}

#[tokio::test]
async fn selecting_another_recipe_replaces_the_read_view() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.select(&shakshuka()).await?;
    let before = f.section.view().pane.scope;
    f.select(&pasta()).await?;
    let view = f.section.view();
    assert_eq!(view.selected, Some(pasta().id));
    assert_eq!(view.pane.value, read(&pasta()));
    assert_ne!(view.pane.scope, before);
    Ok(())
}

#[tokio::test]
async fn clear_selection_empties_the_pane() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.select(&shakshuka()).await?;
    let scope = f.section.view().pane.scope;
    let command = f.send(RecipeSectionEvent::ClearSelection { scope });
    f.settle(command).await?;
    let view = f.section.view();
    assert_eq!(view.pane.value, RecipePaneView::Empty);
    assert_eq!(view.selected, None);
    Ok(())
}

#[tokio::test]
async fn new_recipe_opens_the_editor_with_an_empty_draft_row() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    let before = f.section.view().pane.scope;
    let command = f.send(RecipeSectionEvent::NewRecipe);
    let view = f.section.view();
    assert_eq!(view.pane.value, create("", ""));
    assert_ne!(view.pane.scope, before);
    assert!(!view.can_create);
    f.settle(command).await
}

#[tokio::test]
async fn new_recipe_works_while_a_read_view_is_open() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.select(&shakshuka()).await?;
    f.new_recipe().await?;
    let view = f.section.view();
    assert_eq!(view.pane.value, create("", ""));
    assert_eq!(view.selected, None);
    Ok(())
}

#[tokio::test]
async fn the_draft_row_shows_the_typed_title_trimmed() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.new_recipe().await?;
    f.type_title("  Omelette  ").await?;
    assert_eq!(
        f.section.view().pane.value,
        create("  Omelette  ", "Omelette")
    );
    Ok(())
}

#[tokio::test]
async fn saving_opens_the_saved_recipe_and_loads_the_list_again() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.new_recipe().await?;
    f.type_title("Omelette").await?;
    let saved = f.save(Ok(omelette())).await?;

    let command = f.section.update(Message::Outcome(saved));
    let view = f.section.view();
    assert_eq!(view.selected, Some(omelette().id));
    assert_eq!(
        view.pane.value,
        detail(&omelette(), RecipeDetailStatus::Loading)
    );
    // The rows stay until the new list arrives.
    assert_eq!(view.list.status, ready(&[pasta(), shakshuka()]));

    f.stored = Ok(vec![omelette(), pasta(), shakshuka()]);
    f.settle(command).await?;
    let view = f.section.view();
    assert_eq!(view.pane.value, read(&omelette()));
    assert_eq!(view.list.status, ready(&[omelette(), pasta(), shakshuka()]));
    Ok(())
}

#[tokio::test]
async fn cancel_on_an_untouched_editor_empties_the_pane() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.new_recipe().await?;
    let command = f.editor(RecipeEditorEvent::RequestClose);
    f.settle(command).await?;
    assert_eq!(f.section.view().pane.value, RecipePaneView::Empty);
    assert!(f.section.view().can_create);
    Ok(())
}

#[tokio::test]
async fn cancel_goes_back_to_the_recipe_that_was_open_before_new_recipe() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.select(&shakshuka()).await?;
    f.new_recipe().await?;
    f.type_title("Toast").await?;
    let command = f.editor(RecipeEditorEvent::RequestClose);
    f.settle(command).await?;
    f.decide(ConfirmationEvent::Confirm).await?;
    let view = f.section.view();
    assert_eq!(view.selected, Some(shakshuka().id));
    assert_eq!(view.pane.value, read(&shakshuka()));
    Ok(())
}

#[tokio::test]
async fn a_row_click_on_an_untouched_editor_opens_the_recipe() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.new_recipe().await?;
    f.select(&pasta()).await?;
    assert_eq!(f.section.view().pane.value, read(&pasta()));
    Ok(())
}

#[tokio::test]
async fn a_row_click_with_a_typed_title_asks_first_then_discard_opens_the_clicked_recipe()
-> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.select(&shakshuka()).await?;
    f.new_recipe().await?;
    f.type_title("Toast").await?;
    f.select(&pasta()).await?;
    assert_eq!(f.editor_status()?, asking());

    f.decide(ConfirmationEvent::Confirm).await?;
    let view = f.section.view();
    assert_eq!(view.selected, Some(pasta().id));
    assert_eq!(view.pane.value, read(&pasta()));
    Ok(())
}

#[tokio::test]
async fn keep_editing_forgets_the_clicked_row() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.select(&shakshuka()).await?;
    f.new_recipe().await?;
    f.type_title("Toast").await?;
    f.select(&pasta()).await?;
    f.decide(ConfirmationEvent::Cancel).await?;
    assert_eq!(f.section.view().pane.value, create("Toast", "Toast"));

    // Cancel now goes back to Shakshuka, not to the clicked Pasta.
    let command = f.editor(RecipeEditorEvent::RequestClose);
    f.settle(command).await?;
    f.decide(ConfirmationEvent::Confirm).await?;
    assert_eq!(f.section.view().pane.value, read(&shakshuka()));
    Ok(())
}

#[tokio::test]
async fn edit_opens_the_editor_with_the_saved_title_and_keeps_the_row_selected()
-> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.select(&shakshuka()).await?;
    let read_scope = f.section.view().pane.scope;
    f.edit().await?;
    let view = f.section.view();
    assert_ne!(view.pane.scope, read_scope);
    assert_eq!(view.pane.value, editing(&shakshuka(), "Shakshuka"));
    assert_eq!(view.selected, Some(shakshuka().id));
    assert!(!view.can_create);
    Ok(())
}

#[tokio::test]
async fn saving_an_edit_shows_the_updated_read_view_and_loads_the_list_again() -> anyhow::Result<()>
{
    let mut f = Fixture::started().await?;
    f.select(&shakshuka()).await?;
    f.edit().await?;
    f.type_title("Spicy shakshuka").await?;
    let spicy = recipe(1, "Spicy shakshuka");
    let saved = f.save(Ok(spicy.clone())).await?;

    let command = f.section.update(Message::Outcome(saved));
    let view = f.section.view();
    assert_eq!(view.selected, Some(spicy.id));
    assert_eq!(view.pane.value, detail(&spicy, RecipeDetailStatus::Loading));

    f.stored = Ok(vec![pasta(), spicy.clone()]);
    f.settle(command).await?;
    let view = f.section.view();
    assert_eq!(view.pane.value, read(&spicy));
    assert_eq!(view.list.status, ready(&[pasta(), spicy]));
    assert!(view.can_create);
    Ok(())
}

#[tokio::test]
async fn cancel_on_an_unchanged_edit_goes_back_to_the_read_view() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.select(&shakshuka()).await?;
    f.edit().await?;
    let command = f.editor(RecipeEditorEvent::RequestClose);
    f.settle(command).await?;
    assert_eq!(f.section.view().pane.value, read(&shakshuka()));
    Ok(())
}

#[tokio::test]
async fn discarding_a_changed_edit_goes_back_to_the_read_view() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.select(&shakshuka()).await?;
    f.edit().await?;
    f.type_title("Toast").await?;
    let command = f.editor(RecipeEditorEvent::RequestClose);
    f.settle(command).await?;
    assert_eq!(f.editor_status()?, asking());

    f.decide(ConfirmationEvent::Confirm).await?;
    let view = f.section.view();
    assert_eq!(view.selected, Some(shakshuka().id));
    assert_eq!(view.pane.value, read(&shakshuka()));
    Ok(())
}

#[tokio::test]
async fn a_row_click_on_an_unchanged_edit_opens_the_clicked_recipe() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.select(&shakshuka()).await?;
    f.edit().await?;
    f.select(&pasta()).await?;
    let view = f.section.view();
    assert_eq!(view.selected, Some(pasta().id));
    assert_eq!(view.pane.value, read(&pasta()));
    Ok(())
}

#[tokio::test]
async fn a_row_click_on_a_changed_edit_asks_first_then_discard_opens_the_clicked_recipe()
-> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.select(&shakshuka()).await?;
    f.edit().await?;
    f.type_title("Toast").await?;
    f.select(&pasta()).await?;
    assert_eq!(f.editor_status()?, asking());

    f.decide(ConfirmationEvent::Confirm).await?;
    assert_eq!(f.section.view().pane.value, read(&pasta()));
    Ok(())
}

#[tokio::test]
async fn keep_editing_an_edit_forgets_the_clicked_row() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.select(&shakshuka()).await?;
    f.edit().await?;
    f.type_title("Toast").await?;
    f.select(&pasta()).await?;
    f.decide(ConfirmationEvent::Cancel).await?;
    assert_eq!(f.section.view().pane.value, editing(&shakshuka(), "Toast"));

    // Cancel now goes back to Shakshuka, not to the clicked Pasta.
    let command = f.editor(RecipeEditorEvent::RequestClose);
    f.settle(command).await?;
    f.decide(ConfirmationEvent::Confirm).await?;
    assert_eq!(f.section.view().pane.value, read(&shakshuka()));
    Ok(())
}

#[tokio::test]
async fn a_confirmed_delete_empties_the_pane_and_loads_the_list_again() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.select(&shakshuka()).await?;
    let read_scope = f.section.view().pane.scope;
    f.request_delete().await?;
    f.delete_result = Some(Ok(()));
    f.stored = Ok(vec![pasta()]);
    f.answer_delete(ConfirmationEvent::Confirm).await?;

    let view = f.section.view();
    assert_ne!(view.pane.scope, read_scope);
    assert_eq!(view.pane.value, RecipePaneView::Empty);
    assert_eq!(view.selected, None);
    assert_eq!(view.list.status, ready(&[pasta()]));
    assert!(view.can_create);
    Ok(())
}

#[tokio::test]
async fn cancel_keeps_the_read_view() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.select(&shakshuka()).await?;
    f.request_delete().await?;
    f.answer_delete(ConfirmationEvent::Cancel).await?;
    let view = f.section.view();
    assert_eq!(view.pane.value, read(&shakshuka()));
    assert!(view.can_create);
    Ok(())
}

#[tokio::test]
async fn a_search_keeps_the_open_read_view_and_its_selection() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.select(&shakshuka()).await?;
    let read_scope = f.section.view().pane.scope;

    // The search hides Shakshuka. Its read view stays, and Swift does not
    // highlight the hidden row.
    f.search("pas").await?;
    let view = f.section.view();
    assert_eq!(view.list.search, "pas");
    assert_eq!(view.list.status, ready(&[pasta()]));
    assert_eq!(view.pane.scope, read_scope);
    assert_eq!(view.pane.value, read(&shakshuka()));
    assert_eq!(view.selected, Some(shakshuka().id));

    f.search("").await?;
    let view = f.section.view();
    assert_eq!(view.list.status, ready(&[pasta(), shakshuka()]));
    assert_eq!(view.pane.value, read(&shakshuka()));
    Ok(())
}

#[tokio::test]
async fn a_save_during_a_search_keeps_the_draft_row_and_the_search() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.new_recipe().await?;
    f.type_title("Omelette").await?;
    f.search("sha").await?;
    // The search never hides the draft row.
    assert_eq!(f.section.view().pane.value, create("Omelette", "Omelette"));

    let saved = f.save(Ok(omelette())).await?;
    f.stored = Ok(vec![omelette(), pasta(), shakshuka()]);
    let command = f.section.update(Message::Outcome(saved));
    f.settle(command).await?;
    // The list loads again, and the search still applies to it.
    let view = f.section.view();
    assert_eq!(view.pane.value, read(&omelette()));
    assert_eq!(view.list.search, "sha");
    assert_eq!(view.list.status, ready(&[shakshuka()]));
    Ok(())
}

// Validation

#[tokio::test]
async fn a_click_on_the_edited_row_is_ignored() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.select(&shakshuka()).await?;
    f.edit().await?;
    f.type_title("Toast").await?;
    f.expect_ignored(RecipeSectionEvent::Select(shakshuka().id))
        .await
}

#[tokio::test]
async fn new_recipe_and_clear_selection_are_ignored_while_editing() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.select(&shakshuka()).await?;
    f.edit().await?;
    f.expect_ignored(RecipeSectionEvent::NewRecipe).await?;
    let scope = f.section.view().pane.scope;
    f.expect_ignored(RecipeSectionEvent::ClearSelection { scope })
        .await
}

#[tokio::test]
async fn an_edit_request_from_a_replaced_read_view_is_ignored() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.select(&shakshuka()).await?;
    let request = f.detail(RecipeDetailEvent::Edit);
    let open_edit = f.run_until_outcome(request).await?;
    f.select(&pasta()).await?;

    let before = f.section.view();
    let command = f.section.update(Message::Outcome(open_edit));
    assert_eq!(f.section.view(), before);
    f.done(command).await
}

#[tokio::test]
async fn the_pane_is_locked_while_the_delete_question_waits() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.select(&shakshuka()).await?;
    f.request_delete().await?;
    let view = f.section.view();
    assert_eq!(
        view.pane.value,
        detail(&shakshuka(), deletion(&shakshuka(), asking_delete(None)))
    );
    assert_eq!(view.selected, Some(shakshuka().id));
    assert!(!view.can_create);

    f.expect_ignored(RecipeSectionEvent::Select(pasta().id))
        .await?;
    f.expect_ignored(RecipeSectionEvent::NewRecipe).await?;
    let scope = view.pane.scope;
    f.expect_ignored(RecipeSectionEvent::ClearSelection { scope })
        .await
}

#[tokio::test]
async fn an_edit_request_from_before_the_delete_question_is_ignored() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.select(&shakshuka()).await?;
    let request = f.detail(RecipeDetailEvent::Edit);
    let open_edit = f.run_until_outcome(request).await?;
    f.request_delete().await?;

    let before = f.section.view();
    let command = f.section.update(Message::Outcome(open_edit));
    assert_eq!(f.section.view(), before);
    f.done(command).await
}

#[tokio::test]
async fn a_deleted_report_delivered_twice_is_applied_once() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.select(&shakshuka()).await?;
    f.request_delete().await?;
    f.delete_result = Some(Ok(()));
    let confirm = f.detail(RecipeDetailEvent::Confirmation(ConfirmationEvent::Confirm));
    let decided = f.run_until_outcome(confirm).await?;
    let delete = f.section.update(Message::Outcome(decided));
    let finished = f.run_until_outcome(delete).await?;
    let report = f.section.update(Message::Outcome(finished));
    let deleted = f.run_until_outcome(report).await?;

    f.stored = Ok(vec![pasta()]);
    let command = f.section.update(Message::Outcome(deleted.clone()));
    f.settle(command).await?;
    let before = f.section.view();
    assert_eq!(before.pane.value, RecipePaneView::Empty);

    // The empty pane has a new scope, so the old report is dropped.
    let command = f.section.update(Message::Outcome(deleted));
    assert_eq!(f.section.view(), before);
    f.done(command).await
}

#[tokio::test]
async fn new_recipe_is_ignored_while_the_editor_is_open() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.new_recipe().await?;
    f.type_title("Omelette").await?;
    f.expect_ignored(RecipeSectionEvent::NewRecipe).await
}

#[tokio::test]
async fn selecting_the_open_recipe_is_ignored() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.select(&shakshuka()).await?;
    f.expect_ignored(RecipeSectionEvent::Select(shakshuka().id))
        .await
}

#[tokio::test]
async fn selecting_a_recipe_the_list_does_not_show_is_ignored() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.expect_ignored(RecipeSectionEvent::Select(omelette().id))
        .await
}

#[tokio::test]
async fn selecting_a_recipe_the_search_hides_is_ignored() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.search("sha").await?;
    f.expect_ignored(RecipeSectionEvent::Select(pasta().id))
        .await
}

#[tokio::test]
async fn a_row_click_while_saving_is_ignored() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.new_recipe().await?;
    f.type_title("Omelette").await?;
    let _save = f.editor(RecipeEditorEvent::Save);
    assert_eq!(f.editor_status()?, RecipeEditorStatus::Saving);
    f.expect_ignored(RecipeSectionEvent::Select(pasta().id))
        .await
}

#[tokio::test]
async fn clear_selection_with_an_old_scope_is_ignored() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.select(&shakshuka()).await?;
    let old = f.section.view().pane.scope;
    f.select(&pasta()).await?;
    f.expect_ignored(RecipeSectionEvent::ClearSelection { scope: old })
        .await
}

#[tokio::test]
async fn clear_selection_is_ignored_while_the_editor_is_open() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.new_recipe().await?;
    let scope = f.section.view().pane.scope;
    f.expect_ignored(RecipeSectionEvent::ClearSelection { scope })
        .await
}

#[tokio::test]
async fn a_load_result_of_a_replaced_read_view_is_ignored() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    let first = f.send(RecipeSectionEvent::Select(shakshuka().id));
    let late = f.run_until_outcome(first).await?;
    let second = f.send(RecipeSectionEvent::Select(pasta().id));

    let command = f.section.update(Message::Outcome(late));
    f.done(command).await?;
    assert_eq!(
        f.section.view().pane.value,
        detail(&pasta(), RecipeDetailStatus::Loading)
    );
    f.settle(second).await?;
    assert_eq!(f.section.view().pane.value, read(&pasta()));
    Ok(())
}

#[tokio::test]
async fn a_detail_event_with_an_old_scope_is_ignored() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.stored = Err(RecipeLoadError::Storage);
    f.select(&shakshuka()).await?;
    let old = f.section.view().pane.scope;
    f.select(&pasta()).await?;
    f.expect_ignored(RecipeSectionEvent::Detail(Scoped::new(
        old,
        RecipeDetailEvent::Retry,
    )))
    .await
}

#[tokio::test]
async fn an_editor_event_with_an_old_scope_is_ignored() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    let empty_scope = f.section.view().pane.scope;
    f.new_recipe().await?;
    f.expect_ignored(RecipeSectionEvent::Editor(Scoped::new(
        empty_scope,
        RecipeEditorEvent::SetTitle("Omelette".into()),
    )))
    .await
}

#[tokio::test]
async fn an_outcome_of_a_closed_editor_is_ignored() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.new_recipe().await?;
    f.type_title("Omelette").await?;
    let saved = f.save(Ok(omelette())).await?;
    f.stored = Ok(vec![omelette(), pasta(), shakshuka()]);
    let command = f.section.update(Message::Outcome(saved.clone()));
    f.settle(command).await?;

    // The same Saved again belongs to the closed editor's scope.
    let command = f.section.update(Message::Outcome(saved));
    f.done(command).await?;
    assert_eq!(f.section.view().pane.value, read(&omelette()));
    Ok(())
}

#[tokio::test]
async fn a_save_during_the_first_load_shows_the_newest_list() -> anyhow::Result<()> {
    let mut f = Fixture::new(vec![]);
    // The first load reads the store before the save. Its result is still
    // queued when the save finishes.
    let first_load = f.section.init();
    let stale = f.run_until_outcome(first_load).await?;
    f.new_recipe().await?;
    f.type_title("Omelette").await?;
    let saved = f.save(Ok(omelette())).await?;
    f.stored = Ok(vec![omelette()]);
    let command = f.section.update(Message::Outcome(saved));
    f.settle(command).await?;

    let command = f.section.update(Message::Outcome(stale));
    f.done(command).await?;
    assert_eq!(f.section.view().list.status, ready(&[omelette()]));
    Ok(())
}

// Red cases

#[tokio::test]
async fn a_failed_load_after_a_save_shows_the_failure() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.new_recipe().await?;
    f.type_title("Omelette").await?;
    let saved = f.save(Ok(omelette())).await?;
    f.stored = Err(RecipeLoadError::Storage);
    let command = f.section.update(Message::Outcome(saved));
    f.settle(command).await?;
    let view = f.section.view();
    assert_eq!(
        view.list.status,
        RecipeListStatus::Failed(RecipeLoadError::Storage)
    );
    assert_eq!(
        view.pane.value,
        detail(
            &omelette(),
            RecipeDetailStatus::Failed(RecipeLoadError::Storage)
        )
    );
    Ok(())
}

#[tokio::test]
async fn a_failed_save_keeps_the_editor_open() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.new_recipe().await?;
    f.type_title("Shakshuka").await?;
    let save = f.editor(RecipeEditorEvent::Save);
    let finished = f
        .run_save(save, Err(RecipeSaveError::DuplicateTitle))
        .await?;
    let command = f.section.update(Message::Outcome(finished));
    f.settle(command).await?;
    assert!(matches!(
        f.editor_status()?,
        RecipeEditorStatus::Editing { .. }
    ));
    Ok(())
}

#[tokio::test]
async fn a_failed_update_keeps_the_edit_editor_open_with_its_error() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.select(&shakshuka()).await?;
    f.edit().await?;
    f.type_title("Pasta").await?;
    let finished = f
        .save_finished(Err(RecipeSaveError::DuplicateTitle))
        .await?;
    let command = f.section.update(Message::Outcome(finished));
    f.settle(command).await?;
    let view = f.section.view();
    assert_eq!(
        f.editor_status()?,
        RecipeEditorStatus::Editing {
            problem: Some(RecipeEditorProblem::SaveFailed(
                RecipeSaveError::DuplicateTitle
            ))
        }
    );
    assert_eq!(view.selected, Some(shakshuka().id));
    Ok(())
}

#[tokio::test]
async fn a_failed_delete_keeps_the_read_view_and_asks_again() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.select(&shakshuka()).await?;
    f.request_delete().await?;
    f.delete_result = Some(Err(RecipeDeleteError::Storage));
    f.answer_delete(ConfirmationEvent::Confirm).await?;

    let view = f.section.view();
    assert_eq!(
        view.pane.value,
        detail(
            &shakshuka(),
            deletion(
                &shakshuka(),
                asking_delete(Some(RecipeDeleteError::Storage))
            )
        )
    );
    assert_eq!(view.selected, Some(shakshuka().id));
    assert_eq!(view.list.status, ready(&[pasta(), shakshuka()]));
    Ok(())
}

#[tokio::test]
async fn a_recipe_that_no_longer_exists_shows_missing_and_loads_the_list_again()
-> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    // Another process deletes Shakshuka after the list loaded.
    f.stored = Ok(vec![pasta()]);
    f.select(&shakshuka()).await?;
    let view = f.section.view();
    assert_eq!(
        view.pane.value,
        detail(&shakshuka(), RecipeDetailStatus::Missing)
    );
    assert_eq!(view.list.status, ready(&[pasta()]));
    Ok(())
}

#[tokio::test]
async fn retry_in_a_failed_read_view_loads_the_recipe_again() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.stored = Err(RecipeLoadError::Transport);
    f.select(&shakshuka()).await?;
    f.stored = Ok(vec![pasta(), shakshuka()]);
    let scope = f.section.view().pane.scope;
    let command = f.send(RecipeSectionEvent::Detail(Scoped::new(
        scope,
        RecipeDetailEvent::Retry,
    )));
    f.settle(command).await?;
    assert_eq!(f.section.view().pane.value, read(&shakshuka()));
    Ok(())
}

// Test support

fn recipe(id: u128, title: &str) -> Recipe {
    Recipe {
        id: RecipeId::from(Uuid::from_u128(id)),
        title: title.into(),
        steps: vec![],
    }
}

/// Shakshuka has two steps, so the list row counts them and the read view
/// and the editor get them.
fn shakshuka() -> Recipe {
    Recipe {
        steps: vec![
            Step {
                id: StepId::from(Uuid::from_u128(11)),
                title: "Dice onion".into(),
                position: CanvasPoint { x: 10, y: 20 },
                duration: None,
                kind: StepKind::Active,
                instructions: None,
            },
            Step {
                id: StepId::from(Uuid::from_u128(12)),
                title: "Fry onion".into(),
                position: CanvasPoint { x: 200, y: 40 },
                duration: None,
                kind: StepKind::Active,
                instructions: None,
            },
        ],
        ..recipe(1, "Shakshuka")
    }
}

fn pasta() -> Recipe {
    recipe(2, "Pasta")
}

fn omelette() -> Recipe {
    recipe(3, "Omelette")
}

fn ready(recipes: &[Recipe]) -> RecipeListStatus {
    RecipeListStatus::Ready {
        rows: recipes
            .iter()
            .map(|recipe| RecipeListRow {
                id: recipe.id,
                title: recipe.title.clone(),
                step_count: summary(recipe).step_count,
            })
            .collect(),
    }
}

fn detail(recipe: &Recipe, status: RecipeDetailStatus) -> RecipePaneView {
    RecipePaneView::Detail(RecipeDetailView {
        id: recipe.id,
        status,
    })
}

/// The loaded read view of `recipe`.
fn read(recipe: &Recipe) -> RecipePaneView {
    detail(
        recipe,
        RecipeDetailStatus::Ready {
            title: recipe.title.clone(),
            steps: detail_steps(recipe),
            deletion: RecipeDeletion::Idle,
        },
    )
}

fn detail_steps(recipe: &Recipe) -> Vec<RecipeDetailStep> {
    recipe
        .steps
        .iter()
        .map(|step| RecipeDetailStep {
            id: step.id,
            title: step.title.clone(),
        })
        .collect()
}

/// The read view of `recipe` with this delete state.
fn deletion(recipe: &Recipe, deletion: RecipeDeletion) -> RecipeDetailStatus {
    RecipeDetailStatus::Ready {
        title: recipe.title.clone(),
        steps: detail_steps(recipe),
        deletion,
    }
}

/// "Delete …?" waits, after the failed delete `error` if any.
fn asking_delete(error: Option<RecipeDeleteError>) -> RecipeDeletion {
    RecipeDeletion::Confirming {
        confirmation: ConfirmationView::AwaitingDecision,
        error,
    }
}

fn create(title: &str, draft_row: &str) -> RecipePaneView {
    RecipePaneView::Create {
        editor: RecipeEditorView {
            title: title.into(),
            canvas: StepCanvasView::Unselected { steps: vec![] },
            status: RecipeEditorStatus::Editing { problem: None },
        },
        draft_row: draft_row.into(),
    }
}

/// The edit editor of `saved`, with `title` typed. It opens with the saved
/// steps and no selection.
fn editing(saved: &Recipe, title: &str) -> RecipePaneView {
    let steps = saved
        .steps
        .iter()
        .map(|step| StepNodeView {
            id: step.id,
            title: step.title.clone(),
            label: StepLabel::Titled {
                title: step.title.clone(),
            },
            position: step.position,
            problem: None,
        })
        .collect();
    RecipePaneView::Edit {
        editor: RecipeEditorView {
            title: title.into(),
            canvas: StepCanvasView::Unselected { steps },
            status: RecipeEditorStatus::Editing { problem: None },
        },
        saved_title: saved.title.clone(),
    }
}

/// The editor asks "Discard changes?".
fn asking() -> RecipeEditorStatus {
    RecipeEditorStatus::ConfirmingDiscard(ConfirmationView::AwaitingDecision)
}

/// What `RecipeStore::list` returns for `recipe`.
fn summary(recipe: &Recipe) -> RecipeSummary {
    RecipeSummary {
        id: recipe.id,
        title: recipe.title.clone(),
        step_count: u32::try_from(recipe.steps.len()).unwrap_or(u32::MAX),
    }
}

/// Answers a list or get request from `stored`, and a delete with `deleted`.
/// A create or an update fails the test: saves go through
/// `Fixture::run_save`.
async fn answer_store(
    stored: Result<Vec<Recipe>, RecipeLoadError>,
    deleted: Option<Result<(), RecipeDeleteError>>,
    effect: Envelope<RecipeStoreRequest>,
) -> anyhow::Result<()> {
    match effect.request {
        RecipeStoreRequest::List => {
            let summaries = stored.map(|recipes| recipes.iter().map(summary).collect());
            effect.resolve_list(|| async { summaries }).await?
        }
        RecipeStoreRequest::Get { id } => {
            let found = stored.map(|recipes| recipes.into_iter().find(|r| r.id == id));
            effect.resolve_get(|_| async { found }).await?
        }
        RecipeStoreRequest::Create { .. } => anyhow::bail!("an unexpected create"),
        RecipeStoreRequest::Update { .. } => anyhow::bail!("an unexpected update"),
        RecipeStoreRequest::Delete { .. } => {
            let Some(result) = deleted else {
                anyhow::bail!("an unexpected delete")
            };
            effect.resolve_delete(|_| async { result }).await?
        }
    }
    Ok(())
}

struct Fixture {
    section: RecipeSection,
    effects: RecipeEffectStream,
    /// What the store has: `list` returns it, `get` looks in it.
    stored: Result<Vec<Recipe>, RecipeLoadError>,
    /// What `delete` answers. `None`: a delete fails the test.
    delete_result: Option<Result<(), RecipeDeleteError>>,
}

impl Fixture {
    fn new(stored: Vec<Recipe>) -> Self {
        let (section, effects) = RecipeSection::test_fixture();
        Self {
            section,
            effects,
            stored: Ok(stored),
            delete_result: None,
        }
    }

    /// A section whose list shows Pasta and Shakshuka.
    async fn started() -> anyhow::Result<Self> {
        let mut f = Self::new(vec![pasta(), shakshuka()]);
        let load = f.section.init();
        f.settle(load).await?;
        Ok(f)
    }

    fn send(&mut self, event: RecipeSectionEvent) -> Command<RecipeSectionOutcome> {
        self.section.update(Message::Event(event))
    }

    /// An editor event, sent with the pane's current scope.
    fn editor(&mut self, event: RecipeEditorEvent) -> Command<RecipeSectionOutcome> {
        let RecipeSectionView { pane, .. } = self.section.view();
        self.send(RecipeSectionEvent::Editor(Scoped::new(pane.scope, event)))
    }

    /// A read view event, sent with the pane's current scope.
    fn detail(&mut self, event: RecipeDetailEvent) -> Command<RecipeSectionOutcome> {
        let RecipeSectionView { pane, .. } = self.section.view();
        self.send(RecipeSectionEvent::Detail(Scoped::new(pane.scope, event)))
    }

    fn editor_status(&self) -> anyhow::Result<RecipeEditorStatus> {
        match self.section.view().pane.value {
            RecipePaneView::Create { editor, .. } | RecipePaneView::Edit { editor, .. } => {
                Ok(editor.status)
            }
            pane => anyhow::bail!("expected the editor, found {pane:?}"),
        }
    }

    async fn done(&mut self, command: Command<RecipeSectionOutcome>) -> anyhow::Result<()> {
        command
            .driver(&mut self.effects)
            .step()
            .await?
            .expect_done();
        Ok(())
    }

    /// Sends `event` and checks that nothing changes and nothing runs.
    async fn expect_ignored(&mut self, event: RecipeSectionEvent) -> anyhow::Result<()> {
        let before = self.section.view();
        let command = self.send(event);
        assert_eq!(self.section.view(), before);
        self.done(command).await
    }

    /// Runs `command` to its end: renders resolve and the store answers from
    /// `stored`. Returns its outcomes, not yet dispatched.
    async fn run(
        &mut self,
        command: Command<RecipeSectionOutcome>,
    ) -> anyhow::Result<Vec<RecipeSectionOutcome>> {
        let stored = self.stored.clone();
        let deleted = self.delete_result.clone();
        let mut command = command.driver(&mut self.effects);
        let mut outcomes = vec![];
        loop {
            match command.step().await? {
                CommandEffect::Effect(RecipeEffect::Render(render)) => {
                    render.resolve_render(|| async {}).await?
                }
                CommandEffect::Effect(RecipeEffect::Store(store)) => {
                    answer_store(stored.clone(), deleted.clone(), store).await?
                }
                CommandEffect::Outcome(outcome) => outcomes.push(outcome),
                CommandEffect::Done => break,
            }
        }
        Ok(outcomes)
    }

    /// Runs `command` to its one outcome. Returns it, not yet dispatched.
    async fn run_until_outcome(
        &mut self,
        command: Command<RecipeSectionOutcome>,
    ) -> anyhow::Result<RecipeSectionOutcome> {
        let mut outcomes = self.run(command).await?;
        match (outcomes.pop(), outcomes.is_empty()) {
            (Some(outcome), true) => Ok(outcome),
            _ => anyhow::bail!("expected one outcome"),
        }
    }

    /// Runs `command`, dispatches each outcome, and runs what follows, until
    /// nothing is left.
    async fn settle(&mut self, command: Command<RecipeSectionOutcome>) -> anyhow::Result<()> {
        let mut queue = vec![command];
        while let Some(command) = queue.pop() {
            for outcome in self.run(command).await? {
                queue.push(self.section.update(Message::Outcome(outcome)));
            }
        }
        Ok(())
    }

    async fn select(&mut self, recipe: &Recipe) -> anyhow::Result<()> {
        let command = self.send(RecipeSectionEvent::Select(recipe.id));
        self.settle(command).await
    }

    /// Types `search` in the list's search field.
    async fn search(&mut self, search: &str) -> anyhow::Result<()> {
        let command = self.send(RecipeSectionEvent::List(RecipeListEvent::Search(
            search.into(),
        )));
        self.settle(command).await
    }

    async fn new_recipe(&mut self) -> anyhow::Result<()> {
        let command = self.send(RecipeSectionEvent::NewRecipe);
        self.settle(command).await
    }

    /// Edit in the open read view, and what follows.
    async fn edit(&mut self) -> anyhow::Result<()> {
        let command = self.detail(RecipeDetailEvent::Edit);
        self.settle(command).await
    }

    /// Delete in the open read view: the question opens.
    async fn request_delete(&mut self) -> anyhow::Result<()> {
        let command = self.detail(RecipeDetailEvent::RequestDelete);
        self.settle(command).await
    }

    /// Answers "Delete …?" and runs what follows. The store answers a
    /// delete with `delete_result`.
    async fn answer_delete(&mut self, event: ConfirmationEvent) -> anyhow::Result<()> {
        let command = self.detail(RecipeDetailEvent::Confirmation(event));
        self.settle(command).await
    }

    async fn type_title(&mut self, title: &str) -> anyhow::Result<()> {
        let command = self.editor(RecipeEditorEvent::SetTitle(title.into()));
        self.settle(command).await
    }

    /// Answers "Discard changes?" and runs what follows.
    async fn decide(&mut self, event: ConfirmationEvent) -> anyhow::Result<()> {
        let command = self.editor(RecipeEditorEvent::Confirmation(event));
        self.settle(command).await
    }

    /// Runs a save command with `result`, for a create or an update. Returns
    /// `SaveFinished`, not yet dispatched.
    async fn run_save(
        &mut self,
        save: Command<RecipeSectionOutcome>,
        result: Result<Recipe, RecipeSaveError>,
    ) -> anyhow::Result<RecipeSectionOutcome> {
        let mut command = save.driver(&mut self.effects);
        command
            .step()
            .await?
            .expect_effect()
            .expect_render()
            .resolve_render(|| async {})
            .await?;
        let store = command.step().await?.expect_effect().expect_store();
        match store.request {
            RecipeStoreRequest::Create { .. } => store.resolve_create(|_| async { result }).await?,
            RecipeStoreRequest::Update { .. } => {
                store.resolve_update(|_, _| async { result }).await?
            }
            _ => anyhow::bail!("expected a create or an update"),
        }
        let outcome = command.step().await?.expect_outcome();
        command.step().await?.expect_done();
        Ok(outcome)
    }

    /// Saves the editor with `result`. Returns `SaveFinished`, not yet
    /// dispatched.
    async fn save_finished(
        &mut self,
        result: Result<Recipe, RecipeSaveError>,
    ) -> anyhow::Result<RecipeSectionOutcome> {
        let save = self.editor(RecipeEditorEvent::Save);
        self.run_save(save, result).await
    }

    /// Saves the editor with `result`. Returns the editor's `Saved`, not yet
    /// dispatched to the section.
    async fn save(
        &mut self,
        result: Result<Recipe, RecipeSaveError>,
    ) -> anyhow::Result<RecipeSectionOutcome> {
        let finished = self.save_finished(result).await?;
        let command = self.section.update(Message::Outcome(finished));
        self.run_until_outcome(command).await
    }
}
