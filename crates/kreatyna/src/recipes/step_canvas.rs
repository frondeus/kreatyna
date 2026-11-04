//! The steps on the editor's canvas and the selected step (REC-14). The
//! canvas has no effects, so it is a plain struct, not a widget: the editor
//! renders after each change.
use super::{
    CanvasPoint, DurationError, DurationInput, DurationParts, Step, StepDurationS, StepEdges,
    StepId, StepInstructions, StepKind, StepTitle, StepTitleError, StepWrite,
};

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
    /// The inspector's Duration field: a chip, the minutes field, or the
    /// unit fields under More… (REC-17/Q1). Ignored for an id that names no
    /// step.
    SetDuration { id: StepId, input: DurationInput },
    /// The inspector's Active/Waiting switch. Ignored for an id that names
    /// no step.
    SetKind { id: StepId, kind: StepKind },
    /// The inspector's Instructions field, as typed. Rust trims it only on
    /// Save. Ignored for an id that names no step.
    SetInstructions { id: StepId, text: String },
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
    /// What the node and the Duration field show.
    pub duration: StepDurationView,
    pub kind: StepKind,
    /// The instructions as typed. The inspector's Instructions field edits
    /// them.
    pub instructions: String,
    /// Only after the first Save: the rules this step breaks.
    pub problems: StepProblems,
}

/// A step's duration as the node and the Duration field show it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepDurationView {
    /// The node shows "No duration"; the fields are empty.
    NotSet,
    Set {
        /// What the node shows: "1 h 30 min".
        text: String,
        /// For the unit fields under More….
        parts: DurationParts,
        /// For the minutes field and the chips. `None` when the duration
        /// has seconds; then More… opens by itself.
        whole_minutes: Option<u32>,
    },
    /// More than 365 days: the node shows "No duration", and the unit
    /// fields show what you set.
    TooLong { parts: DurationParts },
}

/// The rules one step breaks. Each field is independent of the other.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StepProblems {
    pub title: Option<StepTitleError>,
    pub duration: Option<DurationError>,
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

/// A step's duration as the editor keeps it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DraftDuration {
    NotSet,
    Set(StepDurationS),
    /// What you set, kept so the fields still show it. Save refuses it.
    TooLong(DurationParts),
}

impl DraftDuration {
    fn from_input(input: DurationInput) -> Self {
        match input.duration() {
            Ok(None) => Self::NotSet,
            Ok(Some(duration)) => Self::Set(duration),
            Err(DurationError::TooLong) => Self::TooLong(input.parts()),
        }
    }

    fn from_saved(duration: Option<StepDurationS>) -> Self {
        duration.map_or(Self::NotSet, Self::Set)
    }

    /// The duration as Save writes it.
    fn checked(self) -> Result<Option<StepDurationS>, DurationError> {
        match self {
            Self::NotSet => Ok(None),
            Self::Set(duration) => Ok(Some(duration)),
            Self::TooLong(_) => Err(DurationError::TooLong),
        }
    }

    fn view(self) -> StepDurationView {
        match self {
            Self::NotSet => StepDurationView::NotSet,
            Self::Set(duration) => StepDurationView::Set {
                text: duration.text(),
                parts: duration.parts(),
                whole_minutes: duration.whole_minutes(),
            },
            Self::TooLong(parts) => StepDurationView::TooLong { parts },
        }
    }
}

