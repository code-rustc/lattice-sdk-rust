pub use crate::prelude::*;

/// Range rings allow visual assessment of map distance at varying zoom levels.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RangeRings {
    /// The minimum range ring distance, specified in meters.
    #[serde(rename = "minDistanceM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_distance_m: Option<f64>,
    /// The maximum range ring distance, specified in meters.
    #[serde(rename = "maxDistanceM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_distance_m: Option<f64>,
    /// The count of range rings.
    #[serde(rename = "ringCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ring_count: Option<i64>,
    /// The color of range rings, specified in hex string.
    #[serde(rename = "ringLineColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ring_line_color: Option<Color>,
}
