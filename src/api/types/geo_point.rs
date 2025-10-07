pub use crate::prelude::*;

/// A point shaped geo-entity.
/// See https://datatracker.ietf.org/doc/html/rfc7946#section-3.1.2
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeoPoint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
}
