pub use crate::prelude::*;

/// The type of the override, defined by the stage of the entity lifecycle that the entity was in when the override
/// was requested.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OverrideType {
    #[serde(rename = "OVERRIDE_TYPE_INVALID")]
    OverrideTypeInvalid,
    #[serde(rename = "OVERRIDE_TYPE_LIVE")]
    OverrideTypeLive,
    #[serde(rename = "OVERRIDE_TYPE_POST_EXPIRY")]
    OverrideTypePostExpiry,
}
impl fmt::Display for OverrideType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::OverrideTypeInvalid => "OVERRIDE_TYPE_INVALID",
            Self::OverrideTypeLive => "OVERRIDE_TYPE_LIVE",
            Self::OverrideTypePostExpiry => "OVERRIDE_TYPE_POST_EXPIRY",
        };
        write!(f, "{}", s)
    }
}
