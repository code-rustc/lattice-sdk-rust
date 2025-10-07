pub use crate::prelude::*;

/// A Schedule associated with this entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Schedule {
    /// expression that represents this schedule's "ON" state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows: Option<Vec<CronWindow>>,
    /// A unique identifier for this schedule.
    #[serde(rename = "scheduleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_id: Option<String>,
    /// The schedule type
    #[serde(rename = "scheduleType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_type: Option<ScheduleScheduleType>,
}
