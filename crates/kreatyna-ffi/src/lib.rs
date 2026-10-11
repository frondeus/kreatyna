use std::sync::{Arc, LazyLock};

use krucyfiks_runtime::{Runtime, RuntimeError};
use parking_lot::Mutex;
use tokio::task::JoinHandle;

use crate::app_shell::{AppSetup, AppShell, AppShellEvent};

mod app_shell;
mod app_types;
mod logging;

pub use app_types::*;

uniffi::setup_scaffolding!();

static RUNTIME: LazyLock<tokio::runtime::Runtime> = LazyLock::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .build()
        .expect("Tokio runtime")
});

#[derive(uniffi::Object)]
pub struct AppFfi {
    runtime: Arc<Runtime<AppShell>>,
    handle: Mutex<Option<JoinHandle<()>>>,
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
impl AppFfi {
    pub fn view(&self) -> Result<AppView, AppTypeConversionError> {
        self.runtime.view()
    }

    /// Starts runtime execution and asynchronous application initialization.
    ///
    /// Prepare all dependencies used by the render handler before calling this
    /// method: callbacks may begin before it returns. A handler
    /// that reads `view()` must already have access to this facade; a handler
    /// that only records a dirty flag need not hold a facade reference.
    /// Construction alone does not invoke these handlers.
    ///
    /// Read `view()` before starting to display the initial `Initializing` state.
    /// Success means execution has started, not that initialization has finished;
    /// observe initialization success or failure through `view()` and render
    /// notifications. Repeated starts return `AlreadyStarted`, and starting after
    /// shutdown returns `Terminated`.
    pub fn start(&self) -> Result<(), AppFfiError> {
        let runner = self.runtime.start()?;
        *self.handle.lock() = Some(RUNTIME.spawn(runner));
        Ok(())
    }

    pub fn retry_initialization(&self) -> Result<(), AppFfiError> {
        self.runtime.dispatch(krucyfiks::Message::Event(
            AppShellEvent::RetryInitialization,
        ))?;
        Ok(())
    }

    pub fn dispatch(&self, event: AppEvent) -> Result<(), AppFfiError> {
        let event: kreatyna::AppEvent = event.try_into()?;
        self.runtime
            .dispatch(krucyfiks::Message::Event(AppShellEvent::App(event)))?;
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

#[derive(uniffi::Record)]
pub struct AppCapabilities {
    render: Arc<dyn RenderHandler>,
}

#[uniffi::export]
pub fn app(caps: AppCapabilities, database_path: String) -> Arc<AppFfi> {
    let render: Arc<dyn kreatyna::capabilities::Render> = Arc::new(RenderHandlerShim {
        handler: caps.render,
    });

    let widget = AppShell::new(AppSetup {
        database_path,
        render,
    });
    let runtime = Arc::new(Runtime::new(widget));

    Arc::new(AppFfi {
        runtime,
        handle: Mutex::new(None),
    })
}

impl Drop for AppFfi {
    fn drop(&mut self) {
        self.shutdown();
        if let Some(handle) = self.handle.get_mut().take() {
            handle.abort();
        }
    }
}

#[cfg(test)]
mod app_runtime_tests;
