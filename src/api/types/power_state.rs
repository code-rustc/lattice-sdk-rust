pub use crate::prelude::*;

/// Represents the state of power sources connected to this entity.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PowerState {
    /// This is a map where the key is a unique id of the power source and the value is additional information about the
    /// power source.
    #[serde(rename = "sourceIdToState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id_to_state: Option<HashMap<String, PowerSource>>,
}
