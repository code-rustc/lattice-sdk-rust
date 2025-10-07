pub use crate::prelude::*;

/// A line of bearing of a signal.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LineOfBearing {
    /// The direction pointing from this entity to the detection
    #[serde(rename = "angleOfArrival")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub angle_of_arrival: Option<AngleOfArrival>,
    /// The estimated distance of the detection
    #[serde(rename = "rangeEstimateM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_estimate_m: Option<Measurement>,
    /// The maximum distance of the detection
    #[serde(rename = "maxRangeM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_range_m: Option<Measurement>,
}
