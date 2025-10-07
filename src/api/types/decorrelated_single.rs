pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DecorrelatedSingle {
    /// The entity that was decorrelated against.
    #[serde(rename = "entityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    /// Metadata about the decorrelation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<CorrelationMetadata>,
}
