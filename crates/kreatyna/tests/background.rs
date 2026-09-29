use kreatyna::capabilities::{
    CounterEffectStream, DatabaseError, ResolveClock, ResolveCounterDatabase, ResolveRender,
};
use kreatyna::{
    Counter, CounterCapabilities, CounterEvent, CounterOutcome, CounterStatus, CounterView,
};
use krucyfiks::{
    Message, Widget,
    effects::{CommandTestExt, EnvelopeError},
};
use test_case::test_case;

// Ordinary behavior

#[test_case(0, 1; "first tick")]
#[test_case(41, 42; "ordinary tick")]
#[test_case(usize::MAX - 1, usize::MAX; "reach maximum")]
#[test_case(usize::MAX, usize::MAX; "saturate at maximum")]
#[tokio::test]
async fn applying_a_tick_increases_the_count_then_renders_and_persists(
    initial: usize,
    expected: usize,
) -> anyhow::Result<()> {
    let (mut counter, mut effects, mut clock_effects) = ready_counter_with_clock(initial).await?;
    let mut background = counter
        .update(Message::Event(CounterEvent::StartBackground))
        .driver(&mut clock_effects);
    assert_eq!(counter.view(), ready_view(initial));

    background
        .step()
        .await?
        .expect_effect()
        .expect_clock()
        .resolve_sleep_ms(|ms| async move {
            assert_eq!(ms, 1_000);
        })
        .await?;
    let CounterOutcome::Tick { id } = background.step().await?.expect_outcome() else {
        panic!("expected a background tick");
    };

    // The command emits a tick. Only dispatching that tick changes the view.
    assert_eq!(counter.view(), ready_view(initial));

    let mut command = counter
        .update(Message::Outcome(CounterOutcome::Tick { id }))
        .driver(&mut effects);
    assert_eq!(counter.view(), ready_view(expected));

    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command
        .step()
        .await?
        .expect_effect()
        .expect_database()
        .resolve_set_count(move |count| async move {
            assert_eq!(count, expected);
            Ok(())
        })
        .await?;
    command.step().await?.expect_done();

    Ok(())
}

#[test_case(CounterEvent::Increase, 11; "increase")]
#[test_case(CounterEvent::Decrease, 9; "decrease")]
#[test_case(CounterEvent::Reset, 0; "reset")]
#[tokio::test]
async fn a_foreground_change_keeps_background_ticks_running_from_the_new_count(
    event: CounterEvent,
    expected: usize,
) -> anyhow::Result<()> {
    let (mut counter, mut effects, mut clock_effects) = ready_counter_with_clock(10).await?;
    let mut background = counter
        .update(Message::Event(CounterEvent::StartBackground))
        .driver(&mut clock_effects);
    assert_eq!(counter.view(), ready_view(10));

    background
        .step()
        .await?
        .expect_effect()
        .expect_clock()
        .resolve_sleep_ms(|ms| async move {
            assert_eq!(ms, 1_000);
        })
        .await?;
    let CounterOutcome::Tick { id } = background.step().await?.expect_outcome() else {
        panic!("expected a background tick");
    };

    let mut command = counter.update(Message::Event(event)).driver(&mut effects);
    assert_eq!(counter.view(), ready_view(expected));

    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command
        .step()
        .await?
        .expect_effect()
        .expect_database()
        .resolve_set_count(move |count| async move {
            assert_eq!(count, expected);
            Ok(())
        })
        .await?;
    command.step().await?.expect_done();
    drop(command);

    background
        .step()
        .await?
        .expect_effect()
        .expect_clock()
        .resolve_sleep_ms(|ms| async move {
            assert_eq!(ms, 1_000);
        })
        .await?;
    let outcome = background.step().await?.expect_outcome();
    assert_eq!(CounterOutcome::Tick { id }, outcome);

    let mut command = counter
        .update(Message::Outcome(outcome))
        .driver(&mut effects);
    assert_eq!(counter.view(), ready_view(expected + 1));

    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command
        .step()
        .await?
        .expect_effect()
        .expect_database()
        .resolve_set_count(move |count| async move {
            assert_eq!(count, expected + 1);
            Ok(())
        })
        .await?;
    command.step().await?.expect_done();

    Ok(())
}

// Edge cases and guards

