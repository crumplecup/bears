/// The MNE dataset includes row numbers for summary statistics over industries that do not
/// correspond to valid NAICS codes.  The `NaicsSupplement` type accommodates these additional
/// categories.
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
pub enum NaicsSupplement {
    /// Accommodation
    Accommodation,
    /// Accommodation and food services
    AccommodationFoodServices,
    /// Administration, support, and waste management
    AdministrationSupportWasteManagement,
    /// Agriculture, forestry, fishing, and hunting
    AgricultureForestryFishingHunting,
    /// Air transportation
    AirTransportation,
    /// Ambulatory health care services
    AmbulatoryHealthCareServices,
    /// Amusement, gambling, and recreation industries
    AmusementGamblingRecreationIndustries,
    /// Animal production
    AnimalProduction,
    /// Apparel
    Apparel,
    /// Arts, entertainment, and recreation
    ArtsEntertainmentRecreation,
    /// Arts, entertainment, recreation, accommodation, and food services
    ArtsEntertainmentRecreationAccommodationFoodServices,
    /// Asphalt and other petroleum and coal products,
    AsphaltOtherPetroleumCoal,
    /// Automobile manufacturing
    AutomobileManufacturing,
    /// Beverages and tobacco products
    BeveragesTobacco,
    /// Branches and agencies
    BranchesAgencies,
    /// Broadcasting, cable networks, and program distribution
    BroadcastingCableNetworksProgramDistribution,
    /// Broadcasting (except internet)
    BroadcastingExceptInternet,
    /// Broadcasting (except internet) and telecommunications
    BroadcastingExceptInternetTelecommunications,
    /// Broadcasting and telecommunications
    BroadcastingTelecommunications,
    /// Building, developing, and general contracting
    BuildingDevelopingGeneralContracting,
    /// Building material and garden equipment and supplies dealers
    BuildingMaterialGardenEquipmentSuppliesDealers,
    /// Cable networks and program distribution
    CableNetworksProgramDistribution,
    /// Cable and other program distribution
    CableOtherProgramDistribution,
    /// Chemicals,
    Chemicals,
    /// Clothing, clothing accessories, shoe, and jewelry retailers
    ClothingAccessoriesShoeJewelryRetailers,
    /// Clothing and clothing accessories stores
    ClothingAccessoriesStores,
    /// Computers and electronic products
    ComputersElectronic,
    /// Construction
    Construction,
    /// Construction of buildings
    ConstructionBuildings,
    /// Copper, nickel, lead, and zinc ores
    CopperNickelLeadZincOres,
    /// Corporate, subsidiary, and regional management offices
    CorporateSubsidiaryRegionalManagementOffices,
    /// Couriers and messengers
    CouriersMessengers,
    /// Crop production
    CropProduction,
    /// Data processing, internet publishing, and other information services
    DataProcessingInternetPublishingOtherInformationServices,
    /// Data processing services
    DataProcessingServices,
    /// Direct life insurance carriers
    DirectLifeInsuranceCarriers,
    /// Educational services
    EducationalServices,
    /// Educational services, health care, and social assistance
    EducationalServicesHealthCareSocialAssistance,
    /// Electrical equipment, appliances, and components
    ElectricalEquipmentAppliancesComponents,
    /// Electronics and appliance retailers
    ElectronicsApplianceRetailers,
    /// Fabricated metal products
    FabricatedMetal,
    /// Finance, except depository institutions
    FinanceExceptDepositoryInstitutions,
    /// Finance (except depository institutions) and insurance
    FinanceExceptDepositoryInstitutionsInsurance,
    /// Finance and insurance
    FinanceInsurance,
    /// Fishing, hunting, and trapping
    FishingHuntingTrapping,
    /// Food
    Food,
    /// Food and beverage retailers
    FoodBeverageRetailers,
    /// Food services and drinking places
    FoodServicesDrinkingPlaces,
    /// Forestry and logging
    ForestryLogging,
    /// Funds, trusts, and other financial vehicles
    FundsTrustsOtherFinancialVehicles,
    /// Furniture and home furnishings retailers
    FurnitureHomeFurnishingsRetailers,
    /// Furniture and related products
    FurnitureRelated,
    /// Gasoline stations
    GasolineStations,
    /// General merchandise retailers
    GeneralMerchandiseRetailers,
    /// General merchandise stores
    GeneralMerchandiseStores,
    /// Gold and silver ores
    GoldSilverOres,
    /// Health care and social assistance
    HealthCareSocialAssistance,
    /// Health and personal care retailers
    HealthPersonalCareRetailers,
    /// Health and personal care stores
    HealthPersonalCareStores,
    /// Heavy and civil engineering construction
    HeavyCivilEngineeringConstruction,
    /// Heavy construction
    HeavyConstruction,
    /// Hospitals
    Hospitals,
    /// Information
    Information,
    /// Information services and data processing services
    InformationDataProcessingServices,
    /// Information services
    InformationServices,
    /// Insurance carriers, except direct life insurance carriers
    InsuranceCarriersExceptDirectLifeInsuranceCarriers,
    /// Insurance carriers and related activities
    InsuranceCarriersRelatedActivities,
    /// Integrated petrolium refining and extraction
    IntegratedPetroliumRefiningExtraction,
    /// International - Drilling Rigs
    InternationalDrillingRigs,
    /// Internet, data processing, and other information services
    InternetDataProcessingOtherInformationServices,
    /// Internet service providers and web search portals
    InternetServiceProvidersWebSearchPortals,
    /// Internet service providers, web search portals, and data processing services
    InternetServiceProvidersWebSearchPortalsDataProcessingServices,
    /// Internet publishing and broadcasting
    InternetPublishingBroadcasting,
    /// Iron ores
    IronOres,
    /// Leather and allied products
    LeatherAlliedProducts,
    /// Light truck and utility vehicle manufacturing
    LightTruckUtilityVehicleManufacturing,
    /// Machinery
    Machinery,
    /// Management of nonbank companies and enterprises
    ManagementNonbankCompaniesEnterprises,
    /// Manufacturing
    Manufacturing,
    /// Metal ore mining
    MetalOreMining,
    /// Mining
    Mining,
    /// Mining (except oil and gas)
    MiningExceptOilGas,
    /// Miscellaneous manufacturing
    MiscellaneousManufacturing,
    /// Miscellaneous services
    MiscellaneousServices,
    /// Miscellaneous store retailers
    MiscellaneousStoreRetailers,
    /// Motion picture and sound recording industries
    MotionPictureSoundRecordingIndustries,
    /// Motor vehicle and parts dealers
    MotorVehiclePartsDealers,
    /// Motor vehicles, bodies and trailers, and parts
    MotorVehiclesBodiesTrailersParts,
    /// Nonbusiness Entities, Except Government
    NonbusinessEntitiesExceptGovernment,
    /// Nondepository credit intermediation and related services
    NondepositoryCreditIntermediationRelatedServices,
    /// Nonmetallic mineral products
    NonmetalicMineral,
    /// Nonstore retailers
    NonstoreRetailers,
    /// Nursing and residential care facilities
    NursingResidentialCareFacilities,
    /// Other durable goods
    OtherDurableGoods,
    /// Other finance, except depository institutions
    OtherFinanceExceptDepositoryInstitutions,
    /// Other financial investment activities and exchanges
    OtherFinancialInvestmentActivitiesExchanges,
    /// Other industries
    OtherIndustries,
    /// Other metal ores
    OtherMetalOres,
    /// Other nondurable goods
    OtherNondurableGoods,
    /// Other manufacturing
    OtherManufacturing,
    /// Other pipeline transportation
    OtherPipelineTransportation,
    /// Other rental and leasing services
    OtherRentalLeasingServices,
    /// Other services (except public administration and private households)
    OtherServicesExceptPublicAdministrationPrivateHouseholds,
    /// Other warehousing and storage
    OtherWarehousingStorage,
    /// Other water transportation
    OtherWaterTransportation,
    /// Paper
    Paper,
    /// Performing arts, spectator sports, and related industries
    PerformingArtsSpectatorSportsRelatedIndustries,
    /// Personal and laundry services
    PersonalLaundryServices,
    /// Petroleum
    Petroleum,
    /// Petroleum refining excluding oil and gas extraction
    PetroleumRefiningExcludingOilGasExtraction,
    /// Petroleum storage for hire
    PetroleumStorageForHire,
    /// Petroleum tanker operations
    PetroleumTankerOperations,
    /// Petrolium and coal products
    PetroliumCoal,
    /// Pipeline transportation
    PipelineTransportation,
    /// Pipeline transportation of crude oil, refined petroleum products, and natural gas
    PipelineTransportationCrudeOilRefinedPetroleumNaturalGas,
    /// Plastics and rubber products
    PlasticsRubber,
    /// Primary and fabricated metals
    PrimaryFabricatedMetals,
    /// Primary metals
    PrimaryMetals,
    /// Professional and commercial equipment and supplies
    ProfessionalCommercialEquipmentSupplies,
    /// Professional, scientific, and technical services
    ProfessionalScientificTechnicalServices,
    /// Publishing industries
    PublishingIndustries,
    /// Real estate
    RealEstate,
    /// Real estate and rental and leasing
    RealEstateRentalLeasing,
    /// Religious, grantmaking, civic, professional, and similar organizations
    ReligiousGrantmakingCivicProfessionalSimilarOrganizations,
    /// Rental and leasing (except real estate)
    RentalLeasingExceptRealEstate,
    /// Repair and maintenance
    RepairMaintenance,
    /// Retail trade
    RetailTrade,
    /// Scenic and sightseeing transportation
    ScenicSightseeingTransportation,
    /// Securities, commodity contracts, and other intermediation and related activities
    SecuritiesCommodityContractsOtherIntermediationRelatedActivities,
    /// Social assistance
    SocialAssistance,
    /// Specialty trade contractors
    SpecialtyTradeContractors,
    /// Sporting goods, hobby, and musical instrument retailers
    SportingGoodsHobbyMusicalInstrumentRetailers,
    /// Support activities for agriculture and forestry
    SupportActivitiesAgricultureForestry,
    /// Support activities for mining
    SupportActivitiesMining,
    /// Support activities for mining, except for oil and gas extraction
    SupportActivitiesMiningExceptOilGasExtraction,
    /// Support activities for oil and gas extraction
    SupportActivitiesOilGasExtraction,
    /// Support activities for transportation
    SupportActivitiesTransportation,
    /// Telecommunications
    Telecommunications,
    /// Textile Mills
    TextileMills,
    /// Textile Product Mills
    TextileProductMills,
    /// Textiles, apparel, and leather products
    TextilesApparelLeather,
    /// Transit and ground passenger transportation
    TransitGroundPassengerTransportation,
    /// Transportation equipment
    TransportationEquipment,
    /// Transportation and warehousing
    TransportationWarehousing,
    /// Truck transportation
    TruckTransportation,
    /// Unspecified
    Unspecified,
    /// Utilities
    Utilities,
    /// Warehousing and storage
    WarehousingStorage,
    /// Waste management and remediation services
    WasteManagementRemediationServices,
    /// Water transportation
    WaterTransportation,
    /// Wholesale trade
    WholesaleTrade,
    /// Wired telecommunications carriers
    WiredTelecommunicationsCarriers,
    /// Wireless telecommunications carriers (except satellite)
    WiredTelecommunicationsCarriersExceptSatellite,
    /// Wood products
    Wood,
}

