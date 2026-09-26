use futures::StreamExt;
use kreatyna::{
    capabilities::ResolveRender,
    confirmation::{ConfirmationEvent, ConfirmationView},
    ingredients::{
        capabilities::{
            IngredientDeleteError, IngredientEffectStream, IngredientSaveError,
            ResolveIngredientStore,
        },
        *,
    },
};
use krucyfiks::{Command, Message, Scoped, Widget, effects::CommandTestExt};
use test_case::test_case;

#[tokio::test]
async fn list_navigation_preserves_search_and_rejects_the_old_activation() -> anyhow::Result<()> {
    let garlic = ingredient("Garlic");
    let mut fixture = CatalogFixture::new();
    fixture.load(vec![garlic.clone()]).await?;
    let list_scope = fixture.scope();
    fixture
        .render_event(IngredientCatalogEvent::List(Scoped::new(
            list_scope,
            IngredientListEvent::Search("gar".into()),
        )))
        .await?;
    fixture.open_detail(garlic.id).await?;
    assert!(matches!(
        fixture.catalog.view().screen,
        IngredientCatalogScreenView::Detail(_)
    ));
    assert_empty(
        fixture
            .catalog
            .update(Message::Event(IngredientCatalogEvent::List(Scoped::new(
                list_scope,
                IngredientListEvent::CreateRequested,
            )))),
    )
    .await;
    fixture.back_and_render().await?;
    let view = fixture.catalog.view();
    assert_eq!(view.navigation, IngredientNavigation::List);
    assert_eq!(view.list.search, "gar");
    assert_ne!(view.scope, list_scope);
    Ok(())
}

#[tokio::test]
async fn successful_creation_updates_a_loaded_cache_without_reloading() -> anyhow::Result<()> {
    let mut fixture = CatalogFixture::new();
    fixture.load(vec![ingredient("Basil")]).await?;
    fixture.open_create().await?;
    fixture
        .render_form_event(field(IngredientDraftField::Name, "Garlic"))
        .await?;
    let saved = ingredient("Garlic");
    let terminal = fixture.save(saved.clone()).await?;
    fixture.apply_and_render(terminal).await?;
    let view = fixture.catalog.view();
    assert_eq!(view.navigation, IngredientNavigation::List);
    assert_eq!(row_names(&view.list), ["Basil", "Garlic"]);
    Ok(())
}

#[test_case(false; "direct edit returns to list")]
#[test_case(true; "detail edit returns to updated detail")]
#[tokio::test]
async fn successful_edit_updates_cache_and_returns_to_its_origin(
    from_detail: bool,
) -> anyhow::Result<()> {
    let original = ingredient("Garlic");
    let mut fixture = CatalogFixture::new();
    fixture.load(vec![original.clone()]).await?;
    if from_detail {
        fixture.open_detail(original.id).await?;
        fixture.open_detail_edit().await?;
    } else {
        fixture.open_list_edit(original.id).await?;
    }
    fixture
        .render_form_event(field(IngredientDraftField::Name, "Roasted garlic"))
        .await?;
    let mut saved = original.clone();
    saved.name = "Roasted garlic".into();
    let terminal = fixture.save(saved.clone()).await?;
    fixture.apply_and_render(terminal).await?;
    let view = fixture.catalog.view();
    assert_eq!(row_names(&view.list), ["Roasted garlic"]);
    if from_detail {
        assert_eq!(
            view.navigation,
            IngredientNavigation::Detail { id: original.id }
        );
        let IngredientCatalogScreenView::Detail(detail) = view.screen else {
            panic!("expected detail")
        };
        assert_eq!(detail.ingredient, saved);
    } else {
        assert_eq!(view.navigation, IngredientNavigation::List);
    }
    Ok(())
}

#[tokio::test]
async fn dirty_back_uses_form_confirmation_and_only_discard_navigates() -> anyhow::Result<()> {
    let mut fixture = CatalogFixture::new();
    fixture.load(vec![]).await?;
    fixture.open_create().await?;
    fixture
        .render_form_event(field(IngredientDraftField::Name, "Garlic"))
        .await?;
    fixture.back_and_render().await?;
    let IngredientCatalogScreenView::Create(form) = fixture.catalog.view().screen else {
        panic!("expected create form")
    };
    assert_eq!(
        form.status,
        IngredientFormStatus::ConfirmingDiscard(ConfirmationView::AwaitingDecision)
    );
    let cancelled = fixture
        .form_outcome(IngredientFormEvent::Confirmation(ConfirmationEvent::Cancel))
        .await;
    fixture.apply_and_render(cancelled).await?;
    assert!(matches!(
        fixture.catalog.view().screen,
        IngredientCatalogScreenView::Create(_)
    ));
    fixture.back_and_render().await?;
    let confirmed = fixture
        .form_outcome(IngredientFormEvent::Confirmation(
            ConfirmationEvent::Confirm,
        ))
        .await;
    let discarded = fixture.apply_render_then_outcome(confirmed).await?;
    fixture.apply_and_render(discarded).await?;
    assert_eq!(
        fixture.catalog.view().navigation,
        IngredientNavigation::List
    );
    Ok(())
}

