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
pub enum AreaOrCountry {
    Africa,
    AfricaOthGdsNsaDetail,
    Algeria,
    AllCountries,
    AllOthSeas,
    AllOthThanCanada,
    Argentina,
    AsiaAndPac,
    AsiaAndPacOthNsaDetail,
    Australia,
    Austria,
    Bahrain,
    Barbados,
    Belgium,
    Bermuda,
    Brazil,
    Brunei,
    Bulgaria,
    CaftaDrCountries,
    Canada,
    CenAm,
    Chile,
    China,
    Colombia,
    CostaRica,
    Croatia,
    Curacao,
    Cyprus,
    CzechRep,
    Denmark,
    DominicanRep,
    Ecuador,
    Egypt,
    ElSalvador,
    Estonia,
    EU,
    EuroArea,
    Europe,
    EuropeOthNsaDetail,
    Finland,
    France,
    Germany,
    Greece,
    Guatemala,
    Honduras,
    HongKong,
    Hungary,
    India,
    Indonesia,
    IntOrgAndUnalloc,
    Ireland,
    Israel,
    Italy,
    Japan,
    Jordan,
    Kuwait,
    LatAmAndOthWestHem,
    Latvia,
    Lebanon,
    Lithuania,
    Luxembourg,
    Malaysia,
    Malta,
    Mexico,
    MiddleEast,
    MiddleEastOthGdsNsaDetail,
    Morocco,
    Netherlands,
    NewZealand,
    Nicaragua,
    Nigeria,
    Norway,
    Oman,
    OthAfricaExcl1,
    OthAfricaExcl3,
    OthAfricaExcl3DiOutward,
    OthAfricaIst,
    OthAsiaAndPacExcl10DiInward,
    OthAsiaAndPacExcl13DiOutward,
    OthAsiaAndPacExcl15,
    OthAsiaAndPacExcl8,
    OthAsiaAndPacIst,
    OthCenAmExcl4DiOutward,
    OthEuropeExcl17DiInward,
    OthEuropeExcl22DiOutward,
    OthEuropeExcl32,
    OthEuropeExcl7,
    OthEuropeIst,
    OthMiddleEastExc3DiOutward,
    OthMiddleEastExc5DiInward,
    OthMiddleEastExcl5,
    OthMiddleEastIst,
    OthSouthAmExcl7DiOutward,
    OthSouthAndCenAmExcl13,
    OthSouthAndCenAmExcl4,
    OthSouthAndCenAmExcl4DiInward,
    OthSouthAndCenAmIst,
    OthWestHem,
    OthWestHemOthExcl3,
    OthWestHemOthExcl4DiInward,
    OthWestHemOthExcl4DiOutward,
    OthWestHemOthIst,
    Panama,
    Peru,
    Philippines,
    Poland,
    Portugal,
    ResidualSeas,
    Romania,
    Russia,
    SaudiArabia,
    Singapore,
    Slovakia,
    Slovenia,
    SouthAfrica,
    SouthAm,
    SouthAndCenAm,
    SouthAndCenAmOthNsaDetail,
    SouthKorea,
    Spain,
    Switzerland,
    Sweden,
    Taiwan,
    Thailand,
    Turkey,
    Uae,
    UkIslandsCarib,
    UnitedKingdom,
    Venezuela,
    Vietnam,
}

impl AreaOrCountry {
    /// Format `self` for insertion into a request BTreeMap(key, value).
    /// The key is the parameter name.  The value is the parameter value.
    pub fn params(&self) -> (String, String) {
        let key = "AreaOrCountry".to_owned();
        let value = self.to_string();
        (key, value)
    }

