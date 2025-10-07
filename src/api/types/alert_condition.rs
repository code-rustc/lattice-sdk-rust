pub use crate::prelude::*;

/// A condition which may trigger an alert.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AlertCondition {
    /// Short, machine-readable code that describes this condition. This code is intended to provide systems off-asset
    /// with a lookup key to retrieve more detailed information about the condition.
    #[serde(rename = "conditionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_code: Option<String>,
    /// Human-readable description of this condition. The description is intended for display in the UI for human
    /// understanding and should not be used for machine processing. If the description is fixed and the vehicle controller
    /// provides no dynamic substitutions, then prefer lookup based on condition_code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
