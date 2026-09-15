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

pub enum CounterEvent {
    Increase,
    Decrease,
    StartBackground,
    StopBackground,
}

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

    fn init(&self) -> Command<Self::Outcome> {
        let db = self.caps.db.clone();
        Command::single(async move {
            let Ok(Some(counter)) = db.count().await else {
                return CounterOutcome::Loaded { counter: 0 };
            };
            CounterOutcome::Loaded { counter }
        })
    }

    fn update(&mut self, event: Message<Self>) -> Command<Self::Outcome> {
        let render = self.caps.render.clone();
        let db = self.caps.db.clone();
        match event {
            Message::Event(CounterEvent::Increase) => {
                self.counter = self.counter.saturating_add(1);
                let counter = self.counter;
                Command::without_outcome(async move {
                    render.render().await;
                    _ = db.set_count(counter).await;
                })
            }
            Message::Event(CounterEvent::Decrease) => {
                self.counter = self.counter.saturating_sub(1);
                let counter = self.counter;
                Command::without_outcome(async move {
                    render.render().await;
                    _ = db.set_count(counter).await;
                })
            }
            Message::Event(CounterEvent::StartBackground) => {
                let clock = self.caps.clock.clone();
                self.background.start(move |id| {
                    async_stream::stream!({
                        loop {
                            clock.sleep_ms(1_000).await;
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
                    _ = db.set_count(counter).await;
                })
            }
        }
    }

    fn view(&self) -> Self::View {
        self.counter
    }
}
