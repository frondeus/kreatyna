use kreatyna::confirmation::{
    Confirmation, ConfirmationEvent, ConfirmationOutcome, ConfirmationView,
};
use krucyfiks::{Message, Widget, effects::CommandTestExt, test_utils::EmptyInitTest};
use test_case::{test_case, test_matrix};

#[tokio::test]
async fn confirmation_widget_has_awaiting_decision_initial_state() -> anyhow::Result<()> {
    let mut confirmation = Confirmation::new();

    assert_eq!(confirmation.view(), ConfirmationView::AwaitingDecision);

    confirmation.expect_noop_init().await;

    assert_eq!(confirmation.view(), ConfirmationView::AwaitingDecision);

    Ok(())
}

#[test_case(ConfirmationEvent::Confirm, ConfirmationOutcome::Confirmed; "confirm")]
#[test_case(ConfirmationEvent::Cancel, ConfirmationOutcome::Cancelled; "cancel")]
#[tokio::test]
async fn decision_changes_state_and_creates_an_outcome(
    event: ConfirmationEvent,
    expected_outcome: ConfirmationOutcome,
) -> anyhow::Result<()> {
    let mut confirmation = Confirmation::new();
    let mut driver = confirmation.update(Message::Event(event)).pure_driver();

    assert_eq!(
        confirmation.view(),
        ConfirmationView::Resolved {
            decision: expected_outcome
        }
    );

    let outcome = driver.step().await?.expect_outcome();
    assert_eq!(expected_outcome, outcome);
    driver.step().await?.expect_done();
    Ok(())
}

#[test_matrix(
    [ConfirmationEvent::Confirm, ConfirmationEvent::Cancel],
    [ConfirmationEvent::Confirm, ConfirmationEvent::Cancel]
)]
#[tokio::test]
async fn repeated_choices_are_ignored_before_the_command_completes(
    first: ConfirmationEvent,
    second: ConfirmationEvent,
) -> anyhow::Result<()> {
    let mut confirmation = Confirmation::new();
    let mut command = confirmation.update(Message::Event(first)).pure_driver();
    let expected = ConfirmationView::Resolved {
        decision: expected_outcome(first),
    };
    assert_eq!(confirmation.view(), expected);

    confirmation
        .update(Message::Event(second))
        .pure_driver()
        .step()
        .await?
        .expect_done(); // Second event produces no outcome.

    assert_eq!(confirmation.view(), expected);

    command.until_done_ignoring_outcomes().await?;

    Ok(())
}

#[test_matrix(
    [ConfirmationEvent::Confirm, ConfirmationEvent::Cancel],
    [ConfirmationEvent::Confirm, ConfirmationEvent::Cancel]
)]
#[tokio::test]
async fn repeated_choices_are_ignored_after_the_command_completes(
    first: ConfirmationEvent,
    second: ConfirmationEvent,
) -> anyhow::Result<()> {
    let mut confirmation = Confirmation::new();
    confirmation
        .update(Message::Event(first))
        .pure_driver()
        .until_done_ignoring_outcomes()
        .await?;

    let expected = ConfirmationView::Resolved {
        decision: expected_outcome(first),
    };
    assert_eq!(confirmation.view(), expected);

    confirmation
        .update(Message::Event(second))
        .pure_driver()
        .step()
        .await?
        .expect_done(); // Second event produces no outcome.

    assert_eq!(confirmation.view(), expected);

    Ok(())
}

#[test_case(ConfirmationOutcome::Confirmed; "confirmed")]
#[test_case(ConfirmationOutcome::Cancelled; "cancelled")]
#[tokio::test]
async fn outcomes_do_not_change_awaiting_state(outcome: ConfirmationOutcome) -> anyhow::Result<()> {
    let mut confirmation = Confirmation::new();
    confirmation
        .update(Message::Outcome(outcome))
        .pure_driver()
        .step()
        .await?
        .expect_done();
    assert_eq!(confirmation.view(), ConfirmationView::AwaitingDecision);
    Ok(())
}

#[test_matrix(
    [ConfirmationEvent::Confirm, ConfirmationEvent::Cancel],
    [ConfirmationOutcome::Confirmed, ConfirmationOutcome::Cancelled]
)]
#[tokio::test]
async fn outcomes_do_not_change_resolved_state(
    initial: ConfirmationEvent,
    outcome: ConfirmationOutcome,
) -> anyhow::Result<()> {
    let mut confirmation = Confirmation::new();
    confirmation
        .update(Message::Event(initial))
        .pure_driver()
        .until_done_ignoring_outcomes()
        .await?;
    confirmation
        .update(Message::Outcome(outcome))
        .pure_driver()
        .step()
        .await?
        .expect_done();
    assert_eq!(
        confirmation.view(),
        ConfirmationView::Resolved {
            decision: expected_outcome(initial)
        }
    );
    Ok(())
}

fn expected_outcome(event: ConfirmationEvent) -> ConfirmationOutcome {
    match event {
        ConfirmationEvent::Confirm => ConfirmationOutcome::Confirmed,
        ConfirmationEvent::Cancel => ConfirmationOutcome::Cancelled,
    }
}
