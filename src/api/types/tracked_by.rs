pub use crate::prelude::*;

/// Describes the relationship between the entity being tracked ("tracked entity") and the entity that is
/// performing the tracking ("tracking entity").
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrackedBy {
    /// Sensor details of the tracking entity's sensors that were active and tracking the tracked entity. This may be
    /// a subset of the total sensors available on the tracking entity.
    #[serde(rename = "activelyTrackingSensors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actively_tracking_sensors: Option<Sensors>,
    /// Latest time that any sensor in actively_tracking_sensors detected the tracked entity.
    #[serde(rename = "lastMeasurementTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_measurement_timestamp: Option<DateTime<Utc>>,
}
