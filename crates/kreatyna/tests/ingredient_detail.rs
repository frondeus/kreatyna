use anyhow::Context;
use kreatyna::{
    capabilities::ResolveRender,
    confirmation::{ConfirmationEvent, ConfirmationOutcome, ConfirmationView},
    ingredients::{
        capabilities::{
            IngredientDeleteError, IngredientEffect, IngredientEffectStream, ResolveIngredientStore,
        },
        *,
    },
};
use krucyfiks::{
    Latest, Message, Widget,
    effects::{CommandDriver, CommandTestExt, EnvelopeError},
    test_utils::EmptyInitTest,
};
use rust_decimal::Decimal;
use test_case::{test_case, test_matrix};
use uuid::Uuid;

// Green path

#[test_case(NutritionBasis::Per100Grams; "mass basis")]
#[test_case(NutritionBasis::Per100Millilitres; "volume basis")]
#[tokio::test]
async fn initialization_does_nothing_and_shows_the_complete_record(
    basis: NutritionBasis,
) -> anyhow::Result<()> {
    let original = ingredient(basis)?;
    let mut fixture = DetailFixture::new(original.clone());
    let view = fixture.detail.view();
    assert_eq!(
        view,
        IngredientDetailView {
            ingredient: original,
            deletion: IngredientDetailDeletion::Idle,
        }
    );
    // Equality alone would not detect lost decimal scale.
    let calories = view.ingredient.nutrition.calories.context("no calories")?;
    assert_eq!(calories.to_string(), "12.3400");
    let cup = view.ingredient.units[0]
        .millilitres
        .context("no cup millilitres")?;
    assert_eq!(cup.to_string(), "250.00");
    fixture.detail.expect_noop_init().await;
    Ok(())
}

#[tokio::test]
async fn editing_emits_the_complete_record_without_changing_the_view() -> anyhow::Result<()> {
    let original = ingredient(NutritionBasis::Per100Grams)?;
    let mut fixture = DetailFixture::new(original.clone());
    let before = fixture.detail.view();
    let mut command = fixture
        .detail
        .update(Message::Event(IngredientDetailEvent::Edit))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.detail.view(), before);

    assert_eq!(
        command.step().await?.expect_outcome(),
        IngredientDetailOutcome::OpenEdit {
            ingredient: original
        }
    );
    command.step().await?.expect_done();
    Ok(())
}

#[tokio::test]
async fn requesting_deletion_asks_for_confirmation() -> anyhow::Result<()> {
    let original = ingredient(NutritionBasis::Per100Grams)?;
    let mut fixture = DetailFixture::new(original.clone());
    let mut command = fixture
        .detail
        .update(Message::Event(IngredientDetailEvent::RequestDelete))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.detail.view(),
        IngredientDetailView {
            ingredient: original,
            deletion: confirming(None),
        }
    );

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

#[tokio::test]
async fn cancelling_deletion_returns_to_idle_without_contacting_the_store() -> anyhow::Result<()> {
    let original = ingredient(NutritionBasis::Per100Grams)?;
    let mut fixture = DetailFixture::new(original.clone());
    fixture.request_deletion().await?;

    let mut command = fixture
        .detail
        .update(Message::Event(IngredientDetailEvent::Confirmation(
            ConfirmationEvent::Cancel,
        )))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.detail.view().deletion,
        resolved(ConfirmationOutcome::Cancelled, None)
    );
    let decision = command.step().await?.expect_outcome();
    assert!(matches!(
        decision,
        IngredientDetailOutcome::ConfirmationDecided(Latest {
            value: ConfirmationOutcome::Cancelled,
            ..
        })
    ));
    command.step().await?.expect_done();
    drop(command);

    let mut command = fixture
        .detail
        .update(Message::Outcome(decision))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.detail.view(),
        IngredientDetailView {
            ingredient: original,
            deletion: IngredientDetailDeletion::Idle,
        }
    );
    // Only a render follows. The store is not asked to delete.
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

