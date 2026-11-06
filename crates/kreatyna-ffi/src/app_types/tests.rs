use anyhow::Context;
use futures::StreamExt;
use kreatyna::{self as core, ingredients as ingredient};
use krucyfiks::{CommandScopeId, Message, Scoped, Widget};
use rust_decimal::Decimal;
use test_case::test_case;

use super::*;

// Green path

#[test]
fn nested_event_conversion_preserves_the_catalog_scope() -> anyhow::Result<()> {
    let event = AppEvent::Stack {
        event: IngredientCatalogEvent::Form {
            scope: ScopeId { value: 29 },
            event: IngredientFormEvent::Save,
        },
    };

    let core::AppEvent::Stack(ingredient::IngredientCatalogEvent::Form(inner)) =
        core::AppEvent::try_from(event)?
    else {
        anyhow::bail!("expected form event");
    };
    assert_eq!(u64::from(inner.scope), 29);
    assert!(matches!(inner.value, ingredient::IngredientFormEvent::Save));
    Ok(())
}

#[test]
fn split_event_conversion_preserves_the_scopes() -> anyhow::Result<()> {
    let clear = AppEvent::Split {
        event: SplitAppEvent::Ingredients {
            event: IngredientSplitCatalogEvent::ClearSelection {
                scope: ScopeId { value: 7 },
            },
        },
    };
    let core::AppEvent::Split(core::SplitAppEvent::Ingredients(
        ingredient::IngredientSplitCatalogEvent::ClearSelection { scope },
    )) = core::AppEvent::try_from(clear)?
    else {
        anyhow::bail!("expected a clear-selection event");
    };
    assert_eq!(u64::from(scope), 7);

    let detail = AppEvent::Split {
        event: SplitAppEvent::Ingredients {
            event: IngredientSplitCatalogEvent::Detail {
                scope: ScopeId { value: 8 },
                event: IngredientDetailEvent::Edit,
            },
        },
    };
    let core::AppEvent::Split(core::SplitAppEvent::Ingredients(
        ingredient::IngredientSplitCatalogEvent::Detail(inner),
    )) = core::AppEvent::try_from(detail)?
    else {
        anyhow::bail!("expected a detail event");
    };
    assert_eq!(u64::from(inner.scope), 8);
    assert_eq!(inner.value, ingredient::IngredientDetailEvent::Edit);
    Ok(())
}

#[test]
fn split_view_conversion_keeps_the_selection_and_the_detail() -> anyhow::Result<()> {
    let id = ingredient::IngredientId::generate();
    let garlic = ingredient::Ingredient {
        id,
        name: ingredient::IngredientName::try_new("Garlic")?,
        notes: None,
        nutrition: ingredient::Nutrition {
            basis: ingredient::NutritionBasis::Per100Grams,
            calories: None,
            protein: None,
            carbohydrates: None,
            fat: None,
        },
        units: vec![],
    };
    let view = ingredient::IngredientSplitCatalogView {
        list: Scoped::new(
            CommandScopeId::from(3),
            ingredient::IngredientListView {
                search: String::new(),
                status: ingredient::IngredientListStatus::Loading,
            },
        ),
        selected: Some(id),
        pane: Scoped::new(
            CommandScopeId::from(4),
            ingredient::IngredientSplitPaneView::Detail(ingredient::IngredientDetailView {
                ingredient: garlic,
                deletion: ingredient::IngredientDetailDeletion::Idle,
            }),
        ),
        create: core::CreateAvailability::Available,
        draft_row: None,
    };

    let catalog = IngredientSplitCatalogView::try_from(view)?;
    assert_eq!(catalog.selected, Some(id));
    assert_eq!(catalog.list.scope, ScopeId { value: 3 });
    assert_eq!(catalog.pane.scope, ScopeId { value: 4 });
    let IngredientSplitPaneView::Detail { detail } = catalog.pane.value else {
        anyhow::bail!("expected a detail pane");
    };
    assert_eq!(detail.ingredient.name, "Garlic");
    assert_eq!(catalog.create, CreateAvailability::Available);
    assert_eq!(catalog.draft_row, None);
    Ok(())
}

#[test]
fn split_form_event_conversion_keeps_the_pane_scope() -> anyhow::Result<()> {
    let save = AppEvent::Split {
        event: SplitAppEvent::Ingredients {
            event: IngredientSplitCatalogEvent::Form {
                scope: ScopeId { value: 5 },
                event: IngredientFormEvent::Save,
            },
        },
    };
    let core::AppEvent::Split(core::SplitAppEvent::Ingredients(
        ingredient::IngredientSplitCatalogEvent::Form(inner),
    )) = core::AppEvent::try_from(save)?
    else {
        anyhow::bail!("expected a form event");
    };
    assert_eq!(u64::from(inner.scope), 5);
    assert_eq!(inner.value, ingredient::IngredientFormEvent::Save);
    Ok(())
}

#[test]
fn split_view_conversion_carries_the_create_form_and_the_draft_row() -> anyhow::Result<()> {
    let mut draft = ingredient::IngredientDraft::new(ingredient::NutritionBasis::Per100Grams);
    draft.set_field(ingredient::IngredientDraftField::Name, "Paprika".into());
    let view = ingredient::IngredientSplitCatalogView {
        list: Scoped::new(
            CommandScopeId::from(3),
            ingredient::IngredientListView {
                search: String::new(),
                status: ingredient::IngredientListStatus::Loading,
            },
        ),
        selected: None,
        pane: Scoped::new(
            CommandScopeId::from(6),
            ingredient::IngredientSplitPaneView::Create(ingredient::IngredientFormView {
                draft,
                issues: vec![],
                save_error: None,
                status: ingredient::IngredientFormStatus::Editing,
            }),
        ),
        create: core::CreateAvailability::Unavailable,
        draft_row: Some("Paprika".into()),
    };

    let catalog = IngredientSplitCatalogView::try_from(view)?;
    assert_eq!(catalog.pane.scope, ScopeId { value: 6 });
    assert_eq!(catalog.create, CreateAvailability::Unavailable);
    assert_eq!(catalog.draft_row.as_deref(), Some("Paprika"));
    let IngredientSplitPaneView::Create { form } = catalog.pane.value else {
        anyhow::bail!("expected the create form");
    };
    assert_eq!(form.draft.name, "Paprika");
    Ok(())
}

