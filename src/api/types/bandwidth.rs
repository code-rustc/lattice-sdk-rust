pub use crate::prelude::*;

/// Describes the bandwidth of a signal
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Bandwidth {
    #[serde(rename = "bandwidthHz")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth_hz: Option<f64>,
}
