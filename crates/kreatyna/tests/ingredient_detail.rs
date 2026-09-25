use futures::StreamExt;
use kreatyna::{
    capabilities::ResolveRender,
    confirmation::{ConfirmationEvent, ConfirmationOutcome, ConfirmationView},
    ingredients::{
        capabilities::{IngredientDeleteError, IngredientEffectStream, ResolveIngredientStore},
        *,
    },
};
use krucyfiks::{Command, Message, Widget, effects::CommandTestExt};
use rust_decimal::Decimal;
use test_case::test_case;

#[test_case(NutritionBasis::Per100Grams; "mass")]
#[test_case(NutritionBasis::Per100Millilitres; "volume")]
#[tokio::test]
async fn initial_view_and_edit_preserve_complete_record(basis: NutritionBasis) {
    let original = ingredient(basis);
    let mut fixture = DetailFixture::new(original.clone());
    assert_eq!(fixture.detail.view().ingredient, original);
    assert_eq!(
        fixture.detail.view().deletion,
        IngredientDetailDeletion::Idle
    );
    assert_empty(fixture.detail.init()).await;
    // Equality alone would not detect lost decimal scale.
    assert_eq!(
        fixture
            .detail
            .view()
            .ingredient
            .nutrition
            .calories
            .unwrap()
            .to_string(),
        "12.3400"
    );
    assert_eq!(
        fixture.detail.view().ingredient.units[0]
            .millilitres
            .unwrap()
            .to_string(),
        "250.00"
    );
    let outcome = fixture
        .detail
        .update(Message::Event(IngredientDetailEvent::Edit))
        .into_stream()
        .collect::<Vec<_>>()
        .await;
    assert_eq!(
        outcome,
        [IngredientDetailOutcome::OpenEdit {
            ingredient: original
        }]
    );
    assert_empty(fixture.detail.update(Message::Outcome(outcome[0].clone()))).await;
    assert_empty(
        fixture
            .detail
            .update(Message::Outcome(IngredientDetailOutcome::Deleted {
                id: fixture.detail.view().ingredient.id,
            })),
    )
    .await;
    assert_eq!(
        fixture.detail.view().deletion,
        IngredientDetailDeletion::Idle
    );
}

#[tokio::test]
async fn confirmation_cancels_and_obsolete_decisions_are_ignored() -> anyhow::Result<()> {
    let mut fixture = DetailFixture::new(ingredient(NutritionBasis::Per100Grams));
    fixture.assert_open_confirmation().await?;
    assert_empty(
        fixture
            .detail
            .update(Message::Event(IngredientDetailEvent::RequestDelete)),
    )
    .await;
    assert_empty(
        fixture
            .detail
            .update(Message::Event(IngredientDetailEvent::Edit)),
    )
    .await;
    let cancelled = fixture.decision(ConfirmationEvent::Cancel).await;
    let command = fixture.detail.update(Message::Outcome(cancelled.clone()));
    assert_eq!(
        fixture.detail.view().deletion,
        IngredientDetailDeletion::Idle
    );
    assert!(fixture.assert_render(command).await?.is_empty());
    fixture.assert_open_confirmation().await?;
    assert_empty(fixture.detail.update(Message::Outcome(cancelled.clone()))).await;
    // Even the active command ID cannot supply a decision the child did not make.
    let confirmed = fixture.decision(ConfirmationEvent::Confirm).await;
    let IngredientDetailOutcome::ConfirmationDecided { id, .. } = confirmed else {
        panic!("expected decision")
    };
    assert_empty(fixture.detail.update(Message::Outcome(
        IngredientDetailOutcome::ConfirmationDecided {
            id,
            decision: ConfirmationOutcome::Cancelled,
        },
    )))
    .await;
    assert_empty(fixture.detail.update(Message::Outcome(cancelled))).await;
    assert!(matches!(
        fixture.detail.view().deletion,
        IngredientDetailDeletion::Confirming {
            confirmation: ConfirmationView::Resolved {
                decision: ConfirmationOutcome::Confirmed
            },
            ..
        }
    ));
    Ok(())
}

