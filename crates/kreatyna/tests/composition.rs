use futures::{StreamExt, stream::select_all};
use kreatyna::capabilities::{DatabaseError, ResolveClock, ResolveCounterDatabase, ResolveRender};
use kreatyna::composability::{
    CounterPair, CounterPairEffect, CounterPairEffectStream, CounterPairEvent, CounterPairOutcome,
};
use kreatyna::{CounterCapabilities, CounterEvent, CounterOutcome};
use krucyfiks::{Message, Widget, effects::CommandTestExt};
use test_case::test_case;

// Ordinary behavior

#[test_case(CounterEvent::Increase, 11, (11, 20); "increase")]
#[test_case(CounterEvent::Decrease, 9, (9, 20); "decrease")]
#[test_case(CounterEvent::Reset, 0, (0, 20); "reset")]
#[tokio::test]
async fn a_left_event_changes_only_the_left_count_and_routes_its_effects(
    event: CounterEvent,
    expected_count: usize,
    expected_view: (usize, usize),
) -> anyhow::Result<()> {
    let PairFixture {
        mut pair,
        mut effects,
        ..
    } = PairFixture::ready(10, 20).await?;
    let mut command = pair
        .update(Message::Event(CounterPairEvent::Left(event)))
        .driver(&mut effects);
    assert_eq!(pair.view(), expected_view);

    command
        .step()
        .await?
        .expect_effect()
        .expect_left()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command
        .step()
        .await?
        .expect_effect()
        .expect_left()
        .expect_database()
        .resolve_set_count(move |count| async move {
            assert_eq!(count, expected_count);
            Ok(())
        })
        .await?;
    command.step().await?.expect_done();

    Ok(())
}

#[test_case(CounterEvent::Increase, 21, (10, 21); "increase")]
#[test_case(CounterEvent::Decrease, 19, (10, 19); "decrease")]
#[test_case(CounterEvent::Reset, 0, (10, 0); "reset")]
#[tokio::test]
async fn a_right_event_changes_only_the_right_count_and_routes_its_effects(
    event: CounterEvent,
    expected_count: usize,
    expected_view: (usize, usize),
) -> anyhow::Result<()> {
    let PairFixture {
        mut pair,
        mut effects,
        ..
    } = PairFixture::ready(10, 20).await?;
    let mut command = pair
        .update(Message::Event(CounterPairEvent::Right(event)))
        .driver(&mut effects);
    assert_eq!(pair.view(), expected_view);

    command
        .step()
        .await?
        .expect_effect()
        .expect_right()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command
        .step()
        .await?
        .expect_effect()
        .expect_right()
        .expect_database()
        .resolve_set_count(move |count| async move {
            assert_eq!(count, expected_count);
            Ok(())
        })
        .await?;
    command.step().await?.expect_done();

    Ok(())
}

#[test_case(None, 0; "missing left row")]
#[test_case(Some(0), 0; "stored left zero")]
#[test_case(Some(17), 17; "stored left count")]
#[test_case(Some(usize::MAX), usize::MAX; "stored left maximum")]
#[tokio::test]
async fn initialization_routes_both_loaded_counts_when_the_left_read_finishes_first(
    stored: Option<usize>,
    expected: usize,
) -> anyhow::Result<()> {
    let (mut pair, mut effects) = CounterPair::test_fixture();
    let mut initialization = pair.init().driver(&mut effects);
    assert_eq!(pair.view(), (0, 0));

    // Both children must request their reads before either receives a response.
    let first = initialization.step().await?.expect_effect();
    let second = initialization.step().await?.expect_effect();
    let (left_read, right_read) = match (first, second) {
        (CounterPairEffect::Left(left), CounterPairEffect::Right(right))
        | (CounterPairEffect::Right(right), CounterPairEffect::Left(left)) => {
            (left.expect_database(), right.expect_database())
        }
        other => panic!("expected one database read per child, got {other:?}"),
    };

    left_read
        .resolve_count(|| async move { Ok(stored) })
        .await?;
    let first_outcome = initialization.step().await?.expect_outcome();
    assert_eq!(
        CounterPairOutcome::Left(CounterOutcome::Loaded { counter: expected }),
        first_outcome
    );

    right_read
        .resolve_count(|| async move { Ok(Some(usize::MAX)) })
        .await?;
    let second_outcome = initialization.step().await?.expect_outcome();
    assert_eq!(
        CounterPairOutcome::Right(CounterOutcome::Loaded {
            counter: usize::MAX
        }),
        second_outcome
    );
    initialization.step().await?.expect_done();
    drop(initialization);

    let mut command = pair
        .update(Message::Outcome(first_outcome))
        .driver(&mut effects);
    assert_eq!(pair.view(), (expected, 0));

    command
        .step()
        .await?
        .expect_effect()
        .expect_left()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command.step().await?.expect_done();
    drop(command);

    let mut command = pair
        .update(Message::Outcome(second_outcome))
        .driver(&mut effects);
    assert_eq!(pair.view(), (expected, usize::MAX));

    command
        .step()
        .await?
        .expect_effect()
        .expect_right()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command.step().await?.expect_done();

    Ok(())
}

