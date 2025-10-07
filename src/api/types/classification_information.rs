pub use crate::prelude::*;

/// Represents all of the necessary information required to generate a summarized
/// classification marking.
///
/// > example: A summarized classification marking of "TOPSECRET//NOFORN//FISA"
/// would be defined as: { "level": 5, "caveats": [ "NOFORN, "FISA" ] }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ClassificationInformation {
    /// Classification level to be applied to the information in question.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<ClassificationInformationLevel>,
    /// Caveats that may further restrict how the information can be disseminated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caveats: Option<Vec<String>>,
}
