pub use crate::prelude::*;

/// Contains an arbitrary serialized message along with a @type that describes the type of the serialized message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GoogleProtobufAny {
    /// The type of the serialized message.
    #[serde(rename = "@type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}
