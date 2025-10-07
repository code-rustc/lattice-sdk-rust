pub use crate::prelude::*;

/// Individual payload configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Payload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<PayloadConfiguration>,
}
