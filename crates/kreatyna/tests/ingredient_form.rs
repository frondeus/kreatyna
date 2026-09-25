use futures::StreamExt;
use kreatyna::{
    capabilities::ResolveRender,
    confirmation::{ConfirmationEvent, ConfirmationView},
    ingredients::{
        capabilities::{IngredientEffectStream, IngredientSaveError, ResolveIngredientStore},
        *,
    },
};
use krucyfiks::{Command, Message, Widget, effects::CommandTestExt};
use rust_decimal::Decimal;
use test_case::test_case;

#[test_case(NutritionBasis::Per100Grams; "mass basis")]
#[test_case(NutritionBasis::Per100Millilitres; "volume basis")]
#[tokio::test]
async fn initial_create_and_edit_views_keep_exact_drafts(basis: NutritionBasis) {
    let create = IngredientDraft::new(basis);
    let mut fixture = FormFixture::new(create.clone());
    assert_eq!(fixture.form.view().draft, create);
    assert!(!fixture.form.view().dirty);
    assert!(fixture.form.view().issues.is_empty());
    assert_eq!(fixture.form.view().status, IngredientFormStatus::Editing);
    assert_empty(fixture.form.init()).await;

    let item = ingredient(basis);
    let mut fixture = FormFixture::new(IngredientDraft::from(&item));
    assert_empty(fixture.form.init()).await;
    let view = fixture.form.view();
    assert_eq!(view.draft.mode, DraftMode::Edit { id: item.id });
    assert_eq!(view.draft.calories, "12.3400");
    assert_eq!(view.draft.units[0].millilitres, "250.00");
    assert_eq!(view.draft.units[1].grams, "1.000");
    assert!(!view.dirty);
}

#[tokio::test]
async fn effective_edits_preserve_text_and_reverting_clears_dirty() -> anyhow::Result<()> {
    let mut fixture = FormFixture::new(IngredientDraft::new(NutritionBasis::Per100Grams));
    fixture
        .edit(field(IngredientDraftField::Name, "  Garlic  "))
        .await?;
    assert_eq!(fixture.form.view().draft.name, "  Garlic  ");
    assert!(fixture.form.view().dirty);
    fixture
        .edit(IngredientFormEvent::SetBasis(
            NutritionBasis::Per100Millilitres,
        ))
        .await?;
    fixture
        .edit(field(IngredientDraftField::Calories, " 0,000000 "))
        .await?;
    assert_eq!(fixture.form.view().draft.calories, " 0,000000 ");
    fixture
        .edit(field(IngredientDraftField::Calories, ""))
        .await?;
    fixture
        .edit(IngredientFormEvent::SetBasis(NutritionBasis::Per100Grams))
        .await?;
    fixture.edit(field(IngredientDraftField::Name, "")).await?;
    assert!(!fixture.form.view().dirty);
    assert_empty(
        fixture
            .form
            .update(Message::Event(field(IngredientDraftField::Name, ""))),
    )
    .await;
    assert_empty(
        fixture
            .form
            .update(Message::Event(IngredientFormEvent::SetBasis(
                NutritionBasis::Per100Grams,
            ))),
    )
    .await;
    assert!(fixture.form.view().issues.is_empty());
    Ok(())
}

