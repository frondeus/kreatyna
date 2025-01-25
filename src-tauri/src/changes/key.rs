use crate::ui::models::ShoppingListItem;

pub trait WithKey {
    type Key: PartialEq + Eq + Ord + PartialOrd + std::fmt::Display + Clone;
    fn key(&self) -> Self::Key;
}

impl WithKey for ShoppingListItem {
    type Key = String;
    fn key(&self) -> Self::Key {
        self.id.clone()
    }
}
