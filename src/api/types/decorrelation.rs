pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Decorrelation {
    /// This will be specified if this entity was decorrelated against all other entities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<DecorrelatedAll>,
    /// A list of decorrelated entities that have been explicitly decorrelated against this entity
    /// which prevents lower precedence correlations from overriding it in the future.
    /// For example, if an operator in the UI decorrelated tracks A and B, any automated
    /// correlators would be unable to correlate them since manual decorrelations have
    /// higher precedence than automatic ones. Precedence is determined by both correlation
    /// type and replication mode.
    #[serde(rename = "decorrelatedEntities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decorrelated_entities: Option<Vec<DecorrelatedSingle>>,
}
