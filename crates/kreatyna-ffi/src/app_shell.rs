use std::{fmt, sync::Arc};

use kreatyna::{
    AppWidget,
    capabilities::{Clock, DatabaseClient, Render},
    ingredients::capabilities::IngredientStoreClient,
};
use krucyfiks::{Command, CommandExt, Message, Widget};
use tokio::task::JoinHandle;

use crate::{
    AppInitializationError, AppTypeConversionError, AppView, RUNTIME, RunningAppView,
    database::sqlite::SqliteDatabase,
};

#[derive(Clone)]
pub(crate) struct AppSetup {
    pub(crate) database_path: String,
    pub(crate) render: Arc<dyn Render>,
    pub(crate) clock: Arc<dyn Clock>,
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

struct DatabaseWorker(JoinHandle<()>);

impl Drop for DatabaseWorker {
    fn drop(&mut self) {
        self.0.abort();
    }
}

async fn prepare_app(setup: AppSetup) -> Result<Box<PreparedApp>, AppInitializationError> {
    let mut database = SqliteDatabase::new(&setup.database_path).await?;
    let (database_client, database_rx) = DatabaseClient::new(32, 0);
    let (ingredient_store, ingredient_rx) = IngredientStoreClient::new(32, 0);
    let worker = DatabaseWorker(RUNTIME.spawn(async move {
        database.worker(database_rx, ingredient_rx).await;
    }));

    let app = AppWidget::new(kreatyna::AppCapabilities {
        ingredients: kreatyna::ingredients::capabilities::IngredientCapabilities {
            render: setup.render.clone(),
            store: Arc::new(ingredient_store),
        },
        counter: kreatyna::CounterCapabilities {
            render: setup.render.clone(),
            clock: setup.clock,
            db: Arc::new(database_client),
        },
        render: setup.render,
    });

    Ok(Box::new(PreparedApp {
        app,
        _database_worker: worker,
    }))
}
