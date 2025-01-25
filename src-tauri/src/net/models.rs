use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumDiscriminants, EnumString};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(transparent)]
pub struct LocalDeviceId(i32);

/// UUID
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(transparent)]
pub struct DocumentId(String);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(transparent)]
pub struct JsonPath(String);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(transparent)]
pub struct VectorClock(String);

#[derive(Debug, Serialize, Deserialize, EnumDiscriminants, Clone, PartialEq)]
#[strum_discriminants(derive(EnumString, Display))]
#[strum_discriminants(vis(pub))]
#[strum_discriminants(name(DocumentEventKind))]
#[strum_discriminants(strum(serialize_all = "lowercase"))]
pub enum DocumentEvent {
    Create {
        creator_id: LocalDeviceId,
        creator_counter: u64,
        document_id: DocumentId,
        document: serde_json::Value,
        vector_clock: VectorClock,
        timestamp: DateTime<Utc>,
    },
    Replace {
        creator_id: LocalDeviceId,
        creator_counter: u64,
        document_id: DocumentId,
        document: serde_json::Value,
        vector_clock: VectorClock,
        timestamp: DateTime<Utc>,
    },
    Update {
        creator_id: LocalDeviceId,
        creator_counter: u64,
        document_id: DocumentId,
        changes: Vec<(JsonPath, DocumentEventChange)>,
        vector_clock: VectorClock,
        timestamp: DateTime<Utc>,
    },
    Delete {
        creator_id: LocalDeviceId,
        creator_counter: u64,
        document_id: DocumentId,
        vector_clock: VectorClock,
        timestamp: DateTime<Utc>,
    },
}

pub type ArrayKey = String;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum DocumentEventChange {
    // Scalars
    Replace(serde_json::Value),
    ReplaceWithUndefined,
    // Arrays
    /// Moves value with given key before the value with the given key.
    Move {
        key: ArrayKey,
        /// If None, moves to the end of the array.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        before: Option<ArrayKey>,
    },
    /// Remove value in array but not using index
    Remove {
        key: ArrayKey,
    },
    /// Tries to insert before the key if it exists, otherwise appends.
    Insert {
        /// If None, appends.
        before: Option<ArrayKey>,
        value: serde_json::Value,
        key: ArrayKey,
    },
}
