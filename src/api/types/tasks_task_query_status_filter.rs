pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TaskQueryStatusFilter {
    /// Status of the Task to filter by, inclusive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TaskQueryStatusFilterStatus>,
}
