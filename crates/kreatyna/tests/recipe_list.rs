use kreatyna::capabilities::ResolveRender;
use kreatyna::recipes::{
    Recipe, RecipeId, RecipeList, RecipeListEvent, RecipeListOutcome, RecipeListRow,
    RecipeListStatus, RecipeListView,
    capabilities::{RecipeEffectStream, RecipeLoadError, ResolveRecipeStore},
};
use krucyfiks::{Command, Message, Widget, effects::CommandTestExt};
use test_case::test_case;
use uuid::Uuid;

// Green path

#[test_case(vec![]; "no recipes")]
#[test_case(vec![recipe(2, "Roasted tomato pasta"), recipe(1, "Shakshuka")]; "two recipes")]
#[tokio::test]
async fn initialization_renders_then_reads_and_dispatching_its_result_shows_the_rows_in_store_order(
    stored: Vec<Recipe>,
) -> anyhow::Result<()> {
    let mut fixture = ListFixture::new();
    let mut command = fixture.list.init().driver(&mut fixture.effects);
    assert_eq!(fixture.list.view(), loading());
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
        .resolve_list(|| async { Ok(stored.clone()) })
        .await?;
    let outcome = command.step().await?.expect_outcome();
    command.step().await?.expect_done();
    drop(command);
    assert_eq!(
        outcome,
        RecipeListOutcome::LoadFinished {
            result: Ok(stored.clone())
        }
    );

    // Reading the store does not apply the result. Dispatch it to leave loading.
    let mut render = fixture
        .list
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.list.view(), ready(&stored));
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

// Validation

#[tokio::test]
async fn refresh_is_ignored_while_the_first_load_runs() -> anyhow::Result<()> {
    let mut fixture = ListFixture::new();
    let _load = fixture.list.init();
    let mut refresh = fixture
        .list
        .update(Message::Event(RecipeListEvent::Refresh))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.list.view(), loading());
    // No second render and no second read.
    refresh.step().await?.expect_done();
    Ok(())
}

#[tokio::test]
async fn refresh_is_ignored_when_the_list_is_ready() -> anyhow::Result<()> {
    let shakshuka = recipe(1, "Shakshuka");
    let mut fixture = ListFixture::new();
    let load = fixture.list.init();
    fixture
        .finish_load(load, Ok(vec![shakshuka.clone()]))
        .await?;
    let mut refresh = fixture
        .list
        .update(Message::Event(RecipeListEvent::Refresh))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.list.view(), ready(&[shakshuka]));
    refresh.step().await?.expect_done();
    Ok(())
}

// Red cases

#[test_case(RecipeLoadError::Storage; "storage failure")]
#[test_case(RecipeLoadError::Transport; "transport failure")]
#[tokio::test]
async fn a_failed_load_shows_its_error(error: RecipeLoadError) -> anyhow::Result<()> {
    let mut fixture = ListFixture::new();
    let load = fixture.list.init();
    fixture.finish_load(load, Err(error.clone())).await?;
    assert_eq!(
        fixture.list.view(),
        RecipeListView {
            status: RecipeListStatus::Failed(error)
        }
    );
    Ok(())
}

#[tokio::test]
async fn refresh_after_a_failure_shows_loading_then_the_rows() -> anyhow::Result<()> {
    let shakshuka = recipe(1, "Shakshuka");
    let mut fixture = ListFixture::new();
    let load = fixture.list.init();
    fixture
        .finish_load(load, Err(RecipeLoadError::Storage))
        .await?;

    let mut retry = fixture
        .list
        .update(Message::Event(RecipeListEvent::Refresh))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.list.view(), loading());
    retry
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    retry
        .step()
        .await?
        .expect_effect()
        .expect_store()
        .resolve_list(|| async { Ok(vec![recipe(1, "Shakshuka")]) })
        .await?;
    let outcome = retry.step().await?.expect_outcome();
    retry.step().await?.expect_done();
    drop(retry);

    let mut render = fixture
        .list
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.list.view(), ready(&[shakshuka]));
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

fn recipe(id: u128, title: &str) -> Recipe {
    Recipe {
        id: RecipeId::from(Uuid::from_u128(id)),
        title: title.into(),
    }
}

fn loading() -> RecipeListView {
    RecipeListView {
        status: RecipeListStatus::Loading,
    }
}

fn ready(recipes: &[Recipe]) -> RecipeListView {
    RecipeListView {
        status: RecipeListStatus::Ready {
            rows: recipes
                .iter()
                .map(|recipe| RecipeListRow {
                    id: recipe.id,
                    title: recipe.title.clone(),
                })
                .collect(),
        },
    }
}

struct ListFixture {
    list: RecipeList,
    effects: RecipeEffectStream,
}

impl ListFixture {
    fn new() -> Self {
        let (list, effects) = RecipeList::test_fixture();
        Self { list, effects }
    }

    /// Runs a load command to its end with `result`, then dispatches the
    /// outcome and runs the render that follows.
    async fn finish_load(
        &mut self,
        load: Command<RecipeListOutcome>,
        result: Result<Vec<Recipe>, RecipeLoadError>,
    ) -> anyhow::Result<()> {
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
        drop(command);

        let mut render = self
            .list
            .update(Message::Outcome(outcome))
            .driver(&mut self.effects);
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
}
