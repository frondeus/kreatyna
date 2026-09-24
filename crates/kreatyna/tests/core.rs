use krucyfiks::{Message, Widget, effects::CommandTestExt};
use kreatyna::capabilities::{DatabaseError, ResolveDatabase, ResolveRender};
use kreatyna::{Counter, CounterEvent, CounterOutcome};
use test_case::{test_case, test_matrix};

#[test_case(Ok(None), 0; "missing row")]
#[test_case(Ok(Some(0)), 0; "stored zero")]
#[test_case(Ok(Some(42)), 42; "stored count")]
#[test_case(Ok(Some(usize::MAX)), usize::MAX; "stored maximum")]
#[test_case(Err(DatabaseError::Storage), 0; "storage failure")]
#[test_case(Err(DatabaseError::OutOfRange), 0; "out of range")]
#[test_case(Err(DatabaseError::Transport), 0; "transport failure")]
#[tokio::test]
async fn init_loads_without_changing_state_or_writing_to_database(
    reply: Result<Option<usize>, DatabaseError>,
    expected: usize,
) -> anyhow::Result<()> {
    let (counter, mut effects) = Counter::test_fixture();
    assert_eq!(counter.view(), 0);
    let mut command = counter.init().driver(&mut effects);

    command
        .drive()
        .await?
        .expect_effect()
        .expect_database()
        .resolve_count(|| async move { reply })
        .await?;
    let outcome = command.drive().await?.expect_outcome();
    assert!(matches!(outcome, CounterOutcome::Loaded { counter } if counter == expected));
    assert_eq!(counter.view(), 0);
    command.drive().await?.expect_done();
    Ok(())
}

#[test_case(19, 0; "replace with zero")]
#[test_case(0, usize::MAX; "replace with maximum")]
#[test_case(usize::MAX, 1; "replace maximum")]
#[test_case(42, 42; "unchanged value still renders")]
#[tokio::test]
async fn loaded_replaces_state_and_only_renders(
    initial: usize,
    value: usize,
) -> anyhow::Result<()> {
    let (mut counter, mut effects) = Counter::test_fixture();
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

    let mut command = counter
        .update(Message::Outcome(CounterOutcome::Loaded { counter: value }))
        .driver(&mut effects);
    assert_eq!(counter.view(), value);
    command
        .drive()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command.drive().await?.expect_done();
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
async fn mutation_changes_state_then_renders_then_persists(
    initial: usize,
    event: CounterEvent,
    expected: usize,
) -> anyhow::Result<()> {
    let (mut counter, mut effects) = Counter::test_fixture();
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

    let mut command = counter.update(Message::Event(event)).driver(&mut effects);
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
    assert_eq!(counter.view(), expected);
    Ok(())
}

#[test_matrix(
    [(CounterEvent::Increase, 11), (CounterEvent::Decrease, 9), (CounterEvent::Reset, 0)],
    [DatabaseError::Storage, DatabaseError::OutOfRange, DatabaseError::Transport]
)]
#[tokio::test]
async fn failed_write_preserves_state_and_allows_next_event(
    (event, expected): (CounterEvent, usize),
    error: DatabaseError,
) -> anyhow::Result<()> {
    let (mut counter, mut effects) = Counter::test_fixture();
    let mut setup = counter
        .update(Message::Outcome(CounterOutcome::Loaded { counter: 10 }))
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
            Err(error)
        })
        .await?;
    command.drive().await?.expect_done();
    assert_eq!(counter.view(), expected);
    drop(command);

    let mut next = counter
        .update(Message::Event(CounterEvent::Increase))
        .driver(&mut effects);
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
            assert_eq!(count, expected + 1);
            Ok(())
        })
        .await?;
    next.drive().await?.expect_done();
    assert_eq!(counter.view(), expected + 1);
    Ok(())
}

#[tokio::test]
async fn delayed_commands_persist_each_events_captured_value() -> anyhow::Result<()> {
    let (mut counter, mut effects) = Counter::test_fixture();
    let increase = counter.update(Message::Event(CounterEvent::Increase));
    let second_increase = counter.update(Message::Event(CounterEvent::Increase));
    let reset = counter.update(Message::Event(CounterEvent::Reset));
    assert_eq!(counter.view(), 0);

    // These are consecutive steps of one scenario, not parameterized cases.
    for (command, expected) in [(increase, 1), (second_increase, 2), (reset, 0)] {
        let mut command = command.driver(&mut effects);
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
        assert_eq!(counter.view(), 0);
    }
    Ok(())
}

#[tokio::test]
async fn dropped_database_read_returns_loaded_zero() -> anyhow::Result<()> {
    let (counter, mut effects) = Counter::test_fixture();
    let mut command = counter.init().driver(&mut effects);
    drop(command.drive().await?.expect_effect().expect_database());
    assert!(matches!(
        command.drive().await?.expect_outcome(),
        CounterOutcome::Loaded { counter: 0 }
    ));
    command.drive().await?.expect_done();
    Ok(())
}

#[tokio::test]
async fn dropped_database_write_preserves_state() -> anyhow::Result<()> {
    let (mut counter, mut effects) = Counter::test_fixture();
    let mut command = counter
        .update(Message::Event(CounterEvent::Increase))
        .driver(&mut effects);
    command
        .drive()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    drop(command.drive().await?.expect_effect().expect_database());
    command.drive().await?.expect_done();
    assert_eq!(counter.view(), 1);
    Ok(())
}

#[tokio::test]
async fn dropping_mutation_while_rendering_cancels_persistence_but_keeps_state()
-> anyhow::Result<()> {
    let (mut counter, mut effects) = Counter::test_fixture();
    let mut command = counter
        .update(Message::Event(CounterEvent::Increase))
        .driver(&mut effects);
    let render = command.drive().await?.expect_effect().expect_render();
    drop(command);
    assert!(matches!(
        render.resolve_render(|| async {}).await,
        Err(krucyfiks::effects::EnvelopeError::Send(_))
    ));
    assert_eq!(counter.view(), 1);

    let mut next = counter
        .update(Message::Event(CounterEvent::Increase))
        .driver(&mut effects);
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
    Ok(())
}

#[tokio::test]
#[should_panic(expected = "Render::render transport failed: response channel closed")]
async fn dropped_render_response_panics() {
    let (mut counter, mut effects) = Counter::test_fixture();
    let mut command = counter
        .update(Message::Event(CounterEvent::Increase))
        .driver(&mut effects);
    drop(
        command
            .drive()
            .await
            .unwrap()
            .expect_effect()
            .expect_render(),
    );
    command.drive().await.unwrap().expect_done();
}
