pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PrimaryCorrelation {
    /// The secondary entity IDs part of this correlation.
    #[serde(rename = "secondaryEntityIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_entity_ids: Option<Vec<String>>,
}
