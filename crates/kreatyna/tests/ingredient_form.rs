use kreatyna::{
    capabilities::ResolveRender,
    confirmation::{ConfirmationEvent, ConfirmationOutcome, ConfirmationView},
    ingredients::{
        capabilities::{
            IngredientEffect, IngredientEffectStream, IngredientSaveError, ResolveIngredientStore,
        },
        *,
    },
};
use krucyfiks::{
    Latest, Message, Widget,
    effects::{CommandDriver, CommandTestExt},
    test_utils::EmptyInitTest,
};
use rust_decimal::Decimal;
use test_case::{test_case, test_matrix};
use uuid::Uuid;

// Ordinary behavior

#[test_case(IngredientDraft::new(NutritionBasis::Per100Grams); "new mass draft")]
#[test_case(IngredientDraft::new(NutritionBasis::Per100Millilitres); "new volume draft")]
#[test_case(IngredientDraft::from(&ingredient(NutritionBasis::Per100Millilitres)); "edit draft")]
#[tokio::test]
async fn initialization_does_nothing_and_exposes_the_supplied_draft_as_clean(
    draft: IngredientDraft,
) {
    let mut fixture = FormFixture::new(draft.clone());
    assert_eq!(
        fixture.form.view(),
        IngredientFormView {
            draft,
            dirty: false,
            issues: vec![],
            save_error: None,
            status: IngredientFormStatus::Editing,
        }
    );
    fixture.form.expect_noop_init().await;
}