#[test_case(None, 0; "missing left row")]
#[test_case(Some(0), 0; "stored left zero")]
#[test_case(Some(17), 17; "stored left count")]
#[test_case(Some(usize::MAX), usize::MAX; "stored left maximum")]
#[tokio::test]
async fn initialization_routes_both_loaded_counts_when_the_right_read_finishes_first(
    stored: Option<usize>,
    expected: usize,
) -> anyhow::Result<()> {
    let (mut pair, mut effects) = CounterPair::test_fixture();
    let mut initialization = pair.init().driver(&mut effects);
    assert_eq!(pair.view(), (0, 0));

    // Both children must request their reads before either receives a response.
    let first = initialization.step().await?.expect_effect();
    let second = initialization.step().await?.expect_effect();
    let (left_read, right_read) = match (first, second) {
        (CounterPairEffect::Left(left), CounterPairEffect::Right(right))
        | (CounterPairEffect::Right(right), CounterPairEffect::Left(left)) => {
            (left.expect_database(), right.expect_database())
        }
        other => panic!("expected one database read per child, got {other:?}"),
    };

    right_read
        .resolve_count(|| async move { Ok(Some(usize::MAX)) })
        .await?;
    let first_outcome = initialization.step().await?.expect_outcome();
    assert_eq!(
        CounterPairOutcome::Right(CounterOutcome::Loaded {
            counter: usize::MAX
        }),
        first_outcome
    );

    left_read
        .resolve_count(|| async move { Ok(stored) })
        .await?;
    let second_outcome = initialization.step().await?.expect_outcome();
    assert_eq!(
        CounterPairOutcome::Left(CounterOutcome::Loaded { counter: expected }),
        second_outcome
    );
    initialization.step().await?.expect_done();
    drop(initialization);

    let mut command = pair
        .update(Message::Outcome(first_outcome))
        .driver(&mut effects);
    assert_eq!(pair.view(), (0, usize::MAX));

    command
        .step()
        .await?
        .expect_effect()
        .expect_right()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command.step().await?.expect_done();
    drop(command);

    let mut command = pair
        .update(Message::Outcome(second_outcome))
        .driver(&mut effects);
    assert_eq!(pair.view(), (expected, usize::MAX));

    command
        .step()
        .await?
        .expect_effect()
        .expect_left()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command.step().await?.expect_done();

    Ok(())
}

#[tokio::test]
async fn a_left_tick_changes_only_its_child_and_routes_render_and_write() -> anyhow::Result<()> {
    let PairFixture {
        mut pair,
        mut effects,
        mut left_clock_effects,
        ..
    } = PairFixture::ready(10, 20).await?;
    let mut left = pair
        .update(Message::Event(CounterPairEvent::Left(
            CounterEvent::StartBackground,
        )))
        .driver(&mut left_clock_effects);
    assert_eq!(pair.view(), (10, 20));

    left.step()
        .await?
        .expect_effect()
        .expect_left()
        .expect_clock()
        .resolve_sleep_ms(|ms| async move {
            assert_eq!(ms, 1_000);
        })
        .await?;
    let CounterPairOutcome::Left(CounterOutcome::Tick { id: left_id }) =
        left.step().await?.expect_outcome()
    else {
        panic!("expected a left tick");
    };

    let mut command = pair
        .update(Message::Outcome(CounterPairOutcome::Left(
            CounterOutcome::Tick { id: left_id },
        )))
        .driver(&mut effects);
    assert_eq!(pair.view(), (11, 20));

    command
        .step()
        .await?
        .expect_effect()
        .expect_left()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command
        .step()
        .await?
        .expect_effect()
        .expect_left()
        .expect_database()
        .resolve_set_count(move |count| async move {
            assert_eq!(count, 11);
            Ok(())
        })
        .await?;
    command.step().await?.expect_done();

    Ok(())
}

