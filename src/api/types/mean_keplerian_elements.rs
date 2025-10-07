pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MeanKeplerianElements {
    /// UTC time of validity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epoch: Option<DateTime<Utc>>,
    /// Preferred: semi major axis in kilometers
    #[serde(rename = "semiMajorAxisKm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semi_major_axis_km: Option<f64>,
    /// If using SGP/SGP4, provide the Keplerian Mean Motion in revolutions per day
    #[serde(rename = "meanMotion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mean_motion: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eccentricity: Option<f64>,
    /// Angle of inclination in deg
    #[serde(rename = "inclinationDeg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inclination_deg: Option<f64>,
    /// Right ascension of the ascending node in deg
    #[serde(rename = "raOfAscNodeDeg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ra_of_asc_node_deg: Option<f64>,
    /// Argument of pericenter in deg
    #[serde(rename = "argOfPericenterDeg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arg_of_pericenter_deg: Option<f64>,
    /// Mean anomaly in deg
    #[serde(rename = "meanAnomalyDeg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mean_anomaly_deg: Option<f64>,
    /// Optional: gravitational coefficient (Gravitational Constant x central mass) in kg^3 / s^2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gm: Option<f64>,
}
