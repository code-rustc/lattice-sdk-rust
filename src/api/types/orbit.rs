pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Orbit {
    /// Orbit Mean Elements data, analogous to the Orbit Mean Elements Message in CCSDS 502.0-B-3
    #[serde(rename = "orbitMeanElements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orbit_mean_elements: Option<OrbitMeanElements>,
}