#[tokio::test]
async fn unit_add_edit_move_and_remove_keep_order_and_guard_indices() -> anyhow::Result<()> {
    let mut fixture = FormFixture::new(named_draft());
    fixture.edit(IngredientFormEvent::AddUnit).await?;
    fixture
        .edit(field(IngredientDraftField::UnitName { index: 0 }, "cup"))
        .await?;
    fixture
        .edit(field(
            IngredientDraftField::UnitMillilitres { index: 0 },
            "250",
        ))
        .await?;
    fixture.edit(IngredientFormEvent::AddUnit).await?;
    fixture
        .edit(field(IngredientDraftField::UnitName { index: 1 }, "clove"))
        .await?;
    fixture
        .edit(field(IngredientDraftField::UnitGrams { index: 1 }, "1"))
        .await?;
    fixture
        .edit(IngredientFormEvent::MoveUnit { from: 0, to: 1 })
        .await?;
    assert_eq!(
        fixture
            .form
            .view()
            .draft
            .units
            .iter()
            .map(|u| u.name.as_str())
            .collect::<Vec<_>>(),
        ["clove", "cup"]
    );
    assert_empty(
        fixture
            .form
            .update(Message::Event(IngredientFormEvent::MoveUnit {
                from: 0,
                to: 0,
            })),
    )
    .await;
    assert_empty(
        fixture
            .form
            .update(Message::Event(IngredientFormEvent::MoveUnit {
                from: 0,
                to: 2,
            })),
    )
    .await;
    assert_empty(
        fixture
            .form
            .update(Message::Event(IngredientFormEvent::RemoveUnit { index: 2 })),
    )
    .await;
    assert_empty(fixture.form.update(Message::Event(field(
        IngredientDraftField::UnitName { index: 2 },
        "bad",
    ))))
    .await;
    fixture
        .edit(IngredientFormEvent::RemoveUnit { index: 0 })
        .await?;
    assert_eq!(fixture.form.view().draft.units[0].name, "cup");
    fixture
        .edit(IngredientFormEvent::RemoveUnit { index: 0 })
        .await?;
    assert!(!fixture.form.view().dirty);
    Ok(())
}

#[tokio::test]
async fn first_save_exposes_all_issues_then_edits_update_their_indices() -> anyhow::Result<()> {
    let mut fixture = FormFixture::new(IngredientDraft::new(NutritionBasis::Per100Grams));
    fixture.edit(IngredientFormEvent::AddUnit).await?;
    assert!(fixture.form.view().issues.is_empty());
    let command = fixture
        .form
        .update(Message::Event(IngredientFormEvent::Save));
    fixture.assert_render(command).await?;
    assert_eq!(
        fixture.form.view().issues,
        vec![
            issue(ValidationField::Name, ValidationReason::Required),
            issue(
                ValidationField::UnitName { index: 0 },
                ValidationReason::Required
            ),
            issue(
                ValidationField::UnitMeasurements { index: 0 },
                ValidationReason::Required
            ),
        ]
    );
    fixture
        .edit(field(IngredientDraftField::Name, "Garlic"))
        .await?;
    fixture
        .edit(field(IngredientDraftField::UnitName { index: 0 }, "cup"))
        .await?;
    fixture
        .edit(field(
            IngredientDraftField::UnitMillilitres { index: 0 },
            "250",
        ))
        .await?;
    assert!(fixture.form.view().issues.is_empty());
    fixture.edit(IngredientFormEvent::AddUnit).await?;
    fixture
        .edit(field(IngredientDraftField::UnitName { index: 1 }, " CUP "))
        .await?;
    fixture
        .edit(field(IngredientDraftField::UnitGrams { index: 1 }, "bad"))
        .await?;
    assert_eq!(
        fixture.form.view().issues,
        [
            issue(
                ValidationField::UnitName { index: 1 },
                ValidationReason::DuplicateName
            ),
            issue(
                ValidationField::UnitGrams { index: 1 },
                ValidationReason::InvalidNumber
            ),
        ]
    );
    fixture
        .edit(IngredientFormEvent::MoveUnit { from: 1, to: 0 })
        .await?;
    assert_eq!(
        fixture.form.view().issues,
        [
            issue(
                ValidationField::UnitGrams { index: 0 },
                ValidationReason::InvalidNumber
            ),
            issue(
                ValidationField::UnitName { index: 1 },
                ValidationReason::DuplicateName
            ),
        ]
    );
    fixture
        .edit(IngredientFormEvent::RemoveUnit { index: 0 })
        .await?;
    assert!(fixture.form.view().issues.is_empty());
    Ok(())
}

