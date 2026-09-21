use futures::{
    StreamExt,
    stream::{BoxStream, select_all},
};
use krucyfiks::{Command, CommandExt, Message, Widget, effects::EffectTree};

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

/// Exact branch navigation does not opt into recursive capability resolution.
///
/// ```compile_fail,E0599
/// use mausritter::{capabilities::ResolveRender, composability::CounterPairEffect};
/// async fn resolve(effect: CounterPairEffect) {
///     effect.resolve_render(|| async {}).await.unwrap();
/// }
/// ```
///
/// ```compile_fail,E0599
/// use mausritter::{capabilities::ResolveRender, composability::CounterPairEffect};
/// async fn resolve(effect: CounterPairEffect) {
///     effect.expect_left().resolve_render(|| async {}).await.unwrap();
/// }
/// ```
#[derive(Debug)]
pub enum CounterPairEffect {
    Left(CounterEffect),
    Right(CounterEffect),
}

impl EffectTree for CounterPairEffect {
    fn extract<T: 'static>(self) -> Result<T, Self> {
        match self {
            CounterPairEffect::Left(eff) => eff.extract::<T>().map_err(Self::Left),
            CounterPairEffect::Right(eff) => eff.extract::<T>().map_err(Self::Right),
        }
    }
}
impl CounterPairEffect {
    pub fn try_into_left(self) -> Result<CounterEffect, Self> {
        match self {
            CounterPairEffect::Left(eff) => Ok(eff),
            _ => Err(self),
        }
    }
    #[track_caller]
    pub fn expect_left(self) -> CounterEffect {
        match self {
            Self::Left(effect) => effect,
            Self::Right(_) => {
                panic!("expected CounterPairEffect::Left, got CounterPairEffect::Right")
            }
        }
    }

    pub fn try_into_right(self) -> Result<CounterEffect, Self> {
        match self {
            CounterPairEffect::Right(eff) => Ok(eff),
            _ => Err(self),
        }
    }
    #[track_caller]
    pub fn expect_right(self) -> CounterEffect {
        match self {
            Self::Right(effect) => effect,
            Self::Left(_) => {
                panic!("expected CounterPairEffect::Right, got CounterPairEffect::Left")
            }
        }
    }
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
    use futures::executor::block_on;
    use krucyfiks::effects::CommandTestExt;

    use crate::capabilities::{ResolveDatabase, ResolveRender};

    use super::*;

    #[test]
    fn test_composed_widgets() -> anyhow::Result<()> {
        block_on(async {
            let (mut counter, mut effects) = CounterPair::test_fixture();
            assert_eq!(counter.view(), (0, 0));

            let mut command = counter
                .update(Message::Event(CounterPairEvent::Left(
                    CounterEvent::Increase,
                )))
                .driver(&mut effects);

            assert_eq!(counter.view(), (1, 0));

            command
                .drive()
                .await?
                .expect_effect()
                .expect_left()
                .expect_render()
                .resolve_render(|| async {})
                .await?;

            command
                .drive()
                .await?
                .expect_effect()
                .any_route()
                .resolve_set_count(|count| async move {
                    assert_eq!(count, 1);
                    Ok(())
                })
                .await?;

            command.drive().await?.expect_done();

            Ok(())
        })
    }
}
