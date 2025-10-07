pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Dimensions {
    /// Length of the entity in meters
    #[serde(rename = "lengthM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length_m: Option<f64>,
}
