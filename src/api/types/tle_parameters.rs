pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TleParameters {
    /// Integer specifying TLE ephemeris type
    #[serde(rename = "ephemerisType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeris_type: Option<i64>,
    /// User-defined free-text message classification/caveats of this TLE
    #[serde(rename = "classificationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_type: Option<String>,
    /// Norad catalog number: integer up to nine digits.
    #[serde(rename = "noradCatId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub norad_cat_id: Option<i64>,
    #[serde(rename = "elementSetNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_set_no: Option<i64>,
    /// Optional: revolution number
    #[serde(rename = "revAtEpoch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rev_at_epoch: Option<i64>,
    /// Drag parameter for SGP-4 in units 1 / Earth radii
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bstar: Option<f64>,
    /// Drag parameter for SGP4-XP in units m^2 / kg
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bterm: Option<f64>,
    /// First time derivative of mean motion in rev / day^2
    #[serde(rename = "meanMotionDot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mean_motion_dot: Option<f64>,
    /// Second time derivative of mean motion in rev / day^3. For use with SGP or PPT3.
    #[serde(rename = "meanMotionDdot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mean_motion_ddot: Option<f64>,
    /// Solar radiation pressure coefficient A_gamma / m in m^2 / kg. For use with SGP4-XP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agom: Option<f64>,
}
