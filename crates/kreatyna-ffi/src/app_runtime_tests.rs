use std::{
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};

use anyhow::Context;
use diesel::{Connection, RunQueryDsl, connection::SimpleConnection};

use super::*;

// Green path

#[test]
fn app_ffi_initializes_before_exposing_the_running_app() -> anyhow::Result<()> {
    let database = TemporaryDatabase::new()?;
    let (render, rendered) = TestRender::new();
    let app = app(
        AppCapabilities {
            render: Arc::new(render),
        },
        database.path()?.into(),
        AppLayout::Stack,
    );

    let initial = app.view()?;
    assert_eq!(initial, AppView::Initializing);
    assert!(!std::path::Path::new(database.path()?).exists());

    app.start()?;
    assert!(matches!(app.start(), Err(AppFfiError::AlreadyStarted)));
    wait_until_ready(&app, &rendered)?;

    app.shutdown();
    assert!(matches!(app.start(), Err(AppFfiError::Terminated)));
    assert!(matches!(
        app.dispatch(AppEvent::Stack {
            event: IngredientCatalogEvent::List {
                scope: stack_view(&app)?.scope,
                event: IngredientListEvent::Search {
                    query: "gar".into(),
                },
            },
        }),
        Err(AppFfiError::Terminated)
    ));
    Ok(())
}

#[test]
fn the_split_layout_starts_on_recipes_with_both_sections_ready() -> anyhow::Result<()> {
    let database = TemporaryDatabase::new()?;
    let (render, rendered) = TestRender::new();
    let app = app(
        AppCapabilities {
            render: Arc::new(render),
        },
        database.path()?.into(),
        AppLayout::Split,
    );

    app.start()?;
    let running = wait_until_ready(&app, &rendered)?;

    let RunningAppView::Split { app: split } = running else {
        anyhow::bail!("expected the split layout, got {running:?}");
    };
    // REC-01/Q1: the Mac app opens on Recipes. A new database has none.
    assert_eq!(split.section, AppSection::Recipes);
    assert_eq!(
        split.recipes.list.status,
        RecipeListStatus::Ready { rows: vec![] }
    );
    assert_eq!(split.ingredients.selected, None);
    assert_eq!(split.ingredients.pane.value, IngredientSplitPaneView::Empty);
    app.shutdown();
    Ok(())
}

#[test]
fn a_created_recipe_is_listed_opens_its_read_view_and_its_title_cannot_be_taken_again()
-> anyhow::Result<()> {
    let database = TemporaryDatabase::new()?;
    let (render, rendered) = TestRender::new();
    let app = app(
        AppCapabilities {
            render: Arc::new(render),
        },
        database.path()?.into(),
        AppLayout::Split,
    );
    app.start()?;
    wait_until_ready(&app, &rendered)?;

    create_recipe(&app, &rendered, "Shakshuka")?;
    wait_until(&app, &rendered, |view| {
        split_of(view).is_some_and(|split| {
            // The saved recipe is listed, highlighted, and read from the store.
            matches!(
                (&split.recipes.list.status, &split.recipes.pane.value),
                (
                    RecipeListStatus::Ready { rows },
                    RecipePaneView::Detail {
                        detail: RecipeDetailView {
                            status: RecipeDetailStatus::Ready { title, .. },
                            id,
                        }
                    },
                ) if rows.iter().map(|row| row.title.as_str()).eq(["Shakshuka"])
                    && title == "Shakshuka"
                    && split.recipes.selected.as_ref() == Some(id)
                    && rows[0].id == *id
            )
        })
    })?;

    create_recipe(&app, &rendered, "shakshuka ")?;
    wait_until(&app, &rendered, |view| {
        split_of(view).is_some_and(|split| {
            matches!(
                &split.recipes.pane.value,
                RecipePaneView::Create {
                    editor: RecipeEditorView {
                        status: RecipeEditorStatus::Editing {
                            problem: Some(RecipeEditorProblem::SaveFailed {
                                error: RecipeSaveError::DuplicateTitle
                            })
                        },
                        ..
                    },
                    ..
                }
            )
        })
    })?;
    app.shutdown();
    Ok(())
}

