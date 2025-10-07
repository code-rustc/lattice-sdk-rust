pub use crate::prelude::*;

/// An ellipsoid shaped geo-entity.
/// Principal axis lengths are defined in entity body space
/// This shape is NOT Geo-JSON compatible.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeoEllipsoid {
    /// Defines the distance from the center point to the surface along the forward axis
    #[serde(rename = "forwardAxisM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_axis_m: Option<f64>,
    /// Defines the distance from the center point to the surface along the side axis
    #[serde(rename = "sideAxisM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side_axis_m: Option<f64>,
    /// Defines the distance from the center point to the surface along the up axis
    #[serde(rename = "upAxisM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub up_axis_m: Option<f64>,
}
