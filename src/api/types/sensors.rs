pub use crate::prelude::*;

/// List of sensors available for an entity.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Sensors {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensors: Option<Vec<Sensor>>,
}
