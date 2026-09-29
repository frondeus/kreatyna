use kreatyna::capabilities::{
    CounterEffectStream, DatabaseError, ResolveCounterDatabase, ResolveRender,
};
use kreatyna::{Counter, CounterEvent, CounterOutcome, CounterStatus, CounterView};
use krucyfiks::{
    Message, Widget,
    effects::{CommandTestExt, EnvelopeError},
};
use test_case::{test_case, test_matrix};

// Ordinary behavior

#[test_case(None, 0; "missing row")]
#[test_case(Some(0), 0; "stored zero")]
#[test_case(Some(42), 42; "stored count")]
#[test_case(Some(usize::MAX), usize::MAX; "stored maximum")]
#[tokio::test]
async fn initialization_reads_the_count_without_applying_it(
    stored: Option<usize>,
    expected: usize,
) -> anyhow::Result<()> {
    let (mut counter, mut effects) = Counter::test_fixture();
    let mut command = counter.init().driver(&mut effects);
    assert_eq!(counter.view(), loading_view());

    command
        .step()
        .await?
        .expect_effect()
        .expect_database()
        .resolve_count(|| async move { Ok(stored) })
        .await?;
    let outcome = command.step().await?.expect_outcome();
    assert_eq!(CounterOutcome::Loaded { counter: expected }, outcome);
    command.step().await?.expect_done();
    Ok(())
}

#[test_case(0; "empty count still renders")]
#[test_case(42; "ordinary count")]
#[test_case(usize::MAX; "maximum count")]
#[tokio::test]
async fn applying_the_loaded_count_makes_the_counter_ready_and_only_renders(
    count: usize,
) -> anyhow::Result<()> {
    let (mut counter, mut effects) = Counter::test_fixture();
    assert_eq!(counter.view(), loading_view());

    let mut command = counter
        .update(Message::Outcome(CounterOutcome::Loaded { counter: count }))
        .driver(&mut effects);
    assert_eq!(counter.view(), ready_view(count));

    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command.step().await?.expect_done();
    Ok(())
}

#[test_case(0, CounterEvent::Increase, 1; "increase zero")]
#[test_case(42, CounterEvent::Increase, 43; "increase ordinary count")]
#[test_case(usize::MAX - 1, CounterEvent::Increase, usize::MAX; "reach maximum")]
#[test_case(usize::MAX, CounterEvent::Increase, usize::MAX; "increase saturates")]
#[test_case(0, CounterEvent::Decrease, 0; "decrease saturates")]
#[test_case(1, CounterEvent::Decrease, 0; "reach zero")]
#[test_case(42, CounterEvent::Decrease, 41; "decrease ordinary count")]
#[test_case(usize::MAX, CounterEvent::Decrease, usize::MAX - 1; "decrease maximum")]
#[test_case(0, CounterEvent::Reset, 0; "reset zero still persists")]
#[test_case(42, CounterEvent::Reset, 0; "reset ordinary count")]
#[test_case(usize::MAX, CounterEvent::Reset, 0; "reset maximum")]
#[tokio::test]
async fn a_count_change_renders_then_persists_the_new_value(
    initial: usize,
    event: CounterEvent,
    expected: usize,
) -> anyhow::Result<()> {
    let (mut counter, mut effects) = ready_counter(initial).await?;
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
    Ok(())
}

// Edge cases and guards

#[tokio::test]
async fn delayed_commands_persist_each_events_captured_value() -> anyhow::Result<()> {
    let (mut counter, mut effects) = ready_counter(0).await?;
    let increase = counter.update(Message::Event(CounterEvent::Increase));
    assert_eq!(counter.view(), ready_view(1));
    let second_increase = counter.update(Message::Event(CounterEvent::Increase));
    assert_eq!(counter.view(), ready_view(2));
    let reset = counter.update(Message::Event(CounterEvent::Reset));
    assert_eq!(counter.view(), ready_view(0));

    // Later events must not replace the value captured by an earlier command.
    for (command, captured_count) in [(increase, 1), (second_increase, 2), (reset, 0)] {
        let mut command = command.driver(&mut effects);
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
                assert_eq!(count, captured_count);
                Ok(())
            })
            .await?;
        command.step().await?.expect_done();
    }
    Ok(())
}

