pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EntityEventRequest {
    #[serde(rename = "sessionToken")]
    pub session_token: String,
    #[serde(rename = "batchSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i64>,
}
