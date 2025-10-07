pub use crate::prelude::*;

/// TaskError contains an error code and message typically associated to a Task.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TaskError {
    /// Error code for Task error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<TaskErrorCode>,
    /// Descriptive human-readable string regarding this error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Any additional details regarding this error.
    #[serde(rename = "errorDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<GoogleProtobufAny>,
}
