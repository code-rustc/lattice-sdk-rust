pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EntityStreamHeartbeat {
    #[serde(flatten)]
    pub heartbeat_object_fields: HeartbeatObject,
}
