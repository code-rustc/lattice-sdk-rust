pub use crate::prelude::*;

/// Media associated with an entity.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Media {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Vec<MediaItem>>,
}
