pub use crate::prelude::*;

/// List of payloads available for an entity.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Payloads {
    #[serde(rename = "payloadConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_configurations: Option<Vec<Payload>>,
}
