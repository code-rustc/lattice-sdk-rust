pub use crate::prelude::*;

/// Ontology of the entity.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Ontology {
    /// A string that describes the entity's high-level type with natural language.
    #[serde(rename = "platformType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_type: Option<String>,
    /// A string that describes the entity's exact model or type.
    #[serde(rename = "specificType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specific_type: Option<String>,
    /// The template used when creating this entity. Specifies minimum required components.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<OntologyTemplate>,
}
