pub use crate::prelude::*;

/// Alert level (Warning, Caution, or Advisory).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AlertLevel {
    #[serde(rename = "ALERT_LEVEL_INVALID")]
    AlertLevelInvalid,
    #[serde(rename = "ALERT_LEVEL_ADVISORY")]
    AlertLevelAdvisory,
    #[serde(rename = "ALERT_LEVEL_CAUTION")]
    AlertLevelCaution,
    #[serde(rename = "ALERT_LEVEL_WARNING")]
    AlertLevelWarning,
}
impl fmt::Display for AlertLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::AlertLevelInvalid => "ALERT_LEVEL_INVALID",
            Self::AlertLevelAdvisory => "ALERT_LEVEL_ADVISORY",
            Self::AlertLevelCaution => "ALERT_LEVEL_CAUTION",
            Self::AlertLevelWarning => "ALERT_LEVEL_WARNING",
        };
        write!(f, "{}", s)
    }
}
