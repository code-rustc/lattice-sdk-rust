pub use crate::prelude::*;

/// The template used when creating this entity. Specifies minimum required components.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OntologyTemplate {
    #[serde(rename = "TEMPLATE_INVALID")]
    TemplateInvalid,
    #[serde(rename = "TEMPLATE_TRACK")]
    TemplateTrack,
    #[serde(rename = "TEMPLATE_SENSOR_POINT_OF_INTEREST")]
    TemplateSensorPointOfInterest,
    #[serde(rename = "TEMPLATE_ASSET")]
    TemplateAsset,
    #[serde(rename = "TEMPLATE_GEO")]
    TemplateGeo,
    #[serde(rename = "TEMPLATE_SIGNAL_OF_INTEREST")]
    TemplateSignalOfInterest,
}
impl fmt::Display for OntologyTemplate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::TemplateInvalid => "TEMPLATE_INVALID",
            Self::TemplateTrack => "TEMPLATE_TRACK",
            Self::TemplateSensorPointOfInterest => "TEMPLATE_SENSOR_POINT_OF_INTEREST",
            Self::TemplateAsset => "TEMPLATE_ASSET",
            Self::TemplateGeo => "TEMPLATE_GEO",
            Self::TemplateSignalOfInterest => "TEMPLATE_SIGNAL_OF_INTEREST",
        };
        write!(f, "{}", s)
    }
}
