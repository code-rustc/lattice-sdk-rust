pub use crate::prelude::*;

/// Allocation contains a list of agents allocated to a Task.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Allocation {
    /// Agents actively being utilized in a Task.
    #[serde(rename = "activeAgents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_agents: Option<Vec<Agent>>,
}