#[test_case(IngredientDraftField::Name, "  Garlic  ", edited(blank_draft(), |d| d.name = "  Garlic  ".into()); "name keeps surrounding spaces")]
#[test_case(IngredientDraftField::Notes, " Fresh ", edited(blank_draft(), |d| d.notes = " Fresh ".into()); "notes keep surrounding spaces")]
#[test_case(IngredientDraftField::Calories, " 0,000000 ", edited(blank_draft(), |d| d.calories = " 0,000000 ".into()); "calories keep raw number text")]
#[test_case(IngredientDraftField::Protein, "bad", edited(blank_draft(), |d| d.protein = "bad".into()); "malformed protein")]
#[test_case(IngredientDraftField::Carbohydrates, "-1", edited(blank_draft(), |d| d.carbohydrates = "-1".into()); "negative carbohydrates")]
#[test_case(IngredientDraftField::Fat, "1.0000000", edited(blank_draft(), |d| d.fat = "1.0000000".into()); "too precise fat")]
#[tokio::test]
async fn editing_a_field_keeps_raw_text_marks_dirty_and_shows_no_issues_before_saving(
    target: IngredientDraftField,
    value: &str,
    expected: IngredientDraft,
) -> anyhow::Result<()> {
    let mut fixture = FormFixture::new(blank_draft());
    let mut command = fixture
        .form
        .update(Message::Event(field(target, value)))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.form.view(), editing_view(expected, true));

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
async fn changing_the_basis_marks_the_draft_dirty() -> anyhow::Result<()> {
    let mut fixture = FormFixture::new(blank_draft());
    let mut command = fixture
        .form
        .update(Message::Event(IngredientFormEvent::SetBasis(
            NutritionBasis::Per100Millilitres,
        )))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.form.view(),
        editing_view(
            IngredientDraft::new(NutritionBasis::Per100Millilitres),
            true
        )
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
async fn reverting_every_change_makes_the_draft_clean_again() -> anyhow::Result<()> {
    let mut fixture = FormFixture::new(blank_draft());
    let mut command = fixture
        .form
        .update(Message::Event(field(IngredientDraftField::Name, "Garlic")))
        .driver(&mut fixture.effects);
    assert!(fixture.form.view().dirty);
    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command.step().await?.expect_done();
    drop(command);

    let mut command = fixture
        .form
        .update(Message::Event(IngredientFormEvent::SetBasis(
            NutritionBasis::Per100Millilitres,
        )))
        .driver(&mut fixture.effects);
    assert!(fixture.form.view().dirty);
    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command.step().await?.expect_done();
    drop(command);

    // The name is original again, but the basis still differs.
    let mut command = fixture
        .form
        .update(Message::Event(field(IngredientDraftField::Name, "")))
        .driver(&mut fixture.effects);
    assert!(fixture.form.view().dirty);
    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command.step().await?.expect_done();
    drop(command);

    let mut command = fixture
        .form
        .update(Message::Event(IngredientFormEvent::SetBasis(
            NutritionBasis::Per100Grams,
        )))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.form.view(), editing_view(blank_draft(), false));
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
async fn adding_a_unit_appends_an_empty_unit() -> anyhow::Result<()> {
    let mut fixture = FormFixture::new(draft_with_units());
    let mut command = fixture
        .form
        .update(Message::Event(IngredientFormEvent::AddUnit))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.form.view(),
        editing_view(
            edited(draft_with_units(), |d| d
                .units
                .push(ReferenceUnitDraft::default())),
            true
        )
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

#[test_case(IngredientDraftField::UnitName { index: 1 }, "garlic clove", edited(draft_with_units(), |d| d.units[1].name = "garlic clove".into()); "unit name")]
#[test_case(IngredientDraftField::UnitGrams { index: 1 }, " 4 ", edited(draft_with_units(), |d| d.units[1].grams = " 4 ".into()); "unit grams")]
#[test_case(IngredientDraftField::UnitMillilitres { index: 1 }, "5", edited(draft_with_units(), |d| d.units[1].millilitres = "5".into()); "unit millilitres")]
#[tokio::test]
async fn editing_a_unit_field_changes_only_that_unit(
    target: IngredientDraftField,
    value: &str,
    expected: IngredientDraft,
) -> anyhow::Result<()> {
    let mut fixture = FormFixture::new(draft_with_units());
    let mut command = fixture
        .form
        .update(Message::Event(field(target, value)))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.form.view(), editing_view(expected, true));

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

#[test_case(0, 2, ["clove", "spoon", "cup"]; "first to last")]
#[test_case(2, 0, ["spoon", "cup", "clove"]; "last to first")]
#[test_case(1, 0, ["clove", "cup", "spoon"]; "middle to first")]
#[tokio::test]
async fn moving_a_unit_places_it_at_the_target_index(
    from: usize,
    to: usize,
    expected: [&str; 3],
) -> anyhow::Result<()> {
    let mut fixture = FormFixture::new(draft_with_units());
    let mut command = fixture
        .form
        .update(Message::Event(IngredientFormEvent::MoveUnit { from, to }))
        .driver(&mut fixture.effects);
    let view = fixture.form.view();
    assert_eq!(unit_names(&view.draft), expected);
    assert!(view.dirty);
    assert_eq!(view.status, IngredientFormStatus::Editing);

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

#[test_case(0, ["clove", "spoon"]; "first")]
#[test_case(1, ["cup", "spoon"]; "middle")]
#[test_case(2, ["cup", "clove"]; "last")]
#[tokio::test]
async fn removing_a_unit_keeps_the_remaining_order(
    index: usize,
    expected: [&str; 2],
) -> anyhow::Result<()> {
    let mut fixture = FormFixture::new(draft_with_units());
    let mut command = fixture
        .form
        .update(Message::Event(IngredientFormEvent::RemoveUnit { index }))
        .driver(&mut fixture.effects);
    let view = fixture.form.view();
    assert_eq!(unit_names(&view.draft), expected);
    assert!(view.dirty);
    assert_eq!(view.status, IngredientFormStatus::Editing);

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
async fn removing_an_added_unit_makes_the_draft_clean_again() -> anyhow::Result<()> {
    let mut fixture = FormFixture::new(draft_with_units());
    fixture.apply(IngredientFormEvent::AddUnit).await?;
    assert!(fixture.form.view().dirty);

    let mut command = fixture
        .form
        .update(Message::Event(IngredientFormEvent::RemoveUnit { index: 3 }))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.form.view(), editing_view(draft_with_units(), false));

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
async fn saving_an_invalid_draft_shows_every_issue_and_does_not_contact_the_store()
-> anyhow::Result<()> {
    let mut fixture = FormFixture::new(blank_draft());
    fixture.apply(IngredientFormEvent::AddUnit).await?;

    let mut command = fixture
        .form
        .update(Message::Event(IngredientFormEvent::Save))
        .driver(&mut fixture.effects);
    let view = fixture.form.view();
    assert_eq!(view.status, IngredientFormStatus::Editing);
    assert_eq!(
        view.issues,
        [
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
    assert_eq!(view.save_error, None);

    // Only a render follows. The store is not asked to save.
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
async fn after_a_failed_validation_each_edit_revalidates() -> anyhow::Result<()> {
    let mut fixture = FormFixture::new(blank_draft());
    fixture.apply(IngredientFormEvent::AddUnit).await?;
    fixture.apply(IngredientFormEvent::Save).await?;

    let mut command = fixture
        .form
        .update(Message::Event(field(IngredientDraftField::Name, "Garlic")))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.form.view().issues,
        [
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
    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command.step().await?.expect_done();
    drop(command);

    let mut command = fixture
        .form
        .update(Message::Event(field(
            IngredientDraftField::UnitName { index: 0 },
            "cup",
        )))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.form.view().issues,
        [issue(
            ValidationField::UnitMeasurements { index: 0 },
            ValidationReason::Required
        )]
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

    let mut command = fixture
        .form
        .update(Message::Event(field(
            IngredientDraftField::UnitMillilitres { index: 0 },
            "250",
        )))
        .driver(&mut fixture.effects);
    assert!(fixture.form.view().issues.is_empty());
    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command.step().await?.expect_done();
    drop(command);

    // A new mistake is reported immediately, without another save.
    let mut command = fixture
        .form
        .update(Message::Event(field(
            IngredientDraftField::UnitGrams { index: 0 },
            "bad",
        )))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.form.view().issues,
        [issue(
            ValidationField::UnitGrams { index: 0 },
            ValidationReason::InvalidNumber
        )]
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

#[test_case(
    IngredientFormEvent::MoveUnit { from: 1, to: 0 },
    vec![
        issue(ValidationField::UnitGrams { index: 0 }, ValidationReason::InvalidNumber),
        issue(ValidationField::UnitName { index: 1 }, ValidationReason::DuplicateName),
    ];
    "move changes indices and the later duplicate"
)]
#[test_case(
    IngredientFormEvent::RemoveUnit { index: 0 },
    vec![issue(ValidationField::UnitGrams { index: 0 }, ValidationReason::InvalidNumber)];
    "remove shifts indices and clears the duplicate"
)]
#[tokio::test]
async fn after_a_failed_validation_unit_reordering_updates_issue_indices(
    event: IngredientFormEvent,
    expected: Vec<ValidationIssue>,
) -> anyhow::Result<()> {
    let mut draft = named_draft();
    draft.units = vec![
        unit("cup", "", "250"),
        // Duplicate name after case folding and an invalid mass.
        unit(" CUP ", "bad", ""),
    ];
    let mut fixture = FormFixture::new(draft);
    fixture.apply(IngredientFormEvent::Save).await?;
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

    let mut command = fixture
        .form
        .update(Message::Event(event))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.form.view().issues, expected);

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
async fn saving_a_valid_create_draft_sends_a_normalized_create_and_finishes_with_the_saved_record()
-> anyhow::Result<()> {
    let mut fixture = FormFixture::new(blank_draft());
    fixture
        .apply(field(IngredientDraftField::Name, "  Garlic  "))
        .await?;
    fixture
        .apply(field(IngredientDraftField::Notes, " Fresh "))
        .await?;
    fixture
        .apply(field(IngredientDraftField::Calories, " 0,50 "))
        .await?;
    fixture.apply(IngredientFormEvent::AddUnit).await?;
    fixture
        .apply(field(IngredientDraftField::UnitName { index: 0 }, " cup "))
        .await?;
    fixture
        .apply(field(
            IngredientDraftField::UnitMillilitres { index: 0 },
            "250",
        ))
        .await?;
    let raw = fixture.form.view().draft;

    let mut command = fixture
        .form
        .update(Message::Event(IngredientFormEvent::Save))
        .driver(&mut fixture.effects);
    // The form keeps the raw text while saving.
    assert_eq!(
        fixture.form.view(),
        IngredientFormView {
            draft: raw.clone(),
            dirty: true,
            issues: vec![],
            save_error: None,
            status: IngredientFormStatus::Saving,
        }
    );

    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    let saved = ingredient(NutritionBasis::Per100Grams);
    let mut request = None;
    command
        .step()
        .await?
        .expect_effect()
        .expect_store()
        .resolve_save(|write| {
            request = Some(write);
            async { Ok(saved.clone()) }
        })
        .await?;
    let outcome = command.step().await?.expect_outcome();
    command.step().await?.expect_done();
    drop(command);

    // The store receives normalized values.
    let Some(IngredientWrite::Create { data }) = request else {
        panic!("expected a create request, got {request:?}");
    };
    assert_eq!(data.name(), "Garlic");
    assert_eq!(data.notes(), Some("Fresh"));
    assert_eq!(
        data.nutrition(),
        &Nutrition {
            basis: NutritionBasis::Per100Grams,
            calories: Some(decimal("0.50").into()),
            protein: None,
            carbohydrates: None,
            fat: None,
        }
    );
    assert_eq!(
        data.units(),
        [ReferenceUnit {
            name: "cup".into(),
            grams: None,
            millilitres: Some(decimal("250").into()),
        }]
    );
    assert!(matches!(
        &outcome,
        IngredientFormOutcome::SaveFinished(Latest { value: Ok(record), .. }) if record == &saved
    ));

    let mut command = fixture
        .form
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    let view = fixture.form.view();
    assert_eq!(view.status, IngredientFormStatus::Finished);
    assert_eq!(view.draft, raw);

    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    assert_eq!(
        command.step().await?.expect_outcome(),
        IngredientFormOutcome::Saved { ingredient: saved }
    );
    command.step().await?.expect_done();
    Ok(())
}

#[tokio::test]
async fn saving_an_edited_draft_sends_an_update_for_the_original_id() -> anyhow::Result<()> {
    let original = ingredient(NutritionBasis::Per100Millilitres);
    let mut fixture = FormFixture::new(IngredientDraft::from(&original));
    fixture
        .apply(field(IngredientDraftField::Name, "  Updated garlic  "))
        .await?;

    let mut command = fixture
        .form
        .update(Message::Event(IngredientFormEvent::Save))
        .driver(&mut fixture.effects);
    let view = fixture.form.view();
    assert_eq!(view.status, IngredientFormStatus::Saving);
    assert_eq!(view.draft.name, "  Updated garlic  ");
    assert!(view.dirty);

    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    let saved = Ingredient {
        name: "Updated garlic".into(),
        ..original.clone()
    };
    let mut request = None;
    command
        .step()
        .await?
        .expect_effect()
        .expect_store()
        .resolve_save(|write| {
            request = Some(write);
            async { Ok(saved.clone()) }
        })
        .await?;
    let outcome = command.step().await?.expect_outcome();
    command.step().await?.expect_done();
    drop(command);

    // Only the name changed. Other stored values round-trip.
    let Some(IngredientWrite::Update { id, data }) = request else {
        panic!("expected an update request, got {request:?}");
    };
    assert_eq!(id, original.id);
    assert_eq!(data.name(), "Updated garlic");
    assert_eq!(data.notes(), original.notes.as_deref());
    assert_eq!(data.nutrition(), &original.nutrition);
    assert_eq!(data.units(), original.units);

    let mut command = fixture
        .form
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.form.view().status, IngredientFormStatus::Finished);

    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    assert_eq!(
        command.step().await?.expect_outcome(),
        IngredientFormOutcome::Saved { ingredient: saved }
    );
    command.step().await?.expect_done();
    Ok(())
}

#[test_case(blank_draft(); "create draft")]
#[test_case(IngredientDraft::from(&ingredient(NutritionBasis::Per100Grams)); "edit draft")]
#[tokio::test]
async fn closing_a_clean_draft_discards_without_confirmation(
    draft: IngredientDraft,
) -> anyhow::Result<()> {
    let mut fixture = FormFixture::new(draft);
    let mut command = fixture
        .form
        .update(Message::Event(IngredientFormEvent::RequestClose))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.form.view().status, IngredientFormStatus::Finished);

    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    assert_eq!(
        command.step().await?.expect_outcome(),
        IngredientFormOutcome::Discarded
    );
    command.step().await?.expect_done();
    Ok(())
}

#[tokio::test]
async fn closing_a_dirty_draft_asks_for_confirmation() -> anyhow::Result<()> {
    let mut fixture = FormFixture::new(named_draft());
    fixture
        .apply(field(IngredientDraftField::Name, "Changed"))
        .await?;

    let mut command = fixture
        .form
        .update(Message::Event(IngredientFormEvent::RequestClose))
        .driver(&mut fixture.effects);
    let view = fixture.form.view();
    assert_eq!(
        view.status,
        IngredientFormStatus::ConfirmingDiscard(ConfirmationView::AwaitingDecision)
    );
    assert_eq!(view.draft.name, "Changed");
    assert!(view.dirty);

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
async fn confirming_discard_finishes_with_discarded() -> anyhow::Result<()> {
    let mut fixture = FormFixture::new(named_draft());
    fixture
        .apply(field(IngredientDraftField::Name, "Changed"))
        .await?;
    fixture.apply(IngredientFormEvent::RequestClose).await?;

    let mut command = fixture
        .form
        .update(Message::Event(IngredientFormEvent::Confirmation(
            ConfirmationEvent::Confirm,
        )))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.form.view().status,
        IngredientFormStatus::ConfirmingDiscard(ConfirmationView::Resolved {
            decision: ConfirmationOutcome::Confirmed
        })
    );
    let decision = command.step().await?.expect_outcome();
    assert!(matches!(
        decision,
        IngredientFormOutcome::ConfirmationDecided(Latest {
            value: ConfirmationOutcome::Confirmed,
            ..
        })
    ));
    command.step().await?.expect_done();
    drop(command);

    let mut command = fixture
        .form
        .update(Message::Outcome(decision))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.form.view().status, IngredientFormStatus::Finished);

    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    assert_eq!(
        command.step().await?.expect_outcome(),
        IngredientFormOutcome::Discarded
    );
    command.step().await?.expect_done();
    Ok(())
}

#[tokio::test]
async fn cancelling_discard_returns_to_editing_with_the_dirty_draft() -> anyhow::Result<()> {
    let mut fixture = FormFixture::new(named_draft());
    fixture
        .apply(field(IngredientDraftField::Name, "Changed"))
        .await?;
    fixture.apply(IngredientFormEvent::RequestClose).await?;

    let mut command = fixture
        .form
        .update(Message::Event(IngredientFormEvent::Confirmation(
            ConfirmationEvent::Cancel,
        )))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.form.view().status,
        IngredientFormStatus::ConfirmingDiscard(ConfirmationView::Resolved {
            decision: ConfirmationOutcome::Cancelled
        })
    );
    let decision = command.step().await?.expect_outcome();
    assert!(matches!(
        decision,
        IngredientFormOutcome::ConfirmationDecided(Latest {
            value: ConfirmationOutcome::Cancelled,
            ..
        })
    ));
    command.step().await?.expect_done();
    drop(command);

    let mut command = fixture
        .form
        .update(Message::Outcome(decision))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.form.view(),
        editing_view(edited(named_draft(), |d| d.name = "Changed".into()), true)
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

// Edge cases and guards

#[test_case(field(IngredientDraftField::Name, "Garlic"); "same field value")]
#[test_case(IngredientFormEvent::SetBasis(NutritionBasis::Per100Grams); "same basis")]
#[test_case(field(IngredientDraftField::UnitName { index: 3 }, "bowl"); "unit field out of range")]
#[test_case(IngredientFormEvent::RemoveUnit { index: 3 }; "remove out of range")]
#[test_case(IngredientFormEvent::MoveUnit { from: 1, to: 1 }; "move to same index")]
#[test_case(IngredientFormEvent::MoveUnit { from: 0, to: 3 }; "move out of range")]
#[tokio::test]
async fn edits_that_change_nothing_do_not_render(event: IngredientFormEvent) -> anyhow::Result<()> {
    let mut fixture = FormFixture::new(draft_with_units());
    let before = fixture.form.view();
    let mut command = fixture
        .form
        .update(Message::Event(event))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.form.view(), before);
    command.step().await?.expect_done();
    Ok(())
}

#[test_case(field(IngredientDraftField::Name, "blocked"); "field edit")]
#[test_case(IngredientFormEvent::SetBasis(NutritionBasis::Per100Millilitres); "basis change")]
#[test_case(IngredientFormEvent::AddUnit; "add unit")]
#[test_case(IngredientFormEvent::RemoveUnit { index: 0 }; "remove unit")]
#[test_case(IngredientFormEvent::MoveUnit { from: 0, to: 1 }; "move unit")]
#[test_case(IngredientFormEvent::Save; "save")]
#[test_case(IngredientFormEvent::RequestClose; "close")]
#[test_case(IngredientFormEvent::Confirmation(ConfirmationEvent::Confirm); "confirmation")]
#[tokio::test]
async fn events_while_saving_are_ignored(event: IngredientFormEvent) -> anyhow::Result<()> {
    let mut fixture = FormFixture::new(draft_with_units());
    let save = fixture
        .form
        .update(Message::Event(IngredientFormEvent::Save));
    assert_eq!(fixture.form.view().status, IngredientFormStatus::Saving);

    let before = fixture.form.view();
    let mut command = fixture
        .form
        .update(Message::Event(event))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.form.view(), before);
    command.step().await?.expect_done();
    drop(save);
    Ok(())
}

