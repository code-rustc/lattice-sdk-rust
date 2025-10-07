pub use crate::prelude::*;

/// A representation of a single emitter notation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmitterNotation {
    #[serde(rename = "emitterNotation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emitter_notation: Option<String>,
    /// confidence as a percentage that the emitter notation in this component is accurate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
}
