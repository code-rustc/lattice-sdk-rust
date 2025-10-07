pub use crate::prelude::*;

/// symmetric 2d matrix only representing the upper right triangle, useful for
/// covariance matrices
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TMat2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mxx: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mxy: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub myy: Option<f64>,
}