#[test_case(field(IngredientDraftField::Name, "blocked"); "field edit")]
#[test_case(IngredientFormEvent::SetBasis(NutritionBasis::Per100Millilitres); "basis change")]
#[test_case(IngredientFormEvent::AddUnit; "add unit")]
#[test_case(IngredientFormEvent::RemoveUnit { index: 0 }; "remove unit")]
#[test_case(IngredientFormEvent::MoveUnit { from: 0, to: 1 }; "move unit")]
#[test_case(IngredientFormEvent::Save; "save")]
#[test_case(IngredientFormEvent::RequestClose; "close again")]
#[tokio::test]
async fn events_while_confirming_discard_are_ignored(
    event: IngredientFormEvent,
) -> anyhow::Result<()> {
    let mut fixture = FormFixture::new(draft_with_units());
    fixture
        .apply(field(IngredientDraftField::Name, "Changed"))
        .await?;
    fixture.apply(IngredientFormEvent::RequestClose).await?;

    let before = fixture.form.view();
    let mut command = fixture
        .form
        .update(Message::Event(event))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.form.view(), before);
    command.step().await?.expect_done();
    Ok(())
}

#[test_case(ConfirmationEvent::Confirm; "confirm")]
#[test_case(ConfirmationEvent::Cancel; "cancel")]
#[tokio::test]
async fn confirmation_events_while_editing_are_ignored(
    event: ConfirmationEvent,
) -> anyhow::Result<()> {
    let mut fixture = FormFixture::new(named_draft());
    fixture
        .apply(field(IngredientDraftField::Name, "Changed"))
        .await?;

    let before = fixture.form.view();
    let mut command = fixture
        .form
        .update(Message::Event(IngredientFormEvent::Confirmation(event)))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.form.view(), before);
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
    let mut fixture = FormFixture::new(named_draft());
    fixture
        .apply(field(IngredientDraftField::Name, "Changed"))
        .await?;
    fixture.apply(IngredientFormEvent::RequestClose).await?;
    let pending = fixture
        .form
        .update(Message::Event(IngredientFormEvent::Confirmation(first)));
    let resolved = fixture.form.view();
    assert_eq!(
        resolved.status,
        IngredientFormStatus::ConfirmingDiscard(ConfirmationView::Resolved { decision: expected })
    );

    let mut command = fixture
        .form
        .update(Message::Event(IngredientFormEvent::Confirmation(second)))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.form.view(), resolved);
    command.step().await?.expect_done();
    drop(command);

    // The first decision is still reported.
    let mut pending = pending.driver(&mut fixture.effects);
    let decision = pending.step().await?.expect_outcome();
    assert!(matches!(
        decision,
        IngredientFormOutcome::ConfirmationDecided(Latest { value: decision, .. }) if decision == expected
    ));
    pending.step().await?.expect_done();
    Ok(())
}

