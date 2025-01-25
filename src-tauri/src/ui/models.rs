use strum::{AsRefStr, EnumDiscriminants, EnumString, IntoStaticStr};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Document {
    pub id: DocumentId,
    pub title: DocumentTitle,
    pub kind: DocumentKind,
}

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct DocumentId(i32);
impl From<i32> for DocumentId {
    fn from(i: i32) -> Self {
        Self(i)
    }
}

#[derive(Serialize, Deserialize)]
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
    pub kind: DocumentKind,
}

#[derive(Serialize, Deserialize, EnumDiscriminants)]
#[strum_discriminants(name(JustDocumentKind))]
#[strum_discriminants(derive(EnumString, IntoStaticStr, serde::Deserialize, serde::Serialize))]
#[strum_discriminants(serde(rename_all = "lowercase"))]
#[strum_discriminants(strum(serialize_all = "lowercase"))]
pub enum DocumentKind {
    ShoppingList {
        items: Vec<String>
    },
}

