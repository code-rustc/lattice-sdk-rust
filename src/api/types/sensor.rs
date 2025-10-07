pub use crate::prelude::*;

/// Individual sensor configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Sensor {
    /// This generally is used to indicate a specific type at a more detailed granularity. E.g. COMInt or LWIR
    #[serde(rename = "sensorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensor_id: Option<String>,
    #[serde(rename = "operationalState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_state: Option<SensorOperationalState>,
    /// The type of sensor
    #[serde(rename = "sensorType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensor_type: Option<SensorSensorType>,
    /// A human readable description of the sensor
    #[serde(rename = "sensorDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensor_description: Option<String>,
    /// RF configuration details of the sensor
    #[serde(rename = "rfConfiguraton")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rf_configuraton: Option<RfConfiguration>,
    /// Time of the latest detection from the sensor
    #[serde(rename = "lastDetectionTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_detection_timestamp: Option<DateTime<Utc>>,
    /// Multiple fields of view for a single sensor component
    #[serde(rename = "fieldsOfView")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields_of_view: Option<Vec<FieldOfView>>,
}
