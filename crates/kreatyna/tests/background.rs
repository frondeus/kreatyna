use std::sync::Arc;

use futures::StreamExt;
use kreatyna::capabilities::{
    ClockClient, DatabaseError, ResolveClock, ResolveCounterDatabase, ResolveRender,
};
use kreatyna::{CLIENT_CAPABILITY, Counter, CounterCapabilities, CounterEvent, CounterOutcome};
use krucyfiks::{Message, Widget, effects::CommandTestExt};
use test_case::{test_case, test_matrix};

#[test_case(0, 1; "first tick")]
#[test_case(41, 42; "ordinary tick")]
#[test_case(usize::MAX - 1, usize::MAX; "reach maximum")]
#[test_case(usize::MAX, usize::MAX; "saturate at maximum")]
#[tokio::test]
async fn accepted_tick_renders_then_persists(
    initial: usize,
    expected: usize,
) -> anyhow::Result<()> {
    // A separate clock inbox lets us retain the background driver while driving updates.
    let (mut caps, mut effects) = CounterCapabilities::testing();
    let (clock, clock_inbox) = ClockClient::new(CLIENT_CAPABILITY);
    caps.clock = Arc::new(clock);
    let mut clock_effects = clock_inbox.into_stream().boxed();
    let mut counter = Counter::new(caps);

    let mut setup = counter
        .update(Message::Outcome(CounterOutcome::Loaded {
            counter: initial,
        }))
        .driver(&mut effects);
    setup
        .drive()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    setup.drive().await?.expect_done();
    drop(setup);

    let mut background = counter
        .update(Message::Event(CounterEvent::StartBackground))
        .driver(&mut clock_effects);

    background
        .drive()
        .await?
        .expect_effect()
        .resolve_sleep_ms(|ms| async move {
            assert_eq!(ms, 1_000);
        })
        .await?;
    let CounterOutcome::Tick { id } = background.drive().await?.expect_outcome() else {
        panic!("expected tick");
    };

    assert_eq!(counter.view(), initial, "emitting a tick does not apply it");
    let mut command = counter
        .update(Message::Outcome(CounterOutcome::Tick { id }))
        .driver(&mut effects);
    assert_eq!(counter.view(), expected);

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
        .resolve_set_count(|count| async move {
            assert_eq!(count, expected);
            Ok(())
        })
        .await?;
    command.drive().await?.expect_done();

    Ok(())
}

#[test_case(DatabaseError::Storage; "storage failure")]
#[test_case(DatabaseError::OutOfRange; "out of range")]
#[test_case(DatabaseError::Transport; "transport failure")]
#[tokio::test]
async fn failed_tick_write_keeps_state_and_background_continues(
    error: DatabaseError,
) -> anyhow::Result<()> {
    // A separate clock inbox lets us retain the background driver while driving updates.
    let (mut caps, mut effects) = CounterCapabilities::testing();
    let (clock, clock_inbox) = ClockClient::new(CLIENT_CAPABILITY);
    caps.clock = Arc::new(clock);
    let mut clock_effects = clock_inbox.into_stream().boxed();
    let mut counter = Counter::new(caps);

    let mut background = counter
        .update(Message::Event(CounterEvent::StartBackground))
        .driver(&mut clock_effects);

    background
        .drive()
        .await?
        .expect_effect()
        .resolve_sleep_ms(|ms| async move {
            assert_eq!(ms, 1_000);
        })
        .await?;
    let CounterOutcome::Tick { id } = background.drive().await?.expect_outcome() else {
        panic!("expected tick");
    };

    let mut command = counter
        .update(Message::Outcome(CounterOutcome::Tick { id }))
        .driver(&mut effects);

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
        .resolve_set_count(|count| async move {
            assert_eq!(count, 1);
            Err(error)
        })
        .await?;
    command.drive().await?.expect_done();

    assert_eq!(counter.view(), 1);
    drop(command);
    background
        .drive()
        .await?
        .expect_effect()
        .resolve_sleep_ms(|ms| async move {
            assert_eq!(ms, 1_000);
        })
        .await?;
    let next = background.drive().await?.expect_outcome();
    assert!(matches!(next, CounterOutcome::Tick { id: next_id } if next_id == id));
    let mut next = counter.update(Message::Outcome(next)).driver(&mut effects);

    next.drive()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    next.drive()
        .await?
        .expect_effect()
        .expect_database()
        .resolve_set_count(|count| async move {
            assert_eq!(count, 2);
            Ok(())
        })
        .await?;
    next.drive().await?.expect_done();

    assert_eq!(counter.view(), 2);
    Ok(())
}

