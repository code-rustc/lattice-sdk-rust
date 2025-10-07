pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OrbitMeanElementsMetadataMeanElementTheory {
    #[serde(rename = "MEAN_ELEMENT_THEORY_INVALID")]
    MeanElementTheoryInvalid,
    #[serde(rename = "MEAN_ELEMENT_THEORY_SGP4")]
    MeanElementTheorySgp4,
}
impl fmt::Display for OrbitMeanElementsMetadataMeanElementTheory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::MeanElementTheoryInvalid => "MEAN_ELEMENT_THEORY_INVALID",
            Self::MeanElementTheorySgp4 => "MEAN_ELEMENT_THEORY_SGP4",
        };
        write!(f, "{}", s)
    }
}
