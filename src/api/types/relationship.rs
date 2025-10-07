pub use crate::prelude::*;

/// The relationship component indicates a relationship to another entity.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Relationship {
    /// The entity ID to which this entity is related.
    #[serde(rename = "relatedEntityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_entity_id: Option<String>,
    /// A unique identifier for this relationship. Allows removing or updating relationships.
    #[serde(rename = "relationshipId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_id: Option<String>,
    /// The relationship type
    #[serde(rename = "relationshipType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_type: Option<RelationshipType>,
}