#[test]
fn split_view_conversion_carries_the_edit_form_and_its_saved_name() -> anyhow::Result<()> {
    let id = ingredient::IngredientId::generate();
    let mut draft = ingredient::IngredientDraft::new(ingredient::NutritionBasis::Per100Grams);
    draft.set_field(
        ingredient::IngredientDraftField::Name,
        "Black Garlic".into(),
    );
    let view = ingredient::IngredientSplitCatalogView {
        list: Scoped::new(
            CommandScopeId::from(3),
            ingredient::IngredientListView {
                search: String::new(),
                status: ingredient::IngredientListStatus::Loading,
            },
        ),
        selected: Some(id),
        pane: Scoped::new(
            CommandScopeId::from(7),
            ingredient::IngredientSplitPaneView::Edit {
                saved_name: "Garlic".into(),
                form: ingredient::IngredientFormView {
                    draft,
                    issues: vec![],
                    save_error: None,
                    status: ingredient::IngredientFormStatus::Editing,
                },
            },
        ),
        create: core::CreateAvailability::Unavailable,
        draft_row: None,
    };

    let catalog = IngredientSplitCatalogView::try_from(view)?;
    assert_eq!(catalog.selected, Some(id));
    assert_eq!(catalog.pane.scope, ScopeId { value: 7 });
    let IngredientSplitPaneView::Edit { saved_name, form } = catalog.pane.value else {
        anyhow::bail!("expected the edit form");
    };
    assert_eq!(saved_name, "Garlic");
    assert_eq!(form.draft.name, "Black Garlic");
    Ok(())
}

#[test]
fn split_app_event_conversion_reaches_the_sections() {
    let select = AppEvent::Split {
        event: SplitAppEvent::SelectSection {
            section: AppSection::Ingredients,
        },
    };
    assert!(matches!(
        core::AppEvent::try_from(select),
        Ok(core::AppEvent::Split(core::SplitAppEvent::SelectSection(
            core::AppSection::Ingredients
        )))
    ));

    let refresh = AppEvent::Split {
        event: SplitAppEvent::Recipes {
            event: RecipeSectionEvent::List {
                event: RecipeListEvent::Refresh,
            },
        },
    };
    assert!(matches!(
        core::AppEvent::try_from(refresh),
        Ok(core::AppEvent::Split(core::SplitAppEvent::Recipes(
            kreatyna::recipes::RecipeSectionEvent::List(
                kreatyna::recipes::RecipeListEvent::Refresh
            )
        )))
    ));

    let search = AppEvent::Split {
        event: SplitAppEvent::Recipes {
            event: RecipeSectionEvent::List {
                event: RecipeListEvent::Search {
                    query: "sha".into(),
                },
            },
        },
    };
    assert!(matches!(
        core::AppEvent::try_from(search),
        Ok(core::AppEvent::Split(core::SplitAppEvent::Recipes(
            kreatyna::recipes::RecipeSectionEvent::List(
                kreatyna::recipes::RecipeListEvent::Search(query)
            )
        ))) if query == "sha"
    ));
}

#[test]
fn recipe_section_events_keep_their_pane_scope() {
    let scope = ScopeId { value: 7 };
    let editor = RecipeSectionEvent::Editor {
        scope,
        event: RecipeEditorEvent::SetTitle {
            title: " Pasta ".into(),
        },
    };
    assert_eq!(
        kreatyna::recipes::RecipeSectionEvent::from(editor),
        kreatyna::recipes::RecipeSectionEvent::Editor(Scoped::new(
            CommandScopeId::from(7),
            kreatyna::recipes::RecipeEditorEvent::SetTitle(" Pasta ".into())
        ))
    );
    let retry = RecipeSectionEvent::Detail {
        scope,
        event: RecipeDetailEvent::Retry,
    };
    assert_eq!(
        kreatyna::recipes::RecipeSectionEvent::from(retry),
        kreatyna::recipes::RecipeSectionEvent::Detail(Scoped::new(
            CommandScopeId::from(7),
            kreatyna::recipes::RecipeDetailEvent::Retry
        ))
    );
    let edit = RecipeSectionEvent::Detail {
        scope,
        event: RecipeDetailEvent::Edit,
    };
    assert_eq!(
        kreatyna::recipes::RecipeSectionEvent::from(edit),
        kreatyna::recipes::RecipeSectionEvent::Detail(Scoped::new(
            CommandScopeId::from(7),
            kreatyna::recipes::RecipeDetailEvent::Edit
        ))
    );
    let delete = RecipeSectionEvent::Detail {
        scope,
        event: RecipeDetailEvent::RequestDelete,
    };
    assert_eq!(
        kreatyna::recipes::RecipeSectionEvent::from(delete),
        kreatyna::recipes::RecipeSectionEvent::Detail(Scoped::new(
            CommandScopeId::from(7),
            kreatyna::recipes::RecipeDetailEvent::RequestDelete
        ))
    );
    let confirm = RecipeSectionEvent::Detail {
        scope,
        event: RecipeDetailEvent::Confirmation {
            event: ConfirmationEvent::Confirm,
        },
    };
    assert_eq!(
        kreatyna::recipes::RecipeSectionEvent::from(confirm),
        kreatyna::recipes::RecipeSectionEvent::Detail(Scoped::new(
            CommandScopeId::from(7),
            kreatyna::recipes::RecipeDetailEvent::Confirmation(
                core::confirmation::ConfirmationEvent::Confirm
            )
        ))
    );
    assert_eq!(
        kreatyna::recipes::RecipeSectionEvent::from(RecipeSectionEvent::ClearSelection { scope }),
        kreatyna::recipes::RecipeSectionEvent::ClearSelection {
            scope: CommandScopeId::from(7)
        }
    );
}

#[test]
fn a_recipe_row_click_carries_the_recipe_id() {
    let id = kreatyna::recipes::RecipeId::generate();
    assert_eq!(
        kreatyna::recipes::RecipeSectionEvent::from(RecipeSectionEvent::Select { id }),
        kreatyna::recipes::RecipeSectionEvent::Select(id)
    );
}

#[test]
fn a_recipe_id_crosses_the_ffi_as_its_uuid_text() -> anyhow::Result<()> {
    let id = kreatyna::recipes::RecipeId::generate();
    assert_eq!(support::lift_recipe_id(id.to_string())?, id);
    Ok(())
}