#[tokio::test]
async fn starting_again_before_the_first_poll_keeps_the_original_command() -> anyhow::Result<()> {
    let (mut counter, mut effects, mut clock_effects) = ready_counter_with_clock(0).await?;
    let mut background = counter
        .update(Message::Event(CounterEvent::StartBackground))
        .driver(&mut clock_effects);
    assert_eq!(counter.view(), ready_view(0));
    let command = counter.update(Message::Event(CounterEvent::StartBackground));
    assert_eq!(counter.view(), ready_view(0));
    command.driver(&mut effects).step().await?.expect_done();

    background
        .step()
        .await?
        .expect_effect()
        .expect_clock()
        .resolve_sleep_ms(|ms| async move {
            assert_eq!(ms, 1_000);
        })
        .await?;
    let CounterOutcome::Tick { id } = background.step().await?.expect_outcome() else {
        panic!("expected a background tick");
    };

    let mut command = counter
        .update(Message::Outcome(CounterOutcome::Tick { id }))
        .driver(&mut effects);
    assert_eq!(counter.view(), ready_view(1));

    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command
        .step()
        .await?
        .expect_effect()
        .expect_database()
        .resolve_set_count(move |count| async move {
            assert_eq!(count, 1);
            Ok(())
        })
        .await?;
    command.step().await?.expect_done();

    Ok(())
}

#[tokio::test]
async fn starting_again_after_a_tick_keeps_the_original_generation() -> anyhow::Result<()> {
    let (mut counter, mut effects, mut clock_effects) = ready_counter_with_clock(0).await?;
    let mut background = counter
        .update(Message::Event(CounterEvent::StartBackground))
        .driver(&mut clock_effects);
    assert_eq!(counter.view(), ready_view(0));

    background
        .step()
        .await?
        .expect_effect()
        .expect_clock()
        .resolve_sleep_ms(|ms| async move {
            assert_eq!(ms, 1_000);
        })
        .await?;
    let CounterOutcome::Tick { id } = background.step().await?.expect_outcome() else {
        panic!("expected a background tick");
    };

    let command = counter.update(Message::Event(CounterEvent::StartBackground));
    assert_eq!(counter.view(), ready_view(0));
    command.driver(&mut effects).step().await?.expect_done();

    background
        .step()
        .await?
        .expect_effect()
        .expect_clock()
        .resolve_sleep_ms(|ms| async move {
            assert_eq!(ms, 1_000);
        })
        .await?;
    let outcome = background.step().await?.expect_outcome();
    assert_eq!(CounterOutcome::Tick { id }, outcome);

    let mut command = counter
        .update(Message::Outcome(outcome))
        .driver(&mut effects);
    assert_eq!(counter.view(), ready_view(1));

    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command
        .step()
        .await?
        .expect_effect()
        .expect_database()
        .resolve_set_count(move |count| async move {
            assert_eq!(count, 1);
            Ok(())
        })
        .await?;
    command.step().await?.expect_done();

    Ok(())
}

#[tokio::test]
async fn stopping_without_a_running_background_is_repeatable() -> anyhow::Result<()> {
    let (mut counter, mut effects, _clock_effects) = ready_counter_with_clock(0).await?;
    // Repeated stops are consecutive interactions in the same lifecycle.
    for _ in 0..2 {
        let command = counter.update(Message::Event(CounterEvent::StopBackground));
        assert_eq!(counter.view(), ready_view(0));
        command.driver(&mut effects).step().await?.expect_done();
    }

    Ok(())
}

#[tokio::test]
async fn stopping_before_the_first_poll_prevents_a_clock_request() -> anyhow::Result<()> {
    let (mut counter, mut effects, mut clock_effects) = ready_counter_with_clock(0).await?;
    let mut background = counter
        .update(Message::Event(CounterEvent::StartBackground))
        .driver(&mut clock_effects);
    assert_eq!(counter.view(), ready_view(0));

    let command = counter.update(Message::Event(CounterEvent::StopBackground));
    assert_eq!(counter.view(), ready_view(0));
    command.driver(&mut effects).step().await?.expect_done();
    background.step().await?.expect_done();

    Ok(())
}

#[tokio::test]
async fn stopping_cancels_a_pending_sleep() -> anyhow::Result<()> {
    let (mut counter, mut effects, mut clock_effects) = ready_counter_with_clock(0).await?;
    let mut background = counter
        .update(Message::Event(CounterEvent::StartBackground))
        .driver(&mut clock_effects);
    assert_eq!(counter.view(), ready_view(0));
    let sleep = background.step().await?.expect_effect().expect_clock();

    let command = counter.update(Message::Event(CounterEvent::StopBackground));
    assert_eq!(counter.view(), ready_view(0));
    command.driver(&mut effects).step().await?.expect_done();
    background.step().await?.expect_done();
    assert!(matches!(
        sleep.resolve_sleep_ms(|_| async {}).await,
        Err(EnvelopeError::Send(_))
    ));

    Ok(())
}

