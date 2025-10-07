pub use crate::prelude::*;

/// Data provenance.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Provenance {
    /// Name of the integration that produced this entity
    #[serde(rename = "integrationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_name: Option<String>,
    /// Source data type of this entity. Examples: ADSB, Link16, etc.
    #[serde(rename = "dataType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    /// An ID that allows an element from a source to be uniquely identified
    #[serde(rename = "sourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// The time, according to the source system, that the data in the entity was last modified. Generally, this should
    /// be the time that the source-reported time of validity of the data in the entity. This field must be
    /// updated with every change to the entity or else Entity Manager will discard the update.
    #[serde(rename = "sourceUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_update_time: Option<DateTime<Utc>>,
    /// Description of the modification source. In the case of a user this is the email address.
    #[serde(rename = "sourceDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_description: Option<String>,
}
