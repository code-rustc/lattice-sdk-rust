pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DecorrelatedAll {
    /// Metadata about the decorrelation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<CorrelationMetadata>,
}