#[tokio::test]
async fn a_tick_queued_before_stopping_is_ignored() -> anyhow::Result<()> {
    let (mut counter, mut effects, mut clock_effects) = ready_counter_with_clock(0).await?;
    let mut background = counter
        .update(Message::Event(CounterEvent::StartBackground))
        .driver(&mut clock_effects);
    assert_eq!(counter.view(), ready_view(0));

    background
        .step()
        .await?
        .expect_effect()
        .expect_clock()
        .resolve_sleep_ms(|ms| async move {
            assert_eq!(ms, 1_000);
        })
        .await?;
    let CounterOutcome::Tick { id } = background.step().await?.expect_outcome() else {
        panic!("expected a background tick");
    };

    let command = counter.update(Message::Event(CounterEvent::StopBackground));
    assert_eq!(counter.view(), ready_view(0));
    command.driver(&mut effects).step().await?.expect_done();
    background.step().await?.expect_done();

    let command = counter.update(Message::Outcome(CounterOutcome::Tick { id }));
    assert_eq!(counter.view(), ready_view(0));
    command.driver(&mut effects).step().await?.expect_done();

    Ok(())
}

#[tokio::test]
async fn restarting_after_a_stop_rejects_old_ticks_and_accepts_new_ticks() -> anyhow::Result<()> {
    let (mut counter, mut effects, mut clock_effects) = ready_counter_with_clock(0).await?;
    let mut background = counter
        .update(Message::Event(CounterEvent::StartBackground))
        .driver(&mut clock_effects);
    assert_eq!(counter.view(), ready_view(0));

    background
        .step()
        .await?
        .expect_effect()
        .expect_clock()
        .resolve_sleep_ms(|ms| async move {
            assert_eq!(ms, 1_000);
        })
        .await?;
    let CounterOutcome::Tick { id: old_id } = background.step().await?.expect_outcome() else {
        panic!("expected a background tick");
    };

    let command = counter.update(Message::Event(CounterEvent::StopBackground));
    assert_eq!(counter.view(), ready_view(0));
    command.driver(&mut effects).step().await?.expect_done();
    background.step().await?.expect_done();
    drop(background);

    let command = counter.update(Message::Outcome(CounterOutcome::Tick { id: old_id }));
    assert_eq!(counter.view(), ready_view(0));
    command.driver(&mut effects).step().await?.expect_done();

    let mut restarted = counter
        .update(Message::Event(CounterEvent::StartBackground))
        .driver(&mut clock_effects);
    assert_eq!(counter.view(), ready_view(0));
    restarted
        .step()
        .await?
        .expect_effect()
        .expect_clock()
        .resolve_sleep_ms(|ms| async move {
            assert_eq!(ms, 1_000);
        })
        .await?;
    let CounterOutcome::Tick { id: new_id } = restarted.step().await?.expect_outcome() else {
        panic!("expected a background tick");
    };

    assert_ne!(old_id, new_id);

    let command = counter.update(Message::Outcome(CounterOutcome::Tick { id: old_id }));
    assert_eq!(counter.view(), ready_view(0));
    command.driver(&mut effects).step().await?.expect_done();

    let mut command = counter
        .update(Message::Outcome(CounterOutcome::Tick { id: new_id }))
        .driver(&mut effects);
    assert_eq!(counter.view(), ready_view(1));

    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command
        .step()
        .await?
        .expect_effect()
        .expect_database()
        .resolve_set_count(move |count| async move {
            assert_eq!(count, 1);
            Ok(())
        })
        .await?;
    command.step().await?.expect_done();

    Ok(())
}

