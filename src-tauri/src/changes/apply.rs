use crate::net::models::DocumentEventChange as DbDocumentEventChange;
use crate::ui::models::{
    Document, DocumentEventChange as UiDocumentEventChange, DocumentId, DocumentKind,
    DocumentTitle, ShoppingListItem, ShoppingListItemState,
};

mod generic;

pub trait ApplyChange {
    fn apply(
        &mut self,
        path: &str,
        change: &DbDocumentEventChange,
    ) -> Option<(String, UiDocumentEventChange)> {
        let path = path.split('.');
        self.apply_change(path, "".to_string(), change)
    }

    fn apply_change<'a>(
        &mut self,
        path: impl Iterator<Item = &'a str>,
        ui_path: String,
        change: &DbDocumentEventChange,
    ) -> Option<(String, UiDocumentEventChange)>;

    fn apply_field<'a>(
        &mut self,
        field: impl std::fmt::Display,
        path: impl Iterator<Item = &'a str>,
        ui_path: String,
        change: &DbDocumentEventChange,
    ) -> Option<(String, UiDocumentEventChange)> {
        if ui_path.is_empty() {
            return self.apply_change(path, field.to_string(), change);
        }
        self.apply_change(path, format!("{ui_path}.{field}"), change)
    }
}

impl ApplyChange for Document {
    fn apply_change<'a>(
        &mut self,
        mut path: impl Iterator<Item = &'a str>,
        ui_path: String,
        change: &DbDocumentEventChange,
    ) -> Option<(String, UiDocumentEventChange)> {
        let segment = path.next()?;
        match segment {
            "id" => self.id.apply_field("id", path, ui_path, change),
            "title" => self.title.apply_field("title", path, ui_path, change),
            _ => {
                let full_path = std::iter::once(segment).chain(path);
                self.kind.apply_change(full_path, ui_path, change)
            }
        }
    }
}

crate::apply_scalar!(DocumentId);
crate::apply_scalar!(DocumentTitle);

impl ApplyChange for DocumentKind {
    fn apply_change<'a>(
        &mut self,
        mut path: impl Iterator<Item = &'a str>,
        ui_path: String,
        change: &DbDocumentEventChange,
    ) -> Option<(String, UiDocumentEventChange)> {
        let segment = path.next()?;
        match self {
            Self::ShoppingList { items } => match segment {
                "items" => items.apply_field("items", path, ui_path, change),
                _ => unreachable!(),
            },
        }
    }
}

impl ApplyChange for ShoppingListItem {
    fn apply_change<'a>(
        &mut self,
        mut path: impl Iterator<Item = &'a str>,
        ui_path: String,
        change: &DbDocumentEventChange,
    ) -> Option<(String, UiDocumentEventChange)> {
        let segment = path.next()?;
        match segment {
            "id" => self.id.apply_field("id", path, ui_path, change),
            "name" => self.name.apply_field("name", path, ui_path, change),
            "amount" => self.amount.apply_field("amount", path, ui_path, change),
            "unit" => self.unit.apply_field("unit", path, ui_path, change),
            "state" => self.state.apply_field("state", path, ui_path, change),
            _ => unreachable!(),
        }
    }
}

crate::apply_scalar!(ShoppingListItemState);