#[tokio::test]
async fn a_cancel_decision_delivered_twice_is_applied_once() -> anyhow::Result<()> {
    let mut fixture = FormFixture::new(named_draft());
    fixture
        .apply(field(IngredientDraftField::Name, "Changed"))
        .await?;
    fixture.apply(IngredientFormEvent::RequestClose).await?;
    let mut command = fixture
        .form
        .update(Message::Event(IngredientFormEvent::Confirmation(
            ConfirmationEvent::Cancel,
        )))
        .driver(&mut fixture.effects);
    let cancelled = command.step().await?.expect_outcome();
    command.step().await?.expect_done();
    drop(command);
    fixture.apply_outcome(cancelled.clone()).await?;
    assert_eq!(fixture.form.view().status, IngredientFormStatus::Editing);

    let before = fixture.form.view();
    let mut command = fixture
        .form
        .update(Message::Outcome(cancelled))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.form.view(), before);
    command.step().await?.expect_done();
    Ok(())
}

#[tokio::test]
async fn a_cancel_decision_from_an_earlier_dialog_does_not_close_a_reopened_dialog()
-> anyhow::Result<()> {
    let mut fixture = FormFixture::new(named_draft());
    fixture
        .apply(field(IngredientDraftField::Name, "Changed"))
        .await?;
    fixture.apply(IngredientFormEvent::RequestClose).await?;
    let mut command = fixture
        .form
        .update(Message::Event(IngredientFormEvent::Confirmation(
            ConfirmationEvent::Cancel,
        )))
        .driver(&mut fixture.effects);
    let cancelled = command.step().await?.expect_outcome();
    command.step().await?.expect_done();
    drop(command);
    fixture.apply_outcome(cancelled.clone()).await?;
    fixture.apply(IngredientFormEvent::RequestClose).await?;
    assert_eq!(
        fixture.form.view().status,
        IngredientFormStatus::ConfirmingDiscard(ConfirmationView::AwaitingDecision)
    );

    let before = fixture.form.view();
    let mut command = fixture
        .form
        .update(Message::Outcome(cancelled))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.form.view(), before);
    command.step().await?.expect_done();
    Ok(())
}

