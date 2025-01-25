use strum::{EnumDiscriminants, EnumString, IntoStaticStr};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Document {
    pub title: String,
    pub path: String,
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

