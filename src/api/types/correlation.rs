pub use crate::prelude::*;

/// Available for Entities that are a correlated (N to 1) set of entities. This will be present on
/// each entity in the set.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Correlation {
    /// This entity is the primary of a correlation meaning that it serves as the representative
    /// entity of the correlation set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<PrimaryCorrelation>,
    /// This entity is a secondary of a correlation meaning that it will be represented by the
    /// primary of the correlation set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary: Option<SecondaryCorrelation>,
    /// If present, this entity is a part of a correlation set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub membership: Option<CorrelationMembership>,
    /// If present, this entity was explicitly decorrelated from one or more entities.
    /// An entity can be both correlated and decorrelated as long as they are disjoint sets.
    /// An example would be if a user in the UI decides that two tracks are not actually the
    /// same despite an automatic correlator having correlated them. The user would then
    /// decorrelate the two tracks and this decorrelation would be preserved preventing the
    /// correlator from re-correlating them at a later time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decorrelation: Option<Decorrelation>,
}
