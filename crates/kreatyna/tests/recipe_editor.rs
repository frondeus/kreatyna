use kreatyna::{
    capabilities::ResolveRender,
    confirmation::{ConfirmationEvent, ConfirmationView},
    recipes::{
        Recipe, RecipeEditor, RecipeEditorEvent, RecipeEditorOutcome, RecipeEditorProblem,
        RecipeEditorStatus, RecipeEditorView, RecipeId, RecipeTitleError,
        capabilities::{
            RecipeEffectStream, RecipeSaveError, RecipeStoreRequest, ResolveRecipeStore,
        },
    },
};
use krucyfiks::{Command, Message, Widget, effects::CommandTestExt, test_utils::EmptyInitTest};
use test_case::test_case;
use uuid::Uuid;

// Green path

#[tokio::test]
async fn a_new_editor_has_an_empty_title_and_starts_nothing() {
    let mut f = Fixture::new();
    assert_eq!(f.editor.view(), editing("", None));
    f.editor.expect_noop_init().await;
}

#[tokio::test]
async fn typing_keeps_the_raw_title_and_renders() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    let command = f.send(RecipeEditorEvent::SetTitle("  Shakshuka ".into()));
    assert_eq!(f.editor.view(), editing("  Shakshuka ", None));
    f.render(command).await
}

#[tokio::test]
async fn save_creates_the_trimmed_title_then_reports_saved() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    f.type_title("  Shakshuka ").await?;

    let save = f.send(RecipeEditorEvent::Save);
    assert_eq!(f.editor.view().status, RecipeEditorStatus::Saving);
    let finished = f.save(save, Ok(shakshuka())).await?;
    assert_eq!(f.writes, [Write::Created("Shakshuka".into())]);

    let command = f.editor.update(Message::Outcome(finished));
    assert_eq!(f.editor.view().status, RecipeEditorStatus::Finished);
    assert_eq!(
        f.render_then_outcome(command).await?,
        RecipeEditorOutcome::Saved {
            recipe: shakshuka()
        }
    );
    Ok(())
}

#[tokio::test]
async fn cancel_on_an_untouched_editor_discards_at_once() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    let command = f.send(RecipeEditorEvent::RequestClose);
    assert_eq!(f.editor.view().status, RecipeEditorStatus::Finished);
    assert_eq!(
        f.render_then_outcome(command).await?,
        RecipeEditorOutcome::Discarded
    );
    Ok(())
}

#[tokio::test]
async fn cancel_after_typing_asks_and_confirming_discards() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    f.type_title("Shakshuka").await?;
    f.request_close().await?;

    let decided = f.decide(ConfirmationEvent::Confirm).await?;
    let command = f.editor.update(Message::Outcome(decided));
    assert_eq!(f.editor.view().status, RecipeEditorStatus::Finished);
    assert_eq!(
        f.render_then_outcome(command).await?,
        RecipeEditorOutcome::Discarded
    );
    Ok(())
}

#[tokio::test]
async fn an_edit_editor_opens_with_the_saved_title_and_starts_nothing() {
    let mut f = Fixture::edit();
    assert_eq!(f.editor.view(), editing("Shakshuka", None));
    f.editor.expect_noop_init().await;
}

#[test_case(Some(" Spicy shakshuka "), "Spicy shakshuka"; "a new title")]
#[test_case(None, "Shakshuka"; "the saved title")]
#[tokio::test]
async fn save_in_an_edit_editor_updates_the_recipe_by_id_then_reports_saved(
    typed: Option<&str>,
    title: &str,
) -> anyhow::Result<()> {
    let mut f = Fixture::edit();
    if let Some(text) = typed {
        f.type_title(text).await?;
    }
    let saved = Recipe {
        title: title.into(),
        ..shakshuka()
    };

    let save = f.send(RecipeEditorEvent::Save);
    let finished = f.save(save, Ok(saved.clone())).await?;
    assert_eq!(f.writes, [Write::Updated(shakshuka().id, title.into())]);

    let command = f.editor.update(Message::Outcome(finished));
    assert_eq!(
        f.render_then_outcome(command).await?,
        RecipeEditorOutcome::Saved { recipe: saved }
    );
    Ok(())
}

#[tokio::test]
async fn cancel_on_an_unchanged_edit_editor_discards_at_once() -> anyhow::Result<()> {
    let mut f = Fixture::edit();
    let command = f.send(RecipeEditorEvent::RequestClose);
    assert_eq!(
        f.render_then_outcome(command).await?,
        RecipeEditorOutcome::Discarded
    );
    Ok(())
}

#[tokio::test]
async fn cancel_after_changing_the_saved_title_asks() -> anyhow::Result<()> {
    let mut f = Fixture::edit();
    f.type_title("Spicy shakshuka").await?;
    f.request_close().await
}

// Validation

