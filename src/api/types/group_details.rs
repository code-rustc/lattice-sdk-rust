pub use crate::prelude::*;

/// Details related to grouping for this entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GroupDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<Team>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub echelon: Option<Echelon>,
}
