pub use crate::prelude::*;

/// Health for this component.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ComponentHealthHealth {
    #[serde(rename = "HEALTH_STATUS_INVALID")]
    HealthStatusInvalid,
    #[serde(rename = "HEALTH_STATUS_HEALTHY")]
    HealthStatusHealthy,
    #[serde(rename = "HEALTH_STATUS_WARN")]
    HealthStatusWarn,
    #[serde(rename = "HEALTH_STATUS_FAIL")]
    HealthStatusFail,
    #[serde(rename = "HEALTH_STATUS_OFFLINE")]
    HealthStatusOffline,
    #[serde(rename = "HEALTH_STATUS_NOT_READY")]
    HealthStatusNotReady,
}
impl fmt::Display for ComponentHealthHealth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::HealthStatusInvalid => "HEALTH_STATUS_INVALID",
            Self::HealthStatusHealthy => "HEALTH_STATUS_HEALTHY",
            Self::HealthStatusWarn => "HEALTH_STATUS_WARN",
            Self::HealthStatusFail => "HEALTH_STATUS_FAIL",
            Self::HealthStatusOffline => "HEALTH_STATUS_OFFLINE",
            Self::HealthStatusNotReady => "HEALTH_STATUS_NOT_READY",
        };
        write!(f, "{}", s)
    }
}
