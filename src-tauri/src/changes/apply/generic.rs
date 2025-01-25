use crate::net::models::DocumentEventChange as DbDocumentEventChange;
use crate::ui::models::DocumentEventChange as UiDocumentEventChange;

use super::ApplyChange;
use crate::changes::key::WithKey;

#[macro_export]
macro_rules! apply_scalar {
    ($scalar:ty) => {
        impl ApplyChange for $scalar {
            fn apply_change<'a>(
                &mut self,
                _path: impl Iterator<Item = &'a str>,
                ui_path: String,
                change: &DbDocumentEventChange,
            ) -> Option<(String, UiDocumentEventChange)> {
                $crate::changes::apply::generic::scalar_replace(self, change, ui_path)
            }
        }
    };
}

pub fn scalar_replace<T: serde::de::DeserializeOwned>(
    scalar: &mut T,
    change: &DbDocumentEventChange,
    ui_path: String,
) -> Option<(String, UiDocumentEventChange)> {
    match change {
        DbDocumentEventChange::Replace { value } => {
            *scalar = serde_json::from_value(value.clone()).unwrap();
            Some((
                ui_path,
                UiDocumentEventChange::Replace {
                    value: value.clone(),
                },
            ))
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
    fn apply_change<'a>(
        &mut self,
        mut path: impl Iterator<Item = &'a str>,
        ui_path: String,
        change: &DbDocumentEventChange,
    ) -> Option<(String, UiDocumentEventChange)> {
        match change {
            DbDocumentEventChange::Replace { .. } | DbDocumentEventChange::ReplaceWithUndefined => {
                let segment = path.next()?;
                let index = self
                    .iter()
                    .position(|t| t.key().to_string().as_str() == segment)?;

                self[index].apply_field(index, path, ui_path, change)
            }
            DbDocumentEventChange::Remove { key } => {
                let index = self.iter().position(|t| &t.key().to_string() == key)?;
                self.remove(index);
                Some((ui_path, UiDocumentEventChange::Remove { index }))
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
                Some((
                    ui_path,
                    UiDocumentEventChange::Move {
                        from,
                        to: to.unwrap_or(self.len() - 1),
                    },
                ))
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

                Some((
                    ui_path,
                    UiDocumentEventChange::Insert {
                        position,
                        value: value.clone(),
                    },
                ))
            }
        }
    }
}

impl<T: ApplyChange + serde::de::DeserializeOwned> ApplyChange for Option<T> {
    fn apply_change<'a>(
        &mut self,
        mut path: impl Iterator<Item = &'a str>,
        ui_path: String,
        change: &DbDocumentEventChange,
    ) -> Option<(String, UiDocumentEventChange)> {
        let segment = path.next();
        let is_empty = segment.is_none();
        let path = segment.into_iter().chain(path);

        if is_empty {
            match change {
                DbDocumentEventChange::Replace { value } => {
                    *self = Some(serde_json::from_value(value.clone()).unwrap());
                    return Some((
                        ui_path,
                        UiDocumentEventChange::Replace {
                            value: value.clone(),
                        },
                    ));
                }
                DbDocumentEventChange::ReplaceWithUndefined => {
                    *self = None;
                    return Some((ui_path, UiDocumentEventChange::ReplaceWithUndefined));
                }
                _ => (),
            }
        }
        match self {
            Some(value) => value.apply_change(path, ui_path, change),
            None => None,
        }
    }
}