#[test_matrix(
    [Finish::Saved, Finish::Discarded],
    [
        field(IngredientDraftField::Name, "blocked"),
        IngredientFormEvent::SetBasis(NutritionBasis::Per100Millilitres),
        IngredientFormEvent::AddUnit,
        IngredientFormEvent::RemoveUnit { index: 0 },
        IngredientFormEvent::MoveUnit { from: 0, to: 1 },
        IngredientFormEvent::Save,
        IngredientFormEvent::RequestClose,
        IngredientFormEvent::Confirmation(ConfirmationEvent::Confirm)
    ]
)]
#[tokio::test]
async fn events_after_finishing_are_ignored(
    finish: Finish,
    event: IngredientFormEvent,
) -> anyhow::Result<()> {
    let mut fixture = FormFixture::new(draft_with_units());
    fixture.finish(finish).await?;
    assert_eq!(fixture.form.view().status, IngredientFormStatus::Finished);

    let before = fixture.form.view();
    let mut command = fixture
        .form
        .update(Message::Event(event))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.form.view(), before);
    command.step().await?.expect_done();
    Ok(())
}

#[tokio::test]
async fn a_save_result_delivered_twice_is_applied_once() -> anyhow::Result<()> {
    let mut fixture = FormFixture::new(named_draft());
    let mut command = fixture
        .form
        .update(Message::Event(IngredientFormEvent::Save))
        .driver(&mut fixture.effects);
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
        .resolve_save(|_| async { Ok(ingredient(NutritionBasis::Per100Grams)) })
        .await?;
    let outcome = command.step().await?.expect_outcome();
    command.step().await?.expect_done();
    drop(command);
    let mut command = fixture
        .form
        .update(Message::Outcome(outcome.clone()))
        .driver(&mut fixture.effects);
    render_then_finish(&mut command).await?;
    drop(command);

    let before = fixture.form.view();
    let mut command = fixture
        .form
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.form.view(), before);
    command.step().await?.expect_done();
    Ok(())
}