#[test_case(
    StepCanvasEvent::Add { at: CanvasPoint { x: 120, y: 80 } },
    kreatyna::recipes::StepCanvasEvent::Add { at: kreatyna::recipes::CanvasPoint { x: 120, y: 80 } };
    "add a step at the point"
)]
#[test_case(
    StepCanvasEvent::Select { id: step_id(1) },
    kreatyna::recipes::StepCanvasEvent::Select(step_id(1));
    "select a step"
)]
#[test_case(
    StepCanvasEvent::ClearSelection,
    kreatyna::recipes::StepCanvasEvent::ClearSelection;
    "clear the selection"
)]
#[test_case(
    StepCanvasEvent::SetTitle { id: step_id(1), title: " Dice onion ".into() },
    kreatyna::recipes::StepCanvasEvent::SetTitle { id: step_id(1), title: " Dice onion ".into() };
    "name a step with the raw text"
)]
#[test_case(
    StepCanvasEvent::Move { id: step_id(1), to: CanvasPoint { x: 320, y: 200 } },
    kreatyna::recipes::StepCanvasEvent::Move { id: step_id(1), to: kreatyna::recipes::CanvasPoint { x: 320, y: 200 } };
    "move a step to the drop point"
)]
#[test_case(
    StepCanvasEvent::Delete { id: step_id(1) },
    kreatyna::recipes::StepCanvasEvent::Delete(step_id(1));
    "delete a step"
)]
#[test_case(
    StepCanvasEvent::SetDuration { id: step_id(1), input: DurationInput::Cleared },
    kreatyna::recipes::StepCanvasEvent::SetDuration { id: step_id(1), input: kreatyna::recipes::DurationInput::Cleared };
    "clear the duration"
)]
#[test_case(
    StepCanvasEvent::SetDuration { id: step_id(1), input: DurationInput::Minutes { minutes: 10 } },
    kreatyna::recipes::StepCanvasEvent::SetDuration { id: step_id(1), input: kreatyna::recipes::DurationInput::Minutes(10) };
    "a chip"
)]
#[test_case(
    StepCanvasEvent::SetDuration { id: step_id(1), input: DurationInput::Parts { parts: DurationParts { days: 1, hours: 2, minutes: 3, seconds: 4 } } },
    kreatyna::recipes::StepCanvasEvent::SetDuration {
        id: step_id(1),
        input: kreatyna::recipes::DurationInput::Parts(kreatyna::recipes::DurationParts { days: 1, hours: 2, minutes: 3, seconds: 4 }),
    };
    "the unit fields"
)]
#[test_case(
    StepCanvasEvent::SetKind { id: step_id(1), kind: StepKind::Waiting },
    kreatyna::recipes::StepCanvasEvent::SetKind { id: step_id(1), kind: kreatyna::recipes::StepKind::Waiting };
    "waiting"
)]
#[test_case(
    StepCanvasEvent::SetKind { id: step_id(1), kind: StepKind::Active },
    kreatyna::recipes::StepCanvasEvent::SetKind { id: step_id(1), kind: kreatyna::recipes::StepKind::Active };
    "active"
)]
#[test_case(
    StepCanvasEvent::SetInstructions { id: step_id(1), text: " Stir. ".into() },
    kreatyna::recipes::StepCanvasEvent::SetInstructions { id: step_id(1), text: " Stir. ".into() };
    "instructions as typed"
)]
#[test_case(
    StepCanvasEvent::Connect { from: step_id(1), to: step_id(2) },
    kreatyna::recipes::StepCanvasEvent::Connect { from: step_id(1), to: step_id(2) };
    "connect"
)]
#[test_case(
    StepCanvasEvent::SelectConnection { edge: StepEdge { from: step_id(1), to: step_id(2) } },
    kreatyna::recipes::StepCanvasEvent::SelectConnection(kreatyna::recipes::StepEdge { from: step_id(1), to: step_id(2) });
    "select a connection"
)]
#[test_case(
    StepCanvasEvent::DeleteConnection { edge: StepEdge { from: step_id(1), to: step_id(2) } },
    kreatyna::recipes::StepCanvasEvent::DeleteConnection(kreatyna::recipes::StepEdge { from: step_id(1), to: step_id(2) });
    "delete a connection"
)]
fn canvas_events_reach_the_editor_in_their_pane_scope(
    event: StepCanvasEvent,
    expected: kreatyna::recipes::StepCanvasEvent,
) {
    let event = RecipeSectionEvent::Editor {
        scope: ScopeId { value: 7 },
        event: RecipeEditorEvent::Canvas { event },
    };
    assert_eq!(
        kreatyna::recipes::RecipeSectionEvent::from(event),
        kreatyna::recipes::RecipeSectionEvent::Editor(Scoped::new(
            CommandScopeId::from(7),
            kreatyna::recipes::RecipeEditorEvent::Canvas(expected)
        ))
    );
}

#[test_case(
    RecipeIngredientsEvent::Add,
    kreatyna::recipes::RecipeIngredientsEvent::Add;
    "add an ingredient"
)]
#[test_case(
    RecipeIngredientsEvent::SetName { id: ingredient_id(1), name: " eggs ".into() },
    kreatyna::recipes::RecipeIngredientsEvent::SetName { id: ingredient_id(1), name: " eggs ".into() };
    "a name as typed"
)]
#[test_case(
    RecipeIngredientsEvent::Remove { id: ingredient_id(1) },
    kreatyna::recipes::RecipeIngredientsEvent::Remove { id: ingredient_id(1) };
    "remove an ingredient"
)]
#[test_case(
    RecipeIngredientsEvent::Move { id: ingredient_id(2), onto: ingredient_id(1) },
    kreatyna::recipes::RecipeIngredientsEvent::Move { id: ingredient_id(2), onto: ingredient_id(1) };
    "move an ingredient"
)]
fn ingredient_events_reach_the_editor_in_their_pane_scope(
    event: RecipeIngredientsEvent,
    expected: kreatyna::recipes::RecipeIngredientsEvent,
) {
    let event = RecipeSectionEvent::Editor {
        scope: ScopeId { value: 7 },
        event: RecipeEditorEvent::Ingredients { event },
    };
    assert_eq!(
        kreatyna::recipes::RecipeSectionEvent::from(event),
        kreatyna::recipes::RecipeSectionEvent::Editor(Scoped::new(
            CommandScopeId::from(7),
            kreatyna::recipes::RecipeEditorEvent::Ingredients(expected)
        ))
    );
}

