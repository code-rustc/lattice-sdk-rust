pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RouteDetails {
    /// Free form text giving the name of the entity's destination
    #[serde(rename = "destinationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_name: Option<String>,
    /// Estimated time of arrival at destination
    #[serde(rename = "estimatedArrivalTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_arrival_time: Option<DateTime<Utc>>,
}
