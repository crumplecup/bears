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
pub enum NaicsInputOutput {
    /// Apparel and leather and allied products
    ApparelLeatherAlliedProducts,
    /// Change in private inventories
    ChangeInPrivateInventories,
    /// CIF/FOB Adjustments on Imports
    CifFobAdjustmentsOnImports,
    /// Compensation of employees
    CompensationOfEmployees,
    /// Exports of goods and services
    ExportsGoodsServices,
    /// Farms
    Farms,
    /// Federal general government (defense)
    FederalGeneralGovernmentDefense,
    /// Federal general government (nondefense)
    FederalGeneralGovernmentNondefense,
    /// Federal government enterprises
    FederalGovernmentEnterprises,
    /// Federal national defense: Gross investment in equipment
    FederalNationalDefenseGrossInvestmentEquipment,
    /// Federal national defense: Gross investment in intellectual property products
    FederalNationalDefenseGrossInvestmentIntellectualPropertyProducts,
    /// Federal national defense: Gross investment in structures
    FederalNationalDefenseGrossInvestmentStructures,
    /// Federal nondefense: Gross investment in equipment
    FederalNondefenseGrossInvestmentEquipment,
    /// Federal nondefense: Gross investment in intellectual property products
    FederalNondefenseGrossInvestmentIntellectualPropertyProducts,
    /// Federal nondefense: Gross investment in structures
    FederalNondefenseGrossInvestmentStructures,
    /// Federal Reserve banks, credit intermediation, and related activities
    FederalReserveBanksCreditIntermediationRelatedActivities,
    /// Finance, insurance, real estate, rental, and leasing
    FinanceInsuranceRealEstateRentalLeasing,
    /// Food and beverage and tobacco products
    FoodBeverageTobaccoProducts,
    /// Forestry, fishing, and related activities
    ForestryFishingRelatedActivities,
    /// Government
    Government,
    /// Government consumption expenditures and gross investment
    GovernmentConsumptionExpendituresGrossInvestment,
    /// Gross operating surplus
    GrossOperatingSurplus,
    /// Housing
    Housing,
    /// Import duties
    ImportDuties,
    /// Imports
    Imports,
    /// Less: Other subsidies on production
    LessOtherSubsidiesOnProduction,
    /// Less: Subsidies on products
    LessSubsidiesOnProducts,
    /// Manufacturing
    Manufacturing,
    /// Miscellaneous professional, scientific, and technical services
    MiscellaneousProfessionalScientificTechnicalServices,
    /// Motor vehicles, bodies and trailers, and parts
    MotorVehiclesBodiesTrailersParts,
    /// National defense: Consumption expenditures
    NationalDefenseConsumptionExpenditures,
    /// Noncomparable imports and rest-of-the-world adjustment
    NoncomparableImportsRestOfTheWorldAdjustment,
    /// Nondefense: Consumption expenditures
    NondefenseConsumptionExpenditures,
    /// Nonresidential private fixed investment in equipment
    NonresidentialPrivateFixedInvestmentEquipment,
    /// Nonresidential private fixed investment in intellectual property products
    NonresidentialPrivateFixedInvestmentIntellectualPropertyRights,
    /// Nonresidential private fixed investment in structures
    NonresidentialPrivateFixedInvestmentStructures,
    /// Other real estate
    OtherRealEstate,
    /// Other retail
    OtherRetail,
    /// Other taxes on production
    OtherTaxesOnProduction,
    /// Other transportation equipment
    OtherTransportationEquipment,
    /// Other transportation and support activities
    OtherTransportationSupportActivities,
    /// Performing arts, spectator sports, museums, and related activities
    PerformingArtsSpectatorSportsMuseumsRelatedActivies,
    /// Personal consumption expenditures
    PersonalConsumptionExpenditures,
    /// Private fixed investment
    PrivateFixedInvestment,
    /// Professional and business services
    ProfessionalBusinessServices,
    /// Rental and leasing services and lessors of intangible assets
    RentalLeasingServicesLessorsIntangibleAssets,
    /// Residential private fixed investment
    ResidentialPrivateFixedInvestment,
    /// Retail trade
    RetailTrade,
    /// Scrap, used and secondhand goods
    ScrapUsedSecondhandGoods,
    /// State and local government consumption expenditures
    StateLocalGovernmentConsumptionExpenditures,
    /// State and local government enterprises
    StateLocalGovernmentEnterprises,
    /// State and local general government
    StateLocalGeneralGovernment,
    /// State and local: Gross investment in equipment
    StateLocalGrossInvestmentEquipment,
    /// State and local: Gross investment in intellectual property products
    StateLocalGrossInvestmentIntellectualPropertyProducts,
    /// State and local: Gross investment in structures
    StateLocalGrossInvestmentStructures,
    /// Subsidies on products
    SubsidiesOnProducts,
    /// Taxes on products and imports
    TaxesOnProductsImports,
    /// Tax on products
    TaxOnProducts,
    /// Textile mills and textile product mills
    TextileMillsTextileProductMills,
    /// Total Commodity Output
    TotalCommodityOutput,
    /// Total industry output (basic prices)
    TotalIndustryOutputBasicPrices,
    /// Total industry supply
    TotalIndustrySupply,
    /// Total Intermediate
    TotalIntermediate,
    /// Total product supply (basic prices)
    TotalProductSupplyBasicPrices,
    /// Total product supply (purchaser prices)
    TotalProductSupplyPurchaserPrices,
    /// Total tax less subsidies on products
    TotalTaxLessSubsidiesOnProducts,
    /// Total trade and transportation margins
    TotalTradeTransportationMargins,
    /// Total use of products
    TotalUseOfProducts,
    /// Trade margins
    TradeMargins,
    /// Transportation and warehousing
    TransportationWarehousing,
    /// Transport margins
    TransportMargins,
    /// Value Added (basic prices)
    ValueAddedBasicPrices,
    /// Value Added (producer prices)
    ValueAddedProducerPrices,
}

