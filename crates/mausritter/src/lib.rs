use std::sync::Arc;

use crate::capabilities::{Clock, Database, Render};
use krucyfiks::{Command, CommandExt, CommandId, CommandSlot, Message, Widget};

pub mod capabilities;

pub struct CounterCapabilities {
    pub render: Arc<dyn Render>,
    pub clock: Arc<dyn Clock>,
    pub db: Arc<dyn Database>,
}

pub struct Counter {
    counter: usize,
    caps: CounterCapabilities,

    background: CommandSlot,
}

#[derive(Debug)]
pub enum CounterEvent {
    Increase,
    Decrease,
    Reset,
    StartBackground,
    StopBackground,
}

#[derive(Debug)]
pub enum CounterOutcome {
    Tick { id: CommandId },
    Loaded { counter: usize },
}

impl Counter {
    pub fn new(caps: CounterCapabilities) -> Self {
        Self {
            caps,
            counter: 0,

            background: CommandSlot::new(),
        }
    }
}

impl Widget for Counter {
    type View = usize;

    type Event = CounterEvent;

    type Outcome = CounterOutcome;

    #[tracing::instrument(skip_all, fields(before = self.counter))]
    fn init(&self) -> Command<Self::Outcome> {
        let db = self.caps.db.clone();
        tracing::info!("Init");
        Command::single(async move {
            tracing::info!("Init command");

            let counter = match db.count().await {
                Ok(Some(counter)) => counter,
                Ok(None) => {
                    return CounterOutcome::Loaded { counter: 0 };
                }
                Err(error) => {
                    tracing::error!(?error, "Could not load initial counter");
                    return CounterOutcome::Loaded { counter: 0 };
                }
            };
            tracing::info!("Loaded: {counter}");
            CounterOutcome::Loaded { counter }
        })
    }

    #[tracing::instrument(skip_all, fields(before = self.counter))]
    fn update(&mut self, event: Message<Self>) -> Command<Self::Outcome> {
        let render = self.caps.render.clone();
        let db = self.caps.db.clone();
        tracing::info!("Update");
        match event {
            Message::Event(CounterEvent::Reset) => {
                self.counter = 0;
                let counter = self.counter;
                Command::without_outcome(async move {
                    render.render().await;
                    if let Err(error) = db.set_count(counter).await {
                        tracing::error!(?error, "Could not set the counter in DB");
                    }
                })
            },
            Message::Event(CounterEvent::Increase) => {
                self.counter = self.counter.saturating_add(1);
                let counter = self.counter;
                Command::without_outcome(async move {
                    render.render().await;
                    if let Err(error) = db.set_count(counter).await {
                        tracing::error!(?error, "Could not set the counter in DB");
                    }
                })
            }
            Message::Event(CounterEvent::Decrease) => {
                self.counter = self.counter.saturating_sub(1);
                let counter = self.counter;
                Command::without_outcome(async move {
                    render.render().await;
                    if let Err(error) = db.set_count(counter).await {
                        tracing::error!(?error, "Could not set the counter in DB");
                    }
                })
            }
            Message::Event(CounterEvent::StartBackground) => {
                let clock = self.caps.clock.clone();
                self.background.start(move |id| {
                    async_stream::stream!({
                        tracing::info!("Started task");
                        loop {
                            clock.sleep_ms(1_000).await;
                            tracing::info!("Waited 1000ms");
                            yield CounterOutcome::Tick { id };
                        }
                    })
                })
            }
            Message::Event(CounterEvent::StopBackground) => {
                self.background.stop();
                Command::empty()
            }
            Message::Outcome(CounterOutcome::Loaded { counter }) => {
                self.counter = counter;
                Command::without_outcome(async move {
                    render.render().await;
                })
            }
            Message::Outcome(CounterOutcome::Tick { id }) => {
                if !self.background.accepts(id) {
                    return Command::empty();
                }

                self.counter = self.counter.saturating_add(1);
                let counter = self.counter;

                Command::without_outcome(async move {
                    render.render().await;
                    if let Err(error) = db.set_count(counter).await {
                        tracing::error!(?error, "Could not set the counter in DB");
                    }
                })
            }
        }
    }

    fn view(&self) -> Self::View {
        self.counter
    }
}
