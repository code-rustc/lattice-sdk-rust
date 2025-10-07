pub use crate::prelude::*;

/// Indicators to describe entity to consumers.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Indicators {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exercise: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emergency: Option<bool>,
    #[serde(rename = "c2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_2: Option<bool>,
    /// Indicates the Entity should be egressed to external sources.
    /// Integrations choose how the egressing happens (e.g. if an Entity needs fuzzing).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egressable: Option<bool>,
    /// A signal of arbitrary importance such that the entity should be globally marked for all users
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starred: Option<bool>,
}
