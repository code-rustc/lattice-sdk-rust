pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Lla {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lon: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt: Option<f64>,
    #[serde(rename = "is2d")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_2_d: Option<bool>,
    /// Meaning of alt.
    /// altitude in meters above either WGS84 or EGM96, use altitude_reference to
    /// determine what zero means.
    #[serde(rename = "altitudeReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub altitude_reference: Option<LlaAltitudeReference>,
}
