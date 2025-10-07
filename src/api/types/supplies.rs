pub use crate::prelude::*;

/// Represents the state of supplies associated with an entity (available but not in condition to use immediately)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Supplies {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel: Option<Vec<Fuel>>,
}
