use serde::{Deserialize, Serialize};
use strum::{EnumDiscriminants, EnumString, IntoStaticStr};

#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    pub id: DocumentId,
    pub title: DocumentTitle,
    #[serde(flatten)]
    pub kind: DocumentKind,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(transparent)]
pub struct DocumentId(i32);
impl From<i32> for DocumentId {
    fn from(i: i32) -> Self {
        Self(i)
    }
}
impl From<DocumentId> for i32 {
    fn from(id: DocumentId) -> Self {
        id.0
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct DocumentTitle(String);
impl AsRef<str> for DocumentTitle {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl From<String> for DocumentTitle {
    fn from(s: String) -> Self {
        Self(s)
    }
}
impl From<&str> for DocumentTitle {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

#[derive(Serialize, Deserialize)]
pub struct NewDocument {
    pub title: DocumentTitle,
    #[serde(flatten)]
    pub kind: DocumentKind,
}

#[derive(Serialize, Deserialize, EnumDiscriminants, Debug)]
#[serde(tag = "kind")]
#[serde(rename_all = "camelCase")]
#[strum_discriminants(name(JustDocumentKind))]
#[strum_discriminants(derive(EnumString, IntoStaticStr, serde::Deserialize, serde::Serialize))]
#[strum_discriminants(serde(rename_all = "camelCase"))]
#[strum_discriminants(strum(serialize_all = "camelCase"))]
pub enum DocumentKind {
    RawIngredient(RawIngredient),
    Recipe(Recipe),
    ShoppingList(ShoppingList),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawIngredient {
    pub name: String,
    pub container: usize,
    pub proteins: f64,
    pub carbs: f64,
    pub fat: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Recipe {
    pub name: String,
    pub ingredients: Vec<RecipeIngredient>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecipeIngredient {
    pub id: String,
    pub raw: RawIngredient,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingList {
    pub items: Vec<ShoppingListItem>,
}

impl std::fmt::Display for JustDocumentKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: &'static str = self.into();
        write!(f, "{}", s)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShoppingListItem {
    pub id: String,
    pub name: String,
    pub amount: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    pub state: ShoppingListItemState,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShoppingListItemState {
    /// Item is in the shopping list
    Todo,
    /// Item is in the cart
    InCart,
    /// Item has been bought
    Bought,
    /// Item is not being bought
    /// For example, it is not available in the store or it is too expensive
    NotBuying,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ShoppingListSuggestion {
    pub id: String,
    pub name: String,
    pub amount: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum AddShoppingListItem {
    Item(ShoppingListItem),
    Suggestion(ShoppingListSuggestion),
}

impl AddShoppingListItem {
    pub fn name(&self) -> &str {
        match self {
            Self::Item(item) => &item.name,
            Self::Suggestion(suggestion) => &suggestion.name,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(transparent)]
pub struct JsonPath(String);
impl AsRef<str> for JsonPath {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl From<String> for JsonPath {
    fn from(s: String) -> Self {
        Self(s)
    }
}

/// Notification about changes in document.
#[derive(Serialize, Clone)]
#[serde(tag = "kind", rename_all = "camelCase")]
#[allow(dead_code)]
pub enum EventNotification {
    /// Document has been created.
    Created { id: DocumentId },
    /// Document has been replaced.
    Replaced { id: DocumentId },
    /// Document has been updated.
    Updated {
        id: DocumentId,
        changes: Vec<(JsonPath, DocumentEventChange)>,
    },
    /// Document has been deleted.
    Deleted { id: DocumentId },
}

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum DocumentEventChange {
    // Scalars
    Replace {
        value: serde_json::Value,
    },
    ReplaceWithUndefined,
    // Arrays
    /// Moves value with given key before the value with the given key.
    Move {
        from: usize,
        to: usize,
    },
    /// Remove value in array but not using index
    Remove {
        index: usize,
    },
    /// Tries to insert before the key if it exists, otherwise appends.
    Insert {
        /// If None, appends.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        position: Option<usize>,
        value: serde_json::Value,
    },
}
