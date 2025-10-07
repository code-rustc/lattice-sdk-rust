pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Enu {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u: Option<f64>,
}