#[tokio::test]
async fn back_dismisses_delete_confirmation_but_is_ignored_while_deleting() -> anyhow::Result<()> {
    let garlic = ingredient("Garlic");
    let mut fixture = CatalogFixture::new();
    fixture.load(vec![garlic.clone()]).await?;
    fixture.open_detail(garlic.id).await?;
    fixture
        .render_detail_event(IngredientDetailEvent::RequestDelete)
        .await?;
    let decision = fixture.back_outcome().await;
    fixture.apply_and_render(decision).await?;
    assert!(matches!(
        fixture.catalog.view().screen,
        IngredientCatalogScreenView::Detail(IngredientDetailView {
            deletion: IngredientDetailDeletion::Idle,
            ..
        })
    ));

    fixture
        .render_detail_event(IngredientDetailEvent::RequestDelete)
        .await?;
    let confirmed = fixture
        .detail_outcome(IngredientDetailEvent::Confirmation(
            ConfirmationEvent::Confirm,
        ))
        .await;
    let delete = fixture.catalog.update(Message::Outcome(confirmed));
    assert!(matches!(
        fixture.catalog.view().screen,
        IngredientCatalogScreenView::Detail(IngredientDetailView {
            deletion: IngredientDetailDeletion::Deleting,
            ..
        })
    ));
    assert_empty(
        fixture
            .catalog
            .update(Message::Event(IngredientCatalogEvent::Back {
                scope: fixture.scope(),
            })),
    )
    .await;
    let finished = fixture.delete(delete, garlic.id).await?;
    let deleted = fixture.apply_render_then_outcome(finished).await?;
    fixture.apply_and_render(deleted).await?;
    assert!(row_names(&fixture.catalog.view().list).is_empty());
    Ok(())
}

#[test_case(IngredientDeleteError::Storage; "storage")]
#[test_case(IngredientDeleteError::Transport; "transport")]
#[tokio::test]
async fn failed_delete_keeps_the_detail_and_exposes_the_error(
    error: IngredientDeleteError,
) -> anyhow::Result<()> {
    let garlic = ingredient("Garlic");
    let mut fixture = CatalogFixture::new();
    fixture.load(vec![garlic.clone()]).await?;
    fixture.open_detail(garlic.id).await?;
    fixture
        .render_detail_event(IngredientDetailEvent::RequestDelete)
        .await?;
    let confirmed = fixture
        .detail_outcome(IngredientDetailEvent::Confirmation(
            ConfirmationEvent::Confirm,
        ))
        .await;
    let delete = fixture.catalog.update(Message::Outcome(confirmed));
    let finished = fixture
        .delete_result(delete, garlic.id, Err(error.clone()))
        .await?;
    fixture.apply_and_render(finished).await?;
    let IngredientCatalogScreenView::Detail(detail) = fixture.catalog.view().screen else {
        panic!("expected detail")
    };
    assert_eq!(detail.ingredient, garlic);
    assert_eq!(
        detail.deletion,
        IngredientDetailDeletion::Confirming {
            confirmation: ConfirmationView::AwaitingDecision,
            error: Some(error),
        }
    );
    Ok(())
}

#[tokio::test]
async fn early_creation_cancels_initial_load_and_reloads_the_complete_catalog() -> anyhow::Result<()>
{
    let mut fixture = CatalogFixture::new();
    let initial = fixture.catalog.init();
    fixture.open_create().await?;
    assert!(initial.into_stream().next().await.is_none());
    fixture
        .render_form_event(field(IngredientDraftField::Name, "Garlic"))
        .await?;
    let saved = ingredient("Garlic");
    let terminal = fixture.save(saved.clone()).await?;
    let reload = fixture.catalog.update(Message::Outcome(terminal));
    fixture
        .complete_load_command(reload, vec![saved.clone(), ingredient("Basil")])
        .await?;
    assert_eq!(row_names(&fixture.catalog.view().list), ["Basil", "Garlic"]);
    Ok(())
}

