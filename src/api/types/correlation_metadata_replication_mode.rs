pub use crate::prelude::*;

/// Indicates how the correlation will be distributed. Because a correlation is composed of
/// multiple secondaries, each of which may have been correlated with different replication
/// modes, the distribution of the correlation is composed of distributions of the individual
/// entities within the correlation set.
/// For example, if there are two secondary entities A and B correlated against a primary C,
/// with A having been correlated globally and B having been correlated locally, then the
/// correlation set that is distributed globally than what is known locally in the node.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CorrelationMetadataReplicationMode {
    #[serde(rename = "CORRELATION_REPLICATION_MODE_INVALID")]
    CorrelationReplicationModeInvalid,
    #[serde(rename = "CORRELATION_REPLICATION_MODE_LOCAL")]
    CorrelationReplicationModeLocal,
    #[serde(rename = "CORRELATION_REPLICATION_MODE_GLOBAL")]
    CorrelationReplicationModeGlobal,
}
impl fmt::Display for CorrelationMetadataReplicationMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::CorrelationReplicationModeInvalid => "CORRELATION_REPLICATION_MODE_INVALID",
            Self::CorrelationReplicationModeLocal => "CORRELATION_REPLICATION_MODE_LOCAL",
            Self::CorrelationReplicationModeGlobal => "CORRELATION_REPLICATION_MODE_GLOBAL",
        };
        write!(f, "{}", s)
    }
}
