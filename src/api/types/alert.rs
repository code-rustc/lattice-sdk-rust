pub use crate::prelude::*;

/// An alert informs operators of critical events related to system performance and mission
/// execution. An alert is produced as a result of one or more alert conditions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Alert {
    /// Short, machine-readable code that describes this alert. This code is intended to provide systems off-asset
    /// with a lookup key to retrieve more detailed information about the alert.
    #[serde(rename = "alertCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_code: Option<String>,
    /// Human-readable description of this alert. The description is intended for display in the UI for human
    /// understanding and should not be used for machine processing. If the description is fixed and the vehicle controller
    /// provides no dynamic substitutions, then prefer lookup based on alert_code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Alert level (Warning, Caution, or Advisory).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<AlertLevel>,
    /// Time at which this alert was activated.
    #[serde(rename = "activatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activated_time: Option<DateTime<Utc>>,
    /// Set of conditions which have activated this alert.
    #[serde(rename = "activeConditions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_conditions: Option<Vec<AlertCondition>>,
}
