pub use crate::prelude::*;

/// Error code for Task error.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TaskErrorCode {
    #[serde(rename = "ERROR_CODE_INVALID")]
    ErrorCodeInvalid,
    #[serde(rename = "ERROR_CODE_CANCELLED")]
    ErrorCodeCancelled,
    #[serde(rename = "ERROR_CODE_REJECTED")]
    ErrorCodeRejected,
    #[serde(rename = "ERROR_CODE_TIMEOUT")]
    ErrorCodeTimeout,
    #[serde(rename = "ERROR_CODE_FAILED")]
    ErrorCodeFailed,
}
impl fmt::Display for TaskErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ErrorCodeInvalid => "ERROR_CODE_INVALID",
            Self::ErrorCodeCancelled => "ERROR_CODE_CANCELLED",
            Self::ErrorCodeRejected => "ERROR_CODE_REJECTED",
            Self::ErrorCodeTimeout => "ERROR_CODE_TIMEOUT",
            Self::ErrorCodeFailed => "ERROR_CODE_FAILED",
        };
        write!(f, "{}", s)
    }
}
