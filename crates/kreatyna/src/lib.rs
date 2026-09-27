pub mod app;
pub use crate::capabilities::CounterCapabilities;
use crate::capabilities::CounterEffectStream;
pub use app::{
    AppCapabilities, AppEffect, AppEffectStream, AppEvent, AppOutcome, AppTab, AppView, AppWidget,
};
use krucyfiks::{Command, CommandExt, CommandId, CommandSlot, Message, Widget};

pub mod capabilities;

pub mod composability;
pub mod confirmation;
pub mod ingredients;

/// Temporary example widget used to exercise navigation between child widgets.
/// Remove it when the recipe catalog provides the second production screen.
/// Its persistence is illustrative: concurrent commands can submit captured
/// counts out of event order (P07 in docs/epics/ingredient-catalog/R13-ffi-problems.md).
/// That Counter-only fix is deferred; do not copy this save pattern into production
/// widgets without an explicit policy for overlapping writes.
pub struct Counter {
    counter: usize,
    status: CounterStatus,
    caps: CounterCapabilities,

    background: CommandSlot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CounterStatus {
    Loading,
    Ready,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CounterView {
    pub count: usize,
    pub status: CounterStatus,
}

impl PartialEq<usize> for CounterView {
    fn eq(&self, other: &usize) -> bool {
        self.count == *other
    }
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
            status: CounterStatus::Loading,

            background: CommandSlot::new(),
        }
    }

    pub fn status(&self) -> CounterStatus {
        self.status
    }
}

impl Widget for Counter {
    type View = CounterView;

    type Event = CounterEvent;

    type Outcome = CounterOutcome;

    #[tracing::instrument(skip_all, fields(before = self.counter))]
    fn init(&mut self) -> Command<Self::Outcome> {
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
                self.status = CounterStatus::Ready;
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
        CounterView {
            count: self.counter,
            status: self.status,
        }
    }
}

impl Counter {
    pub fn test_fixture() -> (Self, CounterEffectStream) {
        let (caps, effects) = CounterCapabilities::testing();
        let this = Counter::new(caps);
        (this, effects)
    }
}

#[cfg(test)]
mod tests {

    use crate::capabilities::{ResolveDatabase, ResolveRender};

    use super::*;
    use futures::executor::block_on;
    use krucyfiks::effects::CommandTestExt;

    #[test]
    fn increase_renders_then_persists_the_new_count() -> anyhow::Result<()> {
        block_on(async {
            let (mut counter, mut effects) = Counter::test_fixture();

            assert_eq!(counter.view(), 0);

            let mut command = counter
                .update(Message::Event(CounterEvent::Increase))
                .driver(&mut effects);

            assert_eq!(counter.view(), 1);

            command
                .drive()
                .await?
                .expect_effect()
                .expect_render()
                .resolve_render(|| async {})
                .await?;

            command
                .drive()
                .await?
                .expect_effect()
                .expect_database()
                .resolve_set_count(|count: usize| async move {
                    assert_eq!(count, 1);
                    Ok(())
                })
                .await?;

            command.drive().await?.expect_done();

            Ok(())
        })
    }
}
