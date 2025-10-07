pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MediaItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<MediaItemType>,
    /// The path, relative to the environment base URL, where media related to an entity can be accessed
    #[serde(rename = "relativePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_path: Option<String>,
}
