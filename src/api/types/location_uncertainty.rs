pub use crate::prelude::*;

/// Uncertainty of entity position and velocity, if available.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LocationUncertainty {
    /// Positional covariance represented by the upper triangle of the covariance matrix. It is valid to populate
    /// only the diagonal of the matrix if the full covariance matrix is unknown.
    #[serde(rename = "positionEnuCov")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_enu_cov: Option<TMat3>,
    /// Velocity covariance represented by the upper triangle of the covariance matrix. It is valid to populate
    /// only the diagonal of the matrix if the full covariance matrix is unknown.
    #[serde(rename = "velocityEnuCov")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub velocity_enu_cov: Option<TMat3>,
    /// An ellipse that describes the certainty probability and error boundary for a given geolocation.
    #[serde(rename = "positionErrorEllipse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_error_ellipse: Option<ErrorEllipse>,
}
