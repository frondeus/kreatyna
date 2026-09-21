use futures::{
    StreamExt,
    stream::{BoxStream, select_all},
};
use krucyfiks::{Command, CommandExt, Message, Widget};

use crate::{
    Counter, CounterCapabilities, CounterEvent, CounterOutcome, capabilities::CounterEffect,
};

pub struct CounterPair {
    left: Counter,
    right: Counter,
}

#[derive(Debug)]
pub enum CounterPairEvent {
    Left(CounterEvent),
    Right(CounterEvent),
}

#[derive(Debug)]
pub enum CounterPairOutcome {
    Left(CounterOutcome),
    Right(CounterOutcome),
}

#[derive(Debug)]
pub enum CounterPairEffect {
    Left(CounterEffect),
    Right(CounterEffect),
}

impl CounterPair {
    pub fn new(left: CounterCapabilities, right: CounterCapabilities) -> Self {
        Self {
            left: Counter::new(left),
            right: Counter::new(right),
        }
    }
}

impl Widget for CounterPair {
    type View = (usize, usize);

    type Event = CounterPairEvent;

    type Outcome = CounterPairOutcome;

    fn update(&mut self, message: Message<Self>) -> Command<Self::Outcome> {
        match message {
            Message::Event(CounterPairEvent::Left(event)) => self
                .left
                .update(Message::Event(event))
                .map(CounterPairOutcome::Left),
            Message::Event(CounterPairEvent::Right(event)) => self
                .right
                .update(Message::Event(event))
                .map(CounterPairOutcome::Right),
            Message::Outcome(CounterPairOutcome::Left(out)) => self
                .left
                .update(Message::Outcome(out))
                .map(CounterPairOutcome::Left),
            Message::Outcome(CounterPairOutcome::Right(out)) => self
                .right
                .update(Message::Outcome(out))
                .map(CounterPairOutcome::Right),
        }
    }

    fn view(&self) -> Self::View {
        (self.left.view(), self.right.view())
    }

    fn init(&self) -> Command<Self::Outcome> {
        let left_cmd = self.left.init();
        let right_cmd = self.right.init();

        Command::select_all(vec![
            left_cmd.map(CounterPairOutcome::Left),
            right_cmd.map(CounterPairOutcome::Right),
        ])
    }
}

pub type CounterPairEffectStream = BoxStream<'static, CounterPairEffect>;
impl CounterPair {
    pub fn test_fixture() -> (Self, CounterPairEffectStream) {
        let (left_caps, left_effects) = CounterCapabilities::testing();
        let (right_caps, right_effects) = CounterCapabilities::testing();
        let effects: Vec<_> = vec![
            left_effects.map(CounterPairEffect::Left).boxed(),
            right_effects.map(CounterPairEffect::Right).boxed(),
        ];
        let effects = select_all(effects).boxed();

        let this = Self::new(left_caps, right_caps);
        (this, effects)
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches;

    use futures::executor::block_on;
    use krucyfiks::effects::{CommandEffect, CommandTestExt};

    use crate::capabilities::{HandleDatabase, HandleRender};

    use super::*;

    #[test]
    fn test_composed_widgets() {
        block_on(async {
            let (mut counter, mut effects) = CounterPair::test_fixture();
            assert_eq!(counter.view(), (0, 0));

            let mut command = counter
                .update(Message::Event(CounterPairEvent::Left(
                    CounterEvent::Increase,
                )))
                .driver(&mut effects);

            assert_eq!(counter.view(), (1, 0));

            let effect = command.process_effect().await.unwrap();
            let envelope = match effect {
                CommandEffect::Effect(CounterPairEffect::Left(CounterEffect::Render(envelope))) => {
                    envelope
                }
                _ => panic!("Expected render effect"),
            };
            envelope.handle_render(|| async {}).await.unwrap();

            let effect = command.process_effect().await.unwrap();
            let envelope = match effect {
                CommandEffect::Effect(CounterPairEffect::Left(CounterEffect::Database(
                    envelope,
                ))) => envelope,
                _ => panic!("Expected database effect"),
            };
            envelope
                .handle_set_count(|count| async move {
                    assert_eq!(count, 1);
                    Ok(())
                })
                .await
                .unwrap();

            let effect = command.process_effect().await.unwrap();
            assert_matches!(effect, CommandEffect::Done);
        })
    }
}
