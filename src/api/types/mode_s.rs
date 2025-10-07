pub use crate::prelude::*;

/// Describes the Mode S codes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ModeS {
    /// Mode S identifier which comprises of 8 alphanumeric characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The Mode S ICAO aircraft address. Expected values are between 1 and 16777214 decimal. The Mode S address is
    /// considered unique.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<i64>,
}