#[test]
fn an_editor_keeps_the_ingredients_in_order_with_their_names_as_typed_and_problems() {
    let editor = kreatyna::recipes::RecipeEditorView {
        title: "Shakshuka".into(),
        ingredients: vec![
            kreatyna::recipes::RecipeIngredientView {
                id: ingredient_id(1),
                name: " eggs ".into(),
                problem: None,
            },
            kreatyna::recipes::RecipeIngredientView {
                id: ingredient_id(2),
                name: " ".into(),
                problem: Some(kreatyna::recipes::RecipeIngredientNameError::NotEmptyViolated),
            },
        ],
        canvas: kreatyna::recipes::StepCanvasView {
            nodes: kreatyna::recipes::StepNodesView::Unselected { steps: vec![] },
            connections: vec![],
            refusal: None,
            duration_chips: vec![],
        },
        status: kreatyna::recipes::RecipeEditorStatus::Editing { problem: None },
    };
    assert_eq!(
        RecipeEditorView::from(editor).ingredients,
        [
            RecipeIngredientView {
                id: ingredient_id(1),
                name: " eggs ".into(),
                problem: None,
            },
            RecipeIngredientView {
                id: ingredient_id(2),
                name: " ".into(),
                problem: Some(RecipeIngredientNameError::NotEmptyViolated),
            },
        ]
    );
}

#[test]
fn an_editor_canvas_keeps_the_selection_the_details_the_problems_and_the_arrows() {
    let untitled = kreatyna::recipes::StepNodeView {
        id: step_id(2),
        title: " ".into(),
        label: kreatyna::recipes::StepLabel::Untitled,
        position: kreatyna::recipes::CanvasPoint { x: 240, y: 40 },
        duration: kreatyna::recipes::StepDurationView::TooLong {
            parts: kreatyna::recipes::DurationParts {
                days: 400,
                ..Default::default()
            },
        },
        kind: kreatyna::recipes::StepKind::Waiting,
        instructions: " Stir. ".into(),
        problems: kreatyna::recipes::StepProblems {
            title: Some(kreatyna::recipes::StepTitleError::NotEmptyViolated),
            duration: Some(kreatyna::recipes::DurationError::TooLong),
        },
        after: vec![kreatyna::recipes::StepAfterView {
            id: step_id(1),
            label: kreatyna::recipes::StepLabel::Titled {
                title: "Dice onion".into(),
            },
        }],
    };
    let named = kreatyna::recipes::StepNodeView {
        id: step_id(1),
        title: "Dice onion".into(),
        label: kreatyna::recipes::StepLabel::Titled {
            title: "Dice onion".into(),
        },
        position: kreatyna::recipes::CanvasPoint { x: 40, y: 40 },
        duration: kreatyna::recipes::StepDurationView::Set {
            text: "1 min 30 s".into(),
            parts: kreatyna::recipes::DurationParts {
                days: 0,
                hours: 0,
                minutes: 1,
                seconds: 30,
            },
            entry: kreatyna::recipes::DurationEntry::Seconds,
        },
        kind: kreatyna::recipes::StepKind::Active,
        instructions: String::new(),
        problems: kreatyna::recipes::StepProblems::default(),
        after: vec![],
    };
    let editor = kreatyna::recipes::RecipeEditorView {
        title: "Shakshuka".into(),
        ingredients: vec![],
        canvas: kreatyna::recipes::StepCanvasView {
            nodes: kreatyna::recipes::StepNodesView::Selected {
                selected: untitled,
                others: vec![named],
            },
            connections: vec![kreatyna::recipes::StepEdge {
                from: step_id(1),
                to: step_id(2),
            }],
            refusal: Some(kreatyna::recipes::ConnectRefusal::Loop),
            duration_chips: vec![kreatyna::recipes::DurationChip {
                minutes: 60,
                text: "1 h".into(),
            }],
        },
        status: kreatyna::recipes::RecipeEditorStatus::Editing { problem: None },
    };
    assert_eq!(
        RecipeEditorView::from(editor),
        RecipeEditorView {
            title: "Shakshuka".into(),
            ingredients: vec![],
            canvas: StepCanvasView {
                nodes: StepNodesView::Selected {
                    selected: StepNodeView {
                        id: step_id(2),
                        title: " ".into(),
                        label: StepLabel::Untitled,
                        position: CanvasPoint { x: 240, y: 40 },
                        duration: StepDurationView::TooLong {
                            parts: DurationParts {
                                days: 400,
                                hours: 0,
                                minutes: 0,
                                seconds: 0,
                            },
                        },
                        kind: StepKind::Waiting,
                        instructions: " Stir. ".into(),
                        problems: StepProblems {
                            title: Some(StepTitleError::NotEmptyViolated),
                            duration: Some(DurationError::TooLong),
                        },
                        after: vec![StepAfterView {
                            id: step_id(1),
                            label: StepLabel::Titled {
                                title: "Dice onion".into(),
                            },
                        }],
                    },
                    others: vec![StepNodeView {
                        id: step_id(1),
                        title: "Dice onion".into(),
                        label: StepLabel::Titled {
                            title: "Dice onion".into(),
                        },
                        position: CanvasPoint { x: 40, y: 40 },
                        duration: StepDurationView::Set {
                            text: "1 min 30 s".into(),
                            parts: DurationParts {
                                days: 0,
                                hours: 0,
                                minutes: 1,
                                seconds: 30,
                            },
                            entry: DurationEntry::Seconds,
                        },
                        kind: StepKind::Active,
                        instructions: String::new(),
                        problems: StepProblems {
                            title: None,
                            duration: None,
                        },
                        after: vec![],
                    }],
                },
                connections: vec![StepEdge {
                    from: step_id(1),
                    to: step_id(2),
                }],
                refusal: Some(ConnectRefusal::Loop),
                duration_chips: vec![DurationChip {
                    minutes: 60,
                    text: "1 h".into(),
                }],
            },
            status: RecipeEditorStatus::Editing { problem: None },
        }
    );
}

