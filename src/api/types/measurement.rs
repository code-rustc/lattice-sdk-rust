pub use crate::prelude::*;

/// A component that describes some measured value with error.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Measurement {
    /// The value of the measurement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    /// Estimated one standard deviation in same unit as the value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sigma: Option<f64>,
}
