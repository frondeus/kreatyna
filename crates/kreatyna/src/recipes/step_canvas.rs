//! The steps on the editor's canvas, the arrows between them (REC-18), and
//! the selected step or arrow (REC-14). The
//! canvas has no effects, so it is a plain struct, not a widget: the editor
//! renders after each change.
use super::{
    CanvasPoint, ConnectRefusal, DurationChip, DurationEntry, DurationError, DurationInput,
    DurationParts, Step, StepDurationS, StepEdge, StepEdges, StepId, StepInstructions, StepKind,
    StepTitle, StepTitleError, StepWrite, duration::duration_chips,
};
use crate::{draft_change::DraftChange, problem_display::ProblemDisplay};

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
    /// A drop of a ● drag on a node (REC-18/Q3): "`to` comes after
    /// `from`". Adds the arrow, or keeps why not for the line under the
    /// canvas. Selects nothing new. Ignored for an id that names no step.
    Connect { from: StepId, to: StepId },
    /// A click on an arrow. Ignored for an arrow that does not exist.
    SelectConnection(StepEdge),
    /// Edit ▸ Delete Connection (⌘⌫) on the selected arrow, or right-click
    /// ▸ Delete Connection (REC-18/Q1). Ignored for an arrow that does not
    /// exist.
    DeleteConnection(StepEdge),
}

/// The canvas as Swift draws it (REC-18): the nodes, the arrows under them,
/// and the line under the canvas.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StepCanvasView {
    pub nodes: StepNodesView,
    /// One arrow each, from the ● of `from` to the ○ of `to`. In one fixed
    /// order (by `from`, then `to`), not the order connected.
    pub connections: Vec<StepEdge>,
    /// The last refused connection, until the next change (REC-18/Q2).
    pub refusal: Option<ConnectRefusal>,
    /// The chips of the inspector's Duration field, shortest first. The same
    /// for every step.
    pub duration_chips: Vec<DurationChip>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepNodesView {
    /// The inspector shows "Select a step". The steps are in the order added.
    Unselected { steps: Vec<StepNodeView> },
    /// The inspector shows `selected`. `others` are the other steps, in the
    /// order added.
    Selected {
        selected: StepNodeView,
        others: Vec<StepNodeView>,
    },
    /// The inspector shows "Dice onion → Fry onion". The steps are in the
    /// order added.
    ConnectionSelected {
        connection: StepConnectionView,
        steps: Vec<StepNodeView>,
    },
}

/// The selected arrow, as the inspector shows it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StepConnectionView {
    pub edge: StepEdge,
    pub from: StepLabel,
    pub to: StepLabel,
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
    /// The inspector's After: the steps with an arrow into this one, in the
    /// order added (REC-18).
    pub after: Vec<StepAfterView>,
}

