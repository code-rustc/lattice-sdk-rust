pub use crate::prelude::*;

/// A component that describes an entity's signal characteristics.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Signal {
    #[serde(rename = "frequencyCenter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_center: Option<Frequency>,
    #[serde(rename = "frequencyRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_range: Option<FrequencyRange>,
    /// Indicates the bandwidth of a signal (Hz).
    #[serde(rename = "bandwidthHz")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth_hz: Option<f64>,
    /// Indicates the signal to noise (SNR) of this signal.
    #[serde(rename = "signalToNoiseRatio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal_to_noise_ratio: Option<f64>,
    #[serde(rename = "lineOfBearing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_of_bearing: Option<LineOfBearing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed: Option<Fixed>,
    /// Emitter notations associated with this entity.
    #[serde(rename = "emitterNotations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emitter_notations: Option<Vec<EmitterNotation>>,
    /// length in time of a single pulse
    #[serde(rename = "pulseWidthS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pulse_width_s: Option<f64>,
    /// length in time between the start of two pulses
    #[serde(rename = "pulseRepetitionInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pulse_repetition_interval: Option<PulseRepetitionInterval>,
    /// describes how a signal is observing the environment
    #[serde(rename = "scanCharacteristics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_characteristics: Option<ScanCharacteristics>,
}