#[test]
fn an_unselected_canvas_keeps_the_steps_in_the_order_added() -> anyhow::Result<()> {
    let node = |n: u128, x: u32| kreatyna::recipes::StepNodeView {
        id: step_id(n),
        title: format!("Step {n}"),
        label: kreatyna::recipes::StepLabel::Titled {
            title: format!("Step {n}"),
        },
        position: kreatyna::recipes::CanvasPoint { x, y: 40 },
        duration: kreatyna::recipes::StepDurationView::NotSet,
        kind: kreatyna::recipes::StepKind::Active,
        instructions: String::new(),
        problems: kreatyna::recipes::StepProblems::default(),
        after: vec![],
    };
    let nodes = kreatyna::recipes::StepNodesView::Unselected {
        steps: vec![node(2, 40), node(1, 240)],
    };
    let StepNodesView::Unselected { steps } = StepNodesView::from(nodes) else {
        anyhow::bail!("expected an unselected canvas");
    };
    assert_eq!(
        steps.iter().map(|step| step.id).collect::<Vec<_>>(),
        [step_id(2), step_id(1)]
    );
    Ok(())
}

#[test]
fn a_selected_connection_keeps_its_edge_its_labels_and_the_steps() -> anyhow::Result<()> {
    let node = |n: u128| kreatyna::recipes::StepNodeView {
        id: step_id(n),
        title: String::new(),
        label: kreatyna::recipes::StepLabel::Untitled,
        position: kreatyna::recipes::CanvasPoint { x: 40, y: 40 },
        duration: kreatyna::recipes::StepDurationView::NotSet,
        kind: kreatyna::recipes::StepKind::Active,
        instructions: String::new(),
        problems: kreatyna::recipes::StepProblems::default(),
        after: vec![],
    };
    let nodes = kreatyna::recipes::StepNodesView::ConnectionSelected {
        connection: kreatyna::recipes::StepConnectionView {
            edge: kreatyna::recipes::StepEdge {
                from: step_id(1),
                to: step_id(2),
            },
            from: kreatyna::recipes::StepLabel::Titled {
                title: "Dice onion".into(),
            },
            to: kreatyna::recipes::StepLabel::Untitled,
        },
        steps: vec![node(1), node(2)],
    };
    let StepNodesView::ConnectionSelected { connection, steps } = StepNodesView::from(nodes) else {
        anyhow::bail!("expected a selected connection");
    };
    assert_eq!(
        connection,
        StepConnectionView {
            edge: StepEdge {
                from: step_id(1),
                to: step_id(2),
            },
            from: StepLabel::Titled {
                title: "Dice onion".into(),
            },
            to: StepLabel::Untitled,
        }
    );
    assert_eq!(
        steps.iter().map(|step| step.id).collect::<Vec<_>>(),
        [step_id(1), step_id(2)]
    );
    Ok(())
}

#[test]
fn a_recipe_ingredient_id_crosses_the_ffi_as_its_uuid_text() -> anyhow::Result<()> {
    let id = kreatyna::recipes::RecipeIngredientId::generate();
    assert_eq!(support::lift_recipe_ingredient_id(id.to_string())?, id);
    Ok(())
}

#[test]
fn a_step_id_crosses_the_ffi_as_its_uuid_text() -> anyhow::Result<()> {
    let id = kreatyna::recipes::StepId::generate();
    assert_eq!(support::lift_step_id(id.to_string())?, id);
    Ok(())
}

#[test_case(kreatyna::recipes::RecipeDetailStatus::Loading, RecipeDetailStatus::Loading; "loading")]
#[test_case(
    kreatyna::recipes::RecipeDetailStatus::Failed(kreatyna::recipes::capabilities::RecipeLoadError::Transport),
    RecipeDetailStatus::Failed { error: RecipeLoadError::Transport };
    "failed"
)]
#[test_case(kreatyna::recipes::RecipeDetailStatus::Missing, RecipeDetailStatus::Missing; "missing")]
#[test_case(
    kreatyna::recipes::RecipeDetailStatus::Ready {
        title: "Shakshuka".into(),
        ingredients: vec![
            kreatyna::recipes::RecipeDetailIngredient {
                id: ingredient_id(1), amount: None, name: "eggs".into(), note: None,
            },
            kreatyna::recipes::RecipeDetailIngredient {
                id: ingredient_id(2), amount: None, name: "tomatoes".into(), note: None,
            },
        ],
        steps: vec![
            kreatyna::recipes::RecipeDetailStep {
                id: step_id(1), title: "Fry onion".into(), duration: Some("1 h 30 min".into()),
                kind: kreatyna::recipes::StepKind::Waiting, instructions: Some("Low heat.".into()),
            },
            kreatyna::recipes::RecipeDetailStep {
                id: step_id(2), title: "Dice onion".into(), duration: None,
                kind: kreatyna::recipes::StepKind::Active, instructions: None,
            },
        ],
        deletion: kreatyna::recipes::RecipeDeletion::Idle,
    },
    RecipeDetailStatus::Ready {
        title: "Shakshuka".into(),
        ingredients: vec![
            RecipeDetailIngredient { id: ingredient_id(1), name: "eggs".into() },
            RecipeDetailIngredient { id: ingredient_id(2), name: "tomatoes".into() },
        ],
        steps: vec![
            RecipeDetailStep {
                id: step_id(1), title: "Fry onion".into(), duration: Some("1 h 30 min".into()),
                kind: StepKind::Waiting, instructions: Some("Low heat.".into()),
            },
            RecipeDetailStep {
                id: step_id(2), title: "Dice onion".into(), duration: None,
                kind: StepKind::Active, instructions: None,
            },
        ],
        deletion: RecipeDeletion::Idle,
    };
    "ready with ingredients and steps in list order, and the step details"
)]
#[test_case(
    kreatyna::recipes::RecipeDetailStatus::Ready { title: "Shakshuka".into(), ingredients: vec![], steps: vec![], deletion: kreatyna::recipes::RecipeDeletion::Idle },
    RecipeDetailStatus::Ready { title: "Shakshuka".into(), ingredients: vec![], steps: vec![], deletion: RecipeDeletion::Idle };
    "ready without steps"
)]
#[test_case(
    kreatyna::recipes::RecipeDetailStatus::Ready {
        title: "Shakshuka".into(),
        ingredients: vec![],
        steps: vec![],
        deletion: kreatyna::recipes::RecipeDeletion::Confirming {
            confirmation: core::confirmation::ConfirmationView::AwaitingDecision,
            error: Some(kreatyna::recipes::capabilities::RecipeDeleteError::Storage),
        },
    },
    RecipeDetailStatus::Ready {
        title: "Shakshuka".into(),
        ingredients: vec![],
        steps: vec![],
        deletion: RecipeDeletion::Confirming {
            confirmation: ConfirmationView::AwaitingDecision,
            error: Some(RecipeDeleteError::Storage),
        },
    };
    "asking again after a failed delete"
)]
#[test_case(
    kreatyna::recipes::RecipeDetailStatus::Ready { title: "Shakshuka".into(), ingredients: vec![], steps: vec![], deletion: kreatyna::recipes::RecipeDeletion::Deleting },
    RecipeDetailStatus::Ready { title: "Shakshuka".into(), ingredients: vec![], steps: vec![], deletion: RecipeDeletion::Deleting };
    "deleting"
)]
#[test_case(
    kreatyna::recipes::RecipeDetailStatus::Ready { title: "Shakshuka".into(), ingredients: vec![], steps: vec![], deletion: kreatyna::recipes::RecipeDeletion::Deleted },
    RecipeDetailStatus::Ready { title: "Shakshuka".into(), ingredients: vec![], steps: vec![], deletion: RecipeDeletion::Deleted };
    "deleted"
)]
fn a_read_view_pane_keeps_the_id_and_the_status(
    status: kreatyna::recipes::RecipeDetailStatus,
    expected: RecipeDetailStatus,
) {
    let id = kreatyna::recipes::RecipeId::generate();
    let pane = kreatyna::recipes::RecipePaneView::Detail(kreatyna::recipes::RecipeDetailView {
        id,
        status,
    });
    assert_eq!(
        RecipePaneView::from(pane),
        RecipePaneView::Detail {
            detail: RecipeDetailView {
                id,
                status: expected,
            }
        }
    );
}

