pub use crate::prelude::*;

/// A closed ring of points. The first and last point must be the same.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LinearRing {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positions: Option<Vec<GeoPolygonPosition>>,
}
