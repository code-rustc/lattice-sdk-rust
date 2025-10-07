pub use crate::prelude::*;

/// TaskStatus is contains information regarding the status of a Task at any given time. Can include related information
/// such as any progress towards Task completion, or any associated results if Task completed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TaskStatus {
    /// Status of the Task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TaskStatusStatus>,
    /// Any errors associated with the Task.
    #[serde(rename = "taskError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_error: Option<TaskError>,
    /// Any incremental progress on the Task, should be from the tasks/v* /progress folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<GoogleProtobufAny>,
    /// Any final result of the Task, should be from tasks/v* /result folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GoogleProtobufAny>,
    /// Time the Task began execution, may not be known even for executing Tasks.
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<DateTime<Utc>>,
    /// Any estimate for how the Task will progress, should be from tasks/v* /estimates folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimate: Option<GoogleProtobufAny>,
    /// Any allocated agents of the Task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation: Option<Allocation>,
}
