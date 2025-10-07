pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListObjectsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "sinceTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_timestamp: Option<DateTime<Utc>>,
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(rename = "allObjectsInMesh")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_objects_in_mesh: Option<bool>,
}