#[tokio::test]
async fn a_discard_decision_delivered_twice_is_applied_once() -> anyhow::Result<()> {
    let mut fixture = FormFixture::new(named_draft());
    fixture
        .apply(field(IngredientDraftField::Name, "Changed"))
        .await?;
    fixture.apply(IngredientFormEvent::RequestClose).await?;
    let mut command = fixture
        .form
        .update(Message::Event(IngredientFormEvent::Confirmation(
            ConfirmationEvent::Confirm,
        )))
        .driver(&mut fixture.effects);
    let confirmed = command.step().await?.expect_outcome();
    command.step().await?.expect_done();
    drop(command);
    let mut command = fixture
        .form
        .update(Message::Outcome(confirmed.clone()))
        .driver(&mut fixture.effects);
    render_then_finish(&mut command).await?;
    drop(command);

    let before = fixture.form.view();
    let mut command = fixture
        .form
        .update(Message::Outcome(confirmed))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.form.view(), before);
    command.step().await?.expect_done();
    Ok(())
}

#[tokio::test]
async fn dropping_the_form_cancels_a_pending_save_before_the_store_request() -> anyhow::Result<()> {
    let mut fixture = FormFixture::new(named_draft());
    let mut command = fixture
        .form
        .update(Message::Event(IngredientFormEvent::Save))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.form.view().status, IngredientFormStatus::Saving);
    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    drop(fixture.form);
    command.step().await?.expect_done();
    Ok(())
}

// Failures and recovery

#[test_case(IngredientSaveError::DuplicateName; "duplicate name")]
#[test_case(IngredientSaveError::NotFound; "missing record")]
#[test_case(IngredientSaveError::Storage; "storage failure")]
#[test_case(IngredientSaveError::Transport; "transport failure")]
#[tokio::test]
async fn a_rejected_save_returns_to_editing_with_the_error_and_the_draft(
    error: IngredientSaveError,
) -> anyhow::Result<()> {
    let original = ingredient(NutritionBasis::Per100Millilitres);
    let mut fixture = FormFixture::new(IngredientDraft::from(&original));
    fixture
        .apply(field(IngredientDraftField::Name, "  Updated garlic  "))
        .await?;
    let raw = fixture.form.view().draft;
    let mut command = fixture
        .form
        .update(Message::Event(IngredientFormEvent::Save))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.form.view().status, IngredientFormStatus::Saving);
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
        .resolve_save(|_| async { Err(error.clone()) })
        .await?;
    let outcome = command.step().await?.expect_outcome();
    command.step().await?.expect_done();
    drop(command);

    let mut command = fixture
        .form
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.form.view(),
        IngredientFormView {
            draft: raw,
            dirty: true,
            issues: vec![],
            save_error: Some(error),
            status: IngredientFormStatus::Editing,
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
    let mut fixture = FormFixture::new(named_draft());
    let mut command = fixture
        .form
        .update(Message::Event(IngredientFormEvent::Save))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.form.view().status, IngredientFormStatus::Saving);
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
        IngredientFormOutcome::SaveFinished(Latest {
            value: Err(IngredientSaveError::Transport),
            ..
        })
    ));
    command.step().await?.expect_done();
    drop(command);

    let mut command = fixture
        .form
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    let view = fixture.form.view();
    assert_eq!(view.status, IngredientFormStatus::Editing);
    assert_eq!(view.save_error, Some(IngredientSaveError::Transport));
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

