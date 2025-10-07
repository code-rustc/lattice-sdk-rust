pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AcmDetails {
    #[serde(rename = "acmType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acm_type: Option<AcmDetailsAcmType>,
    /// Used for loosely typed associations, such as assignment to a specific fires unit.
    /// Limit to 150 characters.
    #[serde(rename = "acmDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acm_description: Option<String>,
}
