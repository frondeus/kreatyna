use std::{
    future::pending,
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
            clock: Arc::new(TestClock),
        },
        database.path().into(),
    );

    let initial = app.view().unwrap();
    assert_eq!(initial, AppView::Initializing);
    assert!(!database.0.exists());

    app.start().unwrap();
    assert!(matches!(app.start(), Err(AppFfiError::AlreadyStarted)));
    let ready = wait_until_ready(&app, &rendered);
    assert_eq!(ready.tab, AppTab::Ingredients);

    assert!(matches!(
        app.dispatch(AppEvent::Ingredients {
            scope: running_view(&app).scope,
            event: IngredientCatalogEvent::List {
                scope: running_view(&app).ingredients.scope,
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
        app.dispatch(AppEvent::SelectTab {
            tab: AppTab::Counter,
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
            clock: Arc::new(TestClock),
        },
        database.path(),
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

    std::fs::create_dir(&database.0).unwrap();
    app.retry_initialization().unwrap();
    app.retry_initialization().unwrap();
    wait_until_ready(&app, &rendered);
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

struct TestClock;

#[async_trait::async_trait]
impl ClockHandler for TestClock {
    async fn sleep_ms(&self, _: u64) {
        pending::<()>().await;
    }
}

struct TemporaryDatabase(PathBuf);

impl TemporaryDatabase {
    fn new() -> Self {
        Self(std::env::temp_dir().join(format!(
            "app-ffi-{}.sqlite",
            kreatyna::ingredients::IngredientId::new()
        )))
    }

    fn path(&self) -> &str {
        self.0.to_str().unwrap()
    }
}

struct RecoverableDatabase(PathBuf);

impl RecoverableDatabase {
    fn new() -> Self {
        Self(std::env::temp_dir().join(format!(
            "app-ffi-parent-{}",
            kreatyna::ingredients::IngredientId::new()
        )))
    }

    fn path(&self) -> String {
        self.0
            .join("database.sqlite")
            .to_string_lossy()
            .into_owned()
    }
}

impl Drop for RecoverableDatabase {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(self.0.join("database.sqlite"));
        let _ = std::fs::remove_dir(&self.0);
    }
}

impl Drop for TemporaryDatabase {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
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
        app.counter.status == CounterStatus::Ready
            && matches!(
                app.ingredients.list.status,
                IngredientListStatus::Ready { .. }
            )
    });
    running_view(app)
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