#[tokio::test]
async fn confirming_deletion_deletes_the_record_and_reports_its_id() -> anyhow::Result<()> {
    let original = ingredient(NutritionBasis::Per100Grams)?;
    let mut fixture = DetailFixture::new(original.clone());
    fixture.request_deletion().await?;

    let mut command = fixture
        .detail
        .update(Message::Event(IngredientDetailEvent::Confirmation(
            ConfirmationEvent::Confirm,
        )))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.detail.view().deletion,
        resolved(ConfirmationOutcome::Confirmed, None)
    );
    let decision = command.step().await?.expect_outcome();
    assert!(matches!(
        decision,
        IngredientDetailOutcome::ConfirmationDecided(Latest {
            value: ConfirmationOutcome::Confirmed,
            ..
        })
    ));
    command.step().await?.expect_done();
    drop(command);

    let mut command = fixture
        .detail
        .update(Message::Outcome(decision))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.detail.view(),
        IngredientDetailView {
            ingredient: original.clone(),
            deletion: IngredientDetailDeletion::Deleting,
        }
    );
    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    let mut requested = None;
    command
        .step()
        .await?
        .expect_effect()
        .expect_store()
        .resolve_delete(|id| {
            requested = Some(id);
            async { Ok(()) }
        })
        .await?;
    let outcome = command.step().await?.expect_outcome();
    command.step().await?.expect_done();
    drop(command);

    assert_eq!(requested, Some(original.id));
    assert!(matches!(
        outcome,
        IngredientDetailOutcome::DeleteFinished(Latest { value: Ok(()), .. })
    ));

    let mut command = fixture
        .detail
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    // The record stays visible until the parent closes the detail.
    assert_eq!(
        fixture.detail.view(),
        IngredientDetailView {
            ingredient: original.clone(),
            deletion: IngredientDetailDeletion::Deleted,
        }
    );
    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    assert_eq!(
        command.step().await?.expect_outcome(),
        IngredientDetailOutcome::Deleted { id: original.id }
    );
    command.step().await?.expect_done();
    Ok(())
}

// Validation

#[test_case(ConfirmationEvent::Confirm; "confirm")]
#[test_case(ConfirmationEvent::Cancel; "cancel")]
#[tokio::test]
async fn confirmation_events_while_idle_are_ignored(
    event: ConfirmationEvent,
) -> anyhow::Result<()> {
    let mut fixture = DetailFixture::new(ingredient(NutritionBasis::Per100Grams)?);
    let before = fixture.detail.view();
    let mut command = fixture
        .detail
        .update(Message::Event(IngredientDetailEvent::Confirmation(event)))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.detail.view(), before);
    command.step().await?.expect_done();
    Ok(())
}

#[test_case(IngredientDetailEvent::Edit; "edit")]
#[test_case(IngredientDetailEvent::RequestDelete; "delete again")]
#[tokio::test]
async fn edit_and_repeated_delete_requests_while_confirming_are_ignored(
    event: IngredientDetailEvent,
) -> anyhow::Result<()> {
    let mut fixture = DetailFixture::new(ingredient(NutritionBasis::Per100Grams)?);
    fixture.request_deletion().await?;

    let before = fixture.detail.view();
    let mut command = fixture
        .detail
        .update(Message::Event(event))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.detail.view(), before);
    command.step().await?.expect_done();
    Ok(())
}

#[test_matrix(
    [
        (ConfirmationEvent::Confirm, ConfirmationOutcome::Confirmed),
        (ConfirmationEvent::Cancel, ConfirmationOutcome::Cancelled)
    ],
    [ConfirmationEvent::Confirm, ConfirmationEvent::Cancel]
)]
#[tokio::test]
async fn a_second_decision_before_the_first_is_applied_is_ignored(
    (first, expected): (ConfirmationEvent, ConfirmationOutcome),
    second: ConfirmationEvent,
) -> anyhow::Result<()> {
    let mut fixture = DetailFixture::new(ingredient(NutritionBasis::Per100Grams)?);
    fixture.request_deletion().await?;
    let pending = fixture
        .detail
        .update(Message::Event(IngredientDetailEvent::Confirmation(first)));
    let resolved_view = fixture.detail.view();
    assert_eq!(resolved_view.deletion, resolved(expected, None));

    let mut command = fixture
        .detail
        .update(Message::Event(IngredientDetailEvent::Confirmation(second)))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.detail.view(), resolved_view);
    command.step().await?.expect_done();
    drop(command);

    // The first decision is still reported.
    let mut pending = pending.driver(&mut fixture.effects);
    let decision = pending.step().await?.expect_outcome();
    assert!(matches!(
        decision,
        IngredientDetailOutcome::ConfirmationDecided(Latest { value: decision, .. }) if decision == expected
    ));
    pending.step().await?.expect_done();
    Ok(())
}

