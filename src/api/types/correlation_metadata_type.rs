pub use crate::prelude::*;

/// What type of (de)correlation was this entity added with.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CorrelationMetadataType {
    #[serde(rename = "CORRELATION_TYPE_INVALID")]
    CorrelationTypeInvalid,
    #[serde(rename = "CORRELATION_TYPE_MANUAL")]
    CorrelationTypeManual,
    #[serde(rename = "CORRELATION_TYPE_AUTOMATED")]
    CorrelationTypeAutomated,
}
impl fmt::Display for CorrelationMetadataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::CorrelationTypeInvalid => "CORRELATION_TYPE_INVALID",
            Self::CorrelationTypeManual => "CORRELATION_TYPE_MANUAL",
            Self::CorrelationTypeAutomated => "CORRELATION_TYPE_AUTOMATED",
        };
        write!(f, "{}", s)
    }
}
