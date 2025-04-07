/// Represents the North American Industry Classification System (NAICS) sectors.
///
/// Each variant corresponds to a major industry sector as defined by NAICS.
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
pub enum NaicsSector {
    /// Agriculture, Forestry, Fishing and Hunting
    AgricultureForestryFishingHunting,
    /// Mining, Quarrying, and Oil and Gas Extraction
    MiningQuarryingOilGasExtraction,
    /// Utilities
    Utilities,
    /// Construction
    Construction,
    /// Manufacturing
    Manufacturing,
    /// Wholesale Trade
    WholesaleTrade,
    /// Retail Trade
    RetailTrade,
    /// Information
    Information,
    /// Finance and Insurance
    FinanceInsurance,
    /// Real Estate and Rental and Leasing
    RealEstateRentalLeasing,
    /// Professional, Scientific, and Technical Services
    ProfessionalScientificTechnicalServices,
    /// Management of Companies and Enterprises
    ManagementOfCompaniesAndEnterprises,
    /// Administrative and Support and Waste Management and Remediation Services
    AdministrativeSupportWasteManagementRemediationServices,
    /// Educational Services
    EducationalServices,
    /// Health Care and Social Assistance
    HealthCareSocialAssistance,
    /// Arts, Entertainment, and Recreation
    ArtsEntertainmentRecreation,
    /// Accommodation and Food Services
    AccommodationFoodServices,
    /// Other Services (except Public Administration)
    OtherServicesExceptPublicAdministration,
    /// Public Administration
    PublicAdministration,
    /// Unclassified Establishments
    UnclassifiedEstablishments,
}

impl NaicsSector {
    /// Returns the official NAICS title for this sector.
    pub fn description(&self) -> &'static str {
        match self {
            Self::AgricultureForestryFishingHunting => "Agriculture, Forestry, Fishing and Hunting",
            Self::MiningQuarryingOilGasExtraction => {
                "Mining, Quarrying, and Oil and Gas Extraction"
            }
            Self::Utilities => "Utilities",
            Self::Construction => "Construction",
            Self::Manufacturing => "Manufacturing",
            Self::WholesaleTrade => "Wholesale Trade",
            Self::RetailTrade => "Retail Trade",
            Self::Information => "Information",
            Self::FinanceInsurance => "Finance and Insurance",
            Self::RealEstateRentalLeasing => "Real Estate and Rental and Leasing",
            Self::ProfessionalScientificTechnicalServices => {
                "Professional, Scientific, and Technical Services"
            }
            Self::ManagementOfCompaniesAndEnterprises => "Management of Companies and Enterprises",
            Self::AdministrativeSupportWasteManagementRemediationServices => {
                "Administrative and Support and Waste Management and Remediation Services"
            }
            Self::EducationalServices => "Educational Services",
            Self::HealthCareSocialAssistance => "Health Care and Social Assistance",
            Self::ArtsEntertainmentRecreation => "Arts, Entertainment, and Recreation",
            Self::AccommodationFoodServices => "Accommodation and Food Services",
            Self::OtherServicesExceptPublicAdministration => {
                "Other Services (except Public Administration)"
            }
            Self::PublicAdministration => "Public Administration",
            Self::UnclassifiedEstablishments => "Unclassified Establishments",
        }
    }

    /// Returns the NAICS code for this sector as an integer.
    ///
    /// For sectors with a range of codes (e.g., 31-33), only the first number in the range is returned.
    pub fn code(&self) -> i64 {
        match self {
            Self::AgricultureForestryFishingHunting => 11,
            Self::MiningQuarryingOilGasExtraction => 21,
            Self::Utilities => 22,
            Self::Construction => 23,
            Self::Manufacturing => 31, // Taking only first number from 31-33
            Self::WholesaleTrade => 42,
            Self::RetailTrade => 44, // Taking only first number from 44-45
            Self::Information => 51,
            Self::FinanceInsurance => 52,
            Self::RealEstateRentalLeasing => 53,
            Self::ProfessionalScientificTechnicalServices => 54,
            Self::ManagementOfCompaniesAndEnterprises => 55,
            Self::AdministrativeSupportWasteManagementRemediationServices => 56,
            Self::EducationalServices => 61,
            Self::HealthCareSocialAssistance => 62,
            Self::ArtsEntertainmentRecreation => 71,
            Self::AccommodationFoodServices => 72,
            Self::OtherServicesExceptPublicAdministration => 81,
            Self::PublicAdministration => 92,
            Self::UnclassifiedEstablishments => 99,
        }
    }

    /// Creates a NaicsSector from a string representation of a NAICS code.
    ///
    /// Returns Some(NaicsSector) if the string can be parsed to a valid NAICS sector code,
    /// or None if the string is not a valid NAICS code or cannot be parsed.
    pub fn from_key(key: &str) -> Option<Self> {
        let code = match key.parse::<i64>() {
            Ok(num) => num,
            Err(_) => return None,
        };

        let result = match code {
            11 => Self::AgricultureForestryFishingHunting,
            21 => Self::MiningQuarryingOilGasExtraction,
            22 => Self::Utilities,
            23 => Self::Construction,
            31..=33 => Self::Manufacturing,
            42 => Self::WholesaleTrade,
            44 | 45 => Self::RetailTrade,
            51 => Self::Information,
            52 => Self::FinanceInsurance,
            53 => Self::RealEstateRentalLeasing,
            54 => Self::ProfessionalScientificTechnicalServices,
            55 => Self::ManagementOfCompaniesAndEnterprises,
            56 => Self::AdministrativeSupportWasteManagementRemediationServices,
            61 => Self::EducationalServices,
            62 => Self::HealthCareSocialAssistance,
            71 => Self::ArtsEntertainmentRecreation,
            72 => Self::AccommodationFoodServices,
            81 => Self::OtherServicesExceptPublicAdministration,
            92 => Self::PublicAdministration,
            99 => Self::UnclassifiedEstablishments,
            _ => return None,
        };

        Some(result)
    }
}
