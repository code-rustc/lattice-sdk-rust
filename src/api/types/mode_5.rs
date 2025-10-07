pub use crate::prelude::*;

/// Describes the Mode 5 transponder interrogation status and codes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Mode5 {
    /// The validity of the response from the Mode 5 interrogation.
    #[serde(rename = "mode5InterrogationResponse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode_5_interrogation_response: Option<Mode5Mode5InterrogationResponse>,
    /// The Mode 5 code assigned to military assets.
    #[serde(rename = "mode5")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode_5: Option<i64>,
    /// The Mode 5 platform identification code.
    #[serde(rename = "mode5PlatformId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode_5_platform_id: Option<i64>,
}