#[tokio::test]
async fn a_cancel_decision_delivered_twice_is_applied_once() -> anyhow::Result<()> {
    let mut fixture = DetailFixture::new(ingredient(NutritionBasis::Per100Grams)?);
    fixture.request_deletion().await?;
    let cancelled = fixture.decide(ConfirmationEvent::Cancel).await?;
    fixture.apply_outcome(cancelled.clone()).await?;
    assert_eq!(
        fixture.detail.view().deletion,
        IngredientDetailDeletion::Idle
    );

    let before = fixture.detail.view();
    let mut command = fixture
        .detail
        .update(Message::Outcome(cancelled))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.detail.view(), before);
    command.step().await?.expect_done();
    Ok(())
}

#[tokio::test]
async fn a_cancel_decision_from_an_earlier_dialog_does_not_close_a_reopened_dialog()
-> anyhow::Result<()> {
    let mut fixture = DetailFixture::new(ingredient(NutritionBasis::Per100Grams)?);
    fixture.request_deletion().await?;
    let cancelled = fixture.decide(ConfirmationEvent::Cancel).await?;
    fixture.apply_outcome(cancelled.clone()).await?;
    fixture.request_deletion().await?;
    assert_eq!(fixture.detail.view().deletion, confirming(None));

    let before = fixture.detail.view();
    let mut command = fixture
        .detail
        .update(Message::Outcome(cancelled))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.detail.view(), before);
    command.step().await?.expect_done();
    Ok(())
}

#[test_case(IngredientDetailEvent::Edit; "edit")]
#[test_case(IngredientDetailEvent::RequestDelete; "delete again")]
#[test_case(IngredientDetailEvent::Confirmation(ConfirmationEvent::Confirm); "confirm")]
#[test_case(IngredientDetailEvent::Confirmation(ConfirmationEvent::Cancel); "cancel")]
#[tokio::test]
async fn events_while_deleting_are_ignored(event: IngredientDetailEvent) -> anyhow::Result<()> {
    let mut fixture = DetailFixture::new(ingredient(NutritionBasis::Per100Grams)?);
    fixture.request_deletion().await?;
    let confirmed = fixture.decide(ConfirmationEvent::Confirm).await?;
    let deletion = fixture.detail.update(Message::Outcome(confirmed));
    assert_eq!(
        fixture.detail.view().deletion,
        IngredientDetailDeletion::Deleting
    );

    let before = fixture.detail.view();
    let mut command = fixture
        .detail
        .update(Message::Event(event))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.detail.view(), before);
    command.step().await?.expect_done();
    drop(deletion);
    Ok(())
}

#[test_case(IngredientDetailEvent::Edit; "edit")]
#[test_case(IngredientDetailEvent::RequestDelete; "delete again")]
#[test_case(IngredientDetailEvent::Confirmation(ConfirmationEvent::Confirm); "confirm")]
#[test_case(IngredientDetailEvent::Confirmation(ConfirmationEvent::Cancel); "cancel")]
#[tokio::test]
async fn events_after_deletion_are_ignored(event: IngredientDetailEvent) -> anyhow::Result<()> {
    let mut fixture = DetailFixture::new(ingredient(NutritionBasis::Per100Grams)?);
    fixture.request_deletion().await?;
    let confirmed = fixture.decide(ConfirmationEvent::Confirm).await?;
    let finished = fixture.delete(confirmed, Ok(())).await?;
    fixture.apply_outcome(finished).await?;
    assert_eq!(
        fixture.detail.view().deletion,
        IngredientDetailDeletion::Deleted
    );

    let before = fixture.detail.view();
    let mut command = fixture
        .detail
        .update(Message::Event(event))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.detail.view(), before);
    command.step().await?.expect_done();
    Ok(())
}

#[tokio::test]
async fn a_deletion_result_delivered_twice_is_applied_once() -> anyhow::Result<()> {
    let mut fixture = DetailFixture::new(ingredient(NutritionBasis::Per100Grams)?);
    fixture.request_deletion().await?;
    let confirmed = fixture.decide(ConfirmationEvent::Confirm).await?;
    let finished = fixture.delete(confirmed, Ok(())).await?;
    fixture.apply_outcome(finished.clone()).await?;
    assert_eq!(
        fixture.detail.view().deletion,
        IngredientDetailDeletion::Deleted
    );

    let before = fixture.detail.view();
    let mut command = fixture
        .detail
        .update(Message::Outcome(finished))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.detail.view(), before);
    command.step().await?.expect_done();
    Ok(())
}

