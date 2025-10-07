pub use crate::prelude::*;

/// A component that describes the scanning characteristics of a signal
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScanCharacteristics {
    #[serde(rename = "scanType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<ScanCharacteristicsScanType>,
    #[serde(rename = "scanPeriodS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_period_s: Option<f64>,
}
