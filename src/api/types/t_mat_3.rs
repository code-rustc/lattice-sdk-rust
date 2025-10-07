pub use crate::prelude::*;

/// Symmetric 3d matrix only representing the upper right triangle.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TMat3 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mxx: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mxy: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mxz: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub myy: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub myz: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mzz: Option<f64>,
}
