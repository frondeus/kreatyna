use std::sync::Arc;

use crate::{
    capabilities::{Clock, Render},
    widget::{Command, CommandExt, CommandId, CommandSlot, Message, Widget},
};

pub mod runtime;
pub mod widget;

pub mod capabilities {
    #[async_trait::async_trait]
    pub trait Render: Send + Sync {
        async fn render(&self);
    }

    #[async_trait::async_trait]
    pub trait Clock: Send + Sync {
        async fn sleep_ms(&self, ms: u64);
    }
}

pub struct Counter {
    counter: usize,
    render: Arc<dyn Render>,
    clock: Arc<dyn Clock>,

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
}

impl Counter {
    pub fn new(render: Arc<dyn Render>, clock: Arc<dyn Clock>) -> Self {
        Self {
            render,
            clock,
            counter: 0,

            background: CommandSlot::new(),
        }
    }
}

impl Widget for Counter {
    type View = usize;

    type Event = CounterEvent;

    type Outcome = CounterOutcome;

    fn update(&mut self, event: Message<Self::Event, Self::Outcome>) -> Command<Self::Outcome> {
        let render = self.render.clone();
        match event {
            Message::Event(CounterEvent::Increase) => {
                self.counter = self.counter.saturating_add(1);
                Command::without_outcome(async move {
                    render.render().await;
                })
            }
            Message::Event(CounterEvent::Decrease) => {
                self.counter = self.counter.saturating_sub(1);
                Command::without_outcome(async move {
                    render.render().await;
                })
            }
            Message::Event(CounterEvent::StartBackground) => {
                let clock = self.clock.clone();
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
            Message::Outcome(CounterOutcome::Tick { id }) => {
                if !self.background.accepts(id) {
                    return Command::empty();
                }

                self.counter = self.counter.saturating_add(1);

                let render = self.render.clone();
                Command::without_outcome(async move {
                    render.render().await;
                })
            }
        }
    }

    fn view(&self) -> Self::View {
        self.counter
    }
}
