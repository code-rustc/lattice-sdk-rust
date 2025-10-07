pub use crate::prelude::*;

/// status of the override
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OverrideStatus {
    #[serde(rename = "OVERRIDE_STATUS_INVALID")]
    OverrideStatusInvalid,
    #[serde(rename = "OVERRIDE_STATUS_APPLIED")]
    OverrideStatusApplied,
    #[serde(rename = "OVERRIDE_STATUS_PENDING")]
    OverrideStatusPending,
    #[serde(rename = "OVERRIDE_STATUS_TIMEOUT")]
    OverrideStatusTimeout,
    #[serde(rename = "OVERRIDE_STATUS_REJECTED")]
    OverrideStatusRejected,
    #[serde(rename = "OVERRIDE_STATUS_DELETION_PENDING")]
    OverrideStatusDeletionPending,
}
impl fmt::Display for OverrideStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::OverrideStatusInvalid => "OVERRIDE_STATUS_INVALID",
            Self::OverrideStatusApplied => "OVERRIDE_STATUS_APPLIED",
            Self::OverrideStatusPending => "OVERRIDE_STATUS_PENDING",
            Self::OverrideStatusTimeout => "OVERRIDE_STATUS_TIMEOUT",
            Self::OverrideStatusRejected => "OVERRIDE_STATUS_REJECTED",
            Self::OverrideStatusDeletionPending => "OVERRIDE_STATUS_DELETION_PENDING",
        };
        write!(f, "{}", s)
    }
}