#[test_case("Smoked garlic", vec![]; "valid edit")]
#[test_case(" ", vec![issue(ValidationField::Name, ValidationReason::Required)]; "blank name")]
#[tokio::test]
async fn editing_after_a_rejected_save_clears_the_error_and_revalidates(
    name: &str,
    expected: Vec<ValidationIssue>,
) -> anyhow::Result<()> {
    let mut fixture = FormFixture::new(named_draft());
    fixture
        .reject_save(IngredientSaveError::DuplicateName)
        .await?;
    assert_eq!(
        fixture.form.view().save_error,
        Some(IngredientSaveError::DuplicateName)
    );

    let mut command = fixture
        .form
        .update(Message::Event(field(IngredientDraftField::Name, name)))
        .driver(&mut fixture.effects);
    let view = fixture.form.view();
    assert_eq!(view.save_error, None);
    assert_eq!(view.issues, expected);
    assert_eq!(view.status, IngredientFormStatus::Editing);

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

#[test_case(IngredientSaveError::Storage; "after storage failure")]
#[test_case(IngredientSaveError::Transport; "after transport failure")]
#[tokio::test]
async fn retrying_a_rejected_save_clears_the_error_and_can_succeed(
    error: IngredientSaveError,
) -> anyhow::Result<()> {
    let mut fixture = FormFixture::new(named_draft());
    fixture.reject_save(error).await?;

    let mut command = fixture
        .form
        .update(Message::Event(IngredientFormEvent::Save))
        .driver(&mut fixture.effects);
    let view = fixture.form.view();
    assert_eq!(view.status, IngredientFormStatus::Saving);
    assert_eq!(view.save_error, None);

    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    let saved = ingredient(NutritionBasis::Per100Grams);
    command
        .step()
        .await?
        .expect_effect()
        .expect_store()
        .resolve_save(|write| {
            assert!(matches!(write, IngredientWrite::Create { .. }));
            async { Ok(saved.clone()) }
        })
        .await?;
    let outcome = command.step().await?.expect_outcome();
    command.step().await?.expect_done();
    drop(command);

    let mut command = fixture
        .form
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.form.view().status, IngredientFormStatus::Finished);
    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    assert_eq!(
        command.step().await?.expect_outcome(),
        IngredientFormOutcome::Saved { ingredient: saved }
    );
    command.step().await?.expect_done();
    Ok(())
}

#[tokio::test]
async fn a_stale_rejection_is_ignored_while_the_retry_is_saving() -> anyhow::Result<()> {
    let mut fixture = FormFixture::new(named_draft());
    let mut command = fixture
        .form
        .update(Message::Event(IngredientFormEvent::Save))
        .driver(&mut fixture.effects);
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
        .resolve_save(|_| async { Err(IngredientSaveError::Storage) })
        .await?;
    let rejected = command.step().await?.expect_outcome();
    command.step().await?.expect_done();
    drop(command);
    let mut command = fixture
        .form
        .update(Message::Outcome(rejected.clone()))
        .driver(&mut fixture.effects);
    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    command.step().await?.expect_done();
    drop(command);

    let retry = fixture
        .form
        .update(Message::Event(IngredientFormEvent::Save));
    let saving = fixture.form.view();
    assert_eq!(saving.status, IngredientFormStatus::Saving);
    let mut command = fixture
        .form
        .update(Message::Outcome(rejected))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.form.view(), saving);
    command.step().await?.expect_done();
    drop(command);

    let saved = ingredient(NutritionBasis::Per100Grams);
    let mut retry = retry.driver(&mut fixture.effects);
    retry
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    retry
        .step()
        .await?
        .expect_effect()
        .expect_store()
        .resolve_save(|_| async { Ok(saved.clone()) })
        .await?;
    let outcome = retry.step().await?.expect_outcome();
    retry.step().await?.expect_done();
    drop(retry);

    let mut command = fixture
        .form
        .update(Message::Outcome(outcome))
        .driver(&mut fixture.effects);
    assert_eq!(fixture.form.view().status, IngredientFormStatus::Finished);
    command
        .step()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    assert_eq!(
        command.step().await?.expect_outcome(),
        IngredientFormOutcome::Saved { ingredient: saved }
    );
    command.step().await?.expect_done();
    Ok(())
}