#[tokio::test]
async fn dropping_the_detail_cancels_deletion_before_the_store_request() -> anyhow::Result<()> {
    let mut fixture = DetailFixture::new(ingredient(NutritionBasis::Per100Grams)?);
    fixture.request_deletion().await?;
    let confirmed = fixture.decide(ConfirmationEvent::Confirm).await?;
    let mut command = fixture
        .detail
        .update(Message::Outcome(confirmed))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.detail.view().deletion,
        IngredientDetailDeletion::Deleting
    );
    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;

    drop(fixture.detail);
    command.step().await?.expect_done();
    Ok(())
}

#[tokio::test]
async fn dropping_the_detail_cancels_an_in_flight_store_request() -> anyhow::Result<()> {
    let mut fixture = DetailFixture::new(ingredient(NutritionBasis::Per100Grams)?);
    fixture.request_deletion().await?;
    let confirmed = fixture.decide(ConfirmationEvent::Confirm).await?;
    let mut command = fixture
        .detail
        .update(Message::Outcome(confirmed))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.detail.view().deletion,
        IngredientDetailDeletion::Deleting
    );
    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    let request = command.step().await?.expect_effect().expect_store();

    drop(fixture.detail);
    command.step().await?.expect_done();
    drop(command);
    // Nobody waits for the answer any more.
    assert!(matches!(
        request.resolve_delete(|_| async { Ok(()) }).await,
        Err(EnvelopeError::Send(_))
    ));
    Ok(())
}

// Red cases

#[test_case(IngredientDeleteError::Storage; "storage failure")]
#[test_case(IngredientDeleteError::Transport; "transport failure")]
#[tokio::test]
async fn a_failed_deletion_keeps_the_record_and_reopens_confirmation_with_the_error(
    error: IngredientDeleteError,
) -> anyhow::Result<()> {
    let original = ingredient(NutritionBasis::Per100Grams)?;
    let mut fixture = DetailFixture::new(original.clone());
    fixture.request_deletion().await?;
    let confirmed = fixture.decide(ConfirmationEvent::Confirm).await?;
    let mut command = fixture
        .detail
        .update(Message::Outcome(confirmed))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.detail.view().deletion,
        IngredientDetailDeletion::Deleting
    );
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
        .expect_store()
        .resolve_delete(|_| async { Err(error.clone()) })
        .await?;
    let outcome = command.step().await?.expect_outcome();
    command.step().await?.expect_done();
    drop(command);

    let mut command = fixture
        .detail
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.detail.view(),
        IngredientDetailView {
            ingredient: original,
            deletion: confirming(Some(error)),
        }
    );
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

#[tokio::test]
async fn a_dropped_store_response_reports_a_transport_failure() -> anyhow::Result<()> {
    let mut fixture = DetailFixture::new(ingredient(NutritionBasis::Per100Grams)?);
    fixture.request_deletion().await?;
    let confirmed = fixture.decide(ConfirmationEvent::Confirm).await?;
    let mut command = fixture
        .detail
        .update(Message::Outcome(confirmed))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.detail.view().deletion,
        IngredientDetailDeletion::Deleting
    );
    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    drop(command.step().await?.expect_effect().expect_store());
    let outcome = command.step().await?.expect_outcome();
    assert!(matches!(
        outcome,
        IngredientDetailOutcome::DeleteFinished(Latest {
            value: Err(IngredientDeleteError::Transport),
            ..
        })
    ));
    command.step().await?.expect_done();
    drop(command);

    let mut command = fixture
        .detail
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.detail.view().deletion,
        confirming(Some(IngredientDeleteError::Transport))
    );
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

