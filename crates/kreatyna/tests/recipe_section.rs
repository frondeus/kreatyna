use kreatyna::{
    capabilities::ResolveRender,
    recipes::{
        Recipe, RecipeEditorEvent, RecipeEditorStatus, RecipeEditorView, RecipeId, RecipeListRow,
        RecipeListStatus, RecipePaneView, RecipeSection, RecipeSectionEvent, RecipeSectionOutcome,
        RecipeSectionView,
        capabilities::{
            RecipeEffect, RecipeEffectStream, RecipeLoadError, RecipeSaveError, ResolveRecipeStore,
        },
    },
};
use krucyfiks::{
    Command, Message, Scoped, Widget,
    effects::{CommandEffect, CommandTestExt},
};
use uuid::Uuid;

// Green path

#[tokio::test]
async fn the_section_starts_with_the_list_and_an_empty_pane() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    let load = f.section.init();
    f.load(load, Ok(vec![shakshuka()])).await?;
    let view = f.section.view();
    assert_eq!(view.list.status, ready(&[shakshuka()]));
    assert_eq!(view.pane.value, RecipePaneView::Empty);
    assert!(view.can_create);
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
    f.render(command).await
}

#[tokio::test]
async fn the_draft_row_shows_the_typed_title_trimmed() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.new_recipe().await?;
    f.type_title("  Pasta  ").await?;
    assert_eq!(f.section.view().pane.value, create("  Pasta  ", "Pasta"));
    Ok(())
}

#[tokio::test]
async fn saving_empties_the_pane_and_loads_the_list_again() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.new_recipe().await?;
    f.type_title("Pasta").await?;
    let saved = f.save(Ok(pasta())).await?;

    let command = f.section.update(Message::Outcome(saved));
    let view = f.section.view();
    assert_eq!(view.pane.value, RecipePaneView::Empty);
    assert!(view.can_create);
    // The rows stay until the new list arrives.
    assert_eq!(view.list.status, ready(&[shakshuka()]));
    f.render_and_load(command, Ok(vec![pasta(), shakshuka()]))
        .await?;
    assert_eq!(f.section.view().list.status, ready(&[pasta(), shakshuka()]));
    Ok(())
}

#[tokio::test]
async fn cancel_on_an_untouched_editor_empties_the_pane() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.new_recipe().await?;
    let close = f.editor(RecipeEditorEvent::RequestClose);
    let discarded = f.render_then_outcome(close).await?;

    let command = f.section.update(Message::Outcome(discarded));
    assert_eq!(f.section.view().pane.value, RecipePaneView::Empty);
    assert!(f.section.view().can_create);
    f.render(command).await
}

// Validation

#[tokio::test]
async fn new_recipe_is_ignored_while_the_editor_is_open() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.new_recipe().await?;
    f.type_title("Pasta").await?;
    let before = f.section.view();
    let command = f.send(RecipeSectionEvent::NewRecipe);
    assert_eq!(f.section.view(), before);
    f.done(command).await
}

#[tokio::test]
async fn an_editor_event_with_an_old_scope_is_ignored() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    let empty_scope = f.section.view().pane.scope;
    f.new_recipe().await?;
    let before = f.section.view();
    let command = f.send(RecipeSectionEvent::Editor(Scoped::new(
        empty_scope,
        RecipeEditorEvent::SetTitle("Pasta".into()),
    )));
    assert_eq!(f.section.view(), before);
    f.done(command).await
}

#[tokio::test]
async fn an_outcome_of_a_closed_editor_is_ignored() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.new_recipe().await?;
    f.type_title("Pasta").await?;
    let saved = f.save(Ok(pasta())).await?;
    let command = f.section.update(Message::Outcome(saved.clone()));
    f.render_and_load(command, Ok(vec![pasta(), shakshuka()]))
        .await?;

    // The same Saved again belongs to the closed editor's scope.
    let command = f.section.update(Message::Outcome(saved));
    f.done(command).await?;
    assert_eq!(f.section.view().pane.value, RecipePaneView::Empty);
    Ok(())
}

#[tokio::test]
async fn a_save_during_the_first_load_shows_the_newest_list() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    // The first load reads the store before the save. Its result is still
    // queued when the save finishes.
    let first_load = f.section.init();
    let stale = f.run_load(first_load, Ok(vec![])).await?;
    f.new_recipe().await?;
    f.type_title("Pasta").await?;
    let saved = f.save(Ok(pasta())).await?;
    let command = f.section.update(Message::Outcome(saved));
    f.render_and_load(command, Ok(vec![pasta()])).await?;

    let command = f.section.update(Message::Outcome(stale));
    f.done(command).await?;
    assert_eq!(f.section.view().list.status, ready(&[pasta()]));
    Ok(())
}

