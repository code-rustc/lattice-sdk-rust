pub use crate::prelude::*;

/// Owner designates the entity responsible for writes of Task data.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Owner {
    /// Entity ID of the owner.
    #[serde(rename = "entityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
}