#[test_case(IngredientDeleteError::Storage; "storage")]
#[test_case(IngredientDeleteError::Transport; "transport")]
#[tokio::test]
async fn failure_reopens_confirmation_and_retry_succeeds(
    error: IngredientDeleteError,
) -> anyhow::Result<()> {
    let original = ingredient(NutritionBasis::Per100Grams);
    let mut fixture = DetailFixture::new(original.clone());
    fixture.assert_open_confirmation().await?;
    let decision = fixture.decision(ConfirmationEvent::Confirm).await;
    let command = fixture.detail.update(Message::Outcome(decision.clone()));
    let failed = fixture.assert_delete(command, Err(error.clone())).await?;
    let render = fixture.detail.update(Message::Outcome(failed.clone()));
    assert_eq!(
        fixture.detail.view().deletion,
        IngredientDetailDeletion::Confirming {
            confirmation: ConfirmationView::AwaitingDecision,
            error: Some(error),
        }
    );
    assert_eq!(fixture.detail.view().ingredient, original);
    assert!(fixture.assert_render(render).await?.is_empty());
    assert_empty(fixture.detail.update(Message::Outcome(decision))).await;
    assert_empty(fixture.detail.update(Message::Outcome(failed.clone()))).await;

    let retry = fixture.decision(ConfirmationEvent::Confirm).await;
    let command = fixture.detail.update(Message::Outcome(retry));
    assert_empty(fixture.detail.update(Message::Outcome(failed))).await;
    let success = fixture.assert_delete(command, Ok(())).await?;
    let render = fixture.detail.update(Message::Outcome(success.clone()));
    assert_eq!(
        fixture.detail.view().deletion,
        IngredientDetailDeletion::Deleted
    );
    assert_eq!(
        fixture.assert_render(render).await?,
        [IngredientDetailOutcome::Deleted { id: original.id }]
    );
    assert_empty(fixture.detail.update(Message::Outcome(success))).await;
    for event in all_events() {
        assert_empty(fixture.detail.update(Message::Event(event))).await;
    }
    Ok(())
}

#[tokio::test]
async fn failed_deletion_can_be_cancelled_and_error_cleared() -> anyhow::Result<()> {
    let mut fixture = DetailFixture::new(ingredient(NutritionBasis::Per100Grams));
    fixture.assert_open_confirmation().await?;
    let decision = fixture.decision(ConfirmationEvent::Confirm).await;
    let command = fixture.detail.update(Message::Outcome(decision));
    let outcome = fixture
        .assert_delete(command, Err(IngredientDeleteError::Storage))
        .await?;
    let command = fixture.detail.update(Message::Outcome(outcome));
    fixture.assert_render(command).await?;
    let cancelled = fixture.decision(ConfirmationEvent::Cancel).await;
    let command = fixture.detail.update(Message::Outcome(cancelled));
    fixture.assert_render(command).await?;
    assert_eq!(
        fixture.detail.view().deletion,
        IngredientDetailDeletion::Idle
    );
    fixture.assert_open_confirmation().await?;
    Ok(())
}

#[tokio::test]
async fn dropping_detail_cancels_before_store_request() -> anyhow::Result<()> {
    let mut fixture = DetailFixture::new(ingredient(NutritionBasis::Per100Grams));
    fixture.assert_open_confirmation().await?;
    let decision = fixture.decision(ConfirmationEvent::Confirm).await;
    let mut command = fixture
        .detail
        .update(Message::Outcome(decision))
        .driver(&mut fixture.effects);
    command
        .drive()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    drop(fixture.detail);
    command.drive().await?.expect_done();
    Ok(())
}

struct DetailFixture {
    detail: IngredientDetail,
    effects: IngredientEffectStream,
}

impl DetailFixture {
    fn new(ingredient: Ingredient) -> Self {
        let (detail, effects) = IngredientDetail::test_fixture(ingredient);
        Self { detail, effects }
    }

