pub use crate::prelude::*;

/// A Principal is an entity that has authority over this Task.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Principal {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<System>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<Team>,
    /// The Principal _this_ Principal is acting on behalf of.
    ///
    /// Likely only populated once in the nesting (i.e. the "on_behalf_of" Principal would not have another "on_behalf_of" in most cases).
    #[serde(rename = "onBehalfOf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<Box<Principal>>,
}
