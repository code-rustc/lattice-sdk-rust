pub use crate::prelude::*;

/// Relations describes the relationships of this Task, such as assignment, or if the Task has any parents.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Relations {
    /// Who or what, if anyone, this Task is currently assigned to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<Principal>,
    /// If this Task is a "sub-Task", what is its parent, none if empty.
    #[serde(rename = "parentTaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_task_id: Option<String>,
}
