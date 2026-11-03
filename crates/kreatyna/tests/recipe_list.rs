use kreatyna::capabilities::ResolveRender;
use kreatyna::recipes::{
    RecipeId, RecipeList, RecipeListEvent, RecipeListOutcome, RecipeListRow, RecipeListStatus,
    RecipeListView, RecipeSummary,
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
    stored: Vec<RecipeSummary>,
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
    let RecipeListOutcome::LoadFinished(finished) = &outcome;
    assert_eq!(finished.value, Ok(stored.clone()));

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

#[test_case("sha", &[shakshuka()]; "a lower-case part of a title")]
#[test_case("SHA", &[shakshuka()]; "the same part in upper case")]
#[test_case("  sha ", &[shakshuka()]; "the part with spaces around it")]
#[test_case("pasta", &[pasta()]; "the end of a title")]
#[test_case("a", &[pasta(), shakshuka()]; "a part of both titles")]
#[test_case("", &[pasta(), shakshuka()]; "an empty search")]
#[tokio::test]
async fn a_search_shows_the_recipes_whose_title_key_contains_its_key(
    search: &str,
    shown: &[RecipeSummary],
) -> anyhow::Result<()> {
    let mut fixture = ListFixture::new();
    let load = fixture.list.init();
    fixture
        .finish_load(load, Ok(vec![pasta(), shakshuka()]))
        .await?;

    // A search renders and does not read the store.
    let command = fixture
        .list
        .update(Message::Event(RecipeListEvent::Search(search.into())));
    fixture.render(command).await?;
    assert_eq!(fixture.list.view(), searched(search, ready_status(shown)));
    Ok(())
}

#[tokio::test]
async fn clearing_the_search_shows_every_recipe_again() -> anyhow::Result<()> {
    let mut fixture = ListFixture::new();
    let load = fixture.list.init();
    fixture
        .finish_load(load, Ok(vec![pasta(), shakshuka()]))
        .await?;
    fixture.search("sha").await?;
    fixture.search("").await?;
    assert_eq!(fixture.list.view(), ready(&[pasta(), shakshuka()]));
    Ok(())
}

#[tokio::test]
async fn a_search_during_the_first_load_applies_to_its_result() -> anyhow::Result<()> {
    let mut fixture = ListFixture::new();
    let load = fixture.list.init();
    fixture.search("sha").await?;
    assert_eq!(
        fixture.list.view(),
        searched("sha", RecipeListStatus::Loading)
    );

    fixture
        .finish_load(load, Ok(vec![pasta(), shakshuka()]))
        .await?;
    assert_eq!(
        fixture.list.view(),
        searched("sha", ready_status(&[shakshuka()]))
    );
    Ok(())
}

#[tokio::test]
async fn a_search_after_a_failure_applies_to_the_result_of_refresh() -> anyhow::Result<()> {
    let mut fixture = ListFixture::new();
    let load = fixture.list.init();
    fixture
        .finish_load(load, Err(RecipeLoadError::Storage))
        .await?;
    fixture.search("sha").await?;
    assert_eq!(
        fixture.list.view(),
        searched("sha", RecipeListStatus::Failed(RecipeLoadError::Storage))
    );

    let retry = fixture
        .list
        .update(Message::Event(RecipeListEvent::Refresh));
    fixture
        .finish_load(retry, Ok(vec![pasta(), shakshuka()]))
        .await?;
    assert_eq!(
        fixture.list.view(),
        searched("sha", ready_status(&[shakshuka()]))
    );
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

#[tokio::test]
async fn a_result_of_an_earlier_load_is_ignored() -> anyhow::Result<()> {
    let mut fixture = ListFixture::new();
    let load = fixture.list.init();
    let failed = fixture
        .run_load(load, Err(RecipeLoadError::Storage))
        .await?;
    let command = fixture.list.update(Message::Outcome(failed.clone()));
    fixture.render(command).await?;

    let _retry = fixture
        .list
        .update(Message::Event(RecipeListEvent::Refresh));
    let mut stale = fixture
        .list
        .update(Message::Outcome(failed))
        .driver(&mut fixture.effects);
    stale.step().await?.expect_done();
    assert_eq!(fixture.list.view(), loading());
    Ok(())
}

#[tokio::test]
async fn a_search_without_recipes_shows_the_empty_list() -> anyhow::Result<()> {
    let mut fixture = ListFixture::new();
    let load = fixture.list.init();
    fixture.finish_load(load, Ok(vec![])).await?;
    fixture.search("sha").await?;
    // "No recipes yet", not "No matches": there is nothing to search.
    assert_eq!(fixture.list.view(), searched("sha", ready_status(&[])));
    Ok(())
}

// Red cases

#[test_case("shax"; "a longer text than a title part")]
#[test_case("omelette"; "a word in no title")]
#[test_case("sh a"; "a space inside the part")]
#[tokio::test]
async fn a_search_that_matches_no_recipe_shows_no_matches(search: &str) -> anyhow::Result<()> {
    let mut fixture = ListFixture::new();
    let load = fixture.list.init();
    fixture
        .finish_load(load, Ok(vec![pasta(), shakshuka()]))
        .await?;
    fixture.search(search).await?;
    assert_eq!(
        fixture.list.view(),
        searched(search, RecipeListStatus::NoMatches)
    );
    Ok(())
}

#[test_case(RecipeLoadError::Storage; "storage failure")]
#[test_case(RecipeLoadError::Transport; "transport failure")]
#[tokio::test]
async fn a_failed_load_shows_its_error(error: RecipeLoadError) -> anyhow::Result<()> {
    let mut fixture = ListFixture::new();
    let load = fixture.list.init();
    fixture.finish_load(load, Err(error.clone())).await?;
    assert_eq!(
        fixture.list.view(),
        searched("", RecipeListStatus::Failed(error))
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

fn recipe(id: u128, title: &str) -> RecipeSummary {
    RecipeSummary {
        id: RecipeId::from(Uuid::from_u128(id)),
        title: title.into(),
        step_count: 0,
    }
}

fn shakshuka() -> RecipeSummary {
    recipe(1, "Shakshuka")
}

fn pasta() -> RecipeSummary {
    recipe(2, "Roasted tomato pasta")
}

/// The view with this search text and status.
fn searched(search: &str, status: RecipeListStatus) -> RecipeListView {
    RecipeListView {
        search: search.into(),
        status,
    }
}

fn loading() -> RecipeListView {
    searched("", RecipeListStatus::Loading)
}

/// The view without a search, with these rows.
fn ready(recipes: &[RecipeSummary]) -> RecipeListView {
    searched("", ready_status(recipes))
}

fn ready_status(recipes: &[RecipeSummary]) -> RecipeListStatus {
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
        result: Result<Vec<RecipeSummary>, RecipeLoadError>,
    ) -> anyhow::Result<()> {
        let outcome = self.run_load(load, result).await?;
        let command = self.list.update(Message::Outcome(outcome));
        self.render(command).await
    }

    /// Types `search` and runs the render that follows.
    async fn search(&mut self, search: &str) -> anyhow::Result<()> {
        let command = self
            .list
            .update(Message::Event(RecipeListEvent::Search(search.into())));
        self.render(command).await
    }

    /// Runs a command that only renders.
    async fn render(&mut self, command: Command<RecipeListOutcome>) -> anyhow::Result<()> {
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
        load: Command<RecipeListOutcome>,
        result: Result<Vec<RecipeSummary>, RecipeLoadError>,
    ) -> anyhow::Result<RecipeListOutcome> {
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
}
