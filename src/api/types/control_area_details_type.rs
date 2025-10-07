pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ControlAreaDetailsType {
    #[serde(rename = "CONTROL_AREA_TYPE_INVALID")]
    ControlAreaTypeInvalid,
    #[serde(rename = "CONTROL_AREA_TYPE_KEEP_IN_ZONE")]
    ControlAreaTypeKeepInZone,
    #[serde(rename = "CONTROL_AREA_TYPE_KEEP_OUT_ZONE")]
    ControlAreaTypeKeepOutZone,
    #[serde(rename = "CONTROL_AREA_TYPE_DITCH_ZONE")]
    ControlAreaTypeDitchZone,
    #[serde(rename = "CONTROL_AREA_TYPE_LOITER_ZONE")]
    ControlAreaTypeLoiterZone,
}
impl fmt::Display for ControlAreaDetailsType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ControlAreaTypeInvalid => "CONTROL_AREA_TYPE_INVALID",
            Self::ControlAreaTypeKeepInZone => "CONTROL_AREA_TYPE_KEEP_IN_ZONE",
            Self::ControlAreaTypeKeepOutZone => "CONTROL_AREA_TYPE_KEEP_OUT_ZONE",
            Self::ControlAreaTypeDitchZone => "CONTROL_AREA_TYPE_DITCH_ZONE",
            Self::ControlAreaTypeLoiterZone => "CONTROL_AREA_TYPE_LOITER_ZONE",
        };
        write!(f, "{}", s)
    }
}