#[tokio::test]
async fn restarting_after_dropping_the_command_rejects_old_ticks_and_accepts_new_ticks()
-> anyhow::Result<()> {
    let (mut counter, mut effects, mut clock_effects) = ready_counter_with_clock(0).await?;
    let mut background = counter
        .update(Message::Event(CounterEvent::StartBackground))
        .driver(&mut clock_effects);
    assert_eq!(counter.view(), ready_view(0));

    background
        .step()
        .await?
        .expect_effect()
        .expect_clock()
        .resolve_sleep_ms(|ms| async move {
            assert_eq!(ms, 1_000);
        })
        .await?;
    let CounterOutcome::Tick { id: old_id } = background.step().await?.expect_outcome() else {
        panic!("expected a background tick");
    };
    drop(background);

    let command = counter.update(Message::Outcome(CounterOutcome::Tick { id: old_id }));
    assert_eq!(counter.view(), ready_view(0));
    command.driver(&mut effects).step().await?.expect_done();

    let mut restarted = counter
        .update(Message::Event(CounterEvent::StartBackground))
        .driver(&mut clock_effects);
    assert_eq!(counter.view(), ready_view(0));
    restarted
        .step()
        .await?
        .expect_effect()
        .expect_clock()
        .resolve_sleep_ms(|ms| async move {
            assert_eq!(ms, 1_000);
        })
        .await?;
    let CounterOutcome::Tick { id: new_id } = restarted.step().await?.expect_outcome() else {
        panic!("expected a background tick");
    };

    assert_ne!(old_id, new_id);

    let command = counter.update(Message::Outcome(CounterOutcome::Tick { id: old_id }));
    assert_eq!(counter.view(), ready_view(0));
    command.driver(&mut effects).step().await?.expect_done();

    let mut command = counter
        .update(Message::Outcome(CounterOutcome::Tick { id: new_id }))
        .driver(&mut effects);
    assert_eq!(counter.view(), ready_view(1));

    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command
        .step()
        .await?
        .expect_effect()
        .expect_database()
        .resolve_set_count(move |count| async move {
            assert_eq!(count, 1);
            Ok(())
        })
        .await?;
    command.step().await?.expect_done();

    Ok(())
}

#[tokio::test]
async fn dropping_an_unpolled_background_command_allows_a_new_start() -> anyhow::Result<()> {
    let (mut counter, mut effects, mut clock_effects) = ready_counter_with_clock(0).await?;
    let command = counter.update(Message::Event(CounterEvent::StartBackground));
    assert_eq!(counter.view(), ready_view(0));
    drop(command);

    let mut background = counter
        .update(Message::Event(CounterEvent::StartBackground))
        .driver(&mut clock_effects);
    assert_eq!(counter.view(), ready_view(0));

    background
        .step()
        .await?
        .expect_effect()
        .expect_clock()
        .resolve_sleep_ms(|ms| async move {
            assert_eq!(ms, 1_000);
        })
        .await?;
    let CounterOutcome::Tick { id } = background.step().await?.expect_outcome() else {
        panic!("expected a background tick");
    };

    let mut command = counter
        .update(Message::Outcome(CounterOutcome::Tick { id }))
        .driver(&mut effects);
    assert_eq!(counter.view(), ready_view(1));

    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command
        .step()
        .await?
        .expect_effect()
        .expect_database()
        .resolve_set_count(move |count| async move {
            assert_eq!(count, 1);
            Ok(())
        })
        .await?;
    command.step().await?.expect_done();

    Ok(())
}

#[tokio::test]
async fn dropping_the_counter_before_the_first_poll_prevents_a_clock_request() -> anyhow::Result<()>
{
    let (mut counter, mut _effects, mut clock_effects) = ready_counter_with_clock(0).await?;
    let mut background = counter
        .update(Message::Event(CounterEvent::StartBackground))
        .driver(&mut clock_effects);
    assert_eq!(counter.view(), ready_view(0));
    drop(counter);
    background.step().await?.expect_done();

    Ok(())
}

#[tokio::test]
async fn dropping_the_counter_cancels_a_pending_sleep() -> anyhow::Result<()> {
    let (mut counter, mut _effects, mut clock_effects) = ready_counter_with_clock(0).await?;
    let mut background = counter
        .update(Message::Event(CounterEvent::StartBackground))
        .driver(&mut clock_effects);
    assert_eq!(counter.view(), ready_view(0));
    let sleep = background.step().await?.expect_effect().expect_clock();
    drop(counter);

    background.step().await?.expect_done();
    assert!(matches!(
        sleep.resolve_sleep_ms(|_| async {}).await,
        Err(EnvelopeError::Send(_))
    ));

    Ok(())
}

