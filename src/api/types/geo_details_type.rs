pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GeoDetailsType {
    #[serde(rename = "GEO_TYPE_INVALID")]
    GeoTypeInvalid,
    #[serde(rename = "GEO_TYPE_GENERAL")]
    GeoTypeGeneral,
    #[serde(rename = "GEO_TYPE_HAZARD")]
    GeoTypeHazard,
    #[serde(rename = "GEO_TYPE_EMERGENCY")]
    GeoTypeEmergency,
    #[serde(rename = "GEO_TYPE_ENGAGEMENT_ZONE")]
    GeoTypeEngagementZone,
    #[serde(rename = "GEO_TYPE_CONTROL_AREA")]
    GeoTypeControlArea,
    #[serde(rename = "GEO_TYPE_BULLSEYE")]
    GeoTypeBullseye,
    #[serde(rename = "GEO_TYPE_ACM")]
    GeoTypeAcm,
}
impl fmt::Display for GeoDetailsType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::GeoTypeInvalid => "GEO_TYPE_INVALID",
            Self::GeoTypeGeneral => "GEO_TYPE_GENERAL",
            Self::GeoTypeHazard => "GEO_TYPE_HAZARD",
            Self::GeoTypeEmergency => "GEO_TYPE_EMERGENCY",
            Self::GeoTypeEngagementZone => "GEO_TYPE_ENGAGEMENT_ZONE",
            Self::GeoTypeControlArea => "GEO_TYPE_CONTROL_AREA",
            Self::GeoTypeBullseye => "GEO_TYPE_BULLSEYE",
            Self::GeoTypeAcm => "GEO_TYPE_ACM",
        };
        write!(f, "{}", s)
    }
}
