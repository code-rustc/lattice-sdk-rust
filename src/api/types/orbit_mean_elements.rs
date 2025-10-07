pub use crate::prelude::*;

/// Orbit Mean Elements data, analogous to the Orbit Mean Elements Message in CCSDS 502.0-B-3
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrbitMeanElements {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<OrbitMeanElementsMetadata>,
    #[serde(rename = "meanKeplerianElements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mean_keplerian_elements: Option<MeanKeplerianElements>,
    #[serde(rename = "tleParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tle_parameters: Option<TleParameters>,
}