#[tokio::test]
async fn valid_create_renders_before_save_and_emits_complete_store_record() -> anyhow::Result<()> {
    let mut draft = named_draft();
    draft.name = "  Garlic  ".into();
    draft.notes = " Fresh ".into();
    draft.calories = "0.00".into();
    draft.units.push(ReferenceUnitDraft {
        name: " cup ".into(),
        grams: String::new(),
        millilitres: "250.00".into(),
    });
    let mut fixture = FormFixture::new(draft.clone());
    let saved = ingredient(NutritionBasis::Per100Grams);
    let command = fixture
        .form
        .update(Message::Event(IngredientFormEvent::Save));
    let outcome = fixture
        .assert_save(command, Ok(saved.clone()), |write| {
            let IngredientWrite::Create { data } = write else {
                panic!("expected create")
            };
            assert_eq!(data.name(), "Garlic");
            assert_eq!(data.notes(), Some("Fresh"));
            assert_eq!(data.nutrition().calories.unwrap().to_string(), "0.00");
            assert!(data.nutrition().protein.is_none());
            assert_eq!(data.units()[0].name, "cup");
            assert_eq!(data.units()[0].millilitres.unwrap().to_string(), "250.00");
        })
        .await?;
    let command = fixture.form.update(Message::Outcome(outcome.clone()));
    assert_eq!(fixture.form.view().status, IngredientFormStatus::Finished);
    assert_eq!(
        fixture.assert_render_outcome(command).await?,
        IngredientFormOutcome::Saved { ingredient: saved }
    );
    assert_empty(fixture.form.update(Message::Outcome(outcome))).await;
    for event in blocked_events() {
        assert_empty(fixture.form.update(Message::Event(event))).await;
    }
    assert_eq!(fixture.form.view().draft, draft);
    Ok(())
}

#[test_case(IngredientSaveError::DuplicateName; "duplicate")]
#[test_case(IngredientSaveError::NotFound; "missing")]
#[test_case(IngredientSaveError::Storage; "storage")]
#[test_case(IngredientSaveError::Transport; "transport")]
#[tokio::test]
async fn edit_save_errors_retain_draft_and_explicit_retry_recovers(
    error: IngredientSaveError,
) -> anyhow::Result<()> {
    let original = ingredient(NutritionBasis::Per100Millilitres);
    let draft = IngredientDraft::from(&original);
    let mut fixture = FormFixture::new(draft.clone());
    fixture
        .edit(field(IngredientDraftField::Name, "  Updated garlic  "))
        .await?;
    let command = fixture
        .form
        .update(Message::Event(IngredientFormEvent::Save));
    let first = fixture
        .assert_save(command, Err(error.clone()), move |write| {
            let IngredientWrite::Update { id, data } = write else {
                panic!("expected update")
            };
            assert_eq!(id, original.id);
            assert_eq!(data.name(), "Updated garlic");
            assert_eq!(data.nutrition().basis, NutritionBasis::Per100Millilitres);
        })
        .await?;
    let command = fixture.form.update(Message::Outcome(first.clone()));
    fixture.assert_render(command).await?;
    assert_eq!(fixture.form.view().status, IngredientFormStatus::Editing);
    assert_eq!(fixture.form.view().save_error, Some(error));
    assert_eq!(fixture.form.view().draft.name, "  Updated garlic  ");
    assert!(fixture.form.view().dirty);
    assert_empty(fixture.form.update(Message::Outcome(first.clone()))).await;
    let saved = ingredient(NutritionBasis::Per100Millilitres);
    let command = fixture
        .form
        .update(Message::Event(IngredientFormEvent::Save));
    let second = fixture
        .assert_save(command, Ok(saved.clone()), |_| {})
        .await?;
    assert_empty(fixture.form.update(Message::Outcome(first))).await;
    let command = fixture.form.update(Message::Outcome(second));
    assert_eq!(
        fixture.assert_render_outcome(command).await?,
        IngredientFormOutcome::Saved { ingredient: saved }
    );
    Ok(())
}

#[tokio::test]
async fn correcting_after_store_error_clears_error_and_revalidates() -> anyhow::Result<()> {
    let mut fixture = FormFixture::new(named_draft());
    let command = fixture
        .form
        .update(Message::Event(IngredientFormEvent::Save));
    let failed = fixture
        .assert_save(command, Err(IngredientSaveError::DuplicateName), |_| {})
        .await?;
    let command = fixture.form.update(Message::Outcome(failed));
    fixture.assert_render(command).await?;
    fixture.edit(field(IngredientDraftField::Name, " ")).await?;
    assert_eq!(fixture.form.view().save_error, None);
    assert_eq!(
        fixture.form.view().issues,
        [issue(ValidationField::Name, ValidationReason::Required)]
    );
    let command = fixture
        .form
        .update(Message::Event(IngredientFormEvent::Save));
    fixture.assert_render(command).await?;
    assert_eq!(fixture.form.view().status, IngredientFormStatus::Editing);
    Ok(())
}

