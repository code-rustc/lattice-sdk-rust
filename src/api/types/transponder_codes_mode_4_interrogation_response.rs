pub use crate::prelude::*;

/// The validity of the response from the Mode 4 interrogation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TransponderCodesMode4InterrogationResponse {
    #[serde(rename = "INTERROGATION_RESPONSE_INVALID")]
    InterrogationResponseInvalid,
    #[serde(rename = "INTERROGATION_RESPONSE_CORRECT")]
    InterrogationResponseCorrect,
    #[serde(rename = "INTERROGATION_RESPONSE_INCORRECT")]
    InterrogationResponseIncorrect,
    #[serde(rename = "INTERROGATION_RESPONSE_NO_RESPONSE")]
    InterrogationResponseNoResponse,
}
impl fmt::Display for TransponderCodesMode4InterrogationResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::InterrogationResponseInvalid => "INTERROGATION_RESPONSE_INVALID",
            Self::InterrogationResponseCorrect => "INTERROGATION_RESPONSE_CORRECT",
            Self::InterrogationResponseIncorrect => "INTERROGATION_RESPONSE_INCORRECT",
            Self::InterrogationResponseNoResponse => "INTERROGATION_RESPONSE_NO_RESPONSE",
        };
        write!(f, "{}", s)
    }
}
