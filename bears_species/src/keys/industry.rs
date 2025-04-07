/// Industry designations from the IIP dataset.
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
pub enum IipIndustry {
    /// Accommodation
    Accommodation,
    /// Accommodation and food services
    AccommodationAndFood,
    /// Accounting, tax preparation, bookkeeping, and payroll services
    AccountingTaxPrepBookkeepingAndPayroll,
    /// Administration, support, and waste management
    AdministrationSupportAndWasteManagement,
    /// Administrative and support services
    AdministrativeAndSupport,
    /// Advertising and related services
    AdvertisingAndRelated,
    /// Agriculture, forestry, fishing, and hunting
    AgricultureForestryFishingAndHunting,
    /// Air transportation
    AirTransportation,
    /// All industries
    AllInd,
    /// All nonbank industries
    AllNonbankInd,
    /// Architectural, engineering, and related services
    ArchitecturalEngineeringAndRelated,
    /// Arts, entertainment, and recreation
    ArtsEntertainmentAndRecreation,
    /// Computer systems design and related services
    ComputerSystemsDesignAndRelated,
    /// Construction
    Construction,
    /// Educational services
    Educational,
    /// Employment services
    Employment,
    /// Finance and insurance
    FinanceAndInsurance,
    /// Depository credit intermediation (banking)
    FinanceAndInsuranceBanking,
    /// Finance
    FinanceAndInsuranceFinance,
    /// Finance, except depository institutions
    FinanceAndInsuranceFinanceExceptBanking,
    /// Insurance carriers and related activities
    FinanceAndInsuranceInsuranceCarriersAndRelated,
    /// Food services and drinking places
    FoodServicesAndDrinkingPlaces,
    /// Health care and social assistance
    HealthCareAndSocialAssistance,
    /// Information
    Info,
    /// Broadcasting (except Internet)
    InfoBroadcasting,
    /// Data processing, hosting, and related services (MOFAs); Computing infrastructure providers, data processing, web hosting, and related services (MOUSAs)
    InfoDataProcessingHostingAndRelated,
    /// Motion picture and sound recording industries
    InfoMotionPictureAndSoundRecording,
    /// Motion picture and video industries
    InfoMotionPictureAndVideo,
    /// Newspaper, periodical, book, and database publishers (MOFAs); Newspaper, periodical, book, and directory publishers (MOUSAs)
    InfoNewspaperPeriodicalBookDatabasePublishers,
    /// Other information services (MOFAs); Web search portals, libraries, archives, and other information services (MOUSAs)
    InfoOth,
    /// Other telecommunications
    InfoOthTelecommunications,
    /// Publishing industries
    InfoPublishing,
    /// Software publishers
    InfoSoftwarePublishers,
    /// Sound recording industries
    InfoSoundRecording,
    /// Telecommunications
    InfoTelecommunications,
    /// Wired telecommunications carriers
    InfoWiredTelecommunicationsCarriers,
    /// Wireless telecommunications carriers (except satellite) (MOFAs); Wired and wireless telecommunications carriers (except satellite) (MOUSAs)
    InfoWirelessTelecommunicationsCarriers,
    /// Legal services
    Legal,
    /// Management of nonbank companies and enterprises
    ManagementOfNonbankCompanies,
    /// Management, scientific, and technical consulting
    ManagementScientificAndTechnicalConsulting,
    /// Other chemicals manufacturing
    ManuAgriculturalChemicalsPaintsCoatingsAdhesivesAndOth,
    /// Agriculture, construction, and mining machinery manufacturing
    ManuAgricultureConstructionAndMiningMachinery,
    /// Audio and video equipment manufacturing
    ManuAudioAndVideoEquipment,
    /// Basic chemicals manufacturing
    ManuBasicChemicals,
    /// Beverages and tobacco products manufacturing
    ManuBeveragesAndTobacco,
    /// Chemicals manufacturing
    ManuChemicals,
    /// Other chemicals manufacturing
    ManuChemicalsOth,
    /// Communications equipment manufacturing
    ManuCommunicationsEquipment,
    /// Computers and electronic products manufacturing
    ManuComputersAndElectronic,
    /// Other computers and electronic products manufacturing
    ManuComputersAndElectronicOth,
    /// Computers and peripheral equipment manufacturing
    ManuComputersAndPeripheralEquipment,
    /// Electrical equipment, appliances, and components manufacturing
    ManuElectricalEquipmentAppliancesAndComponents,
    /// Fabricated metal products manufacturing
    ManuFabricatedMetals,
    /// Manufacturing
    Manufacturing,
    /// Food manufacturing
    ManuFood,
    /// Furniture and related products manufacturing
    ManuFurnitureAndRelated,
    /// Industrial machinery manufacturing
    ManuIndustrialMachinery,
    /// Machinery manufacturing
    ManuMachinery,
    /// Magnetic and optical media manufacturing
    ManuMagneticAndOpticalMedia,
    /// Miscellaneous manufacturing
    ManuMiscellaneous,
    /// Motor vehicles, bodies and trailers, and parts manufacturing
    ManuMotorVehiclesBodiesTrailersAndParts,
    /// Navigational, measuring, and other instruments manufacturing
    ManuNavigationalMeasuringAndOthInstruments,
    /// Nonmetallic mineral products manufacturing
    ManuNonmetallicMineral,
    /// Other machinery manufacturing
    ManuOthMachinery,
    /// Other transportation equipment manufacturing
    ManuOthTransportationEquipment,
    /// Paper manufacturing
    ManuPaper,
    /// Petroleum and coal products manufacturing
    ManuPetroleumAndCoal,
    /// Pharmaceuticals and medicines manufacturing
    ManuPharmaceuticalsAndMedicines,
    /// Plastics and rubber products manufacturing
    ManuPlasticsAndRubber,
    /// Primary and fabricated metals manufacturing
    ManuPrimaryAndFabricatedMetals,
    /// Primary metals manufacturing
    ManuPrimaryMetals,
    /// Printing and related support activities
    ManuPrintingAndRelated,
    /// Resins and synthetic rubber, fibers, and filaments manufacturing
    ManuResinsSyntheticRubberFibersAndFilaments,
    /// Semiconductors and other electronic components manufacturing
    ManuSemiconductorsAndOthElectronicComponents,
    /// Soap, cleaning compounds, and toilet preparations manufacturing
    ManuSoapCleaningCompoundsAndToiletPreparations,
    /// Textiles, apparel, and leather products manufacturing
    ManuTextilesApparelAndLeather,
    /// Transportation equipment manufacturing
    ManuTransportationEquipment,
    /// Wood products manufacturing
    ManuWood,
    /// Mining
    Mining,
    /// Oil and gas extraction
    MiningOilAndGasExtraction,
    /// Mining other than oil and gas extraction
    MiningOth,
    /// Miscellaneous services
    Miscellaneous,
    /// Other administrative and reservation services
    OthAdministrativeAndSupport,
    /// Other industries
    OthIndustries,
    /// Other miscellaneous services
    OthMiscellaneous,
    /// Other transportation and warehousing
    OthTransportationAndWarehousing,
    /// Professional, scientific, and technical services
    ProfessionalScientificAndTechnical,
    /// Other professional, scientific, and technical services
    ProfessionalScientificAndTechnicalOth,
    /// Other professional, scientific, and technical services (Other)
    ProfessionalScientificAndTechnicalOthOth,
    /// Rail transportation
    RailTransportation,
    /// Real estate and rental and leasing
    RealEstateAndRentalAndLeasing,
    /// Real estate
    RealEstateAndRentalAndLeasingRealEstate,
    /// Rental and leasing (except real estate)
    RealEstateAndRentalAndLeasingRentalAndLeasing,
    /// Retail trade
    RetailTrade,
    /// Clothing and clothing accessories stores (MOFAs); Clothing, clothing accessories, shoe, and jewelry retailers (MOUSAs)
    RetailTradeClothingAndClothingAccessories,
    /// Food and beverage stores (MOFAs); Food and beverage retailers (MOUSAs)
    RetailTradeFoodAndBeverage,
    /// General merchandise stores (MOFAs); General merchandise retailers (MOUSAs)
    RetailTradeGeneralMerchandise,
    /// Nonstore retailers
    RetailTradeNonstoreRetailers,
    /// Other retail trade
    RetailTradeOthRetailTrade,
    /// Scientific research and development services
    ScientificResearchAndDevelopment,
    /// Specialized design services
    SpecializedDesign,
    /// Support activities for transportation
    SupportActivitiesForTransportation,
    /// Transportation and warehousing
    TransportationAndWarehousing,
    /// Travel arrangement and reservation services
    TravelArrangementAndReservation,
    /// Truck transportation
    TruckTransportation,
    /// Utilities
    Utilities,
    /// Waste management and remediation services
    WasteManagementAndRemediation,
    /// Water transportation
    WaterTransportation,
    /// Wholesale trade
    WholesaleTrade,
    /// Drugs and druggists' sundries wholesale trade
    WholesaleTradeDrugsDruggistsSundries,
    /// Electronic and electronic goods wholesale trade
    WholesaleTradeElectricalAndElectronicGoods,
    /// Motor vehicles and motor vehicle parts and supplies wholesale trade
    WholesaleTradeMotorVehiclesAndMotorVehicleParts,
    /// Other wholesale trade
    WholesaleTradeOthWholesaleTrade,
    /// Petroleum and petroleum products wholesale trade
    WholesaleTradePetroleumAndPetroleum,
    /// Professional and commercial equipment and supplies wholesale trade
    WholesaleTradeProfessionalAndCommercialEquipment,
}

