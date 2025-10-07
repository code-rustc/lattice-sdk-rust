pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TaskCreation {
    #[serde(rename = "taskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specification: Option<GoogleProtobufAny>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<Principal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relations: Option<Relations>,
    #[serde(rename = "isExecutedElsewhere")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_executed_elsewhere: Option<bool>,
    #[serde(rename = "initialEntities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_entities: Option<Vec<TaskEntity>>,
}
