pub use crate::prelude::*;

/// Indicates ellipse characteristics and probability that an entity lies within the defined ellipse.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ErrorEllipse {
    /// Defines the probability in percentage that an entity lies within the given ellipse: 0-1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probability: Option<f64>,
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
}
