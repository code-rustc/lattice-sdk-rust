pub use crate::prelude::*;

/// A component for describing frequency.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Frequency {
    /// Indicates a frequency of a signal (Hz) with its standard deviation.
    #[serde(rename = "frequencyHz")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_hz: Option<Measurement>,
}