impl IipIndustry {
    /// Returns the human-readable description of the industry.
    pub fn description(&self) -> &'static str {
        match self {
            Self::Accommodation => "Accommodation",
            Self::AccommodationAndFood => "Accommodation and food services",
            Self::AccountingTaxPrepBookkeepingAndPayroll => {
                "Accounting, tax preparation, bookkeeping, and payroll services"
            }
            Self::AdministrationSupportAndWasteManagement => {
                "Administration, support, and waste management"
            }
            Self::AdministrativeAndSupport => "Administrative and support services",
            Self::AdvertisingAndRelated => "Advertising and related services",
            Self::AgricultureForestryFishingAndHunting => {
                "Agriculture, forestry, fishing, and hunting"
            }
            Self::AirTransportation => "Air transportation",
            Self::AllInd => "All industries",
            Self::AllNonbankInd => "All nonbank industries",
            Self::ArchitecturalEngineeringAndRelated => {
                "Architectural, engineering, and related services"
            }
            Self::ArtsEntertainmentAndRecreation => "Arts, entertainment, and recreation",
            Self::ComputerSystemsDesignAndRelated => "Computer systems design and related services",
            Self::Construction => "Construction",
            Self::Educational => "Educational services",
            Self::Employment => "Employment services",
            Self::FinanceAndInsurance => "Finance and insurance",
            Self::FinanceAndInsuranceBanking => "Depository credit intermediation (banking)",
            Self::FinanceAndInsuranceFinance => "Finance",
            Self::FinanceAndInsuranceFinanceExceptBanking => {
                "Finance, except depository institutions"
            }
            Self::FinanceAndInsuranceInsuranceCarriersAndRelated => {
                "Insurance carriers and related activities"
            }
            Self::FoodServicesAndDrinkingPlaces => "Food services and drinking places",
            Self::HealthCareAndSocialAssistance => "Health care and social assistance",
            Self::Info => "Information",
            Self::InfoBroadcasting => "Broadcasting (except Internet)",
            Self::InfoDataProcessingHostingAndRelated => {
                "Data processing, hosting, and related services (MOFAs); Computing infrastructure providers, data processing, web hosting, and related services (MOUSAs)"
            }
            Self::InfoMotionPictureAndSoundRecording => {
                "Motion picture and sound recording industries"
            }
            Self::InfoMotionPictureAndVideo => "Motion picture and video industries",
            Self::InfoNewspaperPeriodicalBookDatabasePublishers => {
                "Newspaper, periodical, book, and database publishers (MOFAs); Newspaper, periodical, book, and directory publishers (MOUSAs)"
            }
            Self::InfoOth => {
                "Other information services (MOFAs); Web search portals, libraries, archives, and other information services (MOUSAs)"
            }
            Self::InfoOthTelecommunications => "Other telecommunications",
            Self::InfoPublishing => "Publishing industries",
            Self::InfoSoftwarePublishers => "Software publishers",
            Self::InfoSoundRecording => "Sound recording industries",
            Self::InfoTelecommunications => "Telecommunications",
            Self::InfoWiredTelecommunicationsCarriers => "Wired telecommunications carriers",
            Self::InfoWirelessTelecommunicationsCarriers => {
                "Wireless telecommunications carriers (except satellite) (MOFAs); Wired and wireless telecommunications carriers (except satellite) (MOUSAs)"
            }
            Self::Legal => "Legal services",
            Self::ManagementOfNonbankCompanies => "Management of nonbank companies and enterprises",
            Self::ManagementScientificAndTechnicalConsulting => {
                "Management, scientific, and technical consulting"
            }
            Self::ManuAgriculturalChemicalsPaintsCoatingsAdhesivesAndOth => {
                "Other chemicals manufacturing"
            }
            Self::ManuAgricultureConstructionAndMiningMachinery => {
                "Agriculture, construction, and mining machinery manufacturing"
            }
            Self::ManuAudioAndVideoEquipment => "Audio and video equipment manufacturing",
            Self::ManuBasicChemicals => "Basic chemicals manufacturing",
            Self::ManuBeveragesAndTobacco => "Beverages and tobacco products manufacturing",
            Self::ManuChemicals => "Chemicals manufacturing",
            Self::ManuChemicalsOth => "Other chemicals manufacturing",
            Self::ManuCommunicationsEquipment => "Communications equipment manufacturing",
            Self::ManuComputersAndElectronic => "Computers and electronic products manufacturing",
            Self::ManuComputersAndElectronicOth => {
                "Other computers and electronic products manufacturing"
            }
            Self::ManuComputersAndPeripheralEquipment => {
                "Computers and peripheral equipment manufacturing"
            }
            Self::ManuElectricalEquipmentAppliancesAndComponents => {
                "Electrical equipment, appliances, and components manufacturing"
            }
            Self::ManuFabricatedMetals => "Fabricated metal products manufacturing",
            Self::Manufacturing => "Manufacturing",
            Self::ManuFood => "Food manufacturing",
            Self::ManuFurnitureAndRelated => "Furniture and related products manufacturing",
            Self::ManuIndustrialMachinery => "Industrial machinery manufacturing",
            Self::ManuMachinery => "Machinery manufacturing",
            Self::ManuMagneticAndOpticalMedia => "Magnetic and optical media manufacturing",
            Self::ManuMiscellaneous => "Miscellaneous manufacturing",
            Self::ManuMotorVehiclesBodiesTrailersAndParts => {
                "Motor vehicles, bodies and trailers, and parts manufacturing"
            }
            Self::ManuNavigationalMeasuringAndOthInstruments => {
                "Navigational, measuring, and other instruments manufacturing"
            }
            Self::ManuNonmetallicMineral => "Nonmetallic mineral products manufacturing",
            Self::ManuOthMachinery => "Other machinery manufacturing",
            Self::ManuOthTransportationEquipment => "Other transportation equipment manufacturing",
            Self::ManuPaper => "Paper manufacturing",
            Self::ManuPetroleumAndCoal => "Petroleum and coal products manufacturing",
            Self::ManuPharmaceuticalsAndMedicines => "Pharmaceuticals and medicines manufacturing",
            Self::ManuPlasticsAndRubber => "Plastics and rubber products manufacturing",
            Self::ManuPrimaryAndFabricatedMetals => "Primary and fabricated metals manufacturing",
            Self::ManuPrimaryMetals => "Primary metals manufacturing",
            Self::ManuPrintingAndRelated => "Printing and related support activities",
            Self::ManuResinsSyntheticRubberFibersAndFilaments => {
                "Resins and synthetic rubber, fibers, and filaments manufacturing"
            }
            Self::ManuSemiconductorsAndOthElectronicComponents => {
                "Semiconductors and other electronic components manufacturing"
            }
            Self::ManuSoapCleaningCompoundsAndToiletPreparations => {
                "Soap, cleaning compounds, and toilet preparations manufacturing"
            }
            Self::ManuTextilesApparelAndLeather => {
                "Textiles, apparel, and leather products manufacturing"
            }
            Self::ManuTransportationEquipment => "Transportation equipment manufacturing",
            Self::ManuWood => "Wood products manufacturing",
            Self::Mining => "Mining",
            Self::MiningOilAndGasExtraction => "Oil and gas extraction",
            Self::MiningOth => "Mining other than oil and gas extraction",
            Self::Miscellaneous => "Miscellaneous services",
            Self::OthAdministrativeAndSupport => "Other administrative and reservation services",
            Self::OthIndustries => "Other industries",
            Self::OthMiscellaneous => "Other miscellaneous services",
            Self::OthTransportationAndWarehousing => "Other transportation and warehousing",
            Self::ProfessionalScientificAndTechnical => {
                "Professional, scientific, and technical services"
            }
            Self::ProfessionalScientificAndTechnicalOth => {
                "Other professional, scientific, and technical services"
            }
            Self::ProfessionalScientificAndTechnicalOthOth => {
                "Other professional, scientific, and technical services (Other)"
            }
            Self::RailTransportation => "Rail transportation",
            Self::RealEstateAndRentalAndLeasing => "Real estate and rental and leasing",
            Self::RealEstateAndRentalAndLeasingRealEstate => "Real estate",
            Self::RealEstateAndRentalAndLeasingRentalAndLeasing => {
                "Rental and leasing (except real estate)"
            }
            Self::RetailTrade => "Retail trade",
            Self::RetailTradeClothingAndClothingAccessories => {
                "Clothing and clothing accessories stores (MOFAs); Clothing, clothing accessories, shoe, and jewelry retailers (MOUSAs)"
            }
            Self::RetailTradeFoodAndBeverage => {
                "Food and beverage stores (MOFAs); Food and beverage retailers (MOUSAs)"
            }
            Self::RetailTradeGeneralMerchandise => {
                "General merchandise stores (MOFAs); General merchandise retailers (MOUSAs)"
            }
            Self::RetailTradeNonstoreRetailers => "Nonstore retailers",
            Self::RetailTradeOthRetailTrade => "Other retail trade",
            Self::ScientificResearchAndDevelopment => {
                "Scientific research and development services"
            }
            Self::SpecializedDesign => "Specialized design services",
            Self::SupportActivitiesForTransportation => "Support activities for transportation",
            Self::TransportationAndWarehousing => "Transportation and warehousing",
            Self::TravelArrangementAndReservation => "Travel arrangement and reservation services",
            Self::TruckTransportation => "Truck transportation",
            Self::Utilities => "Utilities",
            Self::WasteManagementAndRemediation => "Waste management and remediation services",
            Self::WaterTransportation => "Water transportation",
            Self::WholesaleTrade => "Wholesale trade",
            Self::WholesaleTradeDrugsDruggistsSundries => {
                "Drugs and druggists' sundries wholesale trade"
            }
            Self::WholesaleTradeElectricalAndElectronicGoods => {
                "Electronic and electronic goods wholesale trade"
            }
            Self::WholesaleTradeMotorVehiclesAndMotorVehicleParts => {
                "Motor vehicles and motor vehicle parts and supplies wholesale trade"
            }
            Self::WholesaleTradeOthWholesaleTrade => "Other wholesale trade",
            Self::WholesaleTradePetroleumAndPetroleum => {
                "Petroleum and petroleum products wholesale trade"
            }
            Self::WholesaleTradeProfessionalAndCommercialEquipment => {
                "Professional and commercial equipment and supplies wholesale trade"
            }
        }
    }
}
