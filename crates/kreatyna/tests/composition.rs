use std::sync::Arc;

use futures::{StreamExt, stream::select_all};
use kreatyna::capabilities::{
    ClockClient, DatabaseError, ResolveClock, ResolveCounterDatabase, ResolveRender,
};
use kreatyna::composability::{
    CounterPair, CounterPairEffect, CounterPairEvent, CounterPairOutcome,
};
use kreatyna::{CLIENT_CAPABILITY, CounterCapabilities, CounterEvent, CounterOutcome};
use krucyfiks::{Message, Widget, effects::CommandTestExt};
use test_case::{test_case, test_matrix};

#[test_case(CounterPairEvent::Left(CounterEvent::Increase), true, 11, (11, 20); "left increase")]
#[test_case(CounterPairEvent::Left(CounterEvent::Decrease), true, 9, (9, 20); "left decrease")]
#[test_case(CounterPairEvent::Left(CounterEvent::Reset), true, 0, (0, 20); "left reset")]
#[test_case(CounterPairEvent::Right(CounterEvent::Increase), false, 21, (10, 21); "right increase")]
#[test_case(CounterPairEvent::Right(CounterEvent::Decrease), false, 19, (10, 19); "right decrease")]
#[test_case(CounterPairEvent::Right(CounterEvent::Reset), false, 0, (10, 0); "right reset")]
#[tokio::test]
async fn event_changes_only_its_branch_and_routes_effects(
    event: CounterPairEvent,
    left: bool,
    expected_count: usize,
    expected_view: (usize, usize),
) -> anyhow::Result<()> {
    let (mut pair, mut effects) = CounterPair::test_fixture();
    assert_eq!(pair.view(), (0, 0));
    let mut setup = pair
        .update(Message::Outcome(CounterPairOutcome::Left(
            CounterOutcome::Loaded { counter: 10 },
        )))
        .driver(&mut effects);
    setup
        .drive()
        .await?
        .expect_effect()
        .expect_left()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    setup.drive().await?.expect_done();
    drop(setup);
    let mut setup = pair
        .update(Message::Outcome(CounterPairOutcome::Right(
            CounterOutcome::Loaded { counter: 20 },
        )))
        .driver(&mut effects);
    setup
        .drive()
        .await?
        .expect_effect()
        .expect_right()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    setup.drive().await?.expect_done();
    drop(setup);

    let mut command = pair.update(Message::Event(event)).driver(&mut effects);
    assert_eq!(pair.view(), expected_view);
    let render = command.drive().await?.expect_effect();
    let render = if left {
        render.expect_left()
    } else {
        render.expect_right()
    };
    render.expect_render().resolve_render(|| async {}).await?;
    let write = command.drive().await?.expect_effect();
    let write = if left {
        write.expect_left()
    } else {
        write.expect_right()
    };
    write
        .expect_database()
        .resolve_set_count(|count| async move {
            assert_eq!(count, expected_count);
            Ok(())
        })
        .await?;
    command.drive().await?.expect_done();
    Ok(())
}

