pub use crate::prelude::*;

/// A component that describes a geo-entity.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GeoDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<GeoDetailsType>,
    #[serde(rename = "controlArea")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_area: Option<ControlAreaDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acm: Option<AcmDetails>,
}