#[tokio::test]
async fn duplicate_start_keeps_the_original_background_command() -> anyhow::Result<()> {
    // A separate clock inbox lets us retain the background driver while driving updates.
    let (mut caps, mut effects) = CounterCapabilities::testing();
    let (clock, clock_inbox) = ClockClient::new(CLIENT_CAPABILITY);
    caps.clock = Arc::new(clock);
    let mut clock_effects = clock_inbox.into_stream().boxed();
    let mut counter = Counter::new(caps);

    let mut background = counter
        .update(Message::Event(CounterEvent::StartBackground))
        .driver(&mut clock_effects);

    counter
        .update(Message::Event(CounterEvent::StartBackground))
        .driver(&mut effects)
        .drive()
        .await?
        .expect_done();

    background
        .drive()
        .await?
        .expect_effect()
        .resolve_sleep_ms(|ms| async move {
            assert_eq!(ms, 1_000);
        })
        .await?;
    let CounterOutcome::Tick { id } = background.drive().await?.expect_outcome() else {
        panic!("expected tick");
    };

    counter
        .update(Message::Event(CounterEvent::StartBackground))
        .driver(&mut effects)
        .drive()
        .await?
        .expect_done();
    background
        .drive()
        .await?
        .expect_effect()
        .resolve_sleep_ms(|ms| async move {
            assert_eq!(ms, 1_000);
        })
        .await?;
    assert!(
        matches!(background.drive().await?.expect_outcome(), CounterOutcome::Tick { id: next } if next == id)
    );
    assert_eq!(counter.view(), 0);
    Ok(())
}

#[tokio::test]
async fn stop_without_start_is_repeatable() -> anyhow::Result<()> {
    let (mut counter, mut effects) = Counter::test_fixture();
    counter
        .update(Message::Event(CounterEvent::StopBackground))
        .driver(&mut effects)
        .drive()
        .await?
        .expect_done();
    counter
        .update(Message::Event(CounterEvent::StopBackground))
        .driver(&mut effects)
        .drive()
        .await?
        .expect_done();
    assert_eq!(counter.view(), 0);
    Ok(())
}

#[tokio::test]
async fn stop_before_first_poll_prevents_clock_request() -> anyhow::Result<()> {
    // A separate clock inbox lets us retain the background driver while driving updates.
    let (mut caps, mut effects) = CounterCapabilities::testing();
    let (clock, clock_inbox) = ClockClient::new(CLIENT_CAPABILITY);
    caps.clock = Arc::new(clock);
    let mut clock_effects = clock_inbox.into_stream().boxed();
    let mut counter = Counter::new(caps);

    let mut background = counter
        .update(Message::Event(CounterEvent::StartBackground))
        .driver(&mut clock_effects);

    counter
        .update(Message::Event(CounterEvent::StopBackground))
        .driver(&mut effects)
        .drive()
        .await?
        .expect_done();
    background.drive().await?.expect_done();
    assert_eq!(counter.view(), 0);
    Ok(())
}

#[tokio::test]
async fn stop_cancels_pending_sleep_and_rejects_queued_tick() -> anyhow::Result<()> {
    // A separate clock inbox lets us retain the background driver while driving updates.
    let (mut caps, mut effects) = CounterCapabilities::testing();
    let (clock, clock_inbox) = ClockClient::new(CLIENT_CAPABILITY);
    caps.clock = Arc::new(clock);
    let mut clock_effects = clock_inbox.into_stream().boxed();
    let mut counter = Counter::new(caps);

    let mut background = counter
        .update(Message::Event(CounterEvent::StartBackground))
        .driver(&mut clock_effects);

    background
        .drive()
        .await?
        .expect_effect()
        .resolve_sleep_ms(|ms| async move {
            assert_eq!(ms, 1_000);
        })
        .await?;
    let CounterOutcome::Tick { id } = background.drive().await?.expect_outcome() else {
        panic!("expected tick");
    };

    let sleep = background.drive().await?.expect_effect();
    counter
        .update(Message::Event(CounterEvent::StopBackground))
        .driver(&mut effects)
        .drive()
        .await?
        .expect_done();
    background.drive().await?.expect_done();
    assert!(matches!(
        sleep.resolve_sleep_ms(|_| async {}).await,
        Err(krucyfiks::effects::EnvelopeError::Send(_))
    ));
    counter
        .update(Message::Outcome(CounterOutcome::Tick { id }))
        .driver(&mut effects)
        .drive()
        .await?
        .expect_done();
    assert_eq!(counter.view(), 0);
    Ok(())
}

