pub use crate::prelude::*;

/// Request to execute a Task.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExecuteRequest {
    /// Task to execute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<Task>,
}
