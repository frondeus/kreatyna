//! Pure screen-history rules. The coordinator authorizes actions before applying them.
use super::IngredientId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IngredientEditOrigin {
    List,
    Detail,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IngredientNavigation {
    #[default]
    List,
    Detail {
        id: IngredientId,
    },
    Create,
    Edit {
        id: IngredientId,
        origin: IngredientEditOrigin,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IngredientNavigationAction {
    OpenCreate,
    OpenDetail { id: IngredientId },
    OpenEdit { id: IngredientId },
    CloseDetail,
    FormDiscarded,
    FormSaved { id: IngredientId },
    IngredientDeleted { id: IngredientId },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IngredientNavigationError {
    InvalidTransition,
    IngredientMismatch,
}

impl IngredientNavigation {
    /// Compute a destination without changing this value. Form and write actions
    /// must come from accepted child outcomes; this model does not authorize them.
    pub fn transition(
        &self,
        action: IngredientNavigationAction,
    ) -> Result<Self, IngredientNavigationError> {
        use IngredientNavigationAction as Action;
        match (*self, action) {
            (Self::List, Action::OpenCreate) => Ok(Self::Create),
            (Self::List | Self::Detail { .. }, Action::OpenDetail { id }) => {
                Ok(Self::Detail { id })
            }
            (Self::List, Action::OpenEdit { id }) => Ok(Self::Edit {
                id,
                origin: IngredientEditOrigin::List,
            }),
            (Self::Detail { id: expected }, Action::OpenEdit { id }) => {
                check_id(expected, id)?;
                Ok(Self::Edit {
                    id,
                    origin: IngredientEditOrigin::Detail,
                })
            }
            (Self::Detail { .. }, Action::CloseDetail)
            | (Self::Create, Action::FormDiscarded | Action::FormSaved { .. }) => Ok(Self::List),
            (Self::Edit { id, origin }, Action::FormDiscarded) => {
                Ok(Self::edit_destination(id, origin))
            }
            (
                Self::Edit {
                    id: expected,
                    origin,
                },
                Action::FormSaved { id },
            ) => {
                check_id(expected, id)?;
                Ok(Self::edit_destination(id, origin))
            }
            (Self::Detail { id: expected }, Action::IngredientDeleted { id }) => {
                check_id(expected, id)?;
                Ok(Self::List)
            }
            _ => Err(IngredientNavigationError::InvalidTransition),
        }
    }

    /// Query where Back leads. A form must still handle RequestClose and emit
    /// Discarded before the coordinator applies that navigation.
    pub fn back_destination(&self) -> Option<Self> {
        match *self {
            Self::List => None,
            Self::Detail { .. } | Self::Create => Some(Self::List),
            Self::Edit { id, origin } => Some(Self::edit_destination(id, origin)),
        }
    }

    fn edit_destination(id: IngredientId, origin: IngredientEditOrigin) -> Self {
        match origin {
            IngredientEditOrigin::List => Self::List,
            IngredientEditOrigin::Detail => Self::Detail { id },
        }
    }
}

fn check_id(expected: IngredientId, actual: IngredientId) -> Result<(), IngredientNavigationError> {
    if expected == actual {
        Ok(())
    } else {
        Err(IngredientNavigationError::IngredientMismatch)
    }
}
