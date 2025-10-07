pub use crate::prelude::*;

/// Determines the type of control area being represented by the geo-entity,
/// in which an asset can, or cannot, operate.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ControlAreaDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ControlAreaDetailsType>,
}
