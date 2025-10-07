pub use crate::prelude::*;

/// Describes a Echelon group type.  Comprised of entities which are members of the
/// same unit or echelon. Ex: A group of tanks within a armored company or that same company
/// as a member of a battalion.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Echelon {
    #[serde(rename = "armyEchelon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub army_echelon: Option<EchelonArmyEchelon>,
}
