use std::{
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};

use diesel::{Connection, RunQueryDsl, connection::SimpleConnection};

use super::*;

#[test]
fn app_ffi_initializes_before_exposing_the_running_app() {
    let database = TemporaryDatabase::new();
    let (render, rendered) = TestRender::new();
    let app = app(
        AppCapabilities {
            render: Arc::new(render),
        },
        database.path().into(),
        AppLayout::Stack,
    );

    let initial = app.view().unwrap();
    assert_eq!(initial, AppView::Initializing);
    assert!(!std::path::Path::new(database.path()).exists());

    app.start().unwrap();
    assert!(matches!(app.start(), Err(AppFfiError::AlreadyStarted)));
    wait_until_ready(&app, &rendered);

    assert!(matches!(
        app.dispatch(AppEvent::Ingredients {
            event: IngredientCatalogEvent::List {
                scope: stack_view(&app).scope,
                event: IngredientListEvent::Select {
                    id: "not-a-uuid".into(),
                },
            },
        }),
        Err(AppFfiError::Conversion {
            error: AppTypeConversionError::InvalidIngredientId {
                value,
            },
        }) if value == "not-a-uuid"
    ));

    app.shutdown();
    assert!(matches!(app.start(), Err(AppFfiError::Terminated)));
    assert!(matches!(
        app.dispatch(AppEvent::Ingredients {
            event: IngredientCatalogEvent::List {
                scope: stack_view(&app).scope,
                event: IngredientListEvent::Search {
                    query: "gar".into(),
                },
            },
        }),
        Err(AppFfiError::Terminated)
    ));
}

#[test]
fn failed_initialization_can_retry_with_fresh_database_setup() {
    let database = RecoverableDatabase::new();
    let (render, rendered) = TestRender::new();
    let app = app(
        AppCapabilities {
            render: Arc::new(render),
        },
        database.path(),
        AppLayout::Stack,
    );

    app.start().unwrap();
    wait_until(&app, &rendered, |view| {
        matches!(
            view,
            AppView::InitializationFailed {
                error: AppInitializationError::DatabaseOpen,
            }
        )
    });

    database.create_parent();
    app.retry_initialization().unwrap();
    app.retry_initialization().unwrap();
    wait_until_ready(&app, &rendered);
}

#[test]
fn the_split_layout_reaches_a_ready_split_catalog() {
    let database = TemporaryDatabase::new();
    let (render, rendered) = TestRender::new();
    let app = app(
        AppCapabilities {
            render: Arc::new(render),
        },
        database.path().into(),
        AppLayout::Split,
    );

    app.start().unwrap();
    let running = wait_until_ready(&app, &rendered);

    let IngredientsView::Split { catalog } = running.ingredients else {
        panic!("expected the split layout")
    };
    assert_eq!(catalog.selected, None);
    assert_eq!(catalog.pane.value, IngredientSplitPaneView::Empty);
    app.shutdown();
}

#[test]
fn graceful_shutdown_before_initialization_is_polled_drains_prepared_children() {
    let database = TemporaryDatabase::new();
    let (render, _rendered) = TestRender::new();
    let setup = crate::app_shell::AppSetup {
        database_path: database.path().into(),
        database: Default::default(),
        render: Arc::new(RenderHandlerShim {
            handler: Arc::new(render),
        }),
        layout: kreatyna::AppLayout::Stack,
    };
    let runtime = Arc::new(krucyfiks_runtime::Runtime::new(
        crate::app_shell::AppShell::new(setup),
    ));
    let runner = runtime.start().expect("start should succeed");

    runtime.request_shutdown(krucyfiks_runtime::ShutdownMode::Graceful);
    RUNTIME.block_on(runner);

    let AppView::Running { app } = runtime.view().unwrap() else {
        panic!("preparation should finish while the runtime drains")
    };
    assert!(matches!(
        list_status(&app),
        IngredientListStatus::Ready { .. }
    ));
}

#[test]
fn graceful_shutdown_keeps_a_save_accepted_just_before_it() {
    let database = TemporaryDatabase::new();
    let (app, rendered) = started_stack_app(&database);
    // The lock delays the save, so a shutdown that does not wait returns
    // before the commit. The worker would commit later, but a quitting
    // process does not live that long.
    let lock = DatabaseLock::hold(&database, Duration::from_millis(200));
    save_new_ingredient(&app, &rendered, "Garlic");

    let result = RUNTIME.block_on(app.shut_down_within(Duration::from_secs(5)));

    assert_eq!(result, ShutdownResult::Completed);
    assert_eq!(committed_names(&database), ["Garlic"]);
    lock.release();
}

