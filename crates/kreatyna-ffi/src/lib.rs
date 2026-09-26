use std::sync::{Arc, LazyLock};

use kreatyna::{capabilities::DatabaseClient, ingredients::capabilities::IngredientStoreClient};
use krucyfiks_runtime::{Runtime, RuntimeError};
use tokio::task::JoinHandle;

use crate::database::sqlite::SqliteDatabase;

mod app_types;
mod database;
mod logging;

pub use app_types::*;

uniffi::setup_scaffolding!();

static RUNTIME: LazyLock<tokio::runtime::Runtime> = LazyLock::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .build()
        .expect("Tokio runtime")
});

#[derive(uniffi::Object)]
pub struct CounterFfi {
    runtime: Arc<Runtime<kreatyna::Counter>>,
    handle: JoinHandle<()>,
}

#[derive(uniffi::Object)]
pub struct AppFfi {
    runtime: Arc<Runtime<kreatyna::AppWidget>>,
    handle: JoinHandle<()>,
}

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum FfiRuntimeError {
    #[error("Runtime is terminated")]
    Terminated,

    #[error("Runtime has already been started")]
    AlreadyStarted,
}
impl From<RuntimeError> for FfiRuntimeError {
    fn from(value: RuntimeError) -> Self {
        match value {
            RuntimeError::Terminated => Self::Terminated,
            RuntimeError::AlreadyStarted => Self::AlreadyStarted,
        }
    }
}

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum AppFfiError {
    #[error("Invalid app event: {error}")]
    Conversion { error: AppTypeConversionError },
    #[error("Runtime is terminated")]
    Terminated,
    #[error("Runtime has already been started")]
    AlreadyStarted,
}

impl From<AppTypeConversionError> for AppFfiError {
    fn from(error: AppTypeConversionError) -> Self {
        Self::Conversion { error }
    }
}

impl From<RuntimeError> for AppFfiError {
    fn from(value: RuntimeError) -> Self {
        match value {
            RuntimeError::Terminated => Self::Terminated,
            RuntimeError::AlreadyStarted => Self::AlreadyStarted,
        }
    }
}

#[uniffi::export]
impl CounterFfi {
    pub fn view(&self) -> u64 {
        self.runtime.view().count as u64
    }

    pub fn dispatch(&self, event: CounterEvent) -> Result<(), FfiRuntimeError> {
        let event: kreatyna::CounterEvent = event.into();
        self.runtime.dispatch(krucyfiks::Message::Event(event))?;
        Ok(())
    }

    pub fn shutdown(&self) {
        self.runtime.shutdown();
    }
}

#[uniffi::export]
impl AppFfi {
    pub fn view(&self) -> Result<AppView, AppTypeConversionError> {
        self.runtime.view().try_into()
    }

    pub fn dispatch(&self, event: AppEvent) -> Result<(), AppFfiError> {
        let event: kreatyna::AppEvent = event.try_into()?;
        self.runtime.dispatch(krucyfiks::Message::Event(event))?;
        Ok(())
    }

    pub fn shutdown(&self) {
        self.runtime.shutdown();
    }
}

#[uniffi::export(foreign)]
#[async_trait::async_trait]
pub trait RenderHandler: Send + Sync {
    async fn render(&self);
}

struct RenderHandlerShim {
    handler: Arc<dyn RenderHandler>,
}

#[async_trait::async_trait]
impl kreatyna::capabilities::Render for RenderHandlerShim {
    async fn render(&self) {
        self.handler.render().await;
    }
}

#[uniffi::export(foreign)]
#[async_trait::async_trait]
pub trait ClockHandler: Send + Sync {
    async fn sleep_ms(&self, ms: u64);
}

struct ClockHandlerShim {
    handler: Arc<dyn ClockHandler>,
}

#[async_trait::async_trait]
impl kreatyna::capabilities::Clock for ClockHandlerShim {
    async fn sleep_ms(&self, ms: u64) {
        self.handler.sleep_ms(ms).await
    }
}

#[derive(uniffi::Record)]
pub struct CounterCapabilities {
    render: Arc<dyn RenderHandler>,
    clock: Arc<dyn ClockHandler>,
}

#[derive(uniffi::Record)]
pub struct AppCapabilities {
    render: Arc<dyn RenderHandler>,
    clock: Arc<dyn ClockHandler>,
}

#[uniffi::export]
pub fn counter(caps: CounterCapabilities, database_path: String) -> Arc<CounterFfi> {
    let render = Arc::new(RenderHandlerShim {
        handler: caps.render,
    });
    let clock = Arc::new(ClockHandlerShim {
        handler: caps.clock,
    });

    let (db, database_rx) = DatabaseClient::new(32, 0);
    let db = Arc::new(db);
    RUNTIME.spawn(async move {
        let mut database = SqliteDatabase::new(&database_path).await?;
        database.worker(database_rx).await;

        Ok::<(), anyhow::Error>(())
    });

    let caps = kreatyna::CounterCapabilities { render, clock, db };
    let counter = kreatyna::Counter::new(caps);
    let runtime = Arc::new(Runtime::new(counter));
    let handle = RUNTIME.spawn({
        let runtime = runtime.clone();
        async move {
            runtime.run().await.expect("Runtime to run correctly");
        }
    });

    Arc::new(CounterFfi { runtime, handle })
}

#[uniffi::export]
pub fn app(caps: AppCapabilities, database_path: String) -> Arc<AppFfi> {
    let render = Arc::new(RenderHandlerShim {
        handler: caps.render,
    });
    let clock = Arc::new(ClockHandlerShim {
        handler: caps.clock,
    });

    let (db, database_rx) = DatabaseClient::new(32, 0);
    let (store, ingredient_rx) = IngredientStoreClient::new(32, 0);
    RUNTIME.spawn(async move {
        let mut database = SqliteDatabase::new(&database_path).await?;
        database.app_worker(database_rx, ingredient_rx).await;

        Ok::<(), anyhow::Error>(())
    });

    let caps = kreatyna::AppCapabilities {
        ingredients: kreatyna::ingredients::capabilities::IngredientCapabilities {
            render: render.clone(),
            store: Arc::new(store),
        },
        counter: kreatyna::CounterCapabilities {
            render: render.clone(),
            clock,
            db: Arc::new(db),
        },
        render,
    };
    let app = kreatyna::AppWidget::new(caps);
    let runtime = Arc::new(Runtime::new(app));
    let handle = RUNTIME.spawn({
        let runtime = runtime.clone();
        async move {
            runtime.run().await.expect("Runtime to run correctly");
        }
    });

    Arc::new(AppFfi { runtime, handle })
}

impl Drop for CounterFfi {
    fn drop(&mut self) {
        self.shutdown();
        self.handle.abort();
    }
}

impl Drop for AppFfi {
    fn drop(&mut self) {
        self.shutdown();
        self.handle.abort();
    }
}

#[cfg(test)]
mod app_runtime_tests;