#[test]
fn an_edited_recipe_title_is_saved_and_shown_in_the_read_view_and_the_list() -> anyhow::Result<()> {
    let database = TemporaryDatabase::new()?;
    let (render, rendered) = TestRender::new();
    let app = app(
        AppCapabilities {
            render: Arc::new(render),
        },
        database.path()?.into(),
        AppLayout::Split,
    );
    app.start()?;
    wait_until_ready(&app, &rendered)?;
    let recipes = |event| AppEvent::Split {
        event: SplitAppEvent::Recipes { event },
    };

    create_recipe(&app, &rendered, "Shakshuka")?;
    wait_until(&app, &rendered, |view| {
        split_of(view).is_some_and(|split| {
            matches!(
                &split.recipes.pane.value,
                RecipePaneView::Detail {
                    detail: RecipeDetailView {
                        status: RecipeDetailStatus::Ready { .. },
                        ..
                    }
                }
            )
        })
    })?;
    let scope = split_of(&app.view()?)
        .context("expected the split layout")?
        .recipes
        .pane
        .scope;
    app.dispatch(recipes(RecipeSectionEvent::Detail {
        scope,
        event: RecipeDetailEvent::Edit,
    }))?;
    wait_until(&app, &rendered, |view| {
        split_of(view).is_some_and(|split| {
            matches!(
                &split.recipes.pane.value,
                RecipePaneView::Edit { saved_title, .. } if saved_title == "Shakshuka"
            )
        })
    })?;
    let scope = split_of(&app.view()?)
        .context("expected the split layout")?
        .recipes
        .pane
        .scope;
    app.dispatch(recipes(RecipeSectionEvent::Editor {
        scope,
        event: RecipeEditorEvent::SetTitle {
            title: "Spicy shakshuka".into(),
        },
    }))?;
    app.dispatch(recipes(RecipeSectionEvent::Editor {
        scope,
        event: RecipeEditorEvent::Save,
    }))?;
    wait_until(&app, &rendered, |view| {
        split_of(view).is_some_and(|split| {
            matches!(
                (&split.recipes.list.status, &split.recipes.pane.value),
                (
                    RecipeListStatus::Ready { rows },
                    RecipePaneView::Detail {
                        detail: RecipeDetailView {
                            status: RecipeDetailStatus::Ready { title, .. },
                            ..
                        }
                    },
                ) if rows.iter().map(|row| row.title.as_str()).eq(["Spicy shakshuka"])
                    && title == "Spicy shakshuka"
            )
        })
    })?;
    app.shutdown();
    Ok(())
}

#[test]
fn a_deleted_recipe_leaves_the_list_and_empties_the_pane() -> anyhow::Result<()> {
    let database = TemporaryDatabase::new()?;
    let (render, rendered) = TestRender::new();
    let app = app(
        AppCapabilities {
            render: Arc::new(render),
        },
        database.path()?.into(),
        AppLayout::Split,
    );
    app.start()?;
    wait_until_ready(&app, &rendered)?;
    let recipes = |event| AppEvent::Split {
        event: SplitAppEvent::Recipes { event },
    };

    create_recipe(&app, &rendered, "Shakshuka")?;
    wait_until(&app, &rendered, |view| {
        split_of(view).is_some_and(|split| {
            matches!(
                &split.recipes.pane.value,
                RecipePaneView::Detail {
                    detail: RecipeDetailView {
                        status: RecipeDetailStatus::Ready { .. },
                        ..
                    }
                }
            )
        })
    })?;
    let scope = split_of(&app.view()?)
        .context("expected the split layout")?
        .recipes
        .pane
        .scope;
    app.dispatch(recipes(RecipeSectionEvent::Detail {
        scope,
        event: RecipeDetailEvent::RequestDelete,
    }))?;
    wait_until(&app, &rendered, |view| {
        split_of(view).is_some_and(|split| {
            matches!(
                &split.recipes.pane.value,
                RecipePaneView::Detail {
                    detail: RecipeDetailView {
                        status: RecipeDetailStatus::Ready {
                            deletion: RecipeDeletion::Confirming { .. },
                            ..
                        },
                        ..
                    }
                }
            )
        })
    })?;
    app.dispatch(recipes(RecipeSectionEvent::Detail {
        scope,
        event: RecipeDetailEvent::Confirmation {
            event: ConfirmationEvent::Confirm,
        },
    }))?;
    wait_until(&app, &rendered, |view| {
        split_of(view).is_some_and(|split| {
            split.recipes.pane.value == RecipePaneView::Empty
                && split.recipes.selected.is_none()
                && matches!(&split.recipes.list.status, RecipeListStatus::Ready { rows } if rows.is_empty())
        })
    })?;
    app.shutdown();
    Ok(())
}