#[test]
fn graceful_shutdown_cancels_work_that_outlasts_the_grace_period() {
    let database = TemporaryDatabase::new();
    let (app, rendered) = started_stack_app(&database);
    let lock = DatabaseLock::hold(&database, Duration::from_secs(1));
    save_new_ingredient(&app, &rendered, "Garlic");

    let result = RUNTIME.block_on(app.shut_down_within(Duration::from_millis(50)));

    assert_eq!(result, ShutdownResult::Cancelled);
    lock.release();
}

#[test]
fn graceful_shutdown_before_start_completes_at_once() {
    let database = TemporaryDatabase::new();
    let (render, _rendered) = TestRender::new();
    let app = app(
        AppCapabilities {
            render: Arc::new(render),
        },
        database.path().into(),
        AppLayout::Stack,
    );

    let result = RUNTIME.block_on(app.shut_down_within(Duration::from_secs(5)));

    assert_eq!(result, ShutdownResult::Completed);
    assert!(matches!(app.start(), Err(AppFfiError::Terminated)));
}

#[test]
fn events_after_graceful_shutdown_are_refused() {
    let database = TemporaryDatabase::new();
    let (app, _rendered) = started_stack_app(&database);
    let scope = stack_view(&app).scope;

    let result = RUNTIME.block_on(app.shut_down_within(Duration::from_secs(5)));

    assert_eq!(result, ShutdownResult::Completed);
    assert!(matches!(
        app.dispatch(AppEvent::Ingredients {
            event: IngredientCatalogEvent::List {
                scope,
                event: IngredientListEvent::Refresh,
            },
        }),
        Err(AppFfiError::Terminated)
    ));
}

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

/// A database file in its own temporary folder. The folder is deleted on drop.
struct TemporaryDatabase {
    _folder: tempfile::TempDir,
    path: PathBuf,
}

impl TemporaryDatabase {
    fn new() -> Self {
        let folder = tempfile::tempdir().unwrap();
        let path = folder.path().join("app.sqlite");
        Self {
            _folder: folder,
            path,
        }
    }

    fn path(&self) -> &str {
        self.path.to_str().unwrap()
    }
}

/// A database path whose parent folder exists only after `create_parent`.
struct RecoverableDatabase {
    _folder: tempfile::TempDir,
    parent: PathBuf,
}

impl RecoverableDatabase {
    fn new() -> Self {
        let folder = tempfile::tempdir().unwrap();
        let parent = folder.path().join("parent");
        Self {
            _folder: folder,
            parent,
        }
    }

    fn path(&self) -> String {
        self.parent
            .join("database.sqlite")
            .to_string_lossy()
            .into_owned()
    }

    fn create_parent(&self) {
        std::fs::create_dir(&self.parent).unwrap();
    }
}

fn started_stack_app(database: &TemporaryDatabase) -> (Arc<AppFfi>, flume::Receiver<()>) {
    let (render, rendered) = TestRender::new();
    let app = app(
        AppCapabilities {
            render: Arc::new(render),
        },
        database.path().into(),
        AppLayout::Stack,
    );
    app.start().unwrap();
    wait_until_ready(&app, &rendered);
    (app, rendered)
}

/// Opens the create form, types the name, and sends Save without waiting for
/// the save result.
fn save_new_ingredient(app: &AppFfi, rendered: &flume::Receiver<()>, name: &str) {
    let catalog = |event| AppEvent::Ingredients { event };
    app.dispatch(catalog(IngredientCatalogEvent::List {
        scope: stack_view(app).scope,
        event: IngredientListEvent::CreateRequested,
    }))
    .unwrap();
    wait_until(app, rendered, |_| {
        matches!(
            stack_view(app).screen,
            IngredientCatalogScreenView::Create { .. }
        )
    });
    let scope = stack_view(app).scope;
    let form = |event| catalog(IngredientCatalogEvent::Form { scope, event });
    app.dispatch(form(IngredientFormEvent::SetField {
        field: IngredientDraftField::Name,
        value: name.into(),
    }))
    .unwrap();
    app.dispatch(form(IngredientFormEvent::Save)).unwrap();
}

/// A second connection that holds the database's write lock, then releases it
/// after `duration`, so the app's writes wait for it.
struct DatabaseLock {
    thread: std::thread::JoinHandle<()>,
}

impl DatabaseLock {
    fn hold(database: &TemporaryDatabase, duration: Duration) -> Self {
        let mut connection = diesel::SqliteConnection::establish(database.path()).unwrap();
        connection.batch_execute("BEGIN EXCLUSIVE").unwrap();
        let thread = std::thread::spawn(move || {
            std::thread::sleep(duration);
            connection.batch_execute("ROLLBACK").unwrap();
        });
        Self { thread }
    }

