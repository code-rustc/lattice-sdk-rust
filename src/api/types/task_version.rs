pub use crate::prelude::*;

/// Version of a Task.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TaskVersion {
    /// The unique ID for this Task.
    #[serde(rename = "taskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// Increments on definition (i.e. not TaskStatus) change. 0 is unset, starts at 1 on creation.
    #[serde(rename = "definitionVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition_version: Option<i64>,
    /// Increments on changes to TaskStatus. 0 is unset, starts at 1 on creation.
    #[serde(rename = "statusVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_version: Option<i64>,
}
