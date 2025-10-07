pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EntityEventEventType {
    #[serde(rename = "EVENT_TYPE_INVALID")]
    EventTypeInvalid,
    #[serde(rename = "EVENT_TYPE_CREATED")]
    EventTypeCreated,
    #[serde(rename = "EVENT_TYPE_UPDATE")]
    EventTypeUpdate,
    #[serde(rename = "EVENT_TYPE_DELETED")]
    EventTypeDeleted,
    #[serde(rename = "EVENT_TYPE_PREEXISTING")]
    EventTypePreexisting,
    #[serde(rename = "EVENT_TYPE_POST_EXPIRY_OVERRIDE")]
    EventTypePostExpiryOverride,
}
impl fmt::Display for EntityEventEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::EventTypeInvalid => "EVENT_TYPE_INVALID",
            Self::EventTypeCreated => "EVENT_TYPE_CREATED",
            Self::EventTypeUpdate => "EVENT_TYPE_UPDATE",
            Self::EventTypeDeleted => "EVENT_TYPE_DELETED",
            Self::EventTypePreexisting => "EVENT_TYPE_PREEXISTING",
            Self::EventTypePostExpiryOverride => "EVENT_TYPE_POST_EXPIRY_OVERRIDE",
        };
        write!(f, "{}", s)
    }
}
