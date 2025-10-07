pub use crate::prelude::*;

/// A GroupParent relationship is a uni-directional relationship indicating that this entity is a member of
/// the Entity Group represented by the related entity. The presence of this relationship alone determines that
/// the type of group that this entity is a member of is an Entity Group.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GroupParent {}
