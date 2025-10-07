pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EntityStreamEvent {
    #[serde(flatten)]
    pub entity_event_fields: EntityEvent,
}
