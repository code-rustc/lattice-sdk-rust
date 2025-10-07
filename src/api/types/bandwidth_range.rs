pub use crate::prelude::*;

/// A component that describes the min and max bandwidths of a sensor
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BandwidthRange {
    #[serde(rename = "minimumBandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_bandwidth: Option<Bandwidth>,
    #[serde(rename = "maximumBandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_bandwidth: Option<Bandwidth>,
}