#[test]
fn a_create_pane_keeps_the_draft_row_and_the_problem() {
    let pane = kreatyna::recipes::RecipePaneView::Create {
        editor: kreatyna::recipes::RecipeEditorView {
            title: "shakshuka ".into(),
            ingredients: vec![],
            canvas: kreatyna::recipes::StepCanvasView {
                nodes: kreatyna::recipes::StepNodesView::Unselected { steps: vec![] },
                connections: vec![],
                refusal: None,
                duration_chips: vec![],
            },
            status: kreatyna::recipes::RecipeEditorStatus::Editing {
                problem: Some(kreatyna::recipes::RecipeEditorProblem::SaveFailed(
                    kreatyna::recipes::capabilities::RecipeSaveError::DuplicateTitle,
                )),
            },
        },
        draft_row: "shakshuka".into(),
    };
    assert_eq!(
        RecipePaneView::from(pane),
        RecipePaneView::Create {
            editor: RecipeEditorView {
                title: "shakshuka ".into(),
                ingredients: vec![],
                canvas: StepCanvasView {
                    nodes: StepNodesView::Unselected { steps: vec![] },
                    connections: vec![],
                    refusal: None,
                    duration_chips: vec![],
                },
                status: RecipeEditorStatus::Editing {
                    problem: Some(RecipeEditorProblem::SaveFailed {
                        error: RecipeSaveError::DuplicateTitle
                    }),
                },
            },
            draft_row: "shakshuka".into(),
        }
    );
}

#[test]
fn an_edit_pane_keeps_the_saved_title_and_the_problem() {
    let pane = kreatyna::recipes::RecipePaneView::Edit {
        editor: kreatyna::recipes::RecipeEditorView {
            title: "Spicy shakshuka".into(),
            ingredients: vec![],
            canvas: kreatyna::recipes::StepCanvasView {
                nodes: kreatyna::recipes::StepNodesView::Unselected { steps: vec![] },
                connections: vec![],
                refusal: None,
                duration_chips: vec![],
            },
            status: kreatyna::recipes::RecipeEditorStatus::Editing {
                problem: Some(kreatyna::recipes::RecipeEditorProblem::SaveFailed(
                    kreatyna::recipes::capabilities::RecipeSaveError::NotFound,
                )),
            },
        },
        saved_title: "Shakshuka".into(),
    };
    assert_eq!(
        RecipePaneView::from(pane),
        RecipePaneView::Edit {
            editor: RecipeEditorView {
                title: "Spicy shakshuka".into(),
                ingredients: vec![],
                canvas: StepCanvasView {
                    nodes: StepNodesView::Unselected { steps: vec![] },
                    connections: vec![],
                    refusal: None,
                    duration_chips: vec![],
                },
                status: RecipeEditorStatus::Editing {
                    problem: Some(RecipeEditorProblem::SaveFailed {
                        error: RecipeSaveError::NotFound
                    }),
                },
            },
            saved_title: "Shakshuka".into(),
        }
    );
}

#[test]
fn split_app_view_conversion_keeps_the_section_and_the_recipe_rows() -> anyhow::Result<()> {
    let (mut app, _effects) = core::AppWidget::test_fixture_with(core::AppLayout::Split);
    drop(app.init());
    let core::AppView::Split(mut view) = app.view() else {
        anyhow::bail!("expected the split layout")
    };
    let id = kreatyna::recipes::RecipeId::generate();
    view.recipes.list.search = "sha".into();
    view.recipes.list.status = kreatyna::recipes::RecipeListStatus::Ready {
        rows: vec![kreatyna::recipes::RecipeListRow {
            id,
            title: "Shakshuka".into(),
            step_count: 3,
        }],
    };

    let RunningAppView::Split { app } = RunningAppView::try_from(core::AppView::Split(view))?
    else {
        anyhow::bail!("expected the split layout")
    };
    assert_eq!(app.section, AppSection::Recipes);
    assert_eq!(app.recipes.list.search, "sha");
    assert_eq!(
        app.recipes.list.status,
        RecipeListStatus::Ready {
            rows: vec![RecipeListRow {
                id,
                title: "Shakshuka".into(),
                step_count: 3,
            }]
        }
    );
    assert_eq!(app.recipes.pane.value, RecipePaneView::Empty);
    assert_eq!(app.recipes.selected, None);
    assert_eq!(app.recipes.create, CreateAvailability::Available);
    Ok(())
}

