use krucyfiks::{Command, CommandExt, Latest, LatestCommandSlot, Message, Widget};

use crate::confirmation::{Confirmation, ConfirmationEvent, ConfirmationOutcome, ConfirmationView};

use super::{
    Ingredient, IngredientDraft, IngredientDraftField, IngredientWrite, NutritionBasis,
    ValidationIssue,
    capabilities::{IngredientCapabilities, IngredientEffectStream, IngredientSaveError},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngredientFormEvent {
    SetField {
        field: IngredientDraftField,
        value: String,
    },
    SetBasis(NutritionBasis),
    AddUnit,
    RemoveUnit {
        index: usize,
    },
    MoveUnit {
        from: usize,
        to: usize,
    },
    Save,
    RequestClose,
    Confirmation(ConfirmationEvent),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngredientFormOutcome {
    ConfirmationDecided(Latest<ConfirmationOutcome>),
    SaveFinished(Latest<Result<Ingredient, IngredientSaveError>>),
    Saved { ingredient: Ingredient },
    Discarded,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngredientFormStatus {
    Editing,
    ConfirmingDiscard(ConfirmationView),
    Saving,
    Finished,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IngredientFormView {
    pub draft: IngredientDraft,
    pub dirty: bool,
    pub issues: Vec<ValidationIssue>,
    pub save_error: Option<IngredientSaveError>,
    pub status: IngredientFormStatus,
}

enum FormState {
    Editing,
    ConfirmingDiscard(Confirmation),
    Saving,
    Finished,
}

pub struct IngredientForm {
    caps: IngredientCapabilities,
    initial: IngredientDraft,
    draft: IngredientDraft,
    validation_attempted: bool,
    issues: Vec<ValidationIssue>,
    save_error: Option<IngredientSaveError>,
    state: FormState,
    confirmation: LatestCommandSlot,
    save: LatestCommandSlot,
}

impl IngredientForm {
    pub fn new(caps: IngredientCapabilities, draft: IngredientDraft) -> Self {
        Self {
            caps,
            initial: draft.clone(),
            draft,
            validation_attempted: false,
            issues: Vec::new(),
            save_error: None,
            state: FormState::Editing,
            confirmation: LatestCommandSlot::new(),
            save: LatestCommandSlot::new(),
        }
    }

    pub fn test_fixture(draft: IngredientDraft) -> (Self, IngredientEffectStream) {
        let (caps, effects) = IngredientCapabilities::testing();
        (Self::new(caps, draft), effects)
    }

    fn render(&self) -> Command<IngredientFormOutcome> {
        let render = self.caps.render.clone();
        Command::without_outcome(async move { render.render().await })
    }

    fn finish_discard(&mut self) -> Command<IngredientFormOutcome> {
        self.state = FormState::Finished;
        let render = self.caps.render.clone();
        Command::single(async move {
            render.render().await;
            IngredientFormOutcome::Discarded
        })
    }

    fn update_issues(&mut self) {
        if self.validation_attempted {
            self.issues = IngredientWrite::try_from(&self.draft)
                .err()
                .unwrap_or_default();
        }
    }

    fn edit(&mut self, event: IngredientFormEvent) -> bool {
        match event {
            IngredientFormEvent::SetField { field, value } => self.draft.set_field(field, value),
            IngredientFormEvent::SetBasis(basis) => self.draft.set_basis(basis),
            IngredientFormEvent::AddUnit => {
                self.draft.add_unit();
                true
            }
            IngredientFormEvent::RemoveUnit { index } => self.draft.remove_unit(index),
            IngredientFormEvent::MoveUnit { from, to } => self.draft.move_unit(from, to),
            _ => false,
        }
    }
}

impl Widget for IngredientForm {
    type View = IngredientFormView;
    type Event = IngredientFormEvent;
    type Outcome = IngredientFormOutcome;

    fn update(&mut self, message: Message<Self>) -> Command<Self::Outcome> {
        match message {
            Message::Event(IngredientFormEvent::Save)
                if matches!(self.state, FormState::Editing) =>
            {
                self.validation_attempted = true;
                let write = match IngredientWrite::try_from(&self.draft) {
                    Ok(write) => write,
                    Err(issues) => {
                        self.issues = issues;
                        self.save_error = None;
                        return self.render();
                    }
                };
                self.issues.clear();
                self.save_error = None;
                self.state = FormState::Saving;
                let render = self.caps.render.clone();
                let store = self.caps.store.clone();
                Command::single(async move {
                    render.render().await;
                    store.save(write).await
                })
                .latest(&mut self.save)
                .map(IngredientFormOutcome::SaveFinished)
            }
            Message::Event(IngredientFormEvent::RequestClose)
                if matches!(self.state, FormState::Editing) =>
            {
                if self.draft == self.initial {
                    self.finish_discard()
                } else {
                    self.confirmation.stop();
                    self.state = FormState::ConfirmingDiscard(Confirmation::new());
                    self.render()
                }
            }
            Message::Event(IngredientFormEvent::Confirmation(event)) => {
                let FormState::ConfirmingDiscard(confirmation) = &mut self.state else {
                    return Command::empty();
                };
                if !matches!(confirmation.view(), ConfirmationView::AwaitingDecision) {
                    return Command::empty();
                }
                confirmation
                    .update(Message::Event(event))
                    .latest(&mut self.confirmation)
                    .map(IngredientFormOutcome::ConfirmationDecided)
            }
            Message::Event(event) if matches!(self.state, FormState::Editing) => {
                if !self.edit(event) {
                    return Command::empty();
                }
                self.save_error = None;
                self.update_issues();
                self.render()
            }
            Message::Outcome(IngredientFormOutcome::ConfirmationDecided(decided)) => {
                let FormState::ConfirmingDiscard(confirmation) = &self.state else {
                    return Command::empty();
                };
                let Some(decision) = self.confirmation.accept(decided) else {
                    return Command::empty();
                };
                if confirmation.view() != (ConfirmationView::Resolved { decision }) {
                    return Command::empty();
                }
                self.confirmation.stop();
                match decision {
                    ConfirmationOutcome::Confirmed => self.finish_discard(),
                    ConfirmationOutcome::Cancelled => {
                        self.state = FormState::Editing;
                        self.render()
                    }
                }
            }
            Message::Outcome(IngredientFormOutcome::SaveFinished(finished)) => {
                let Some(result) = self.save.accept(finished) else {
                    return Command::empty();
                };
                self.save.stop();
                match result {
                    Ok(ingredient) => {
                        self.state = FormState::Finished;
                        let render = self.caps.render.clone();
                        Command::single(async move {
                            render.render().await;
                            IngredientFormOutcome::Saved { ingredient }
                        })
                    }
                    Err(error) => {
                        self.state = FormState::Editing;
                        self.save_error = Some(error);
                        self.render()
                    }
                }
            }
            Message::Outcome(
                IngredientFormOutcome::Saved { .. } | IngredientFormOutcome::Discarded,
            )
            | Message::Event(_) => Command::empty(),
        }
    }

    fn view(&self) -> Self::View {
        let status = match &self.state {
            FormState::Editing => IngredientFormStatus::Editing,
            FormState::ConfirmingDiscard(confirmation) => {
                IngredientFormStatus::ConfirmingDiscard(confirmation.view())
            }
            FormState::Saving => IngredientFormStatus::Saving,
            FormState::Finished => IngredientFormStatus::Finished,
        };
        IngredientFormView {
            draft: self.draft.clone(),
            dirty: self.draft != self.initial,
            issues: self.issues.clone(),
            save_error: self.save_error.clone(),
            status,
        }
    }
}
