pub use crate::prelude::*;

/// Defines a supported task by the task specification URL of its "Any" type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TaskDefinition {
    /// Url path must be prefixed with `type.googleapis.com/`.
    #[serde(rename = "taskSpecificationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_specification_url: Option<String>,
}
