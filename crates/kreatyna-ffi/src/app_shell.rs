use std::{fmt, sync::Arc};

use futures::StreamExt;
use kreatyna::{
    AppWidget, CLIENT_CAPABILITY, capabilities::Render,
    ingredients::capabilities::IngredientStoreClient,
};
use krucyfiks::{Command, CommandExt, Message, Widget, effects::RequestCloseHandle};
use tokio::task::JoinHandle;

use crate::{AppInitializationError, AppTypeConversionError, AppView, RUNTIME, RunningAppView};
use kreatyna_db::sqlite::{DatabaseEffect, SqliteDatabase};

#[derive(Clone)]
pub(crate) struct AppSetup {
    pub(crate) database_path: String,
    pub(crate) render: Arc<dyn Render>,
    pub(crate) layout: kreatyna::AppLayout,
}

pub(crate) enum AppShellEvent {
    RetryInitialization,
    App(kreatyna::AppEvent),
}

impl fmt::Debug for AppShellEvent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RetryInitialization => formatter.write_str("RetryInitialization"),
            Self::App(event) => formatter.debug_tuple("App").field(event).finish(),
        }
    }
}

pub(crate) enum AppShellOutcome {
    InitializationFinished(Result<Box<PreparedApp>, AppInitializationError>),
    App(kreatyna::AppOutcome),
}

impl fmt::Debug for AppShellOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InitializationFinished(Ok(_)) => {
                formatter.write_str("InitializationFinished(Ok)")
            }
            Self::InitializationFinished(Err(error)) => formatter
                .debug_tuple("InitializationFinished")
                .field(error)
                .finish(),
            Self::App(outcome) => formatter.debug_tuple("App").field(outcome).finish(),
        }
    }
}

enum AppShellState {
    Initializing,
    InitializationFailed(AppInitializationError),
    Running(Box<PreparedApp>),
}

pub(crate) struct AppShell {
    state: AppShellState,
    setup: AppSetup,
}

impl AppShell {
    pub(crate) fn new(setup: AppSetup) -> Self {
        Self {
            state: AppShellState::Initializing,
            setup,
        }
    }

    fn render(&self) -> Command<AppShellOutcome> {
        let render = self.setup.render.clone();
        Command::without_outcome(async move { render.render().await })
    }
}

impl Widget for AppShell {
    type View = Result<AppView, AppTypeConversionError>;
    type Event = AppShellEvent;
    type Outcome = AppShellOutcome;

    fn init(&mut self) -> Command<Self::Outcome> {
        let setup = self.setup.clone();
        Command::single(
            async move { AppShellOutcome::InitializationFinished(prepare_app(setup).await) },
        )
    }

    fn can_begin_shutdown(&self) -> bool {
        match &self.state {
            AppShellState::Initializing => false,
            AppShellState::InitializationFailed(_) => true,
            AppShellState::Running(prepared) => prepared.app.can_begin_shutdown(),
        }
    }

    fn begin_shutdown(&mut self) -> Command<Self::Outcome> {
        let AppShellState::Running(prepared) = &mut self.state else {
            return Command::empty();
        };
        prepared.app.begin_shutdown().map(AppShellOutcome::App)
    }

    fn update(&mut self, message: Message<Self>) -> Command<Self::Outcome> {
        match message {
            Message::Event(AppShellEvent::RetryInitialization)
                if matches!(self.state, AppShellState::InitializationFailed(_)) =>
            {
                self.state = AppShellState::Initializing;
                let initialize = self.init();
                Command::select_all(vec![self.render(), initialize])
            }
            Message::Outcome(AppShellOutcome::InitializationFinished(result))
                if matches!(self.state, AppShellState::Initializing) =>
            {
                match result {
                    Ok(mut prepared) => {
                        let init = prepared.app.init().map(AppShellOutcome::App);
                        self.state = AppShellState::Running(prepared);
                        Command::select_all(vec![self.render(), init])
                    }
                    Err(error) => {
                        self.state = AppShellState::InitializationFailed(error);
                        self.render()
                    }
                }
            }
            Message::Event(AppShellEvent::App(event)) => {
                let AppShellState::Running(prepared) = &mut self.state else {
                    return Command::empty();
                };
                prepared
                    .app
                    .update(Message::Event(event))
                    .map(AppShellOutcome::App)
            }
            Message::Outcome(AppShellOutcome::App(outcome)) => {
                let AppShellState::Running(prepared) = &mut self.state else {
                    return Command::empty();
                };
                prepared
                    .app
                    .update(Message::Outcome(outcome))
                    .map(AppShellOutcome::App)
            }
            Message::Event(AppShellEvent::RetryInitialization)
            | Message::Outcome(AppShellOutcome::InitializationFinished(_)) => Command::empty(),
        }
    }

    fn view(&self) -> Self::View {
        Ok(match &self.state {
            AppShellState::Initializing => AppView::Initializing,
            AppShellState::InitializationFailed(error) => {
                AppView::InitializationFailed { error: *error }
            }
            AppShellState::Running(prepared) => AppView::Running {
                app: RunningAppView::try_from(prepared.app.view())?,
            },
        })
    }
}

pub(crate) struct PreparedApp {
    app: AppWidget,
    _database_worker: DatabaseWorker,
}

#[allow(dead_code)]
struct DatabaseWorker {
    task: JoinHandle<()>,

    close_handles: Vec<RequestCloseHandle>,
}

#[allow(dead_code)]
impl DatabaseWorker {
    /// Closes request admission for every capability served by this worker.
    /// Already admitted requests remain available for the worker to drain.
    pub fn request_close(&self) {
        self.close_handles
            .iter()
            .for_each(RequestCloseHandle::close);
    }

    /// Waits for the worker to terminate; does not request closure.
    ///
    /// Dropping an unfinished wait leaves the task owned here, so it can be
    /// awaited again. After this method returns, it must not be called again.
    pub async fn wait(&mut self) -> Result<(), tokio::task::JoinError> {
        (&mut self.task).await
    }
}

impl Drop for DatabaseWorker {
    fn drop(&mut self) {
        // Best effort only: dropping cannot wait for draining or task completion.
        self.request_close();
    }
}

async fn prepare_app(setup: AppSetup) -> Result<Box<PreparedApp>, AppInitializationError> {
    let mut database = SqliteDatabase::new(&setup.database_path).await?;
    let (ingredient_store, ingredient_rx) = IngredientStoreClient::new(CLIENT_CAPABILITY);
    let worker = DatabaseWorker {
        close_handles: vec![ingredient_rx.close_handle()],
        task: RUNTIME.spawn(async move {
            let inbox = ingredient_rx.into_stream().map(DatabaseEffect::Ingredient);
            database.worker(inbox).await;
        }),
    };

    let app = AppWidget::new(
        kreatyna::AppCapabilities {
            ingredients: kreatyna::ingredients::capabilities::IngredientCapabilities {
                render: setup.render,
                store: Arc::new(ingredient_store),
            },
        },
        setup.layout,
    );

    Ok(Box::new(PreparedApp {
        app,
        _database_worker: worker,
    }))
}

#[cfg(test)]
mod tests;
