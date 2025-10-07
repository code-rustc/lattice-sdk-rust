pub use crate::prelude::*;

/// Describes whether an entity is a threat or not.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Threat {
    /// Indicates that the entity has been determined to be a threat.
    #[serde(rename = "isThreat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_threat: Option<bool>,
}
