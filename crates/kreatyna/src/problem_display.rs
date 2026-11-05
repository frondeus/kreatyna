/// Whether a form or an editor shows its validation problems. They are hidden
/// until the first Save; after it, every change checks again (REC-14/Q2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ProblemDisplay {
    Hidden,
    Shown,
}