impl NaicsSupplement {
    /// Returns the official NAICS title for this supplement.
    pub fn description(&self) -> &'static str {
        match self {
            Self::Accommodation => "Accommodation",
            Self::AccommodationFoodServices => "Accommodation and food services",
            Self::AdministrationSupportWasteManagement => {
                "Administration, support, and waste management"
            }
            Self::AgricultureForestryFishingHunting => {
                "Agriculture, forestry, fishing, and hunting"
            }
            Self::AirTransportation => "Air transportation",
            Self::AmbulatoryHealthCareServices => "Ambulatory health care services",
            Self::AmusementGamblingRecreationIndustries => {
                "Amusement, gambling, and recreation industries"
            }
            Self::AnimalProduction => "Animal production",
            Self::Apparel => "Apparel",
            Self::ArtsEntertainmentRecreation => "Arts, entertainment, and recreation",
            Self::ArtsEntertainmentRecreationAccommodationFoodServices => {
                "Arts, entertainment, recreation, accommodation, and food services"
            }
            Self::AsphaltOtherPetroleumCoal => "Asphalt and other petroleum and coal products",
            Self::AutomobileManufacturing => "Automobile manufacturing",
            Self::BeveragesTobacco => "Beverages and tobacco products",
            Self::BranchesAgencies => "Branches and agencies",
            Self::BroadcastingCableNetworksProgramDistribution => {
                "Broadcasting, cable networks, and program distribution"
            }
            Self::BroadcastingExceptInternet => "Broadcasting (except internet)",
            Self::BroadcastingExceptInternetTelecommunications => {
                "Broadcasting (except internet) and telecommunications"
            }
            Self::BroadcastingTelecommunications => "Broadcasting and telecommunications",
            Self::BuildingDevelopingGeneralContracting => {
                "Building, developing, and general contracting"
            }
            Self::BuildingMaterialGardenEquipmentSuppliesDealers => {
                "Building material and garden equipment and supplies dealers"
            }
            Self::CableNetworksProgramDistribution => "Cable networks and program distribution",
            Self::CableOtherProgramDistribution => "Cable and other program distribution",
            Self::Chemicals => "Chemicals",
            Self::ClothingAccessoriesShoeJewelryRetailers => {
                "Clothing, clothing accessories, shoe, and jewelry retailers"
            }
            Self::ClothingAccessoriesStores => "Clothing and clothing accessories stores",
            Self::ComputersElectronic => "Computers and electronic products",
            Self::Construction => "Construction",
            Self::ConstructionBuildings => "Construction of buildings",
            Self::CopperNickelLeadZincOres => "Copper, nickel, lead, and zinc ores",
            Self::CorporateSubsidiaryRegionalManagementOffices => {
                "Corporate, subsidiary, and regional management offices"
            }
            Self::CouriersMessengers => "Couriers and messengers",
            Self::CropProduction => "Crop production",
            Self::DataProcessingInternetPublishingOtherInformationServices => {
                "Data processing, internet publishing, and other information services"
            }
            Self::DataProcessingServices => "Data processing services",
            Self::DirectLifeInsuranceCarriers => "Direct life insurance carriers",
            Self::EducationalServices => "Educational services",
            Self::EducationalServicesHealthCareSocialAssistance => {
                "Educational services, health care, and social assistance"
            }
            Self::ElectricalEquipmentAppliancesComponents => {
                "Electrical equipment, appliances, and components"
            }
            Self::ElectronicsApplianceRetailers => "Electronics and appliance retailers",
            Self::FabricatedMetal => "Fabricated metal products",
            Self::FinanceExceptDepositoryInstitutions => "Finance, except depository institutions",
            Self::FinanceExceptDepositoryInstitutionsInsurance => {
                "Finance (except depository institutions) and insurance"
            }
            Self::FinanceInsurance => "Finance and insurance",
            Self::FishingHuntingTrapping => "Fishing, hunting, and trapping",
            Self::Food => "Food",
            Self::FoodBeverageRetailers => "Food and beverage retailers",
            Self::FoodServicesDrinkingPlaces => "Food services and drinking places",
            Self::ForestryLogging => "Forestry and logging",
            Self::FundsTrustsOtherFinancialVehicles => {
                "Funds, trusts, and other financial vehicles"
            }
            Self::FurnitureHomeFurnishingsRetailers => "Furniture and home furnishings retailers",
            Self::FurnitureRelated => "Furniture and related products",
            Self::GasolineStations => "Gasoline stations",
            Self::GeneralMerchandiseRetailers => "General merchandise retailers",
            Self::GeneralMerchandiseStores => "General merchandise stores",
            Self::GoldSilverOres => "Gold and silver ores",
            Self::HealthCareSocialAssistance => "Health care and social assistance",
            Self::HealthPersonalCareRetailers => "Health and personal care retailers",
            Self::HealthPersonalCareStores => "Health and personal care stores",
            Self::HeavyCivilEngineeringConstruction => "Heavy and civil engineering construction",
            Self::HeavyConstruction => "Heavy construction",
            Self::Hospitals => "Hospitals",
            Self::Information => "Information",
            Self::InformationDataProcessingServices => {
                "Information services and data processing services"
            }
            Self::InformationServices => "Information services",
            Self::InsuranceCarriersExceptDirectLifeInsuranceCarriers => {
                "Insurance carriers, except direct life insurance carriers"
            }
            Self::InsuranceCarriersRelatedActivities => "Insurance carriers and related activities",
            Self::IntegratedPetroliumRefiningExtraction => {
                "Integrated petrolium refining and extraction"
            }
            Self::InternationalDrillingRigs => "International - Drilling Rigs",
            Self::InternetDataProcessingOtherInformationServices => {
                "Internet, data processing, and other information services"
            }
            Self::InternetPublishingBroadcasting => "Internet publishing and broadcasting",
            Self::InternetServiceProvidersWebSearchPortals => {
                "Internet service providers and web search portals"
            }
            Self::InternetServiceProvidersWebSearchPortalsDataProcessingServices => {
                "Internet service providers, web search portals, and data processing services"
            }
            Self::IronOres => "Iron ores",
            Self::LeatherAlliedProducts => "Leather and allied products",
            Self::LightTruckUtilityVehicleManufacturing => {
                "Light truck and utility vehicle manufacturing"
            }
            Self::Machinery => "Machinery",
            Self::ManagementNonbankCompaniesEnterprises => {
                "Management of nonbank companies and enterprises"
            }
            Self::Manufacturing => "Manufacturing",
            Self::MetalOreMining => "Metal ore mining",
            Self::Mining => "Mining",
            Self::MiningExceptOilGas => "Mining (except oil and gas)",
            Self::MiscellaneousManufacturing => "Miscellaneous manufacturing",
            Self::MiscellaneousServices => "Miscellaneous services",
            Self::MiscellaneousStoreRetailers => "Miscellaneous store retailers",
            Self::MotionPictureSoundRecordingIndustries => {
                "Motion picture and sound recording industries"
            }
            Self::MotorVehiclePartsDealers => "Motor vehicle and parts dealers",
            Self::MotorVehiclesBodiesTrailersParts => {
                "Motor vehicles, bodies and trailers, and parts"
            }
            Self::NonbusinessEntitiesExceptGovernment => "Nonbusiness Entities, Except Government",
            Self::NondepositoryCreditIntermediationRelatedServices => {
                "Nondepository credit intermediation and related services"
            }
            Self::NonmetalicMineral => "Nonmetallic mineral products",
            Self::NonstoreRetailers => "Nonstore retailers",
            Self::NursingResidentialCareFacilities => "Nursing and residential care facilities",
            Self::OtherDurableGoods => "Other durable goods",
            Self::OtherFinanceExceptDepositoryInstitutions => {
                "Other finance, except depository institutions"
            }
            Self::OtherFinancialInvestmentActivitiesExchanges => {
                "Other financial investment activities and exchanges"
            }
            Self::OtherIndustries => "Other industries",
            Self::OtherMetalOres => "Other metal ores",
            Self::OtherNondurableGoods => "Other nondurable goods",
            Self::OtherManufacturing => "Other manufacturing",
            Self::OtherPipelineTransportation => "Other pipeline transportation",
            Self::OtherRentalLeasingServices => "Other rental and leasing services",
            Self::OtherServicesExceptPublicAdministrationPrivateHouseholds => {
                "Other services (except public administration and private households)"
            }
            Self::OtherWarehousingStorage => "Other warehousing and storage",
            Self::OtherWaterTransportation => "Other water transportation",
            Self::Paper => "Paper",
            Self::PerformingArtsSpectatorSportsRelatedIndustries => {
                "Performing arts, spectator sports, and related industries"
            }
            Self::PersonalLaundryServices => "Personal and laundry services",
            Self::Petroleum => "Petroleum",
            Self::PetroleumRefiningExcludingOilGasExtraction => {
                "Petroleum refining excluding oil and gas extraction"
            }
            Self::PetroleumStorageForHire => "Petroleum storage for hire",
            Self::PetroleumTankerOperations => "Petroleum tanker operations",
            Self::PetroliumCoal => "Petrolium and coal products",
            Self::PipelineTransportation => "Pipeline transportation",
            Self::PipelineTransportationCrudeOilRefinedPetroleumNaturalGas => {
                "Pipeline transportation of crude oil, refined petroleum products, and natural gas"
            }
            Self::PlasticsRubber => "Plastic and rubber products",
            Self::PrimaryFabricatedMetals => "Primary and fabricated metals",
            Self::PrimaryMetals => "Primary metals",
            Self::ProfessionalCommercialEquipmentSupplies => {
                "Professional and commercial equipment and supplies"
            }
            Self::ProfessionalScientificTechnicalServices => {
                " Professional, scientific, and technical services"
            }
            Self::PublishingIndustries => "Publishing industries",
            Self::RealEstate => "Real estate",
            Self::RealEstateRentalLeasing => "Real estate and rental and leasing",
            Self::ReligiousGrantmakingCivicProfessionalSimilarOrganizations => {
                "Religious, grantmaking, civic, professional, and similar organizations"
            }
            Self::RentalLeasingExceptRealEstate => "Rental and leasing (except real estate)",
            Self::RepairMaintenance => "Repair and maintenance",
            Self::RetailTrade => "Retail trade",
            Self::ScenicSightseeingTransportation => "Scenic and sightseeing transportation",
            Self::SecuritiesCommodityContractsOtherIntermediationRelatedActivities => {
                "Securities, commodity contracts, and other intermediation and related activities"
            }
            Self::SocialAssistance => "Social assistance",
            Self::SpecialtyTradeContractors => "Specialty trade contractors",
            Self::SportingGoodsHobbyMusicalInstrumentRetailers => {
                "Sporting goods, hobby, and musical instrument retailers"
            }
            Self::SupportActivitiesAgricultureForestry => {
                "Support activities for agriculture and forestry"
            }
            Self::SupportActivitiesMining => "Support activities for mining",
            Self::SupportActivitiesMiningExceptOilGasExtraction => {
                "Support activities for mining, except for oil and gas extraction"
            }
            Self::SupportActivitiesOilGasExtraction => {
                "Support activities for oil and gas extraction"
            }
            Self::SupportActivitiesTransportation => "Support activities for transportation",
            Self::Telecommunications => "Telecommunications",
            Self::TextileMills => "Textile Mills",
            Self::TextileProductMills => "Textile Product Mills",
            Self::TextilesApparelLeather => "Textiles, apparel, and leather products",
            Self::TransitGroundPassengerTransportation => {
                "Transit and ground passenger transportation"
            }
            Self::TransportationEquipment => "Transportation equipment",
            Self::TransportationWarehousing => "Transportation and warehousing",
            Self::TruckTransportation => "Truck transportation",
            Self::Unspecified => "Unspecified",
            Self::Utilities => "Utilities",
            Self::WarehousingStorage => "Warehousing and storage",
            Self::WasteManagementRemediationServices => "Waste management and remediation services",
            Self::WaterTransportation => "Water transportation",
            Self::WholesaleTrade => "Wholesale trade",
            Self::WiredTelecommunicationsCarriers => "Wired telecommunications carriers",
            Self::WiredTelecommunicationsCarriersExceptSatellite => {
                "Wireless telecommunications carriers (except satellite)"
            }
            Self::Wood => "Wood products",
        }
    }

    /// Returns the row code associated with the category name.
    pub fn code(&self) -> i64 {
        match self {
            Self::Accommodation => 7210,
            Self::AccommodationFoodServices => 7200,
            Self::AdministrationSupportWasteManagement => 5600,
            Self::AgricultureForestryFishingHunting => 1100,
            Self::AirTransportation => 4810,
            Self::AmbulatoryHealthCareServices => 6210,
            Self::AmusementGamblingRecreationIndustries => 7130,
            Self::AnimalProduction => 1120,
            Self::Apparel => 3150,
            Self::ArtsEntertainmentRecreation => 7100,
            Self::ArtsEntertainmentRecreationAccommodationFoodServices => 7,
            Self::AsphaltOtherPetroleumCoal => 3244,
            Self::AutomobileManufacturing => 336111,
            Self::BranchesAgencies => 5229,
            Self::BeveragesTobacco => 3120,
            Self::BroadcastingCableNetworksProgramDistribution => 5147,
            Self::BroadcastingExceptInternet => 5150,
            Self::BroadcastingExceptInternetTelecommunications => 5149,
            Self::BroadcastingTelecommunications => 5148,
            Self::BuildingDevelopingGeneralContracting => 2330,
            Self::BuildingMaterialGardenEquipmentSuppliesDealers => 4440,
            Self::CableNetworksProgramDistribution => 5176,
            Self::CableOtherProgramDistribution => 5175,
            Self::Chemicals => 3250,
            Self::ClothingAccessoriesShoeJewelryRetailers => 4480,
            Self::ClothingAccessoriesStores => 448,
            Self::ComputersElectronic => 3340,
            Self::Construction => 2300,
            Self::ConstructionBuildings => 2360,
            Self::CopperNickelLeadZincOres => 2126,
            Self::CorporateSubsidiaryRegionalManagementOffices => 5513,
            Self::CouriersMessengers => 4920,
            Self::CropProduction => 1110,
            Self::DataProcessingInternetPublishingOtherInformationServices => 514,
            Self::DataProcessingServices => 5142,
            Self::DirectLifeInsuranceCarriers => 5249,
            Self::EducationalServices => 6110,
            Self::EducationalServicesHealthCareSocialAssistance => 6,
            Self::ElectricalEquipmentAppliancesComponents => 3350,
            Self::ElectronicsApplianceRetailers => 4431,
            Self::FabricatedMetal => 3320,
            Self::FinanceExceptDepositoryInstitutions => 5220,
            Self::FinanceExceptDepositoryInstitutionsInsurance => 5210,
            Self::FinanceInsurance => 5200,
            Self::FishingHuntingTrapping => 1140,
            Self::Food => 3110,
            Self::FoodBeverageRetailers => 4450,
            Self::FoodServicesDrinkingPlaces => 7220,
            Self::ForestryLogging => 1130,
            Self::FundsTrustsOtherFinancialVehicles => 5252,
            Self::FurnitureHomeFurnishingsRetailers => 4420,
            Self::FurnitureRelated => 3370,
            // Duplicated by child entry 4471
            // Self::GasolineStations => 447,
            Self::GasolineStations => 4471,
            Self::GeneralMerchandiseRetailers => 4520,
            Self::GeneralMerchandiseStores => 452,
            Self::GoldSilverOres => 2125,
            Self::HealthCareSocialAssistance => 6200,
            Self::HealthPersonalCareRetailers => 4461,
            Self::HealthPersonalCareStores => 446,
            Self::HeavyCivilEngineeringConstruction => 2370,
            Self::HeavyConstruction => 2340,
            Self::Hospitals => 6220,
            Self::Information => 5100,
            Self::InformationServices => 5141,
            Self::InformationDataProcessingServices => 5188,
            Self::InsuranceCarriersExceptDirectLifeInsuranceCarriers => 5243,
            Self::InsuranceCarriersRelatedActivities => 5240,
            Self::IntegratedPetroliumRefiningExtraction => 3242,
            Self::InternationalDrillingRigs => 998,
            Self::InternetDataProcessingOtherInformationServices => 5189,
            Self::InternetPublishingBroadcasting => 5161,
            Self::InternetServiceProvidersWebSearchPortals => 5181,
            Self::InternetServiceProvidersWebSearchPortalsDataProcessingServices => 5180,
            Self::IronOres => 2124,
            Self::LeatherAlliedProducts => 3160,
            Self::LightTruckUtilityVehicleManufacturing => 336112,
            Self::Machinery => 3330,
            Self::ManagementNonbankCompaniesEnterprises => 5512,
            Self::Manufacturing => 3000,
            Self::MetalOreMining => 2128,
            Self::Mining => 2100,
            Self::MiningExceptOilGas => 2199,
            Self::MiscellaneousManufacturing => 3390,
            Self::MiscellaneousServices => 6000,
            Self::MiscellaneousStoreRetailers => 4530,
            Self::MotionPictureSoundRecordingIndustries => 5120,
            Self::MotorVehiclePartsDealers => 4410,
            Self::MotorVehiclesBodiesTrailersParts => 3368,
            Self::NonbusinessEntitiesExceptGovernment => 9100,
            Self::NondepositoryCreditIntermediationRelatedServices => 5224,
            Self::NonmetalicMineral => 3270,
            Self::NonstoreRetailers => 4540,
            // Duplicated by child 4540
            // Self::NonstoreRetailers => 454,
            Self::NursingResidentialCareFacilities => 6230,
            Self::OtherDurableGoods => 4299,
            Self::OtherFinanceExceptDepositoryInstitutions => 5299,
            Self::OtherFinancialInvestmentActivitiesExchanges => 5238,
            Self::OtherIndustries => 8000,
            Self::OtherMetalOres => 2127,
            Self::OtherNondurableGoods => 4298,
            Self::OtherManufacturing => 3900,
            Self::OtherPipelineTransportation => 4868,
            Self::OtherRentalLeasingServices => 5329,
            Self::OtherServicesExceptPublicAdministrationPrivateHouseholds => 8100,
            Self::OtherWarehousingStorage => 4939,
            Self::OtherWaterTransportation => 4839,
            Self::Paper => 3220,
            Self::PerformingArtsSpectatorSportsRelatedIndustries => 7110,
            Self::PersonalLaundryServices => 8120,
            Self::Petroleum => 8999,
            Self::PetroleumRefiningExcludingOilGasExtraction => 3243,
            Self::PetroleumStorageForHire => 4932,
            Self::PetroleumTankerOperations => 4833,
            Self::PetroliumCoal => 3240,
            Self::PipelineTransportation => 4860,
            Self::PipelineTransportationCrudeOilRefinedPetroleumNaturalGas => 4863,
            Self::PlasticsRubber => 3260,
            Self::PrimaryFabricatedMetals => 3300,
            Self::PrimaryMetals => 3310,
            Self::ProfessionalCommercialEquipmentSupplies => 4214,
            Self::ProfessionalScientificTechnicalServices => 5400,
            Self::PublishingIndustries => 5110,
            Self::RealEstate => 5310,
            Self::RealEstateRentalLeasing => 5300,
            Self::ReligiousGrantmakingCivicProfessionalSimilarOrganizations => 8130,
            Self::RentalLeasingExceptRealEstate => 5320,
            Self::RepairMaintenance => 8110,
            Self::RetailTrade => 4400,
            Self::ScenicSightseeingTransportation => 4870,
            Self::SecuritiesCommodityContractsOtherIntermediationRelatedActivities => 5230,
            Self::SocialAssistance => 6240,
            Self::SpecialtyTradeContractors => 2350,
            Self::SportingGoodsHobbyMusicalInstrumentRetailers => 4510,
            Self::SupportActivitiesAgricultureForestry => 1150,
            Self::SupportActivitiesMining => 2130,
            Self::SupportActivitiesMiningExceptOilGasExtraction => 2133,
            Self::SupportActivitiesOilGasExtraction => 2132,
            Self::SupportActivitiesTransportation => 4880,
            Self::Telecommunications => 5133,
            Self::TextileMills => 3130,
            Self::TextileProductMills => 3140,
            Self::TextilesApparelLeather => 3170,
            Self::TransitGroundPassengerTransportation => 4850,
            Self::TransportationEquipment => 3360,
            Self::TransportationWarehousing => 4800,
            Self::TruckTransportation => 4840,
            Self::Unspecified => 9000,
            Self::Utilities => 2200,
            Self::WarehousingStorage => 4930,
            Self::WasteManagementRemediationServices => 5620,
            Self::WaterTransportation => 4830,
            Self::WholesaleTrade => 4200,
            Self::WiredTelecommunicationsCarriers => 5171,
            Self::WiredTelecommunicationsCarriersExceptSatellite => 5172,
            Self::Wood => 3210,
        }
    }

    /// Returns an Option with the value if `code` is a number matching the row code of a variant.
    pub fn from_code(code: &str) -> Option<Self> {
        let code = match code.parse::<i64>() {
            Ok(num) => num,
            Err(_) => return None,
        };

        let result = match code {
            6 => Self::EducationalServicesHealthCareSocialAssistance,
            7 => Self::ArtsEntertainmentRecreationAccommodationFoodServices,
            446 => Self::HealthPersonalCareStores,
            447 => Self::GasolineStations,
            448 => Self::ClothingAccessoriesStores,
            452 => Self::GeneralMerchandiseStores,
            454 => Self::NonstoreRetailers,
            513 => Self::BroadcastingTelecommunications,
            514 => Self::DataProcessingInternetPublishingOtherInformationServices,
            998 => Self::InternationalDrillingRigs,
            1100 => Self::AgricultureForestryFishingHunting,
            1110 => Self::CropProduction,
            1120 => Self::AnimalProduction,
            1130 => Self::ForestryLogging,
            1140 => Self::FishingHuntingTrapping,
            1150 => Self::SupportActivitiesAgricultureForestry,
            2100 => Self::Mining,
            2124 => Self::IronOres,
            2125 => Self::GoldSilverOres,
            2126 => Self::CopperNickelLeadZincOres,
            2127 => Self::OtherMetalOres,
            2128 => Self::MetalOreMining,
            2130 => Self::SupportActivitiesMining,
            2132 => Self::SupportActivitiesOilGasExtraction,
            2133 => Self::SupportActivitiesMiningExceptOilGasExtraction,
            2199 => Self::MiningExceptOilGas,
            2200 => Self::Utilities,
            2300 => Self::Construction,
            2330 => Self::BuildingDevelopingGeneralContracting,
            2340 => Self::HeavyConstruction,
            2350 => Self::SpecialtyTradeContractors,
            2360 => Self::ConstructionBuildings,
            2370 => Self::HeavyCivilEngineeringConstruction,
            3000 => Self::Manufacturing,
            3110 => Self::Food,
            3120 => Self::BeveragesTobacco,
            3130 => Self::TextileMills,
            3140 => Self::TextileProductMills,
            3150 => Self::Apparel,
            3160 => Self::LeatherAlliedProducts,
            3170 => Self::TextilesApparelLeather,
            3210 => Self::Wood,
            3220 => Self::Paper,
            3240 => Self::PetroliumCoal,
            3242 => Self::IntegratedPetroliumRefiningExtraction,
            3243 => Self::PetroleumRefiningExcludingOilGasExtraction,
            3244 => Self::AsphaltOtherPetroleumCoal,
            3250 => Self::Chemicals,
            3260 => Self::PlasticsRubber,
            3270 => Self::NonmetalicMineral,
            3300 => Self::PrimaryFabricatedMetals,
            3310 => Self::PrimaryMetals,
            3320 => Self::FabricatedMetal,
            3330 => Self::Machinery,
            3340 => Self::ComputersElectronic,
            3350 => Self::ElectricalEquipmentAppliancesComponents,
            3360 => Self::TransportationEquipment,
            3368 => Self::MotorVehiclesBodiesTrailersParts,
            3370 => Self::FurnitureRelated,
            3390 => Self::MiscellaneousManufacturing,
            3900 => Self::OtherManufacturing,
            4200 => Self::WholesaleTrade,
            4214 => Self::ProfessionalCommercialEquipmentSupplies,
            4298 => Self::OtherNondurableGoods,
            4299 => Self::OtherDurableGoods,
            4400 => Self::RetailTrade,
            4410 => Self::MotorVehiclePartsDealers,
            4420 => Self::FurnitureHomeFurnishingsRetailers,
            4431 => Self::ElectronicsApplianceRetailers,
            4440 => Self::BuildingMaterialGardenEquipmentSuppliesDealers,
            4450 => Self::FoodBeverageRetailers,
            4461 => Self::HealthPersonalCareRetailers,
            4471 => Self::GasolineStations,
            4480 => Self::ClothingAccessoriesShoeJewelryRetailers,
            4510 => Self::SportingGoodsHobbyMusicalInstrumentRetailers,
            4520 => Self::GeneralMerchandiseRetailers,
            4530 => Self::MiscellaneousStoreRetailers,
            4540 => Self::NonstoreRetailers,
            4800 => Self::TransportationWarehousing,
            4810 => Self::AirTransportation,
            4830 => Self::WaterTransportation,
            4833 => Self::PetroleumTankerOperations,
            4839 => Self::OtherWaterTransportation,
            4840 => Self::TruckTransportation,
            4850 => Self::TransitGroundPassengerTransportation,
            4860 => Self::PipelineTransportation,
            4863 => Self::PipelineTransportationCrudeOilRefinedPetroleumNaturalGas,
            4868 => Self::OtherPipelineTransportation,
            4870 => Self::ScenicSightseeingTransportation,
            4880 => Self::SupportActivitiesTransportation,
            4920 => Self::CouriersMessengers,
            4930 => Self::WarehousingStorage,
            4932 => Self::PetroleumStorageForHire,
            4939 => Self::OtherWarehousingStorage,
            5100 => Self::Information,
            5110 => Self::PublishingIndustries,
            5120 => Self::MotionPictureSoundRecordingIndustries,
            5133 => Self::Telecommunications,
            5141 => Self::InformationServices,
            5142 => Self::DataProcessingServices,
            5147 => Self::BroadcastingCableNetworksProgramDistribution,
            5148 => Self::BroadcastingTelecommunications,
            5149 => Self::BroadcastingExceptInternetTelecommunications,
            5150 => Self::BroadcastingExceptInternet,
            5161 => Self::InternetPublishingBroadcasting,
            5171 => Self::WiredTelecommunicationsCarriers,
            5172 => Self::WiredTelecommunicationsCarriersExceptSatellite,
            5175 => Self::CableOtherProgramDistribution,
            5176 => Self::CableNetworksProgramDistribution,
            5180 => Self::InternetServiceProvidersWebSearchPortalsDataProcessingServices,
            5181 => Self::InternetServiceProvidersWebSearchPortals,
            5188 => Self::InformationDataProcessingServices,
            5189 => Self::InternetDataProcessingOtherInformationServices,
            5200 => Self::FinanceInsurance,
            5210 => Self::FinanceExceptDepositoryInstitutionsInsurance,
            5220 => Self::FinanceExceptDepositoryInstitutions,
            5224 => Self::NondepositoryCreditIntermediationRelatedServices,
            5229 => Self::BranchesAgencies,
            5230 => Self::SecuritiesCommodityContractsOtherIntermediationRelatedActivities,
            5238 => Self::OtherFinancialInvestmentActivitiesExchanges,
            5240 => Self::InsuranceCarriersRelatedActivities,
            5243 => Self::InsuranceCarriersExceptDirectLifeInsuranceCarriers,
            5249 => Self::DirectLifeInsuranceCarriers,
            5252 => Self::FundsTrustsOtherFinancialVehicles,
            5299 => Self::OtherFinanceExceptDepositoryInstitutions,
            5300 => Self::RealEstateRentalLeasing,
            5310 => Self::RealEstate,
            5320 => Self::RentalLeasingExceptRealEstate,
            5329 => Self::OtherRentalLeasingServices,
            5400 => Self::ProfessionalScientificTechnicalServices,
            5512 => Self::ManagementNonbankCompaniesEnterprises,
            5513 => Self::CorporateSubsidiaryRegionalManagementOffices,
            5600 => Self::AdministrationSupportWasteManagement,
            5620 => Self::WasteManagementRemediationServices,
            6000 => Self::MiscellaneousServices,
            6110 => Self::EducationalServices,
            6200 => Self::HealthCareSocialAssistance,
            6210 => Self::AmbulatoryHealthCareServices,
            6220 => Self::Hospitals,
            6230 => Self::NursingResidentialCareFacilities,
            6240 => Self::SocialAssistance,
            7100 => Self::ArtsEntertainmentRecreation,
            7110 => Self::PerformingArtsSpectatorSportsRelatedIndustries,
            7130 => Self::AmusementGamblingRecreationIndustries,
            7200 => Self::AccommodationFoodServices,
            7210 => Self::Accommodation,
            7220 => Self::FoodServicesDrinkingPlaces,
            8000 => Self::OtherIndustries,
            8100 => Self::OtherServicesExceptPublicAdministrationPrivateHouseholds,
            8110 => Self::RepairMaintenance,
            8120 => Self::PersonalLaundryServices,
            8130 => Self::ReligiousGrantmakingCivicProfessionalSimilarOrganizations,
            8999 => Self::Petroleum,
            9000 => Self::Unspecified,
            9100 => Self::NonbusinessEntitiesExceptGovernment,
            336111 => Self::AutomobileManufacturing,
            336112 => Self::LightTruckUtilityVehicleManufacturing,
            _ => return None,
        };
        Some(result)
    }
}