#[test_case(true; "explicit stop")]
#[test_case(false; "dropped command")]
#[tokio::test]
async fn restart_rejects_old_generation_and_accepts_new_ticks(stop: bool) -> anyhow::Result<()> {
    // A separate clock inbox lets us retain the background driver while driving updates.
    let (mut caps, mut effects) = CounterCapabilities::testing();
    let (clock, clock_inbox) = ClockClient::new(CLIENT_CAPABILITY);
    caps.clock = Arc::new(clock);
    let mut clock_effects = clock_inbox.into_stream().boxed();
    let mut counter = Counter::new(caps);

    let mut background = counter
        .update(Message::Event(CounterEvent::StartBackground))
        .driver(&mut clock_effects);

    background
        .drive()
        .await?
        .expect_effect()
        .resolve_sleep_ms(|ms| async move {
            assert_eq!(ms, 1_000);
        })
        .await?;
    let CounterOutcome::Tick { id } = background.drive().await?.expect_outcome() else {
        panic!("expected tick");
    };

    if stop {
        counter
            .update(Message::Event(CounterEvent::StopBackground))
            .driver(&mut effects)
            .drive()
            .await?
            .expect_done();
        background.drive().await?.expect_done();
    }
    drop(background);
    counter
        .update(Message::Outcome(CounterOutcome::Tick { id }))
        .driver(&mut effects)
        .drive()
        .await?
        .expect_done();
    let mut restarted = counter
        .update(Message::Event(CounterEvent::StartBackground))
        .driver(&mut clock_effects);
    restarted
        .drive()
        .await?
        .expect_effect()
        .resolve_sleep_ms(|ms| async move {
            assert_eq!(ms, 1_000);
        })
        .await?;
    let CounterOutcome::Tick { id: new_id } = restarted.drive().await?.expect_outcome() else {
        panic!("expected tick");
    };
    assert_ne!(id, new_id);
    counter
        .update(Message::Outcome(CounterOutcome::Tick { id }))
        .driver(&mut effects)
        .drive()
        .await?
        .expect_done();
    assert_eq!(counter.view(), 0);
    let mut command = counter
        .update(Message::Outcome(CounterOutcome::Tick { id: new_id }))
        .driver(&mut effects);

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
        .resolve_set_count(|count| async move {
            assert_eq!(count, 1);
            Ok(())
        })
        .await?;
    command.drive().await?.expect_done();

    assert_eq!(counter.view(), 1);
    Ok(())
}

#[tokio::test]
async fn dropping_unpolled_background_allows_restart() -> anyhow::Result<()> {
    // A separate clock inbox lets us retain the background driver while driving updates.
    let (mut caps, mut effects) = CounterCapabilities::testing();
    let (clock, clock_inbox) = ClockClient::new(CLIENT_CAPABILITY);
    caps.clock = Arc::new(clock);
    let mut clock_effects = clock_inbox.into_stream().boxed();
    let mut counter = Counter::new(caps);

    drop(counter.update(Message::Event(CounterEvent::StartBackground)));

    let mut background = counter
        .update(Message::Event(CounterEvent::StartBackground))
        .driver(&mut clock_effects);

    background
        .drive()
        .await?
        .expect_effect()
        .resolve_sleep_ms(|ms| async move {
            assert_eq!(ms, 1_000);
        })
        .await?;
    let CounterOutcome::Tick { id } = background.drive().await?.expect_outcome() else {
        panic!("expected tick");
    };

    let mut command = counter
        .update(Message::Outcome(CounterOutcome::Tick { id }))
        .driver(&mut effects);

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
        .resolve_set_count(|count| async move {
            assert_eq!(count, 1);
            Ok(())
        })
        .await?;
    command.drive().await?.expect_done();

    Ok(())
}

#[test_case(false; "before first poll")]
#[test_case(true; "while sleeping")]
#[tokio::test]
async fn dropping_counter_cancels_background(sleeping: bool) -> anyhow::Result<()> {
    // A separate clock inbox lets us retain the background driver while driving updates.
    let (mut caps, _effects) = CounterCapabilities::testing();
    let (clock, clock_inbox) = ClockClient::new(CLIENT_CAPABILITY);
    caps.clock = Arc::new(clock);
    let mut clock_effects = clock_inbox.into_stream().boxed();
    let mut counter = Counter::new(caps);

    let mut background = counter
        .update(Message::Event(CounterEvent::StartBackground))
        .driver(&mut clock_effects);

    let sleep = if sleeping {
        Some(background.drive().await?.expect_effect())
    } else {
        None
    };
    drop(counter);
    background.drive().await?.expect_done();
    if let Some(sleep) = sleep {
        assert!(matches!(
            sleep.resolve_sleep_ms(|_| async {}).await,
            Err(krucyfiks::effects::EnvelopeError::Send(_))
        ));
    }
    Ok(())
}

