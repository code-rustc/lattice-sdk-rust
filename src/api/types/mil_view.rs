pub use crate::prelude::*;

/// Provides the disposition, environment, and nationality of an Entity.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MilView {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disposition: Option<MilViewDisposition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<MilViewEnvironment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<MilViewNationality>,
}
