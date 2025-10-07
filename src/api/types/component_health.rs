pub use crate::prelude::*;

/// Health of an individual component.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ComponentHealth {
    /// Consistent internal ID for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Display name for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Health for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health: Option<ComponentHealthHealth>,
    /// Human-readable describing the component state. These messages should be understandable by end users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<ComponentMessage>>,
    /// The last update time for this specific component.
    /// If this timestamp is unset, the data is assumed to be most recent
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<DateTime<Utc>>,
}
