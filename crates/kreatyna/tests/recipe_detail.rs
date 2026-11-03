use kreatyna::capabilities::ResolveRender;
use kreatyna::confirmation::{ConfirmationEvent, ConfirmationOutcome, ConfirmationView};
use kreatyna::recipes::{
    CanvasPoint, Recipe, RecipeDeletion, RecipeDetail, RecipeDetailEvent, RecipeDetailOutcome,
    RecipeDetailStatus, RecipeDetailStep, RecipeDetailView, RecipeId, Step, StepId,
    capabilities::{RecipeDeleteError, RecipeEffectStream, RecipeLoadError, ResolveRecipeStore},
};
use krucyfiks::{Command, Latest, Message, Widget, effects::CommandTestExt};
use test_case::test_case;
use uuid::Uuid;

// Green path

#[tokio::test]
async fn opening_renders_loading_then_reads_the_recipe_by_id_and_shows_its_title_and_steps()
-> anyhow::Result<()> {
    let mut f = Fixture::new();
    let mut command = f.detail.init().driver(&mut f.effects);
    assert_eq!(f.detail.view(), view(RecipeDetailStatus::Loading));
    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    let mut asked = None;
    command
        .step()
        .await?
        .expect_effect()
        .expect_store()
        .resolve_get(|id| {
            asked = Some(id);
            async { Ok(Some(shakshuka())) }
        })
        .await?;
    let outcome = command.step().await?.expect_outcome();
    command.step().await?.expect_done();
    drop(command);
    assert_eq!(asked, Some(id()));

    // Reading the store does not apply the result. Dispatch it to leave loading.
    let command = f.detail.update(Message::Outcome(outcome));
    assert_eq!(f.detail.view(), view(ready("Shakshuka")));
    f.render(command).await
}

#[tokio::test]
async fn edit_on_a_shown_recipe_opens_the_editor_with_it() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    let load = f.detail.init();
    f.finish_load(load, Ok(Some(shakshuka()))).await?;

    let mut edit = f
        .detail
        .update(Message::Event(RecipeDetailEvent::Edit))
        .driver(&mut f.effects);
    assert_eq!(
        edit.step().await?.expect_outcome(),
        RecipeDetailOutcome::OpenEdit {
            recipe: shakshuka()
        }
    );
    edit.step().await?.expect_done();
    drop(edit);
    assert_eq!(f.detail.view(), view(ready("Shakshuka")));
    Ok(())
}

#[tokio::test]
async fn a_recipe_without_steps_shows_an_empty_step_list() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    let load = f.detail.init();
    let toast = Recipe {
        id: id(),
        title: "Toast".into(),
        steps: vec![],
    };
    f.finish_load(load, Ok(Some(toast))).await?;
    assert_eq!(
        f.detail.view(),
        view(RecipeDetailStatus::Ready {
            title: "Toast".into(),
            steps: vec![],
            deletion: RecipeDeletion::Idle,
        })
    );
    Ok(())
}

#[tokio::test]
async fn delete_asks_before_it_deletes() -> anyhow::Result<()> {
    let mut f = Fixture::shown().await?;
    let command = f
        .detail
        .update(Message::Event(RecipeDetailEvent::RequestDelete));
    assert_eq!(f.detail.view(), view(asking(None)));
    f.render(command).await
}

#[tokio::test]
async fn cancel_closes_the_question_without_a_delete() -> anyhow::Result<()> {
    let mut f = Fixture::shown().await?;
    f.request_delete().await?;
    let cancelled = f.decide(ConfirmationEvent::Cancel).await?;
    assert!(matches!(
        cancelled,
        RecipeDetailOutcome::ConfirmationDecided(Latest {
            value: ConfirmationOutcome::Cancelled,
            ..
        })
    ));
    // Only a render follows: the store is not asked.
    let command = f.detail.update(Message::Outcome(cancelled));
    assert_eq!(f.detail.view(), view(ready("Shakshuka")));
    f.render(command).await
}

#[tokio::test]
async fn confirm_deletes_the_shown_recipe_then_reports_it() -> anyhow::Result<()> {
    let mut f = Fixture::shown().await?;
    f.request_delete().await?;
    let confirmed = f.decide(ConfirmationEvent::Confirm).await?;

    let mut command = f
        .detail
        .update(Message::Outcome(confirmed))
        .driver(&mut f.effects);
    assert_eq!(f.detail.view(), view(deletion(RecipeDeletion::Deleting)));
    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    let mut asked = None;
    command
        .step()
        .await?
        .expect_effect()
        .expect_store()
        .resolve_delete(|id| {
            asked = Some(id);
            async { Ok(()) }
        })
        .await?;
    let finished = command.step().await?.expect_outcome();
    command.step().await?.expect_done();
    drop(command);
    assert_eq!(asked, Some(id()));

    let mut command = f
        .detail
        .update(Message::Outcome(finished))
        .driver(&mut f.effects);
    // The title stays until the section replaces the pane.
    assert_eq!(f.detail.view(), view(deletion(RecipeDeletion::Deleted)));
    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    assert_eq!(
        command.step().await?.expect_outcome(),
        RecipeDetailOutcome::Deleted { id: id() }
    );
    command.step().await?.expect_done();
    Ok(())
}

