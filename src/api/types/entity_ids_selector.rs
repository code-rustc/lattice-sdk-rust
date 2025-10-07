pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EntityIdsSelector {
    /// Receive tasks as an assignee for one or more of the supplied entity ids.
    #[serde(rename = "entityIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_ids: Option<Vec<String>>,
}