#[test]
fn graceful_shutdown_before_initialization_is_polled_drains_prepared_children() -> anyhow::Result<()>
{
    let database = TemporaryDatabase::new()?;
    let (render, _rendered) = TestRender::new();
    let setup = crate::app_shell::AppSetup {
        database_path: database.path()?.into(),
        database: Default::default(),
        render: Arc::new(RenderHandlerShim {
            handler: Arc::new(render),
        }),
        layout: kreatyna::AppLayout::Stack,
    };
    let runtime = Arc::new(krucyfiks_runtime::Runtime::new(
        crate::app_shell::AppShell::new(setup),
    ));
    let runner = runtime.start()?;

    runtime.request_shutdown(krucyfiks_runtime::ShutdownMode::Graceful);
    RUNTIME.block_on(runner);

    let view = runtime.view()?;
    let AppView::Running { app } = view else {
        anyhow::bail!("preparation should finish while the runtime drains, got {view:?}");
    };
    assert!(sections_ready(&app));
    Ok(())
}

#[test]
fn graceful_shutdown_keeps_a_save_accepted_just_before_it() -> anyhow::Result<()> {
    let database = TemporaryDatabase::new()?;
    let (app, rendered) = started_stack_app(&database)?;
    // The lock delays the save, so a shutdown that does not wait returns
    // before the commit. The worker would commit later, but a quitting
    // process does not live that long.
    let lock = DatabaseLock::hold(&database, Duration::from_millis(200))?;
    save_new_ingredient(&app, &rendered, "Garlic")?;

    let result = RUNTIME.block_on(app.shut_down_within(Duration::from_secs(5)));

    assert_eq!(result, ShutdownResult::Completed);
    assert_eq!(committed_names(&database)?, ["Garlic"]);
    lock.release()
}

#[test]
fn graceful_shutdown_before_start_completes_at_once() -> anyhow::Result<()> {
    let database = TemporaryDatabase::new()?;
    let (render, _rendered) = TestRender::new();
    let app = app(
        AppCapabilities {
            render: Arc::new(render),
        },
        database.path()?.into(),
        AppLayout::Stack,
    );

    let result = RUNTIME.block_on(app.shut_down_within(Duration::from_secs(5)));

    assert_eq!(result, ShutdownResult::Completed);
    assert!(matches!(app.start(), Err(AppFfiError::Terminated)));
    Ok(())
}

#[test]
fn graceful_shutdown_returns_after_the_database_closes() -> anyhow::Result<()> {
    let database = TemporaryDatabase::new()?;
    let (app, _rendered) = started_stack_app(&database)?;
    assert!(!app.database.is_finished());

    let result = RUNTIME.block_on(app.shut_down_within(Duration::from_secs(5)));

    assert_eq!(result, ShutdownResult::Completed);
    assert!(app.database.is_finished());
    Ok(())
}

#[test]
fn shutdown_lets_the_database_close() -> anyhow::Result<()> {
    let database = TemporaryDatabase::new()?;
    let (app, _rendered) = started_stack_app(&database)?;

    app.shutdown();

    // Cancel mode does not wait; the task finishes its admitted requests and ends.
    let deadline = Instant::now() + Duration::from_secs(2);
    while !app.database.is_finished() {
        assert!(
            Instant::now() < deadline,
            "the database task should end after shutdown"
        );
        std::thread::sleep(Duration::from_millis(10));
    }
    Ok(())
}

