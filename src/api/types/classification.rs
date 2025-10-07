pub use crate::prelude::*;

/// A component that describes an entity's security classification levels.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Classification {
    /// The default classification information which should be assumed to apply to everything in
    /// the entity unless a specific field level classification is present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<ClassificationInformation>,
    /// The set of individual field classification information which should always precedence
    /// over the default classification information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<FieldClassificationInformation>>,
}
