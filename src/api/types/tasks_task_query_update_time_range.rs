pub use crate::prelude::*;

/// If provided, only provides Tasks updated within the time range.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TaskQueryUpdateTimeRange {
    /// If provided, returns Tasks only updated after this time.
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// If provided, returns Tasks only updated before this time.
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}
