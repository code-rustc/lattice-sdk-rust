pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UInt32Range {
    #[serde(rename = "lowerBound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_bound: Option<i64>,
    #[serde(rename = "upperBound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_bound: Option<i64>,
}
