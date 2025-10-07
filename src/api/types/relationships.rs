pub use crate::prelude::*;

/// The relationships between this entity and other entities in the common operational picture.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Relationships {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationships: Option<Vec<Relationship>>,
}