#[tokio::test]
async fn interrupted_refresh_is_cancelled_and_restarted_on_return() -> anyhow::Result<()> {
    let garlic = ingredient("Garlic");
    let mut fixture = CatalogFixture::new();
    fixture.load(vec![garlic.clone()]).await?;
    let refresh = fixture
        .catalog
        .update(Message::Event(IngredientCatalogEvent::List(Scoped::new(
            fixture.scope(),
            IngredientListEvent::Refresh,
        ))));
    fixture.open_detail(garlic.id).await?;
    assert!(refresh.into_stream().next().await.is_none());
    let back = fixture
        .catalog
        .update(Message::Event(IngredientCatalogEvent::Back {
            scope: fixture.scope(),
        }));
    fixture
        .complete_load_command(back, vec![garlic, ingredient("Basil")])
        .await?;
    assert_eq!(row_names(&fixture.catalog.view().list), ["Basil", "Garlic"]);
    Ok(())
}

#[test_case(IngredientSaveError::DuplicateName; "duplicate name")]
#[test_case(IngredientSaveError::NotFound; "missing update")]
#[test_case(IngredientSaveError::Storage; "storage")]
#[test_case(IngredientSaveError::Transport; "transport")]
#[tokio::test]
async fn failed_save_keeps_the_form_and_exact_draft(
    error: IngredientSaveError,
) -> anyhow::Result<()> {
    let mut fixture = CatalogFixture::new();
    fixture.load(vec![]).await?;
    fixture.open_create().await?;
    fixture
        .render_form_event(field(IngredientDraftField::Name, "  Garlic  "))
        .await?;
    let finished = fixture.save_result(Err(error.clone())).await?;
    fixture.apply_and_render(finished).await?;
    let IngredientCatalogScreenView::Create(form) = fixture.catalog.view().screen else {
        panic!("expected create form")
    };
    assert_eq!(form.draft.name, "  Garlic  ");
    assert_eq!(form.save_error, Some(error));
    assert_eq!(form.status, IngredientFormStatus::Editing);
    Ok(())
}

#[tokio::test]
async fn stale_outcomes_and_inactive_branches_do_nothing() -> anyhow::Result<()> {
    let garlic = ingredient("Garlic");
    let mut fixture = CatalogFixture::new();
    let old_load = fixture.load(vec![garlic.clone()]).await?;
    let old_scope = fixture.scope();
    fixture.open_detail(garlic.id).await?;
    assert_empty(fixture.catalog.update(Message::Outcome(old_load))).await;
    assert_empty(
        fixture
            .catalog
            .update(Message::Event(IngredientCatalogEvent::Detail(Scoped::new(
                old_scope,
                IngredientDetailEvent::Edit,
            )))),
    )
    .await;
    assert_empty(
        fixture
            .catalog
            .update(Message::Outcome(IngredientCatalogOutcome::Detail(
                Scoped::new(
                    fixture.scope(),
                    IngredientDetailOutcome::Deleted { id: garlic.id },
                ),
            ))),
    )
    .await;
    assert_empty(
        fixture
            .catalog
            .update(Message::Event(IngredientCatalogEvent::Form(Scoped::new(
                fixture.scope(),
                IngredientFormEvent::Save,
            )))),
    )
    .await;
    assert_eq!(
        fixture.catalog.view().navigation,
        IngredientNavigation::Detail { id: garlic.id }
    );
    Ok(())
}

#[tokio::test]
async fn terminal_form_outcomes_require_a_finished_active_form() -> anyhow::Result<()> {
    let mut fixture = CatalogFixture::new();
    fixture.load(vec![]).await?;
    fixture.open_create().await?;
    let scope = fixture.scope();
    assert_empty(
        fixture
            .catalog
            .update(Message::Outcome(IngredientCatalogOutcome::Form(
                Scoped::new(
                    scope,
                    IngredientFormOutcome::Saved {
                        ingredient: ingredient("Fabricated"),
                    },
                ),
            ))),
    )
    .await;
    assert_empty(
        fixture
            .catalog
            .update(Message::Outcome(IngredientCatalogOutcome::Form(
                Scoped::new(scope, IngredientFormOutcome::Discarded),
            ))),
    )
    .await;
    assert_eq!(
        fixture.catalog.view().navigation,
        IngredientNavigation::Create
    );
    Ok(())
}

