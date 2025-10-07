pub use crate::prelude::*;

/// A line shaped geo-entity.
/// See https://datatracker.ietf.org/doc/html/rfc7946#section-3.1.4
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeoLine {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positions: Option<Vec<Position>>,
}
