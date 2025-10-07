pub use crate::prelude::*;

/// Catalog of supported tasks.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TaskCatalog {
    #[serde(rename = "taskDefinitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definitions: Option<Vec<TaskDefinition>>,
}