#[tokio::test]
async fn typing_the_saved_title_again_makes_the_edit_editor_unchanged() -> anyhow::Result<()> {
    let mut f = Fixture::edit();
    f.type_title("Pasta").await?;
    f.type_title("Shakshuka").await?;
    let command = f.send(RecipeEditorEvent::RequestClose);
    assert_eq!(
        f.render_then_outcome(command).await?,
        RecipeEditorOutcome::Discarded
    );
    Ok(())
}

#[test_case(&[], ""; "nothing typed")]
#[test_case(&["   "], "   "; "spaces only")]
#[tokio::test]
async fn save_with_a_blank_title_shows_the_issue_and_does_not_call_the_store(
    typed: &[&str],
    title: &str,
) -> anyhow::Result<()> {
    let mut f = Fixture::new();
    for text in typed {
        f.type_title(text).await?;
    }
    let command = f.send(RecipeEditorEvent::Save);
    assert_eq!(f.editor.view(), editing(title, Some(empty_title())));
    // One render and nothing else: no store call.
    f.render(command).await
}

#[tokio::test]
async fn before_the_first_save_a_blank_title_shows_no_issue() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    f.type_title("S").await?;
    f.type_title("").await?;
    assert_eq!(f.editor.view(), editing("", None));
    Ok(())
}

#[tokio::test]
async fn after_the_first_save_every_change_checks_the_title_again() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    let command = f.send(RecipeEditorEvent::Save);
    f.render(command).await?;

    f.type_title("S").await?;
    assert_eq!(f.editor.view(), editing("S", None));
    f.type_title(" ").await?;
    assert_eq!(f.editor.view(), editing(" ", Some(empty_title())));
    Ok(())
}

#[tokio::test]
async fn typing_the_same_title_again_does_nothing() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    f.type_title("Shakshuka").await?;
    let command = f.send(RecipeEditorEvent::SetTitle("Shakshuka".into()));
    command.driver(&mut f.effects).step().await?.expect_done();
    Ok(())
}

#[tokio::test]
async fn keep_editing_returns_to_the_editor_with_its_problem() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    f.type_title(" ").await?;
    let command = f.send(RecipeEditorEvent::Save);
    f.render(command).await?;
    f.request_close().await?;

    let decided = f.decide(ConfirmationEvent::Cancel).await?;
    let command = f.editor.update(Message::Outcome(decided));
    assert_eq!(f.editor.view(), editing(" ", Some(empty_title())));
    f.render(command).await
}

#[tokio::test]
async fn edits_save_and_cancel_are_ignored_while_saving() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    f.type_title("Shakshuka").await?;
    let _save = f.send(RecipeEditorEvent::Save);
    for event in [
        RecipeEditorEvent::SetTitle("Pasta".into()),
        RecipeEditorEvent::Save,
        RecipeEditorEvent::RequestClose,
    ] {
        let command = f.send(event);
        command.driver(&mut f.effects).step().await?.expect_done();
    }
    assert_eq!(
        f.editor.view(),
        RecipeEditorView {
            title: "Shakshuka".into(),
            status: RecipeEditorStatus::Saving,
        }
    );
    Ok(())
}

#[tokio::test]
async fn a_result_of_an_earlier_save_is_ignored() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    f.type_title("Shakshuka").await?;
    let first = f.send(RecipeEditorEvent::Save);
    let failed = f.save(first, Err(RecipeSaveError::Storage)).await?;
    let command = f.editor.update(Message::Outcome(failed.clone()));
    f.render(command).await?;

    let _second = f.send(RecipeEditorEvent::Save);
    let command = f.editor.update(Message::Outcome(failed));
    command.driver(&mut f.effects).step().await?.expect_done();
    assert_eq!(f.editor.view().status, RecipeEditorStatus::Saving);
    Ok(())
}

// Red cases

#[test_case(RecipeSaveError::DuplicateTitle; "duplicate title")]
#[test_case(RecipeSaveError::Storage; "storage")]
#[test_case(RecipeSaveError::Transport; "transport")]
#[tokio::test]
async fn a_failed_save_shows_its_error_until_the_title_changes(
    error: RecipeSaveError,
) -> anyhow::Result<()> {
    let mut f = Fixture::new();
    f.type_title("shakshuka").await?;
    let save = f.send(RecipeEditorEvent::Save);
    let failed = f.save(save, Err(error.clone())).await?;

    let command = f.editor.update(Message::Outcome(failed));
    assert_eq!(
        f.editor.view(),
        editing("shakshuka", Some(RecipeEditorProblem::SaveFailed(error)))
    );
    f.render(command).await?;

    f.type_title("Shakshuka (spicy)").await?;
    assert_eq!(f.editor.view(), editing("Shakshuka (spicy)", None));
    Ok(())
}

