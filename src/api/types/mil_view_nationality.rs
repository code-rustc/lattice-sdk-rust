pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MilViewNationality {
    #[serde(rename = "NATIONALITY_INVALID")]
    NationalityInvalid,
    #[serde(rename = "NATIONALITY_ALBANIA")]
    NationalityAlbania,
    #[serde(rename = "NATIONALITY_ALGERIA")]
    NationalityAlgeria,
    #[serde(rename = "NATIONALITY_ARGENTINA")]
    NationalityArgentina,
    #[serde(rename = "NATIONALITY_ARMENIA")]
    NationalityArmenia,
    #[serde(rename = "NATIONALITY_AUSTRALIA")]
    NationalityAustralia,
    #[serde(rename = "NATIONALITY_AUSTRIA")]
    NationalityAustria,
    #[serde(rename = "NATIONALITY_AZERBAIJAN")]
    NationalityAzerbaijan,
    #[serde(rename = "NATIONALITY_BELARUS")]
    NationalityBelarus,
    #[serde(rename = "NATIONALITY_BELGIUM")]
    NationalityBelgium,
    #[serde(rename = "NATIONALITY_BOLIVIA")]
    NationalityBolivia,
    #[serde(rename = "NATIONALITY_BOSNIA_AND_HERZEGOVINA")]
    NationalityBosniaAndHerzegovina,
    #[serde(rename = "NATIONALITY_BRAZIL")]
    NationalityBrazil,
    #[serde(rename = "NATIONALITY_BULGARIA")]
    NationalityBulgaria,
    #[serde(rename = "NATIONALITY_CAMBODIA")]
    NationalityCambodia,
    #[serde(rename = "NATIONALITY_CANADA")]
    NationalityCanada,
    #[serde(rename = "NATIONALITY_CHILE")]
    NationalityChile,
    #[serde(rename = "NATIONALITY_CHINA")]
    NationalityChina,
    #[serde(rename = "NATIONALITY_COLOMBIA")]
    NationalityColombia,
    #[serde(rename = "NATIONALITY_CROATIA")]
    NationalityCroatia,
    #[serde(rename = "NATIONALITY_CUBA")]
    NationalityCuba,
    #[serde(rename = "NATIONALITY_CYPRUS")]
    NationalityCyprus,
    #[serde(rename = "NATIONALITY_CZECH_REPUBLIC")]
    NationalityCzechRepublic,
    #[serde(rename = "NATIONALITY_DEMOCRATIC_PEOPLES_REPUBLIC_OF_KOREA")]
    NationalityDemocraticPeoplesRepublicOfKorea,
    #[serde(rename = "NATIONALITY_DENMARK")]
    NationalityDenmark,
    #[serde(rename = "NATIONALITY_DOMINICAN_REPUBLIC")]
    NationalityDominicanRepublic,
    #[serde(rename = "NATIONALITY_ECUADOR")]
    NationalityEcuador,
    #[serde(rename = "NATIONALITY_EGYPT")]
    NationalityEgypt,
    #[serde(rename = "NATIONALITY_ESTONIA")]
    NationalityEstonia,
    #[serde(rename = "NATIONALITY_ETHIOPIA")]
    NationalityEthiopia,
    #[serde(rename = "NATIONALITY_FINLAND")]
    NationalityFinland,
    #[serde(rename = "NATIONALITY_FRANCE")]
    NationalityFrance,
    #[serde(rename = "NATIONALITY_GEORGIA")]
    NationalityGeorgia,
    #[serde(rename = "NATIONALITY_GERMANY")]
    NationalityGermany,
    #[serde(rename = "NATIONALITY_GREECE")]
    NationalityGreece,
    #[serde(rename = "NATIONALITY_GUATEMALA")]
    NationalityGuatemala,
    #[serde(rename = "NATIONALITY_GUINEA")]
    NationalityGuinea,
    #[serde(rename = "NATIONALITY_HUNGARY")]
    NationalityHungary,
    #[serde(rename = "NATIONALITY_ICELAND")]
    NationalityIceland,
    #[serde(rename = "NATIONALITY_INDIA")]
    NationalityIndia,
    #[serde(rename = "NATIONALITY_INDONESIA")]
    NationalityIndonesia,
    #[serde(rename = "NATIONALITY_INTERNATIONAL_RED_CROSS")]
    NationalityInternationalRedCross,
    #[serde(rename = "NATIONALITY_IRAQ")]
    NationalityIraq,
    #[serde(rename = "NATIONALITY_IRELAND")]
    NationalityIreland,
    #[serde(rename = "NATIONALITY_ISLAMIC_REPUBLIC_OF_IRAN")]
    NationalityIslamicRepublicOfIran,
    #[serde(rename = "NATIONALITY_ISRAEL")]
    NationalityIsrael,
    #[serde(rename = "NATIONALITY_ITALY")]
    NationalityItaly,
    #[serde(rename = "NATIONALITY_JAMAICA")]
    NationalityJamaica,
    #[serde(rename = "NATIONALITY_JAPAN")]
    NationalityJapan,
    #[serde(rename = "NATIONALITY_JORDAN")]
    NationalityJordan,
    #[serde(rename = "NATIONALITY_KAZAKHSTAN")]
    NationalityKazakhstan,
    #[serde(rename = "NATIONALITY_KUWAIT")]
    NationalityKuwait,
    #[serde(rename = "NATIONALITY_KYRGHYZ_REPUBLIC")]
    NationalityKyrghyzRepublic,
    #[serde(rename = "NATIONALITY_LAO_PEOPLES_DEMOCRATIC_REPUBLIC")]
    NationalityLaoPeoplesDemocraticRepublic,
    #[serde(rename = "NATIONALITY_LATVIA")]
    NationalityLatvia,
    #[serde(rename = "NATIONALITY_LEBANON")]
    NationalityLebanon,
    #[serde(rename = "NATIONALITY_LIBERIA")]
    NationalityLiberia,
    #[serde(rename = "NATIONALITY_LITHUANIA")]
    NationalityLithuania,
    #[serde(rename = "NATIONALITY_LUXEMBOURG")]
    NationalityLuxembourg,
    #[serde(rename = "NATIONALITY_MADAGASCAR")]
    NationalityMadagascar,
    #[serde(rename = "NATIONALITY_MALAYSIA")]
    NationalityMalaysia,
    #[serde(rename = "NATIONALITY_MALTA")]
    NationalityMalta,
    #[serde(rename = "NATIONALITY_MEXICO")]
    NationalityMexico,
    #[serde(rename = "NATIONALITY_MOLDOVA")]
    NationalityMoldova,
    #[serde(rename = "NATIONALITY_MONTENEGRO")]
    NationalityMontenegro,
    #[serde(rename = "NATIONALITY_MOROCCO")]
    NationalityMorocco,
    #[serde(rename = "NATIONALITY_MYANMAR")]
    NationalityMyanmar,
    #[serde(rename = "NATIONALITY_NATO")]
    NationalityNato,
    #[serde(rename = "NATIONALITY_NETHERLANDS")]
    NationalityNetherlands,
    #[serde(rename = "NATIONALITY_NEW_ZEALAND")]
    NationalityNewZealand,
    #[serde(rename = "NATIONALITY_NICARAGUA")]
    NationalityNicaragua,
    #[serde(rename = "NATIONALITY_NIGERIA")]
    NationalityNigeria,
    #[serde(rename = "NATIONALITY_NORWAY")]
    NationalityNorway,
    #[serde(rename = "NATIONALITY_PAKISTAN")]
    NationalityPakistan,
    #[serde(rename = "NATIONALITY_PANAMA")]
    NationalityPanama,
    #[serde(rename = "NATIONALITY_PARAGUAY")]
    NationalityParaguay,
    #[serde(rename = "NATIONALITY_PERU")]
    NationalityPeru,
    #[serde(rename = "NATIONALITY_PHILIPPINES")]
    NationalityPhilippines,
    #[serde(rename = "NATIONALITY_POLAND")]
    NationalityPoland,
    #[serde(rename = "NATIONALITY_PORTUGAL")]
    NationalityPortugal,
    #[serde(rename = "NATIONALITY_REPUBLIC_OF_KOREA")]
    NationalityRepublicOfKorea,
    #[serde(rename = "NATIONALITY_ROMANIA")]
    NationalityRomania,
    #[serde(rename = "NATIONALITY_RUSSIA")]
    NationalityRussia,
    #[serde(rename = "NATIONALITY_SAUDI_ARABIA")]
    NationalitySaudiArabia,
    #[serde(rename = "NATIONALITY_SENEGAL")]
    NationalitySenegal,
    #[serde(rename = "NATIONALITY_SERBIA")]
    NationalitySerbia,
    #[serde(rename = "NATIONALITY_SINGAPORE")]
    NationalitySingapore,
    #[serde(rename = "NATIONALITY_SLOVAKIA")]
    NationalitySlovakia,
    #[serde(rename = "NATIONALITY_SLOVENIA")]
    NationalitySlovenia,
    #[serde(rename = "NATIONALITY_SOUTH_AFRICA")]
    NationalitySouthAfrica,
    #[serde(rename = "NATIONALITY_SPAIN")]
    NationalitySpain,
    #[serde(rename = "NATIONALITY_SUDAN")]
    NationalitySudan,
    #[serde(rename = "NATIONALITY_SWEDEN")]
    NationalitySweden,
    #[serde(rename = "NATIONALITY_SWITZERLAND")]
    NationalitySwitzerland,
    #[serde(rename = "NATIONALITY_SYRIAN_ARAB_REPUBLIC")]
    NationalitySyrianArabRepublic,
    #[serde(rename = "NATIONALITY_TAIWAN")]
    NationalityTaiwan,
    #[serde(rename = "NATIONALITY_TAJIKISTAN")]
    NationalityTajikistan,
    #[serde(rename = "NATIONALITY_THAILAND")]
    NationalityThailand,
    #[serde(rename = "NATIONALITY_THE_FORMER_YUGOSLAV_REPUBLIC_OF_MACEDONIA")]
    NationalityTheFormerYugoslavRepublicOfMacedonia,
    #[serde(rename = "NATIONALITY_TUNISIA")]
    NationalityTunisia,
    #[serde(rename = "NATIONALITY_TURKEY")]
    NationalityTurkey,
    #[serde(rename = "NATIONALITY_TURKMENISTAN")]
    NationalityTurkmenistan,
    #[serde(rename = "NATIONALITY_UGANDA")]
    NationalityUganda,
    #[serde(rename = "NATIONALITY_UKRAINE")]
    NationalityUkraine,
    #[serde(rename = "NATIONALITY_UNITED_KINGDOM")]
    NationalityUnitedKingdom,
    #[serde(rename = "NATIONALITY_UNITED_NATIONS")]
    NationalityUnitedNations,
    #[serde(rename = "NATIONALITY_UNITED_REPUBLIC_OF_TANZANIA")]
    NationalityUnitedRepublicOfTanzania,
    #[serde(rename = "NATIONALITY_UNITED_STATES_OF_AMERICA")]
    NationalityUnitedStatesOfAmerica,
    #[serde(rename = "NATIONALITY_URUGUAY")]
    NationalityUruguay,
    #[serde(rename = "NATIONALITY_UZBEKISTAN")]
    NationalityUzbekistan,
    #[serde(rename = "NATIONALITY_VENEZUELA")]
    NationalityVenezuela,
    #[serde(rename = "NATIONALITY_VIETNAM")]
    NationalityVietnam,
    #[serde(rename = "NATIONALITY_YEMEN")]
    NationalityYemen,
    #[serde(rename = "NATIONALITY_ZIMBABWE")]
    NationalityZimbabwe,
}
impl fmt::Display for MilViewNationality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::NationalityInvalid => "NATIONALITY_INVALID",
            Self::NationalityAlbania => "NATIONALITY_ALBANIA",
            Self::NationalityAlgeria => "NATIONALITY_ALGERIA",
            Self::NationalityArgentina => "NATIONALITY_ARGENTINA",
            Self::NationalityArmenia => "NATIONALITY_ARMENIA",
            Self::NationalityAustralia => "NATIONALITY_AUSTRALIA",
            Self::NationalityAustria => "NATIONALITY_AUSTRIA",
            Self::NationalityAzerbaijan => "NATIONALITY_AZERBAIJAN",
            Self::NationalityBelarus => "NATIONALITY_BELARUS",
            Self::NationalityBelgium => "NATIONALITY_BELGIUM",
            Self::NationalityBolivia => "NATIONALITY_BOLIVIA",
            Self::NationalityBosniaAndHerzegovina => "NATIONALITY_BOSNIA_AND_HERZEGOVINA",
            Self::NationalityBrazil => "NATIONALITY_BRAZIL",
            Self::NationalityBulgaria => "NATIONALITY_BULGARIA",
            Self::NationalityCambodia => "NATIONALITY_CAMBODIA",
            Self::NationalityCanada => "NATIONALITY_CANADA",
            Self::NationalityChile => "NATIONALITY_CHILE",
            Self::NationalityChina => "NATIONALITY_CHINA",
            Self::NationalityColombia => "NATIONALITY_COLOMBIA",
            Self::NationalityCroatia => "NATIONALITY_CROATIA",
            Self::NationalityCuba => "NATIONALITY_CUBA",
            Self::NationalityCyprus => "NATIONALITY_CYPRUS",
            Self::NationalityCzechRepublic => "NATIONALITY_CZECH_REPUBLIC",
            Self::NationalityDemocraticPeoplesRepublicOfKorea => {
                "NATIONALITY_DEMOCRATIC_PEOPLES_REPUBLIC_OF_KOREA"
            }
            Self::NationalityDenmark => "NATIONALITY_DENMARK",
            Self::NationalityDominicanRepublic => "NATIONALITY_DOMINICAN_REPUBLIC",
            Self::NationalityEcuador => "NATIONALITY_ECUADOR",
            Self::NationalityEgypt => "NATIONALITY_EGYPT",
            Self::NationalityEstonia => "NATIONALITY_ESTONIA",
            Self::NationalityEthiopia => "NATIONALITY_ETHIOPIA",
            Self::NationalityFinland => "NATIONALITY_FINLAND",
            Self::NationalityFrance => "NATIONALITY_FRANCE",
            Self::NationalityGeorgia => "NATIONALITY_GEORGIA",
            Self::NationalityGermany => "NATIONALITY_GERMANY",
            Self::NationalityGreece => "NATIONALITY_GREECE",
            Self::NationalityGuatemala => "NATIONALITY_GUATEMALA",
            Self::NationalityGuinea => "NATIONALITY_GUINEA",
            Self::NationalityHungary => "NATIONALITY_HUNGARY",
            Self::NationalityIceland => "NATIONALITY_ICELAND",
            Self::NationalityIndia => "NATIONALITY_INDIA",
            Self::NationalityIndonesia => "NATIONALITY_INDONESIA",
            Self::NationalityInternationalRedCross => "NATIONALITY_INTERNATIONAL_RED_CROSS",
            Self::NationalityIraq => "NATIONALITY_IRAQ",
            Self::NationalityIreland => "NATIONALITY_IRELAND",
            Self::NationalityIslamicRepublicOfIran => "NATIONALITY_ISLAMIC_REPUBLIC_OF_IRAN",
            Self::NationalityIsrael => "NATIONALITY_ISRAEL",
            Self::NationalityItaly => "NATIONALITY_ITALY",
            Self::NationalityJamaica => "NATIONALITY_JAMAICA",
            Self::NationalityJapan => "NATIONALITY_JAPAN",
            Self::NationalityJordan => "NATIONALITY_JORDAN",
            Self::NationalityKazakhstan => "NATIONALITY_KAZAKHSTAN",
            Self::NationalityKuwait => "NATIONALITY_KUWAIT",
            Self::NationalityKyrghyzRepublic => "NATIONALITY_KYRGHYZ_REPUBLIC",
            Self::NationalityLaoPeoplesDemocraticRepublic => {
                "NATIONALITY_LAO_PEOPLES_DEMOCRATIC_REPUBLIC"
            }
            Self::NationalityLatvia => "NATIONALITY_LATVIA",
            Self::NationalityLebanon => "NATIONALITY_LEBANON",
            Self::NationalityLiberia => "NATIONALITY_LIBERIA",
            Self::NationalityLithuania => "NATIONALITY_LITHUANIA",
            Self::NationalityLuxembourg => "NATIONALITY_LUXEMBOURG",
            Self::NationalityMadagascar => "NATIONALITY_MADAGASCAR",
            Self::NationalityMalaysia => "NATIONALITY_MALAYSIA",
            Self::NationalityMalta => "NATIONALITY_MALTA",
            Self::NationalityMexico => "NATIONALITY_MEXICO",
            Self::NationalityMoldova => "NATIONALITY_MOLDOVA",
            Self::NationalityMontenegro => "NATIONALITY_MONTENEGRO",
            Self::NationalityMorocco => "NATIONALITY_MOROCCO",
            Self::NationalityMyanmar => "NATIONALITY_MYANMAR",
            Self::NationalityNato => "NATIONALITY_NATO",
            Self::NationalityNetherlands => "NATIONALITY_NETHERLANDS",
            Self::NationalityNewZealand => "NATIONALITY_NEW_ZEALAND",
            Self::NationalityNicaragua => "NATIONALITY_NICARAGUA",
            Self::NationalityNigeria => "NATIONALITY_NIGERIA",
            Self::NationalityNorway => "NATIONALITY_NORWAY",
            Self::NationalityPakistan => "NATIONALITY_PAKISTAN",
            Self::NationalityPanama => "NATIONALITY_PANAMA",
            Self::NationalityParaguay => "NATIONALITY_PARAGUAY",
            Self::NationalityPeru => "NATIONALITY_PERU",
            Self::NationalityPhilippines => "NATIONALITY_PHILIPPINES",
            Self::NationalityPoland => "NATIONALITY_POLAND",
            Self::NationalityPortugal => "NATIONALITY_PORTUGAL",
            Self::NationalityRepublicOfKorea => "NATIONALITY_REPUBLIC_OF_KOREA",
            Self::NationalityRomania => "NATIONALITY_ROMANIA",
            Self::NationalityRussia => "NATIONALITY_RUSSIA",
            Self::NationalitySaudiArabia => "NATIONALITY_SAUDI_ARABIA",
            Self::NationalitySenegal => "NATIONALITY_SENEGAL",
            Self::NationalitySerbia => "NATIONALITY_SERBIA",
            Self::NationalitySingapore => "NATIONALITY_SINGAPORE",
            Self::NationalitySlovakia => "NATIONALITY_SLOVAKIA",
            Self::NationalitySlovenia => "NATIONALITY_SLOVENIA",
            Self::NationalitySouthAfrica => "NATIONALITY_SOUTH_AFRICA",
            Self::NationalitySpain => "NATIONALITY_SPAIN",
            Self::NationalitySudan => "NATIONALITY_SUDAN",
            Self::NationalitySweden => "NATIONALITY_SWEDEN",
            Self::NationalitySwitzerland => "NATIONALITY_SWITZERLAND",
            Self::NationalitySyrianArabRepublic => "NATIONALITY_SYRIAN_ARAB_REPUBLIC",
            Self::NationalityTaiwan => "NATIONALITY_TAIWAN",
            Self::NationalityTajikistan => "NATIONALITY_TAJIKISTAN",
            Self::NationalityThailand => "NATIONALITY_THAILAND",
            Self::NationalityTheFormerYugoslavRepublicOfMacedonia => {
                "NATIONALITY_THE_FORMER_YUGOSLAV_REPUBLIC_OF_MACEDONIA"
            }
            Self::NationalityTunisia => "NATIONALITY_TUNISIA",
            Self::NationalityTurkey => "NATIONALITY_TURKEY",
            Self::NationalityTurkmenistan => "NATIONALITY_TURKMENISTAN",
            Self::NationalityUganda => "NATIONALITY_UGANDA",
            Self::NationalityUkraine => "NATIONALITY_UKRAINE",
            Self::NationalityUnitedKingdom => "NATIONALITY_UNITED_KINGDOM",
            Self::NationalityUnitedNations => "NATIONALITY_UNITED_NATIONS",
            Self::NationalityUnitedRepublicOfTanzania => "NATIONALITY_UNITED_REPUBLIC_OF_TANZANIA",
            Self::NationalityUnitedStatesOfAmerica => "NATIONALITY_UNITED_STATES_OF_AMERICA",
            Self::NationalityUruguay => "NATIONALITY_URUGUAY",
            Self::NationalityUzbekistan => "NATIONALITY_UZBEKISTAN",
            Self::NationalityVenezuela => "NATIONALITY_VENEZUELA",
            Self::NationalityVietnam => "NATIONALITY_VIETNAM",
            Self::NationalityYemen => "NATIONALITY_YEMEN",
            Self::NationalityZimbabwe => "NATIONALITY_ZIMBABWE",
        };
        write!(f, "{}", s)
    }
}
