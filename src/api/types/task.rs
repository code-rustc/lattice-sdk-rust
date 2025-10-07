pub use crate::prelude::*;

/// A Task is something an agent can be asked to do.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Task {
    /// Version of this Task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<TaskVersion>,
    /// DEPRECATED: Human readable display name for this Task, should be short (<100 chars).
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Full Task parameterization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specification: Option<GoogleProtobufAny>,
    /// Records who created this Task. This field will not change after the Task has been created.
    #[serde(rename = "createdBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Principal>,
    /// Records who updated this Task last.
    #[serde(rename = "lastUpdatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by: Option<Principal>,
    /// Records the time of last update.
    #[serde(rename = "lastUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<DateTime<Utc>>,
    /// The status of this Task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TaskStatus>,
    /// If the Task has been scheduled to execute, what time it should execute at.
    #[serde(rename = "scheduledTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_time: Option<DateTime<Utc>>,
    /// Any related Tasks associated with this, typically includes an assignee for this Task and/or a parent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relations: Option<Relations>,
    /// Longer, free form human readable description of this Task
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// If set, execution of this Task is managed elsewhere, not by Task Manager.
    /// In other words, Task manager will not attempt to update the assigned agent with execution instructions.
    #[serde(rename = "isExecutedElsewhere")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_executed_elsewhere: Option<bool>,
    /// Time of Task creation.
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<DateTime<Utc>>,
    /// If populated, designates this to be a replicated Task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication: Option<Replication>,
    /// If populated, indicates an initial set of entities that can be used to execute an entity aware task
    /// For example, an entity Objective, an entity Keep In Zone, etc.
    /// These will not be updated during execution. If a taskable agent needs continuous updates on the entities from the
    /// COP, can call entity-manager, or use an AlternateId escape hatch.
    #[serde(rename = "initialEntities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_entities: Option<Vec<TaskEntity>>,
    /// The networked owner of this Task. It is used to ensure that linear writes occur on the node responsible
    /// for replication of task data to other nodes running Task Manager.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<Owner>,
}
