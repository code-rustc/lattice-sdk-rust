pub use crate::prelude::*;

/// Available for Entities that are tracked.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tracked {
    /// Quality score, 0-15, nil if none
    #[serde(rename = "trackQualityWrapper")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_quality_wrapper: Option<i64>,
    /// Sensor hits aggregation on the tracked entity.
    #[serde(rename = "sensorHits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensor_hits: Option<i64>,
    /// Estimated number of objects or units that are represented by this entity. Known as Strength in certain contexts (Link16)
    /// if UpperBound == LowerBound; (strength = LowerBound)
    /// If both UpperBound and LowerBound are defined; strength is between LowerBound and UpperBound (represented as string "Strength: 4-5")
    /// If UpperBound is defined only (LowerBound unset), Strength ≤ UpperBound
    /// If LowerBound is defined only (UpperBound unset), LowerBound ≤ Strength
    /// 0 indicates unset.
    #[serde(rename = "numberOfObjects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_objects: Option<UInt32Range>,
    /// The radar cross section (RCS) is a measure of how detectable an object is by radar. A large RCS indicates an object is more easily
    /// detected. The unit is “decibels per square meter,” or dBsm
    #[serde(rename = "radarCrossSection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_cross_section: Option<f64>,
    /// Timestamp of the latest tracking measurement for this entity.
    #[serde(rename = "lastMeasurementTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_measurement_time: Option<DateTime<Utc>>,
    /// The relative position of a track with respect to the entity that is tracking it. Used for tracks that do not yet have a 3D position.
    /// For this entity (A), being tracked by some entity (B), this LineOfBearing would express a ray from B to A.
    #[serde(rename = "lineOfBearing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_of_bearing: Option<LineOfBearing>,
}