#[tokio::test]
async fn beginning_shutdown_cancels_sleep_and_rejects_a_queued_tick() -> anyhow::Result<()> {
    let (mut counter, mut effects, mut clock_effects) = ready_counter_with_clock(0).await?;
    let mut background = counter
        .update(Message::Event(CounterEvent::StartBackground))
        .driver(&mut clock_effects);
    assert_eq!(counter.view(), ready_view(0));

    background
        .step()
        .await?
        .expect_effect()
        .expect_clock()
        .resolve_sleep_ms(|ms| async move {
            assert_eq!(ms, 1_000);
        })
        .await?;
    let CounterOutcome::Tick { id } = background.step().await?.expect_outcome() else {
        panic!("expected a background tick");
    };

    let sleep = background.step().await?.expect_effect().expect_clock();
    let command = counter.begin_shutdown();
    assert_eq!(counter.view(), ready_view(0));
    command.driver(&mut effects).step().await?.expect_done();
    background.step().await?.expect_done();
    assert!(matches!(
        sleep.resolve_sleep_ms(|_| async {}).await,
        Err(EnvelopeError::Send(_))
    ));

    let command = counter.update(Message::Outcome(CounterOutcome::Tick { id }));
    assert_eq!(counter.view(), ready_view(0));
    command.driver(&mut effects).step().await?.expect_done();

    Ok(())
}

// Failures and recovery

#[test_case(DatabaseError::Storage; "storage failure")]
#[test_case(DatabaseError::OutOfRange; "out of range")]
#[test_case(DatabaseError::Transport; "transport failure")]
#[tokio::test]
async fn a_failed_tick_write_keeps_the_count_and_the_next_tick_can_be_saved(
    error: DatabaseError,
) -> anyhow::Result<()> {
    let (mut counter, mut effects, mut clock_effects) = ready_counter_with_clock(0).await?;
    let mut background = counter
        .update(Message::Event(CounterEvent::StartBackground))
        .driver(&mut clock_effects);
    assert_eq!(counter.view(), ready_view(0));

    background
        .step()
        .await?
        .expect_effect()
        .expect_clock()
        .resolve_sleep_ms(|ms| async move {
            assert_eq!(ms, 1_000);
        })
        .await?;
    let CounterOutcome::Tick { id } = background.step().await?.expect_outcome() else {
        panic!("expected a background tick");
    };

    let mut command = counter
        .update(Message::Outcome(CounterOutcome::Tick { id }))
        .driver(&mut effects);
    assert_eq!(counter.view(), ready_view(1));

    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command
        .step()
        .await?
        .expect_effect()
        .expect_database()
        .resolve_set_count(move |count| async move {
            assert_eq!(count, 1);
            Err(error)
        })
        .await?;
    command.step().await?.expect_done();
    drop(command);

    background
        .step()
        .await?
        .expect_effect()
        .expect_clock()
        .resolve_sleep_ms(|ms| async move {
            assert_eq!(ms, 1_000);
        })
        .await?;
    let outcome = background.step().await?.expect_outcome();
    assert_eq!(CounterOutcome::Tick { id }, outcome);

    let mut next = counter
        .update(Message::Outcome(outcome))
        .driver(&mut effects);
    assert_eq!(counter.view(), ready_view(2));

    next.step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    next.step()
        .await?
        .expect_effect()
        .expect_database()
        .resolve_set_count(move |count| async move {
            assert_eq!(count, 2);
            Ok(())
        })
        .await?;
    next.step().await?.expect_done();

    Ok(())
}

#[tokio::test]
#[should_panic(expected = "Clock::sleep_ms transport failed: response channel closed")]
async fn a_dropped_clock_response_panics() {
    let (mut counter, _effects, mut clock_effects) = ready_counter_with_clock(0).await.unwrap();
    let mut background = counter
        .update(Message::Event(CounterEvent::StartBackground))
        .driver(&mut clock_effects);
    assert_eq!(counter.view(), ready_view(0));

    drop(
        background
            .step()
            .await
            .unwrap()
            .expect_effect()
            .expect_clock(),
    );
    background.step().await.unwrap().expect_outcome();
}

// Test support

fn ready_view(count: usize) -> CounterView {
    CounterView {
        count,
        status: CounterStatus::Ready,
    }
}

async fn ready_counter_with_clock(
    count: usize,
) -> anyhow::Result<(Counter, CounterEffectStream, CounterEffectStream)> {
    // Clock and foreground commands need independent driver borrows.
    let (mut capabilities, clock_effects) = CounterCapabilities::testing();
    let (foreground_capabilities, mut effects) = CounterCapabilities::testing();
    capabilities.render = foreground_capabilities.render;
    capabilities.db = foreground_capabilities.db;
    let mut counter = Counter::new(capabilities);

    let mut setup = counter
        .update(Message::Outcome(CounterOutcome::Loaded { counter: count }))
        .driver(&mut effects);
    setup
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    setup.step().await?.expect_done();
    drop(setup);
    Ok((counter, effects, clock_effects))
}
