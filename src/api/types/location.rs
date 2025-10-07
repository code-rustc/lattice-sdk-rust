pub use crate::prelude::*;

/// Available for Entities that have a single or primary Location.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Location {
    /// see Position definition for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    /// Velocity in an ENU reference frame centered on the corresponding position. All units are meters per second.
    #[serde(rename = "velocityEnu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub velocity_enu: Option<Enu>,
    /// Speed is the magnitude of velocity_enu vector [sqrt(e^2 + n^2 + u^2)] when present, measured in m/s.
    #[serde(rename = "speedMps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_mps: Option<f64>,
    /// The entity's acceleration in meters/s^2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceleration: Option<Enu>,
    /// quaternion to translate from entity body frame to it's ENU frame
    #[serde(rename = "attitudeEnu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attitude_enu: Option<Quaternion>,
}
