use kreatyna::capabilities::ResolveRender;
use kreatyna::recipes::{
    Recipe, RecipeDetail, RecipeDetailEvent, RecipeDetailOutcome, RecipeDetailStatus,
    RecipeDetailView, RecipeId,
    capabilities::{RecipeEffectStream, RecipeLoadError, ResolveRecipeStore},
};
use krucyfiks::{Command, Message, Widget, effects::CommandTestExt};
use test_case::test_case;
use uuid::Uuid;

// Green path

#[tokio::test]
async fn opening_renders_loading_then_reads_the_recipe_by_id_and_shows_its_title()
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

// Validation

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

// Test support

fn id() -> RecipeId {
    RecipeId::from(Uuid::from_u128(1))
}

fn shakshuka() -> Recipe {
    Recipe {
        id: id(),
        title: "Shakshuka".into(),
    }
}

fn ready(title: &str) -> RecipeDetailStatus {
    RecipeDetailStatus::Ready {
        title: title.into(),
    }
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
