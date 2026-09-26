use std::{
    future::pending,
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};

use super::*;

#[test]
fn app_ffi_matches_the_counter_ffi_runtime_shape() {
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
    assert_eq!(initial.tab, AppTab::Ingredients);
    wait_until_ready(&app, &rendered);

    assert!(matches!(
        app.dispatch(AppEvent::Ingredients {
            scope: app.view().unwrap().scope,
            event: IngredientCatalogEvent::List {
                scope: app.view().unwrap().ingredients.scope,
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
    assert!(matches!(
        app.dispatch(AppEvent::SelectTab {
            tab: AppTab::Counter,
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

impl Drop for TemporaryDatabase {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

fn wait_until_ready(app: &AppFfi, rendered: &flume::Receiver<()>) {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let view = app.view().unwrap();
        if view.counter.status == CounterStatus::Ready
            && matches!(
                view.ingredients.list.status,
                IngredientListStatus::Ready { .. }
            )
        {
            return;
        }
        rendered
            .recv_deadline(deadline)
            .expect("app initialization should render a ready view");
    }
}
