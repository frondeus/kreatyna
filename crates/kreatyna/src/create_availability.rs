/// Whether the New button (**＋** for ingredients, New Recipe for recipes)
/// opens a create form now. It is `Unavailable` while a form or an editor is
/// open, or while a delete runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreateAvailability {
    Available,
    Unavailable,
}