#[test_case(RecipeSaveError::NotFound; "recipe no longer exists")]
#[test_case(RecipeSaveError::DuplicateTitle; "duplicate title")]
#[tokio::test]
async fn a_failed_update_keeps_the_edit_editor_open_with_its_error(
    error: RecipeSaveError,
) -> anyhow::Result<()> {
    let mut f = Fixture::edit();
    f.type_title("Pasta").await?;
    let save = f.send(RecipeEditorEvent::Save);
    let failed = f.save(save, Err(error.clone())).await?;

    let command = f.editor.update(Message::Outcome(failed));
    assert_eq!(
        f.editor.view(),
        editing("Pasta", Some(RecipeEditorProblem::SaveFailed(error)))
    );
    f.render(command).await
}

// Test support

fn shakshuka() -> Recipe {
    Recipe {
        id: RecipeId::from(Uuid::from_u128(1)),
        title: "Shakshuka".into(),
    }
}

fn empty_title() -> RecipeEditorProblem {
    RecipeEditorProblem::Invalid(RecipeTitleError::NotEmptyViolated)
}

fn editing(title: &str, problem: Option<RecipeEditorProblem>) -> RecipeEditorView {
    RecipeEditorView {
        title: title.into(),
        status: RecipeEditorStatus::Editing { problem },
    }
}

/// A store call that Save made, with the title it wrote.
#[derive(Debug, PartialEq, Eq)]
enum Write {
    Created(String),
    Updated(RecipeId, String),
}

struct Fixture {
    editor: RecipeEditor,
    effects: RecipeEffectStream,
    /// The store calls of the saves, in order.
    writes: Vec<Write>,
}

impl Fixture {
    /// An editor for a new recipe.
    fn new() -> Self {
        let (editor, effects) = RecipeEditor::test_fixture();
        Self {
            editor,
            effects,
            writes: vec![],
        }
    }

    /// An editor for the saved `shakshuka()`.
    fn edit() -> Self {
        let (editor, effects) = RecipeEditor::edit_test_fixture(&shakshuka());
        Self {
            editor,
            effects,
            writes: vec![],
        }
    }

    fn send(&mut self, event: RecipeEditorEvent) -> Command<RecipeEditorOutcome> {
        self.editor.update(Message::Event(event))
    }

    /// Runs a command that only renders.
    async fn render(&mut self, command: Command<RecipeEditorOutcome>) -> anyhow::Result<()> {
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
        command: Command<RecipeEditorOutcome>,
    ) -> anyhow::Result<RecipeEditorOutcome> {
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

    async fn type_title(&mut self, title: &str) -> anyhow::Result<()> {
        let command = self.send(RecipeEditorEvent::SetTitle(title.into()));
        self.render(command).await
    }

    /// Runs a save command: its render, then the store call (create or
    /// update) answered with `result`. Returns `SaveFinished`, not yet
    /// dispatched.
    async fn save(
        &mut self,
        save: Command<RecipeEditorOutcome>,
        result: Result<Recipe, RecipeSaveError>,
    ) -> anyhow::Result<RecipeEditorOutcome> {
        let mut command = save.driver(&mut self.effects);
        command
            .step()
            .await?
            .expect_effect()
            .expect_render()
            .resolve_render(|| async {})
            .await?;
        let mut write = None;
        let store = command.step().await?.expect_effect().expect_store();
        match store.request {
            RecipeStoreRequest::Create { .. } => {
                store
                    .resolve_create(|recipe| {
                        write = Some(Write::Created(recipe.title.as_ref().to_owned()));
                        async { result }
                    })
                    .await?
            }
            RecipeStoreRequest::Update { .. } => {
                store
                    .resolve_update(|id, recipe| {
                        write = Some(Write::Updated(id, recipe.title.as_ref().to_owned()));
                        async { result }
                    })
                    .await?
            }
            _ => anyhow::bail!("expected a create or an update"),
        }
        let finished = command.step().await?.expect_outcome();
        command.step().await?.expect_done();
        drop(command);
        self.writes.extend(write);
        Ok(finished)
    }

    /// Cancel on a changed title: the editor asks "Discard changes?".
    async fn request_close(&mut self) -> anyhow::Result<()> {
        let command = self.send(RecipeEditorEvent::RequestClose);
        assert_eq!(
            self.editor.view().status,
            RecipeEditorStatus::ConfirmingDiscard(ConfirmationView::AwaitingDecision)
        );
        self.render(command).await
    }

    /// Answers "Discard changes?". Returns `ConfirmationDecided`, not yet
    /// dispatched.
    async fn decide(&mut self, event: ConfirmationEvent) -> anyhow::Result<RecipeEditorOutcome> {
        let command = self.send(RecipeEditorEvent::Confirmation(event));
        let mut command = command.driver(&mut self.effects);
        let decided = command.step().await?.expect_outcome();
        command.step().await?.expect_done();
        Ok(decided)
    }
}
