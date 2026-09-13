use std::sync::Arc;

use mausritter_core::runtime::{Runtime, RuntimeError};

uniffi::setup_scaffolding!();

#[derive(uniffi::Object)]
pub struct CounterFfi {
    runtime: Runtime<mausritter_core::Counter>,
}

#[derive(uniffi::Enum)]
pub enum CounterEvent {
    Increase,
    Decrease,
    StartBackground,
    StopBackground,
}

impl From<CounterEvent> for mausritter_core::CounterEvent {
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
        let event: mausritter_core::CounterEvent = event.into();
        self.runtime
            .dispatch(mausritter_core::widget::Message::Event(event))?;
        Ok(())
    }

    pub async fn run(&self) -> Result<(), FfiRuntimeError> {
        self.runtime.run().await?;
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
impl mausritter_core::capabilities::Render for RenderHandlerShim {
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
impl mausritter_core::capabilities::Clock for ClockHandlerShim {
    async fn sleep_ms(&self, ms: u64) {
        self.handler.sleep_ms(ms).await
    }
}

#[uniffi::export]
pub fn counter(render: Arc<dyn RenderHandler>, clock: Arc<dyn ClockHandler>) -> Arc<CounterFfi> {
    let render = Arc::new(RenderHandlerShim { handler: render });
    let clock = Arc::new(ClockHandlerShim { handler: clock });
    let counter = mausritter_core::Counter::new(render, clock);

    let runtime = Runtime::new(counter);

    Arc::new(CounterFfi { runtime })
}
