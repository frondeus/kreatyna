use std::{
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};

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
