/// Whether an edit changed an editor's draft: its steps, arrows, selection or
/// ingredients. The editor renders only after `Changed`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[must_use]
pub(crate) enum DraftChange {
    Changed,
    Unchanged,
}

impl DraftChange {
    /// Sets `field` to `value`: `Changed` when they differed, `Unchanged`
    /// when `field` already held `value`.
    pub(crate) fn set<T: PartialEq>(field: &mut T, value: T) -> DraftChange {
        if *field == value {
            return DraftChange::Unchanged;
        }
        *field = value;
        DraftChange::Changed
    }

    /// `Changed` when either is `Changed`.
    pub(crate) fn or(self, other: DraftChange) -> DraftChange {
        match (self, other) {
            (DraftChange::Unchanged, DraftChange::Unchanged) => DraftChange::Unchanged,
            _ => DraftChange::Changed,
        }
    }
}