    async fn assert_render(
        &mut self,
        command: Command<IngredientDetailOutcome>,
    ) -> anyhow::Result<Vec<IngredientDetailOutcome>> {
        let mut driver = command.driver(&mut self.effects);
        driver
            .drive()
            .await?
            .expect_effect()
            .expect_render()
            .resolve_render(|| async {})
            .await?;
        // Drain through the driver, so any unexpected store request fails the test.
        let mut outcomes = Vec::new();
        // Render-only and render-then-deleted are the two expected shapes.
        if matches!(
            self.detail.view().deletion,
            IngredientDetailDeletion::Deleted
        ) {
            outcomes.push(driver.drive().await?.expect_outcome());
        }
        driver.drive().await?.expect_done();
        Ok(outcomes)
    }

    async fn assert_open_confirmation(&mut self) -> anyhow::Result<()> {
        let command = self
            .detail
            .update(Message::Event(IngredientDetailEvent::RequestDelete));
        assert_eq!(
            self.detail.view().deletion,
            IngredientDetailDeletion::Confirming {
                confirmation: ConfirmationView::AwaitingDecision,
                error: None,
            }
        );
        assert!(self.assert_render(command).await?.is_empty());
        Ok(())
    }

    async fn decision(&mut self, event: ConfirmationEvent) -> IngredientDetailOutcome {
        let command = self
            .detail
            .update(Message::Event(IngredientDetailEvent::Confirmation(event)));
        // The child has already resolved, even though its command has not been polled.
        for next in [ConfirmationEvent::Confirm, ConfirmationEvent::Cancel] {
            assert_empty(
                self.detail
                    .update(Message::Event(IngredientDetailEvent::Confirmation(next))),
            )
            .await;
        }
        let mut outcomes = command.into_stream().collect::<Vec<_>>().await;
        assert_eq!(outcomes.len(), 1);
        outcomes.remove(0)
    }

    async fn assert_delete(
        &mut self,
        command: Command<IngredientDetailOutcome>,
        result: Result<(), IngredientDeleteError>,
    ) -> anyhow::Result<IngredientDetailOutcome> {
        assert_eq!(
            self.detail.view().deletion,
            IngredientDetailDeletion::Deleting
        );
        for event in all_events() {
            assert_empty(self.detail.update(Message::Event(event))).await;
        }
        let expected_id = self.detail.view().ingredient.id;
        let mut driver = command.driver(&mut self.effects);
        driver
            .drive()
            .await?
            .expect_effect()
            .expect_render()
            .resolve_render(|| async {})
            .await?;
        driver
            .drive()
            .await?
            .expect_effect()
            .expect_store()
            .resolve_delete(|id| async move {
                assert_eq!(id, expected_id);
                result
            })
            .await?;
        let outcome = driver.drive().await?.expect_outcome();
        driver.drive().await?.expect_done();
        Ok(outcome)
    }
}

async fn assert_empty(command: Command<IngredientDetailOutcome>) {
    assert!(command.into_stream().next().await.is_none());
}

fn all_events() -> [IngredientDetailEvent; 4] {
    [
        IngredientDetailEvent::Edit,
        IngredientDetailEvent::RequestDelete,
        IngredientDetailEvent::Confirmation(ConfirmationEvent::Confirm),
        IngredientDetailEvent::Confirmation(ConfirmationEvent::Cancel),
    ]
}

fn ingredient(basis: NutritionBasis) -> Ingredient {
    Ingredient {
        id: IngredientId::new(),
        name: "Garlic".into(),
        notes: Some("Fresh".into()),
        nutrition: Nutrition {
            basis,
            calories: Some(Decimal::from_str_exact("12.3400").unwrap().into()),
            protein: None,
            carbohydrates: Some(Decimal::ZERO.into()),
            fat: None,
        },
        units: vec![
            ReferenceUnit {
                name: "cup".into(),
                grams: None,
                millilitres: Some(Decimal::from_str_exact("250.00").unwrap().into()),
            },
            ReferenceUnit {
                name: "clove".into(),
                grams: Some(Decimal::ONE.into()),
                millilitres: None,
            },
        ],
    }
}