#[tokio::test]
async fn cancelling_discard_keeps_a_rejected_save_visible() -> anyhow::Result<()> {
    let mut fixture = FormFixture::new(named_draft());
    fixture
        .apply(field(IngredientDraftField::Name, "Changed"))
        .await?;
    fixture.reject_save(IngredientSaveError::Storage).await?;
    fixture.apply(IngredientFormEvent::RequestClose).await?;
    let mut command = fixture
        .form
        .update(Message::Event(IngredientFormEvent::Confirmation(
            ConfirmationEvent::Cancel,
        )))
        .driver(&mut fixture.effects);
    let cancelled = command.step().await?.expect_outcome();
    command.step().await?.expect_done();
    drop(command);

    let mut command = fixture
        .form
        .update(Message::Outcome(cancelled))
        .driver(&mut fixture.effects);
    assert_eq!(
        fixture.form.view(),
        IngredientFormView {
            draft: edited(named_draft(), |d| d.name = "Changed".into()),
            dirty: true,
            issues: vec![],
            save_error: Some(IngredientSaveError::Storage),
            status: IngredientFormStatus::Editing,
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

// Test support

#[derive(Debug, Clone, Copy)]
enum Finish {
    Saved,
    Discarded,
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

    /// Setup only: dispatch an event that renders once and completes.
    async fn apply(&mut self, event: IngredientFormEvent) -> anyhow::Result<()> {
        let mut command = self
            .form
            .update(Message::Event(event))
            .driver(&mut self.effects);
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

    /// Setup only: save, let the store reject it, and apply the rejection.
    async fn reject_save(&mut self, error: IngredientSaveError) -> anyhow::Result<()> {
        let mut command = self
            .form
            .update(Message::Event(IngredientFormEvent::Save))
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
            .resolve_save(|_| async { Err(error) })
            .await?;
        let outcome = command.step().await?.expect_outcome();
        command.step().await?.expect_done();
        drop(command);
        self.apply_outcome(outcome).await
    }

    /// Setup only: finish a clean form by saving it or by closing it.
    async fn finish(&mut self, finish: Finish) -> anyhow::Result<()> {
        match finish {
            Finish::Saved => {
                let mut command = self
                    .form
                    .update(Message::Event(IngredientFormEvent::Save))
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
                    .resolve_save(|_| async { Ok(ingredient(NutritionBasis::Per100Grams)) })
                    .await?;
                let outcome = command.step().await?.expect_outcome();
                command.step().await?.expect_done();
                drop(command);
                self.apply_outcome(outcome).await
            }
            Finish::Discarded => {
                let mut command = self
                    .form
                    .update(Message::Event(IngredientFormEvent::RequestClose))
                    .driver(&mut self.effects);
                render_then_finish(&mut command).await
            }
        }
    }

    /// Setup only: apply an outcome whose command renders once.
    async fn apply_outcome(&mut self, outcome: IngredientFormOutcome) -> anyhow::Result<()> {
        let mut command = self
            .form
            .update(Message::Outcome(outcome))
            .driver(&mut self.effects);
        render_then_finish(&mut command).await
    }
}

/// Setup only: resolve one render, then finish while ignoring outcomes.
async fn render_then_finish(
    command: &mut CommandDriver<'_, IngredientEffect, IngredientFormOutcome>,
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

fn editing_view(draft: IngredientDraft, dirty: bool) -> IngredientFormView {
    IngredientFormView {
        draft,
        dirty,
        issues: vec![],
        save_error: None,
        status: IngredientFormStatus::Editing,
    }
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

fn edited(
    mut draft: IngredientDraft,
    change: impl FnOnce(&mut IngredientDraft),
) -> IngredientDraft {
    change(&mut draft);
    draft
}

fn unit_names(draft: &IngredientDraft) -> Vec<&str> {
    draft.units.iter().map(|unit| unit.name.as_str()).collect()
}

fn blank_draft() -> IngredientDraft {
    IngredientDraft::new(NutritionBasis::Per100Grams)
}

fn named_draft() -> IngredientDraft {
    edited(blank_draft(), |draft| draft.name = "Garlic".into())
}

fn draft_with_units() -> IngredientDraft {
    edited(named_draft(), |draft| {
        draft.units = vec![
            unit("cup", "", "250"),
            unit("clove", "3", ""),
            unit("spoon", "", "15"),
        ]
    })
}

fn unit(name: &str, grams: &str, millilitres: &str) -> ReferenceUnitDraft {
    ReferenceUnitDraft {
        name: name.into(),
        grams: grams.into(),
        millilitres: millilitres.into(),
    }
}

fn decimal(value: &str) -> Decimal {
    Decimal::from_str_exact(value).unwrap()
}

fn ingredient(basis: NutritionBasis) -> Ingredient {
    Ingredient {
        id: Uuid::from_u128(1).into(),
        name: "Garlic".into(),
        notes: Some("Fresh".into()),
        nutrition: Nutrition {
            basis,
            calories: Some(decimal("12.3400").into()),
            protein: None,
            carbohydrates: Some(Decimal::ZERO.into()),
            fat: None,
        },
        units: vec![
            ReferenceUnit {
                name: "cup".into(),
                grams: None,
                millilitres: Some(decimal("250.00").into()),
            },
            ReferenceUnit {
                name: "clove".into(),
                grams: Some(decimal("1.000").into()),
                millilitres: None,
            },
        ],
    }
}
