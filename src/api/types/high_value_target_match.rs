pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct HighValueTargetMatch {
    /// The ID of the high value target list that matches the target description.
    #[serde(rename = "highValueTargetListId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_value_target_list_id: Option<String>,
    /// The ID of the specific high value target description within a high value target list that was matched against.
    /// The ID is considered to be a globally unique identifier across all high value target IDs.
    #[serde(rename = "highValueTargetDescriptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_value_target_description_id: Option<String>,
}