#[tokio::test]
async fn a_right_tick_changes_only_its_child_and_routes_render_and_write() -> anyhow::Result<()> {
    let PairFixture {
        mut pair,
        mut effects,
        mut right_clock_effects,
        ..
    } = PairFixture::ready(10, 20).await?;
    let mut right = pair
        .update(Message::Event(CounterPairEvent::Right(
            CounterEvent::StartBackground,
        )))
        .driver(&mut right_clock_effects);
    assert_eq!(pair.view(), (10, 20));

    right
        .step()
        .await?
        .expect_effect()
        .expect_right()
        .expect_clock()
        .resolve_sleep_ms(|ms| async move {
            assert_eq!(ms, 1_000);
        })
        .await?;
    let CounterPairOutcome::Right(CounterOutcome::Tick { id: right_id }) =
        right.step().await?.expect_outcome()
    else {
        panic!("expected a right tick");
    };

    let mut command = pair
        .update(Message::Outcome(CounterPairOutcome::Right(
            CounterOutcome::Tick { id: right_id },
        )))
        .driver(&mut effects);
    assert_eq!(pair.view(), (10, 21));

    command
        .step()
        .await?
        .expect_effect()
        .expect_right()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command
        .step()
        .await?
        .expect_effect()
        .expect_right()
        .expect_database()
        .resolve_set_count(move |count| async move {
            assert_eq!(count, 21);
            Ok(())
        })
        .await?;
    command.step().await?.expect_done();

    Ok(())
}

// Edge cases and guards

#[tokio::test]
async fn ticks_from_the_siblings_command_are_ignored() -> anyhow::Result<()> {
    let PairFixture {
        mut pair,
        mut effects,
        mut left_clock_effects,
        mut right_clock_effects,
    } = PairFixture::ready(10, 20).await?;
    let mut left = pair
        .update(Message::Event(CounterPairEvent::Left(
            CounterEvent::StartBackground,
        )))
        .driver(&mut left_clock_effects);
    assert_eq!(pair.view(), (10, 20));

    let mut right = pair
        .update(Message::Event(CounterPairEvent::Right(
            CounterEvent::StartBackground,
        )))
        .driver(&mut right_clock_effects);
    assert_eq!(pair.view(), (10, 20));

    left.step()
        .await?
        .expect_effect()
        .expect_left()
        .expect_clock()
        .resolve_sleep_ms(|ms| async move {
            assert_eq!(ms, 1_000);
        })
        .await?;
    let CounterPairOutcome::Left(CounterOutcome::Tick { id: left_id }) =
        left.step().await?.expect_outcome()
    else {
        panic!("expected a left tick");
    };

    right
        .step()
        .await?
        .expect_effect()
        .expect_right()
        .expect_clock()
        .resolve_sleep_ms(|ms| async move {
            assert_eq!(ms, 1_000);
        })
        .await?;
    let CounterPairOutcome::Right(CounterOutcome::Tick { id: right_id }) =
        right.step().await?.expect_outcome()
    else {
        panic!("expected a right tick");
    };
    assert_ne!(left_id, right_id);

    let command = pair.update(Message::Outcome(CounterPairOutcome::Left(
        CounterOutcome::Tick { id: right_id },
    )));
    assert_eq!(pair.view(), (10, 20));
    command.driver(&mut effects).step().await?.expect_done();

    let command = pair.update(Message::Outcome(CounterPairOutcome::Right(
        CounterOutcome::Tick { id: left_id },
    )));
    assert_eq!(pair.view(), (10, 20));
    command.driver(&mut effects).step().await?.expect_done();

    Ok(())
}

