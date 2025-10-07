pub use crate::prelude::*;

/// Reference frame, assumed to be Earth-centered
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OrbitMeanElementsMetadataRefFrame {
    #[serde(rename = "ECI_REFERENCE_FRAME_INVALID")]
    EciReferenceFrameInvalid,
    #[serde(rename = "ECI_REFERENCE_FRAME_TEME")]
    EciReferenceFrameTeme,
}
impl fmt::Display for OrbitMeanElementsMetadataRefFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::EciReferenceFrameInvalid => "ECI_REFERENCE_FRAME_INVALID",
            Self::EciReferenceFrameTeme => "ECI_REFERENCE_FRAME_TEME",
        };
        write!(f, "{}", s)
    }
}
