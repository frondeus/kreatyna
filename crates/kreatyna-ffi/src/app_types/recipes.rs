use kreatyna::recipes as core;
use krucyfiks::Scoped;

use super::{ConfirmationEvent, ConfirmationView, CreateAvailability, ScopeId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum RecipeLoadError {
    Storage,
    Transport,
}

impl From<core::capabilities::RecipeLoadError> for RecipeLoadError {
    fn from(value: core::capabilities::RecipeLoadError) -> Self {
        match value {
            core::capabilities::RecipeLoadError::Storage => Self::Storage,
            core::capabilities::RecipeLoadError::Transport => Self::Transport,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum RecipeListEvent {
    /// Load again after a failure (the Retry button).
    Refresh,
    /// The search text as typed. Rust filters the rows by its key.
    Search { query: String },
}

impl From<RecipeListEvent> for core::RecipeListEvent {
    fn from(value: RecipeListEvent) -> Self {
        match value {
            RecipeListEvent::Refresh => Self::Refresh,
            RecipeListEvent::Search { query } => Self::Search(query),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct RecipeListRow {
    pub id: core::RecipeId,
    pub title: String,
    /// Swift shows "1 step" or "N steps", and nothing for 0.
    pub step_count: u32,
}

impl From<core::RecipeListRow> for RecipeListRow {
    fn from(value: core::RecipeListRow) -> Self {
        Self {
            id: value.id,
            title: value.title,
            step_count: value.step_count,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum RecipeListStatus {
    Loading,
    Failed {
        error: RecipeLoadError,
    },
    /// The recipes that match the search, sorted by title. Empty only when
    /// no recipe is saved: "No recipes yet".
    Ready {
        rows: Vec<RecipeListRow>,
    },
    /// Recipes are saved, but none matches the search: "No matches".
    NoMatches,
}

impl From<core::RecipeListStatus> for RecipeListStatus {
    fn from(value: core::RecipeListStatus) -> Self {
        match value {
            core::RecipeListStatus::Loading => Self::Loading,
            core::RecipeListStatus::Failed(error) => Self::Failed {
                error: error.into(),
            },
            core::RecipeListStatus::Ready { rows } => Self::Ready {
                rows: rows.into_iter().map(Into::into).collect(),
            },
            core::RecipeListStatus::NoMatches => Self::NoMatches,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct RecipeListView {
    /// The search text as typed.
    pub search: String,
    pub status: RecipeListStatus,
}

impl From<core::RecipeListView> for RecipeListView {
    fn from(value: core::RecipeListView) -> Self {
        Self {
            search: value.search,
            status: value.status.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum RecipeSaveError {
    DuplicateTitle,
    NotFound,
    Storage,
    Transport,
}

impl From<core::capabilities::RecipeSaveError> for RecipeSaveError {
    fn from(value: core::capabilities::RecipeSaveError) -> Self {
        match value {
            core::capabilities::RecipeSaveError::DuplicateTitle => Self::DuplicateTitle,
            core::capabilities::RecipeSaveError::NotFound => Self::NotFound,
            core::capabilities::RecipeSaveError::Storage => Self::Storage,
            core::capabilities::RecipeSaveError::Transport => Self::Transport,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum RecipeDeleteError {
    Storage,
    Transport,
}

impl From<core::capabilities::RecipeDeleteError> for RecipeDeleteError {
    fn from(value: core::capabilities::RecipeDeleteError) -> Self {
        match value {
            core::capabilities::RecipeDeleteError::Storage => Self::Storage,
            core::capabilities::RecipeDeleteError::Transport => Self::Transport,
        }
    }
}

/// The save rule a title breaks (nutype's error in Rust).
#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum RecipeTitleError {
    NotEmptyViolated,
}

impl From<core::RecipeTitleError> for RecipeTitleError {
    fn from(value: core::RecipeTitleError) -> Self {
        match value {
            core::RecipeTitleError::NotEmptyViolated => Self::NotEmptyViolated,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum RecipeEditorProblem {
    Invalid { error: RecipeTitleError },
    SaveFailed { error: RecipeSaveError },
}

impl From<core::RecipeEditorProblem> for RecipeEditorProblem {
    fn from(value: core::RecipeEditorProblem) -> Self {
        match value {
            core::RecipeEditorProblem::Invalid(error) => Self::Invalid {
                error: error.into(),
            },
            core::RecipeEditorProblem::SaveFailed(error) => Self::SaveFailed {
                error: error.into(),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum RecipeEditorStatus {
    Editing {
        problem: Option<RecipeEditorProblem>,
    },
    ConfirmingDiscard {
        confirmation: ConfirmationView,
    },
    Saving,
    Finished,
}

impl From<core::RecipeEditorStatus> for RecipeEditorStatus {
    fn from(value: core::RecipeEditorStatus) -> Self {
        match value {
            core::RecipeEditorStatus::Editing { problem } => Self::Editing {
                problem: problem.map(Into::into),
            },
            core::RecipeEditorStatus::ConfirmingDiscard(confirmation) => Self::ConfirmingDiscard {
                confirmation: confirmation.into(),
            },
            core::RecipeEditorStatus::Saving => Self::Saving,
            core::RecipeEditorStatus::Finished => Self::Finished,
        }
    }
}

/// A position on the step canvas in whole points, from the top left corner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Record)]
pub struct CanvasPoint {
    pub x: u32,
    pub y: u32,
}

impl From<core::CanvasPoint> for CanvasPoint {
    fn from(value: core::CanvasPoint) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<CanvasPoint> for core::CanvasPoint {
    fn from(value: CanvasPoint) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

/// The save rule a step title breaks (nutype's error in Rust).
#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum StepTitleError {
    NotEmptyViolated,
}

impl From<core::StepTitleError> for StepTitleError {
    fn from(value: core::StepTitleError) -> Self {
        match value {
            core::StepTitleError::NotEmptyViolated => Self::NotEmptyViolated,
        }
    }
}

/// What a step node shows. Rust's StepTitle rule decides it.
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum StepLabel {
    /// The title, trimmed.
    Titled { title: String },
    /// "Untitled step".
    Untitled,
}

impl From<core::StepLabel> for StepLabel {
    fn from(value: core::StepLabel) -> Self {
        match value {
            core::StepLabel::Titled { title } => Self::Titled { title },
            core::StepLabel::Untitled => Self::Untitled,
        }
    }
}

/// Whether your hands are busy during a step (D-09, D-30).
#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum StepKind {
    /// Hands busy, such as "Dice onion".
    Active,
    /// Hands free, such as "Bake sauce". The node has a dashed border.
    Waiting,
}

impl From<core::StepKind> for StepKind {
    fn from(value: core::StepKind) -> Self {
        match value {
            core::StepKind::Active => Self::Active,
            core::StepKind::Waiting => Self::Waiting,
        }
    }
}

impl From<StepKind> for core::StepKind {
    fn from(value: StepKind) -> Self {
        match value {
            StepKind::Active => Self::Active,
            StepKind::Waiting => Self::Waiting,
        }
    }
}

/// A duration in units. Rust fills each unit in its range; the unit fields
/// send it back as typed, and a field may then hold more, such as 75 min.
#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Record)]
pub struct DurationParts {
    pub days: u32,
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
}

impl From<core::DurationParts> for DurationParts {
    fn from(value: core::DurationParts) -> Self {
        Self {
            days: value.days,
            hours: value.hours,
            minutes: value.minutes,
            seconds: value.seconds,
        }
    }
}

impl From<DurationParts> for core::DurationParts {
    fn from(value: DurationParts) -> Self {
        Self {
            days: value.days,
            hours: value.hours,
            minutes: value.minutes,
            seconds: value.seconds,
        }
    }
}

/// What the Duration field sends (REC-17/Q1). Rust adds the units up and
/// checks the range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum DurationInput {
    /// An empty minutes field.
    Cleared,
    /// A chip, or the minutes field.
    Minutes { minutes: u32 },
    /// The unit fields under More….
    Parts { parts: DurationParts },
}

impl From<DurationInput> for core::DurationInput {
    fn from(value: DurationInput) -> Self {
        match value {
            DurationInput::Cleared => Self::Cleared,
            DurationInput::Minutes { minutes } => Self::Minutes(minutes),
            DurationInput::Parts { parts } => Self::Parts(parts.into()),
        }
    }
}

/// Why a duration cannot be saved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum DurationError {
    /// More than 365 days.
    TooLong,
}

impl From<core::DurationError> for DurationError {
    fn from(value: core::DurationError) -> Self {
        match value {
            core::DurationError::TooLong => Self::TooLong,
        }
    }
}

/// How the Duration field shows a set duration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum DurationEntry {
    /// Whole minutes, less than a day. The minutes field shows them, the
    /// chip with the same minutes is on, and More… opens only on a click.
    Minutes { minutes: u32 },
    /// Whole minutes, a day or more. The minutes field shows them, and the
    /// unit fields open by themselves.
    MinutesWithDays { minutes: u32 },
    /// The duration has seconds. The minutes field shows "…", and the unit
    /// fields open by themselves.
    Seconds,
}

impl From<core::DurationEntry> for DurationEntry {
    fn from(value: core::DurationEntry) -> Self {
        match value {
            core::DurationEntry::Minutes { minutes } => Self::Minutes { minutes },
            core::DurationEntry::MinutesWithDays { minutes } => Self::MinutesWithDays { minutes },
            core::DurationEntry::Seconds => Self::Seconds,
        }
    }
}

/// One chip of the Duration field.
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct DurationChip {
    /// What a click sends, as `DurationInput.minutes`.
    pub minutes: u32,
    /// "10 min", "1 h".
    pub text: String,
}

impl From<core::DurationChip> for DurationChip {
    fn from(value: core::DurationChip) -> Self {
        Self {
            minutes: value.minutes,
            text: value.text,
        }
    }
}

/// A step's duration as the node and the Duration field show it.
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
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

impl From<core::StepDurationView> for StepDurationView {
    fn from(value: core::StepDurationView) -> Self {
        match value {
            core::StepDurationView::NotSet => Self::NotSet,
            core::StepDurationView::Set { text, parts, entry } => Self::Set {
                text,
                parts: parts.into(),
                entry: entry.into(),
            },
            core::StepDurationView::TooLong { parts } => Self::TooLong {
                parts: parts.into(),
            },
        }
    }
}

/// The rules one step breaks, only after the first Save. Each field is
/// independent of the other.
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct StepProblems {
    pub title: Option<StepTitleError>,
    pub duration: Option<DurationError>,
}

impl From<core::StepProblems> for StepProblems {
    fn from(value: core::StepProblems) -> Self {
        Self {
            title: value.title.map(Into::into),
            duration: value.duration.map(Into::into),
        }
    }
}

/// A step node on the canvas.
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct StepNodeView {
    pub id: core::StepId,
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
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct StepAfterView {
    pub id: core::StepId,
    pub label: StepLabel,
}

impl From<core::StepAfterView> for StepAfterView {
    fn from(value: core::StepAfterView) -> Self {
        Self {
            id: value.id,
            label: value.label.into(),
        }
    }
}

/// "`to` comes after `from`": an arrow from the ● of `from` to the ○ of
/// `to` (REC-18).
#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Record)]
pub struct StepEdge {
    pub from: core::StepId,
    pub to: core::StepId,
}

impl From<core::StepEdge> for StepEdge {
    fn from(value: core::StepEdge) -> Self {
        Self {
            from: value.from,
            to: value.to,
        }
    }
}

impl From<StepEdge> for core::StepEdge {
    fn from(value: StepEdge) -> Self {
        Self {
            from: value.from,
            to: value.to,
        }
    }
}

/// The selected arrow, as the inspector shows it: "Dice onion → Fry onion".
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct StepConnectionView {
    pub edge: StepEdge,
    pub from: StepLabel,
    pub to: StepLabel,
}

impl From<core::StepConnectionView> for StepConnectionView {
    fn from(value: core::StepConnectionView) -> Self {
        Self {
            edge: value.edge.into(),
            from: value.from.into(),
            to: value.to.into(),
        }
    }
}

/// Why Rust refused a connection (REC-18/Q2). Swift shows the line under
/// the canvas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum ConnectRefusal {
    /// "A step cannot come after itself."
    SameStep,
    /// "These steps are already connected."
    Duplicate,
    /// "That would make a loop."
    Loop,
}

impl From<core::ConnectRefusal> for ConnectRefusal {
    fn from(value: core::ConnectRefusal) -> Self {
        match value {
            core::ConnectRefusal::SameStep => Self::SameStep,
            core::ConnectRefusal::Duplicate => Self::Duplicate,
            core::ConnectRefusal::Loop => Self::Loop,
        }
    }
}

impl From<core::StepNodeView> for StepNodeView {
    fn from(value: core::StepNodeView) -> Self {
        Self {
            id: value.id,
            title: value.title,
            label: value.label.into(),
            position: value.position.into(),
            duration: value.duration.into(),
            kind: value.kind.into(),
            instructions: value.instructions,
            problems: value.problems.into(),
            after: value.after.into_iter().map(Into::into).collect(),
        }
    }
}

/// The canvas as Swift draws it (REC-18): the nodes, the arrows under them,
/// and the line under the canvas.
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct StepCanvasView {
    pub nodes: StepNodesView,
    /// One arrow each, from the ● of `from` to the ○ of `to`.
    pub connections: Vec<StepEdge>,
    /// The last refused connection, until the next change.
    pub refusal: Option<ConnectRefusal>,
    /// The chips of the inspector's Duration field, shortest first. The same
    /// for every step.
    pub duration_chips: Vec<DurationChip>,
}

impl From<core::StepCanvasView> for StepCanvasView {
    fn from(value: core::StepCanvasView) -> Self {
        Self {
            nodes: value.nodes.into(),
            connections: value.connections.into_iter().map(Into::into).collect(),
            refusal: value.refusal.map(Into::into),
            duration_chips: value.duration_chips.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum StepNodesView {
    /// The inspector shows "Select a step". The steps are in the order added.
    Unselected { steps: Vec<StepNodeView> },
    /// The inspector shows `selected`. `others` are the other steps, in the
    /// order added.
    Selected {
        selected: StepNodeView,
        others: Vec<StepNodeView>,
    },
    /// The inspector shows the arrow. The steps are in the order added.
    ConnectionSelected {
        connection: StepConnectionView,
        steps: Vec<StepNodeView>,
    },
}

impl From<core::StepNodesView> for StepNodesView {
    fn from(value: core::StepNodesView) -> Self {
        match value {
            core::StepNodesView::Unselected { steps } => Self::Unselected {
                steps: steps.into_iter().map(Into::into).collect(),
            },
            core::StepNodesView::Selected { selected, others } => Self::Selected {
                selected: selected.into(),
                others: others.into_iter().map(Into::into).collect(),
            },
            core::StepNodesView::ConnectionSelected { connection, steps } => {
                Self::ConnectionSelected {
                    connection: connection.into(),
                    steps: steps.into_iter().map(Into::into).collect(),
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct RecipeEditorView {
    /// The title as typed.
    pub title: String,
    pub canvas: StepCanvasView,
    pub status: RecipeEditorStatus,
}

impl From<core::RecipeEditorView> for RecipeEditorView {
    fn from(value: core::RecipeEditorView) -> Self {
        Self {
            title: value.title,
            canvas: value.canvas.into(),
            status: value.status.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum StepCanvasEvent {
    /// Right-click ▸ Add Step. Adds an untitled step at `at` and selects it.
    Add { at: CanvasPoint },
    /// A click on a node.
    Select { id: core::StepId },
    /// A click on empty canvas.
    ClearSelection,
    /// The title field of the inspector, as typed.
    SetTitle { id: core::StepId, title: String },
    /// A drop after a drag: the step goes to `to` and becomes selected.
    Move { id: core::StepId, to: CanvasPoint },
    /// Edit ▸ Delete Step (⌘⌫) or right-click ▸ Delete Step.
    Delete { id: core::StepId },
    /// The inspector's Duration field.
    SetDuration {
        id: core::StepId,
        input: DurationInput,
    },
    /// The inspector's Active/Waiting switch.
    SetKind { id: core::StepId, kind: StepKind },
    /// The inspector's Instructions field, as typed.
    SetInstructions { id: core::StepId, text: String },
    /// A drop of a ● drag on a node: "`to` comes after `from`".
    Connect {
        from: core::StepId,
        to: core::StepId,
    },
    /// A click on an arrow.
    SelectConnection { edge: StepEdge },
    /// Edit ▸ Delete Connection (⌘⌫) or right-click ▸ Delete Connection.
    DeleteConnection { edge: StepEdge },
}

impl From<StepCanvasEvent> for core::StepCanvasEvent {
    fn from(value: StepCanvasEvent) -> Self {
        match value {
            StepCanvasEvent::Add { at } => Self::Add { at: at.into() },
            StepCanvasEvent::Select { id } => Self::Select(id),
            StepCanvasEvent::ClearSelection => Self::ClearSelection,
            StepCanvasEvent::SetTitle { id, title } => Self::SetTitle { id, title },
            StepCanvasEvent::Move { id, to } => Self::Move { id, to: to.into() },
            StepCanvasEvent::Delete { id } => Self::Delete(id),
            StepCanvasEvent::SetDuration { id, input } => Self::SetDuration {
                id,
                input: input.into(),
            },
            StepCanvasEvent::SetKind { id, kind } => Self::SetKind {
                id,
                kind: kind.into(),
            },
            StepCanvasEvent::SetInstructions { id, text } => Self::SetInstructions { id, text },
            StepCanvasEvent::Connect { from, to } => Self::Connect { from, to },
            StepCanvasEvent::SelectConnection { edge } => Self::SelectConnection(edge.into()),
            StepCanvasEvent::DeleteConnection { edge } => Self::DeleteConnection(edge.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum RecipeEditorEvent {
    SetTitle {
        title: String,
    },
    /// The steps on the canvas and in the inspector.
    Canvas {
        event: StepCanvasEvent,
    },
    Save,
    /// Cancel. A changed title or step list asks "Discard changes?" first.
    RequestClose,
    Confirmation {
        event: ConfirmationEvent,
    },
}

impl From<RecipeEditorEvent> for core::RecipeEditorEvent {
    fn from(value: RecipeEditorEvent) -> Self {
        match value {
            RecipeEditorEvent::SetTitle { title } => Self::SetTitle(title),
            RecipeEditorEvent::Canvas { event } => Self::Canvas(event.into()),
            RecipeEditorEvent::Save => Self::Save,
            RecipeEditorEvent::RequestClose => Self::RequestClose,
            RecipeEditorEvent::Confirmation { event } => Self::Confirmation(event.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum RecipeDetailEvent {
    /// Load again after a failure (the Retry button).
    Retry,
    /// The Edit button: open the editor for the shown recipe.
    Edit,
    /// The Delete button: ask "Delete …?".
    RequestDelete,
    /// Delete or Cancel in the question.
    Confirmation { event: ConfirmationEvent },
}

impl From<RecipeDetailEvent> for core::RecipeDetailEvent {
    fn from(value: RecipeDetailEvent) -> Self {
        match value {
            RecipeDetailEvent::Retry => Self::Retry,
            RecipeDetailEvent::Edit => Self::Edit,
            RecipeDetailEvent::RequestDelete => Self::RequestDelete,
            RecipeDetailEvent::Confirmation { event } => Self::Confirmation(event.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum RecipeDetailStatus {
    Loading,
    Failed {
        error: RecipeLoadError,
    },
    /// No recipe has this id any more.
    Missing,
    Ready {
        title: String,
        /// In the order added. Empty shows "None yet".
        steps: Vec<RecipeDetailStep>,
        deletion: RecipeDeletion,
    },
}

/// A step in the read view: "Boil water · 10 min · waiting", with the
/// instructions under it in smaller text (REC-17/Q2).
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct RecipeDetailStep {
    pub id: core::StepId,
    pub title: String,
    /// The duration as text, such as "1 h 30 min". `nil` shows nothing.
    pub duration: Option<String>,
    pub kind: StepKind,
    pub instructions: Option<String>,
}

impl From<core::RecipeDetailStep> for RecipeDetailStep {
    fn from(value: core::RecipeDetailStep) -> Self {
        Self {
            id: value.id,
            title: value.title,
            duration: value.duration,
            kind: value.kind.into(),
            instructions: value.instructions,
        }
    }
}

/// The delete of the shown recipe. Swift shows the question while
/// `Confirming` waits, and "Deleting…" while `Deleting` or `Deleted`.
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum RecipeDeletion {
    Idle,
    Confirming {
        confirmation: ConfirmationView,
        /// The last failed delete, if any.
        error: Option<RecipeDeleteError>,
    },
    Deleting,
    Deleted,
}

impl From<core::RecipeDeletion> for RecipeDeletion {
    fn from(value: core::RecipeDeletion) -> Self {
        match value {
            core::RecipeDeletion::Idle => Self::Idle,
            core::RecipeDeletion::Confirming {
                confirmation,
                error,
            } => Self::Confirming {
                confirmation: confirmation.into(),
                error: error.map(Into::into),
            },
            core::RecipeDeletion::Deleting => Self::Deleting,
            core::RecipeDeletion::Deleted => Self::Deleted,
        }
    }
}

impl From<core::RecipeDetailStatus> for RecipeDetailStatus {
    fn from(value: core::RecipeDetailStatus) -> Self {
        match value {
            core::RecipeDetailStatus::Loading => Self::Loading,
            core::RecipeDetailStatus::Failed(error) => Self::Failed {
                error: error.into(),
            },
            core::RecipeDetailStatus::Missing => Self::Missing,
            core::RecipeDetailStatus::Ready {
                title,
                steps,
                deletion,
            } => Self::Ready {
                title,
                steps: steps.into_iter().map(Into::into).collect(),
                deletion: deletion.into(),
            },
        }
    }
}

/// The read view of one recipe.
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct RecipeDetailView {
    pub id: core::RecipeId,
    pub status: RecipeDetailStatus,
}

impl From<core::RecipeDetailView> for RecipeDetailView {
    fn from(value: core::RecipeDetailView) -> Self {
        Self {
            id: value.id,
            status: value.status.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum RecipePaneView {
    /// "Select a recipe".
    Empty,
    /// The read view of the selected recipe.
    Detail { detail: RecipeDetailView },
    Create {
        editor: RecipeEditorView,
        /// The typed title, trimmed, for the draft row at the top of the
        /// list. Empty shows "New Recipe".
        draft_row: String,
    },
    /// The edit editor. `saved_title` is the title as stored: the heading
    /// shows it until Save.
    Edit {
        editor: RecipeEditorView,
        saved_title: String,
    },
}

impl From<core::RecipePaneView> for RecipePaneView {
    fn from(value: core::RecipePaneView) -> Self {
        match value {
            core::RecipePaneView::Empty => Self::Empty,
            core::RecipePaneView::Detail(detail) => Self::Detail {
                detail: detail.into(),
            },
            core::RecipePaneView::Create { editor, draft_row } => Self::Create {
                editor: editor.into(),
                draft_row,
            },
            core::RecipePaneView::Edit {
                editor,
                saved_title,
            } => Self::Edit {
                editor: editor.into(),
                saved_title,
            },
        }
    }
}

/// The pane view and the scope that its events must be sent with.
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct ScopedRecipePaneView {
    pub scope: ScopeId,
    pub value: RecipePaneView,
}

impl From<Scoped<core::RecipePaneView>> for ScopedRecipePaneView {
    fn from(value: Scoped<core::RecipePaneView>) -> Self {
        Self {
            scope: value.scope.into(),
            value: value.value.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct RecipeSectionView {
    pub list: RecipeListView,
    /// The highlighted row: the open recipe's id. `None` while the editor is
    /// open.
    pub selected: Option<core::RecipeId>,
    pub pane: ScopedRecipePaneView,
    /// Whether New Recipe works now.
    pub create: CreateAvailability,
}

impl From<core::RecipeSectionView> for RecipeSectionView {
    fn from(value: core::RecipeSectionView) -> Self {
        Self {
            list: value.list.into(),
            selected: value.selected,
            pane: value.pane.into(),
            create: value.create.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum RecipeSectionEvent {
    List {
        event: RecipeListEvent,
    },
    /// New Recipe or ⌘N.
    NewRecipe,
    /// A row click.
    Select {
        id: core::RecipeId,
    },
    /// ⌘-click on the open recipe.
    ClearSelection {
        scope: ScopeId,
    },
    Detail {
        scope: ScopeId,
        event: RecipeDetailEvent,
    },
    Editor {
        scope: ScopeId,
        event: RecipeEditorEvent,
    },
}

impl From<RecipeSectionEvent> for core::RecipeSectionEvent {
    fn from(value: RecipeSectionEvent) -> Self {
        match value {
            RecipeSectionEvent::List { event } => Self::List(event.into()),
            RecipeSectionEvent::NewRecipe => Self::NewRecipe,
            RecipeSectionEvent::Select { id } => Self::Select(id),
            RecipeSectionEvent::ClearSelection { scope } => Self::ClearSelection {
                scope: scope.into(),
            },
            RecipeSectionEvent::Detail { scope, event } => {
                Self::Detail(Scoped::new(scope.into(), event.into()))
            }
            RecipeSectionEvent::Editor { scope, event } => {
                Self::Editor(Scoped::new(scope.into(), event.into()))
            }
        }
    }
}