#[tokio::test]
async fn stopping_the_left_background_preserves_the_right_background() -> anyhow::Result<()> {
    let PairFixture {
        mut pair,
        mut effects,
        mut left_clock_effects,
        mut right_clock_effects,
    } = PairFixture::ready(10, 20).await?;
    let mut left = pair
        .update(Message::Event(CounterPairEvent::Left(
            CounterEvent::StartBackground,
        )))
        .driver(&mut left_clock_effects);
    assert_eq!(pair.view(), (10, 20));

    let mut right = pair
        .update(Message::Event(CounterPairEvent::Right(
            CounterEvent::StartBackground,
        )))
        .driver(&mut right_clock_effects);
    assert_eq!(pair.view(), (10, 20));

    left.step()
        .await?
        .expect_effect()
        .expect_left()
        .expect_clock()
        .resolve_sleep_ms(|ms| async move {
            assert_eq!(ms, 1_000);
        })
        .await?;
    let CounterPairOutcome::Left(CounterOutcome::Tick { id: left_id }) =
        left.step().await?.expect_outcome()
    else {
        panic!("expected a left tick");
    };

    right
        .step()
        .await?
        .expect_effect()
        .expect_right()
        .expect_clock()
        .resolve_sleep_ms(|ms| async move {
            assert_eq!(ms, 1_000);
        })
        .await?;
    let CounterPairOutcome::Right(CounterOutcome::Tick { id: right_id }) =
        right.step().await?.expect_outcome()
    else {
        panic!("expected a right tick");
    };

    let command = pair.update(Message::Event(CounterPairEvent::Left(
        CounterEvent::StopBackground,
    )));
    assert_eq!(pair.view(), (10, 20));
    command.driver(&mut effects).step().await?.expect_done();
    left.step().await?.expect_done();

    let command = pair.update(Message::Outcome(CounterPairOutcome::Left(
        CounterOutcome::Tick { id: left_id },
    )));
    assert_eq!(pair.view(), (10, 20));
    command.driver(&mut effects).step().await?.expect_done();

    right
        .step()
        .await?
        .expect_effect()
        .expect_right()
        .expect_clock()
        .resolve_sleep_ms(|ms| async move {
            assert_eq!(ms, 1_000);
        })
        .await?;
    let outcome = right.step().await?.expect_outcome();
    assert_eq!(
        CounterPairOutcome::Right(CounterOutcome::Tick { id: right_id }),
        outcome
    );

    let mut command = pair.update(Message::Outcome(outcome)).driver(&mut effects);
    assert_eq!(pair.view(), (10, 21));

    command
        .step()
        .await?
        .expect_effect()
        .expect_right()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command
        .step()
        .await?
        .expect_effect()
        .expect_right()
        .expect_database()
        .resolve_set_count(move |count| async move {
            assert_eq!(count, 21);
            Ok(())
        })
        .await?;
    command.step().await?.expect_done();

    Ok(())
}

#[tokio::test]
async fn stopping_the_right_background_preserves_the_left_background() -> anyhow::Result<()> {
    let PairFixture {
        mut pair,
        mut effects,
        mut left_clock_effects,
        mut right_clock_effects,
    } = PairFixture::ready(10, 20).await?;
    let mut left = pair
        .update(Message::Event(CounterPairEvent::Left(
            CounterEvent::StartBackground,
        )))
        .driver(&mut left_clock_effects);
    assert_eq!(pair.view(), (10, 20));

    let mut right = pair
        .update(Message::Event(CounterPairEvent::Right(
            CounterEvent::StartBackground,
        )))
        .driver(&mut right_clock_effects);
    assert_eq!(pair.view(), (10, 20));

    left.step()
        .await?
        .expect_effect()
        .expect_left()
        .expect_clock()
        .resolve_sleep_ms(|ms| async move {
            assert_eq!(ms, 1_000);
        })
        .await?;
    let CounterPairOutcome::Left(CounterOutcome::Tick { id: left_id }) =
        left.step().await?.expect_outcome()
    else {
        panic!("expected a left tick");
    };

    right
        .step()
        .await?
        .expect_effect()
        .expect_right()
        .expect_clock()
        .resolve_sleep_ms(|ms| async move {
            assert_eq!(ms, 1_000);
        })
        .await?;
    let CounterPairOutcome::Right(CounterOutcome::Tick { id: right_id }) =
        right.step().await?.expect_outcome()
    else {
        panic!("expected a right tick");
    };

    let command = pair.update(Message::Event(CounterPairEvent::Right(
        CounterEvent::StopBackground,
    )));
    assert_eq!(pair.view(), (10, 20));
    command.driver(&mut effects).step().await?.expect_done();
    right.step().await?.expect_done();

    let command = pair.update(Message::Outcome(CounterPairOutcome::Right(
        CounterOutcome::Tick { id: right_id },
    )));
    assert_eq!(pair.view(), (10, 20));
    command.driver(&mut effects).step().await?.expect_done();

    left.step()
        .await?
        .expect_effect()
        .expect_left()
        .expect_clock()
        .resolve_sleep_ms(|ms| async move {
            assert_eq!(ms, 1_000);
        })
        .await?;
    let outcome = left.step().await?.expect_outcome();
    assert_eq!(
        CounterPairOutcome::Left(CounterOutcome::Tick { id: left_id }),
        outcome
    );

    let mut command = pair.update(Message::Outcome(outcome)).driver(&mut effects);
    assert_eq!(pair.view(), (11, 20));

    command
        .step()
        .await?
        .expect_effect()
        .expect_left()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command
        .step()
        .await?
        .expect_effect()
        .expect_left()
        .expect_database()
        .resolve_set_count(move |count| async move {
            assert_eq!(count, 11);
            Ok(())
        })
        .await?;
    command.step().await?.expect_done();

    Ok(())
}

