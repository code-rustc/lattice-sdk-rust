pub use crate::prelude::*;

/// The target prioritization associated with an entity.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TargetPriority {
    /// Describes the target priority in relation to high value target lists.
    #[serde(rename = "highValueTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_value_target: Option<HighValueTarget>,
    /// Describes whether the entity should be treated as a threat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat: Option<Threat>,
}