#[tokio::test]
async fn clean_close_finishes_without_store_and_dirty_close_uses_confirmation() -> anyhow::Result<()>
{
    let mut fixture = FormFixture::new(named_draft());
    let command = fixture
        .form
        .update(Message::Event(IngredientFormEvent::RequestClose));
    assert_eq!(fixture.form.view().status, IngredientFormStatus::Finished);
    assert_eq!(
        fixture.assert_render_outcome(command).await?,
        IngredientFormOutcome::Discarded
    );

    let mut fixture = FormFixture::new(named_draft());
    fixture
        .edit(field(IngredientDraftField::Name, "Changed"))
        .await?;
    let command = fixture
        .form
        .update(Message::Event(IngredientFormEvent::RequestClose));
    assert_eq!(
        fixture.form.view().status,
        IngredientFormStatus::ConfirmingDiscard(ConfirmationView::AwaitingDecision)
    );
    fixture.assert_render(command).await?;
    assert_empty(
        fixture
            .form
            .update(Message::Event(IngredientFormEvent::RequestClose)),
    )
    .await;
    assert_empty(
        fixture
            .form
            .update(Message::Event(IngredientFormEvent::Save)),
    )
    .await;
    let cancelled = fixture.decision(ConfirmationEvent::Cancel).await;
    let command = fixture.form.update(Message::Outcome(cancelled.clone()));
    fixture.assert_render(command).await?;
    assert_eq!(fixture.form.view().draft.name, "Changed");
    assert!(fixture.form.view().dirty);
    let command = fixture
        .form
        .update(Message::Event(IngredientFormEvent::RequestClose));
    fixture.assert_render(command).await?;
    assert_empty(fixture.form.update(Message::Outcome(cancelled))).await;
    let confirmed = fixture.decision(ConfirmationEvent::Confirm).await;
    let command = fixture.form.update(Message::Outcome(confirmed.clone()));
    assert_eq!(
        fixture.assert_render_outcome(command).await?,
        IngredientFormOutcome::Discarded
    );
    assert_empty(fixture.form.update(Message::Outcome(confirmed))).await;
    for event in blocked_events() {
        assert_empty(fixture.form.update(Message::Event(event))).await;
    }
    Ok(())
}

#[tokio::test]
async fn cancelling_discard_keeps_a_failed_save_visible() -> anyhow::Result<()> {
    let mut fixture = FormFixture::new(named_draft());
    fixture
        .edit(field(IngredientDraftField::Name, "Changed"))
        .await?;
    let command = fixture
        .form
        .update(Message::Event(IngredientFormEvent::Save));
    let failed = fixture
        .assert_save(command, Err(IngredientSaveError::Storage), |_| {})
        .await?;
    let command = fixture.form.update(Message::Outcome(failed));
    fixture.assert_render(command).await?;
    let command = fixture
        .form
        .update(Message::Event(IngredientFormEvent::RequestClose));
    fixture.assert_render(command).await?;
    let cancelled = fixture.decision(ConfirmationEvent::Cancel).await;
    let command = fixture.form.update(Message::Outcome(cancelled));
    fixture.assert_render(command).await?;
    let view = fixture.form.view();
    assert_eq!(view.status, IngredientFormStatus::Editing);
    assert_eq!(view.draft.name, "Changed");
    assert_eq!(view.save_error, Some(IngredientSaveError::Storage));
    assert!(view.dirty);
    Ok(())
}

#[tokio::test]
async fn dropping_form_cancels_pending_save_before_store_request() -> anyhow::Result<()> {
    let mut fixture = FormFixture::new(named_draft());
    let mut command = fixture
        .form
        .update(Message::Event(IngredientFormEvent::Save))
        .driver(&mut fixture.effects);
    command
        .drive()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    drop(fixture.form);
    command.drive().await?.expect_done();
    Ok(())
}

