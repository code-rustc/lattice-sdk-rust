pub use crate::prelude::*;

/// Event representing some type of entity change.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EntityEvent {
    #[serde(rename = "eventType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<EntityEventEventType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<Entity>,
}