#[test_matrix([false, true], [Ok(None), Ok(Some(17)), Err(DatabaseError::Storage), Err(DatabaseError::OutOfRange), Err(DatabaseError::Transport)])]
#[tokio::test]
async fn init_routes_both_outcomes_in_either_response_order(
    right_first: bool,
    left_reply: Result<Option<usize>, DatabaseError>,
) -> anyhow::Result<()> {
    let (mut pair, mut effects) = CounterPair::test_fixture();
    let expected_left = match &left_reply {
        Ok(Some(value)) => *value,
        _ => 0,
    };
    let mut init = pair.init().driver(&mut effects);
    let first = init.drive().await?.expect_effect();
    let second = init.drive().await?.expect_effect();
    let (left, right) = match (first, second) {
        (CounterPairEffect::Left(left), CounterPairEffect::Right(right))
        | (CounterPairEffect::Right(right), CounterPairEffect::Left(left)) => {
            (left.expect_database(), right.expect_database())
        }
        other => panic!("expected one read per branch: {other:?}"),
    };
    let first_outcome;
    if right_first {
        right
            .resolve_count(|| async { Ok(Some(usize::MAX)) })
            .await?;
        first_outcome = init.drive().await?.expect_outcome();
        assert!(matches!(
            first_outcome,
            CounterPairOutcome::Right(CounterOutcome::Loaded {
                counter: usize::MAX
            })
        ));
        left.resolve_count(|| async move { left_reply }).await?;
    } else {
        left.resolve_count(|| async move { left_reply }).await?;
        first_outcome = init.drive().await?.expect_outcome();
        assert!(
            matches!(first_outcome, CounterPairOutcome::Left(CounterOutcome::Loaded { counter }) if counter == expected_left)
        );
        right
            .resolve_count(|| async { Ok(Some(usize::MAX)) })
            .await?;
    }
    let second_outcome = init.drive().await?.expect_outcome();
    if right_first {
        assert!(
            matches!(second_outcome, CounterPairOutcome::Left(CounterOutcome::Loaded { counter }) if counter == expected_left)
        );
    } else {
        assert!(matches!(
            second_outcome,
            CounterPairOutcome::Right(CounterOutcome::Loaded {
                counter: usize::MAX
            })
        ));
    }
    init.drive().await?.expect_done();
    drop(init);
    assert_eq!(pair.view(), (0, 0));

    // Apply each result in delivery order and verify the render follows its route.
    for outcome in [first_outcome, second_outcome] {
        let left = matches!(outcome, CounterPairOutcome::Left(_));
        let before = pair.view();
        let mut command = pair.update(Message::Outcome(outcome)).driver(&mut effects);
        assert_eq!(
            pair.view(),
            if left {
                (expected_left, before.1)
            } else {
                (before.0, usize::MAX)
            }
        );
        let render = command.drive().await?.expect_effect();
        let render = if left {
            render.expect_left()
        } else {
            render.expect_right()
        };
        render.expect_render().resolve_render(|| async {}).await?;
        command.drive().await?.expect_done();
    }
    assert_eq!(pair.view(), (expected_left, usize::MAX));
    Ok(())
}

