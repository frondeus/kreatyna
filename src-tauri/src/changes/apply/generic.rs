use crate::net::models::DocumentEventChange as DbDocumentEventChange;
use crate::ui::models::DocumentEventChange as UiDocumentEventChange;

use super::ApplyChange;
use crate::changes::key::WithKey;

#[macro_export]
macro_rules! apply_scalar {
    ($scalar:ty) => {
        impl ApplyChange for $scalar {
            fn apply_change(
                &mut self,
                _path: &[&str],
                change: &DbDocumentEventChange,
            ) -> Option<UiDocumentEventChange> {
                $crate::changes::apply::generic::scalar_replace(self, change)
            }
        }
    };
}

pub fn scalar_replace<T: serde::de::DeserializeOwned>(
    scalar: &mut T,
    change: &DbDocumentEventChange,
) -> Option<UiDocumentEventChange> {
    match change {
        DbDocumentEventChange::Replace(value) => {
            *scalar = serde_json::from_value(value.clone()).unwrap();
            Some(UiDocumentEventChange::Replace(value.clone()))
        }
        _ => unreachable!(),
    }
}
apply_scalar!(bool);
apply_scalar!(usize);
apply_scalar!(u32);
apply_scalar!(String);

impl<T: WithKey + ApplyChange + serde::de::DeserializeOwned + std::fmt::Debug> ApplyChange
    for Vec<T>
{
    fn apply_change(
        &mut self,
        path: &[&str],
        change: &DbDocumentEventChange,
    ) -> Option<UiDocumentEventChange> {
        match change {
            DbDocumentEventChange::Replace(_value) => {
                let [segment, path @ ..] = path else {
                    unreachable!()
                };
                let index = self
                    .iter()
                    .position(|t| t.key().to_string().as_str() == *segment)?;

                self[index].apply_change(path, change)
            }
            DbDocumentEventChange::ReplaceWithUndefined => unreachable!(),
            DbDocumentEventChange::Remove { key } => {
                let index = self.iter().position(|t| &t.key().to_string() == key)?;
                self.remove(index);
                Some(UiDocumentEventChange::Remove { index })
            }
            DbDocumentEventChange::Move { key, before } => {
                let from = self.iter().position(|t| &t.key().to_string() == key)?;
                let item = self.remove(from);
                let to = before
                    .as_ref()
                    .and_then(|key| self.iter().position(|t| &t.key().to_string() == key));
                match to {
                    Some(pos) => {
                        self.insert(pos, item);
                    }
                    None => {
                        self.push(item);
                    }
                }
                Some(UiDocumentEventChange::Move {
                    from,
                    to: to.unwrap_or(self.len() - 1),
                })
            }
            DbDocumentEventChange::Insert {
                before,
                value,
                key: _,
            } => {
                let t: T = serde_json::from_value(value.clone()).unwrap();
                let position = match before {
                    None => {
                        self.push(t);
                        None
                    }
                    Some(key) => {
                        let pos = self.iter().position(|t| &t.key().to_string() == key)?;
                        self.insert(pos, t);
                        Some(pos)
                    }
                };

                Some(UiDocumentEventChange::Insert {
                    position,
                    value: value.clone(),
                })
            }
        }
    }
}

impl<T: ApplyChange> ApplyChange for Option<T> {
    fn apply_change(
        &mut self,
        path: &[&str],
        change: &DbDocumentEventChange,
    ) -> Option<UiDocumentEventChange> {
        if path.is_empty() && matches!(change, DbDocumentEventChange::ReplaceWithUndefined) {
            *self = None;
            return Some(UiDocumentEventChange::ReplaceWithUndefined);
        }
        match self {
            Some(value) => value.apply_change(path, change),
            None => None,
        }
    }
}
