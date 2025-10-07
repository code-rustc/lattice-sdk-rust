pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Error {
    #[serde(rename = "errorCode")]
    pub error_code: String,
    pub message: String,
}
