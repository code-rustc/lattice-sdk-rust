pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AlternateIdType {
    #[serde(rename = "ALT_ID_TYPE_INVALID")]
    AltIdTypeInvalid,
    #[serde(rename = "ALT_ID_TYPE_TRACK_ID_2")]
    AltIdTypeTrackId2,
    #[serde(rename = "ALT_ID_TYPE_TRACK_ID_1")]
    AltIdTypeTrackId1,
    #[serde(rename = "ALT_ID_TYPE_SPI_ID")]
    AltIdTypeSpiId,
    #[serde(rename = "ALT_ID_TYPE_NITF_FILE_TITLE")]
    AltIdTypeNitfFileTitle,
    #[serde(rename = "ALT_ID_TYPE_TRACK_REPO_ALERT_ID")]
    AltIdTypeTrackRepoAlertId,
    #[serde(rename = "ALT_ID_TYPE_ASSET_ID")]
    AltIdTypeAssetId,
    #[serde(rename = "ALT_ID_TYPE_LINK16_TRACK_NUMBER")]
    AltIdTypeLink16TrackNumber,
    #[serde(rename = "ALT_ID_TYPE_LINK16_JU")]
    AltIdTypeLink16Ju,
    #[serde(rename = "ALT_ID_TYPE_NCCT_MESSAGE_ID")]
    AltIdTypeNcctMessageId,
    #[serde(rename = "ALT_ID_TYPE_CALLSIGN")]
    AltIdTypeCallsign,
    #[serde(rename = "ALT_ID_TYPE_MMSI_ID")]
    AltIdTypeMmsiId,
    #[serde(rename = "ALT_ID_TYPE_VMF_URN")]
    AltIdTypeVmfUrn,
    #[serde(rename = "ALT_ID_TYPE_IMO_ID")]
    AltIdTypeImoId,
    #[serde(rename = "ALT_ID_TYPE_VMF_TARGET_NUMBER")]
    AltIdTypeVmfTargetNumber,
    #[serde(rename = "ALT_ID_TYPE_SERIAL_NUMBER")]
    AltIdTypeSerialNumber,
    #[serde(rename = "ALT_ID_TYPE_REGISTRATION_ID")]
    AltIdTypeRegistrationId,
    #[serde(rename = "ALT_ID_TYPE_IBS_GID")]
    AltIdTypeIbsGid,
    #[serde(rename = "ALT_ID_TYPE_DODAAC")]
    AltIdTypeDodaac,
    #[serde(rename = "ALT_ID_TYPE_UIC")]
    AltIdTypeUic,
    #[serde(rename = "ALT_ID_TYPE_NORAD_CAT_ID")]
    AltIdTypeNoradCatId,
    #[serde(rename = "ALT_ID_TYPE_UNOOSA_NAME")]
    AltIdTypeUnoosaName,
    #[serde(rename = "ALT_ID_TYPE_UNOOSA_ID")]
    AltIdTypeUnoosaId,
}
impl fmt::Display for AlternateIdType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::AltIdTypeInvalid => "ALT_ID_TYPE_INVALID",
            Self::AltIdTypeTrackId2 => "ALT_ID_TYPE_TRACK_ID_2",
            Self::AltIdTypeTrackId1 => "ALT_ID_TYPE_TRACK_ID_1",
            Self::AltIdTypeSpiId => "ALT_ID_TYPE_SPI_ID",
            Self::AltIdTypeNitfFileTitle => "ALT_ID_TYPE_NITF_FILE_TITLE",
            Self::AltIdTypeTrackRepoAlertId => "ALT_ID_TYPE_TRACK_REPO_ALERT_ID",
            Self::AltIdTypeAssetId => "ALT_ID_TYPE_ASSET_ID",
            Self::AltIdTypeLink16TrackNumber => "ALT_ID_TYPE_LINK16_TRACK_NUMBER",
            Self::AltIdTypeLink16Ju => "ALT_ID_TYPE_LINK16_JU",
            Self::AltIdTypeNcctMessageId => "ALT_ID_TYPE_NCCT_MESSAGE_ID",
            Self::AltIdTypeCallsign => "ALT_ID_TYPE_CALLSIGN",
            Self::AltIdTypeMmsiId => "ALT_ID_TYPE_MMSI_ID",
            Self::AltIdTypeVmfUrn => "ALT_ID_TYPE_VMF_URN",
            Self::AltIdTypeImoId => "ALT_ID_TYPE_IMO_ID",
            Self::AltIdTypeVmfTargetNumber => "ALT_ID_TYPE_VMF_TARGET_NUMBER",
            Self::AltIdTypeSerialNumber => "ALT_ID_TYPE_SERIAL_NUMBER",
            Self::AltIdTypeRegistrationId => "ALT_ID_TYPE_REGISTRATION_ID",
            Self::AltIdTypeIbsGid => "ALT_ID_TYPE_IBS_GID",
            Self::AltIdTypeDodaac => "ALT_ID_TYPE_DODAAC",
            Self::AltIdTypeUic => "ALT_ID_TYPE_UIC",
            Self::AltIdTypeNoradCatId => "ALT_ID_TYPE_NORAD_CAT_ID",
            Self::AltIdTypeUnoosaName => "ALT_ID_TYPE_UNOOSA_NAME",
            Self::AltIdTypeUnoosaId => "ALT_ID_TYPE_UNOOSA_ID",
        };
        write!(f, "{}", s)
    }
}