    /// The description field associated with the variant in the BEA response.
    pub fn description(&self) -> String {
        let desc = match self {
            Self::Africa => "Africa",
            Self::AfricaOthGdsNsaDetail => {
                "Africa; other countries (those not listed in table 2.3)"
            }
            Self::Algeria => "Algeria",
            Self::AllCountries => "All countries",
            Self::AllOthSeas => {
                "All other countries (those not listed separately in tables 2.2 and 3.2)"
            }
            Self::AllOthThanCanada => "All countries other than Canada",
            Self::Argentina => "Argentina",
            Self::AsiaAndPac => "Asia and Pacific",
            Self::AsiaAndPacOthNsaDetail => {
                "Asia and Pacific; other countries (those not listed in table 2.3)"
            }
            Self::Australia => "Australia",
            Self::Austria => "Austria",
            Self::Bahrain => "Bahrain",
            Self::Barbados => "Barbados",
            Self::Belgium => "Belgium",
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
            Self::EuropeOthNsaDetail => "Europe; other countries (those not listed in table 2.3)",
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
            Self::IntOrgAndUnalloc => "International organizations and unallocated",
            Self::Ireland => "Ireland",
            Self::Israel => "Israel",
            Self::Italy => "Italy",
            Self::Japan => "Japan",
            Self::Jordan => "Jordan",
            Self::Kuwait => "Kuwait",
            Self::LatAmAndOthWestHem => "Latin America and Other Western Hemisphere",
            Self::Latvia => "Latvia",
            Self::Lebanon => "Lebanon",
            Self::Lithuania => "Lithuania",
            Self::Luxembourg => "Luxembourg",
            Self::Malaysia => "Malaysia",
            Self::Malta => "Malta",
            Self::Mexico => "Mexico",
            Self::MiddleEast => "Middle East",
            Self::MiddleEastOthGdsNsaDetail => {
                "Middle East; other countries (those not listed in table 2.3)"
            }
            Self::Morocco => "Morocco",
            Self::Netherlands => "Netherlands",
            Self::NewZealand => "New Zealand",
            Self::Nicaragua => "Nicaragua",
            Self::Nigeria => "Nigeria",
            Self::Norway => "Norway",
            Self::Oman => "Oman",
            Self::OthAfricaExcl1 => "Other Africa (excluding 1 country, South Africa)",
            Self::OthAfricaExcl3 => "Other Africa (excluding 3 countries)",
            Self::OthAfricaExcl3DiOutward => {
                "Africa, other (excluding 3 countries; outward direct investment)"
            }
            Self::OthAfricaIst => "Other Africa (excluding 3 countries)",
            Self::OthAsiaAndPacExcl10DiInward => {
                "Asia and Pacific, other (excluding 10 countries; inward direct investment)"
            }
            Self::OthAsiaAndPacExcl13DiOutward => {
                "Asia and Pacific, other (excluding 13 countries; outward direct investment)"
            }
            Self::OthAsiaAndPacExcl15 => "Other Asia and Pacific (excluding 15 countries)",
            Self::OthAsiaAndPacExcl8 => "Other Asia and Pacific (excluding 8 countries)",
            Self::OthAsiaAndPacIst => "Other Asia and Pacific (excluding 15 countries)",
            Self::OthCenAmExcl4DiOutward => {
                "Central America, other (excluding 4 countries; outward direct investment)"
            }
            Self::OthEuropeExcl17DiInward => {
                "Europe, other (excluding 17 countries; inward direct investment)"
            }
            Self::OthEuropeExcl22DiOutward => {
                "Europe, other (excluding 22 countries; outward direct investment)"
            }
            Self::OthEuropeExcl32 => "Other Europe (excluding 32 countries)",
            Self::OthEuropeExcl7 => "Other Europe (excluding 7 countries)",
            Self::OthEuropeIst => "Other Europe (excluding 32 countries)",
            Self::OthMiddleEastExc3DiOutward => {
                "Middle East, other (excluding 3 countries; outward direct investment)"
            }
            Self::OthMiddleEastExc5DiInward => {
                "Middle East, other (excluding 5 countries; inward direct investment)"
            }
            Self::OthMiddleEastExcl5 => "Other Middle East (excluding 5 countries)",
            Self::OthMiddleEastIst => "Other Middle East (excluding 5 countries)",
            Self::OthSouthAmExcl7DiOutward => {
                "South America, other (excluding 7 countries; outward direct investment)"
            }
            Self::OthSouthAndCenAmExcl13 => {
                "Other South And Central America (excluding 13 countries)"
            }
            Self::OthSouthAndCenAmExcl4 => {
                "Other South and Central America (excluding 4 countries)"
            }
            Self::OthSouthAndCenAmExcl4DiInward => {
                "South and Central America, other (excluding 4 countries; inward direct investment)"
            }
            Self::OthSouthAndCenAmIst => "Other South and Central America (excluding 13 countries)",
            Self::OthWestHem => "Other Western Hemisphere",
            Self::OthWestHemOthExcl3 => "Other Western Hemisphere, Other (excluding 3 countries)",
            Self::OthWestHemOthExcl4DiInward => {
                "Other Western Hemisphere, other (excluding 4 countries; inward direct investment)"
            }
            Self::OthWestHemOthExcl4DiOutward => {
                "Other Western Hemisphere, other (excluding 4 countries; outward direct investment)"
            }
            Self::OthWestHemOthIst => "Other Western Hemisphere, Other (excluding 3 countries)",
            Self::Panama => "Panama",
            Self::Peru => "Peru",
            Self::Philippines => "Philippines",
            Self::Poland => "Poland",
            Self::Portugal => "Portugal",
            Self::ResidualSeas => {
                "Residual between the seasonally adjusted total based on service type or commodity and the sum of the seasonally adjusted individual countries and the \"all other countries\" aggregate"
            }
            Self::Romania => "Romania",
            Self::Russia => "Russia",
            Self::SaudiArabia => "Saudi Arabia",
            Self::Singapore => "Singapore",
            Self::Slovakia => "Slovakia",
            Self::Slovenia => "Slovenia",
            Self::SouthAfrica => "South Africa",
            Self::SouthAm => "South America",
            Self::SouthAndCenAm => "South and Central America",
            Self::SouthAndCenAmOthNsaDetail => {
                "South and Central America; other countries (those not listed in table 2.3)"
            }
            Self::SouthKorea => "South Korea",
            Self::Spain => "Spain",
            Self::Sweden => "Sweden",
            Self::Switzerland => "Switzerland",
            Self::Taiwan => "Taiwan",
            Self::Thailand => "Thailand",
            Self::Turkey => "Turkey",
            Self::Uae => "United Arab Emirates",
            Self::UkIslandsCarib => "United Kingdom Islands, Caribbean",
            Self::UnitedKingdom => "United Kingdom",
            Self::Venezuela => "Venezuela",
            Self::Vietnam => "Vietnam",
        };
        desc.to_owned()
    }
}
