use std::sync::{Arc, LazyLock};

use krucyfiks_runtime::{Runtime, RuntimeError};
use mausritter::capabilities::{Database, DatabaseError};
use tokio::task::JoinHandle;

uniffi::setup_scaffolding!();

static RUNTIME: LazyLock<tokio::runtime::Runtime> = LazyLock::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .build()
        .expect("Tokio runtime")
});

#[derive(uniffi::Object)]
pub struct CounterFfi {
    runtime: Arc<Runtime<mausritter::Counter>>,
    handle: JoinHandle<()>,
}

#[derive(uniffi::Enum)]
pub enum CounterEvent {
    Increase,
    Decrease,
    StartBackground,
    StopBackground,
}

impl From<CounterEvent> for mausritter::CounterEvent {
    fn from(value: CounterEvent) -> Self {
        match value {
            CounterEvent::Increase => Self::Increase,
            CounterEvent::Decrease => Self::Decrease,
            CounterEvent::StartBackground => Self::StartBackground,
            CounterEvent::StopBackground => Self::StopBackground,
        }
    }
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

#[uniffi::export]
impl CounterFfi {
    pub fn view(&self) -> u64 {
        self.runtime.view() as u64
    }

    pub fn dispatch(&self, event: CounterEvent) -> Result<(), FfiRuntimeError> {
        let event: mausritter::CounterEvent = event.into();
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
impl mausritter::capabilities::Render for RenderHandlerShim {
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
impl mausritter::capabilities::Clock for ClockHandlerShim {
    async fn sleep_ms(&self, ms: u64) {
        self.handler.sleep_ms(ms).await
    }
}

#[uniffi::export]
pub fn counter(render: Arc<dyn RenderHandler>, clock: Arc<dyn ClockHandler>) -> Arc<CounterFfi> {
    let render = Arc::new(RenderHandlerShim { handler: render });
    let clock = Arc::new(ClockHandlerShim { handler: clock });
    let db = Arc::new(DatabaseHandler);
    let caps = mausritter::CounterCapabilities { render, clock, db };
    let counter = mausritter::Counter::new(caps);

    let runtime = Arc::new(Runtime::new(counter));
    let handle = RUNTIME.spawn({
        let runtime = runtime.clone();
        async move {
            runtime.run().await.expect("Runtime to run correctly");
        }
    });

    Arc::new(CounterFfi { runtime, handle })
}

impl Drop for CounterFfi {
    fn drop(&mut self) {
        self.shutdown();
        self.handle.abort();
    }
}

struct DatabaseHandler;

#[async_trait::async_trait]
impl Database for DatabaseHandler {
    async fn count(&self) -> Result<Option<usize>, DatabaseError> {
        Ok(Some(42))
    }

    async fn set_count(&self, _count: usize) -> Result<(), DatabaseError> {
        Ok(())
    }
}
