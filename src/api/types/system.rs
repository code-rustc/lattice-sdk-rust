pub use crate::prelude::*;

/// System Principal representing some autonomous system.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct System {
    /// Name of the service associated with this System.
    #[serde(rename = "serviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    /// The Entity ID of the System.
    #[serde(rename = "entityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    /// Whether the System Principal (for example, an Asset) can own scheduling.
    /// This means we bypass manager-owned scheduling and defer to the system
    /// Principal to handle scheduling and give us status updates for the Task.
    /// Regardless of the value defined by the client, the Task Manager will
    /// determine and set this value appropriately.
    #[serde(rename = "managesOwnScheduling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manages_own_scheduling: Option<bool>,
}
