pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentListener {
    #[serde(rename = "agentSelector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_selector: Option<EntityIdsSelector>,
}
