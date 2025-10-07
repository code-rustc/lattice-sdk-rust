pub use crate::prelude::*;

/// Any metadata associated with the replication of a Task.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Replication {
    /// Time by which this Task should be assumed to be stale.
    #[serde(rename = "staleTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stale_time: Option<DateTime<Utc>>,
}
