pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PayloadConfiguration {
    /// Identifying ID for the capability.
    /// This ID may be used multiple times to represent payloads that are the same capability but have different operational states
    #[serde(rename = "capabilityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability_id: Option<String>,
    /// The number of payloads currently available in the configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    /// The target environments the configuration is effective against.
    #[serde(rename = "effectiveEnvironment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_environment: Option<Vec<PayloadConfigurationEffectiveEnvironmentItem>>,
    /// The operational state of this payload.
    #[serde(rename = "payloadOperationalState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_operational_state: Option<PayloadConfigurationPayloadOperationalState>,
    /// A human readable description of the payload
    #[serde(rename = "payloadDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_description: Option<String>,
}
