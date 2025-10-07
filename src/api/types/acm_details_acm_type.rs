pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AcmDetailsAcmType {
    #[serde(rename = "ACM_DETAIL_TYPE_INVALID")]
    AcmDetailTypeInvalid,
    #[serde(rename = "ACM_DETAIL_TYPE_LANDING_ZONE")]
    AcmDetailTypeLandingZone,
}
impl fmt::Display for AcmDetailsAcmType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::AcmDetailTypeInvalid => "ACM_DETAIL_TYPE_INVALID",
            Self::AcmDetailTypeLandingZone => "ACM_DETAIL_TYPE_LANDING_ZONE",
        };
        write!(f, "{}", s)
    }
}