#[test_case(true; "stop left")]
#[test_case(false; "stop right")]
#[tokio::test]
async fn backgrounds_route_ticks_reject_sibling_ids_and_stop_independently(
    stop_left: bool,
) -> anyhow::Result<()> {
    let (mut left_caps, left_effects) = CounterCapabilities::testing();
    let (mut right_caps, right_effects) = CounterCapabilities::testing();
    let (left_clock, left_inbox) = ClockClient::new(CLIENT_CAPABILITY);
    let (right_clock, right_inbox) = ClockClient::new(CLIENT_CAPABILITY);
    left_caps.clock = Arc::new(left_clock);
    right_caps.clock = Arc::new(right_clock);
    let mut left_clock_effects = left_inbox.into_stream().boxed();
    let mut right_clock_effects = right_inbox.into_stream().boxed();
    let mut effects = select_all(vec![
        left_effects.map(CounterPairEffect::Left).boxed(),
        right_effects.map(CounterPairEffect::Right).boxed(),
    ])
    .boxed();
    let mut pair = CounterPair::new(left_caps, right_caps);
    let mut left = pair
        .update(Message::Event(CounterPairEvent::Left(
            CounterEvent::StartBackground,
        )))
        .driver(&mut left_clock_effects);
    let mut right = pair
        .update(Message::Event(CounterPairEvent::Right(
            CounterEvent::StartBackground,
        )))
        .driver(&mut right_clock_effects);

    left.drive()
        .await?
        .expect_effect()
        .resolve_sleep_ms(|ms| async move {
            assert_eq!(ms, 1_000);
        })
        .await?;
    let CounterPairOutcome::Left(CounterOutcome::Tick { id: left_id }) =
        left.drive().await?.expect_outcome()
    else {
        panic!("expected left tick");
    };
    right
        .drive()
        .await?
        .expect_effect()
        .resolve_sleep_ms(|ms| async move {
            assert_eq!(ms, 1_000);
        })
        .await?;
    let CounterPairOutcome::Right(CounterOutcome::Tick { id: right_id }) =
        right.drive().await?.expect_outcome()
    else {
        panic!("expected right tick");
    };
    assert_ne!(left_id, right_id);

    pair.update(Message::Outcome(CounterPairOutcome::Left(
        CounterOutcome::Tick { id: right_id },
    )))
    .driver(&mut effects)
    .drive()
    .await?
    .expect_done();
    pair.update(Message::Outcome(CounterPairOutcome::Right(
        CounterOutcome::Tick { id: left_id },
    )))
    .driver(&mut effects)
    .drive()
    .await?
    .expect_done();
    assert_eq!(pair.view(), (0, 0));
    let mut applied = pair
        .update(Message::Outcome(CounterPairOutcome::Left(
            CounterOutcome::Tick { id: left_id },
        )))
        .driver(&mut effects);
    applied
        .drive()
        .await?
        .expect_effect()
        .expect_left()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    applied
        .drive()
        .await?
        .expect_effect()
        .expect_left()
        .expect_database()
        .resolve_set_count(|count| async move {
            assert_eq!(count, 1);
            Ok(())
        })
        .await?;
    applied.drive().await?.expect_done();
    drop(applied);
    let mut applied = pair
        .update(Message::Outcome(CounterPairOutcome::Right(
            CounterOutcome::Tick { id: right_id },
        )))
        .driver(&mut effects);
    applied
        .drive()
        .await?
        .expect_effect()
        .expect_right()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    applied
        .drive()
        .await?
        .expect_effect()
        .expect_right()
        .expect_database()
        .resolve_set_count(|count| async move {
            assert_eq!(count, 1);
            Ok(())
        })
        .await?;
    applied.drive().await?.expect_done();
    drop(applied);
    assert_eq!(pair.view(), (1, 1));

    let stop = if stop_left {
        CounterPairEvent::Left(CounterEvent::StopBackground)
    } else {
        CounterPairEvent::Right(CounterEvent::StopBackground)
    };
    pair.update(Message::Event(stop))
        .driver(&mut effects)
        .drive()
        .await?
        .expect_done();
    let stale = if stop_left {
        CounterPairOutcome::Left(CounterOutcome::Tick { id: left_id })
    } else {
        CounterPairOutcome::Right(CounterOutcome::Tick { id: right_id })
    };
    pair.update(Message::Outcome(stale))
        .driver(&mut effects)
        .drive()
        .await?
        .expect_done();
    let outcome = if stop_left {
        left.drive().await?.expect_done();
        right
            .drive()
            .await?
            .expect_effect()
            .resolve_sleep_ms(|ms| async move {
                assert_eq!(ms, 1_000);
            })
            .await?;
        let outcome = right.drive().await?.expect_outcome();
        assert!(
            matches!(outcome, CounterPairOutcome::Right(CounterOutcome::Tick { id }) if id == right_id)
        );
        outcome
    } else {
        right.drive().await?.expect_done();
        left.drive()
            .await?
            .expect_effect()
            .resolve_sleep_ms(|ms| async move {
                assert_eq!(ms, 1_000);
            })
            .await?;
        let outcome = left.drive().await?.expect_outcome();
        assert!(
            matches!(outcome, CounterPairOutcome::Left(CounterOutcome::Tick { id }) if id == left_id)
        );
        outcome
    };
    let mut applied = pair.update(Message::Outcome(outcome)).driver(&mut effects);
    let render = applied.drive().await?.expect_effect();
    let render = if stop_left {
        render.expect_right()
    } else {
        render.expect_left()
    };
    render.expect_render().resolve_render(|| async {}).await?;
    let write = applied.drive().await?.expect_effect();
    let write = if stop_left {
        write.expect_right()
    } else {
        write.expect_left()
    };
    write
        .expect_database()
        .resolve_set_count(|count| async move {
            assert_eq!(count, 2);
            Ok(())
        })
        .await?;
    applied.drive().await?.expect_done();
    assert_eq!(pair.view(), if stop_left { (1, 2) } else { (2, 1) });
    Ok(())
}