    fn release(self) {
        self.thread.join().unwrap();
    }
}

/// Names that SQLite has committed, read through a new connection.
fn committed_names(database: &TemporaryDatabase) -> Vec<String> {
    #[derive(diesel::QueryableByName)]
    struct Row {
        #[diesel(sql_type = diesel::sql_types::Text)]
        name: String,
    }
    let mut connection = diesel::SqliteConnection::establish(database.path()).unwrap();
    diesel::sql_query("SELECT name FROM ingredients ORDER BY name")
        .load::<Row>(&mut connection)
        .unwrap()
        .into_iter()
        .map(|row| row.name)
        .collect()
}

fn running_view(app: &AppFfi) -> RunningAppView {
    let AppView::Running { app } = app.view().unwrap() else {
        panic!("app should be running")
    };
    app
}

fn wait_until_ready(app: &AppFfi, rendered: &flume::Receiver<()>) -> RunningAppView {
    wait_until(app, rendered, |view| {
        let AppView::Running { app } = view else {
            return false;
        };
        matches!(list_status(app), IngredientListStatus::Ready { .. })
    });
    running_view(app)
}

fn stack_view(app: &AppFfi) -> IngredientCatalogView {
    let IngredientsView::Stack { catalog } = running_view(app).ingredients else {
        panic!("expected the stack layout")
    };
    catalog
}

fn list_status(app: &RunningAppView) -> &IngredientListStatus {
    match &app.ingredients {
        IngredientsView::Stack { catalog } => &catalog.list.status,
        IngredientsView::Split { catalog } => &catalog.list.value.status,
    }
}

fn wait_until(app: &AppFfi, rendered: &flume::Receiver<()>, predicate: impl Fn(&AppView) -> bool) {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let view = app.view().unwrap();
        if predicate(&view) {
            return;
        }
        rendered
            .recv_deadline(deadline)
            .expect("app initialization should render a ready view");
    }
}

#[test]
fn graceful_shutdown_returns_after_the_database_closes() {
    let database = TemporaryDatabase::new();
    let (app, _rendered) = started_stack_app(&database);
    assert!(!app.database.is_finished());

    let result = RUNTIME.block_on(app.shut_down_within(Duration::from_secs(5)));

    assert_eq!(result, ShutdownResult::Completed);
    assert!(app.database.is_finished());
}

#[test]
fn shutdown_lets_the_database_close() {
    let database = TemporaryDatabase::new();
    let (app, _rendered) = started_stack_app(&database);

    app.shutdown();

    // Cancel mode does not wait; the task finishes its admitted requests and ends.
    let deadline = std::time::Instant::now() + Duration::from_secs(2);
    while !app.database.is_finished() {
        assert!(
            std::time::Instant::now() < deadline,
            "the database task should end after shutdown"
        );
        std::thread::sleep(Duration::from_millis(10));
    }
}

#[test]
fn a_runtime_panic_shows_the_crash_view() {
    let database = TemporaryDatabase::new();
    let (render, rendered) = TestRender::new();
    let render = Arc::new(PanicOnceRender {
        armed: std::sync::atomic::AtomicBool::new(false),
        inner: render,
    });
    let app = app(
        AppCapabilities {
            render: render.clone(),
        },
        database.path().into(),
        AppLayout::Stack,
    );
    app.start().unwrap();
    wait_until_ready(&app, &rendered);
    while rendered.try_recv().is_ok() {}

    // The search's render command panics inside the runtime.
    let scope = stack_view(&app).scope;
    render
        .armed
        .store(true, std::sync::atomic::Ordering::SeqCst);
    app.dispatch(AppEvent::Ingredients {
        event: IngredientCatalogEvent::List {
            scope,
            event: IngredientListEvent::Search {
                query: "gar".into(),
            },
        },
    })
    .unwrap();

    rendered
        .recv_timeout(Duration::from_secs(5))
        .expect("the facade renders after the crash");
    assert_eq!(app.view().unwrap(), AppView::Crashed);
    assert!(matches!(
        app.dispatch(AppEvent::Ingredients {
            event: IngredientCatalogEvent::List {
                scope,
                event: IngredientListEvent::Refresh,
            },
        }),
        Err(AppFfiError::Terminated)
    ));
}

/// Panics on the first render after `armed` is set, then forwards renders.
struct PanicOnceRender {
    armed: std::sync::atomic::AtomicBool,
    inner: TestRender,
}

#[async_trait::async_trait]
impl RenderHandler for PanicOnceRender {
    async fn render(&self) {
        if self.armed.swap(false, std::sync::atomic::Ordering::SeqCst) {
            panic!("test panic in a render command");
        }
        self.inner.render().await;
    }
}
