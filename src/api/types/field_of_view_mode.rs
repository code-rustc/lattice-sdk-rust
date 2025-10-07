pub use crate::prelude::*;

/// The mode that this sensor is currently in, used to display for context in the UI. Some sensors can emit multiple
/// sensor field of views with different modes, for example a radar can simultaneously search broadly and perform
/// tighter bounded tracking.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FieldOfViewMode {
    #[serde(rename = "SENSOR_MODE_INVALID")]
    SensorModeInvalid,
    #[serde(rename = "SENSOR_MODE_SEARCH")]
    SensorModeSearch,
    #[serde(rename = "SENSOR_MODE_TRACK")]
    SensorModeTrack,
    #[serde(rename = "SENSOR_MODE_WEAPON_SUPPORT")]
    SensorModeWeaponSupport,
    #[serde(rename = "SENSOR_MODE_AUTO")]
    SensorModeAuto,
    #[serde(rename = "SENSOR_MODE_MUTE")]
    SensorModeMute,
}
impl fmt::Display for FieldOfViewMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::SensorModeInvalid => "SENSOR_MODE_INVALID",
            Self::SensorModeSearch => "SENSOR_MODE_SEARCH",
            Self::SensorModeTrack => "SENSOR_MODE_TRACK",
            Self::SensorModeWeaponSupport => "SENSOR_MODE_WEAPON_SUPPORT",
            Self::SensorModeAuto => "SENSOR_MODE_AUTO",
            Self::SensorModeMute => "SENSOR_MODE_MUTE",
        };
        write!(f, "{}", s)
    }
}
