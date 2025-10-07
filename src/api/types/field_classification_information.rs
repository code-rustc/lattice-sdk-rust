pub use crate::prelude::*;

/// A field specific classification information definition.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FieldClassificationInformation {
    /// Proto field path which is the string representation of a field.
    /// > example: signal.bandwidth_hz would be bandwidth_hz in the signal component
    #[serde(rename = "fieldPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_path: Option<String>,
    /// The information which makes up the field level classification marking.
    #[serde(rename = "classificationInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_information: Option<ClassificationInformation>,
}