// Red cases

#[tokio::test]
async fn a_failed_load_after_a_save_shows_the_failure() -> anyhow::Result<()> {
    let mut f = Fixture::started().await?;
    f.new_recipe().await?;
    f.type_title("Pasta").await?;
    let saved = f.save(Ok(pasta())).await?;
    let command = f.section.update(Message::Outcome(saved));
    f.render_and_load(command, Err(RecipeLoadError::Storage))
        .await?;
    assert_eq!(
        f.section.view().list.status,
        RecipeListStatus::Failed(RecipeLoadError::Storage)
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
    f.render(command).await?;
    let RecipePaneView::Create { editor, .. } = f.section.view().pane.value else {
        anyhow::bail!("expected the editor to stay open");
    };
    assert!(matches!(editor.status, RecipeEditorStatus::Editing { .. }));
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

fn create(title: &str, draft_row: &str) -> RecipePaneView {
    RecipePaneView::Create {
        editor: RecipeEditorView {
            title: title.into(),
            status: RecipeEditorStatus::Editing { problem: None },
        },
        draft_row: draft_row.into(),
    }
}

struct Fixture {
    section: RecipeSection,
    effects: RecipeEffectStream,
}

impl Fixture {
    fn new() -> Self {
        let (section, effects) = RecipeSection::test_fixture();
        Self { section, effects }
    }

    /// A section whose list shows Shakshuka.
    async fn started() -> anyhow::Result<Self> {
        let mut f = Self::new();
        let load = f.section.init();
        f.load(load, Ok(vec![shakshuka()])).await?;
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

    async fn done(&mut self, command: Command<RecipeSectionOutcome>) -> anyhow::Result<()> {
        command
            .driver(&mut self.effects)
            .step()
            .await?
            .expect_done();
        Ok(())
    }

    /// Runs a command that only renders.
    async fn render(&mut self, command: Command<RecipeSectionOutcome>) -> anyhow::Result<()> {
        let mut command = command.driver(&mut self.effects);
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

    /// Runs a command that renders, then reports one outcome.
    async fn render_then_outcome(
        &mut self,
        command: Command<RecipeSectionOutcome>,
    ) -> anyhow::Result<RecipeSectionOutcome> {
        let mut command = command.driver(&mut self.effects);
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

    async fn new_recipe(&mut self) -> anyhow::Result<()> {
        let command = self.send(RecipeSectionEvent::NewRecipe);
        self.render(command).await
    }

    async fn type_title(&mut self, title: &str) -> anyhow::Result<()> {
        let command = self.editor(RecipeEditorEvent::SetTitle(title.into()));
        self.render(command).await
    }

    /// Runs a load command with `result`. Returns its outcome, not yet
    /// dispatched.
    async fn run_load(
        &mut self,
        load: Command<RecipeSectionOutcome>,
        result: Result<Vec<Recipe>, RecipeLoadError>,
    ) -> anyhow::Result<RecipeSectionOutcome> {
        let mut command = load.driver(&mut self.effects);
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

    /// Runs a load command with `result` and applies it.
    async fn load(
        &mut self,
        load: Command<RecipeSectionOutcome>,
        result: Result<Vec<Recipe>, RecipeLoadError>,
    ) -> anyhow::Result<()> {
        let finished = self.run_load(load, result).await?;
        let command = self.section.update(Message::Outcome(finished));
        self.render(command).await
    }

    /// Runs the command that follows a save: the empty pane's render and the
    /// list load, in any order. Then applies the load's result.
    async fn render_and_load(
        &mut self,
        command: Command<RecipeSectionOutcome>,
        result: Result<Vec<Recipe>, RecipeLoadError>,
    ) -> anyhow::Result<()> {
        let mut command = command.driver(&mut self.effects);
        let mut outcomes = vec![];
        let mut result = Some(result);
        loop {
            match command.step().await? {
                CommandEffect::Effect(RecipeEffect::Render(render)) => {
                    render.resolve_render(|| async {}).await?
                }
                CommandEffect::Effect(RecipeEffect::Store(store)) => {
                    let result = result.take().ok_or(anyhow::anyhow!("a second list call"))?;
                    store.resolve_list(|| async { result }).await?;
                }
                CommandEffect::Outcome(outcome) => outcomes.push(outcome),
                CommandEffect::Done => break,
            }
        }
        drop(command);
        for outcome in outcomes {
            let command = self.section.update(Message::Outcome(outcome));
            self.render(command).await?;
        }
        Ok(())
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
        self.render_then_outcome(command).await
    }
}
