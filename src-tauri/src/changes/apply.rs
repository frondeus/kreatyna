use crate::net::models::DocumentEventChange as DbDocumentEventChange;
use crate::ui::models::{
    Document, DocumentEventChange as UiDocumentEventChange, DocumentId, DocumentKind,
    DocumentTitle, ShoppingListItem, ShoppingListItemState,
};

mod generic;

pub trait ApplyChange {
    fn apply_change(
        &mut self,
        path: &[&str],
        change: &DbDocumentEventChange,
    ) -> Option<UiDocumentEventChange>;
}

fn split_segment<'a>(path: &'a [&str]) -> (&'a str, &'a [&'a str]) {
    let [segment, path @ ..] = path else {
        panic!();
    };
    (segment, path)
}

impl ApplyChange for Document {
    fn apply_change(
        &mut self,
        path: &[&str],
        change: &DbDocumentEventChange,
    ) -> Option<UiDocumentEventChange> {
        let full_path = path;
        let (segment, path) = split_segment(path);
        match segment {
            "id" => self.id.apply_change(path, change),
            "title" => self.title.apply_change(path, change),
            _ => self.kind.apply_change(full_path, change),
        }
    }
}

crate::apply_scalar!(DocumentId);
crate::apply_scalar!(DocumentTitle);

impl ApplyChange for DocumentKind {
    fn apply_change(
        &mut self,
        path: &[&str],
        change: &DbDocumentEventChange,
    ) -> Option<UiDocumentEventChange> {
        let (segment, path) = split_segment(path);
        match self {
            Self::ShoppingList { items } => match segment {
                "items" => items.apply_change(path, change),
                _ => unreachable!(),
            },
        }
    }
}

impl ApplyChange for ShoppingListItem {
    fn apply_change(
        &mut self,
        path: &[&str],
        change: &DbDocumentEventChange,
    ) -> Option<UiDocumentEventChange> {
        let (segment, path) = split_segment(path);
        match segment {
            "id" => self.id.apply_change(path, change),
            "name" => self.name.apply_change(path, change),
            "amount" => self.amount.apply_change(path, change),
            "unit" => self.unit.apply_change(path, change),
            "state" => self.state.apply_change(path, change),
            _ => unreachable!(),
        }
    }
}

crate::apply_scalar!(ShoppingListItemState);
