pub use crate::prelude::*;

/// A GroupChild relationship is a uni-directional relationship indicating that (1) this entity
/// represents an Entity Group and (2) the related entity is a child member of this group. The presence of this
/// relationship alone determines that the type of group is an Entity Group.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GroupChild {}
