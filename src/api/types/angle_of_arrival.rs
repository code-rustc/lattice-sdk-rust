pub use crate::prelude::*;

/// The direction from which the signal is received
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AngleOfArrival {
    /// Origin (LLA) and attitude (relative to ENU) of a ray pointing towards the detection. The attitude represents a
    /// forward-left-up (FLU) frame where the x-axis (1, 0, 0) is pointing towards the target.
    #[serde(rename = "relativePose")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_pose: Option<Pose>,
    /// Bearing/elevation covariance matrix where bearing is defined in radians CCW+ about the z-axis from the x-axis of FLU frame
    /// and elevation is positive down from the FL/XY plane.
    /// mxx = bearing variance in rad^2
    /// mxy = bearing/elevation covariance in rad^2
    /// myy = elevation variance in rad^2
    #[serde(rename = "bearingElevationCovarianceRad2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bearing_elevation_covariance_rad_2: Option<TMat2>,
}
