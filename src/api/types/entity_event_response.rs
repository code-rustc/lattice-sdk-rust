pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EntityEventResponse {
    /// Long-poll session identifier. Use this token to resume polling on subsequent requests.
    #[serde(rename = "sessionToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_token: Option<String>,
    #[serde(rename = "entityEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_events: Option<Vec<EntityEvent>>,
}
