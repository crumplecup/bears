use crate::DeriveFromStr;
use std::str::FromStr;

/// Area or Country Statistical Area enum representing geographic regions and countries
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Display,
    derive_more::FromStr,
    strum::EnumIter,
)]
pub enum AocSta {
    /// Africa
    Africa,
    /// Africa, Middle East, and Asia and Pacific
    AfricaMidEastAndAsiaAndPac,
    /// All countries
    AllCountries,
    /// Argentina
    Argentina,
    /// Asia and Pacific
    AsiaAndPac,
    /// Australia
    Australia,
    /// Austria
    Austria,
    /// Bahamas
    Bahamas,
    /// Bahrain
    Bahrain,
    /// Barbados
    Barbados,
    /// Belgium
    Belgium,
    /// Belize
    Belize,
    /// Bermuda
    Bermuda,
    /// Brazil
    Brazil,
    /// Brunei
    Brunei,
    /// Bulgaria
    Bulgaria,
    /// CAFTA-DR countries
    CaftaDrCountries,
    /// Canada
    Canada,
    /// Central America
    CenAm,
    /// Chile
    Chile,
    /// China
    China,
    /// Colombia
    Colombia,
    /// Costa Rica
    CostaRica,
    /// Croatia
    Croatia,
    /// Curacao
    Curacao,
    /// Cyprus
    Cyprus,
    /// Czech Republic
    CzechRep,
    /// Denmark
    Denmark,
    /// Dominican Republic
    DominicanRep,
    /// Ecuador
    Ecuador,
    /// Egypt
    Egypt,
    /// El Salvador
    ElSalvador,
    /// Estonia
    Estonia,
    /// European Union
    EU,
    /// Euro Area
    EuroArea,
    /// Europe
    Europe,
    /// Finland
    Finland,
    /// France
    France,
    /// Germany
    Germany,
    /// Greece
    Greece,
    /// Guatemala
    Guatemala,
    /// Honduras
    Honduras,
    /// Hong Kong
    HongKong,
    /// Hungary
    Hungary,
    /// India
    India,
    /// Indonesia
    Indonesia,
    /// Ireland
    Ireland,
    /// Israel
    Israel,
    /// Italy
    Italy,
    /// Japan
    Japan,
    /// Jordan
    Jordan,
    /// Kuwait
    Kuwait,
    /// Latin America and Other Western Hemisphere
    LatAmAndOthWestHem,
    /// Latvia
    Latvia,
    /// Lithuania
    Lithuania,
    /// Luxembourg
    Luxembourg,
    /// Macau
    Macau,
    /// Malaysia
    Malaysia,
    /// Malta
    Malta,
    /// Mexico
    Mexico,
    /// Middle East
    MiddleEast,
    /// Morocco
    Morocco,
    /// Netherlands
    Netherlands,
    /// New Zealand
    NewZealand,
    /// Nicaragua
    Nicaragua,
    /// Nigeria
    Nigeria,
    /// Norway
    Norway,
    /// Oman
    Oman,
    /// Africa, other (excluding 4 countries; services supplied through affiliates)
    OthAfricaExcl4,
    /// Asia and Pacific, other (excluding 16 countries; services supplied through affiliates)
    OthAsiaAndPacExcl16,
    /// Europe, other (excluding 32 countries)
    OthEuropeExcl32,
    /// Middle East, other (excluding 8 countries; services supplied through affiliates)
    OthMiddleEastExcl8,
    /// South America, other (excluding 7 countries; services supplied through affiliates)
    OthSouthAmExcl7,
    /// Other Western Hemisphere
    OthWestHem,
    /// Other Western Hemisphere, other (excluding 6 countries; services supplied through affiliates)
    OthWestHemOthExcl6,
    /// Panama
    Panama,
    /// Peru
    Peru,
    /// Philippines
    Philippines,
    /// Poland
    Poland,
    /// Portugal
    Portugal,
    /// Qatar
    Qatar,
    /// Romania
    Romania,
    /// Russia
    Russia,
    /// Saudi Arabia
    SaudiArabia,
    /// Singapore
    Singapore,
    /// Slovakia
    Slovakia,
    /// Slovenia
    Slovenia,
    /// South Africa
    SouthAfrica,
    /// South America
    SouthAm,
    /// South and Central America
    SouthAndCenAm,
    /// South Korea
    SouthKorea,
    /// Spain
    Spain,
    /// Sweden
    Sweden,
    /// Switzerland
    Switzerland,
    /// Taiwan
    Taiwan,
    /// Thailand
    Thailand,
    /// Turkey
    Turkey,
    /// United Kingdom Islands, Caribbean
    UkIslandsCarib,
    /// United Arab Emirates
    UnitedArabEm,
    /// United Kingdom
    UnitedKingdom,
    /// United States
    UnitedStates,
    /// Venezuela
    Venezuela,
    /// Vietnam
    Vietnam,
}

