pub use crate::prelude::*;

/// A component to represent a frequency range.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FrequencyRange {
    /// Indicates the lowest measured frequency of a signal (Hz).
    #[serde(rename = "minimumFrequencyHz")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_frequency_hz: Option<Frequency>,
    /// Indicates the maximum measured frequency of a signal (Hz).
    #[serde(rename = "maximumFrequencyHz")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_frequency_hz: Option<Frequency>,
}
