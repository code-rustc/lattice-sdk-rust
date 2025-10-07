pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ScanCharacteristicsScanType {
    #[serde(rename = "SCAN_TYPE_INVALID")]
    ScanTypeInvalid,
    #[serde(rename = "SCAN_TYPE_CIRCULAR")]
    ScanTypeCircular,
    #[serde(rename = "SCAN_TYPE_BIDIRECTIONAL_HORIZONTAL_SECTOR")]
    ScanTypeBidirectionalHorizontalSector,
    #[serde(rename = "SCAN_TYPE_BIDIRECTIONAL_VERTICAL_SECTOR")]
    ScanTypeBidirectionalVerticalSector,
    #[serde(rename = "SCAN_TYPE_NON_SCANNING")]
    ScanTypeNonScanning,
    #[serde(rename = "SCAN_TYPE_IRREGULAR")]
    ScanTypeIrregular,
    #[serde(rename = "SCAN_TYPE_CONICAL")]
    ScanTypeConical,
    #[serde(rename = "SCAN_TYPE_LOBE_SWITCHING")]
    ScanTypeLobeSwitching,
    #[serde(rename = "SCAN_TYPE_RASTER")]
    ScanTypeRaster,
    #[serde(rename = "SCAN_TYPE_CIRCULAR_VERTICAL_SECTOR")]
    ScanTypeCircularVerticalSector,
    #[serde(rename = "SCAN_TYPE_CIRCULAR_CONICAL")]
    ScanTypeCircularConical,
    #[serde(rename = "SCAN_TYPE_SECTOR_CONICAL")]
    ScanTypeSectorConical,
    #[serde(rename = "SCAN_TYPE_AGILE_BEAM")]
    ScanTypeAgileBeam,
    #[serde(rename = "SCAN_TYPE_UNIDIRECTIONAL_VERTICAL_SECTOR")]
    ScanTypeUnidirectionalVerticalSector,
    #[serde(rename = "SCAN_TYPE_UNIDIRECTIONAL_HORIZONTAL_SECTOR")]
    ScanTypeUnidirectionalHorizontalSector,
    #[serde(rename = "SCAN_TYPE_UNIDIRECTIONAL_SECTOR")]
    ScanTypeUnidirectionalSector,
    #[serde(rename = "SCAN_TYPE_BIDIRECTIONAL_SECTOR")]
    ScanTypeBidirectionalSector,
}
impl fmt::Display for ScanCharacteristicsScanType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ScanTypeInvalid => "SCAN_TYPE_INVALID",
            Self::ScanTypeCircular => "SCAN_TYPE_CIRCULAR",
            Self::ScanTypeBidirectionalHorizontalSector => {
                "SCAN_TYPE_BIDIRECTIONAL_HORIZONTAL_SECTOR"
            }
            Self::ScanTypeBidirectionalVerticalSector => "SCAN_TYPE_BIDIRECTIONAL_VERTICAL_SECTOR",
            Self::ScanTypeNonScanning => "SCAN_TYPE_NON_SCANNING",
            Self::ScanTypeIrregular => "SCAN_TYPE_IRREGULAR",
            Self::ScanTypeConical => "SCAN_TYPE_CONICAL",
            Self::ScanTypeLobeSwitching => "SCAN_TYPE_LOBE_SWITCHING",
            Self::ScanTypeRaster => "SCAN_TYPE_RASTER",
            Self::ScanTypeCircularVerticalSector => "SCAN_TYPE_CIRCULAR_VERTICAL_SECTOR",
            Self::ScanTypeCircularConical => "SCAN_TYPE_CIRCULAR_CONICAL",
            Self::ScanTypeSectorConical => "SCAN_TYPE_SECTOR_CONICAL",
            Self::ScanTypeAgileBeam => "SCAN_TYPE_AGILE_BEAM",
            Self::ScanTypeUnidirectionalVerticalSector => {
                "SCAN_TYPE_UNIDIRECTIONAL_VERTICAL_SECTOR"
            }
            Self::ScanTypeUnidirectionalHorizontalSector => {
                "SCAN_TYPE_UNIDIRECTIONAL_HORIZONTAL_SECTOR"
            }
            Self::ScanTypeUnidirectionalSector => "SCAN_TYPE_UNIDIRECTIONAL_SECTOR",
            Self::ScanTypeBidirectionalSector => "SCAN_TYPE_BIDIRECTIONAL_SECTOR",
        };
        write!(f, "{}", s)
    }
}
