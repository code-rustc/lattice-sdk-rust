pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "event")]
pub enum StreamEntitiesResponse {
    Heartbeat {
        #[serde(flatten)]
        data: EntityStreamHeartbeat,
    },

    Entity {
        #[serde(flatten)]
        data: EntityStreamEvent,
    },
}
