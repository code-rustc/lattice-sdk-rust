pub use crate::prelude::*;

/// A target relationship is the inverse of TrackedBy; a one-way relation
/// from sensor to target, indicating track(s) currently prioritized by a robot.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ActiveTarget {}
