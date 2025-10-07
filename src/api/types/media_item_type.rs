pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MediaItemType {
    #[serde(rename = "MEDIA_TYPE_INVALID")]
    MediaTypeInvalid,
    #[serde(rename = "MEDIA_TYPE_IMAGE")]
    MediaTypeImage,
    #[serde(rename = "MEDIA_TYPE_VIDEO")]
    MediaTypeVideo,
}
impl fmt::Display for MediaItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::MediaTypeInvalid => "MEDIA_TYPE_INVALID",
            Self::MediaTypeImage => "MEDIA_TYPE_IMAGE",
            Self::MediaTypeVideo => "MEDIA_TYPE_VIDEO",
        };
        write!(f, "{}", s)
    }
}
