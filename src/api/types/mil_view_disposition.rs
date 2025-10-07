pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MilViewDisposition {
    #[serde(rename = "DISPOSITION_UNKNOWN")]
    DispositionUnknown,
    #[serde(rename = "DISPOSITION_FRIENDLY")]
    DispositionFriendly,
    #[serde(rename = "DISPOSITION_HOSTILE")]
    DispositionHostile,
    #[serde(rename = "DISPOSITION_SUSPICIOUS")]
    DispositionSuspicious,
    #[serde(rename = "DISPOSITION_ASSUMED_FRIENDLY")]
    DispositionAssumedFriendly,
    #[serde(rename = "DISPOSITION_NEUTRAL")]
    DispositionNeutral,
    #[serde(rename = "DISPOSITION_PENDING")]
    DispositionPending,
}
impl fmt::Display for MilViewDisposition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::DispositionUnknown => "DISPOSITION_UNKNOWN",
            Self::DispositionFriendly => "DISPOSITION_FRIENDLY",
            Self::DispositionHostile => "DISPOSITION_HOSTILE",
            Self::DispositionSuspicious => "DISPOSITION_SUSPICIOUS",
            Self::DispositionAssumedFriendly => "DISPOSITION_ASSUMED_FRIENDLY",
            Self::DispositionNeutral => "DISPOSITION_NEUTRAL",
            Self::DispositionPending => "DISPOSITION_PENDING",
        };
        write!(f, "{}", s)
    }
}