// Failures and recovery

#[test_case(Err(DatabaseError::Storage), Ok(Some(20)), (0, 20); "left storage failure")]
#[test_case(Err(DatabaseError::OutOfRange), Ok(Some(20)), (0, 20); "left out of range")]
#[test_case(Err(DatabaseError::Transport), Ok(Some(20)), (0, 20); "left transport failure")]
#[test_case(Ok(Some(10)), Err(DatabaseError::Storage), (10, 0); "right storage failure")]
#[test_case(Ok(Some(10)), Err(DatabaseError::OutOfRange), (10, 0); "right out of range")]
#[test_case(Ok(Some(10)), Err(DatabaseError::Transport), (10, 0); "right transport failure")]
#[tokio::test]
async fn a_failed_child_read_preserves_the_sibling_when_the_left_read_finishes_first(
    left_reply: Result<Option<usize>, DatabaseError>,
    right_reply: Result<Option<usize>, DatabaseError>,
    expected_view: (usize, usize),
) -> anyhow::Result<()> {
    let (mut pair, mut effects) = CounterPair::test_fixture();
    let mut initialization = pair.init().driver(&mut effects);
    assert_eq!(pair.view(), (0, 0));

    // Both children must request their reads before either receives a response.
    let first = initialization.step().await?.expect_effect();
    let second = initialization.step().await?.expect_effect();
    let (left_read, right_read) = match (first, second) {
        (CounterPairEffect::Left(left), CounterPairEffect::Right(right))
        | (CounterPairEffect::Right(right), CounterPairEffect::Left(left)) => {
            (left.expect_database(), right.expect_database())
        }
        other => panic!("expected one database read per child, got {other:?}"),
    };

    left_read
        .resolve_count(|| async move { left_reply })
        .await?;
    let first_outcome = initialization.step().await?.expect_outcome();
    assert_eq!(
        CounterPairOutcome::Left(CounterOutcome::Loaded {
            counter: expected_view.0
        }),
        first_outcome
    );

    right_read
        .resolve_count(|| async move { right_reply })
        .await?;
    let second_outcome = initialization.step().await?.expect_outcome();
    assert_eq!(
        CounterPairOutcome::Right(CounterOutcome::Loaded {
            counter: expected_view.1
        }),
        second_outcome
    );
    initialization.step().await?.expect_done();
    drop(initialization);

    let mut command = pair
        .update(Message::Outcome(first_outcome))
        .driver(&mut effects);
    assert_eq!(pair.view(), (expected_view.0, 0));

    command
        .step()
        .await?
        .expect_effect()
        .expect_left()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command.step().await?.expect_done();
    drop(command);

    let mut command = pair
        .update(Message::Outcome(second_outcome))
        .driver(&mut effects);
    assert_eq!(pair.view(), (expected_view.0, expected_view.1));

    command
        .step()
        .await?
        .expect_effect()
        .expect_right()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command.step().await?.expect_done();

    Ok(())
}

