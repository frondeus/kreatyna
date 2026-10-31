use std::{
    sync::{Arc, LazyLock},
    time::Duration,
};

use krucyfiks_runtime::{Runtime, RuntimeError, ShutdownMode};
use parking_lot::Mutex;
use tokio::task::JoinHandle;

use crate::app_shell::{AppSetup, AppShell, AppShellEvent};

mod app_shell;
mod app_types;
mod logging;
mod seed;

pub use app_types::*;
pub use seed::*;

uniffi::setup_scaffolding!();

static RUNTIME: LazyLock<tokio::runtime::Runtime> = LazyLock::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_time()
        .build()
        .expect("Tokio runtime")
});

/// How long graceful shutdown waits for accepted work (P05/Q2).
const SHUTDOWN_GRACE: Duration = Duration::from_secs(5);

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum ShutdownResult {
    /// All accepted work finished: every accepted save committed or failed
    /// visibly before the runtime stopped.
    Completed,
    /// Accepted work outlasted the grace period and was cancelled.
    Cancelled,
}

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

    /// Stops at once and drops pending work, including accepted saves.
    pub fn shutdown(&self) {
        self.runtime.shutdown();
    }

    /// Stops accepting events, finishes accepted work, and returns when the
    /// runtime has stopped. Work that outlasts the grace period is cancelled.
    /// Call it once, when the app quits; a later call returns `Completed` at
    /// once without waiting for the first.
    pub async fn shut_down_gracefully(&self) -> ShutdownResult {
        self.shut_down_within(SHUTDOWN_GRACE).await
    }
}

impl AppFfi {
    async fn shut_down_within(&self, grace: Duration) -> ShutdownResult {
        self.runtime.request_shutdown(ShutdownMode::Graceful);
        // Not started: the runtime accepted no work.
        let Some(mut runner) = self.handle.lock().take() else {
            return ShutdownResult::Completed;
        };
        let runtime = self.runtime.clone();
        // The timer needs the app's Tokio runtime; the caller's executor only
        // polls the join handle.
        RUNTIME
            .spawn(async move {
                match tokio::time::timeout(grace, &mut runner).await {
                    Ok(_) => ShutdownResult::Completed,
                    Err(_) => {
                        tracing::warn!(
                            ?grace,
                            "Graceful shutdown timed out; cancelling pending work"
                        );
                        runtime.request_shutdown(ShutdownMode::Cancel);
                        let _ = runner.await;
                        ShutdownResult::Cancelled
                    }
                }
            })
            .await
            .unwrap_or(ShutdownResult::Cancelled)
    }
}

/// Rust calls `render` after each state change. The handler then reads
/// `AppFfi::view()`.
///
/// Rust keeps the handler as long as the facade lives. A handler must not
/// hold the facade strongly: that forms a cycle (facade, runtime, widget,
/// handler, facade) that nothing breaks (P08). Hold the facade's owner
/// weakly, or only record a dirty flag.
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
pub fn app(caps: AppCapabilities, database_path: String, layout: AppLayout) -> Arc<AppFfi> {
    let render: Arc<dyn kreatyna::capabilities::Render> = Arc::new(RenderHandlerShim {
        handler: caps.render,
    });

    let widget = AppShell::new(AppSetup {
        database_path,
        render,
        layout: layout.into(),
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