#[test]
fn list_events_carry_the_ingredient_id() {
    let id = ingredient::IngredientId::generate();
    assert_eq!(
        ingredient::IngredientListEvent::from(IngredientListEvent::Select { id }),
        ingredient::IngredientListEvent::Select(id)
    );
    assert_eq!(
        ingredient::IngredientListEvent::from(IngredientListEvent::Edit { id }),
        ingredient::IngredientListEvent::Edit(id)
    );
}

#[test]
fn an_ingredient_id_crosses_the_ffi_as_its_uuid_text() -> anyhow::Result<()> {
    let id = ingredient::IngredientId::generate();
    assert_eq!(support::lift_ingredient_id(id.to_string())?, id);
    Ok(())
}

#[test_case(ingredient::IngredientListStatus::Loading, IngredientListStatus::Loading; "loading")]
#[test_case(
    ingredient::IngredientListStatus::Failed(ingredient::capabilities::IngredientLoadError::Transport),
    IngredientListStatus::Failed { error: IngredientLoadError::Transport };
    "failed"
)]
fn list_statuses_keep_their_kind(
    source: ingredient::IngredientListStatus,
    expected: IngredientListStatus,
) -> anyhow::Result<()> {
    assert_eq!(IngredientListStatus::try_from(source)?, expected);
    Ok(())
}

#[test]
fn a_ready_list_keeps_its_rows_the_total_and_the_refresh_error() -> anyhow::Result<()> {
    let id = ingredient::IngredientId::generate();
    let source = ingredient::IngredientListStatus::Ready {
        rows: vec![ingredient::IngredientListRow {
            id,
            name: "Garlic".into(),
        }],
        total_count: 3,
        refresh: ingredient::IngredientListRefresh::Failed(
            ingredient::capabilities::IngredientLoadError::Storage,
        ),
    };
    assert_eq!(
        IngredientListStatus::try_from(source)?,
        IngredientListStatus::Ready {
            rows: vec![IngredientListRow {
                id,
                name: "Garlic".into(),
            }],
            total_count: 3,
            refresh: IngredientListRefresh::Failed {
                error: IngredientLoadError::Storage,
            },
        }
    );
    Ok(())
}

#[test]
fn ingredient_values_keep_exact_text_unknowns_zero_and_unit_order() -> anyhow::Result<()> {
    let source = sample_ingredient()?;
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
    Ok(())
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
fn draft_conversion_preserves_raw_invalid_text_and_validation_indices() -> anyhow::Result<()> {
    let id = ingredient::IngredientId::generate();
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
        issues: vec![ingredient::ValidationIssue {
            field: ingredient::ValidationField::UnitGrams { index: 0 },
            reason: ingredient::ValidationReason::InvalidNumber,
        }],
        save_error: Some(ingredient::capabilities::IngredientSaveError::DuplicateName),
        status: ingredient::IngredientFormStatus::ConfirmingDiscard(
            core::confirmation::ConfirmationView::AwaitingDecision,
        ),
    };

    let converted = IngredientFormView::try_from(source)?;
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
    Ok(())
}

#[test_case(|_| (Navigation::List, Screen::List), "list"; "list")]
#[test_case(
    |garlic| (Navigation::Detail { id: garlic.id }, Screen::Detail(detail_of(garlic))),
    "detail";
    "detail"
)]
#[test_case(|garlic| (Navigation::Create, Screen::Create(form_of(garlic))), "create"; "create")]
#[test_case(
    |garlic| (
        Navigation::Edit { id: garlic.id, origin: ingredient::IngredientEditOrigin::List },
        Screen::EditFromList(form_of(garlic)),
    ),
    "edit-list";
    "edit from the list"
)]
#[test_case(
    |garlic| (
        Navigation::Edit { id: garlic.id, origin: ingredient::IngredientEditOrigin::Detail },
        Screen::EditFromDetail { detail: detail_of(garlic), form: form_of(garlic) },
    ),
    "edit-detail";
    "edit from the detail"
)]
fn every_catalog_screen_and_both_edit_origins_are_representable(
    navigation_and_screen: fn(&ingredient::Ingredient) -> (Navigation, Screen),
    expected: &str,
) -> anyhow::Result<()> {
    let (navigation, screen) = navigation_and_screen(&sample_ingredient()?);
    let converted = IngredientCatalogView::try_from(ingredient::IngredientCatalogView {
        navigation,
        list: ingredient::IngredientListView {
            search: "gar".into(),
            status: ingredient::IngredientListStatus::Loading,
        },
        scope: CommandScopeId::from(91),
        screen,
    })?;
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
    Ok(())
}

// Validation

#[tokio::test]
async fn converted_stale_event_is_rejected_by_existing_app_routing() -> anyhow::Result<()> {
    let (mut app, _effects) = core::AppWidget::test_fixture();
    let old = stack(RunningAppView::try_from(app.view())?)?;

    let create = AppEvent::Stack {
        event: IngredientCatalogEvent::List {
            scope: old.scope,
            event: IngredientListEvent::CreateRequested,
        },
    };
    let outcome = app
        .update(Message::Event(create.try_into()?))
        .into_stream()
        .next()
        .await
        .context("the create request produced no outcome")?;
    drop(app.update(Message::Outcome(outcome)));
    let current = app.view();
    assert!(matches!(
        current,
        core::AppView::Stack(ingredient::IngredientCatalogView {
            screen: ingredient::IngredientCatalogScreenView::Create(_),
            ..
        })
    ));

    let stale = AppEvent::Stack {
        event: IngredientCatalogEvent::List {
            scope: old.scope,
            event: IngredientListEvent::Search {
                query: "gar".into(),
            },
        },
    };
    let mut command = app.update(Message::Event(stale.try_into()?)).into_stream();
    assert!(command.next().await.is_none());
    assert_eq!(app.view(), current);
    Ok(())
}

#[test]
fn invalid_recipe_id_text_fails_the_call_with_a_conversion_error() {
    let error = support::lift_recipe_id("not-a-uuid".into())
        .err()
        .and_then(|error| error.downcast::<crate::AppFfiError>().ok());
    assert!(matches!(
        error,
        Some(crate::AppFfiError::Conversion {
            error: AppTypeConversionError::InvalidRecipeId { value }
        }) if value == "not-a-uuid"
    ));
}

