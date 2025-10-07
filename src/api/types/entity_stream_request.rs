pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EntityStreamRequest {
    #[serde(rename = "heartbeatIntervalMS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heartbeat_interval_ms: Option<i64>,
    #[serde(rename = "preExistingOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_existing_only: Option<bool>,
    #[serde(rename = "componentsToInclude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components_to_include: Option<Vec<String>>,
}
