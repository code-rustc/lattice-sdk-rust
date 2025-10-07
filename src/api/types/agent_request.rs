pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgentRequest {
    #[serde(rename = "executeRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execute_request: Option<ExecuteRequest>,
    #[serde(rename = "cancelRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_request: Option<CancelRequest>,
    #[serde(rename = "completeRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete_request: Option<CompleteRequest>,
}
