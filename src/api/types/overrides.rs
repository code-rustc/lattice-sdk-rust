pub use crate::prelude::*;

/// Metadata about entity overrides present.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Overrides {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#override: Option<Vec<Override>>,
}
