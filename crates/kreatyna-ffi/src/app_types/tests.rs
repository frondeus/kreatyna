use futures::{StreamExt, executor::block_on};
use kreatyna::{self as core, ingredients as ingredient};
use krucyfiks::{CommandScopeId, Message, Widget};
use rust_decimal::Decimal;
use test_case::test_case;

use super::*;

#[test]
fn nested_event_conversion_preserves_both_scope_ids() {
    let event = AppEvent::Ingredients {
        scope: ScopeId { value: 17 },
        event: IngredientCatalogEvent::Form {
            scope: ScopeId { value: 29 },
            event: IngredientFormEvent::Save,
        },
    };

    let core::AppEvent::Ingredients(outer) = core::AppEvent::try_from(event).unwrap() else {
        panic!("expected ingredient event")
    };
    assert_eq!(u64::from(outer.scope), 17);
    let ingredient::IngredientCatalogEvent::Form(inner) = outer.value else {
        panic!("expected form event")
    };
    assert_eq!(u64::from(inner.scope), 29);
    assert!(matches!(inner.value, ingredient::IngredientFormEvent::Save));
}

#[test]
fn converted_stale_event_is_rejected_by_existing_app_routing() {
    block_on(async {
        let (mut app, _effects) = core::AppWidget::test_fixture();
        let old = AppView::try_from(app.view()).unwrap();

        drop(app.update(Message::Event(core::AppEvent::SelectTab(
            core::AppTab::Counter,
        ))));
        drop(app.update(Message::Event(core::AppEvent::SelectTab(
            core::AppTab::Ingredients,
        ))));

        let stale = AppEvent::Ingredients {
            scope: old.scope,
            event: IngredientCatalogEvent::List {
                scope: old.ingredients.scope,
                event: IngredientListEvent::CreateRequested,
            },
        };
        let mut command = app
            .update(Message::Event(stale.try_into().unwrap()))
            .into_stream();
        assert!(command.next().await.is_none());
        assert_eq!(app.view().tab, core::AppTab::Ingredients);
    });
}

#[test_case(IngredientListEvent::Select { id: "bad".into() }; "select")]
#[test_case(IngredientListEvent::Edit { id: "bad".into() }; "edit")]
fn malformed_ingredient_ids_are_rejected(event: IngredientListEvent) {
    assert_eq!(
        ingredient::IngredientListEvent::try_from(event),
        Err(AppTypeConversionError::InvalidIngredientId {
            value: "bad".into()
        })
    );
}

#[test]
fn index_conversion_is_checked_but_out_of_bounds_values_remain_events() {
    let converted =
        ingredient::IngredientFormEvent::try_from(IngredientFormEvent::RemoveUnit { index: 100 })
            .unwrap();
    assert!(matches!(
        converted,
        ingredient::IngredientFormEvent::RemoveUnit { index: 100 }
    ));

    let largest = IngredientFormEvent::RemoveUnit { index: u64::MAX };
    let result = ingredient::IngredientFormEvent::try_from(largest);
    if usize::BITS < u64::BITS {
        assert_eq!(result, Err(AppTypeConversionError::IntegerOutOfRange));
    } else {
        assert!(result.is_ok());
    }
}

#[test_case(core::CounterStatus::Loading, CounterStatus::Loading; "loading")]
#[test_case(core::CounterStatus::Ready, CounterStatus::Ready; "ready")]
fn counter_view_preserves_readiness(core_status: core::CounterStatus, expected: CounterStatus) {
    assert_eq!(
        CounterView::try_from(core::CounterView {
            count: 42,
            status: core_status,
        })
        .unwrap(),
        CounterView {
            count: 42,
            status: expected,
        }
    );
}

#[test]
fn list_statuses_and_typed_errors_are_preserved() {
    for (source, expected) in [
        (
            ingredient::IngredientListStatus::Loading,
            IngredientListStatus::Loading,
        ),
        (
            ingredient::IngredientListStatus::Failed(
                ingredient::capabilities::IngredientLoadError::Transport,
            ),
            IngredientListStatus::Failed {
                error: IngredientLoadError::Transport,
            },
        ),
        (
            ingredient::IngredientListStatus::Ready {
                rows: vec![ingredient::IngredientListRow {
                    id: ingredient::IngredientId::new(),
                    name: "Garlic".into(),
                }],
                total_count: 3,
                refresh: ingredient::IngredientListRefresh::Failed(
                    ingredient::capabilities::IngredientLoadError::Storage,
                ),
            },
            IngredientListStatus::Ready {
                rows: vec![IngredientListRow {
                    id: String::new(),
                    name: "Garlic".into(),
                }],
                total_count: 3,
                refresh: IngredientListRefresh::Failed {
                    error: IngredientLoadError::Storage,
                },
            },
        ),
    ] {
        let converted = IngredientListStatus::try_from(source).unwrap();
        match (converted, expected) {
            (
                IngredientListStatus::Ready {
                    rows,
                    total_count,
                    refresh,
                },
                IngredientListStatus::Ready {
                    total_count: expected_count,
                    refresh: expected_refresh,
                    ..
                },
            ) => {
                assert_eq!(rows[0].name, "Garlic");
                assert_eq!(rows[0].id.len(), 36);
                assert_eq!(total_count, expected_count);
                assert_eq!(refresh, expected_refresh);
            }
            (actual, expected) => assert_eq!(actual, expected),
        }
    }
}

