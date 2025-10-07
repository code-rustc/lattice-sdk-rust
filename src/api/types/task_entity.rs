pub use crate::prelude::*;

/// Wrapper of an entity passed in Tasking, used to hold an additional information, and as a future extension point.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TaskEntity {
    /// The wrapped entity-manager entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<Entity>,
    /// Indicates that this entity was generated from a snapshot of a live entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<bool>,
}
