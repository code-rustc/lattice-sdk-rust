pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SecondaryCorrelation {
    /// The primary of this correlation.
    #[serde(rename = "primaryEntityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_entity_id: Option<String>,
    /// Metadata about the correlation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<CorrelationMetadata>,
}
