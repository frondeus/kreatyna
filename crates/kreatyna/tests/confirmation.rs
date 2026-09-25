use futures::StreamExt;
use kreatyna::confirmation::{
    Confirmation, ConfirmationEvent, ConfirmationOutcome, ConfirmationView,
};
use krucyfiks::{Message, Widget};
use test_case::test_case;

#[tokio::test]
async fn initialization_keeps_confirmation_awaiting_a_decision() {
    let mut confirmation = Confirmation::new();
    assert_eq!(confirmation.view(), ConfirmationView::AwaitingDecision);
    assert!(confirmation.init().into_stream().next().await.is_none());
    assert_eq!(confirmation.view(), ConfirmationView::AwaitingDecision);
}

#[test_case(ConfirmationEvent::Confirm, ConfirmationOutcome::Confirmed; "confirm")]
#[test_case(ConfirmationEvent::Cancel, ConfirmationOutcome::Cancelled; "cancel")]
#[tokio::test]
async fn first_decision_is_recorded_synchronously_and_emitted_once(
    event: ConfirmationEvent,
    decision: ConfirmationOutcome,
) {
    let mut confirmation = Confirmation::new();
    let command = confirmation.update(Message::Event(event));
    let expected = ConfirmationView::Resolved { decision };
    assert_eq!(confirmation.view(), expected);

    // Neither choice can replace the decision before its command is polled.
    for next in [ConfirmationEvent::Confirm, ConfirmationEvent::Cancel] {
        assert!(
            confirmation
                .update(Message::Event(next))
                .into_stream()
                .next()
                .await
                .is_none()
        );
        assert_eq!(confirmation.view(), expected);
    }

    assert_eq!(command.into_stream().collect::<Vec<_>>().await, [decision]);
    for next in [ConfirmationEvent::Confirm, ConfirmationEvent::Cancel] {
        assert!(
            confirmation
                .update(Message::Event(next))
                .into_stream()
                .next()
                .await
                .is_none()
        );
    }
    assert!(confirmation.init().into_stream().next().await.is_none());
    assert_eq!(confirmation.view(), expected);
}

#[test_case(ConfirmationOutcome::Confirmed; "confirmed")]
#[test_case(ConfirmationOutcome::Cancelled; "cancelled")]
#[tokio::test]
async fn outcomes_do_not_change_awaiting_or_resolved_state(outcome: ConfirmationOutcome) {
    let mut confirmation = Confirmation::new();
    for resolve in [false, true] {
        if resolve {
            let _command = confirmation.update(Message::Event(ConfirmationEvent::Confirm));
        }
        let before = confirmation.view();
        assert!(
            confirmation
                .update(Message::Outcome(outcome))
                .into_stream()
                .next()
                .await
                .is_none()
        );
        assert_eq!(confirmation.view(), before);
    }
}
