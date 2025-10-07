pub use crate::prelude::*;

/// WGS84 position. Position includes four altitude references.
/// The data model does not currently support Mean Sea Level (MSL) references,
/// such as the Earth Gravitational Model 1996 (EGM-96) and the Earth Gravitational Model 2008 (EGM-08).
/// If the only altitude reference available to your integration is MSL, convert it to
/// Height Above Ellipsoid (HAE) and populate the altitude_hae_meters field.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Position {
    /// WGS84 geodetic latitude in decimal degrees.
    #[serde(rename = "latitudeDegrees")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude_degrees: Option<f64>,
    /// WGS84 longitude in decimal degrees.
    #[serde(rename = "longitudeDegrees")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude_degrees: Option<f64>,
    /// altitude as height above ellipsoid (WGS84) in meters. DoubleValue wrapper is used to distinguish optional from
    /// default 0.
    #[serde(rename = "altitudeHaeMeters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub altitude_hae_meters: Option<f64>,
    /// Altitude as AGL (Above Ground Level) if the upstream data source has this value set. This value represents the
    /// entity's height above the terrain. This is typically measured with a radar altimeter or by using a terrain tile
    /// set lookup. If the value is not set from the upstream, this value is not set.
    #[serde(rename = "altitudeAglMeters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub altitude_agl_meters: Option<f64>,
    /// Altitude as ASF (Above Sea Floor) if the upstream data source has this value set. If the value is not set from the upstream, this value is
    /// not set.
    #[serde(rename = "altitudeAsfMeters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub altitude_asf_meters: Option<f64>,
    /// The depth of the entity from the surface of the water through sensor measurements based on differential pressure
    /// between the interior and exterior of the vessel. If the value is not set from the upstream, this value is not set.
    #[serde(rename = "pressureDepthMeters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pressure_depth_meters: Option<f64>,
}
