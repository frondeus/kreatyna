use kreatyna::{
    capabilities::ResolveRender,
    confirmation::{ConfirmationEvent, ConfirmationView},
    recipes::{
        Recipe, RecipeDetailEvent, RecipeDetailStatus, RecipeDetailView, RecipeEditorEvent,
        RecipeEditorStatus, RecipeEditorView, RecipeId, RecipeListRow, RecipeListStatus,
        RecipePaneView, RecipeSection, RecipeSectionEvent, RecipeSectionOutcome, RecipeSectionView,
        capabilities::{
            RecipeEffect, RecipeEffectStream, RecipeLoadError, RecipeSaveError, RecipeStoreRequest,
            ResolveRecipeStore,
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

// Validation

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
    }
}

fn shakshuka() -> Recipe {
    recipe(1, "Shakshuka")
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
        },
    )
}

fn create(title: &str, draft_row: &str) -> RecipePaneView {
    RecipePaneView::Create {
        editor: RecipeEditorView {
            title: title.into(),
            status: RecipeEditorStatus::Editing { problem: None },
        },
        draft_row: draft_row.into(),
    }
}

/// The editor asks "Discard changes?".
fn asking() -> RecipeEditorStatus {
    RecipeEditorStatus::ConfirmingDiscard(ConfirmationView::AwaitingDecision)
}

/// Answers a list or get request from `stored`. A create fails the test:
/// saves go through `Fixture::run_save`.
async fn answer_store(
    stored: Result<Vec<Recipe>, RecipeLoadError>,
    effect: Envelope<RecipeStoreRequest>,
) -> anyhow::Result<()> {
    match effect.request {
        RecipeStoreRequest::List => effect.resolve_list(|| async { stored }).await?,
        RecipeStoreRequest::Get { id } => {
            let found = stored.map(|recipes| recipes.into_iter().find(|r| r.id == id));
            effect.resolve_get(|_| async { found }).await?
        }
        RecipeStoreRequest::Create { .. } => anyhow::bail!("an unexpected create"),
    }
    Ok(())
}

struct Fixture {
    section: RecipeSection,
    effects: RecipeEffectStream,
    /// What the store has: `list` returns it, `get` looks in it.
    stored: Result<Vec<Recipe>, RecipeLoadError>,
}

impl Fixture {
    fn new(stored: Vec<Recipe>) -> Self {
        let (section, effects) = RecipeSection::test_fixture();
        Self {
            section,
            effects,
            stored: Ok(stored),
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

    fn editor_status(&self) -> anyhow::Result<RecipeEditorStatus> {
        match self.section.view().pane.value {
            RecipePaneView::Create { editor, .. } => Ok(editor.status),
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
        let mut command = command.driver(&mut self.effects);
        let mut outcomes = vec![];
        loop {
            match command.step().await? {
                CommandEffect::Effect(RecipeEffect::Render(render)) => {
                    render.resolve_render(|| async {}).await?
                }
                CommandEffect::Effect(RecipeEffect::Store(store)) => {
                    answer_store(stored.clone(), store).await?
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

    async fn new_recipe(&mut self) -> anyhow::Result<()> {
        let command = self.send(RecipeSectionEvent::NewRecipe);
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

    /// Runs a save command with `result`. Returns `SaveFinished`, not yet
    /// dispatched.
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
        command
            .step()
            .await?
            .expect_effect()
            .expect_store()
            .resolve_create(|_| async { result })
            .await?;
        let outcome = command.step().await?.expect_outcome();
        command.step().await?.expect_done();
        Ok(outcome)
    }

    /// Saves the editor with `result`. Returns the editor's `Saved`, not yet
    /// dispatched to the section.
    async fn save(
        &mut self,
        result: Result<Recipe, RecipeSaveError>,
    ) -> anyhow::Result<RecipeSectionOutcome> {
        let save = self.editor(RecipeEditorEvent::Save);
        let finished = self.run_save(save, result).await?;
        let command = self.section.update(Message::Outcome(finished));
        self.run_until_outcome(command).await
    }
}
