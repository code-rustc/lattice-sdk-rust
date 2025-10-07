pub use crate::prelude::*;

/// A position in a GeoPolygon with an optional extruded height.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeoPolygonPosition {
    /// base position. if no altitude set, its on the ground.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    /// optional height above base position to extrude in meters.
    /// for a given polygon, all points should have a height or none of them.
    /// strictly GeoJSON compatible polygons will not have this set.
    #[serde(rename = "heightM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height_m: Option<f64>,
}