struct FormFixture {
    form: IngredientForm,
    effects: IngredientEffectStream,
}

impl FormFixture {
    fn new(draft: IngredientDraft) -> Self {
        let (form, effects) = IngredientForm::test_fixture(draft);
        Self { form, effects }
    }

    async fn edit(&mut self, event: IngredientFormEvent) -> anyhow::Result<()> {
        let command = self.form.update(Message::Event(event));
        self.assert_render(command).await
    }

    async fn assert_render(
        &mut self,
        command: Command<IngredientFormOutcome>,
    ) -> anyhow::Result<()> {
        let mut driver = command.driver(&mut self.effects);
        driver
            .drive()
            .await?
            .expect_effect()
            .expect_render()
            .resolve_render(|| async {})
            .await?;
        driver.drive().await?.expect_done();
        Ok(())
    }

    async fn assert_render_outcome(
        &mut self,
        command: Command<IngredientFormOutcome>,
    ) -> anyhow::Result<IngredientFormOutcome> {
        let mut driver = command.driver(&mut self.effects);
        driver
            .drive()
            .await?
            .expect_effect()
            .expect_render()
            .resolve_render(|| async {})
            .await?;
        let outcome = driver.drive().await?.expect_outcome();
        driver.drive().await?.expect_done();
        Ok(outcome)
    }

    async fn assert_save<F>(
        &mut self,
        command: Command<IngredientFormOutcome>,
        result: Result<Ingredient, IngredientSaveError>,
        check: F,
    ) -> anyhow::Result<IngredientFormOutcome>
    where
        F: FnOnce(IngredientWrite) + Send + 'static,
    {
        assert_eq!(self.form.view().status, IngredientFormStatus::Saving);
        for event in blocked_events() {
            assert_empty(self.form.update(Message::Event(event))).await;
        }
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
            .resolve_save(move |write| async move {
                check(write);
                result
            })
            .await?;
        let outcome = driver.drive().await?.expect_outcome();
        driver.drive().await?.expect_done();
        Ok(outcome)
    }

    async fn decision(&mut self, event: ConfirmationEvent) -> IngredientFormOutcome {
        let command = self
            .form
            .update(Message::Event(IngredientFormEvent::Confirmation(event)));
        for next in [ConfirmationEvent::Confirm, ConfirmationEvent::Cancel] {
            assert_empty(
                self.form
                    .update(Message::Event(IngredientFormEvent::Confirmation(next))),
            )
            .await;
        }
        let mut outcomes = command.into_stream().collect::<Vec<_>>().await;
        assert_eq!(outcomes.len(), 1);
        outcomes.remove(0)
    }
}

async fn assert_empty(command: Command<IngredientFormOutcome>) {
    assert!(command.into_stream().next().await.is_none());
}

fn field(field: IngredientDraftField, value: &str) -> IngredientFormEvent {
    IngredientFormEvent::SetField {
        field,
        value: value.into(),
    }
}

fn issue(field: ValidationField, reason: ValidationReason) -> ValidationIssue {
    ValidationIssue { field, reason }
}

fn blocked_events() -> [IngredientFormEvent; 8] {
    [
        field(IngredientDraftField::Name, "blocked"),
        IngredientFormEvent::SetBasis(NutritionBasis::Per100Millilitres),
        IngredientFormEvent::AddUnit,
        IngredientFormEvent::RemoveUnit { index: 0 },
        IngredientFormEvent::MoveUnit { from: 0, to: 0 },
        IngredientFormEvent::Save,
        IngredientFormEvent::RequestClose,
        IngredientFormEvent::Confirmation(ConfirmationEvent::Confirm),
    ]
}

fn named_draft() -> IngredientDraft {
    let mut draft = IngredientDraft::new(NutritionBasis::Per100Grams);
    draft.name = "Garlic".into();
    draft
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
                grams: Some(Decimal::from_str_exact("1.000").unwrap().into()),
                millilitres: None,
            },
        ],
    }
}
