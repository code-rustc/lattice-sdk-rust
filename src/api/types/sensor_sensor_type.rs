pub use crate::prelude::*;

/// The type of sensor
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SensorSensorType {
    #[serde(rename = "SENSOR_TYPE_INVALID")]
    SensorTypeInvalid,
    #[serde(rename = "SENSOR_TYPE_RADAR")]
    SensorTypeRadar,
    #[serde(rename = "SENSOR_TYPE_CAMERA")]
    SensorTypeCamera,
    #[serde(rename = "SENSOR_TYPE_TRANSPONDER")]
    SensorTypeTransponder,
    #[serde(rename = "SENSOR_TYPE_RF")]
    SensorTypeRf,
    #[serde(rename = "SENSOR_TYPE_GPS")]
    SensorTypeGps,
    #[serde(rename = "SENSOR_TYPE_PTU_POS")]
    SensorTypePtuPos,
    #[serde(rename = "SENSOR_TYPE_PERIMETER")]
    SensorTypePerimeter,
    #[serde(rename = "SENSOR_TYPE_SONAR")]
    SensorTypeSonar,
}
impl fmt::Display for SensorSensorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::SensorTypeInvalid => "SENSOR_TYPE_INVALID",
            Self::SensorTypeRadar => "SENSOR_TYPE_RADAR",
            Self::SensorTypeCamera => "SENSOR_TYPE_CAMERA",
            Self::SensorTypeTransponder => "SENSOR_TYPE_TRANSPONDER",
            Self::SensorTypeRf => "SENSOR_TYPE_RF",
            Self::SensorTypeGps => "SENSOR_TYPE_GPS",
            Self::SensorTypePtuPos => "SENSOR_TYPE_PTU_POS",
            Self::SensorTypePerimeter => "SENSOR_TYPE_PERIMETER",
            Self::SensorTypeSonar => "SENSOR_TYPE_SONAR",
        };
        write!(f, "{}", s)
    }
}
