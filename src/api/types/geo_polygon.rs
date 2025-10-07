pub use crate::prelude::*;

/// A polygon shaped geo-entity.
/// See https://datatracker.ietf.org/doc/html/rfc7946#section-3.1.6, only canonical representations accepted
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeoPolygon {
    /// An array of LinearRings where the first item is the exterior ring and subsequent items are interior rings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rings: Option<Vec<LinearRing>>,
    /// An extension hint that this polygon is a rectangle. When true this implies several things:
    /// * exactly 1 linear ring with 5 points (starting corner, 3 other corners and start again)
    /// * each point has the same altitude corresponding with the plane of the rectangle
    /// * each point has the same height (either all present and equal, or all not present)
    #[serde(rename = "isRectangle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_rectangle: Option<bool>,
}
