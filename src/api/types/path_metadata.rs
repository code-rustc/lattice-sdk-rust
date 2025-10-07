pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PathMetadata {
    pub content_identifier: ContentIdentifier,
    pub size_bytes: i64,
    pub last_updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_time: Option<DateTime<Utc>>,
}
