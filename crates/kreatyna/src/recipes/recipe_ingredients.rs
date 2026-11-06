//! The recipe ingredients in the recipe editor (REC-07): ＋ Line adds one, you
//! type its name, drag it to another place, or remove it. The lines have no
//! effects, so they are a plain struct, not a widget: the editor renders
//! after each change.
use super::{
    RecipeIngredient, RecipeIngredientId, RecipeIngredientName, RecipeIngredientNameError,
    RecipeIngredientQuantity,
};
use crate::{draft_change::DraftChange, problem_display::ProblemDisplay};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecipeIngredientsEvent {
    /// ＋ Line: an empty line at the bottom, with a new id.
    Add,
    /// The raw text of a name field. Rust trims it only on Save. Ignored for
    /// an id that names no line.
    SetName {
        id: RecipeIngredientId,
        name: String,
    },
    /// The − button. Ignored for an id that names no line.
    Remove { id: RecipeIngredientId },
    /// A drop of the line `id` onto the line `onto`: `id` takes `onto`'s
    /// place, and the lines between move one place (REC-07/Q1). So the
    /// dragged line ends up before `onto` when it moves up, and after `onto`
    /// when it moves down. With the lines penne, oil, basil:
    ///
    /// | Drag              | Result            | The dragged line ends up |
    /// |-------------------|-------------------|--------------------------|
    /// | basil onto penne  | basil, penne, oil | before penne             |
    /// | penne onto basil  | oil, basil, penne | after basil              |
    ///
    /// Ignored when either id names no line, or when both are the same line.
    Move {
        id: RecipeIngredientId,
        onto: RecipeIngredientId,
    },
}

/// One line as the editor shows it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecipeIngredientView {
    pub id: RecipeIngredientId,
    /// The name as typed.
    pub name: String,
    /// `None` until the first Save. After it, the name's problem, if any.
    pub problem: Option<RecipeIngredientNameError>,
}

/// Save found a line with an empty name. Each such line shows its problem
/// (`RecipeIngredientView::problem`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct UnnamedRecipeIngredient;

/// A line as the editor keeps it: the name as typed.
#[derive(Debug, Clone, PartialEq, Eq)]
struct DraftRecipeIngredient {
    id: RecipeIngredientId,
    name: String,
}

/// The editor's lines, in list order.
pub(crate) struct DraftRecipeIngredients(Vec<DraftRecipeIngredient>);

impl DraftRecipeIngredients {
    /// The lines that the editor opens with: none for a new recipe, the
    /// stored ones for a saved one.
    pub(crate) fn new(lines: &[RecipeIngredient]) -> Self {
        Self(
            lines
                .iter()
                .map(|line| DraftRecipeIngredient {
                    id: line.id,
                    name: line.name.as_ref().to_owned(),
                })
                .collect(),
        )
    }

    pub(crate) fn apply(&mut self, event: RecipeIngredientsEvent) -> DraftChange {
        match event {
            RecipeIngredientsEvent::Add => {
                self.0.push(DraftRecipeIngredient {
                    id: RecipeIngredientId::generate(),
                    name: String::new(),
                });
                DraftChange::Changed
            }
            RecipeIngredientsEvent::SetName { id, name } => {
                match self.0.iter_mut().find(|line| line.id == id) {
                    Some(line) if line.name != name => {
                        line.name = name;
                        DraftChange::Changed
                    }
                    Some(_) | None => DraftChange::Unchanged,
                }
            }
            RecipeIngredientsEvent::Remove { id } => match self.index(id) {
                Some(index) => {
                    self.0.remove(index);
                    DraftChange::Changed
                }
                None => DraftChange::Unchanged,
            },
            RecipeIngredientsEvent::Move { id, onto } => match (self.index(id), self.index(onto)) {
                (Some(from), Some(to)) if from != to => {
                    let line = self.0.remove(from);
                    self.0.insert(to, line);
                    DraftChange::Changed
                }
                _ => DraftChange::Unchanged,
            },
        }
    }

    pub(crate) fn view(&self, problems: ProblemDisplay) -> Vec<RecipeIngredientView> {
        self.0
            .iter()
            .map(|line| RecipeIngredientView {
                id: line.id,
                name: line.name.clone(),
                problem: match problems {
                    ProblemDisplay::Hidden => None,
                    ProblemDisplay::Shown => {
                        RecipeIngredientName::try_new(line.name.as_str()).err()
                    }
                },
            })
            .collect()
    }

    /// Every line as Save writes it, with its name trimmed, or `UnnamedRecipeIngredient`
    /// when a name is empty.
    pub(crate) fn writes(&self) -> Result<Vec<RecipeIngredient>, UnnamedRecipeIngredient> {
        self.0
            .iter()
            .map(|line| {
                RecipeIngredientName::try_new(line.name.as_str())
                    .map(|name| RecipeIngredient {
                        id: line.id,
                        name,
                        // The editor sets amounts and notes in REC-08 change 3.
                        quantity: RecipeIngredientQuantity::Unmeasured,
                        note: None,
                    })
                    .map_err(|_| UnnamedRecipeIngredient)
            })
            .collect()
    }

    /// Whether the lines differ from `saved`: in order, ids or names as
    /// typed.
    pub(crate) fn differs_from(&self, saved: &[RecipeIngredient]) -> bool {
        self.0.len() != saved.len()
            || self
                .0
                .iter()
                .zip(saved)
                .any(|(line, saved)| line.id != saved.id || line.name != saved.name.as_ref())
    }

    fn index(&self, id: RecipeIngredientId) -> Option<usize> {
        self.0.iter().position(|line| line.id == id)
    }
}
