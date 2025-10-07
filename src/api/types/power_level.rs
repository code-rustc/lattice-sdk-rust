pub use crate::prelude::*;

/// Represents the power level of a system.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PowerLevel {
    /// Total power capacity of the system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<f64>,
    /// Remaining power capacity of the system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining: Option<f64>,
    /// Percent of power remaining.
    #[serde(rename = "percentRemaining")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_remaining: Option<f64>,
    /// Voltage of the power source subsystem, as reported by the power source. If the source does not report this value
    /// this field will be null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voltage: Option<f64>,
    /// Current in amps of the power source subsystem, as reported by the power source. If the source does not
    /// report this value this field will be null.
    #[serde(rename = "currentAmps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_amps: Option<f64>,
    /// Estimated minutes until empty. Calculated with consumption at the moment, as reported by the power source. If the source does not
    /// report this value this field will be null.
    #[serde(rename = "runTimeToEmptyMins")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_time_to_empty_mins: Option<f64>,
    /// Fuel consumption rate in liters per second.
    #[serde(rename = "consumptionRateLPerS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumption_rate_l_per_s: Option<f64>,
}
