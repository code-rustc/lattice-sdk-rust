pub use crate::prelude::*;

/// A message describing the component's health status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ComponentMessage {
    /// The status associated with this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ComponentMessageStatus>,
    /// The human-readable content of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
