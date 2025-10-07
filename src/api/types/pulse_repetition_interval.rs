pub use crate::prelude::*;

/// A component that describe the length in time between two pulses
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PulseRepetitionInterval {
    #[serde(rename = "pulseRepetitionIntervalS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pulse_repetition_interval_s: Option<Measurement>,
}
