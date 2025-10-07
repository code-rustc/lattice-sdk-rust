pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EchelonArmyEchelon {
    #[serde(rename = "ARMY_ECHELON_INVALID")]
    ArmyEchelonInvalid,
    #[serde(rename = "ARMY_ECHELON_FIRE_TEAM")]
    ArmyEchelonFireTeam,
    #[serde(rename = "ARMY_ECHELON_SQUAD")]
    ArmyEchelonSquad,
    #[serde(rename = "ARMY_ECHELON_PLATOON")]
    ArmyEchelonPlatoon,
    #[serde(rename = "ARMY_ECHELON_COMPANY")]
    ArmyEchelonCompany,
    #[serde(rename = "ARMY_ECHELON_BATTALION")]
    ArmyEchelonBattalion,
    #[serde(rename = "ARMY_ECHELON_REGIMENT")]
    ArmyEchelonRegiment,
    #[serde(rename = "ARMY_ECHELON_BRIGADE")]
    ArmyEchelonBrigade,
    #[serde(rename = "ARMY_ECHELON_DIVISION")]
    ArmyEchelonDivision,
    #[serde(rename = "ARMY_ECHELON_CORPS")]
    ArmyEchelonCorps,
    #[serde(rename = "ARMY_ECHELON_ARMY")]
    ArmyEchelonArmy,
}
impl fmt::Display for EchelonArmyEchelon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ArmyEchelonInvalid => "ARMY_ECHELON_INVALID",
            Self::ArmyEchelonFireTeam => "ARMY_ECHELON_FIRE_TEAM",
            Self::ArmyEchelonSquad => "ARMY_ECHELON_SQUAD",
            Self::ArmyEchelonPlatoon => "ARMY_ECHELON_PLATOON",
            Self::ArmyEchelonCompany => "ARMY_ECHELON_COMPANY",
            Self::ArmyEchelonBattalion => "ARMY_ECHELON_BATTALION",
            Self::ArmyEchelonRegiment => "ARMY_ECHELON_REGIMENT",
            Self::ArmyEchelonBrigade => "ARMY_ECHELON_BRIGADE",
            Self::ArmyEchelonDivision => "ARMY_ECHELON_DIVISION",
            Self::ArmyEchelonCorps => "ARMY_ECHELON_CORPS",
            Self::ArmyEchelonArmy => "ARMY_ECHELON_ARMY",
        };
        write!(f, "{}", s)
    }
}