#[test]
fn ingredient_values_keep_exact_text_unknowns_zero_and_unit_order() {
    let source = sample_ingredient();
    let converted = Ingredient::from(source);
    assert_eq!(converted.nutrition.calories.as_deref(), Some("12.3400"));
    assert_eq!(converted.nutrition.protein, None);
    assert_eq!(
        converted.nutrition.carbohydrates.as_deref(),
        Some("79228162514264337593543950335")
    );
    assert_eq!(converted.nutrition.fat.as_deref(), Some("0.00"));
    assert_eq!(converted.units[0].name, "clove");
    assert_eq!(converted.units[0].grams.as_deref(), Some("2.500"));
    assert_eq!(converted.units[1].name, "cup");
    assert_eq!(converted.units[1].millilitres.as_deref(), Some("250.00"));
}

#[test]
fn detail_and_confirmation_statuses_preserve_typed_delete_errors() {
    let statuses = [
        ingredient::IngredientDetailDeletion::Idle,
        ingredient::IngredientDetailDeletion::Confirming {
            confirmation: core::confirmation::ConfirmationView::AwaitingDecision,
            error: Some(ingredient::capabilities::IngredientDeleteError::Storage),
        },
        ingredient::IngredientDetailDeletion::Confirming {
            confirmation: core::confirmation::ConfirmationView::Resolved {
                decision: core::confirmation::ConfirmationOutcome::Cancelled,
            },
            error: Some(ingredient::capabilities::IngredientDeleteError::Transport),
        },
        ingredient::IngredientDetailDeletion::Deleting,
        ingredient::IngredientDetailDeletion::Deleted,
    ];

    let converted = statuses
        .into_iter()
        .map(IngredientDetailDeletion::from)
        .collect::<Vec<_>>();
    assert!(matches!(converted[0], IngredientDetailDeletion::Idle));
    assert!(matches!(
        converted[1],
        IngredientDetailDeletion::Confirming {
            confirmation: ConfirmationView::AwaitingDecision,
            error: Some(IngredientDeleteError::Storage),
        }
    ));
    assert!(matches!(
        converted[2],
        IngredientDetailDeletion::Confirming {
            confirmation: ConfirmationView::Resolved {
                decision: ConfirmationOutcome::Cancelled,
            },
            error: Some(IngredientDeleteError::Transport),
        }
    ));
    assert!(matches!(converted[3], IngredientDetailDeletion::Deleting));
    assert!(matches!(converted[4], IngredientDetailDeletion::Deleted));
}

#[test_case(ingredient::capabilities::IngredientSaveError::DuplicateName, IngredientSaveError::DuplicateName; "duplicate")]
#[test_case(ingredient::capabilities::IngredientSaveError::NotFound, IngredientSaveError::NotFound; "not found")]
#[test_case(ingredient::capabilities::IngredientSaveError::Storage, IngredientSaveError::Storage; "storage")]
#[test_case(ingredient::capabilities::IngredientSaveError::Transport, IngredientSaveError::Transport; "transport")]
fn save_errors_remain_typed(
    source: ingredient::capabilities::IngredientSaveError,
    expected: IngredientSaveError,
) {
    assert_eq!(IngredientSaveError::from(source), expected);
}

#[test]
fn every_form_status_is_representable() {
    let statuses = [
        ingredient::IngredientFormStatus::Editing,
        ingredient::IngredientFormStatus::ConfirmingDiscard(
            core::confirmation::ConfirmationView::AwaitingDecision,
        ),
        ingredient::IngredientFormStatus::Saving,
        ingredient::IngredientFormStatus::Finished,
    ];
    let converted = statuses
        .into_iter()
        .map(IngredientFormStatus::from)
        .collect::<Vec<_>>();
    assert!(matches!(converted[0], IngredientFormStatus::Editing));
    assert!(matches!(
        converted[1],
        IngredientFormStatus::ConfirmingDiscard { .. }
    ));
    assert!(matches!(converted[2], IngredientFormStatus::Saving));
    assert!(matches!(converted[3], IngredientFormStatus::Finished));
}

