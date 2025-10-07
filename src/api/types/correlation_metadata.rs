pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CorrelationMetadata {
    /// Who or what added this entity to the (de)correlation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provenance: Option<Provenance>,
    /// Indicates how the correlation will be distributed. Because a correlation is composed of
    /// multiple secondaries, each of which may have been correlated with different replication
    /// modes, the distribution of the correlation is composed of distributions of the individual
    /// entities within the correlation set.
    /// For example, if there are two secondary entities A and B correlated against a primary C,
    /// with A having been correlated globally and B having been correlated locally, then the
    /// correlation set that is distributed globally than what is known locally in the node.
    #[serde(rename = "replicationMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_mode: Option<CorrelationMetadataReplicationMode>,
    /// What type of (de)correlation was this entity added with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<CorrelationMetadataType>,
}