#[tokio::test]
async fn cancelling_a_change_during_render_keeps_the_count_and_skips_its_write()
-> anyhow::Result<()> {
    let (mut counter, mut effects) = ready_counter(0).await?;
    let mut command = counter
        .update(Message::Event(CounterEvent::Increase))
        .driver(&mut effects);
    assert_eq!(counter.view(), ready_view(1));

    let render = command.step().await?.expect_effect().expect_render();
    drop(command);
    assert_eq!(counter.view(), ready_view(1));
    assert!(matches!(
        render.resolve_render(|| async {}).await,
        Err(EnvelopeError::Send(_))
    ));

    // The same inbox must contain only the next change's render and write.
    let mut next = counter
        .update(Message::Event(CounterEvent::Increase))
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
        .resolve_set_count(|count| async move {
            assert_eq!(count, 2);
            Ok(())
        })
        .await?;
    next.step().await?.expect_done();
    Ok(())
}

// Failures and recovery

#[test_case(DatabaseError::Storage; "storage failure")]
#[test_case(DatabaseError::OutOfRange; "out of range")]
#[test_case(DatabaseError::Transport; "transport failure")]
#[tokio::test]
async fn a_failed_read_falls_back_to_zero(error: DatabaseError) -> anyhow::Result<()> {
    let (mut counter, mut effects) = Counter::test_fixture();
    let mut command = counter.init().driver(&mut effects);
    assert_eq!(counter.view(), loading_view());

    command
        .step()
        .await?
        .expect_effect()
        .expect_database()
        .resolve_count(|| async move { Err(error) })
        .await?;
    let outcome = command.step().await?.expect_outcome();
    assert_eq!(CounterOutcome::Loaded { counter: 0 }, outcome);
    command.step().await?.expect_done();
    Ok(())
}

#[test_matrix(
    [(CounterEvent::Increase, 11), (CounterEvent::Decrease, 9), (CounterEvent::Reset, 0)],
    [DatabaseError::Storage, DatabaseError::OutOfRange, DatabaseError::Transport]
)]
#[tokio::test]
async fn a_failed_write_keeps_the_count_and_allows_the_next_change(
    (event, expected): (CounterEvent, usize),
    error: DatabaseError,
) -> anyhow::Result<()> {
    let (mut counter, mut effects) = ready_counter(10).await?;
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
            Err(error)
        })
        .await?;
    command.step().await?.expect_done();
    drop(command);

    let mut next = counter
        .update(Message::Event(CounterEvent::Increase))
        .driver(&mut effects);
    assert_eq!(counter.view(), ready_view(expected + 1));

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
            assert_eq!(count, expected + 1);
            Ok(())
        })
        .await?;
    next.step().await?.expect_done();
    Ok(())
}

#[tokio::test]
async fn a_dropped_read_response_falls_back_to_zero() -> anyhow::Result<()> {
    let (mut counter, mut effects) = Counter::test_fixture();
    let mut command = counter.init().driver(&mut effects);
    assert_eq!(counter.view(), loading_view());

    drop(command.step().await?.expect_effect().expect_database());
    let outcome = command.step().await?.expect_outcome();
    assert_eq!(CounterOutcome::Loaded { counter: 0 }, outcome);
    command.step().await?.expect_done();
    Ok(())
}

#[tokio::test]
async fn a_dropped_write_response_keeps_the_count() -> anyhow::Result<()> {
    let (mut counter, mut effects) = ready_counter(0).await?;
    let mut command = counter
        .update(Message::Event(CounterEvent::Increase))
        .driver(&mut effects);
    assert_eq!(counter.view(), ready_view(1));

    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    drop(command.step().await?.expect_effect().expect_database());
    command.step().await?.expect_done();
    Ok(())
}

#[tokio::test]
#[should_panic(expected = "Render::render transport failed: response channel closed")]
async fn a_dropped_render_response_panics() {
    let (mut counter, mut effects) = ready_counter(0).await.unwrap();
    let mut command = counter
        .update(Message::Event(CounterEvent::Increase))
        .driver(&mut effects);
    assert_eq!(counter.view(), ready_view(1));

    drop(
        command
            .step()
            .await
            .unwrap()
            .expect_effect()
            .expect_render(),
    );
    command.step().await.unwrap().expect_done();
}

// Test support

fn loading_view() -> CounterView {
    CounterView {
        count: 0,
        status: CounterStatus::Loading,
    }
}

fn ready_view(count: usize) -> CounterView {
    CounterView {
        count,
        status: CounterStatus::Ready,
    }
}

async fn ready_counter(count: usize) -> anyhow::Result<(Counter, CounterEffectStream)> {
    let (mut counter, mut effects) = Counter::test_fixture();
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
    Ok((counter, effects))
}
