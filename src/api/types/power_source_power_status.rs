pub use crate::prelude::*;

/// Status of the power source.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PowerSourcePowerStatus {
    #[serde(rename = "POWER_STATUS_INVALID")]
    PowerStatusInvalid,
    #[serde(rename = "POWER_STATUS_UNKNOWN")]
    PowerStatusUnknown,
    #[serde(rename = "POWER_STATUS_NOT_PRESENT")]
    PowerStatusNotPresent,
    #[serde(rename = "POWER_STATUS_OPERATING")]
    PowerStatusOperating,
    #[serde(rename = "POWER_STATUS_DISABLED")]
    PowerStatusDisabled,
    #[serde(rename = "POWER_STATUS_ERROR")]
    PowerStatusError,
}
impl fmt::Display for PowerSourcePowerStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PowerStatusInvalid => "POWER_STATUS_INVALID",
            Self::PowerStatusUnknown => "POWER_STATUS_UNKNOWN",
            Self::PowerStatusNotPresent => "POWER_STATUS_NOT_PRESENT",
            Self::PowerStatusOperating => "POWER_STATUS_OPERATING",
            Self::PowerStatusDisabled => "POWER_STATUS_DISABLED",
            Self::PowerStatusError => "POWER_STATUS_ERROR",
        };
        write!(f, "{}", s)
    }
}
