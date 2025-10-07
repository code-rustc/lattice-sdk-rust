pub use crate::prelude::*;

/// Request to Cancel a Task.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CancelRequest {
    /// ID of the Task to cancel.
    #[serde(rename = "taskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// The assignee of the Task. Useful for agent routing where an endpoint owns multiple agents,
    /// especially onBehalfOf assignees.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<Principal>,
}
