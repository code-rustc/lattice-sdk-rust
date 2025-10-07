pub use crate::prelude::*;

/// Available for any Entities with alternate ids in other systems.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Aliases {
    #[serde(rename = "alternateIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternate_ids: Option<Vec<AlternateId>>,
    /// The best available version of the entity's display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
