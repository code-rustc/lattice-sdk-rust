pub use crate::prelude::*;

/// The operational state of this payload.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PayloadConfigurationPayloadOperationalState {
    #[serde(rename = "PAYLOAD_OPERATIONAL_STATE_INVALID")]
    PayloadOperationalStateInvalid,
    #[serde(rename = "PAYLOAD_OPERATIONAL_STATE_OFF")]
    PayloadOperationalStateOff,
    #[serde(rename = "PAYLOAD_OPERATIONAL_STATE_NON_OPERATIONAL")]
    PayloadOperationalStateNonOperational,
    #[serde(rename = "PAYLOAD_OPERATIONAL_STATE_DEGRADED")]
    PayloadOperationalStateDegraded,
    #[serde(rename = "PAYLOAD_OPERATIONAL_STATE_OPERATIONAL")]
    PayloadOperationalStateOperational,
    #[serde(rename = "PAYLOAD_OPERATIONAL_STATE_OUT_OF_SERVICE")]
    PayloadOperationalStateOutOfService,
    #[serde(rename = "PAYLOAD_OPERATIONAL_STATE_UNKNOWN")]
    PayloadOperationalStateUnknown,
}
impl fmt::Display for PayloadConfigurationPayloadOperationalState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PayloadOperationalStateInvalid => "PAYLOAD_OPERATIONAL_STATE_INVALID",
            Self::PayloadOperationalStateOff => "PAYLOAD_OPERATIONAL_STATE_OFF",
            Self::PayloadOperationalStateNonOperational => {
                "PAYLOAD_OPERATIONAL_STATE_NON_OPERATIONAL"
            }
            Self::PayloadOperationalStateDegraded => "PAYLOAD_OPERATIONAL_STATE_DEGRADED",
            Self::PayloadOperationalStateOperational => "PAYLOAD_OPERATIONAL_STATE_OPERATIONAL",
            Self::PayloadOperationalStateOutOfService => "PAYLOAD_OPERATIONAL_STATE_OUT_OF_SERVICE",
            Self::PayloadOperationalStateUnknown => "PAYLOAD_OPERATIONAL_STATE_UNKNOWN",
        };
        write!(f, "{}", s)
    }
}
