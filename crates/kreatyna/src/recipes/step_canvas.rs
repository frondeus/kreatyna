//! The steps on the editor's canvas and the selected step (REC-14). The
//! canvas has no effects, so it is a plain struct, not a widget: the editor
//! renders after each change.
use super::{CanvasPoint, Step, StepId, StepTitle, StepTitleError, StepWrite};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepCanvasEvent {
    /// Right-click ▸ Add Step. Adds an untitled step at `at` and selects it.
    Add { at: CanvasPoint },
    /// A click on a node. Ignored for an id that names no step.
    Select(StepId),
    /// A click on empty canvas.
    ClearSelection,
    /// The title field of the inspector, as typed. Rust trims it only on
    /// Save. Ignored for an id that names no step.
    SetTitle { id: StepId, title: String },
    /// A drop after a drag. Puts the step at `to` and selects it
    /// (REC-15/Q1). Ignored for an id that names no step.
    Move { id: StepId, to: CanvasPoint },
    /// Edit ▸ Delete Step (⌘⌫) on the selected node, or right-click ▸
    /// Delete Step. Removes the step; a deleted selected step leaves nothing
    /// selected. Ignored for an id that names no step.
    Delete(StepId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepCanvasView {
    /// The inspector shows "Select a step". The steps are in the order added.
    Unselected { steps: Vec<StepNodeView> },
    /// The inspector shows `selected`. `others` are the other steps, in the
    /// order added.
    Selected {
        selected: StepNodeView,
        others: Vec<StepNodeView>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StepNodeView {
    pub id: StepId,
    /// The title as typed. The inspector's Title field edits it.
    pub title: String,
    /// What the node shows.
    pub label: StepLabel,
    pub position: CanvasPoint,
    /// Only after the first Save: the title breaks the rule.
    pub problem: Option<StepTitleError>,
}

/// What a step node shows. The StepTitle rule decides it, so a title of
/// spaces is untitled too.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepLabel {
    /// The title, trimmed.
    Titled { title: String },
    /// "Untitled step".
    Untitled,
}

/// Whether a canvas call changed the steps or the selection. The editor
/// renders only after `Changed`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[must_use]
pub(crate) enum CanvasChange {
    Changed,
    Unchanged,
}

impl CanvasChange {
    /// `Changed` when either is `Changed`.
    fn or(self, other: CanvasChange) -> CanvasChange {
        match (self, other) {
            (CanvasChange::Unchanged, CanvasChange::Unchanged) => CanvasChange::Unchanged,
            _ => CanvasChange::Changed,
        }
    }
}

/// A step as the editor keeps it: its title is the raw text of the field.
#[derive(Debug, Clone, PartialEq, Eq)]
struct DraftStep {
    id: StepId,
    title: String,
    position: CanvasPoint,
}

impl DraftStep {
    fn node(&self, show_problems: bool) -> StepNodeView {
        let checked = StepTitle::try_new(self.title.as_str());
        let label = match &checked {
            Ok(title) => StepLabel::Titled {
                title: title.clone().into_inner(),
            },
            Err(StepTitleError::NotEmptyViolated) => StepLabel::Untitled,
        };
        let problem = if show_problems { checked.err() } else { None };
        StepNodeView {
            id: self.id,
            title: self.title.clone(),
            label,
            position: self.position,
            problem,
        }
    }

    /// Puts the step at `to`, and says whether its position changed.
    fn place(&mut self, to: CanvasPoint) -> CanvasChange {
        if self.position == to {
            return CanvasChange::Unchanged;
        }
        self.position = to;
        CanvasChange::Changed
    }

    fn matches(&self, step: &Step) -> bool {
        self.id == step.id && self.title == step.title && self.position == step.position
    }
}

/// The steps in the order added, and the selection. Only these methods
/// change them, so `selected` always names a step in `steps`.
pub(crate) struct StepCanvas {
    steps: Vec<DraftStep>,
    selected: Option<StepId>,
}

impl StepCanvas {
    /// A canvas with the saved steps and no selection.
    pub(crate) fn new(steps: &[Step]) -> Self {
        let steps = steps
            .iter()
            .map(|step| DraftStep {
                id: step.id,
                title: step.title.clone(),
                position: step.position,
            })
            .collect();
        Self {
            steps,
            selected: None,
        }
    }

    /// Applies `event`, and says whether the steps or the selection changed.
    pub(crate) fn apply(&mut self, event: StepCanvasEvent) -> CanvasChange {
        match event {
            StepCanvasEvent::Add { at } => {
                let id = StepId::generate();
                self.steps.push(DraftStep {
                    id,
                    title: String::new(),
                    position: at,
                });
                self.selected = Some(id);
                CanvasChange::Changed
            }
            StepCanvasEvent::Select(id) => self.select(id),
            StepCanvasEvent::ClearSelection => match self.selected.take() {
                Some(_) => CanvasChange::Changed,
                None => CanvasChange::Unchanged,
            },
            StepCanvasEvent::SetTitle { id, title } => {
                let Some(step) = self.step_mut(id) else {
                    return CanvasChange::Unchanged;
                };
                if step.title == title {
                    return CanvasChange::Unchanged;
                }
                step.title = title;
                CanvasChange::Changed
            }
            StepCanvasEvent::Move { id, to } => {
                let Some(step) = self.step_mut(id) else {
                    return CanvasChange::Unchanged;
                };
                let placed = step.place(to);
                // Both run: the position and the selection can each change.
                placed.or(self.select(id))
            }
            StepCanvasEvent::Delete(id) => {
                let Some(index) = self.index_of(id) else {
                    return CanvasChange::Unchanged;
                };
                self.steps.remove(index);
                // `selected` must always name a step in `steps`.
                if self.selected == Some(id) {
                    self.selected = None;
                }
                CanvasChange::Changed
            }
        }
    }

    /// Where the step `id` is in `steps`. `None` for an id that names no
    /// step.
    fn index_of(&self, id: StepId) -> Option<usize> {
        self.steps.iter().position(|step| step.id == id)
    }

    /// The step `id`. `None` for an id that names no step.
    fn step(&self, id: StepId) -> Option<&DraftStep> {
        self.steps.iter().find(|step| step.id == id)
    }

    fn step_mut(&mut self, id: StepId) -> Option<&mut DraftStep> {
        self.steps.iter_mut().find(|step| step.id == id)
    }

    /// Selects the step `id`, and says whether the selection changed. An id
    /// that names no step changes nothing.
    pub(crate) fn select(&mut self, id: StepId) -> CanvasChange {
        if self.selected == Some(id) || self.step(id).is_none() {
            return CanvasChange::Unchanged;
        }
        self.selected = Some(id);
        CanvasChange::Changed
    }

    /// The steps as Save writes them, in the order added. Fails with the id
    /// of the first step whose title breaks the rule.
    pub(crate) fn writes(&self) -> Result<Vec<StepWrite>, StepId> {
        self.steps
            .iter()
            .map(|step| {
                let title = StepTitle::try_new(step.title.as_str()).map_err(|_| step.id)?;
                Ok(StepWrite {
                    id: step.id,
                    title,
                    position: step.position,
                })
            })
            .collect()
    }

    /// Whether the steps differ from `saved` in number, order, id, title, or
    /// position.
    pub(crate) fn differs_from(&self, saved: &[Step]) -> bool {
        self.steps.len() != saved.len()
            || self
                .steps
                .iter()
                .zip(saved)
                .any(|(step, saved)| !step.matches(saved))
    }

    /// The canvas as Swift draws it. `show_problems` is true after the first
    /// Save.
    pub(crate) fn view(&self, show_problems: bool) -> StepCanvasView {
        let selected = self.selected.and_then(|id| self.step(id));
        match selected {
            None => StepCanvasView::Unselected {
                steps: self
                    .steps
                    .iter()
                    .map(|step| step.node(show_problems))
                    .collect(),
            },
            Some(selected) => StepCanvasView::Selected {
                selected: selected.node(show_problems),
                others: self
                    .steps
                    .iter()
                    .filter(|step| step.id != selected.id)
                    .map(|step| step.node(show_problems))
                    .collect(),
            },
        }
    }
}
