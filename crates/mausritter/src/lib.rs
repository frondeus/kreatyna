pub use crate::capabilities::CounterCapabilities;
use krucyfiks::{Command, CommandExt, CommandId, CommandSlot, Message, Widget};

pub mod capabilities;

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
            }
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

#[cfg(test)]
mod tests {
    use std::assert_matches;

    use crate::capabilities::{CounterEffect, DatabaseRequest, RenderRequest, RenderResponse};

    use super::*;
    use futures::executor::block_on;
    use krucyfiks::effects::{CommandEffect, CommandTestExt};

    #[test]
    fn increase_renders_then_persists_the_new_count() {
        block_on(async {
            let (caps, effects) = CounterCapabilities::testing();
            let mut counter = Counter::new(caps);

            assert_eq!(counter.view(), 0);

            let command = counter.update(Message::Event(CounterEvent::Increase));
            let mut driver = command.driver(effects);

            assert_eq!(counter.view(), 1);

            let effect = driver.process_effect().await.unwrap();
            let envelope = match effect {
                CommandEffect::Effect(CounterEffect::Render(envelope)) => envelope,
                _ => panic!("Expected render effect"),
            };
            assert_matches!(envelope.request, RenderRequest::Render);
            envelope.answer_async(RenderResponse::Render).await.unwrap();

            let effect = driver.process_effect().await.unwrap();
            let envelope = match effect {
                CommandEffect::Effect(CounterEffect::Database(envelope)) => envelope,
                _ => panic!("Expected database effect"),
            };
            assert_matches!(envelope.request, DatabaseRequest::SetCount { count: 1 });
            envelope
                .answer_async(capabilities::DatabaseResponse::SetCount(Ok(())))
                .await
                .unwrap();

            let effect = driver.process_effect().await.unwrap();
            assert_matches!(effect, CommandEffect::Done);
        })
    }
}
