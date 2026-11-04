//! The step graph (REC-18): arrows between steps, and the rules that keep
//! them a graph with no loop. Pure functions; the editor and the store use
//! them.
use std::collections::{BTreeSet, HashSet};

use uuid::Uuid;

use super::{StepId, step_canvas::CanvasChange};

/// "`to` comes after `from`": an arrow from the ● of `from` to the ○ of
/// `to`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StepEdge {
    pub from: StepId,
    pub to: StepId,
}

/// Why Rust refuses a connection (REC-18/Q2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum ConnectRefusal {
    #[error("A step cannot come after itself.")]
    SameStep,
    #[error("These steps are already connected.")]
    Duplicate,
    #[error("That would make a loop.")]
    Loop,
}

/// A recipe's edges: never a step to itself, a duplicate, or a loop. Only
/// `connect` and `from_saved` add an edge, so a value always passes. The
/// edges are a set: two values with the same edges are equal, and `iter`
/// gives them in one fixed order.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StepEdges(BTreeSet<StepEdge>);

impl StepEdges {
    /// No edges. `const`, so an empty set can be a `static`.
    pub const fn new() -> Self {
        Self(BTreeSet::new())
    }

    /// Adds `edge`, or says why not and changes nothing.
    pub fn connect(&mut self, edge: StepEdge) -> Result<(), ConnectRefusal> {
        if edge.from == edge.to {
            return Err(ConnectRefusal::SameStep);
        }
        if self.0.contains(&edge) {
            return Err(ConnectRefusal::Duplicate);
        }
        // The new edge closes a loop when its start is already after its end.
        if self.reaches(edge.to, edge.from) {
            return Err(ConnectRefusal::Loop);
        }
        self.0.insert(edge);
        Ok(())
    }

    /// The edges that the store read. Fails for the first edge that
    /// `connect` refuses, so a broken file cannot load a loop.
    pub fn from_saved(edges: impl IntoIterator<Item = StepEdge>) -> Result<Self, ConnectRefusal> {
        let mut saved = Self::default();
        for edge in edges {
            saved.connect(edge)?;
        }
        Ok(saved)
    }

    /// Removes `edge`, and says whether it was there.
    pub(crate) fn disconnect(&mut self, edge: StepEdge) -> CanvasChange {
        if self.0.remove(&edge) {
            CanvasChange::Changed
        } else {
            CanvasChange::Unchanged
        }
    }

    /// Removes every edge into or out of `step`.
    pub(crate) fn remove_step(&mut self, step: StepId) {
        self.0.retain(|edge| edge.from != step && edge.to != step);
    }

    pub fn contains(&self, edge: StepEdge) -> bool {
        self.0.contains(&edge)
    }

    pub fn iter(&self) -> impl Iterator<Item = &StepEdge> {
        self.0.iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Whether `goal` can be reached from `start` along the edges. A
    /// depth-first search that visits each step at most once.
    fn reaches(&self, start: StepId, goal: StepId) -> bool {
        let mut visited = HashSet::new();
        let mut stack = vec![start];
        while let Some(step) = stack.pop() {
            if step == goal {
                return true;
            }
            if visited.insert(step) {
                stack.extend(self.edges_from(step).map(|edge| edge.to));
            }
        }
        false
    }

    /// The edges that start at `step`. The set is sorted by `from` first, so
    /// they are one range of it.
    fn edges_from(&self, step: StepId) -> impl Iterator<Item = &StepEdge> {
        let first = StepEdge {
            from: step,
            to: StepId::from(Uuid::nil()),
        };
        self.0
            .range(first..)
            .take_while(move |edge| edge.from == step)
    }
}
