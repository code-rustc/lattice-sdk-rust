pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SensorOperationalState {
    #[serde(rename = "OPERATIONAL_STATE_INVALID")]
    OperationalStateInvalid,
    #[serde(rename = "OPERATIONAL_STATE_OFF")]
    OperationalStateOff,
    #[serde(rename = "OPERATIONAL_STATE_NON_OPERATIONAL")]
    OperationalStateNonOperational,
    #[serde(rename = "OPERATIONAL_STATE_DEGRADED")]
    OperationalStateDegraded,
    #[serde(rename = "OPERATIONAL_STATE_OPERATIONAL")]
    OperationalStateOperational,
    #[serde(rename = "OPERATIONAL_STATE_DENIED")]
    OperationalStateDenied,
}
impl fmt::Display for SensorOperationalState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::OperationalStateInvalid => "OPERATIONAL_STATE_INVALID",
            Self::OperationalStateOff => "OPERATIONAL_STATE_OFF",
            Self::OperationalStateNonOperational => "OPERATIONAL_STATE_NON_OPERATIONAL",
            Self::OperationalStateDegraded => "OPERATIONAL_STATE_DEGRADED",
            Self::OperationalStateOperational => "OPERATIONAL_STATE_OPERATIONAL",
            Self::OperationalStateDenied => "OPERATIONAL_STATE_DENIED",
        };
        write!(f, "{}", s)
    }
}
