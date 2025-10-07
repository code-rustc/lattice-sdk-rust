pub use crate::prelude::*;

/// Represents RF configurations supported on this sensor.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RfConfiguration {
    /// Frequency ranges that are available for this sensor.
    #[serde(rename = "frequencyRangeHz")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_range_hz: Option<Vec<FrequencyRange>>,
    /// Bandwidth ranges that are available for this sensor.
    #[serde(rename = "bandwidthRangeHz")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth_range_hz: Option<Vec<BandwidthRange>>,
}