#[test]
fn draft_conversion_preserves_raw_invalid_text_and_validation_indices() {
    let id = ingredient::IngredientId::new();
    let source = ingredient::IngredientFormView {
        draft: ingredient::IngredientDraft {
            mode: ingredient::DraftMode::Edit { id },
            name: "  Garlic  ".into(),
            notes: "\n note \n".into(),
            basis: ingredient::NutritionBasis::Per100Millilitres,
            calories: "1,0000000".into(),
            protein: ".5".into(),
            carbohydrates: "".into(),
            fat: "-0.00".into(),
            units: vec![ingredient::ReferenceUnitDraft {
                name: " cup ".into(),
                grams: "bad".into(),
                millilitres: "250,00".into(),
            }],
        },
        dirty: true,
        issues: vec![ingredient::ValidationIssue {
            field: ingredient::ValidationField::UnitGrams { index: 0 },
            reason: ingredient::ValidationReason::InvalidNumber,
        }],
        save_error: Some(ingredient::capabilities::IngredientSaveError::DuplicateName),
        status: ingredient::IngredientFormStatus::ConfirmingDiscard(
            core::confirmation::ConfirmationView::AwaitingDecision,
        ),
    };

    let converted = IngredientFormView::try_from(source).unwrap();
    assert_eq!(converted.draft.name, "  Garlic  ");
    assert_eq!(converted.draft.calories, "1,0000000");
    assert_eq!(converted.draft.protein, ".5");
    assert_eq!(converted.draft.units[0].grams, "bad");
    assert_eq!(
        converted.issues,
        [ValidationIssue {
            field: ValidationField::UnitGrams { index: 0 },
            reason: ValidationReason::InvalidNumber,
        }]
    );
    assert_eq!(
        converted.save_error,
        Some(IngredientSaveError::DuplicateName)
    );
    assert!(matches!(
        converted.status,
        IngredientFormStatus::ConfirmingDiscard { .. }
    ));
}

#[test]
fn every_catalog_screen_and_both_edit_origins_are_representable() {
    let ingredient = sample_ingredient();
    let id = ingredient.id;
    let detail = ingredient::IngredientDetailView {
        ingredient: ingredient.clone(),
        deletion: ingredient::IngredientDetailDeletion::Idle,
    };
    let form = ingredient::IngredientFormView {
        draft: ingredient::IngredientDraft::from(&ingredient),
        dirty: false,
        issues: vec![],
        save_error: None,
        status: ingredient::IngredientFormStatus::Editing,
    };

    let cases = [
        (
            ingredient::IngredientNavigation::List,
            ingredient::IngredientCatalogScreenView::List,
            "list",
        ),
        (
            ingredient::IngredientNavigation::Detail { id },
            ingredient::IngredientCatalogScreenView::Detail(detail.clone()),
            "detail",
        ),
        (
            ingredient::IngredientNavigation::Create,
            ingredient::IngredientCatalogScreenView::Create(form.clone()),
            "create",
        ),
        (
            ingredient::IngredientNavigation::Edit {
                id,
                origin: ingredient::IngredientEditOrigin::List,
            },
            ingredient::IngredientCatalogScreenView::EditFromList(form.clone()),
            "edit-list",
        ),
        (
            ingredient::IngredientNavigation::Edit {
                id,
                origin: ingredient::IngredientEditOrigin::Detail,
            },
            ingredient::IngredientCatalogScreenView::EditFromDetail { detail, form },
            "edit-detail",
        ),
    ];

    for (navigation, screen, expected) in cases {
        let converted = IngredientCatalogView::try_from(ingredient::IngredientCatalogView {
            navigation,
            list: ingredient::IngredientListView {
                search: "gar".into(),
                status: ingredient::IngredientListStatus::Loading,
            },
            scope: CommandScopeId::from(91),
            screen,
        })
        .unwrap();
        assert_eq!(converted.scope, ScopeId { value: 91 });
        assert_eq!(converted.list.search, "gar");
        let actual = match converted.screen {
            IngredientCatalogScreenView::List => "list",
            IngredientCatalogScreenView::Detail { .. } => "detail",
            IngredientCatalogScreenView::Create { .. } => "create",
            IngredientCatalogScreenView::EditFromList { .. } => "edit-list",
            IngredientCatalogScreenView::EditFromDetail { .. } => "edit-detail",
        };
        assert_eq!(actual, expected);
    }
}

fn sample_ingredient() -> ingredient::Ingredient {
    ingredient::Ingredient {
        id: ingredient::IngredientId::new(),
        name: "Garlic".into(),
        notes: Some("Fresh".into()),
        nutrition: ingredient::Nutrition {
            basis: ingredient::NutritionBasis::Per100Grams,
            calories: Some(Decimal::from_str_exact("12.3400").unwrap().into()),
            protein: None,
            carbohydrates: Some(Decimal::MAX.into()),
            fat: Some(Decimal::from_str_exact("0.00").unwrap().into()),
        },
        units: vec![
            ingredient::ReferenceUnit {
                name: "clove".into(),
                grams: Some(Decimal::from_str_exact("2.500").unwrap().into()),
                millilitres: None,
            },
            ingredient::ReferenceUnit {
                name: "cup".into(),
                grams: None,
                millilitres: Some(Decimal::from_str_exact("250.00").unwrap().into()),
            },
        ],
    }
}