// Validation

#[test]
fn events_after_graceful_shutdown_are_refused() -> anyhow::Result<()> {
    let database = TemporaryDatabase::new()?;
    let (app, _rendered) = started_stack_app(&database)?;
    let scope = stack_view(&app)?.scope;

    let result = RUNTIME.block_on(app.shut_down_within(Duration::from_secs(5)));

    assert_eq!(result, ShutdownResult::Completed);
    assert!(matches!(
        app.dispatch(AppEvent::Stack {
            event: IngredientCatalogEvent::List {
                scope,
                event: IngredientListEvent::Refresh,
            },
        }),
        Err(AppFfiError::Terminated)
    ));
    Ok(())
}

// Red cases

#[test]
fn failed_initialization_can_retry_with_fresh_database_setup() -> anyhow::Result<()> {
    let database = RecoverableDatabase::new()?;
    let (render, rendered) = TestRender::new();
    let app = app(
        AppCapabilities {
            render: Arc::new(render),
        },
        database.path(),
        AppLayout::Stack,
    );

    app.start()?;
    wait_until(&app, &rendered, |view| {
        matches!(
            view,
            AppView::InitializationFailed {
                error: AppInitializationError::DatabaseOpen,
            }
        )
    })?;

    database.create_parent()?;
    app.retry_initialization()?;
    app.retry_initialization()?;
    wait_until_ready(&app, &rendered)?;
    Ok(())
}

#[test]
fn graceful_shutdown_cancels_work_that_outlasts_the_grace_period() -> anyhow::Result<()> {
    let database = TemporaryDatabase::new()?;
    let (app, rendered) = started_stack_app(&database)?;
    let lock = DatabaseLock::hold(&database, Duration::from_secs(1))?;
    save_new_ingredient(&app, &rendered, "Garlic")?;

    let result = RUNTIME.block_on(app.shut_down_within(Duration::from_millis(50)));

    assert_eq!(result, ShutdownResult::Cancelled);
    lock.release()
}

#[test]
fn a_runtime_panic_shows_the_crash_view() -> anyhow::Result<()> {
    let database = TemporaryDatabase::new()?;
    let (render, rendered) = TestRender::new();
    let render = Arc::new(PanicOnceRender {
        armed: std::sync::atomic::AtomicBool::new(false),
        inner: render,
    });
    let app = app(
        AppCapabilities {
            render: render.clone(),
        },
        database.path()?.into(),
        AppLayout::Stack,
    );
    app.start()?;
    wait_until_ready(&app, &rendered)?;
    while rendered.try_recv().is_ok() {}

    // The search's render command panics inside the runtime.
    let scope = stack_view(&app)?.scope;
    render
        .armed
        .store(true, std::sync::atomic::Ordering::SeqCst);
    app.dispatch(AppEvent::Stack {
        event: IngredientCatalogEvent::List {
            scope,
            event: IngredientListEvent::Search {
                query: "gar".into(),
            },
        },
    })?;

    rendered
        .recv_timeout(Duration::from_secs(5))
        .context("the facade did not render after the crash")?;
    assert_eq!(app.view()?, AppView::Crashed);
    assert!(matches!(
        app.dispatch(AppEvent::Stack {
            event: IngredientCatalogEvent::List {
                scope,
                event: IngredientListEvent::Refresh,
            },
        }),
        Err(AppFfiError::Terminated)
    ));
    Ok(())
}

// Test support

struct TestRender {
    rendered: flume::Sender<()>,
}

impl TestRender {
    fn new() -> (Self, flume::Receiver<()>) {
        let (rendered, receiver) = flume::unbounded();
        (Self { rendered }, receiver)
    }
}

#[async_trait::async_trait]
impl RenderHandler for TestRender {
    async fn render(&self) {
        let _ = self.rendered.send(());
    }
}

