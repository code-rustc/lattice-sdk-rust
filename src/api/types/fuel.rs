pub use crate::prelude::*;

/// Fuel describes an entity's repository of fuels stores including current amount, operational requirements, and maximum authorized capacity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Fuel {
    /// unique fuel identifier
    #[serde(rename = "fuelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel_id: Option<String>,
    /// long form name of the fuel source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// timestamp the information was reported
    #[serde(rename = "reportedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reported_date: Option<DateTime<Utc>>,
    /// amount of gallons on hand
    #[serde(rename = "amountGallons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_gallons: Option<i64>,
    /// how much the asset is allowed to have available (in gallons)
    #[serde(rename = "maxAuthorizedCapacityGallons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_authorized_capacity_gallons: Option<i64>,
    /// minimum required for operations (in gallons)
    #[serde(rename = "operationalRequirementGallons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_requirement_gallons: Option<i64>,
    /// fuel in a single asset may have different levels of classification
    /// use case: fuel for a SECRET asset while diesel fuel may be UNCLASSIFIED
    #[serde(rename = "dataClassification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_classification: Option<Classification>,
    /// source of information
    #[serde(rename = "dataSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<String>,
}