impl NaicsInputOutput {
    pub fn description(&self) -> &'static str {
        match self {
            Self::ApparelLeatherAlliedProducts => "Apparel and leather and allied products",
            Self::ChangeInPrivateInventories => "Change in private inventories",
            Self::CifFobAdjustmentsOnImports => "CIF/FOB Adjustments on Imports",
            Self::CompensationOfEmployees => "Compensation of employees",
            Self::ExportsGoodsServices => "Exports of goods and services",
            Self::Farms => "Farms",
            Self::FederalGeneralGovernmentDefense => "Federal general government (defense)",
            Self::FederalGeneralGovernmentNondefense => "Federal general government (nondefense)",
            Self::FederalGovernmentEnterprises => "Federal government enterprises",
            Self::FederalNationalDefenseGrossInvestmentEquipment => {
                "Federal national defense: Gross investment in equipment"
            }
            Self::FederalNationalDefenseGrossInvestmentIntellectualPropertyProducts => {
                "Federal national defense: Gross investment in intellectual property products"
            }
            Self::FederalNationalDefenseGrossInvestmentStructures => {
                "Federal national defense: Gross investment in structures"
            }
            Self::FederalNondefenseGrossInvestmentEquipment => {
                "Federal nondefense: Gross investment in equipment"
            }
            Self::FederalNondefenseGrossInvestmentIntellectualPropertyProducts => {
                "Federal nondefense: Gross investment in intellectual property products"
            }
            Self::FederalNondefenseGrossInvestmentStructures => {
                "Federal nondefense: Gross investment in structures"
            }
            Self::FederalReserveBanksCreditIntermediationRelatedActivities => {
                "Federal Reserve banks, credit intermediation, and related activities"
            }
            Self::FinanceInsuranceRealEstateRentalLeasing => {
                "Finance, insurance, real estate, rental, and leasing"
            }
            Self::FoodBeverageTobaccoProducts => "Food and beverage and tobacco products",
            Self::ForestryFishingRelatedActivities => "Forestry, fishing, and related activities",
            Self::Government => "Government",
            Self::GovernmentConsumptionExpendituresGrossInvestment => {
                "Government consumption expenditures and gross investment"
            }
            Self::GrossOperatingSurplus => "Gross operating surplus",
            Self::Housing => "Housing",
            Self::ImportDuties => "Import duties",
            Self::Imports => "Imports",
            Self::LessOtherSubsidiesOnProduction => "Less: Other subsidies on production",
            Self::LessSubsidiesOnProducts => "Less: Subsidies on products",
            Self::Manufacturing => "Manufacturing",
            Self::MiscellaneousProfessionalScientificTechnicalServices => {
                "Miscellaneous professional, scientific, and technical services"
            }
            Self::MotorVehiclesBodiesTrailersParts => {
                "Motor vehicles, bodies and trailers, and parts"
            }
            Self::NationalDefenseConsumptionExpenditures => {
                "National defense: Consumption expenditures"
            }
            Self::NoncomparableImportsRestOfTheWorldAdjustment => {
                "Noncomparable imports and rest-of-the-world adjustment"
            }
            Self::NondefenseConsumptionExpenditures => "Nondefense: Consumption expenditures",
            Self::NonresidentialPrivateFixedInvestmentEquipment => {
                "Nonresidential private fixed investment in equipment"
            }
            Self::NonresidentialPrivateFixedInvestmentIntellectualPropertyRights => {
                "Nonresidential private fixed investment in intellectual property products"
            }
            Self::NonresidentialPrivateFixedInvestmentStructures => {
                "Nonresidential private fixed investment in structures"
            }
            Self::OtherRealEstate => "Other real estate",
            Self::OtherRetail => "Other retail",
            Self::OtherTaxesOnProduction => "Other taxes on production",
            Self::OtherTransportationEquipment => "Other transportation equipment",
            Self::OtherTransportationSupportActivities => {
                "Other transportation and support activities"
            }
            Self::PerformingArtsSpectatorSportsMuseumsRelatedActivies => {
                "Performing arts, spectator sports, museums, and related activities"
            }
            Self::PersonalConsumptionExpenditures => "Personal consumption expenditures",
            Self::PrivateFixedInvestment => "Private fixed investment",
            Self::ProfessionalBusinessServices => "Professional and business services",
            Self::RentalLeasingServicesLessorsIntangibleAssets => {
                "Rental and leasing services and lessors of intangible assets"
            }
            Self::ResidentialPrivateFixedInvestment => "Residential private fixed investment",
            Self::RetailTrade => "Retail trade",
            Self::ScrapUsedSecondhandGoods => "Scrap, used and secondhand goods",
            Self::StateLocalGovernmentConsumptionExpenditures => {
                "State and local government consumption expenditures"
            }
            Self::StateLocalGovernmentEnterprises => "State and local government enterprises",
            Self::StateLocalGeneralGovernment => "State and local general government",
            Self::StateLocalGrossInvestmentEquipment => {
                "State and local: Gross investment in equipment"
            }
            Self::StateLocalGrossInvestmentIntellectualPropertyProducts => {
                "State and local: Gross investment in intellectual property products"
            }
            Self::StateLocalGrossInvestmentStructures => {
                "State and local: Gross investment in structures"
            }
            Self::SubsidiesOnProducts => "Subsidies on products",
            Self::TaxesOnProductsImports => "Taxes on products and imports",
            Self::TaxOnProducts => "Tax on products",
            Self::TextileMillsTextileProductMills => "Textile mills and textile product mills",
            Self::TotalCommodityOutput => "Total Commodity Output",
            Self::TotalIndustryOutputBasicPrices => "Total industry output (basic prices)",
            Self::TotalIndustrySupply => "Total industry supply",
            Self::TotalIntermediate => "Total Intermediate",
            Self::TotalProductSupplyBasicPrices => "Total product supply (basic prices)",
            Self::TotalProductSupplyPurchaserPrices => "Total product supply (purchaser prices)",
            Self::TotalTaxLessSubsidiesOnProducts => "Total tax less subsidies on products",
            Self::TotalTradeTransportationMargins => "Total trade and transportation margins",
            Self::TotalUseOfProducts => "Total use of products",
            Self::TradeMargins => "Trade margins",
            Self::TransportationWarehousing => "Transportation and warehousing",
            Self::TransportMargins => "Transport margins",
            Self::ValueAddedBasicPrices => "Value Added (basic prices)",
            Self::ValueAddedProducerPrices => "Value Added (producer prices)",
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Self::ApparelLeatherAlliedProducts => "315AL",
            Self::ChangeInPrivateInventories => "F030",
            Self::CifFobAdjustmentsOnImports => "MADJ",
            Self::CompensationOfEmployees => "V001",
            Self::ExportsGoodsServices => "F040",
            Self::Farms => "111CA",
            Self::FederalGeneralGovernmentDefense => "GFGD",
            Self::FederalGeneralGovernmentNondefense => "GFGN",
            Self::FederalGovernmentEnterprises => "GFE",
            Self::FederalNationalDefenseGrossInvestmentEquipment => "F06E",
            Self::FederalNationalDefenseGrossInvestmentIntellectualPropertyProducts => "F06N",
            Self::FederalNationalDefenseGrossInvestmentStructures => "F06S",
            Self::FederalNondefenseGrossInvestmentEquipment => "F07E",
            Self::FederalNondefenseGrossInvestmentIntellectualPropertyProducts => "F07N",
            Self::FederalNondefenseGrossInvestmentStructures => "F07S",
            Self::FederalReserveBanksCreditIntermediationRelatedActivities => "521CI",
            Self::FinanceInsuranceRealEstateRentalLeasing => "FIRE",
            Self::FoodBeverageTobaccoProducts => "311FT",
            Self::ForestryFishingRelatedActivities => "113FF",
            Self::Government => "G",
            Self::GovernmentConsumptionExpendituresGrossInvestment => "F100",
            Self::GrossOperatingSurplus => "V003",
            Self::Housing => "HS",
            Self::ImportDuties => "MDTY",
            Self::Imports => "MCIF",
            Self::LessOtherSubsidiesOnProduction => "T00OSUB",
            Self::LessSubsidiesOnProducts => "T00SUB",
            Self::Manufacturing => "31G",
            Self::MiscellaneousProfessionalScientificTechnicalServices => "5412OP",
            Self::MotorVehiclesBodiesTrailersParts => "3361MV",
            Self::NationalDefenseConsumptionExpenditures => "F06C",
            Self::NoncomparableImportsRestOfTheWorldAdjustment => "Other",
            Self::NondefenseConsumptionExpenditures => "F07C",
            Self::NonresidentialPrivateFixedInvestmentEquipment => "F02E",
            Self::NonresidentialPrivateFixedInvestmentIntellectualPropertyRights => "F02N",
            Self::NonresidentialPrivateFixedInvestmentStructures => "F02S",
            Self::OtherRealEstate => "ORE",
            Self::OtherRetail => "4A0",
            Self::OtherTaxesOnProduction => "T00OTOP",
            Self::OtherTransportationEquipment => "3364OT",
            Self::OtherTransportationSupportActivities => "487OS",
            Self::PerformingArtsSpectatorSportsMuseumsRelatedActivies => "711AS",
            Self::PersonalConsumptionExpenditures => "F010",
            Self::PrivateFixedInvestment => "F020",
            Self::ProfessionalBusinessServices => "PROF",
            Self::RentalLeasingServicesLessorsIntangibleAssets => "532RL",
            Self::ResidentialPrivateFixedInvestment => "F02R",
            Self::RetailTrade => "44RT",
            Self::ScrapUsedSecondhandGoods => "Used",
            Self::StateLocalGovernmentConsumptionExpenditures => "F10C",
            Self::StateLocalGovernmentEnterprises => "GSLE",
            Self::StateLocalGeneralGovernment => "GSLG",
            Self::StateLocalGrossInvestmentEquipment => "F10E",
            Self::StateLocalGrossInvestmentIntellectualPropertyProducts => "F10N",
            Self::StateLocalGrossInvestmentStructures => "F10S",
            Self::SubsidiesOnProducts => "SUB",
            Self::TaxesOnProductsImports => "T00TOP",
            Self::TaxOnProducts => "TOP",
            Self::TotalCommodityOutput => "T007",
            Self::TotalIndustryOutputBasicPrices => "T018",
            Self::TextileMillsTextileProductMills => "313TT",
            Self::TotalIndustrySupply => "T017",
            Self::TotalIntermediate => "T005",
            Self::TotalProductSupplyBasicPrices => "T013",
            Self::TotalProductSupplyPurchaserPrices => "T016",
            Self::TotalTaxLessSubsidiesOnProducts => "T015",
            Self::TotalTradeTransportationMargins => "T014",
            Self::TotalUseOfProducts => "T019",
            Self::TradeMargins => "Trade",
            Self::TransportationWarehousing => "48TW",
            Self::TransportMargins => "Trans",
            Self::ValueAddedBasicPrices => "VABAS",
            Self::ValueAddedProducerPrices => "VAPRO",
        }
    }

    pub fn from_code(code: &str) -> Option<Self> {
        let naics = match code {
            "111CA" => Self::Farms,
            "113FF" => Self::ForestryFishingRelatedActivities,
            "311FT" => Self::FoodBeverageTobaccoProducts,
            "313TT" => Self::TextileMillsTextileProductMills,
            "315AL" => Self::ApparelLeatherAlliedProducts,
            "31G" => Self::Manufacturing,
            "3361MV" => Self::MotorVehiclesBodiesTrailersParts,
            "3364OT" => Self::OtherTransportationEquipment,
            "44RT" => Self::RetailTrade,
            "487OS" => Self::OtherTransportationSupportActivities,
            "48TW" => Self::TransportationWarehousing,
            "4A0" => Self::OtherRetail,
            "521CI" => Self::FederalReserveBanksCreditIntermediationRelatedActivities,
            "532RL" => Self::RentalLeasingServicesLessorsIntangibleAssets,
            "5412OP" => Self::MiscellaneousProfessionalScientificTechnicalServices,
            "711AS" => Self::PerformingArtsSpectatorSportsMuseumsRelatedActivies,
            "F010" => Self::PersonalConsumptionExpenditures,
            "F020" => Self::PrivateFixedInvestment,
            "F02E" => Self::NonresidentialPrivateFixedInvestmentEquipment,
            "F02N" => Self::NonresidentialPrivateFixedInvestmentIntellectualPropertyRights,
            "F02R" => Self::ResidentialPrivateFixedInvestment,
            "F02S" => Self::NonresidentialPrivateFixedInvestmentStructures,
            "F030" => Self::ChangeInPrivateInventories,
            "F040" => Self::ExportsGoodsServices,
            "F06C" => Self::NationalDefenseConsumptionExpenditures,
            "F06E" => Self::FederalNationalDefenseGrossInvestmentEquipment,
            "F06N" => Self::FederalNationalDefenseGrossInvestmentIntellectualPropertyProducts,
            "F06S" => Self::FederalNationalDefenseGrossInvestmentStructures,
            "F07C" => Self::NondefenseConsumptionExpenditures,
            "F07E" => Self::FederalNondefenseGrossInvestmentEquipment,
            "F07N" => Self::FederalNondefenseGrossInvestmentIntellectualPropertyProducts,
            "F07S" => Self::FederalNondefenseGrossInvestmentStructures,
            "F100" => Self::GovernmentConsumptionExpendituresGrossInvestment,
            "F10C" => Self::StateLocalGovernmentConsumptionExpenditures,
            "F10E" => Self::StateLocalGrossInvestmentEquipment,
            "F10N" => Self::StateLocalGrossInvestmentIntellectualPropertyProducts,
            "F10S" => Self::StateLocalGrossInvestmentStructures,
            "FIRE" => Self::FinanceInsuranceRealEstateRentalLeasing,
            "G" => Self::Government,
            "GFE" => Self::FederalGovernmentEnterprises,
            "GFGD" => Self::FederalGeneralGovernmentDefense,
            "GFGN" => Self::FederalGeneralGovernmentNondefense,
            "GSLE" => Self::StateLocalGovernmentEnterprises,
            "GSLG" => Self::StateLocalGeneralGovernment,
            "HS" => Self::Housing,
            "MADJ" => Self::CifFobAdjustmentsOnImports,
            "MCIF" => Self::Imports,
            "MDTY" => Self::ImportDuties,
            "ORE" => Self::OtherRealEstate,
            "Other" => Self::NoncomparableImportsRestOfTheWorldAdjustment,
            "PROF" => Self::ProfessionalBusinessServices,
            "SUB" => Self::SubsidiesOnProducts,
            "T00OSUB" => Self::LessOtherSubsidiesOnProduction,
            "T00OTOP" => Self::OtherTaxesOnProduction,
            "T00SUB" => Self::LessSubsidiesOnProducts,
            "T00TOP" => Self::TaxesOnProductsImports,
            "T001" => Self::TotalIntermediate,
            "T005" => Self::TotalIntermediate,
            "T007" => Self::TotalCommodityOutput,
            "T013" => Self::TotalProductSupplyBasicPrices,
            "T014" => Self::TotalTradeTransportationMargins,
            "T015" => Self::TotalTaxLessSubsidiesOnProducts,
            "T016" => Self::TotalProductSupplyPurchaserPrices,
            "T017" => Self::TotalIndustrySupply,
            "T018" => Self::TotalIndustryOutputBasicPrices,
            "T019" => Self::TotalUseOfProducts,
            "TOP" => Self::TaxOnProducts,
            "Trade" => Self::TradeMargins,
            "Trans" => Self::TransportMargins,
            "Used" => Self::ScrapUsedSecondhandGoods,
            "V001" => Self::CompensationOfEmployees,
            "V003" => Self::GrossOperatingSurplus,
            "VABAS" => Self::ValueAddedBasicPrices,
            "VAPRO" => Self::ValueAddedProducerPrices,
            "" => Self::TotalIndustrySupply,
            _ => return None,
        };
        Some(naics)
    }
}
