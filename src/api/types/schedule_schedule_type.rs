pub use crate::prelude::*;

/// The schedule type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ScheduleScheduleType {
    #[serde(rename = "SCHEDULE_TYPE_INVALID")]
    ScheduleTypeInvalid,
    #[serde(rename = "SCHEDULE_TYPE_ZONE_ENABLED")]
    ScheduleTypeZoneEnabled,
    #[serde(rename = "SCHEDULE_TYPE_ZONE_TEMP_ENABLED")]
    ScheduleTypeZoneTempEnabled,
}
impl fmt::Display for ScheduleScheduleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ScheduleTypeInvalid => "SCHEDULE_TYPE_INVALID",
            Self::ScheduleTypeZoneEnabled => "SCHEDULE_TYPE_ZONE_ENABLED",
            Self::ScheduleTypeZoneTempEnabled => "SCHEDULE_TYPE_ZONE_TEMP_ENABLED",
        };
        write!(f, "{}", s)
    }
}
