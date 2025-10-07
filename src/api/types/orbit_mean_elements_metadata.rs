pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct OrbitMeanElementsMetadata {
    /// Creation date/time in UTC
    #[serde(rename = "creationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<DateTime<Utc>>,
    /// Creating agency or operator
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originator: Option<String>,
    /// ID that uniquely identifies a message from a given originator.
    #[serde(rename = "messageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// Reference frame, assumed to be Earth-centered
    #[serde(rename = "refFrame")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_frame: Option<OrbitMeanElementsMetadataRefFrame>,
    /// Reference frame epoch in UTC - mandatory only if not intrinsic to frame definition
    #[serde(rename = "refFrameEpoch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_frame_epoch: Option<DateTime<Utc>>,
    #[serde(rename = "meanElementTheory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mean_element_theory: Option<OrbitMeanElementsMetadataMeanElementTheory>,
}
