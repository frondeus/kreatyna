/// Whether an edit changed an editor's draft: its steps, arrows, selection or
/// ingredient lines. The editor renders only after `Changed`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[must_use]
pub(crate) enum DraftChange {
    Changed,
    Unchanged,
}

impl DraftChange {
    /// `Changed` when either is `Changed`.
    pub(crate) fn or(self, other: DraftChange) -> DraftChange {
        match (self, other) {
            (DraftChange::Unchanged, DraftChange::Unchanged) => DraftChange::Unchanged,
            _ => DraftChange::Changed,
        }
    }
}
