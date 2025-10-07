pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CronWindow {
    /// in UTC, describes when and at what cadence this window starts, in the quartz flavor of cron
    ///
    /// examples:
    /// This schedule is begins at 7:00:00am UTC everyday between Monday and Friday
    /// 0 0 7 ? * MON-FRI *
    /// This schedule begins every 5 minutes starting at 12:00:00pm UTC until 8:00:00pm UTC everyday
    /// 0 0/5 12-20 * * ? *
    /// This schedule begins at 12:00:00pm UTC on March 2nd 2023
    /// 0 0 12 2 3 ? 2023
    #[serde(rename = "cronExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cron_expression: Option<String>,
    /// describes the duration
    #[serde(rename = "durationMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_millis: Option<String>,
}
