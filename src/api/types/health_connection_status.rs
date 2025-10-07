pub use crate::prelude::*;

/// Status indicating whether the entity is able to communicate with Entity Manager.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum HealthConnectionStatus {
    #[serde(rename = "CONNECTION_STATUS_INVALID")]
    ConnectionStatusInvalid,
    #[serde(rename = "CONNECTION_STATUS_ONLINE")]
    ConnectionStatusOnline,
    #[serde(rename = "CONNECTION_STATUS_OFFLINE")]
    ConnectionStatusOffline,
}
impl fmt::Display for HealthConnectionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ConnectionStatusInvalid => "CONNECTION_STATUS_INVALID",
            Self::ConnectionStatusOnline => "CONNECTION_STATUS_ONLINE",
            Self::ConnectionStatusOffline => "CONNECTION_STATUS_OFFLINE",
        };
        write!(f, "{}", s)
    }
}
