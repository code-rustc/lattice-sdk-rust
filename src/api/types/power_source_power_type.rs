pub use crate::prelude::*;

/// Used to determine the type of power source.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PowerSourcePowerType {
    #[serde(rename = "POWER_TYPE_INVALID")]
    PowerTypeInvalid,
    #[serde(rename = "POWER_TYPE_UNKNOWN")]
    PowerTypeUnknown,
    #[serde(rename = "POWER_TYPE_GAS")]
    PowerTypeGas,
    #[serde(rename = "POWER_TYPE_BATTERY")]
    PowerTypeBattery,
}
impl fmt::Display for PowerSourcePowerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PowerTypeInvalid => "POWER_TYPE_INVALID",
            Self::PowerTypeUnknown => "POWER_TYPE_UNKNOWN",
            Self::PowerTypeGas => "POWER_TYPE_GAS",
            Self::PowerTypeBattery => "POWER_TYPE_BATTERY",
        };
        write!(f, "{}", s)
    }
}