// Validation

/// `finished` is the result of the first load; `None` leaves it running.
#[test_case(None, RecipeDetailStatus::Loading; "while loading")]
#[test_case(Some(Ok(None)), RecipeDetailStatus::Missing; "when missing")]
#[test_case(Some(Err(RecipeLoadError::Storage)), RecipeDetailStatus::Failed(RecipeLoadError::Storage); "after a failure")]
#[tokio::test]
async fn edit_is_ignored_unless_the_recipe_is_shown(
    finished: Option<Result<Option<Recipe>, RecipeLoadError>>,
    status: RecipeDetailStatus,
) -> anyhow::Result<()> {
    let mut f = Fixture::new();
    let load = f.detail.init();
    if let Some(result) = finished {
        f.finish_load(load, result).await?;
    }
    let mut edit = f
        .detail
        .update(Message::Event(RecipeDetailEvent::Edit))
        .driver(&mut f.effects);
    edit.step().await?.expect_done();
    drop(edit);
    assert_eq!(f.detail.view(), view(status));
    Ok(())
}

/// `finished` is the result of the first load; `None` leaves it running.
#[test_case(None, RecipeDetailStatus::Loading; "while loading")]
#[test_case(Some(Ok(Some(shakshuka()))), ready("Shakshuka"); "when ready")]
#[test_case(Some(Ok(None)), RecipeDetailStatus::Missing; "when missing")]
#[tokio::test]
async fn retry_is_ignored_unless_the_load_failed(
    finished: Option<Result<Option<Recipe>, RecipeLoadError>>,
    status: RecipeDetailStatus,
) -> anyhow::Result<()> {
    let mut f = Fixture::new();
    let load = f.detail.init();
    if let Some(result) = finished {
        f.finish_load(load, result).await?;
    }
    let mut retry = f
        .detail
        .update(Message::Event(RecipeDetailEvent::Retry))
        .driver(&mut f.effects);
    // No render and no read.
    retry.step().await?.expect_done();
    assert_eq!(f.detail.view(), view(status));
    Ok(())
}

#[tokio::test]
async fn a_result_of_a_replaced_load_is_ignored() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    let load = f.detail.init();
    let failed = f.run_load(load, Err(RecipeLoadError::Storage)).await?;
    let command = f.detail.update(Message::Outcome(failed.clone()));
    f.render(command).await?;

    let _retry = f.detail.update(Message::Event(RecipeDetailEvent::Retry));
    let mut stale = f
        .detail
        .update(Message::Outcome(failed))
        .driver(&mut f.effects);
    stale.step().await?.expect_done();
    assert_eq!(f.detail.view(), view(RecipeDetailStatus::Loading));
    Ok(())
}

/// `finished` is the result of the first load; `None` leaves it running.
#[test_case(None, RecipeDetailStatus::Loading; "while loading")]
#[test_case(Some(Ok(None)), RecipeDetailStatus::Missing; "when missing")]
#[test_case(Some(Err(RecipeLoadError::Storage)), RecipeDetailStatus::Failed(RecipeLoadError::Storage); "after a failure")]
#[tokio::test]
async fn delete_is_ignored_unless_the_recipe_is_shown(
    finished: Option<Result<Option<Recipe>, RecipeLoadError>>,
    status: RecipeDetailStatus,
) -> anyhow::Result<()> {
    let mut f = Fixture::new();
    let load = f.detail.init();
    if let Some(result) = finished {
        f.finish_load(load, result).await?;
    }
    let mut delete = f
        .detail
        .update(Message::Event(RecipeDetailEvent::RequestDelete))
        .driver(&mut f.effects);
    delete.step().await?.expect_done();
    drop(delete);
    assert_eq!(f.detail.view(), view(status));
    Ok(())
}

#[test_case(RecipeDetailEvent::Edit; "edit")]
#[test_case(RecipeDetailEvent::RequestDelete; "delete again")]
#[test_case(RecipeDetailEvent::Retry; "retry")]
#[tokio::test]
async fn buttons_are_ignored_while_the_question_waits(
    event: RecipeDetailEvent,
) -> anyhow::Result<()> {
    let mut f = Fixture::shown().await?;
    f.request_delete().await?;
    let mut ignored = f
        .detail
        .update(Message::Event(event))
        .driver(&mut f.effects);
    ignored.step().await?.expect_done();
    drop(ignored);
    assert_eq!(f.detail.view(), view(asking(None)));
    Ok(())
}