/// Panics on the first render after `armed` is set, then forwards renders.
struct PanicOnceRender {
    armed: std::sync::atomic::AtomicBool,
    inner: TestRender,
}

#[async_trait::async_trait]
impl RenderHandler for PanicOnceRender {
    #[expect(
        clippy::panic,
        reason = "the test checks that the runtime survives a panic in a render command"
    )]
    async fn render(&self) {
        if self.armed.swap(false, std::sync::atomic::Ordering::SeqCst) {
            panic!("test panic in a render command");
        }
        self.inner.render().await;
    }
}

/// A database file in its own temporary folder. The folder is deleted on drop.
struct TemporaryDatabase {
    _folder: tempfile::TempDir,
    path: PathBuf,
}

impl TemporaryDatabase {
    fn new() -> anyhow::Result<Self> {
        let folder = tempfile::tempdir()?;
        let path = folder.path().join("app.sqlite");
        Ok(Self {
            _folder: folder,
            path,
        })
    }

    fn path(&self) -> anyhow::Result<&str> {
        self.path
            .to_str()
            .context("the temporary path is not UTF-8")
    }
}

/// A database path whose parent folder exists only after `create_parent`.
struct RecoverableDatabase {
    _folder: tempfile::TempDir,
    parent: PathBuf,
}

impl RecoverableDatabase {
    fn new() -> anyhow::Result<Self> {
        let folder = tempfile::tempdir()?;
        let parent = folder.path().join("parent");
        Ok(Self {
            _folder: folder,
            parent,
        })
    }

    fn path(&self) -> String {
        self.parent
            .join("database.sqlite")
            .to_string_lossy()
            .into_owned()
    }

    fn create_parent(&self) -> anyhow::Result<()> {
        std::fs::create_dir(&self.parent)?;
        Ok(())
    }
}

/// A second connection that holds the database's write lock, then releases it
/// after `duration`, so the app's writes wait for it.
struct DatabaseLock {
    thread: std::thread::JoinHandle<diesel::QueryResult<()>>,
}

impl DatabaseLock {
    fn hold(database: &TemporaryDatabase, duration: Duration) -> anyhow::Result<Self> {
        let mut connection = diesel::SqliteConnection::establish(database.path()?)?;
        connection.batch_execute("BEGIN EXCLUSIVE")?;
        let thread = std::thread::spawn(move || {
            std::thread::sleep(duration);
            connection.batch_execute("ROLLBACK")
        });
        Ok(Self { thread })
    }

    fn release(self) -> anyhow::Result<()> {
        self.thread
            .join()
            .map_err(|_| anyhow::anyhow!("the lock thread panicked"))??;
        Ok(())
    }
}

fn started_stack_app(
    database: &TemporaryDatabase,
) -> anyhow::Result<(Arc<AppFfi>, flume::Receiver<()>)> {
    let (render, rendered) = TestRender::new();
    let app = app(
        AppCapabilities {
            render: Arc::new(render),
        },
        database.path()?.into(),
        AppLayout::Stack,
    );
    app.start()?;
    wait_until_ready(&app, &rendered)?;
    Ok((app, rendered))
}

/// New Recipe, type `title`, Save. The runtime handles events in the
/// background, so the editor's scope is read once the editor shows.
fn create_recipe(app: &AppFfi, rendered: &flume::Receiver<()>, title: &str) -> anyhow::Result<()> {
    let recipes = |event| AppEvent::Split {
        event: SplitAppEvent::Recipes { event },
    };
    app.dispatch(recipes(RecipeSectionEvent::NewRecipe))?;
    wait_until(app, rendered, |view| {
        split_of(view)
            .is_some_and(|split| matches!(split.recipes.pane.value, RecipePaneView::Create { .. }))
    })?;
    let scope = split_of(&app.view()?)
        .context("expected the split layout")?
        .recipes
        .pane
        .scope;
    app.dispatch(recipes(RecipeSectionEvent::Editor {
        scope,
        event: RecipeEditorEvent::SetTitle {
            title: title.into(),
        },
    }))?;
    app.dispatch(recipes(RecipeSectionEvent::Editor {
        scope,
        event: RecipeEditorEvent::Save,
    }))?;
    Ok(())
}

