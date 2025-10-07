pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CorrelationMembership {
    /// The ID of the correlation set this entity belongs to.
    #[serde(rename = "correlationSetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_set_id: Option<String>,
    /// This entity is the primary of a correlation set meaning that it serves as the representative
    /// entity of the correlation set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<PrimaryMembership>,
    /// This entity is not the primary of the correlation set. Note that there may not
    /// be a primary at all.
    #[serde(rename = "nonPrimary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_primary: Option<NonPrimaryMembership>,
    /// Additional metadata on this correlation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<CorrelationMetadata>,
}