/// One step of the inspector's After.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StepAfterView {
    pub id: StepId,
    pub label: StepLabel,
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
        /// For the minutes field, the chips, and More….
        entry: DurationEntry,
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
                entry: duration.entry(),
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
    /// The node, with `after` the steps that come before this one.
    fn node(&self, problems: ProblemDisplay, after: Vec<StepAfterView>) -> StepNodeView {
        let problems = match problems {
            ProblemDisplay::Hidden => StepProblems::default(),
            ProblemDisplay::Shown => StepProblems {
                title: StepTitle::try_new(self.title.as_str()).err(),
                duration: self.duration.checked().err(),
            },
        };
        StepNodeView {
            id: self.id,
            title: self.title.clone(),
            label: self.label(),
            position: self.position,
            duration: self.duration.view(),
            kind: self.kind,
            instructions: self.instructions.clone(),
            problems,
            after,
        }
    }

    /// What the node shows. The StepTitle rule decides it.
    fn label(&self) -> StepLabel {
        match StepTitle::try_new(self.title.as_str()) {
            Ok(title) => StepLabel::Titled {
                title: title.into_inner(),
            },
            Err(StepTitleError::NotEmptyViolated) => StepLabel::Untitled,
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
    fn place(&mut self, to: CanvasPoint) -> DraftChange {
        if self.position == to {
            return DraftChange::Unchanged;
        }
        self.position = to;
        DraftChange::Changed
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

/// What the canvas has selected: one step, one arrow, or nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Selection {
    None,
    Step(StepId),
    Connection(StepEdge),
}

/// The steps in the order added, the arrows, and the selection. Only these
/// methods change them, so `edges` only joins steps in `steps`, and
/// `selection` always names a step in `steps` or an arrow in `edges`.
pub(crate) struct StepCanvas {
    steps: Vec<DraftStep>,
    /// The arrows between `steps` (REC-18).
    edges: StepEdges,
    selection: Selection,
    /// The last refused `Connect`, until the next change (REC-18/Q2).
    refusal: Option<ConnectRefusal>,
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
            selection: Selection::None,
            refusal: None,
        }
    }

    /// Applies `event`, and says whether the steps, the arrows, the
    /// selection, or the refusal changed. Any change other than a refused
    /// `Connect` clears the refusal.
    pub(crate) fn apply(&mut self, event: StepCanvasEvent) -> DraftChange {
        if let StepCanvasEvent::Connect { from, to } = event {
            return self.connect(StepEdge { from, to });
        }
        let change = self.change(event);
        self.settle(change)
    }

    /// Clears the refusal after a change.
    fn settle(&mut self, change: DraftChange) -> DraftChange {
        if change == DraftChange::Changed {
            self.refusal = None;
        }
        change
    }

    /// Adds `edge`, or keeps why not. A refusal changes nothing but the
    /// refusal itself.
    fn connect(&mut self, edge: StepEdge) -> DraftChange {
        if self.step(edge.from).is_none() || self.step(edge.to).is_none() {
            return DraftChange::Unchanged;
        }
        match self.edges.connect(edge) {
            Ok(()) => self.settle(DraftChange::Changed),
            Err(refusal) if self.refusal == Some(refusal) => DraftChange::Unchanged,
            Err(refusal) => {
                self.refusal = Some(refusal);
                DraftChange::Changed
            }
        }
    }

    /// Every event but `Connect`.
    fn change(&mut self, event: StepCanvasEvent) -> DraftChange {
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
                self.selection = Selection::Step(id);
                DraftChange::Changed
            }
            StepCanvasEvent::Select(id) => self.select_step(id),
            StepCanvasEvent::ClearSelection => self.set_selection(Selection::None),
            StepCanvasEvent::SetTitle { id, title } => self.edit(id, |step| &mut step.title, title),
            StepCanvasEvent::Move { id, to } => {
                let Some(step) = self.step_mut(id) else {
                    return DraftChange::Unchanged;
                };
                let placed = step.place(to);
                // Both run: the position and the selection can each change.
                placed.or(self.select_step(id))
            }
            StepCanvasEvent::Delete(id) => {
                let Some(index) = self.index_of(id) else {
                    return DraftChange::Unchanged;
                };
                self.steps.remove(index);
                // `edges` only joins steps in `steps`, and `selection` only
                // names what is left.
                self.edges.remove_step(id);
                let selected_goes = match self.selection {
                    Selection::None => false,
                    Selection::Step(step) => step == id,
                    Selection::Connection(edge) => edge.from == id || edge.to == id,
                };
                if selected_goes {
                    self.selection = Selection::None;
                }
                DraftChange::Changed
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
            StepCanvasEvent::SelectConnection(edge) => {
                if !self.edges.contains(edge) {
                    return DraftChange::Unchanged;
                }
                self.set_selection(Selection::Connection(edge))
            }
            StepCanvasEvent::DeleteConnection(edge) => {
                if self.selection == Selection::Connection(edge) {
                    self.selection = Selection::None;
                }
                self.edges.disconnect(edge)
            }
            // `apply` handles it before `change`.
            StepCanvasEvent::Connect { .. } => DraftChange::Unchanged,
        }
    }

    /// Sets the selection, and says whether it changed.
    fn set_selection(&mut self, selection: Selection) -> DraftChange {
        if self.selection == selection {
            return DraftChange::Unchanged;
        }
        self.selection = selection;
        DraftChange::Changed
    }

    /// Sets one field of the step `id` to `value`, and says whether it
    /// changed. An id that names no step changes nothing.
    fn edit<T: PartialEq>(
        &mut self,
        id: StepId,
        field: impl FnOnce(&mut DraftStep) -> &mut T,
        value: T,
    ) -> DraftChange {
        let Some(step) = self.step_mut(id) else {
            return DraftChange::Unchanged;
        };
        DraftChange::set(field(step), value)
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

    /// Selects the step `id` for Save, which shows the first step that
    /// breaks a rule. Clears the refusal when the selection changes.
    pub(crate) fn select(&mut self, id: StepId) -> DraftChange {
        let change = self.select_step(id);
        self.settle(change)
    }

    /// Selects the step `id`, and says whether the selection changed. An id
    /// that names no step changes nothing.
    fn select_step(&mut self, id: StepId) -> DraftChange {
        if self.step(id).is_none() {
            return DraftChange::Unchanged;
        }
        self.set_selection(Selection::Step(id))
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

    /// The canvas as Swift draws it.
    pub(crate) fn view(&self, problems: ProblemDisplay) -> StepCanvasView {
        let node = |step: &DraftStep| step.node(problems, self.after(step.id));
        let all = || self.steps.iter().map(node).collect();
        let nodes = match self.selection {
            Selection::None => StepNodesView::Unselected { steps: all() },
            Selection::Step(id) => match self.step(id) {
                // `selection` always names a step in `steps`.
                None => StepNodesView::Unselected { steps: all() },
                Some(selected) => StepNodesView::Selected {
                    selected: node(selected),
                    others: self
                        .steps
                        .iter()
                        .filter(|step| step.id != id)
                        .map(node)
                        .collect(),
                },
            },
            Selection::Connection(edge) => match (self.step(edge.from), self.step(edge.to)) {
                (Some(from), Some(to)) => StepNodesView::ConnectionSelected {
                    connection: StepConnectionView {
                        edge,
                        from: from.label(),
                        to: to.label(),
                    },
                    steps: all(),
                },
                // `edges` only joins steps in `steps`.
                _ => StepNodesView::Unselected { steps: all() },
            },
        };
        StepCanvasView {
            nodes,
            connections: self.edges.iter().copied().collect(),
            refusal: self.refusal,
            duration_chips: duration_chips(),
        }
    }

    /// The steps with an arrow into `step`, in the order added.
    fn after(&self, step: StepId) -> Vec<StepAfterView> {
        self.steps
            .iter()
            .filter(|before| {
                self.edges.contains(StepEdge {
                    from: before.id,
                    to: step,
                })
            })
            .map(|before| StepAfterView {
                id: before.id,
                label: before.label(),
            })
            .collect()
    }
}