/// Opens the create form, types the name, and sends Save without waiting for
/// the save result.
fn save_new_ingredient(
    app: &AppFfi,
    rendered: &flume::Receiver<()>,
    name: &str,
) -> anyhow::Result<()> {
    let catalog = |event| AppEvent::Stack { event };
    app.dispatch(catalog(IngredientCatalogEvent::List {
        scope: stack_view(app)?.scope,
        event: IngredientListEvent::CreateRequested,
    }))?;
    wait_until(app, rendered, |view| {
        stack_of(view).is_some_and(|catalog| {
            matches!(catalog.screen, IngredientCatalogScreenView::Create { .. })
        })
    })?;
    let scope = stack_view(app)?.scope;
    let form = |event| catalog(IngredientCatalogEvent::Form { scope, event });
    app.dispatch(form(IngredientFormEvent::SetField {
        field: IngredientDraftField::Name,
        value: name.into(),
    }))?;
    app.dispatch(form(IngredientFormEvent::Save))?;
    Ok(())
}

/// Names that SQLite has committed, read through a new connection.
fn committed_names(database: &TemporaryDatabase) -> anyhow::Result<Vec<String>> {
    #[derive(diesel::QueryableByName)]
    struct Row {
        #[diesel(sql_type = diesel::sql_types::Text)]
        name: String,
    }
    let mut connection = diesel::SqliteConnection::establish(database.path()?)?;
    let rows = diesel::sql_query("SELECT name FROM ingredients ORDER BY name")
        .load::<Row>(&mut connection)?;
    Ok(rows.into_iter().map(|row| row.name).collect())
}

fn running_view(app: &AppFfi) -> anyhow::Result<RunningAppView> {
    let view = app.view()?;
    let AppView::Running { app } = view else {
        anyhow::bail!("expected a running app, got {view:?}");
    };
    Ok(app)
}

fn wait_until_ready(
    app: &AppFfi,
    rendered: &flume::Receiver<()>,
) -> anyhow::Result<RunningAppView> {
    wait_until(app, rendered, |view| {
        let AppView::Running { app } = view else {
            return false;
        };
        sections_ready(app)
    })?;
    running_view(app)
}

fn stack_view(app: &AppFfi) -> anyhow::Result<IngredientCatalogView> {
    let running = running_view(app)?;
    let RunningAppView::Stack { catalog } = running else {
        anyhow::bail!("expected the stack layout, got {running:?}");
    };
    Ok(catalog)
}

fn stack_of(view: &AppView) -> Option<&IngredientCatalogView> {
    match view {
        AppView::Running {
            app: RunningAppView::Stack { catalog },
        } => Some(catalog),
        _ => None,
    }
}

fn split_of(view: &AppView) -> Option<&SplitAppView> {
    match view {
        AppView::Running {
            app: RunningAppView::Split { app },
        } => Some(app),
        _ => None,
    }
}

/// The stack has one list; the split layout has the recipe and ingredient lists.
fn sections_ready(app: &RunningAppView) -> bool {
    match app {
        RunningAppView::Stack { catalog } => {
            matches!(catalog.list.status, IngredientListStatus::Ready { .. })
        }
        RunningAppView::Split { app } => {
            matches!(app.recipes.list.status, RecipeListStatus::Ready { .. })
                && matches!(
                    app.ingredients.list.value.status,
                    IngredientListStatus::Ready { .. }
                )
        }
    }
}

/// Waits up to 5 seconds, render by render, until the view matches.
fn wait_until(
    app: &AppFfi,
    rendered: &flume::Receiver<()>,
    predicate: impl Fn(&AppView) -> bool,
) -> anyhow::Result<()> {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let view = app.view()?;
        if predicate(&view) {
            return Ok(());
        }
        rendered
            .recv_deadline(deadline)
            .with_context(|| format!("no render brought the expected view; last: {view:?}"))?;
    }
}
