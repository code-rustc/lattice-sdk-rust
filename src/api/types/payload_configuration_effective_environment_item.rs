pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PayloadConfigurationEffectiveEnvironmentItem {
    #[serde(rename = "ENVIRONMENT_UNKNOWN")]
    EnvironmentUnknown,
    #[serde(rename = "ENVIRONMENT_AIR")]
    EnvironmentAir,
    #[serde(rename = "ENVIRONMENT_SURFACE")]
    EnvironmentSurface,
    #[serde(rename = "ENVIRONMENT_SUB_SURFACE")]
    EnvironmentSubSurface,
    #[serde(rename = "ENVIRONMENT_LAND")]
    EnvironmentLand,
    #[serde(rename = "ENVIRONMENT_SPACE")]
    EnvironmentSpace,
}
impl fmt::Display for PayloadConfigurationEffectiveEnvironmentItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::EnvironmentUnknown => "ENVIRONMENT_UNKNOWN",
            Self::EnvironmentAir => "ENVIRONMENT_AIR",
            Self::EnvironmentSurface => "ENVIRONMENT_SURFACE",
            Self::EnvironmentSubSurface => "ENVIRONMENT_SUB_SURFACE",
            Self::EnvironmentLand => "ENVIRONMENT_LAND",
            Self::EnvironmentSpace => "ENVIRONMENT_SPACE",
        };
        write!(f, "{}", s)
    }
}