#[test_case(IngredientDeleteError::Storage; "after storage failure")]
#[test_case(IngredientDeleteError::Transport; "after transport failure")]
#[tokio::test]
async fn retrying_after_a_failed_deletion_can_succeed(
    error: IngredientDeleteError,
) -> anyhow::Result<()> {
    let original = ingredient(NutritionBasis::Per100Grams)?;
    let mut fixture = DetailFixture::new(original.clone());
    fixture.fail_deletion(error.clone()).await?;

    let mut command = fixture
        .detail
        .update(Message::Event(IngredientDetailEvent::Confirmation(
            ConfirmationEvent::Confirm,
        )))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.detail.view().deletion,
        resolved(ConfirmationOutcome::Confirmed, Some(error))
    );
    let retry = command.step().await?.expect_outcome();
    command.step().await?.expect_done();
    drop(command);

    let mut command = fixture
        .detail
        .update(Message::Outcome(retry))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.detail.view().deletion,
        IngredientDetailDeletion::Deleting
    );
    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    let mut requested = None;
    command
        .step()
        .await?
        .expect_effect()
        .expect_store()
        .resolve_delete(|id| {
            requested = Some(id);
            async { Ok(()) }
        })
        .await?;
    let outcome = command.step().await?.expect_outcome();
    command.step().await?.expect_done();
    drop(command);
    assert_eq!(requested, Some(original.id));

    let mut command = fixture
        .detail
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.detail.view().deletion,
        IngredientDetailDeletion::Deleted
    );
    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    assert_eq!(
        command.step().await?.expect_outcome(),
        IngredientDetailOutcome::Deleted { id: original.id }
    );
    command.step().await?.expect_done();
    Ok(())
}

#[test_case(IngredientDeleteError::Storage; "after storage failure")]
#[test_case(IngredientDeleteError::Transport; "after transport failure")]
#[tokio::test]
async fn cancelling_after_a_failed_deletion_clears_the_error(
    error: IngredientDeleteError,
) -> anyhow::Result<()> {
    let original = ingredient(NutritionBasis::Per100Grams)?;
    let mut fixture = DetailFixture::new(original.clone());
    fixture.fail_deletion(error).await?;
    let cancelled = fixture.decide(ConfirmationEvent::Cancel).await?;

    let mut command = fixture
        .detail
        .update(Message::Outcome(cancelled))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.detail.view(),
        IngredientDetailView {
            ingredient: original,
            deletion: IngredientDetailDeletion::Idle,
        }
    );
    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command.step().await?.expect_done();
    drop(command);

    // A new request starts without the old error.
    let mut command = fixture
        .detail
        .update(Message::Event(IngredientDetailEvent::RequestDelete))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.detail.view().deletion, confirming(None));
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

#[tokio::test]
async fn a_stale_decision_is_ignored_after_a_failed_deletion_reopens_confirmation()
-> anyhow::Result<()> {
    let mut fixture = DetailFixture::new(ingredient(NutritionBasis::Per100Grams)?);
    fixture.request_deletion().await?;
    let confirmed = fixture.decide(ConfirmationEvent::Confirm).await?;
    let failed = fixture
        .delete(confirmed.clone(), Err(IngredientDeleteError::Storage))
        .await?;
    fixture.apply_outcome(failed).await?;
    assert_eq!(
        fixture.detail.view().deletion,
        confirming(Some(IngredientDeleteError::Storage))
    );

    let before = fixture.detail.view();
    let mut command = fixture
        .detail
        .update(Message::Outcome(confirmed))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.detail.view(), before);
    command.step().await?.expect_done();
    Ok(())
}

#[tokio::test]
async fn a_stale_failure_is_ignored_while_the_retry_is_deleting() -> anyhow::Result<()> {
    let original = ingredient(NutritionBasis::Per100Grams)?;
    let mut fixture = DetailFixture::new(original.clone());
    fixture.request_deletion().await?;
    let confirmed = fixture.decide(ConfirmationEvent::Confirm).await?;
    let failed = fixture
        .delete(confirmed, Err(IngredientDeleteError::Storage))
        .await?;
    fixture.apply_outcome(failed.clone()).await?;
    let retry = fixture.decide(ConfirmationEvent::Confirm).await?;
    let deletion = fixture.detail.update(Message::Outcome(retry));
    let deleting = fixture.detail.view();
    assert_eq!(deleting.deletion, IngredientDetailDeletion::Deleting);

    let mut command = fixture
        .detail
        .update(Message::Outcome(failed))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.detail.view(), deleting);
    command.step().await?.expect_done();
    drop(command);

    // The retry still completes.
    let mut deletion = deletion.driver(&mut fixture.effects);
    deletion
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    deletion
        .step()
        .await?
        .expect_effect()
        .expect_store()
        .resolve_delete(|_| async { Ok(()) })
        .await?;
    let outcome = deletion.step().await?.expect_outcome();
    deletion.step().await?.expect_done();
    drop(deletion);

    let mut command = fixture
        .detail
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.detail.view().deletion,
        IngredientDetailDeletion::Deleted
    );
    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    assert_eq!(
        command.step().await?.expect_outcome(),
        IngredientDetailOutcome::Deleted { id: original.id }
    );
    command.step().await?.expect_done();
    Ok(())
}

