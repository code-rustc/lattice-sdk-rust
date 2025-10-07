pub use crate::prelude::*;

/// Represents a frustum in which which all four corner points project onto the ground. All points in this message
/// are optional, if the projection to the ground fails then they will not be populated.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectedFrustum {
    /// Upper left point of the frustum.
    #[serde(rename = "upperLeft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_left: Option<Position>,
    /// Upper right point of the frustum.
    #[serde(rename = "upperRight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_right: Option<Position>,
    /// Bottom right point of the frustum.
    #[serde(rename = "bottomRight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom_right: Option<Position>,
    /// Bottom left point of the frustum.
    #[serde(rename = "bottomLeft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom_left: Option<Position>,
}
