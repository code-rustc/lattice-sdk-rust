pub use crate::prelude::*;

/// Classification level to be applied to the information in question.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ClassificationInformationLevel {
    #[serde(rename = "CLASSIFICATION_LEVELS_INVALID")]
    ClassificationLevelsInvalid,
    #[serde(rename = "CLASSIFICATION_LEVELS_UNCLASSIFIED")]
    ClassificationLevelsUnclassified,
    #[serde(rename = "CLASSIFICATION_LEVELS_CONTROLLED_UNCLASSIFIED")]
    ClassificationLevelsControlledUnclassified,
    #[serde(rename = "CLASSIFICATION_LEVELS_CONFIDENTIAL")]
    ClassificationLevelsConfidential,
    #[serde(rename = "CLASSIFICATION_LEVELS_SECRET")]
    ClassificationLevelsSecret,
    #[serde(rename = "CLASSIFICATION_LEVELS_TOP_SECRET")]
    ClassificationLevelsTopSecret,
}
impl fmt::Display for ClassificationInformationLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ClassificationLevelsInvalid => "CLASSIFICATION_LEVELS_INVALID",
            Self::ClassificationLevelsUnclassified => "CLASSIFICATION_LEVELS_UNCLASSIFIED",
            Self::ClassificationLevelsControlledUnclassified => {
                "CLASSIFICATION_LEVELS_CONTROLLED_UNCLASSIFIED"
            }
            Self::ClassificationLevelsConfidential => "CLASSIFICATION_LEVELS_CONFIDENTIAL",
            Self::ClassificationLevelsSecret => "CLASSIFICATION_LEVELS_SECRET",
            Self::ClassificationLevelsTopSecret => "CLASSIFICATION_LEVELS_TOP_SECRET",
        };
        write!(f, "{}", s)
    }
}