impl AocSta {
    /// Returns the full description of the area or country
    pub fn description(&self) -> &'static str {
        match self {
            Self::Africa => "Africa",
            Self::AfricaMidEastAndAsiaAndPac => "Africa, Middle East, and Asia and Pacific",
            Self::AllCountries => "All countries",
            Self::Argentina => "Argentina",
            Self::AsiaAndPac => "Asia and Pacific",
            Self::Australia => "Australia",
            Self::Austria => "Austria",
            Self::Bahamas => "Bahamas",
            Self::Bahrain => "Bahrain",
            Self::Barbados => "Barbados",
            Self::Belgium => "Belgium",
            Self::Belize => "Belize",
            Self::Bermuda => "Bermuda",
            Self::Brazil => "Brazil",
            Self::Brunei => "Brunei",
            Self::Bulgaria => "Bulgaria",
            Self::CaftaDrCountries => "CAFTA-DR countries",
            Self::Canada => "Canada",
            Self::CenAm => "Central America",
            Self::Chile => "Chile",
            Self::China => "China",
            Self::Colombia => "Colombia",
            Self::CostaRica => "Costa Rica",
            Self::Croatia => "Croatia",
            Self::Curacao => "Curacao",
            Self::Cyprus => "Cyprus",
            Self::CzechRep => "Czech Republic",
            Self::Denmark => "Denmark",
            Self::DominicanRep => "Dominican Republic",
            Self::Ecuador => "Ecuador",
            Self::Egypt => "Egypt",
            Self::ElSalvador => "El Salvador",
            Self::Estonia => "Estonia",
            Self::EU => "European Union",
            Self::EuroArea => "Euro Area",
            Self::Europe => "Europe",
            Self::Finland => "Finland",
            Self::France => "France",
            Self::Germany => "Germany",
            Self::Greece => "Greece",
            Self::Guatemala => "Guatemala",
            Self::Honduras => "Honduras",
            Self::HongKong => "Hong Kong",
            Self::Hungary => "Hungary",
            Self::India => "India",
            Self::Indonesia => "Indonesia",
            Self::Ireland => "Ireland",
            Self::Israel => "Israel",
            Self::Italy => "Italy",
            Self::Japan => "Japan",
            Self::Jordan => "Jordan",
            Self::Kuwait => "Kuwait",
            Self::LatAmAndOthWestHem => "Latin America and Other Western Hemisphere",
            Self::Latvia => "Latvia",
            Self::Lithuania => "Lithuania",
            Self::Luxembourg => "Luxembourg",
            Self::Macau => "Macau",
            Self::Malaysia => "Malaysia",
            Self::Malta => "Malta",
            Self::Mexico => "Mexico",
            Self::MiddleEast => "Middle East",
            Self::Morocco => "Morocco",
            Self::Netherlands => "Netherlands",
            Self::NewZealand => "New Zealand",
            Self::Nicaragua => "Nicaragua",
            Self::Nigeria => "Nigeria",
            Self::Norway => "Norway",
            Self::Oman => "Oman",
            Self::OthAfricaExcl4 => {
                "Africa, other (excluding 4 countries; services supplied through affiliates)"
            }
            Self::OthAsiaAndPacExcl16 => {
                "Asia and Pacific, other (excluding 16 countries; services supplied through affiliates)"
            }
            Self::OthEuropeExcl32 => "Europe, other (excluding 32 countries)",
            Self::OthMiddleEastExcl8 => {
                "Middle East, other (excluding 8 countries; services supplied through affiliates)"
            }
            Self::OthSouthAmExcl7 => {
                "South America, other (excluding 7 countries; services supplied through affiliates)"
            }
            Self::OthWestHem => "Other Western Hemisphere",
            Self::OthWestHemOthExcl6 => {
                "Other Western Hemisphere, other (excluding 6 countries; services supplied through affiliates)"
            }
            Self::Panama => "Panama",
            Self::Peru => "Peru",
            Self::Philippines => "Philippines",
            Self::Poland => "Poland",
            Self::Portugal => "Portugal",
            Self::Qatar => "Qatar",
            Self::Romania => "Romania",
            Self::Russia => "Russia",
            Self::SaudiArabia => "Saudi Arabia",
            Self::Singapore => "Singapore",
            Self::Slovakia => "Slovakia",
            Self::Slovenia => "Slovenia",
            Self::SouthAfrica => "South Africa",
            Self::SouthAm => "South America",
            Self::SouthAndCenAm => "South and Central America",
            Self::SouthKorea => "South Korea",
            Self::Spain => "Spain",
            Self::Sweden => "Sweden",
            Self::Switzerland => "Switzerland",
            Self::Taiwan => "Taiwan",
            Self::Thailand => "Thailand",
            Self::Turkey => "Turkey",
            Self::UkIslandsCarib => "United Kingdom Islands, Caribbean",
            Self::UnitedArabEm => "United Arab Emirates",
            Self::UnitedKingdom => "United Kingdom",
            Self::UnitedStates => "United States",
            Self::Venezuela => "Venezuela",
            Self::Vietnam => "Vietnam",
        }
    }

    pub fn from_key(key: &str) -> Result<Self, DeriveFromStr> {
        let mut trim = String::new();
        key.split_whitespace()
            .map(|v| trim.push_str(v))
            .for_each(drop);
        Self::from_str(&trim)
            .map_err(|e| DeriveFromStr::new(trim.to_owned(), e, line!(), file!().to_owned()))
    }
}
