pub use crate::prelude::*;

/// A message describing any transponder codes associated with Mode 1, 2, 3, 4, 5, S interrogations.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TransponderCodes {
    /// The mode 1 code assigned to military assets.
    #[serde(rename = "mode1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode_1: Option<i64>,
    /// The Mode 2 code assigned to military assets.
    #[serde(rename = "mode2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode_2: Option<i64>,
    /// The Mode 3 code assigned by ATC to the asset.
    #[serde(rename = "mode3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode_3: Option<i64>,
    /// The validity of the response from the Mode 4 interrogation.
    #[serde(rename = "mode4InterrogationResponse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode_4_interrogation_response: Option<TransponderCodesMode4InterrogationResponse>,
    /// The Mode 5 transponder codes.
    #[serde(rename = "mode5")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode_5: Option<Mode5>,
    /// The Mode S transponder codes.
    #[serde(rename = "modeS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode_s: Option<ModeS>,
}
