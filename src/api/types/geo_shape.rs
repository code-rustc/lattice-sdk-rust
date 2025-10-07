pub use crate::prelude::*;

/// A component that describes the shape of a geo-entity.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeoShape {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point: Option<GeoPoint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<GeoLine>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polygon: Option<GeoPolygon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ellipse: Option<GeoEllipse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ellipsoid: Option<GeoEllipsoid>,
}
