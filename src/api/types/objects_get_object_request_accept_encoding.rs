pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GetObjectRequestAcceptEncoding {
    #[serde(rename = "identity")]
    Identity,
    #[serde(rename = "zstd")]
    Zstd,
}
impl fmt::Display for GetObjectRequestAcceptEncoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Identity => "identity",
            Self::Zstd => "zstd",
        };
        write!(f, "{}", s)
    }
}
