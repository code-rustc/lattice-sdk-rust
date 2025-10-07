pub use crate::prelude::*;

/// Describes whether something is a high value target or not.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct HighValueTarget {
    /// Indicates whether the target matches any description from a high value target list.
    #[serde(rename = "isHighValueTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_high_value_target: Option<bool>,
    /// The priority associated with the target. If the target's description appears on multiple high value target lists,
    /// the priority will be a reflection of the highest priority of all of those list's target description.
    ///
    /// A lower value indicates the target is of a higher priority, with 1 being the highest possible priority. A value of
    /// 0 indicates there is no priority associated with this target.
    #[serde(rename = "targetPriority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_priority: Option<i64>,
    /// All of the high value target descriptions that the target matches against.
    #[serde(rename = "targetMatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_matches: Option<Vec<HighValueTargetMatch>>,
    /// Indicates whether the target is a 'High Payoff Target'. Targets can be one or both of high value and high payoff.
    #[serde(rename = "isHighPayoffTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_high_payoff_target: Option<bool>,
}
