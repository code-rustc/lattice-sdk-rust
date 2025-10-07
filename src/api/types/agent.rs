pub use crate::prelude::*;

/// Represents an Agent in the COP.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Agent {
    /// Entity ID of the agent.
    #[serde(rename = "entityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
}
