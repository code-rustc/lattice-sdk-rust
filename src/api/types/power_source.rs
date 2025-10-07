pub use crate::prelude::*;

/// Represents the state of a single power source that is connected to this entity.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PowerSource {
    /// Status of the power source.
    #[serde(rename = "powerStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_status: Option<PowerSourcePowerStatus>,
    /// Used to determine the type of power source.
    #[serde(rename = "powerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_type: Option<PowerSourcePowerType>,
    /// Power level of the system. If absent, the power level is assumed to be unknown.
    #[serde(rename = "powerLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_level: Option<PowerLevel>,
    /// Set of human-readable messages with status of the power system. Typically this would be used in an error state
    /// to provide additional error information. This can also be used for informational messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<String>>,
    /// Whether the power source is offloadable. If the value is missing (as opposed to false) then the entity does not
    /// report whether the power source is offloadable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offloadable: Option<bool>,
}
