pub use crate::prelude::*;

/// Determines the type of relationship between this entity and another.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RelationshipType {
    #[serde(rename = "trackedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracked_by: Option<TrackedBy>,
    #[serde(rename = "groupChild")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_child: Option<GroupChild>,
    #[serde(rename = "groupParent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_parent: Option<GroupParent>,
    #[serde(rename = "mergedFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merged_from: Option<MergedFrom>,
    #[serde(rename = "activeTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_target: Option<ActiveTarget>,
}