// Test support

struct DetailFixture {
    detail: IngredientDetail,
    effects: IngredientEffectStream,
}

impl DetailFixture {
    fn new(ingredient: Ingredient) -> Self {
        let (detail, effects) = IngredientDetail::test_fixture(ingredient);
        Self { detail, effects }
    }

    /// Setup only: open the deletion confirmation.
    async fn request_deletion(&mut self) -> anyhow::Result<()> {
        let mut command = self
            .detail
            .update(Message::Event(IngredientDetailEvent::RequestDelete))
            .driver(&mut self.effects);
        render_then_finish(&mut command).await
    }

    /// Setup only: make a decision and return it without applying it.
    async fn decide(
        &mut self,
        event: ConfirmationEvent,
    ) -> anyhow::Result<IngredientDetailOutcome> {
        let mut command = self
            .detail
            .update(Message::Event(IngredientDetailEvent::Confirmation(event)))
            .driver(&mut self.effects);
        let outcome = command.step().await?.expect_outcome();
        command.step().await?.expect_done();
        Ok(outcome)
    }

    /// Setup only: apply a confirmed decision and answer the store request.
    /// Returns the deletion result without applying it.
    async fn delete(
        &mut self,
        confirmed: IngredientDetailOutcome,
        result: Result<(), IngredientDeleteError>,
    ) -> anyhow::Result<IngredientDetailOutcome> {
        let mut command = self
            .detail
            .update(Message::Outcome(confirmed))
            .driver(&mut self.effects);
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
            .expect_store()
            .resolve_delete(|_| async { result })
            .await?;
        let outcome = command.step().await?.expect_outcome();
        command.step().await?.expect_done();
        Ok(outcome)
    }

    /// Setup only: apply an outcome whose command renders once.
    async fn apply_outcome(&mut self, outcome: IngredientDetailOutcome) -> anyhow::Result<()> {
        let mut command = self
            .detail
            .update(Message::Outcome(outcome))
            .driver(&mut self.effects);
        render_then_finish(&mut command).await
    }

    /// Setup only: request deletion, confirm it, and apply a store failure.
    async fn fail_deletion(&mut self, error: IngredientDeleteError) -> anyhow::Result<()> {
        self.request_deletion().await?;
        let confirmed = self.decide(ConfirmationEvent::Confirm).await?;
        let failed = self.delete(confirmed, Err(error)).await?;
        self.apply_outcome(failed).await
    }
}

/// Setup only: resolve one render, then finish while ignoring outcomes.
async fn render_then_finish(
    command: &mut CommandDriver<'_, IngredientEffect, IngredientDetailOutcome>,
) -> anyhow::Result<()> {
    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command.until_done_ignoring_outcomes().await?;
    Ok(())
}

fn confirming(error: Option<IngredientDeleteError>) -> IngredientDetailDeletion {
    IngredientDetailDeletion::Confirming {
        confirmation: ConfirmationView::AwaitingDecision,
        error,
    }
}

fn resolved(
    decision: ConfirmationOutcome,
    error: Option<IngredientDeleteError>,
) -> IngredientDetailDeletion {
    IngredientDetailDeletion::Confirming {
        confirmation: ConfirmationView::Resolved { decision },
        error,
    }
}

fn ingredient(basis: NutritionBasis) -> anyhow::Result<Ingredient> {
    Ok(Ingredient {
        id: Uuid::from_u128(1).into(),
        name: "Garlic".into(),
        notes: Some("Fresh".into()),
        nutrition: Nutrition {
            basis,
            calories: Some(Decimal::new(123_400, 4).try_into()?),
            protein: None,
            carbohydrates: Some(Decimal::ZERO.try_into()?),
            fat: None,
        },
        units: vec![
            ReferenceUnit {
                name: "cup".into(),
                grams: None,
                millilitres: Some(Decimal::new(25_000, 2).try_into()?),
            },
            ReferenceUnit {
                name: "clove".into(),
                grams: Some(Decimal::ONE.try_into()?),
                millilitres: None,
            },
        ],
    })
}