#[test]
fn invalid_step_id_text_fails_the_call_with_a_conversion_error() {
    let error = support::lift_step_id("not-a-uuid".into())
        .err()
        .and_then(|error| error.downcast::<crate::AppFfiError>().ok());
    assert!(matches!(
        error,
        Some(crate::AppFfiError::Conversion {
            error: AppTypeConversionError::InvalidStepId { value }
        }) if value == "not-a-uuid"
    ));
}

#[test]
fn invalid_recipe_ingredient_id_text_fails_the_call_with_a_conversion_error() {
    let error = support::lift_recipe_ingredient_id("not-a-uuid".into())
        .err()
        .and_then(|error| error.downcast::<crate::AppFfiError>().ok());
    assert!(matches!(
        error,
        Some(crate::AppFfiError::Conversion {
            error: AppTypeConversionError::InvalidRecipeIngredientId { value }
        }) if value == "not-a-uuid"
    ));
}

#[test]
fn invalid_ingredient_id_text_fails_the_call_with_a_conversion_error() {
    let error = support::lift_ingredient_id("not-a-uuid".into())
        .err()
        .and_then(|error| error.downcast::<crate::AppFfiError>().ok());
    assert!(matches!(
        error,
        Some(crate::AppFfiError::Conversion {
            error: AppTypeConversionError::InvalidIngredientId { value }
        }) if value == "not-a-uuid"
    ));
}

#[test]
fn index_conversion_is_checked_but_out_of_bounds_values_remain_events() -> anyhow::Result<()> {
    let converted =
        ingredient::IngredientFormEvent::try_from(IngredientFormEvent::RemoveUnit { index: 100 })?;
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
    Ok(())
}

// Red cases

#[test_case(kreatyna::recipes::capabilities::RecipeLoadError::Storage, RecipeLoadError::Storage; "storage")]
#[test_case(kreatyna::recipes::capabilities::RecipeLoadError::Transport, RecipeLoadError::Transport; "transport")]
fn recipe_load_errors_keep_their_kind(
    error: kreatyna::recipes::capabilities::RecipeLoadError,
    expected: RecipeLoadError,
) {
    assert_eq!(
        RecipeListStatus::from(kreatyna::recipes::RecipeListStatus::Failed(error)),
        RecipeListStatus::Failed { error: expected }
    );
}

#[test]
fn a_recipe_search_without_matches_stays_no_matches() {
    assert_eq!(
        RecipeListStatus::from(kreatyna::recipes::RecipeListStatus::NoMatches),
        RecipeListStatus::NoMatches
    );
}

#[test_case(kreatyna::recipes::capabilities::RecipeSaveError::DuplicateTitle, RecipeSaveError::DuplicateTitle; "duplicate title")]
#[test_case(kreatyna::recipes::capabilities::RecipeSaveError::NotFound, RecipeSaveError::NotFound; "not found")]
#[test_case(kreatyna::recipes::capabilities::RecipeSaveError::Storage, RecipeSaveError::Storage; "storage")]
#[test_case(kreatyna::recipes::capabilities::RecipeSaveError::Transport, RecipeSaveError::Transport; "transport")]
fn recipe_save_errors_keep_their_kind(
    error: kreatyna::recipes::capabilities::RecipeSaveError,
    expected: RecipeSaveError,
) {
    assert_eq!(RecipeSaveError::from(error), expected);
}

#[test_case(kreatyna::recipes::capabilities::RecipeDeleteError::Storage, RecipeDeleteError::Storage; "storage")]
#[test_case(kreatyna::recipes::capabilities::RecipeDeleteError::Transport, RecipeDeleteError::Transport; "transport")]
fn recipe_delete_errors_keep_their_kind(
    error: kreatyna::recipes::capabilities::RecipeDeleteError,
    expected: RecipeDeleteError,
) {
    assert_eq!(RecipeDeleteError::from(error), expected);
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

// Test support

/// A fixed step id, so a test can name it twice.
fn step_id(n: u128) -> kreatyna::recipes::StepId {
    kreatyna::recipes::StepId::from(uuid::Uuid::from_u128(n))
}

/// A fixed ingredient id, so a test can name it twice.
fn ingredient_id(n: u128) -> kreatyna::recipes::RecipeIngredientId {
    kreatyna::recipes::RecipeIngredientId::from(uuid::Uuid::from_u128(n))
}

fn stack(view: RunningAppView) -> anyhow::Result<IngredientCatalogView> {
    let RunningAppView::Stack { catalog } = view else {
        anyhow::bail!("expected the stack layout, got {view:?}");
    };
    Ok(catalog)
}

/// Garlic with exact decimals: 12.3400, unknown protein, the largest
/// carbohydrates, 0.00 fat, and two units in order.
fn sample_ingredient() -> anyhow::Result<ingredient::Ingredient> {
    Ok(ingredient::Ingredient {
        id: ingredient::IngredientId::generate(),
        name: ingredient::IngredientName::try_new("Garlic")?,
        notes: Some("Fresh".into()),
        nutrition: ingredient::Nutrition {
            basis: ingredient::NutritionBasis::Per100Grams,
            calories: Some(Decimal::new(123_400, 4).try_into()?),
            protein: None,
            carbohydrates: Some(Decimal::MAX.try_into()?),
            fat: Some(Decimal::new(0, 2).try_into()?),
        },
        units: vec![
            ingredient::ReferenceUnit {
                name: ingredient::UnitName::try_new("clove")?,
                grams: Some(Decimal::new(2_500, 3).try_into()?),
                millilitres: None,
            },
            ingredient::ReferenceUnit {
                name: ingredient::UnitName::try_new("cup")?,
                grams: None,
                millilitres: Some(Decimal::new(25_000, 2).try_into()?),
            },
        ],
    })
}

type Navigation = ingredient::IngredientNavigation;
type Screen = ingredient::IngredientCatalogScreenView;

fn detail_of(ingredient: &ingredient::Ingredient) -> ingredient::IngredientDetailView {
    ingredient::IngredientDetailView {
        ingredient: ingredient.clone(),
        deletion: ingredient::IngredientDetailDeletion::Idle,
    }
}

fn form_of(ingredient: &ingredient::Ingredient) -> ingredient::IngredientFormView {
    ingredient::IngredientFormView {
        draft: ingredient::IngredientDraft::from(ingredient),
        issues: vec![],
        save_error: None,
        status: ingredient::IngredientFormStatus::Editing,
    }
}
