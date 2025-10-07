pub use crate::prelude::*;

/// General health of the entity as reported by the entity.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Health {
    /// Status indicating whether the entity is able to communicate with Entity Manager.
    #[serde(rename = "connectionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_status: Option<HealthConnectionStatus>,
    /// Top-level health status; typically a roll-up of individual component healths.
    #[serde(rename = "healthStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_status: Option<HealthHealthStatus>,
    /// Health of individual components running on this Entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<ComponentHealth>>,
    /// The update time for the top-level health information.
    /// If this timestamp is unset, the data is assumed to be most recent
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<DateTime<Utc>>,
    /// Active alerts indicate a critical change in system state sent by the asset
    /// that must be made known to an operator or consumer of the common operating picture.
    /// Alerts are different from ComponentHealth messages--an active alert does not necessarily
    /// indicate a component is in an unhealthy state. For example, an asset may trigger
    /// an active alert based on fuel levels running low. Alerts should be removed from this list when their conditions
    /// are cleared. In other words, only active alerts should be reported here.
    #[serde(rename = "activeAlerts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_alerts: Option<Vec<Alert>>,
}