#[test_case(RecipeDetailEvent::Edit; "edit")]
#[test_case(RecipeDetailEvent::RequestDelete; "delete again")]
#[test_case(RecipeDetailEvent::Confirmation(ConfirmationEvent::Confirm); "confirm")]
#[test_case(RecipeDetailEvent::Confirmation(ConfirmationEvent::Cancel); "cancel")]
#[tokio::test]
async fn events_while_deleting_are_ignored(event: RecipeDetailEvent) -> anyhow::Result<()> {
    let mut f = Fixture::shown().await?;
    f.request_delete().await?;
    let confirmed = f.decide(ConfirmationEvent::Confirm).await?;
    let _deleting = f.detail.update(Message::Outcome(confirmed));

    let mut ignored = f
        .detail
        .update(Message::Event(event))
        .driver(&mut f.effects);
    ignored.step().await?.expect_done();
    drop(ignored);
    assert_eq!(f.detail.view(), view(deletion(RecipeDeletion::Deleting)));
    Ok(())
}

#[test_case(ConfirmationEvent::Confirm; "confirm")]
#[test_case(ConfirmationEvent::Cancel; "cancel")]
#[tokio::test]
async fn an_answer_without_a_question_is_ignored(event: ConfirmationEvent) -> anyhow::Result<()> {
    let mut f = Fixture::shown().await?;
    let mut ignored = f
        .detail
        .update(Message::Event(RecipeDetailEvent::Confirmation(event)))
        .driver(&mut f.effects);
    ignored.step().await?.expect_done();
    drop(ignored);
    assert_eq!(f.detail.view(), view(ready("Shakshuka")));
    Ok(())
}

#[tokio::test]
async fn a_result_of_a_replaced_delete_is_ignored() -> anyhow::Result<()> {
    let mut f = Fixture::shown().await?;
    f.request_delete().await?;
    let confirmed = f.decide(ConfirmationEvent::Confirm).await?;
    let failed = f.delete(confirmed, Err(RecipeDeleteError::Storage)).await?;
    let command = f.detail.update(Message::Outcome(failed.clone()));
    f.render(command).await?;

    // The second delete runs; the first result arrives again.
    let retry = f.decide(ConfirmationEvent::Confirm).await?;
    let _deleting = f.detail.update(Message::Outcome(retry));
    let mut stale = f
        .detail
        .update(Message::Outcome(failed))
        .driver(&mut f.effects);
    stale.step().await?.expect_done();
    drop(stale);
    assert_eq!(f.detail.view(), view(deletion(RecipeDeletion::Deleting)));
    Ok(())
}

// Red cases

#[tokio::test]
async fn a_recipe_that_no_longer_exists_shows_missing() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    let load = f.detail.init();
    f.finish_load(load, Ok(None)).await?;
    assert_eq!(f.detail.view(), view(RecipeDetailStatus::Missing));
    Ok(())
}

#[test_case(RecipeLoadError::Storage; "storage failure")]
#[test_case(RecipeLoadError::Transport; "transport failure")]
#[tokio::test]
async fn a_failed_load_shows_its_error(error: RecipeLoadError) -> anyhow::Result<()> {
    let mut f = Fixture::new();
    let load = f.detail.init();
    f.finish_load(load, Err(error.clone())).await?;
    assert_eq!(f.detail.view(), view(RecipeDetailStatus::Failed(error)));
    Ok(())
}

#[tokio::test]
async fn retry_after_a_failure_shows_loading_then_the_recipe() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    let load = f.detail.init();
    f.finish_load(load, Err(RecipeLoadError::Storage)).await?;

    let retry = f.detail.update(Message::Event(RecipeDetailEvent::Retry));
    assert_eq!(f.detail.view(), view(RecipeDetailStatus::Loading));
    f.finish_load(retry, Ok(Some(shakshuka()))).await?;
    assert_eq!(f.detail.view(), view(ready("Shakshuka")));
    Ok(())
}

#[test_case(RecipeDeleteError::Storage; "storage failure")]
#[test_case(RecipeDeleteError::Transport; "transport failure")]
#[tokio::test]
async fn a_failed_delete_asks_again_with_the_error(error: RecipeDeleteError) -> anyhow::Result<()> {
    let mut f = Fixture::shown().await?;
    f.request_delete().await?;
    let confirmed = f.decide(ConfirmationEvent::Confirm).await?;
    let failed = f.delete(confirmed, Err(error.clone())).await?;
    let command = f.detail.update(Message::Outcome(failed));
    assert_eq!(f.detail.view(), view(asking(Some(error))));
    f.render(command).await
}

