pub use crate::prelude::*;

/// Visual details associated with the display of an entity in the client.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VisualDetails {
    /// The range rings to display around an entity.
    #[serde(rename = "rangeRings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_rings: Option<RangeRings>,
}
