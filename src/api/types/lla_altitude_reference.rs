pub use crate::prelude::*;

/// Meaning of alt.
/// altitude in meters above either WGS84 or EGM96, use altitude_reference to
/// determine what zero means.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LlaAltitudeReference {
    #[serde(rename = "ALTITUDE_REFERENCE_INVALID")]
    AltitudeReferenceInvalid,
    #[serde(rename = "ALTITUDE_REFERENCE_HEIGHT_ABOVE_WGS84")]
    AltitudeReferenceHeightAboveWgs84,
    #[serde(rename = "ALTITUDE_REFERENCE_HEIGHT_ABOVE_EGM96")]
    AltitudeReferenceHeightAboveEgm96,
    #[serde(rename = "ALTITUDE_REFERENCE_UNKNOWN")]
    AltitudeReferenceUnknown,
    #[serde(rename = "ALTITUDE_REFERENCE_BAROMETRIC")]
    AltitudeReferenceBarometric,
    #[serde(rename = "ALTITUDE_REFERENCE_ABOVE_SEA_FLOOR")]
    AltitudeReferenceAboveSeaFloor,
    #[serde(rename = "ALTITUDE_REFERENCE_BELOW_SEA_SURFACE")]
    AltitudeReferenceBelowSeaSurface,
}
impl fmt::Display for LlaAltitudeReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::AltitudeReferenceInvalid => "ALTITUDE_REFERENCE_INVALID",
            Self::AltitudeReferenceHeightAboveWgs84 => "ALTITUDE_REFERENCE_HEIGHT_ABOVE_WGS84",
            Self::AltitudeReferenceHeightAboveEgm96 => "ALTITUDE_REFERENCE_HEIGHT_ABOVE_EGM96",
            Self::AltitudeReferenceUnknown => "ALTITUDE_REFERENCE_UNKNOWN",
            Self::AltitudeReferenceBarometric => "ALTITUDE_REFERENCE_BAROMETRIC",
            Self::AltitudeReferenceAboveSeaFloor => "ALTITUDE_REFERENCE_ABOVE_SEA_FLOOR",
            Self::AltitudeReferenceBelowSeaSurface => "ALTITUDE_REFERENCE_BELOW_SEA_SURFACE",
        };
        write!(f, "{}", s)
    }
}