#[tokio::test]
async fn pending_save_blocks_back_and_catalog_drop_cancels_the_command() -> anyhow::Result<()> {
    let mut fixture = CatalogFixture::new();
    fixture.load(vec![]).await?;
    fixture.open_create().await?;
    fixture
        .render_form_event(field(IngredientDraftField::Name, "Garlic"))
        .await?;
    let scope = fixture.scope();
    let mut save = fixture
        .catalog
        .update(Message::Event(IngredientCatalogEvent::Form(Scoped::new(
            scope,
            IngredientFormEvent::Save,
        ))))
        .driver(&mut fixture.effects);
    save.drive()
        .await?
        .expect_effect()
        .expect_render()
        .resolve_render(|| async {})
        .await?;
    assert_empty(
        fixture
            .catalog
            .update(Message::Event(IngredientCatalogEvent::Back { scope })),
    )
    .await;
    drop(fixture.catalog);
    save.drive().await?.expect_done();
    Ok(())
}

fn ingredient(name: &str) -> Ingredient {
    Ingredient {
        id: IngredientId::new(),
        name: name.into(),
        notes: None,
        nutrition: Nutrition {
            basis: NutritionBasis::Per100Grams,
            calories: None,
            protein: None,
            carbohydrates: None,
            fat: None,
        },
        units: vec![],
    }
}

fn field(field: IngredientDraftField, value: &str) -> IngredientFormEvent {
    IngredientFormEvent::SetField {
        field,
        value: value.into(),
    }
}

fn row_names(view: &IngredientListView) -> Vec<&str> {
    let IngredientListStatus::Ready { rows, .. } = &view.status else {
        panic!("expected ready list")
    };
    rows.iter().map(|row| row.name.as_str()).collect()
}

async fn assert_empty(command: Command<IngredientCatalogOutcome>) {
    assert!(command.into_stream().next().await.is_none());
}

struct CatalogFixture {
    catalog: IngredientCatalog,
    effects: IngredientEffectStream,
}

impl CatalogFixture {
    fn new() -> Self {
        let (catalog, effects) = IngredientCatalog::test_fixture(NutritionBasis::Per100Grams);
        Self { catalog, effects }
    }

    fn scope(&self) -> krucyfiks::CommandScopeId {
        self.catalog.view().scope
    }

    fn with_scope(&self, event: IngredientCatalogEvent) -> IngredientCatalogEvent {
        match event {
            IngredientCatalogEvent::List(scoped) => {
                IngredientCatalogEvent::List(Scoped::new(self.scope(), scoped.value))
            }
            IngredientCatalogEvent::Detail(scoped) => {
                IngredientCatalogEvent::Detail(Scoped::new(self.scope(), scoped.value))
            }
            IngredientCatalogEvent::Form(scoped) => {
                IngredientCatalogEvent::Form(Scoped::new(self.scope(), scoped.value))
            }
            IngredientCatalogEvent::Back { .. } => IngredientCatalogEvent::Back {
                scope: self.scope(),
            },
        }
    }

    async fn load(&mut self, records: Vec<Ingredient>) -> anyhow::Result<IngredientCatalogOutcome> {
        let command = self.catalog.init();
        self.complete_load_command(command, records).await
    }

    async fn complete_load_command(
        &mut self,
        command: Command<IngredientCatalogOutcome>,
        records: Vec<Ingredient>,
    ) -> anyhow::Result<IngredientCatalogOutcome> {
        let mut command = command.driver(&mut self.effects);
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
            .expect_store()
            .resolve_list(|| async move { Ok(records) })
            .await?;
        let outcome = command.drive().await?.expect_outcome();
        command.drive().await?.expect_done();
        drop(command);
        self.apply_and_render(outcome.clone()).await?;
        Ok(outcome)
    }

    async fn render_event(&mut self, event: IngredientCatalogEvent) -> anyhow::Result<()> {
        let event = self.with_scope(event);
        let command = self.catalog.update(Message::Event(event));
        self.render(command).await
    }

    async fn render_form_event(&mut self, event: IngredientFormEvent) -> anyhow::Result<()> {
        self.render_event(IngredientCatalogEvent::Form(Scoped::new(
            self.scope(),
            event,
        )))
        .await
    }

    async fn render_detail_event(&mut self, event: IngredientDetailEvent) -> anyhow::Result<()> {
        self.render_event(IngredientCatalogEvent::Detail(Scoped::new(
            self.scope(),
            event,
        )))
        .await
    }