#[test_matrix([(CounterEvent::Increase, 1), (CounterEvent::Decrease, 0), (CounterEvent::Reset, 0)])]
#[tokio::test]
async fn foreground_event_keeps_background_running(
    (event, expected): (CounterEvent, usize),
) -> anyhow::Result<()> {
    // A separate clock inbox lets us retain the background driver while driving updates.
    let (mut caps, mut effects) = CounterCapabilities::testing();
    let (clock, clock_inbox) = ClockClient::new(CLIENT_CAPABILITY);
    caps.clock = Arc::new(clock);
    let mut clock_effects = clock_inbox.into_stream().boxed();
    let mut counter = Counter::new(caps);

    let mut background = counter
        .update(Message::Event(CounterEvent::StartBackground))
        .driver(&mut clock_effects);

    background
        .drive()
        .await?
        .expect_effect()
        .resolve_sleep_ms(|ms| async move {
            assert_eq!(ms, 1_000);
        })
        .await?;
    let CounterOutcome::Tick { id } = background.drive().await?.expect_outcome() else {
        panic!("expected tick");
    };

    let mut command = counter.update(Message::Event(event)).driver(&mut effects);

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
        .resolve_set_count(|count| async move {
            assert_eq!(count, expected);
            Ok(())
        })
        .await?;
    command.drive().await?.expect_done();

    drop(command);
    background
        .drive()
        .await?
        .expect_effect()
        .resolve_sleep_ms(|ms| async move {
            assert_eq!(ms, 1_000);
        })
        .await?;
    let outcome = background.drive().await?.expect_outcome();
    assert!(matches!(outcome, CounterOutcome::Tick { id: next } if next == id));
    let mut command = counter
        .update(Message::Outcome(outcome))
        .driver(&mut effects);

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
        .resolve_set_count(|count| async move {
            assert_eq!(count, expected + 1);
            Ok(())
        })
        .await?;
    command.drive().await?.expect_done();

    assert_eq!(counter.view(), expected + 1);
    Ok(())
}

#[tokio::test]
async fn loaded_keeps_background_running_and_tick_uses_loaded_value() -> anyhow::Result<()> {
    // A separate clock inbox lets us retain the background driver while driving updates.
    let (mut caps, mut effects) = CounterCapabilities::testing();
    let (clock, clock_inbox) = ClockClient::new(CLIENT_CAPABILITY);
    caps.clock = Arc::new(clock);
    let mut clock_effects = clock_inbox.into_stream().boxed();
    let mut counter = Counter::new(caps);

    let mut background = counter
        .update(Message::Event(CounterEvent::StartBackground))
        .driver(&mut clock_effects);

    background
        .drive()
        .await?
        .expect_effect()
        .resolve_sleep_ms(|ms| async move {
            assert_eq!(ms, 1_000);
        })
        .await?;
    let CounterOutcome::Tick { id } = background.drive().await?.expect_outcome() else {
        panic!("expected tick");
    };

    let mut loaded = counter
        .update(Message::Outcome(CounterOutcome::Loaded { counter: 88 }))
        .driver(&mut effects);
    loaded
        .drive()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    loaded.drive().await?.expect_done();
    drop(loaded);
    background
        .drive()
        .await?
        .expect_effect()
        .resolve_sleep_ms(|ms| async move {
            assert_eq!(ms, 1_000);
        })
        .await?;
    let outcome = background.drive().await?.expect_outcome();
    assert!(matches!(outcome, CounterOutcome::Tick { id: next } if next == id));
    let mut command = counter
        .update(Message::Outcome(outcome))
        .driver(&mut effects);

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
        .resolve_set_count(|count| async move {
            assert_eq!(count, 89);
            Ok(())
        })
        .await?;
    command.drive().await?.expect_done();

    assert_eq!(counter.view(), 89);
    Ok(())
}

#[tokio::test]
#[should_panic(expected = "Clock::sleep_ms transport failed: response channel closed")]
async fn dropped_clock_response_panics() {
    // A separate clock inbox lets us retain the background driver while driving updates.
    let (mut caps, _effects) = CounterCapabilities::testing();
    let (clock, clock_inbox) = ClockClient::new(CLIENT_CAPABILITY);
    caps.clock = Arc::new(clock);
    let mut clock_effects = clock_inbox.into_stream().boxed();
    let mut counter = Counter::new(caps);

    let mut background = counter
        .update(Message::Event(CounterEvent::StartBackground))
        .driver(&mut clock_effects);

    drop(background.drive().await.unwrap().expect_effect());
    background.drive().await.unwrap().expect_outcome();
}