/// A step as the editor keeps it: its title and instructions are the raw
/// text of their fields.
#[derive(Debug, Clone, PartialEq, Eq)]
struct DraftStep {
    id: StepId,
    title: String,
    position: CanvasPoint,
    duration: DraftDuration,
    kind: StepKind,
    instructions: String,
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
        let problems = if show_problems {
            StepProblems {
                title: checked.err(),
                duration: self.duration.checked().err(),
            }
        } else {
            StepProblems::default()
        };
        StepNodeView {
            id: self.id,
            title: self.title.clone(),
            label,
            position: self.position,
            duration: self.duration.view(),
            kind: self.kind,
            instructions: self.instructions.clone(),
            problems,
        }
    }

    /// The instructions as Save writes them: trimmed, and `None` when
    /// nothing is left.
    fn checked_instructions(&self) -> Option<StepInstructions> {
        // The only error is "empty": then there are no instructions.
        StepInstructions::try_new(self.instructions.as_str()).ok()
    }

    /// The step as Save writes it. Fails when a rule is broken.
    fn write(&self) -> Option<StepWrite> {
        Some(StepWrite {
            id: self.id,
            title: StepTitle::try_new(self.title.as_str()).ok()?,
            position: self.position,
            duration: self.duration.checked().ok()?,
            kind: self.kind,
            instructions: self.checked_instructions(),
        })
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
        self.id == step.id
            && self.title == step.title
            && self.position == step.position
            && self.duration == DraftDuration::from_saved(step.duration)
            && self.kind == step.kind
            && self.checked_instructions() == step.instructions
    }
}

/// The steps in the order added, and the selection. Only these methods
/// change them, so `selected` always names a step in `steps`.
pub(crate) struct StepCanvas {
    steps: Vec<DraftStep>,
    /// The arrows between `steps` (REC-18).
    edges: StepEdges,
    selected: Option<StepId>,
}

impl StepCanvas {
    /// A canvas with the saved steps and edges, and no selection.
    pub(crate) fn new(steps: &[Step], edges: &StepEdges) -> Self {
        let steps = steps
            .iter()
            .map(|step| DraftStep {
                id: step.id,
                title: step.title.clone(),
                position: step.position,
                duration: DraftDuration::from_saved(step.duration),
                kind: step.kind,
                instructions: step
                    .instructions
                    .as_ref()
                    .map_or_else(String::new, |text| text.as_ref().to_owned()),
            })
            .collect();
        Self {
            steps,
            edges: edges.clone(),
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
                    duration: DraftDuration::NotSet,
                    kind: StepKind::default(),
                    instructions: String::new(),
                });
                self.selected = Some(id);
                CanvasChange::Changed
            }
            StepCanvasEvent::Select(id) => self.select(id),
            StepCanvasEvent::ClearSelection => match self.selected.take() {
                Some(_) => CanvasChange::Changed,
                None => CanvasChange::Unchanged,
            },
            StepCanvasEvent::SetTitle { id, title } => self.edit(id, |step| &mut step.title, title),
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
            StepCanvasEvent::SetDuration { id, input } => self.edit(
                id,
                |step| &mut step.duration,
                DraftDuration::from_input(input),
            ),
            StepCanvasEvent::SetKind { id, kind } => self.edit(id, |step| &mut step.kind, kind),
            StepCanvasEvent::SetInstructions { id, text } => {
                self.edit(id, |step| &mut step.instructions, text)
            }
        }
    }

    /// Sets one field of the step `id` to `value`, and says whether it
    /// changed. An id that names no step changes nothing.
    fn edit<T: PartialEq>(
        &mut self,
        id: StepId,
        field: impl FnOnce(&mut DraftStep) -> &mut T,
        value: T,
    ) -> CanvasChange {
        let Some(step) = self.step_mut(id) else {
            return CanvasChange::Unchanged;
        };
        let field = field(step);
        if *field == value {
            return CanvasChange::Unchanged;
        }
        *field = value;
        CanvasChange::Changed
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
    /// of the first step that breaks a rule: its title or its duration.
    pub(crate) fn writes(&self) -> Result<Vec<StepWrite>, StepId> {
        self.steps
            .iter()
            .map(|step| step.write().ok_or(step.id))
            .collect()
    }

    /// The edges as Save writes them.
    pub(crate) fn edges(&self) -> &StepEdges {
        &self.edges
    }

    /// Whether the steps differ from `saved` in number, order, or any field
    /// that Save writes, or the edges differ from `saved_edges`.
    pub(crate) fn differs_from(&self, saved: &[Step], saved_edges: &StepEdges) -> bool {
        self.edges != *saved_edges
            || self.steps.len() != saved.len()
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