// Test support

fn id() -> RecipeId {
    RecipeId::from(Uuid::from_u128(1))
}

/// Shakshuka with two steps. Fry onion was added first, so the read view
/// lists it first, although it is right of Dice onion on the canvas.
fn shakshuka() -> Recipe {
    Recipe {
        id: id(),
        title: "Shakshuka".into(),
        steps: vec![
            step(11, "Fry onion", 200, 40),
            step(12, "Dice onion", 10, 20),
        ],
    }
}

fn step(id: u128, title: &str, x: u32, y: u32) -> Step {
    Step {
        id: StepId::from(Uuid::from_u128(id)),
        title: title.into(),
        position: CanvasPoint { x, y },
    }
}

/// Shakshuka's steps as the read view lists them.
fn shakshuka_steps() -> Vec<RecipeDetailStep> {
    shakshuka()
        .steps
        .into_iter()
        .map(|step| RecipeDetailStep {
            id: step.id,
            title: step.title,
        })
        .collect()
}

fn ready(title: &str) -> RecipeDetailStatus {
    RecipeDetailStatus::Ready {
        title: title.into(),
        steps: shakshuka_steps(),
        deletion: RecipeDeletion::Idle,
    }
}

fn deletion(deletion: RecipeDeletion) -> RecipeDetailStatus {
    RecipeDetailStatus::Ready {
        title: "Shakshuka".into(),
        steps: shakshuka_steps(),
        deletion,
    }
}

/// Shakshuka with the question open.
fn asking(error: Option<RecipeDeleteError>) -> RecipeDetailStatus {
    deletion(RecipeDeletion::Confirming {
        confirmation: ConfirmationView::AwaitingDecision,
        error,
    })
}

fn view(status: RecipeDetailStatus) -> RecipeDetailView {
    RecipeDetailView { id: id(), status }
}

struct Fixture {
    detail: RecipeDetail,
    effects: RecipeEffectStream,
}

impl Fixture {
    fn new() -> Self {
        let (detail, effects) = RecipeDetail::test_fixture(id());
        Self { detail, effects }
    }

    /// Setup only: a read view that shows Shakshuka.
    async fn shown() -> anyhow::Result<Self> {
        let mut f = Self::new();
        let load = f.detail.init();
        f.finish_load(load, Ok(Some(shakshuka()))).await?;
        Ok(f)
    }

    /// Setup only: opens the question.
    async fn request_delete(&mut self) -> anyhow::Result<()> {
        let command = self
            .detail
            .update(Message::Event(RecipeDetailEvent::RequestDelete));
        self.render(command).await
    }

    /// Setup only: answers the question. Returns the decision, not yet
    /// dispatched.
    async fn decide(&mut self, event: ConfirmationEvent) -> anyhow::Result<RecipeDetailOutcome> {
        let mut command = self
            .detail
            .update(Message::Event(RecipeDetailEvent::Confirmation(event)))
            .driver(&mut self.effects);
        let outcome = command.step().await?.expect_outcome();
        command.step().await?.expect_done();
        Ok(outcome)
    }

    /// Setup only: applies a confirmed decision and answers the delete with
    /// `result`. Returns `DeleteFinished`, not yet dispatched.
    async fn delete(
        &mut self,
        confirmed: RecipeDetailOutcome,
        result: Result<(), RecipeDeleteError>,
    ) -> anyhow::Result<RecipeDetailOutcome> {
        let mut command = self
            .detail
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

    /// Runs a load command to its end with `result`, then dispatches the
    /// outcome and runs the render that follows.
    async fn finish_load(
        &mut self,
        load: Command<RecipeDetailOutcome>,
        result: Result<Option<Recipe>, RecipeLoadError>,
    ) -> anyhow::Result<()> {
        let outcome = self.run_load(load, result).await?;
        let command = self.detail.update(Message::Outcome(outcome));
        self.render(command).await
    }

    /// Runs a command that only renders.
    async fn render(&mut self, command: Command<RecipeDetailOutcome>) -> anyhow::Result<()> {
        let mut render = command.driver(&mut self.effects);
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

    /// Runs a load command to its end with `result`. Returns its outcome,
    /// not yet dispatched.
    async fn run_load(
        &mut self,
        load: Command<RecipeDetailOutcome>,
        result: Result<Option<Recipe>, RecipeLoadError>,
    ) -> anyhow::Result<RecipeDetailOutcome> {
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
            .resolve_get(|_| async { result })
            .await?;
        let outcome = command.step().await?.expect_outcome();
        command.step().await?.expect_done();
        Ok(outcome)
    }
}
