pub use crate::prelude::*;

/// An ellipse shaped geo-entity.
/// For a circle, the major and minor axis would be the same values.
/// This shape is NOT Geo-JSON compatible.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeoEllipse {
    /// Defines the distance from the center point of the ellipse to the furthest distance on the perimeter in meters.
    #[serde(rename = "semiMajorAxisM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semi_major_axis_m: Option<f64>,
    /// Defines the distance from the center point of the ellipse to the shortest distance on the perimeter in meters.
    #[serde(rename = "semiMinorAxisM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semi_minor_axis_m: Option<f64>,
    /// The orientation of the semi-major relative to true north in degrees from clockwise: 0-180 due to symmetry across the semi-minor axis.
    #[serde(rename = "orientationD")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation_d: Option<f64>,
    /// Optional height above entity position to extrude in meters. A non-zero value creates an elliptic cylinder
    #[serde(rename = "heightM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height_m: Option<f64>,
}
