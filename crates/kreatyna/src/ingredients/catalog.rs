use krucyfiks::{Command, CommandExt, CommandScope, CommandScopeId, Message, Scoped, Widget};

use crate::confirmation::ConfirmationEvent;

use super::{
    DraftMode, Ingredient, IngredientDetail, IngredientDetailDeletion, IngredientDetailEvent,
    IngredientDetailOutcome, IngredientDetailView, IngredientDraft, IngredientEditOrigin,
    IngredientForm, IngredientFormEvent, IngredientFormOutcome, IngredientFormStatus,
    IngredientFormView, IngredientList, IngredientListEvent, IngredientListOutcome,
    IngredientListView, IngredientNavigation, IngredientNavigationAction, NutritionBasis,
    capabilities::{IngredientCapabilities, IngredientEffectStream},
    list::ListRefresh,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngredientCatalogEvent {
    List(Scoped<IngredientListEvent>),
    Detail(Scoped<IngredientDetailEvent>),
    Form(Scoped<IngredientFormEvent>),
    Back { scope: CommandScopeId },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngredientCatalogOutcome {
    List(Scoped<IngredientListOutcome>),
    Detail(Scoped<IngredientDetailOutcome>),
    Form(Scoped<IngredientFormOutcome>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngredientCatalogScreenView {
    List,
    Detail(IngredientDetailView),
    Create(IngredientFormView),
    EditFromList(IngredientFormView),
    EditFromDetail {
        detail: IngredientDetailView,
        form: IngredientFormView,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IngredientCatalogView {
    pub navigation: IngredientNavigation,
    pub list: IngredientListView,
    pub scope: CommandScopeId,
    pub screen: IngredientCatalogScreenView,
}

enum ActiveScreen {
    List,
    Detail(IngredientDetail),
    Create(IngredientForm),
    EditFromList(IngredientForm),
    EditFromDetail {
        detail: IngredientDetail,
        form: Box<IngredientForm>,
    },
}

pub struct IngredientCatalog {
    caps: IngredientCapabilities,
    creation_basis: NutritionBasis,
    list: IngredientList,
    active: ActiveScreen,
    scope: CommandScope,
}

impl IngredientCatalog {
    pub fn new(caps: IngredientCapabilities) -> Self {
        Self {
            list: IngredientList::new(caps.clone()),
            caps,
            creation_basis: NutritionBasis::Per100Grams,
            active: ActiveScreen::List,
            scope: CommandScope::new(),
        }
    }

    pub fn test_fixture() -> (Self, IngredientEffectStream) {
        let (caps, effects) = IngredientCapabilities::testing();
        (Self::new(caps), effects)
    }

    fn navigation(&self) -> IngredientNavigation {
        match &self.active {
            ActiveScreen::List => IngredientNavigation::List,
            ActiveScreen::Detail(detail) => IngredientNavigation::Detail {
                id: detail.view().ingredient.id,
            },
            ActiveScreen::Create(_) => IngredientNavigation::Create,
            ActiveScreen::EditFromList(form) => IngredientNavigation::Edit {
                id: edit_id(form),
                origin: IngredientEditOrigin::List,
            },
            ActiveScreen::EditFromDetail { form, .. } => IngredientNavigation::Edit {
                id: edit_id(form),
                origin: IngredientEditOrigin::Detail,
            },
        }
    }

    fn accepts(&self, scope: CommandScopeId) -> bool {
        self.scope.accepts(scope)
    }

    fn replace_scope(&mut self) {
        self.scope = CommandScope::new();
    }

    fn render(&mut self) -> Command<IngredientCatalogOutcome> {
        let render = self.caps.render.clone();
        Command::<IngredientCatalogOutcome>::without_outcome(async move { render.render().await })
            .scoped(&mut self.scope)
            .map(|_| unreachable!("a render command has no outcome"))
    }

    fn list_command(
        &mut self,
        command: Command<IngredientListOutcome>,
    ) -> Command<IngredientCatalogOutcome> {
        command
            .scoped(&mut self.scope)
            .map(IngredientCatalogOutcome::List)
    }

    fn detail_command(
        &mut self,
        command: Command<IngredientDetailOutcome>,
    ) -> Command<IngredientCatalogOutcome> {
        command
            .scoped(&mut self.scope)
            .map(IngredientCatalogOutcome::Detail)
    }

    fn form_command(
        &mut self,
        command: Command<IngredientFormOutcome>,
    ) -> Command<IngredientCatalogOutcome> {
        command
            .scoped(&mut self.scope)
            .map(IngredientCatalogOutcome::Form)
    }

    fn open_create(&mut self) -> Command<IngredientCatalogOutcome> {
        if self
            .navigation()
            .transition(IngredientNavigationAction::OpenCreate)
            .is_err()
        {
            return Command::empty();
        }
        self.list.suspend();
        let mut form =
            IngredientForm::new(self.caps.clone(), IngredientDraft::new(self.creation_basis));
        let init = form.init();
        self.active = ActiveScreen::Create(form);
        self.replace_scope();
        Command::select_all(vec![self.form_command(init), self.render()])
    }

    fn open_detail(&mut self, ingredient: Ingredient) -> Command<IngredientCatalogOutcome> {
        if self
            .navigation()
            .transition(IngredientNavigationAction::OpenDetail { id: ingredient.id })
            .is_err()
        {
            return Command::empty();
        }
        self.list.suspend();
        let mut detail = IngredientDetail::new(self.caps.clone(), ingredient);
        let init = detail.init();
        self.active = ActiveScreen::Detail(detail);
        self.replace_scope();
        Command::select_all(vec![self.detail_command(init), self.render()])
    }

    fn open_edit_from_list(&mut self, ingredient: Ingredient) -> Command<IngredientCatalogOutcome> {
        if self
            .navigation()
            .transition(IngredientNavigationAction::OpenEdit { id: ingredient.id })
            .is_err()
        {
            return Command::empty();
        }
        self.list.suspend();
        let mut form = IngredientForm::new(self.caps.clone(), IngredientDraft::from(&ingredient));
        let init = form.init();
        self.active = ActiveScreen::EditFromList(form);
        self.replace_scope();
        Command::select_all(vec![self.form_command(init), self.render()])
    }

    fn open_edit_from_detail(
        &mut self,
        ingredient: Ingredient,
    ) -> Command<IngredientCatalogOutcome> {
        if self
            .navigation()
            .transition(IngredientNavigationAction::OpenEdit { id: ingredient.id })
            .is_err()
        {
            return Command::empty();
        }
        let ActiveScreen::Detail(detail) = std::mem::replace(&mut self.active, ActiveScreen::List)
        else {
            unreachable!("detail edit intent is only routed from detail")
        };
        let mut form = IngredientForm::new(self.caps.clone(), IngredientDraft::from(&ingredient));
        let init = form.init();
        self.active = ActiveScreen::EditFromDetail {
            detail,
            form: Box::new(form),
        };
        self.replace_scope();
        Command::select_all(vec![self.form_command(init), self.render()])
    }

    fn activate_list(&mut self, refresh: ListRefresh) -> Command<IngredientCatalogOutcome> {
        self.active = ActiveScreen::List;
        self.replace_scope();
        let load = match refresh {
            ListRefresh::Resume => self.list.resume(),
            ListRefresh::Reload => Some(self.list.reload()),
        };
        match load {
            Some(load) => self.list_command(load),
            None => self.render(),
        }
    }

    fn activate_detail(
        &mut self,
        mut detail: IngredientDetail,
    ) -> Command<IngredientCatalogOutcome> {
        let init = detail.init();
        self.active = ActiveScreen::Detail(detail);
        self.replace_scope();
        Command::select_all(vec![self.detail_command(init), self.render()])
    }

    fn back(&mut self) -> Command<IngredientCatalogOutcome> {
        match &mut self.active {
            ActiveScreen::List => Command::empty(),
            ActiveScreen::Detail(detail) => match detail.view().deletion {
                IngredientDetailDeletion::Idle => {
                    if self
                        .navigation()
                        .transition(IngredientNavigationAction::CloseDetail)
                        .is_err()
                    {
                        return Command::empty();
                    }
                    self.activate_list(ListRefresh::Resume)
                }
                IngredientDetailDeletion::Confirming { .. } => {
                    let command = detail.update(Message::Event(
                        IngredientDetailEvent::Confirmation(ConfirmationEvent::Cancel),
                    ));
                    self.detail_command(command)
                }
                IngredientDetailDeletion::Deleting | IngredientDetailDeletion::Deleted => {
                    Command::empty()
                }
            },
            ActiveScreen::Create(form) | ActiveScreen::EditFromList(form) => {
                let command = form.update(Message::Event(IngredientFormEvent::RequestClose));
                self.form_command(command)
            }
            ActiveScreen::EditFromDetail { form, .. } => {
                let command = form.update(Message::Event(IngredientFormEvent::RequestClose));
                self.form_command(command)
            }
        }
    }

    fn handle_list_outcome(
        &mut self,
        outcome: IngredientListOutcome,
    ) -> Command<IngredientCatalogOutcome> {
        if !matches!(self.active, ActiveScreen::List) {
            return Command::empty();
        }
        match outcome {
            IngredientListOutcome::CreateRequested => self.open_create(),
            IngredientListOutcome::OpenDetail { ingredient } => {
                if self.list.contains_visible(&ingredient) {
                    self.open_detail(ingredient)
                } else {
                    Command::empty()
                }
            }
            IngredientListOutcome::OpenEdit { ingredient } => {
                if self.list.contains_visible(&ingredient) {
                    self.open_edit_from_list(ingredient)
                } else {
                    Command::empty()
                }
            }
            outcome @ IngredientListOutcome::LoadFinished { .. } => {
                let command = self.list.update(Message::Outcome(outcome));
                self.list_command(command)
            }
        }
    }

    fn handle_detail_outcome(
        &mut self,
        outcome: IngredientDetailOutcome,
    ) -> Command<IngredientCatalogOutcome> {
        if !matches!(self.active, ActiveScreen::Detail(_)) {
            return Command::empty();
        }
        match outcome {
            IngredientDetailOutcome::OpenEdit { ingredient } => {
                let ActiveScreen::Detail(detail) = &self.active else {
                    unreachable!()
                };
                if detail.view().deletion == IngredientDetailDeletion::Idle
                    && detail.view().ingredient == ingredient
                {
                    self.open_edit_from_detail(ingredient)
                } else {
                    Command::empty()
                }
            }
            IngredientDetailOutcome::Deleted { id } => {
                let ActiveScreen::Detail(detail) = &self.active else {
                    unreachable!()
                };
                if detail.view().deletion != IngredientDetailDeletion::Deleted
                    || detail.view().ingredient.id != id
                {
                    return Command::empty();
                }
                if self
                    .navigation()
                    .transition(IngredientNavigationAction::IngredientDeleted { id })
                    .is_err()
                {
                    return Command::empty();
                }
                let refresh = self.list.apply_deleted(id);
                self.activate_list(refresh)
            }
            outcome => {
                let ActiveScreen::Detail(detail) = &mut self.active else {
                    unreachable!()
                };
                let command = detail.update(Message::Outcome(outcome));
                self.detail_command(command)
            }
        }
    }

    fn handle_form_outcome(
        &mut self,
        outcome: IngredientFormOutcome,
    ) -> Command<IngredientCatalogOutcome> {
        if !matches!(
            self.active,
            ActiveScreen::Create(_)
                | ActiveScreen::EditFromList(_)
                | ActiveScreen::EditFromDetail { .. }
        ) {
            return Command::empty();
        }
        match outcome {
            IngredientFormOutcome::Saved { ingredient } => {
                if active_form(&self.active).view().status != IngredientFormStatus::Finished {
                    return Command::empty();
                }
                if self
                    .navigation()
                    .transition(IngredientNavigationAction::FormSaved { id: ingredient.id })
                    .is_err()
                {
                    return Command::empty();
                }
                let refresh = self.list.apply_saved(ingredient.clone());
                match std::mem::replace(&mut self.active, ActiveScreen::List) {
                    ActiveScreen::EditFromDetail { .. } => {
                        self.activate_detail(IngredientDetail::new(self.caps.clone(), ingredient))
                    }
                    ActiveScreen::Create(_) | ActiveScreen::EditFromList(_) => {
                        self.activate_list(refresh)
                    }
                    _ => unreachable!(),
                }
            }
            IngredientFormOutcome::Discarded => {
                if active_form(&self.active).view().status != IngredientFormStatus::Finished {
                    return Command::empty();
                }
                if self
                    .navigation()
                    .transition(IngredientNavigationAction::FormDiscarded)
                    .is_err()
                {
                    return Command::empty();
                }
                match std::mem::replace(&mut self.active, ActiveScreen::List) {
                    ActiveScreen::EditFromDetail { detail, .. } => self.activate_detail(detail),
                    ActiveScreen::Create(_) | ActiveScreen::EditFromList(_) => {
                        self.activate_list(ListRefresh::Resume)
                    }
                    _ => unreachable!(),
                }
            }
            outcome => {
                let form = match &mut self.active {
                    ActiveScreen::Create(form) | ActiveScreen::EditFromList(form) => form,
                    ActiveScreen::EditFromDetail { form, .. } => form,
                    _ => unreachable!(),
                };
                let command = form.update(Message::Outcome(outcome));
                self.form_command(command)
            }
        }
    }
}

impl Widget for IngredientCatalog {
    type View = IngredientCatalogView;
    type Event = IngredientCatalogEvent;
    type Outcome = IngredientCatalogOutcome;

    fn init(&mut self) -> Command<Self::Outcome> {
        let command = self.list.init();
        self.list_command(command)
    }

    fn can_begin_shutdown(&self) -> bool {
        self.list.can_begin_shutdown()
            && match &self.active {
                ActiveScreen::List => true,
                ActiveScreen::Detail(detail) => detail.can_begin_shutdown(),
                ActiveScreen::Create(form) | ActiveScreen::EditFromList(form) => {
                    form.can_begin_shutdown()
                }
                ActiveScreen::EditFromDetail { detail, form } => {
                    detail.can_begin_shutdown() && form.can_begin_shutdown()
                }
            }
    }

    fn begin_shutdown(&mut self) -> Command<Self::Outcome> {
        let list = self.list.begin_shutdown();
        let (detail, form) = match &mut self.active {
            ActiveScreen::List => (None, None),
            ActiveScreen::Detail(detail) => (Some(detail.begin_shutdown()), None),
            ActiveScreen::Create(form) | ActiveScreen::EditFromList(form) => {
                (None, Some(form.begin_shutdown()))
            }
            ActiveScreen::EditFromDetail { detail, form } => {
                (Some(detail.begin_shutdown()), Some(form.begin_shutdown()))
            }
        };

        let mut commands = vec![self.list_command(list)];
        if let Some(detail) = detail {
            commands.push(self.detail_command(detail));
        }
        if let Some(form) = form {
            commands.push(self.form_command(form));
        }
        Command::select_all(commands)
    }

    fn update(&mut self, message: Message<Self>) -> Command<Self::Outcome> {
        match message {
            Message::Event(IngredientCatalogEvent::Back { scope }) => {
                if self.accepts(scope) {
                    self.back()
                } else {
                    Command::empty()
                }
            }
            Message::Event(IngredientCatalogEvent::List(scoped)) => {
                if !self.accepts(scoped.scope) || !matches!(self.active, ActiveScreen::List) {
                    return Command::empty();
                }
                let command = self.list.update(Message::Event(scoped.value));
                self.list_command(command)
            }
            Message::Event(IngredientCatalogEvent::Detail(scoped)) => {
                if !self.accepts(scoped.scope) {
                    return Command::empty();
                }
                let ActiveScreen::Detail(detail) = &mut self.active else {
                    return Command::empty();
                };
                let command = detail.update(Message::Event(scoped.value));
                self.detail_command(command)
            }
            Message::Event(IngredientCatalogEvent::Form(scoped)) => {
                if !self.accepts(scoped.scope) {
                    return Command::empty();
                }
                let form = match &mut self.active {
                    ActiveScreen::Create(form) | ActiveScreen::EditFromList(form) => form,
                    ActiveScreen::EditFromDetail { form, .. } => form,
                    _ => return Command::empty(),
                };
                let command = form.update(Message::Event(scoped.value));
                self.form_command(command)
            }
            Message::Outcome(IngredientCatalogOutcome::List(scoped)) => {
                if self.accepts(scoped.scope) {
                    self.handle_list_outcome(scoped.value)
                } else {
                    Command::empty()
                }
            }
            Message::Outcome(IngredientCatalogOutcome::Detail(scoped)) => {
                if self.accepts(scoped.scope) {
                    self.handle_detail_outcome(scoped.value)
                } else {
                    Command::empty()
                }
            }
            Message::Outcome(IngredientCatalogOutcome::Form(scoped)) => {
                if self.accepts(scoped.scope) {
                    self.handle_form_outcome(scoped.value)
                } else {
                    Command::empty()
                }
            }
        }
    }

    fn view(&self) -> Self::View {
        let screen = match &self.active {
            ActiveScreen::List => IngredientCatalogScreenView::List,
            ActiveScreen::Detail(detail) => IngredientCatalogScreenView::Detail(detail.view()),
            ActiveScreen::Create(form) => IngredientCatalogScreenView::Create(form.view()),
            ActiveScreen::EditFromList(form) => {
                IngredientCatalogScreenView::EditFromList(form.view())
            }
            ActiveScreen::EditFromDetail { detail, form } => {
                IngredientCatalogScreenView::EditFromDetail {
                    detail: detail.view(),
                    form: form.view(),
                }
            }
        };
        IngredientCatalogView {
            navigation: self.navigation(),
            list: self.list.view(),
            scope: self.scope.id(),
            screen,
        }
    }
}

fn edit_id(form: &IngredientForm) -> super::IngredientId {
    let DraftMode::Edit { id } = form.view().draft.mode else {
        unreachable!("edit screens always contain edit drafts")
    };
    id
}

fn active_form(active: &ActiveScreen) -> &IngredientForm {
    match active {
        ActiveScreen::Create(form) | ActiveScreen::EditFromList(form) => form,
        ActiveScreen::EditFromDetail { form, .. } => form,
        _ => unreachable!("form outcomes are only routed from form screens"),
    }
}