#[test_case(Err(DatabaseError::Storage), Ok(Some(20)), (0, 20); "left storage failure")]
#[test_case(Err(DatabaseError::OutOfRange), Ok(Some(20)), (0, 20); "left out of range")]
#[test_case(Err(DatabaseError::Transport), Ok(Some(20)), (0, 20); "left transport failure")]
#[test_case(Ok(Some(10)), Err(DatabaseError::Storage), (10, 0); "right storage failure")]
#[test_case(Ok(Some(10)), Err(DatabaseError::OutOfRange), (10, 0); "right out of range")]
#[test_case(Ok(Some(10)), Err(DatabaseError::Transport), (10, 0); "right transport failure")]
#[tokio::test]
async fn a_failed_child_read_preserves_the_sibling_when_the_right_read_finishes_first(
    left_reply: Result<Option<usize>, DatabaseError>,
    right_reply: Result<Option<usize>, DatabaseError>,
    expected_view: (usize, usize),
) -> anyhow::Result<()> {
    let (mut pair, mut effects) = CounterPair::test_fixture();
    let mut initialization = pair.init().driver(&mut effects);
    assert_eq!(pair.view(), (0, 0));

    // Both children must request their reads before either receives a response.
    let first = initialization.step().await?.expect_effect();
    let second = initialization.step().await?.expect_effect();
    let (left_read, right_read) = match (first, second) {
        (CounterPairEffect::Left(left), CounterPairEffect::Right(right))
        | (CounterPairEffect::Right(right), CounterPairEffect::Left(left)) => {
            (left.expect_database(), right.expect_database())
        }
        other => panic!("expected one database read per child, got {other:?}"),
    };

    right_read
        .resolve_count(|| async move { right_reply })
        .await?;
    let first_outcome = initialization.step().await?.expect_outcome();
    assert_eq!(
        CounterPairOutcome::Right(CounterOutcome::Loaded {
            counter: expected_view.1
        }),
        first_outcome
    );

    left_read
        .resolve_count(|| async move { left_reply })
        .await?;
    let second_outcome = initialization.step().await?.expect_outcome();
    assert_eq!(
        CounterPairOutcome::Left(CounterOutcome::Loaded {
            counter: expected_view.0
        }),
        second_outcome
    );
    initialization.step().await?.expect_done();
    drop(initialization);

    let mut command = pair
        .update(Message::Outcome(first_outcome))
        .driver(&mut effects);
    assert_eq!(pair.view(), (0, expected_view.1));

    command
        .step()
        .await?
        .expect_effect()
        .expect_right()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command.step().await?.expect_done();
    drop(command);

    let mut command = pair
        .update(Message::Outcome(second_outcome))
        .driver(&mut effects);
    assert_eq!(pair.view(), (expected_view.0, expected_view.1));

    command
        .step()
        .await?
        .expect_effect()
        .expect_left()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command.step().await?.expect_done();

    Ok(())
}

// Test support

struct PairFixture {
    pair: CounterPair,
    effects: CounterPairEffectStream,
    left_clock_effects: CounterPairEffectStream,
    right_clock_effects: CounterPairEffectStream,
}

impl PairFixture {
    async fn ready(left_count: usize, right_count: usize) -> anyhow::Result<Self> {
        // Independent clock streams let both backgrounds run while updates are driven.
        let (mut left_capabilities, left_effects) = CounterCapabilities::testing();
        let (mut right_capabilities, right_effects) = CounterCapabilities::testing();
        let (left_clock_capabilities, left_clock_effects) = CounterCapabilities::testing();
        let (right_clock_capabilities, right_clock_effects) = CounterCapabilities::testing();
        left_capabilities.clock = left_clock_capabilities.clock;
        right_capabilities.clock = right_clock_capabilities.clock;
        let mut effects = select_all(vec![
            left_effects.map(CounterPairEffect::Left).boxed(),
            right_effects.map(CounterPairEffect::Right).boxed(),
        ])
        .boxed();
        let mut pair = CounterPair::new(left_capabilities, right_capabilities);

        let mut setup = pair
            .update(Message::Outcome(CounterPairOutcome::Left(
                CounterOutcome::Loaded {
                    counter: left_count,
                },
            )))
            .driver(&mut effects);
        setup
            .step()
            .await?
            .expect_effect()
            .expect_left()
            .expect_render()
            .resolve_render(|| async {})
            .await?;
        setup.step().await?.expect_done();
        drop(setup);

        let mut setup = pair
            .update(Message::Outcome(CounterPairOutcome::Right(
                CounterOutcome::Loaded {
                    counter: right_count,
                },
            )))
            .driver(&mut effects);
        setup
            .step()
            .await?
            .expect_effect()
            .expect_right()
            .expect_render()
            .resolve_render(|| async {})
            .await?;
        setup.step().await?.expect_done();
        drop(setup);

        Ok(Self {
            pair,
            effects,
            left_clock_effects: left_clock_effects.map(CounterPairEffect::Left).boxed(),
            right_clock_effects: right_clock_effects.map(CounterPairEffect::Right).boxed(),
        })
    }
}
