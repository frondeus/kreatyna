use strum::{AsRefStr, EnumDiscriminants, EnumString, IntoStaticStr};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Document {
    pub id: DocumentId,
    pub title: DocumentTitle,
    #[serde(flatten)]
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
    #[serde(flatten)]
    pub kind: DocumentKind,
}

#[derive(Serialize, Deserialize, EnumDiscriminants)]
#[serde(tag = "kind")]
#[serde(rename_all = "camelCase")]
#[strum_discriminants(name(JustDocumentKind))]
#[strum_discriminants(derive(EnumString, IntoStaticStr, serde::Deserialize, serde::Serialize))]
#[strum_discriminants(serde(rename_all = "camelCase"))]
#[strum_discriminants(strum(serialize_all = "camelCase"))]
pub enum DocumentKind {
    ShoppingList {
        items: Vec<String>
    },
}


impl std::fmt::Display for JustDocumentKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: &'static str = self.into();
        write!(f, "{}", s)
    }
}