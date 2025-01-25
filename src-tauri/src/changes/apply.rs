use crate::net::models::DocumentEventChange as DbDocumentEventChange;
use crate::ui::models::{
    Document, DocumentEventChange as UiDocumentEventChange, DocumentId, DocumentKind,
    DocumentTitle, RawIngredient, Recipe, RecipeIngredient, ShoppingList, ShoppingListItem,
    ShoppingListItemState,
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
        path: impl Iterator<Item = &'a str>,
        ui_path: String,
        change: &DbDocumentEventChange,
    ) -> Option<(String, UiDocumentEventChange)> {
        match self {
            Self::ShoppingList(list) => list.apply_change(path, ui_path, change),
            Self::RawIngredient(ingredient) => ingredient.apply_change(path, ui_path, change),
            Self::Recipe(recipe) => recipe.apply_change(path, ui_path, change),
        }
    }
}

impl ApplyChange for ShoppingList {
    fn apply_change<'a>(
        &mut self,
        mut path: impl Iterator<Item = &'a str>,
        ui_path: String,
        change: &DbDocumentEventChange,
    ) -> Option<(String, UiDocumentEventChange)> {
        let segment = path.next()?;
        match segment {
            "items" => self.items.apply_field("items", path, ui_path, change),
            _ => unreachable!(),
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

impl ApplyChange for RawIngredient {
    fn apply_change<'a>(
        &mut self,
        mut path: impl Iterator<Item = &'a str>,
        ui_path: String,
        change: &DbDocumentEventChange,
    ) -> Option<(String, UiDocumentEventChange)> {
        let segment = path.next()?;
        match segment {
            "name" => self.name.apply_field("name", path, ui_path, change),
            "container" => self
                .container
                .apply_field("container", path, ui_path, change),
            "proteins" => self.proteins.apply_field("proteins", path, ui_path, change),
            "carbs" => self.carbs.apply_field("carbs", path, ui_path, change),
            "fat" => self.fat.apply_field("fat", path, ui_path, change),
            _ => unreachable!(),
        }
    }
}

impl ApplyChange for Recipe {
    fn apply_change<'a>(
        &mut self,
        mut path: impl Iterator<Item = &'a str>,
        ui_path: String,
        change: &DbDocumentEventChange,
    ) -> Option<(String, UiDocumentEventChange)> {
        let segment = path.next()?;
        match segment {
            "name" => self.name.apply_field("name", path, ui_path, change),
            "ingredients" => self
                .ingredients
                .apply_field("ingredients", path, ui_path, change),
            _ => unreachable!(),
        }
    }
}

impl ApplyChange for RecipeIngredient {
    fn apply_change<'a>(
        &mut self,
        mut path: impl Iterator<Item = &'a str>,
        ui_path: String,
        change: &DbDocumentEventChange,
    ) -> Option<(String, UiDocumentEventChange)> {
        let segment = path.next()?;
        match segment {
            "id" => self.id.apply_field("id", path, ui_path, change),
            "raw" => self.raw.apply_field("raw", path, ui_path, change),
            _ => unreachable!(),
        }
    }
}
