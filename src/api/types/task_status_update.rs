pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TaskStatusUpdate {
    #[serde(rename = "statusVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_version: Option<i64>,
    #[serde(rename = "newStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_status: Option<TaskStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<Principal>,
}