    async fn render(&mut self, command: Command<IngredientCatalogOutcome>) -> anyhow::Result<()> {
        let mut command = command.driver(&mut self.effects);
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

    async fn event_outcome(&mut self, event: IngredientCatalogEvent) -> IngredientCatalogOutcome {
        self.catalog
            .update(Message::Event(self.with_scope(event)))
            .into_stream()
            .next()
            .await
            .expect("expected child outcome")
    }

    async fn form_outcome(&mut self, event: IngredientFormEvent) -> IngredientCatalogOutcome {
        self.event_outcome(IngredientCatalogEvent::Form(Scoped::new(
            self.scope(),
            event,
        )))
        .await
    }

    async fn detail_outcome(&mut self, event: IngredientDetailEvent) -> IngredientCatalogOutcome {
        self.event_outcome(IngredientCatalogEvent::Detail(Scoped::new(
            self.scope(),
            event,
        )))
        .await
    }

    async fn apply_and_render(&mut self, outcome: IngredientCatalogOutcome) -> anyhow::Result<()> {
        let command = self.catalog.update(Message::Outcome(outcome));
        self.render(command).await
    }

    async fn apply_render_then_outcome(
        &mut self,
        outcome: IngredientCatalogOutcome,
    ) -> anyhow::Result<IngredientCatalogOutcome> {
        let mut command = self
            .catalog
            .update(Message::Outcome(outcome))
            .driver(&mut self.effects);
        command
            .drive()
            .await?
            .expect_effect()
            .expect_render()
            .resolve_render(|| async {})
            .await?;
        let outcome = command.drive().await?.expect_outcome();
        command.drive().await?.expect_done();
        Ok(outcome)
    }

    async fn open_create(&mut self) -> anyhow::Result<()> {
        let outcome = self
            .event_outcome(IngredientCatalogEvent::List(Scoped::new(
                self.scope(),
                IngredientListEvent::CreateRequested,
            )))
            .await;
        self.apply_and_render(outcome).await
    }

    async fn open_detail(&mut self, id: IngredientId) -> anyhow::Result<()> {
        let outcome = self
            .event_outcome(IngredientCatalogEvent::List(Scoped::new(
                self.scope(),
                IngredientListEvent::Select(id),
            )))
            .await;
        self.apply_and_render(outcome).await
    }

    async fn open_list_edit(&mut self, id: IngredientId) -> anyhow::Result<()> {
        let outcome = self
            .event_outcome(IngredientCatalogEvent::List(Scoped::new(
                self.scope(),
                IngredientListEvent::Edit(id),
            )))
            .await;
        self.apply_and_render(outcome).await
    }

    async fn open_detail_edit(&mut self) -> anyhow::Result<()> {
        let outcome = self
            .event_outcome(IngredientCatalogEvent::Detail(Scoped::new(
                self.scope(),
                IngredientDetailEvent::Edit,
            )))
            .await;
        self.apply_and_render(outcome).await
    }

    async fn back_and_render(&mut self) -> anyhow::Result<()> {
        let command = self
            .catalog
            .update(Message::Event(IngredientCatalogEvent::Back {
                scope: self.scope(),
            }));
        self.render(command).await
    }

    async fn back_outcome(&mut self) -> IngredientCatalogOutcome {
        self.catalog
            .update(Message::Event(IngredientCatalogEvent::Back {
                scope: self.scope(),
            }))
            .into_stream()
            .next()
            .await
            .expect("expected cancellation outcome")
    }

    async fn save(&mut self, saved: Ingredient) -> anyhow::Result<IngredientCatalogOutcome> {
        let finished = self.save_result(Ok(saved)).await?;
        self.apply_render_then_outcome(finished).await
    }

    async fn save_result(
        &mut self,
        result: Result<Ingredient, IngredientSaveError>,
    ) -> anyhow::Result<IngredientCatalogOutcome> {
        let command = self
            .catalog
            .update(Message::Event(IngredientCatalogEvent::Form(Scoped::new(
                self.scope(),
                IngredientFormEvent::Save,
            ))));
        let mut command = command.driver(&mut self.effects);
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
            .expect_store()
            .resolve_save(|_| async move { result })
            .await?;
        let finished = command.drive().await?.expect_outcome();
        command.drive().await?.expect_done();
        Ok(finished)
    }

    async fn delete(
        &mut self,
        command: Command<IngredientCatalogOutcome>,
        expected: IngredientId,
    ) -> anyhow::Result<IngredientCatalogOutcome> {
        self.delete_result(command, expected, Ok(())).await
    }

    async fn delete_result(
        &mut self,
        command: Command<IngredientCatalogOutcome>,
        expected: IngredientId,
        result: Result<(), IngredientDeleteError>,
    ) -> anyhow::Result<IngredientCatalogOutcome> {
        let mut command = command.driver(&mut self.effects);
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
            .expect_store()
            .resolve_delete(move |actual| async move {
                assert_eq!(actual, expected);
                result
            })
            .await?;
        let outcome = command.drive().await?.expect_outcome();
        command.drive().await?.expect_done();
        Ok(outcome)
    }
}
