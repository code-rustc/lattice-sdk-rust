pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaskQuery {
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(rename = "parentTaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_task_id: Option<String>,
    #[serde(rename = "statusFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_filter: Option<TaskQueryStatusFilter>,
    #[serde(rename = "updateTimeRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time_range: Option<TaskQueryUpdateTimeRange>,
}
