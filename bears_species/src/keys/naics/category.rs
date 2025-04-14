/// NAICS Category codes
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
pub enum NaicsCategory {
    /// Oilseed and Grain Farming
    OilseedAndGrainFarming,
    /// Vegetable and Melon Farming
    VegetableAndMelonFarming,
    /// Fruit and Tree Nut Farming
    FruitAndTreeNutFarming,
    /// Greenhouse, Nursery, and Floriculture Production
    GreenhouseNurseryAndFloricultureProduction,
    /// Other Crop Farming
    OtherCropFarming,
    /// Cattle Ranching and Farming
    CattleRanchingAndFarming,
    /// Hog and Pig Farming
    HogAndPigFarming,
    /// Poultry and Egg Production
    PoultryAndEggProduction,
    /// Sheep and Goat Farming
    SheepAndGoatFarming,
    /// Aquaculture
    Aquaculture,
    /// Other Animal Production
    OtherAnimalProduction,
    /// Timber Tract Operations
    TimberTractOperations,
    /// Forest Nurseries and Gathering of Forest Products
    ForestNurseriesAndGatheringOfForestProducts,
    /// Logging
    Logging,
    /// Fishing
    Fishing,
    /// Hunting and Trapping
    HuntingAndTrapping,
    /// Support Activities for Crop Production
    SupportActivitiesForCropProduction,
    /// Support Activities for Animal Production
    SupportActivitiesForAnimalProduction,
    /// Support Activities for Forestry
    SupportActivitiesForForestry,
    /// Oil and Gas Extraction
    OilAndGasExtraction,
    /// Coal Mining
    CoalMining,
    /// Metal Ore Mining
    MetalOreMining,
    /// Nonmetallic Mineral Mining and Quarrying
    NonmetallicMineralMiningAndQuarrying,
    /// Support Activities for Mining
    SupportActivitiesForMining,
    /// Electric Power Generation, Transmission and Distribution
    ElectricPowerGenerationTransmissionAndDistribution,
    /// Natural Gas Distribution
    NaturalGasDistribution,
    /// Water, Sewage and Other Systems
    WaterSewageAndOtherSystems,
    /// Residential Building Construction
    ResidentialBuildingConstruction,
    /// Nonresidential Building Construction
    NonresidentialBuildingConstruction,
    /// Utility System Construction
    UtilitySystemConstruction,
    /// Land Subdivision
    LandSubdivision,
    /// Highway, Street, and Bridge Construction
    HighwayStreetAndBridgeConstruction,
    /// Other Heavy and Civil Engineering Construction
    OtherHeavyAndCivilEngineeringConstruction,
    /// Foundation, Structure, and Building Exterior Contractors
    FoundationStructureAndBuildingExteriorContractors,
    /// Building Equipment Contractors
    BuildingEquipmentContractors,
    /// Building Finishing Contractors
    BuildingFinishingContractors,
    /// Other Specialty Trade Contractors
    OtherSpecialtyTradeContractors,
    /// Animal Food Manufacturing
    AnimalFoodManufacturing,
    /// Grain and Oilseed Milling
    GrainAndOilseedMilling,
    /// Sugar and Confectionery Product Manufacturing
    SugarAndConfectioneryProductManufacturing,
    /// Fruit and Vegetable Preserving and Specialty Food Manufacturing
    FruitAndVegetablePreservingAndSpecialtyFoodManufacturing,
    /// Dairy Product Manufacturing
    DairyProductManufacturing,
    /// Animal Slaughtering and Processing
    AnimalSlaughteringAndProcessing,
    /// Seafood Product Preparation and Packaging
    SeafoodProductPreparationAndPackaging,
    /// Bakeries and Tortilla Manufacturing
    BakeriesAndTortillaManufacturing,
    /// Other Food Manufacturing
    OtherFoodManufacturing,
    /// Beverage Manufacturing
    BeverageManufacturing,
    /// Tobacco Manufacturing
    TobaccoManufacturing,
    /// Fiber, Yarn, and Thread Mills
    FiberYarnAndThreadMills,
    /// Fabric Mills
    FabricMills,
    /// Textile and Fabric Finishing and Fabric Coating Mills
    TextileAndFabricFinishingAndFabricCoatingMills,
    /// Textile Furnishings Mills
    TextileFurnishingsMills,
    /// Other Textile Product Mills
    OtherTextileProductMills,
    /// Apparel Knitting Mills
    ApparelKnittingMills,
    /// Cut and Sew Apparel Manufacturing
    CutAndSewApparelManufacturing,
    /// Apparel Accessories and Other Apparel Manufacturing
    ApparelAccessoriesAndOtherApparelManufacturing,
    /// Leather and Hide Tanning and Finishing
    LeatherAndHideTanningAndFinishing,
    /// Footwear Manufacturing
    FootwearManufacturing,
    /// Other Leather and Allied Product Manufacturing
    OtherLeatherAndAlliedProductManufacturing,
    /// Sawmills and Wood Preservation
    SawmillsAndWoodPreservation,
    /// Veneer, Plywood, and Engineered Wood Product Manufacturing
    VeneerPlywoodAndEngineeredWoodProductManufacturing,
    /// Other Wood Product Manufacturing
    OtherWoodProductManufacturing,
    /// Pulp, Paper, and Paperboard Mills
    PulpPaperAndPaperboardMills,
    /// Converted Paper Product Manufacturing
    ConvertedPaperProductManufacturing,
    /// Printing and Related Support Activities
    PrintingAndRelatedSupportActivities,
    /// Petroleum and Coal Products Manufacturing
    PetroleumAndCoalProductsManufacturing,
    /// Basic Chemical Manufacturing
    BasicChemicalManufacturing,
    /// Resin, Synthetic Rubber, and Artificial and Synthetic Fibers and Filaments Manufacturing
    ResinSyntheticRubberAndArtificialAndSyntheticFibersAndFilamentsManufacturing,
    /// Pesticide, Fertilizer, and Other Agricultural Chemical Manufacturing
    PesticideFertilizerAndOtherAgriculturalChemicalManufacturing,
    /// Pharmaceutical and Medicine Manufacturing
    PharmaceuticalAndMedicineManufacturing,
    /// Paint, Coating, and Adhesive Manufacturing
    PaintCoatingAndAdhesiveManufacturing,
    /// Soap, Cleaning Compound, and Toilet Preparation Manufacturing
    SoapCleaningCompoundAndToiletPreparationManufacturing,
    /// Other Chemical Product and Preparation Manufacturing
    OtherChemicalProductAndPreparationManufacturing,
    /// Plastics Product Manufacturing
    PlasticsProductManufacturing,
    /// Rubber Product Manufacturing
    RubberProductManufacturing,
    /// Clay Product and Refractory Manufacturing
    ClayProductAndRefractoryManufacturing,
    /// Glass and Glass Product Manufacturing
    GlassAndGlassProductManufacturing,
    /// Cement and Concrete Product Manufacturing
    CementAndConcreteProductManufacturing,
    /// Lime and Gypsum Product Manufacturing
    LimeAndGypsumProductManufacturing,
    /// Other Nonmetallic Mineral Product Manufacturing
    OtherNonmetallicMineralProductManufacturing,
    /// Iron and Steel Mills and Ferroalloy Manufacturing
    IronAndSteelMillsAndFerroalloyManufacturing,
    /// Steel Product Manufacturing from Purchased Steel
    SteelProductManufacturingFromPurchasedSteel,
    /// Alumina and Aluminum Production and Processing
    AluminaAndAluminumProductionAndProcessing,
    /// Nonferrous Metal (except Aluminum) Production and Processing
    NonferrousMetalExceptAluminumProductionAndProcessing,
    /// Foundries
    Foundries,
    /// Forging and Stamping
    ForgingAndStamping,
    /// Cutlery and Handtool Manufacturing
    CutleryAndHandtoolManufacturing,
    /// Architectural and Structural Metals Manufacturing
    ArchitecturalAndStructuralMetalsManufacturing,
    /// Boiler, Tank, and Shipping Container Manufacturing
    BoilerTankAndShippingContainerManufacturing,
    /// Hardware Manufacturing
    HardwareManufacturing,
    /// Spring and Wire Product Manufacturing
    SpringAndWireProductManufacturing,
    /// Machine Shops; Turned Product; and Screw, Nut, and Bolt Manufacturing
    MachineShopsTurnedProductAndScrewNutAndBoltManufacturing,
    /// Coating, Engraving, Heat Treating, and Allied Activities
    CoatingEngravingHeatTreatingAndAlliedActivities,
    /// Other Fabricated Metal Product Manufacturing
    OtherFabricatedMetalProductManufacturing,
    /// Agriculture, Construction, and Mining Machinery Manufacturing
    AgricultureConstructionAndMiningMachineryManufacturing,
    /// Industrial Machinery Manufacturing
    IndustrialMachineryManufacturing,
    /// Commercial and Service Industry Machinery Manufacturing
    CommercialAndServiceIndustryMachineryManufacturing,
    /// Ventilation, Heating, Air-Conditioning, and Commercial Refrigeration Equipment Manufacturing
    VentilationHeatingAirconditioningAndCommercialRefrigerationEquipmentManufacturing,
    /// Metalworking Machinery Manufacturing
    MetalworkingMachineryManufacturing,
    /// Engine, Turbine, and Power Transmission Equipment Manufacturing
    EngineTurbineAndPowerTransmissionEquipmentManufacturing,
    /// Other General Purpose Machinery Manufacturing
    OtherGeneralPurposeMachineryManufacturing,
    /// Computer and Peripheral Equipment Manufacturing
    ComputerAndPeripheralEquipmentManufacturing,
    /// Communications Equipment Manufacturing
    CommunicationsEquipmentManufacturing,
    /// Audio and Video Equipment Manufacturing
    AudioAndVideoEquipmentManufacturing,
    /// Semiconductor and Other Electronic Component Manufacturing
    SemiconductorAndOtherElectronicComponentManufacturing,
    /// Navigational, Measuring, Electromedical, and Control Instruments Manufacturing
    NavigationalMeasuringElectromedicalAndControlInstrumentsManufacturing,
    /// Manufacturing and Reproducing Magnetic and Optical Media
    ManufacturingAndReproducingMagneticAndOpticalMedia,
    /// Electric Lighting Equipment Manufacturing
    ElectricLightingEquipmentManufacturing,
    /// Household Appliance Manufacturing
    HouseholdApplianceManufacturing,
    /// Electrical Equipment Manufacturing
    ElectricalEquipmentManufacturing,
    /// Other Electrical Equipment and Component Manufacturing
    OtherElectricalEquipmentAndComponentManufacturing,
    /// Motor Vehicle Manufacturing
    MotorVehicleManufacturing,
    /// Motor Vehicle Body and Trailer Manufacturing
    MotorVehicleBodyAndTrailerManufacturing,
    /// Motor Vehicle Parts Manufacturing
    MotorVehiclePartsManufacturing,
    /// Aerospace Product and Parts Manufacturing
    AerospaceProductAndPartsManufacturing,
    /// Railroad Rolling Stock Manufacturing
    RailroadRollingStockManufacturing,
    /// Ship and Boat Building
    ShipAndBoatBuilding,
    /// Other Transportation Equipment Manufacturing
    OtherTransportationEquipmentManufacturing,
    /// Household and Institutional Furniture and Kitchen Cabinet Manufacturing
    HouseholdAndInstitutionalFurnitureAndKitchenCabinetManufacturing,
    /// Office Furniture (including Fixtures) Manufacturing
    OfficeFurnitureIncludingFixturesManufacturing,
    /// Other Furniture Related Product Manufacturing
    OtherFurnitureRelatedProductManufacturing,
    /// Medical Equipment and Supplies Manufacturing
    MedicalEquipmentAndSuppliesManufacturing,
    /// Other Miscellaneous Manufacturing
    OtherMiscellaneousManufacturing,
    /// Motor Vehicle and Motor Vehicle Parts and Supplies Merchant Wholesalers
    MotorVehicleAndMotorVehiclePartsAndSuppliesMerchantWholesalers,
    /// Furniture and Home Furnishing Merchant Wholesalers
    FurnitureAndHomeFurnishingMerchantWholesalers,
    /// Lumber and Other Construction Materials Merchant Wholesalers
    LumberAndOtherConstructionMaterialsMerchantWholesalers,
    /// Professional and Commercial Equipment and Supplies Merchant Wholesalers
    ProfessionalAndCommercialEquipmentAndSuppliesMerchantWholesalers,
    /// Metal and Mineral (except Petroleum) Merchant Wholesalers
    MetalAndMineralExceptPetroleumMerchantWholesalers,
    /// Household Appliances and Electrical and Electronic Goods Merchant Wholesalers
    HouseholdAppliancesAndElectricalAndElectronicGoodsMerchantWholesalers,
    /// Hardware, and Plumbing and Heating Equipment and Supplies Merchant Wholesalers
    HardwareAndPlumbingAndHeatingEquipmentAndSuppliesMerchantWholesalers,
    /// Machinery, Equipment, and Supplies Merchant Wholesalers
    MachineryEquipmentAndSuppliesMerchantWholesalers,
    /// Miscellaneous Durable Goods Merchant Wholesalers
    MiscellaneousDurableGoodsMerchantWholesalers,
    /// Paper and Paper Product Merchant Wholesalers
    PaperAndPaperProductMerchantWholesalers,
    /// Drugs and Druggists' Sundries Merchant Wholesalers
    DrugsAndDruggistsSundriesMerchantWholesalers,
    /// Apparel, Piece Goods, and Notions Merchant Wholesalers
    ApparelPieceGoodsAndNotionsMerchantWholesalers,
    /// Grocery and Related Product Merchant Wholesalers
    GroceryAndRelatedProductMerchantWholesalers,
    /// Farm Product Raw Material Merchant Wholesalers
    FarmProductRawMaterialMerchantWholesalers,
    /// Chemical and Allied Products Merchant Wholesalers
    ChemicalAndAlliedProductsMerchantWholesalers,
    /// Petroleum and Petroleum Products Merchant Wholesalers
    PetroleumAndPetroleumProductsMerchantWholesalers,
    /// Beer, Wine, and Distilled Alcoholic Beverage Merchant Wholesalers
    BeerWineAndDistilledAlcoholicBeverageMerchantWholesalers,
    /// Miscellaneous Nondurable Goods Merchant Wholesalers
    MiscellaneousNondurableGoodsMerchantWholesalers,
    /// Wholesale Trade Agents and Brokers
    WholesaleTradeAgentsAndBrokers,
    /// Automobile Dealers
    AutomobileDealers,
    /// Other Motor Vehicle Dealers
    OtherMotorVehicleDealers,
    /// Automotive Parts, Accessories, and Tire Retailers
    AutomotivePartsAccessoriesAndTireRetailers,
    /// Building Material and Supplies Dealers
    BuildingMaterialAndSuppliesDealers,
    /// Lawn and Garden Equipment and Supplies Retailers
    LawnAndGardenEquipmentAndSuppliesRetailers,
    /// Grocery and Convenience Retailers
    GroceryAndConvenienceRetailers,
    /// Specialty Food Retailers
    SpecialtyFoodRetailers,
    /// Beer, Wine, and Liquor Retailers
    BeerWineAndLiquorRetailers,
    /// Furniture and Home Furnishings Retailers
    FurnitureAndHomeFurnishingsRetailers,
    /// Electronics and Appliance Retailers
    ElectronicsAndApplianceRetailers,
    /// Department Stores
    DepartmentStores,
    /// Warehouse Clubs, Supercenters, and Other General Merchandise Retailers
    WarehouseClubsSupercentersAndOtherGeneralMerchandiseRetailers,
    /// Health and Personal Care Retailers
    HealthAndPersonalCareRetailers,
    /// Gasoline Stations
    GasolineStations,
    /// Fuel Dealers
    FuelDealers,
    /// Clothing and Clothing Accessories Retailers
    ClothingAndClothingAccessoriesRetailers,
    /// Shoe Retailers
    ShoeRetailers,
    /// Jewelry, Luggage, and Leather Goods Retailers
    JewelryLuggageAndLeatherGoodsRetailers,
    /// Sporting Goods and Musical Instrument Retailers
    SportingGoodsAndMusicalInstrumentRetailers,
    /// Book Retailers and News Dealers
    BookRetailersAndNewsDealers,
    /// Florists
    Florists,
    /// Office Supplies, Stationery, and Gift Retailers
    OfficeSuppliesStationeryAndGiftRetailers,
    /// Used Merchandise Retailers
    UsedMerchandiseRetailers,
    /// Other Miscellaneous Retailers
    OtherMiscellaneousRetailers,
    /// Scheduled Air Transportation
    ScheduledAirTransportation,
    /// Nonscheduled Air Transportation
    NonscheduledAirTransportation,
    /// Rail Transportation
    RailTransportation,
    /// Deep Sea, Coastal, and Great Lakes Water Transportation
    DeepSeaCoastalAndGreatLakesWaterTransportation,
    /// Inland Water Transportation
    InlandWaterTransportation,
    /// General Freight Trucking
    GeneralFreightTrucking,
    /// Specialized Freight Trucking
    SpecializedFreightTrucking,
    /// Urban Transit Systems
    UrbanTransitSystems,
    /// Interurban and Rural Bus Transportation
    InterurbanAndRuralBusTransportation,
    /// Taxi and Limousine Service
    TaxiAndLimousineService,
    /// School and Employee Bus Transportation
    SchoolAndEmployeeBusTransportation,
    /// Charter Bus Industry
    CharterBusIndustry,
    /// Other Transit and Ground Passenger Transportation
    OtherTransitAndGroundPassengerTransportation,
    /// Pipeline Transportation of Crude Oil
    PipelineTransportationOfCrudeOil,
    /// Pipeline Transportation of Natural Gas
    PipelineTransportationOfNaturalGas,
    /// Other Pipeline Transportation
    OtherPipelineTransportation,
    /// Scenic and Sightseeing Transportation, Land
    ScenicAndSightseeingTransportationLand,
    /// Scenic and Sightseeing Transportation, Water
    ScenicAndSightseeingTransportationWater,
    /// Scenic and Sightseeing Transportation, Other
    ScenicAndSightseeingTransportationOther,
    /// Support Activities for Air Transportation
    SupportActivitiesForAirTransportation,
    /// Support Activities for Rail Transportation
    SupportActivitiesForRailTransportation,
    /// Support Activities for Water Transportation
    SupportActivitiesForWaterTransportation,
    /// Support Activities for Road Transportation
    SupportActivitiesForRoadTransportation,
    /// Freight Transportation Arrangement
    FreightTransportationArrangement,
    /// Other Support Activities for Transportation
    OtherSupportActivitiesForTransportation,
    /// Postal Service
    PostalService,
    /// Couriers and Express Delivery Services
    CouriersAndExpressDeliveryServices,
    /// Local Messengers and Local Delivery
    LocalMessengersAndLocalDelivery,
    /// Warehousing and Storage
    WarehousingAndStorage,
    /// Newspaper, Periodical, Book, and Directory Publishers
    NewspaperPeriodicalBookAndDirectoryPublishers,
    /// Software Publishers
    SoftwarePublishers,
    /// Motion Picture and Video Industries
    MotionPictureAndVideoIndustries,
    /// Sound Recording Industries
    SoundRecordingIndustries,
    /// Radio and Television Broadcasting
    RadioAndTelevisionBroadcasting,
    /// Cable and Other Subscription Programming
    CableAndOtherSubscriptionProgramming,
    /// Wired and Wireless Telecommunications Carriers
    WiredAndWirelessTelecommunicationsCarriers,
    /// Satellite Telecommunications
    SatelliteTelecommunications,
    /// Other Telecommunications
    OtherTelecommunications,
    /// Computing Infrastructure Providers, Data Processing, Web Hosting, and Related Services
    ComputingInfrastructureProvidersDataProcessingWebHostingAndRelatedServices,
    /// Other Information Services
    OtherInformationServices,
    /// Monetary Authorities-Central Bank
    MonetaryAuthoritiesCentralBank,
    /// Depository Credit Intermediation
    DepositoryCreditIntermediation,
    /// Nondepository Credit Intermediation
    NondepositoryCreditIntermediation,
    /// Activities Related to Credit Intermediation
    ActivitiesRelatedToCreditIntermediation,
    /// Securities and Commodity Contracts Intermediation and Brokerage
    SecuritiesAndCommodityContractsIntermediationAndBrokerage,
    /// Securities and Commodity Exchanges
    SecuritiesAndCommodityExchanges,
    /// Other Financial Investment Activities
    OtherFinancialInvestmentActivities,
    /// Insurance Carriers
    InsuranceCarriers,
    /// Agencies, Brokerages, and Other Insurance Related Activities
    AgenciesBrokeragesAndOtherInsuranceRelatedActivities,
    /// Insurance and Employee Benefit Funds
    InsuranceAndEmployeeBenefitFunds,
    /// Other Investment Pools and Funds
    OtherInvestmentPoolsAndFunds,
    /// Lessors of Real Estate
    LessorsOfRealEstate,
    /// Offices of Real Estate Agents and Brokers
    OfficesOfRealEstateAgentsAndBrokers,
    /// Activities Related to Real Estate
    ActivitiesRelatedToRealEstate,
    /// Automotive Equipment Rental and Leasing
    AutomotiveEquipmentRentalAndLeasing,
    /// Consumer Goods Rental
    ConsumerGoodsRental,
    /// General Rental Centers
    GeneralRentalCenters,
    /// Commercial and Industrial Machinery and Equipment Rental and Leasing
    CommercialAndIndustrialMachineryAndEquipmentRentalAndLeasing,
    /// Lessors of Nonfinancial Intangible Assets (except Copyrighted Works)
    LessorsOfNonfinancialIntangibleAssetsExceptCopyrightedWorks,
    /// Legal Services
    LegalServices,
    /// Accounting, Tax Preparation, Bookkeeping, and Payroll Services
    AccountingTaxPreparationBookkeepingAndPayrollServices,
    /// Architectural, Engineering, and Related Services
    ArchitecturalEngineeringAndRelatedServices,
    /// Specialized Design Services
    SpecializedDesignServices,
    /// Computer Systems Design and Related Services
    ComputerSystemsDesignAndRelatedServices,
    /// Management, Scientific, and Technical Consulting Services
    ManagementScientificAndTechnicalConsultingServices,
    /// Scientific Research and Development Services
    ScientificResearchAndDevelopmentServices,
    /// Advertising, Public Relations, and Related Services
    AdvertisingPublicRelationsAndRelatedServices,
    /// Other Professional, Scientific, and Technical Services
    OtherProfessionalScientificAndTechnicalServices,
    /// Management of Companies and Enterprises
    ManagementOfCompaniesAndEnterprises,
    /// Office Administrative Services
    OfficeAdministrativeServices,
    /// Facilities Support Services
    FacilitiesSupportServices,
    /// Employment Services
    EmploymentServices,
    /// Business Support Services
    BusinessSupportServices,
    /// Travel Arrangement and Reservation Services
    TravelArrangementAndReservationServices,
    /// Investigation and Security Services
    InvestigationAndSecurityServices,
    /// Services to Buildings and Dwellings
    ServicesToBuildingsAndDwellings,
    /// Other Support Services
    OtherSupportServices,
    /// Waste Collection
    WasteCollection,
    /// Waste Treatment and Disposal
    WasteTreatmentAndDisposal,
    /// Remediation and Other Waste Management Services
    RemediationAndOtherWasteManagementServices,
    /// Elementary and Secondary Schools
    ElementaryAndSecondarySchools,
    /// Junior Colleges
    JuniorColleges,
    /// Colleges, Universities, and Professional Schools
    CollegesUniversitiesAndProfessionalSchools,
    /// Business Schools and Computer and Management Training
    BusinessSchoolsAndComputerAndManagementTraining,
    /// Technical and Trade Schools
    TechnicalAndTradeSchools,
    /// Other Schools and Instruction
    OtherSchoolsAndInstruction,
    /// Educational Support Services
    EducationalSupportServices,
    /// Offices of Physicians
    OfficesOfPhysicians,
    /// Offices of Dentists
    OfficesOfDentists,
    /// Offices of Other Health Practitioners
    OfficesOfOtherHealthPractitioners,
    /// Outpatient Care Centers
    OutpatientCareCenters,
    /// Medical and Diagnostic Laboratories
    MedicalAndDiagnosticLaboratories,
    /// Home Health Care Services
    HomeHealthCareServices,
    /// Other Ambulatory Health Care Services
    OtherAmbulatoryHealthCareServices,
    /// General Medical and Surgical Hospitals
    GeneralMedicalAndSurgicalHospitals,
    /// Psychiatric and Substance Abuse Hospitals
    PsychiatricAndSubstanceAbuseHospitals,
    /// Specialty (except Psychiatric and Substance Abuse) Hospitals
    SpecialtyExceptPsychiatricAndSubstanceAbuseHospitals,
    /// Nursing Care Facilities (Skilled Nursing Facilities)
    NursingCareFacilitiesSkilledNursingFacilities,
    /// Residential Intellectual and Developmental Disability, Mental Health, and Substance Abuse Facilities
    ResidentialIntellectualAndDevelopmentalDisabilityMentalHealthAndSubstanceAbuseFacilities,
    /// Continuing Care Retirement Communities and Assisted Living Facilities for the Elderly
    ContinuingCareRetirementCommunitiesAndAssistedLivingFacilitiesForTheElderly,
    /// Other Residential Care Facilities
    OtherResidentialCareFacilities,
    /// Individual and Family Services
    IndividualAndFamilyServices,
    /// Community Food and Housing, and Emergency and Other Relief Services
    CommunityFoodAndHousingAndEmergencyAndOtherReliefServices,
    /// Vocational Rehabilitation Services
    VocationalRehabilitationServices,
    /// Child Day Care Services
    ChildDayCareServices,
    /// Performing Arts Companies
    PerformingArtsCompanies,
    /// Spectator Sports
    SpectatorSports,
    /// Promoters of Performing Arts, Sports, and Similar Events
    PromotersOfPerformingArtsSportsAndSimilarEvents,
    /// Agents and Managers for Artists, Athletes, Entertainers, and Other Public Figures
    AgentsAndManagersForArtistsAthletesEntertainersAndOtherPublicFigures,
    /// Independent Artists, Writers, and Performers
    IndependentArtistsWritersAndPerformers,
    /// Museums, Historical Sites, and Similar Institutions
    MuseumsHistoricalSitesAndSimilarInstitutions,
    /// Amusement Parks and Arcades
    AmusementParksAndArcades,
    /// Gambling Industries
    GamblingIndustries,
    /// Other Amusement and Recreation Industries
    OtherAmusementAndRecreationIndustries,
    /// Traveler Accommodation
    TravelerAccommodation,
    /// RV (Recreational Vehicle) Parks and Recreational Camps
    RvRecreationalVehicleParksAndRecreationalCamps,
    /// Rooming and Boarding Houses
    RoomingAndBoardingHouses,
    /// Special Food Services
    SpecialFoodServices,
    /// Drinking Places (Alcoholic Beverages)
    DrinkingPlacesAlcoholicBeverages,
    /// Restaurants and Other Eating Places
    RestaurantsAndOtherEatingPlaces,
    /// Automotive Repair and Maintenance
    AutomotiveRepairAndMaintenance,
    /// Electronic and Precision Equipment Repair and Maintenance
    ElectronicAndPrecisionEquipmentRepairAndMaintenance,
    /// Commercial and Industrial Machinery and Equipment (except Automotive and Electronic) Repair and Maintenance
    CommercialAndIndustrialMachineryAndEquipmentExceptAutomotiveAndElectronicRepairAndMaintenance,
    /// Personal and Household Goods Repair and Maintenance
    PersonalAndHouseholdGoodsRepairAndMaintenance,
    /// Personal Care Services
    PersonalCareServices,
    /// Death Care Services
    DeathCareServices,
    /// Drycleaning and Laundry Services
    DrycleaningAndLaundryServices,
    /// Other Personal Services
    OtherPersonalServices,
    /// Religious Organizations
    ReligiousOrganizations,
    /// Grantmaking and Giving Services
    GrantmakingAndGivingServices,
    /// Social Advocacy Organizations
    SocialAdvocacyOrganizations,
    /// Civic and Social Organizations
    CivicAndSocialOrganizations,
    /// Business, Professional, Labor, Political, and Similar Organizations
    BusinessProfessionalLaborPoliticalAndSimilarOrganizations,
    /// Private Households
    PrivateHouseholds,
    /// Executive, Legislative, and Other General Government Support
    ExecutiveLegislativeAndOtherGeneralGovernmentSupport,
    /// Justice, Public Order, and Safety Activities
    JusticePublicOrderAndSafetyActivities,
    /// Administration of Human Resource Programs
    AdministrationOfHumanResourcePrograms,
    /// Administration of Environmental Quality Programs
    AdministrationOfEnvironmentalQualityPrograms,
    /// Administration of Housing Programs, Urban Planning, and Community Development
    AdministrationOfHousingProgramsUrbanPlanningAndCommunityDevelopment,
    /// Administration of Economic Programs
    AdministrationOfEconomicPrograms,
    /// Space Research and Technology
    SpaceResearchAndTechnology,
    /// National Security and International Affairs
    NationalSecurityAndInternationalAffairs,
    /// Unclassified Establishments
    UnclassifiedEstablishments,
}

impl NaicsCategory {
    /// Returns the description (NAICS title) for the category
    pub fn description(&self) -> &'static str {
        match self {
            Self::OilseedAndGrainFarming => "Oilseed and Grain Farming",
            Self::VegetableAndMelonFarming => "Vegetable and Melon Farming",
            Self::FruitAndTreeNutFarming => "Fruit and Tree Nut Farming",
            Self::GreenhouseNurseryAndFloricultureProduction => "Greenhouse, Nursery, and Floriculture Production",
            Self::OtherCropFarming => "Other Crop Farming",
            Self::CattleRanchingAndFarming => "Cattle Ranching and Farming",
            Self::HogAndPigFarming => "Hog and Pig Farming",
            Self::PoultryAndEggProduction => "Poultry and Egg Production",
            Self::SheepAndGoatFarming => "Sheep and Goat Farming",
            Self::Aquaculture => "Aquaculture",
            Self::OtherAnimalProduction => "Other Animal Production",
            Self::TimberTractOperations => "Timber Tract Operations",
            Self::ForestNurseriesAndGatheringOfForestProducts => "Forest Nurseries and Gathering of Forest Products",
            Self::Logging => "Logging",
            Self::Fishing => "Fishing",
            Self::HuntingAndTrapping => "Hunting and Trapping",
            Self::SupportActivitiesForCropProduction => "Support Activities for Crop Production",
            Self::SupportActivitiesForAnimalProduction => "Support Activities for Animal Production",
            Self::SupportActivitiesForForestry => "Support Activities for Forestry",
            Self::OilAndGasExtraction => "Oil and Gas Extraction",
            Self::CoalMining => "Coal Mining",
            Self::MetalOreMining => "Metal Ore Mining",
            Self::NonmetallicMineralMiningAndQuarrying => "Nonmetallic Mineral Mining and Quarrying",
            Self::SupportActivitiesForMining => "Support Activities for Mining",
            Self::ElectricPowerGenerationTransmissionAndDistribution => "Electric Power Generation, Transmission and Distribution",
            Self::NaturalGasDistribution => "Natural Gas Distribution",
            Self::WaterSewageAndOtherSystems => "Water, Sewage and Other Systems",
            Self::ResidentialBuildingConstruction => "Residential Building Construction",
            Self::NonresidentialBuildingConstruction => "Nonresidential Building Construction",
            Self::UtilitySystemConstruction => "Utility System Construction",
            Self::LandSubdivision => "Land Subdivision",
            Self::HighwayStreetAndBridgeConstruction => "Highway, Street, and Bridge Construction",
            Self::OtherHeavyAndCivilEngineeringConstruction => "Other Heavy and Civil Engineering Construction",
            Self::FoundationStructureAndBuildingExteriorContractors => "Foundation, Structure, and Building Exterior Contractors",
            Self::BuildingEquipmentContractors => "Building Equipment Contractors",
            Self::BuildingFinishingContractors => "Building Finishing Contractors",
            Self::OtherSpecialtyTradeContractors => "Other Specialty Trade Contractors",
            Self::AnimalFoodManufacturing => "Animal Food Manufacturing",
            Self::GrainAndOilseedMilling => "Grain and Oilseed Milling",
            Self::SugarAndConfectioneryProductManufacturing => "Sugar and Confectionery Product Manufacturing",
            Self::FruitAndVegetablePreservingAndSpecialtyFoodManufacturing => "Fruit and Vegetable Preserving and Specialty Food Manufacturing",
            Self::DairyProductManufacturing => "Dairy Product Manufacturing",
            Self::AnimalSlaughteringAndProcessing => "Animal Slaughtering and Processing",
            Self::SeafoodProductPreparationAndPackaging => "Seafood Product Preparation and Packaging",
            Self::BakeriesAndTortillaManufacturing => "Bakeries and Tortilla Manufacturing",
            Self::OtherFoodManufacturing => "Other Food Manufacturing",
            Self::BeverageManufacturing => "Beverage Manufacturing",
            Self::TobaccoManufacturing => "Tobacco Manufacturing",
            Self::FiberYarnAndThreadMills => "Fiber, Yarn, and Thread Mills",
            Self::FabricMills => "Fabric Mills",
            Self::TextileAndFabricFinishingAndFabricCoatingMills => "Textile and Fabric Finishing and Fabric Coating Mills",
            Self::TextileFurnishingsMills => "Textile Furnishings Mills",
            Self::OtherTextileProductMills => "Other Textile Product Mills",
            Self::ApparelKnittingMills => "Apparel Knitting Mills",
            Self::CutAndSewApparelManufacturing => "Cut and Sew Apparel Manufacturing",
            Self::ApparelAccessoriesAndOtherApparelManufacturing => "Apparel Accessories and Other Apparel Manufacturing",
            Self::LeatherAndHideTanningAndFinishing => "Leather and Hide Tanning and Finishing",
            Self::FootwearManufacturing => "Footwear Manufacturing",
            Self::OtherLeatherAndAlliedProductManufacturing => "Other Leather and Allied Product Manufacturing",
            Self::SawmillsAndWoodPreservation => "Sawmills and Wood Preservation",
            Self::VeneerPlywoodAndEngineeredWoodProductManufacturing => "Veneer, Plywood, and Engineered Wood Product Manufacturing",
            Self::OtherWoodProductManufacturing => "Other Wood Product Manufacturing",
            Self::PulpPaperAndPaperboardMills => "Pulp, Paper, and Paperboard Mills",
            Self::ConvertedPaperProductManufacturing => "Converted Paper Product Manufacturing",
            Self::PrintingAndRelatedSupportActivities => "Printing and Related Support Activities",
            Self::PetroleumAndCoalProductsManufacturing => "Petroleum and Coal Products Manufacturing",
            Self::BasicChemicalManufacturing => "Basic Chemical Manufacturing",
            Self::ResinSyntheticRubberAndArtificialAndSyntheticFibersAndFilamentsManufacturing => "Resin, Synthetic Rubber, and Artificial and Synthetic Fibers and Filaments Manufacturing",
            Self::PesticideFertilizerAndOtherAgriculturalChemicalManufacturing => "Pesticide, Fertilizer, and Other Agricultural Chemical Manufacturing",
            Self::PharmaceuticalAndMedicineManufacturing => "Pharmaceutical and Medicine Manufacturing",
            Self::PaintCoatingAndAdhesiveManufacturing => "Paint, Coating, and Adhesive Manufacturing",
            Self::SoapCleaningCompoundAndToiletPreparationManufacturing => "Soap, Cleaning Compound, and Toilet Preparation Manufacturing",
            Self::OtherChemicalProductAndPreparationManufacturing => "Other Chemical Product and Preparation Manufacturing",
            Self::PlasticsProductManufacturing => "Plastics Product Manufacturing",
            Self::RubberProductManufacturing => "Rubber Product Manufacturing",
            Self::ClayProductAndRefractoryManufacturing => "Clay Product and Refractory Manufacturing",
            Self::GlassAndGlassProductManufacturing => "Glass and Glass Product Manufacturing",
            Self::CementAndConcreteProductManufacturing => "Cement and Concrete Product Manufacturing",
            Self::LimeAndGypsumProductManufacturing => "Lime and Gypsum Product Manufacturing",
            Self::OtherNonmetallicMineralProductManufacturing => "Other Nonmetallic Mineral Product Manufacturing",
            Self::IronAndSteelMillsAndFerroalloyManufacturing => "Iron and Steel Mills and Ferroalloy Manufacturing",
            Self::SteelProductManufacturingFromPurchasedSteel => "Steel Product Manufacturing from Purchased Steel",
            Self::AluminaAndAluminumProductionAndProcessing => "Alumina and Aluminum Production and Processing",
            Self::NonferrousMetalExceptAluminumProductionAndProcessing => "Nonferrous Metal (except Aluminum) Production and Processing",
            Self::Foundries => "Foundries",
            Self::ForgingAndStamping => "Forging and Stamping",
            Self::CutleryAndHandtoolManufacturing => "Cutlery and Handtool Manufacturing",
            Self::ArchitecturalAndStructuralMetalsManufacturing => "Architectural and Structural Metals Manufacturing",
            Self::BoilerTankAndShippingContainerManufacturing => "Boiler, Tank, and Shipping Container Manufacturing",
            Self::HardwareManufacturing => "Hardware Manufacturing",
            Self::SpringAndWireProductManufacturing => "Spring and Wire Product Manufacturing",
            Self::MachineShopsTurnedProductAndScrewNutAndBoltManufacturing => "Machine Shops; Turned Product; and Screw, Nut, and Bolt Manufacturing",
            Self::CoatingEngravingHeatTreatingAndAlliedActivities => "Coating, Engraving, Heat Treating, and Allied Activities",
            Self::OtherFabricatedMetalProductManufacturing => "Other Fabricated Metal Product Manufacturing",
            Self::AgricultureConstructionAndMiningMachineryManufacturing => "Agriculture, Construction, and Mining Machinery Manufacturing",
            Self::IndustrialMachineryManufacturing => "Industrial Machinery Manufacturing",
            Self::CommercialAndServiceIndustryMachineryManufacturing => "Commercial and Service Industry Machinery Manufacturing",
            Self::VentilationHeatingAirconditioningAndCommercialRefrigerationEquipmentManufacturing => "Ventilation, Heating, Air-Conditioning, and Commercial Refrigeration Equipment Manufacturing",
            Self::MetalworkingMachineryManufacturing => "Metalworking Machinery Manufacturing",
            Self::EngineTurbineAndPowerTransmissionEquipmentManufacturing => "Engine, Turbine, and Power Transmission Equipment Manufacturing",
            Self::OtherGeneralPurposeMachineryManufacturing => "Other General Purpose Machinery Manufacturing",
            Self::ComputerAndPeripheralEquipmentManufacturing => "Computer and Peripheral Equipment Manufacturing",
            Self::CommunicationsEquipmentManufacturing => "Communications Equipment Manufacturing",
            Self::AudioAndVideoEquipmentManufacturing => "Audio and Video Equipment Manufacturing",
            Self::SemiconductorAndOtherElectronicComponentManufacturing => "Semiconductor and Other Electronic Component Manufacturing",
            Self::NavigationalMeasuringElectromedicalAndControlInstrumentsManufacturing => "Navigational, Measuring, Electromedical, and Control Instruments Manufacturing",
            Self::ManufacturingAndReproducingMagneticAndOpticalMedia => "Manufacturing and Reproducing Magnetic and Optical Media",
            Self::ElectricLightingEquipmentManufacturing => "Electric Lighting Equipment Manufacturing",
            Self::HouseholdApplianceManufacturing => "Household Appliance Manufacturing",
            Self::ElectricalEquipmentManufacturing => "Electrical Equipment Manufacturing",
            Self::OtherElectricalEquipmentAndComponentManufacturing => "Other Electrical Equipment and Component Manufacturing",
            Self::MotorVehicleManufacturing => "Motor Vehicle Manufacturing",
            Self::MotorVehicleBodyAndTrailerManufacturing => "Motor Vehicle Body and Trailer Manufacturing",
            Self::MotorVehiclePartsManufacturing => "Motor Vehicle Parts Manufacturing",
            Self::AerospaceProductAndPartsManufacturing => "Aerospace Product and Parts Manufacturing",
            Self::RailroadRollingStockManufacturing => "Railroad Rolling Stock Manufacturing",
            Self::ShipAndBoatBuilding => "Ship and Boat Building",
            Self::OtherTransportationEquipmentManufacturing => "Other Transportation Equipment Manufacturing",
            Self::HouseholdAndInstitutionalFurnitureAndKitchenCabinetManufacturing => "Household and Institutional Furniture and Kitchen Cabinet Manufacturing",
            Self::OfficeFurnitureIncludingFixturesManufacturing => "Office Furniture (including Fixtures) Manufacturing",
            Self::OtherFurnitureRelatedProductManufacturing => "Other Furniture Related Product Manufacturing",
            Self::MedicalEquipmentAndSuppliesManufacturing => "Medical Equipment and Supplies Manufacturing",
            Self::OtherMiscellaneousManufacturing => "Other Miscellaneous Manufacturing",
            Self::MotorVehicleAndMotorVehiclePartsAndSuppliesMerchantWholesalers => "Motor Vehicle and Motor Vehicle Parts and Supplies Merchant Wholesalers",
            Self::FurnitureAndHomeFurnishingMerchantWholesalers => "Furniture and Home Furnishing Merchant Wholesalers",
            Self::LumberAndOtherConstructionMaterialsMerchantWholesalers => "Lumber and Other Construction Materials Merchant Wholesalers",
            Self::ProfessionalAndCommercialEquipmentAndSuppliesMerchantWholesalers => "Professional and Commercial Equipment and Supplies Merchant Wholesalers",
            Self::MetalAndMineralExceptPetroleumMerchantWholesalers => "Metal and Mineral (except Petroleum) Merchant Wholesalers",
            Self::HouseholdAppliancesAndElectricalAndElectronicGoodsMerchantWholesalers => "Household Appliances and Electrical and Electronic Goods Merchant Wholesalers",
            Self::HardwareAndPlumbingAndHeatingEquipmentAndSuppliesMerchantWholesalers => "Hardware, and Plumbing and Heating Equipment and Supplies Merchant Wholesalers",
            Self::MachineryEquipmentAndSuppliesMerchantWholesalers => "Machinery, Equipment, and Supplies Merchant Wholesalers",
            Self::MiscellaneousDurableGoodsMerchantWholesalers => "Miscellaneous Durable Goods Merchant Wholesalers",
            Self::PaperAndPaperProductMerchantWholesalers => "Paper and Paper Product Merchant Wholesalers",
            Self::DrugsAndDruggistsSundriesMerchantWholesalers => "Drugs and Druggists' Sundries Merchant Wholesalers",
            Self::ApparelPieceGoodsAndNotionsMerchantWholesalers => "Apparel, Piece Goods, and Notions Merchant Wholesalers",
            Self::GroceryAndRelatedProductMerchantWholesalers => "Grocery and Related Product Merchant Wholesalers",
            Self::FarmProductRawMaterialMerchantWholesalers => "Farm Product Raw Material Merchant Wholesalers",
            Self::ChemicalAndAlliedProductsMerchantWholesalers => "Chemical and Allied Products Merchant Wholesalers",
            Self::PetroleumAndPetroleumProductsMerchantWholesalers => "Petroleum and Petroleum Products Merchant Wholesalers",
            Self::BeerWineAndDistilledAlcoholicBeverageMerchantWholesalers => "Beer, Wine, and Distilled Alcoholic Beverage Merchant Wholesalers",
            Self::MiscellaneousNondurableGoodsMerchantWholesalers => "Miscellaneous Nondurable Goods Merchant Wholesalers",
            Self::WholesaleTradeAgentsAndBrokers => "Wholesale Trade Agents and Brokers",
            Self::AutomobileDealers => "Automobile Dealers",
            Self::OtherMotorVehicleDealers => "Other Motor Vehicle Dealers",
            Self::AutomotivePartsAccessoriesAndTireRetailers => "Automotive Parts, Accessories, and Tire Retailers",
            Self::BuildingMaterialAndSuppliesDealers => "Building Material and Supplies Dealers",
            Self::LawnAndGardenEquipmentAndSuppliesRetailers => "Lawn and Garden Equipment and Supplies Retailers",
            Self::GroceryAndConvenienceRetailers => "Grocery and Convenience Retailers",
            Self::SpecialtyFoodRetailers => "Specialty Food Retailers",
            Self::BeerWineAndLiquorRetailers => "Beer, Wine, and Liquor Retailers",
            Self::FurnitureAndHomeFurnishingsRetailers => "Furniture and Home Furnishings Retailers",
            Self::ElectronicsAndApplianceRetailers => "Electronics and Appliance Retailers",
            Self::DepartmentStores => "Department Stores",
            Self::WarehouseClubsSupercentersAndOtherGeneralMerchandiseRetailers => "Warehouse Clubs, Supercenters, and Other General Merchandise Retailers",
            Self::HealthAndPersonalCareRetailers => "Health and Personal Care Retailers",
            Self::GasolineStations => "Gasoline Stations",
            Self::FuelDealers => "Fuel Dealers",
            Self::ClothingAndClothingAccessoriesRetailers => "Clothing and Clothing Accessories Retailers",
            Self::ShoeRetailers => "Shoe Retailers",
            Self::JewelryLuggageAndLeatherGoodsRetailers => "Jewelry, Luggage, and Leather Goods Retailers",
            Self::SportingGoodsAndMusicalInstrumentRetailers => "Sporting Goods and Musical Instrument Retailers",
            Self::BookRetailersAndNewsDealers => "Book Retailers and News Dealers",
            Self::Florists => "Florists",
            Self::OfficeSuppliesStationeryAndGiftRetailers => "Office Supplies, Stationery, and Gift Retailers",
            Self::UsedMerchandiseRetailers => "Used Merchandise Retailers",
            Self::OtherMiscellaneousRetailers => "Other Miscellaneous Retailers",
            Self::ScheduledAirTransportation => "Scheduled Air Transportation",
            Self::NonscheduledAirTransportation => "Nonscheduled Air Transportation",
            Self::RailTransportation => "Rail Transportation",
            Self::DeepSeaCoastalAndGreatLakesWaterTransportation => "Deep Sea, Coastal, and Great Lakes Water Transportation",
            Self::InlandWaterTransportation => "Inland Water Transportation",
            Self::GeneralFreightTrucking => "General Freight Trucking",
            Self::SpecializedFreightTrucking => "Specialized Freight Trucking",
            Self::UrbanTransitSystems => "Urban Transit Systems",
            Self::InterurbanAndRuralBusTransportation => "Interurban and Rural Bus Transportation",
            Self::TaxiAndLimousineService => "Taxi and Limousine Service",
            Self::SchoolAndEmployeeBusTransportation => "School and Employee Bus Transportation",
            Self::CharterBusIndustry => "Charter Bus Industry",
            Self::OtherTransitAndGroundPassengerTransportation => "Other Transit and Ground Passenger Transportation",
            Self::PipelineTransportationOfCrudeOil => "Pipeline Transportation of Crude Oil",
            Self::PipelineTransportationOfNaturalGas => "Pipeline Transportation of Natural Gas",
            Self::OtherPipelineTransportation => "Other Pipeline Transportation",
            Self::ScenicAndSightseeingTransportationLand => "Scenic and Sightseeing Transportation, Land",
            Self::ScenicAndSightseeingTransportationWater => "Scenic and Sightseeing Transportation, Water",
            Self::ScenicAndSightseeingTransportationOther => "Scenic and Sightseeing Transportation, Other",
            Self::SupportActivitiesForAirTransportation => "Support Activities for Air Transportation",
            Self::SupportActivitiesForRailTransportation => "Support Activities for Rail Transportation",
            Self::SupportActivitiesForWaterTransportation => "Support Activities for Water Transportation",
            Self::SupportActivitiesForRoadTransportation => "Support Activities for Road Transportation",
            Self::FreightTransportationArrangement => "Freight Transportation Arrangement",
            Self::OtherSupportActivitiesForTransportation => "Other Support Activities for Transportation",
            Self::PostalService => "Postal Service",
            Self::CouriersAndExpressDeliveryServices => "Couriers and Express Delivery Services",
            Self::LocalMessengersAndLocalDelivery => "Local Messengers and Local Delivery",
            Self::WarehousingAndStorage => "Warehousing and Storage",
            Self::NewspaperPeriodicalBookAndDirectoryPublishers => "Newspaper, Periodical, Book, and Directory Publishers",
            Self::SoftwarePublishers => "Software Publishers",
            Self::MotionPictureAndVideoIndustries => "Motion Picture and Video Industries",
            Self::SoundRecordingIndustries => "Sound Recording Industries",
            Self::RadioAndTelevisionBroadcasting => "Radio and Television Broadcasting",
            Self::CableAndOtherSubscriptionProgramming => "Cable and Other Subscription Programming",
            Self::WiredAndWirelessTelecommunicationsCarriers => "Wired and Wireless Telecommunications Carriers",
            Self::SatelliteTelecommunications => "Satellite Telecommunications",
            Self::OtherTelecommunications => "Other Telecommunications",
            Self::ComputingInfrastructureProvidersDataProcessingWebHostingAndRelatedServices => "Computing Infrastructure Providers, Data Processing, Web Hosting, and Related Services",
            Self::OtherInformationServices => "Other Information Services",
            Self::MonetaryAuthoritiesCentralBank => "Monetary Authorities-Central Bank",
            Self::DepositoryCreditIntermediation => "Depository Credit Intermediation",
            Self::NondepositoryCreditIntermediation => "Nondepository Credit Intermediation",
            Self::ActivitiesRelatedToCreditIntermediation => "Activities Related to Credit Intermediation",
            Self::SecuritiesAndCommodityContractsIntermediationAndBrokerage => "Securities and Commodity Contracts Intermediation and Brokerage",
            Self::SecuritiesAndCommodityExchanges => "Securities and Commodity Exchanges",
            Self::OtherFinancialInvestmentActivities => "Other Financial Investment Activities",
            Self::InsuranceCarriers => "Insurance Carriers",
            Self::AgenciesBrokeragesAndOtherInsuranceRelatedActivities => "Agencies, Brokerages, and Other Insurance Related Activities",
            Self::InsuranceAndEmployeeBenefitFunds => "Insurance and Employee Benefit Funds",
            Self::OtherInvestmentPoolsAndFunds => "Other Investment Pools and Funds",
            Self::LessorsOfRealEstate => "Lessors of Real Estate",
            Self::OfficesOfRealEstateAgentsAndBrokers => "Offices of Real Estate Agents and Brokers",
            Self::ActivitiesRelatedToRealEstate => "Activities Related to Real Estate",
            Self::AutomotiveEquipmentRentalAndLeasing => "Automotive Equipment Rental and Leasing",
            Self::ConsumerGoodsRental => "Consumer Goods Rental",
            Self::GeneralRentalCenters => "General Rental Centers",
            Self::CommercialAndIndustrialMachineryAndEquipmentRentalAndLeasing => "Commercial and Industrial Machinery and Equipment Rental and Leasing",
            Self::LessorsOfNonfinancialIntangibleAssetsExceptCopyrightedWorks => "Lessors of Nonfinancial Intangible Assets (except Copyrighted Works)",
            Self::LegalServices => "Legal Services",
            Self::AccountingTaxPreparationBookkeepingAndPayrollServices => "Accounting, Tax Preparation, Bookkeeping, and Payroll Services",
            Self::ArchitecturalEngineeringAndRelatedServices => "Architectural, Engineering, and Related Services",
            Self::SpecializedDesignServices => "Specialized Design Services",
            Self::ComputerSystemsDesignAndRelatedServices => "Computer Systems Design and Related Services",
            Self::ManagementScientificAndTechnicalConsultingServices => "Management, Scientific, and Technical Consulting Services",
            Self::ScientificResearchAndDevelopmentServices => "Scientific Research and Development Services",
            Self::AdvertisingPublicRelationsAndRelatedServices => "Advertising, Public Relations, and Related Services",
            Self::OtherProfessionalScientificAndTechnicalServices => "Other Professional, Scientific, and Technical Services",
            Self::ManagementOfCompaniesAndEnterprises => "Management of Companies and Enterprises",
            Self::OfficeAdministrativeServices => "Office Administrative Services",
            Self::FacilitiesSupportServices => "Facilities Support Services",
            Self::EmploymentServices => "Employment Services",
            Self::BusinessSupportServices => "Business Support Services",
            Self::TravelArrangementAndReservationServices => "Travel Arrangement and Reservation Services",
            Self::InvestigationAndSecurityServices => "Investigation and Security Services",
            Self::ServicesToBuildingsAndDwellings => "Services to Buildings and Dwellings",
            Self::OtherSupportServices => "Other Support Services",
            Self::WasteCollection => "Waste Collection",
            Self::WasteTreatmentAndDisposal => "Waste Treatment and Disposal",
            Self::RemediationAndOtherWasteManagementServices => "Remediation and Other Waste Management Services",
            Self::ElementaryAndSecondarySchools => "Elementary and Secondary Schools",
            Self::JuniorColleges => "Junior Colleges",
            Self::CollegesUniversitiesAndProfessionalSchools => "Colleges, Universities, and Professional Schools",
            Self::BusinessSchoolsAndComputerAndManagementTraining => "Business Schools and Computer and Management Training",
            Self::TechnicalAndTradeSchools => "Technical and Trade Schools",
            Self::OtherSchoolsAndInstruction => "Other Schools and Instruction",
            Self::EducationalSupportServices => "Educational Support Services",
            Self::OfficesOfPhysicians => "Offices of Physicians",
            Self::OfficesOfDentists => "Offices of Dentists",
            Self::OfficesOfOtherHealthPractitioners => "Offices of Other Health Practitioners",
            Self::OutpatientCareCenters => "Outpatient Care Centers",
            Self::MedicalAndDiagnosticLaboratories => "Medical and Diagnostic Laboratories",
            Self::HomeHealthCareServices => "Home Health Care Services",
            Self::OtherAmbulatoryHealthCareServices => "Other Ambulatory Health Care Services",
            Self::GeneralMedicalAndSurgicalHospitals => "General Medical and Surgical Hospitals",
            Self::PsychiatricAndSubstanceAbuseHospitals => "Psychiatric and Substance Abuse Hospitals",
            Self::SpecialtyExceptPsychiatricAndSubstanceAbuseHospitals => "Specialty (except Psychiatric and Substance Abuse) Hospitals",
            Self::NursingCareFacilitiesSkilledNursingFacilities => "Nursing Care Facilities (Skilled Nursing Facilities)",
            Self::ResidentialIntellectualAndDevelopmentalDisabilityMentalHealthAndSubstanceAbuseFacilities => "Residential Intellectual and Developmental Disability, Mental Health, and Substance Abuse Facilities",
            Self::ContinuingCareRetirementCommunitiesAndAssistedLivingFacilitiesForTheElderly => "Continuing Care Retirement Communities and Assisted Living Facilities for the Elderly",
            Self::OtherResidentialCareFacilities => "Other Residential Care Facilities",
            Self::IndividualAndFamilyServices => "Individual and Family Services",
            Self::CommunityFoodAndHousingAndEmergencyAndOtherReliefServices => "Community Food and Housing, and Emergency and Other Relief Services",
            Self::VocationalRehabilitationServices => "Vocational Rehabilitation Services",
            Self::ChildDayCareServices => "Child Day Care Services",
            Self::PerformingArtsCompanies => "Performing Arts Companies",
            Self::SpectatorSports => "Spectator Sports",
            Self::PromotersOfPerformingArtsSportsAndSimilarEvents => "Promoters of Performing Arts, Sports, and Similar Events",
            Self::AgentsAndManagersForArtistsAthletesEntertainersAndOtherPublicFigures => "Agents and Managers for Artists, Athletes, Entertainers, and Other Public Figures",
            Self::IndependentArtistsWritersAndPerformers => "Independent Artists, Writers, and Performers",
            Self::MuseumsHistoricalSitesAndSimilarInstitutions => "Museums, Historical Sites, and Similar Institutions",
            Self::AmusementParksAndArcades => "Amusement Parks and Arcades",
            Self::GamblingIndustries => "Gambling Industries",
            Self::OtherAmusementAndRecreationIndustries => "Other Amusement and Recreation Industries",
            Self::TravelerAccommodation => "Traveler Accommodation",
            Self::RvRecreationalVehicleParksAndRecreationalCamps => "RV (Recreational Vehicle) Parks and Recreational Camps",
            Self::RoomingAndBoardingHouses => "Rooming and Boarding Houses",
            Self::SpecialFoodServices => "Special Food Services",
            Self::DrinkingPlacesAlcoholicBeverages => "Drinking Places (Alcoholic Beverages)",
            Self::RestaurantsAndOtherEatingPlaces => "Restaurants and Other Eating Places",
            Self::AutomotiveRepairAndMaintenance => "Automotive Repair and Maintenance",
            Self::ElectronicAndPrecisionEquipmentRepairAndMaintenance => "Electronic and Precision Equipment Repair and Maintenance",
            Self::CommercialAndIndustrialMachineryAndEquipmentExceptAutomotiveAndElectronicRepairAndMaintenance => "Commercial and Industrial Machinery and Equipment (except Automotive and Electronic) Repair and Maintenance",
            Self::PersonalAndHouseholdGoodsRepairAndMaintenance => "Personal and Household Goods Repair and Maintenance",
            Self::PersonalCareServices => "Personal Care Services",
            Self::DeathCareServices => "Death Care Services",
            Self::DrycleaningAndLaundryServices => "Drycleaning and Laundry Services",
            Self::OtherPersonalServices => "Other Personal Services",
            Self::ReligiousOrganizations => "Religious Organizations",
            Self::GrantmakingAndGivingServices => "Grantmaking and Giving Services",
            Self::SocialAdvocacyOrganizations => "Social Advocacy Organizations",
            Self::CivicAndSocialOrganizations => "Civic and Social Organizations",
            Self::BusinessProfessionalLaborPoliticalAndSimilarOrganizations => "Business, Professional, Labor, Political, and Similar Organizations",
            Self::PrivateHouseholds => "Private Households",
            Self::ExecutiveLegislativeAndOtherGeneralGovernmentSupport => "Executive, Legislative, and Other General Government Support",
            Self::JusticePublicOrderAndSafetyActivities => "Justice, Public Order, and Safety Activities",
            Self::AdministrationOfHumanResourcePrograms => "Administration of Human Resource Programs",
            Self::AdministrationOfEnvironmentalQualityPrograms => "Administration of Environmental Quality Programs",
            Self::AdministrationOfHousingProgramsUrbanPlanningAndCommunityDevelopment => "Administration of Housing Programs, Urban Planning, and Community Development",
            Self::AdministrationOfEconomicPrograms => "Administration of Economic Programs",
            Self::SpaceResearchAndTechnology => "Space Research and Technology",
            Self::NationalSecurityAndInternationalAffairs => "National Security and International Affairs",
            Self::UnclassifiedEstablishments => "Unclassified Establishments",
        }
    }
    /// Returns the NAICS code for the category as an i64
    pub fn code(&self) -> i64 {
        match self {
            Self::OilseedAndGrainFarming => 1111,
            Self::VegetableAndMelonFarming => 1112,
            Self::FruitAndTreeNutFarming => 1113,
            Self::GreenhouseNurseryAndFloricultureProduction => 1114,
            Self::OtherCropFarming => 1119,
            Self::CattleRanchingAndFarming => 1121,
            Self::HogAndPigFarming => 1122,
            Self::PoultryAndEggProduction => 1123,
            Self::SheepAndGoatFarming => 1124,
            Self::Aquaculture => 1125,
            Self::OtherAnimalProduction => 1129,
            Self::TimberTractOperations => 1131,
            Self::ForestNurseriesAndGatheringOfForestProducts => 1132,
            Self::Logging => 1133,
            Self::Fishing => 1141,
            Self::HuntingAndTrapping => 1142,
            Self::SupportActivitiesForCropProduction => 1151,
            Self::SupportActivitiesForAnimalProduction => 1152,
            Self::SupportActivitiesForForestry => 1153,
            Self::OilAndGasExtraction => 2111,
            Self::CoalMining => 2121,
            Self::MetalOreMining => 2122,
            Self::NonmetallicMineralMiningAndQuarrying => 2123,
            Self::SupportActivitiesForMining => 2131,
            Self::ElectricPowerGenerationTransmissionAndDistribution => 2211,
            Self::NaturalGasDistribution => 2212,
            Self::WaterSewageAndOtherSystems => 2213,
            Self::ResidentialBuildingConstruction => 2361,
            Self::NonresidentialBuildingConstruction => 2362,
            Self::UtilitySystemConstruction => 2371,
            Self::LandSubdivision => 2372,
            Self::HighwayStreetAndBridgeConstruction => 2373,
            Self::OtherHeavyAndCivilEngineeringConstruction => 2379,
            Self::FoundationStructureAndBuildingExteriorContractors => 2381,
            Self::BuildingEquipmentContractors => 2382,
            Self::BuildingFinishingContractors => 2383,
            Self::OtherSpecialtyTradeContractors => 2389,
            Self::AnimalFoodManufacturing => 3111,
            Self::GrainAndOilseedMilling => 3112,
            Self::SugarAndConfectioneryProductManufacturing => 3113,
            Self::FruitAndVegetablePreservingAndSpecialtyFoodManufacturing => 3114,
            Self::DairyProductManufacturing => 3115,
            Self::AnimalSlaughteringAndProcessing => 3116,
            Self::SeafoodProductPreparationAndPackaging => 3117,
            Self::BakeriesAndTortillaManufacturing => 3118,
            Self::OtherFoodManufacturing => 3119,
            Self::BeverageManufacturing => 3121,
            Self::TobaccoManufacturing => 3122,
            Self::FiberYarnAndThreadMills => 3131,
            Self::FabricMills => 3132,
            Self::TextileAndFabricFinishingAndFabricCoatingMills => 3133,
            Self::TextileFurnishingsMills => 3141,
            Self::OtherTextileProductMills => 3149,
            Self::ApparelKnittingMills => 3151,
            Self::CutAndSewApparelManufacturing => 3152,
            Self::ApparelAccessoriesAndOtherApparelManufacturing => 3159,
            Self::LeatherAndHideTanningAndFinishing => 3161,
            Self::FootwearManufacturing => 3162,
            Self::OtherLeatherAndAlliedProductManufacturing => 3169,
            Self::SawmillsAndWoodPreservation => 3211,
            Self::VeneerPlywoodAndEngineeredWoodProductManufacturing => 3212,
            Self::OtherWoodProductManufacturing => 3219,
            Self::PulpPaperAndPaperboardMills => 3221,
            Self::ConvertedPaperProductManufacturing => 3222,
            Self::PrintingAndRelatedSupportActivities => 3231,
            Self::PetroleumAndCoalProductsManufacturing => 3241,
            Self::BasicChemicalManufacturing => 3251,
            Self::ResinSyntheticRubberAndArtificialAndSyntheticFibersAndFilamentsManufacturing => 3252,
            Self::PesticideFertilizerAndOtherAgriculturalChemicalManufacturing => 3253,
            Self::PharmaceuticalAndMedicineManufacturing => 3254,
            Self::PaintCoatingAndAdhesiveManufacturing => 3255,
            Self::SoapCleaningCompoundAndToiletPreparationManufacturing => 3256,
            Self::OtherChemicalProductAndPreparationManufacturing => 3259,
            Self::PlasticsProductManufacturing => 3261,
            Self::RubberProductManufacturing => 3262,
            Self::ClayProductAndRefractoryManufacturing => 3271,
            Self::GlassAndGlassProductManufacturing => 3272,
            Self::CementAndConcreteProductManufacturing => 3273,
            Self::LimeAndGypsumProductManufacturing => 3274,
            Self::OtherNonmetallicMineralProductManufacturing => 3279,
            Self::IronAndSteelMillsAndFerroalloyManufacturing => 3311,
            Self::SteelProductManufacturingFromPurchasedSteel => 3312,
            Self::AluminaAndAluminumProductionAndProcessing => 3313,
            Self::NonferrousMetalExceptAluminumProductionAndProcessing => 3314,
            Self::Foundries => 3315,
            Self::ForgingAndStamping => 3321,
            Self::CutleryAndHandtoolManufacturing => 3322,
            Self::ArchitecturalAndStructuralMetalsManufacturing => 3323,
            Self::BoilerTankAndShippingContainerManufacturing => 3324,
            Self::HardwareManufacturing => 3325,
            Self::SpringAndWireProductManufacturing => 3326,
            Self::MachineShopsTurnedProductAndScrewNutAndBoltManufacturing => 3327,
            Self::CoatingEngravingHeatTreatingAndAlliedActivities => 3328,
            Self::OtherFabricatedMetalProductManufacturing => 3329,
            Self::AgricultureConstructionAndMiningMachineryManufacturing => 3331,
            Self::IndustrialMachineryManufacturing => 3332,
            Self::CommercialAndServiceIndustryMachineryManufacturing => 3333,
            Self::VentilationHeatingAirconditioningAndCommercialRefrigerationEquipmentManufacturing => 3334,
            Self::MetalworkingMachineryManufacturing => 3335,
            Self::EngineTurbineAndPowerTransmissionEquipmentManufacturing => 3336,
            Self::OtherGeneralPurposeMachineryManufacturing => 3339,
            Self::ComputerAndPeripheralEquipmentManufacturing => 3341,
            Self::CommunicationsEquipmentManufacturing => 3342,
            Self::AudioAndVideoEquipmentManufacturing => 3343,
            Self::SemiconductorAndOtherElectronicComponentManufacturing => 3344,
            Self::NavigationalMeasuringElectromedicalAndControlInstrumentsManufacturing => 3345,
            Self::ManufacturingAndReproducingMagneticAndOpticalMedia => 3346,
            Self::ElectricLightingEquipmentManufacturing => 3351,
            Self::HouseholdApplianceManufacturing => 3352,
            Self::ElectricalEquipmentManufacturing => 3353,
            Self::OtherElectricalEquipmentAndComponentManufacturing => 3359,
            Self::MotorVehicleManufacturing => 3361,
            Self::MotorVehicleBodyAndTrailerManufacturing => 3362,
            Self::MotorVehiclePartsManufacturing => 3363,
            Self::AerospaceProductAndPartsManufacturing => 3364,
            Self::RailroadRollingStockManufacturing => 3365,
            Self::ShipAndBoatBuilding => 3366,
            Self::OtherTransportationEquipmentManufacturing => 3369,
            Self::HouseholdAndInstitutionalFurnitureAndKitchenCabinetManufacturing => 3371,
            Self::OfficeFurnitureIncludingFixturesManufacturing => 3372,
            Self::OtherFurnitureRelatedProductManufacturing => 3379,
            Self::MedicalEquipmentAndSuppliesManufacturing => 3391,
            Self::OtherMiscellaneousManufacturing => 3399,
            Self::MotorVehicleAndMotorVehiclePartsAndSuppliesMerchantWholesalers => 4231,
            Self::FurnitureAndHomeFurnishingMerchantWholesalers => 4232,
            Self::LumberAndOtherConstructionMaterialsMerchantWholesalers => 4233,
            Self::ProfessionalAndCommercialEquipmentAndSuppliesMerchantWholesalers => 4234,
            Self::MetalAndMineralExceptPetroleumMerchantWholesalers => 4235,
            Self::HouseholdAppliancesAndElectricalAndElectronicGoodsMerchantWholesalers => 4236,
            Self::HardwareAndPlumbingAndHeatingEquipmentAndSuppliesMerchantWholesalers => 4237,
            Self::MachineryEquipmentAndSuppliesMerchantWholesalers => 4238,
            Self::MiscellaneousDurableGoodsMerchantWholesalers => 4239,
            Self::PaperAndPaperProductMerchantWholesalers => 4241,
            Self::DrugsAndDruggistsSundriesMerchantWholesalers => 4242,
            Self::ApparelPieceGoodsAndNotionsMerchantWholesalers => 4243,
            Self::GroceryAndRelatedProductMerchantWholesalers => 4244,
            Self::FarmProductRawMaterialMerchantWholesalers => 4245,
            Self::ChemicalAndAlliedProductsMerchantWholesalers => 4246,
            Self::PetroleumAndPetroleumProductsMerchantWholesalers => 4247,
            Self::BeerWineAndDistilledAlcoholicBeverageMerchantWholesalers => 4248,
            Self::MiscellaneousNondurableGoodsMerchantWholesalers => 4249,
            Self::WholesaleTradeAgentsAndBrokers => 4251,
            Self::AutomobileDealers => 4411,
            Self::OtherMotorVehicleDealers => 4412,
            Self::AutomotivePartsAccessoriesAndTireRetailers => 4413,
            Self::BuildingMaterialAndSuppliesDealers => 4441,
            Self::LawnAndGardenEquipmentAndSuppliesRetailers => 4442,
            Self::GroceryAndConvenienceRetailers => 4451,
            Self::SpecialtyFoodRetailers => 4452,
            Self::BeerWineAndLiquorRetailers => 4453,
            Self::FurnitureAndHomeFurnishingsRetailers => 4491,
            Self::ElectronicsAndApplianceRetailers => 4492,
            Self::DepartmentStores => 4551,
            Self::WarehouseClubsSupercentersAndOtherGeneralMerchandiseRetailers => 4552,
            Self::HealthAndPersonalCareRetailers => 4561,
            Self::GasolineStations => 4571,
            Self::FuelDealers => 4572,
            Self::ClothingAndClothingAccessoriesRetailers => 4581,
            Self::ShoeRetailers => 4582,
            Self::JewelryLuggageAndLeatherGoodsRetailers => 4583,
            Self::SportingGoodsAndMusicalInstrumentRetailers => 4591,
            Self::BookRetailersAndNewsDealers => 4592,
            Self::Florists => 4593,
            Self::OfficeSuppliesStationeryAndGiftRetailers => 4594,
            Self::UsedMerchandiseRetailers => 4595,
            Self::OtherMiscellaneousRetailers => 4599,
            Self::ScheduledAirTransportation => 4811,
            Self::NonscheduledAirTransportation => 4812,
            Self::RailTransportation => 4821,
            Self::DeepSeaCoastalAndGreatLakesWaterTransportation => 4831,
            Self::InlandWaterTransportation => 4832,
            Self::GeneralFreightTrucking => 4841,
            Self::SpecializedFreightTrucking => 4842,
            Self::UrbanTransitSystems => 4851,
            Self::InterurbanAndRuralBusTransportation => 4852,
            Self::TaxiAndLimousineService => 4853,
            Self::SchoolAndEmployeeBusTransportation => 4854,
            Self::CharterBusIndustry => 4855,
            Self::OtherTransitAndGroundPassengerTransportation => 4859,
            Self::PipelineTransportationOfCrudeOil => 4861,
            Self::PipelineTransportationOfNaturalGas => 4862,
            Self::OtherPipelineTransportation => 4869,
            Self::ScenicAndSightseeingTransportationLand => 4871,
            Self::ScenicAndSightseeingTransportationWater => 4872,
            Self::ScenicAndSightseeingTransportationOther => 4879,
            Self::SupportActivitiesForAirTransportation => 4881,
            Self::SupportActivitiesForRailTransportation => 4882,
            Self::SupportActivitiesForWaterTransportation => 4883,
            Self::SupportActivitiesForRoadTransportation => 4884,
            Self::FreightTransportationArrangement => 4885,
            Self::OtherSupportActivitiesForTransportation => 4889,
            Self::PostalService => 4911,
            Self::CouriersAndExpressDeliveryServices => 4921,
            Self::LocalMessengersAndLocalDelivery => 4922,
            Self::WarehousingAndStorage => 4931,
            Self::NewspaperPeriodicalBookAndDirectoryPublishers => 5111,
            Self::SoftwarePublishers => 5112,
            Self::MotionPictureAndVideoIndustries => 5121,
            Self::SoundRecordingIndustries => 5122,
            Self::RadioAndTelevisionBroadcasting => 5151,
            Self::CableAndOtherSubscriptionProgramming => 5152,
            Self::WiredAndWirelessTelecommunicationsCarriers => 5173,
            Self::SatelliteTelecommunications => 5174,
            Self::OtherTelecommunications => 5179,
            Self::ComputingInfrastructureProvidersDataProcessingWebHostingAndRelatedServices => 5182,
            Self::OtherInformationServices => 5191,
            Self::MonetaryAuthoritiesCentralBank => 5211,
            Self::DepositoryCreditIntermediation => 5221,
            Self::NondepositoryCreditIntermediation => 5222,
            Self::ActivitiesRelatedToCreditIntermediation => 5223,
            Self::SecuritiesAndCommodityContractsIntermediationAndBrokerage => 5231,
            Self::SecuritiesAndCommodityExchanges => 5232,
            Self::OtherFinancialInvestmentActivities => 5239,
            Self::InsuranceCarriers => 5241,
            Self::AgenciesBrokeragesAndOtherInsuranceRelatedActivities => 5242,
            Self::InsuranceAndEmployeeBenefitFunds => 5251,
            Self::OtherInvestmentPoolsAndFunds => 5259,
            Self::LessorsOfRealEstate => 5311,
            Self::OfficesOfRealEstateAgentsAndBrokers => 5312,
            Self::ActivitiesRelatedToRealEstate => 5313,
            Self::AutomotiveEquipmentRentalAndLeasing => 5321,
            Self::ConsumerGoodsRental => 5322,
            Self::GeneralRentalCenters => 5323,
            Self::CommercialAndIndustrialMachineryAndEquipmentRentalAndLeasing => 5324,
            Self::LessorsOfNonfinancialIntangibleAssetsExceptCopyrightedWorks => 5331,
            Self::LegalServices => 5411,
            Self::AccountingTaxPreparationBookkeepingAndPayrollServices => 5412,
            Self::ArchitecturalEngineeringAndRelatedServices => 5413,
            Self::SpecializedDesignServices => 5414,
            Self::ComputerSystemsDesignAndRelatedServices => 5415,
            Self::ManagementScientificAndTechnicalConsultingServices => 5416,
            Self::ScientificResearchAndDevelopmentServices => 5417,
            Self::AdvertisingPublicRelationsAndRelatedServices => 5418,
            Self::OtherProfessionalScientificAndTechnicalServices => 5419,
            Self::ManagementOfCompaniesAndEnterprises => 5511,
            Self::OfficeAdministrativeServices => 5611,
            Self::FacilitiesSupportServices => 5612,
            Self::EmploymentServices => 5613,
            Self::BusinessSupportServices => 5614,
            Self::TravelArrangementAndReservationServices => 5615,
            Self::InvestigationAndSecurityServices => 5616,
            Self::ServicesToBuildingsAndDwellings => 5617,
            Self::OtherSupportServices => 5619,
            Self::WasteCollection => 5621,
            Self::WasteTreatmentAndDisposal => 5622,
            Self::RemediationAndOtherWasteManagementServices => 5629,
            Self::ElementaryAndSecondarySchools => 6111,
            Self::JuniorColleges => 6112,
            Self::CollegesUniversitiesAndProfessionalSchools => 6113,
            Self::BusinessSchoolsAndComputerAndManagementTraining => 6114,
            Self::TechnicalAndTradeSchools => 6115,
            Self::OtherSchoolsAndInstruction => 6116,
            Self::EducationalSupportServices => 6117,
            Self::OfficesOfPhysicians => 6211,
            Self::OfficesOfDentists => 6212,
            Self::OfficesOfOtherHealthPractitioners => 6213,
            Self::OutpatientCareCenters => 6214,
            Self::MedicalAndDiagnosticLaboratories => 6215,
            Self::HomeHealthCareServices => 6216,
            Self::OtherAmbulatoryHealthCareServices => 6219,
            Self::GeneralMedicalAndSurgicalHospitals => 6221,
            Self::PsychiatricAndSubstanceAbuseHospitals => 6222,
            Self::SpecialtyExceptPsychiatricAndSubstanceAbuseHospitals => 6223,
            Self::NursingCareFacilitiesSkilledNursingFacilities => 6231,
            Self::ResidentialIntellectualAndDevelopmentalDisabilityMentalHealthAndSubstanceAbuseFacilities => 6232,
            Self::ContinuingCareRetirementCommunitiesAndAssistedLivingFacilitiesForTheElderly => 6233,
            Self::OtherResidentialCareFacilities => 6239,
            Self::IndividualAndFamilyServices => 6241,
            Self::CommunityFoodAndHousingAndEmergencyAndOtherReliefServices => 6242,
            Self::VocationalRehabilitationServices => 6243,
            Self::ChildDayCareServices => 6244,
            Self::PerformingArtsCompanies => 7111,
            Self::SpectatorSports => 7112,
            Self::PromotersOfPerformingArtsSportsAndSimilarEvents => 7113,
            Self::AgentsAndManagersForArtistsAthletesEntertainersAndOtherPublicFigures => 7114,
            Self::IndependentArtistsWritersAndPerformers => 7115,
            Self::MuseumsHistoricalSitesAndSimilarInstitutions => 7121,
            Self::AmusementParksAndArcades => 7131,
            Self::GamblingIndustries => 7132,
            Self::OtherAmusementAndRecreationIndustries => 7139,
            Self::TravelerAccommodation => 7211,
            Self::RvRecreationalVehicleParksAndRecreationalCamps => 7212,
            Self::RoomingAndBoardingHouses => 7213,
            Self::SpecialFoodServices => 7223,
            Self::DrinkingPlacesAlcoholicBeverages => 7224,
            Self::RestaurantsAndOtherEatingPlaces => 7225,
            Self::AutomotiveRepairAndMaintenance => 8111,
            Self::ElectronicAndPrecisionEquipmentRepairAndMaintenance => 8112,
            Self::CommercialAndIndustrialMachineryAndEquipmentExceptAutomotiveAndElectronicRepairAndMaintenance => 8113,
            Self::PersonalAndHouseholdGoodsRepairAndMaintenance => 8114,
            Self::PersonalCareServices => 8121,
            Self::DeathCareServices => 8122,
            Self::DrycleaningAndLaundryServices => 8123,
            Self::OtherPersonalServices => 8129,
            Self::ReligiousOrganizations => 8131,
            Self::GrantmakingAndGivingServices => 8132,
            Self::SocialAdvocacyOrganizations => 8133,
            Self::CivicAndSocialOrganizations => 8134,
            Self::BusinessProfessionalLaborPoliticalAndSimilarOrganizations => 8139,
            Self::PrivateHouseholds => 8141,
            Self::ExecutiveLegislativeAndOtherGeneralGovernmentSupport => 9211,
            Self::JusticePublicOrderAndSafetyActivities => 9221,
            Self::AdministrationOfHumanResourcePrograms => 9231,
            Self::AdministrationOfEnvironmentalQualityPrograms => 9241,
            Self::AdministrationOfHousingProgramsUrbanPlanningAndCommunityDevelopment => 9251,
            Self::AdministrationOfEconomicPrograms => 9261,
            Self::SpaceResearchAndTechnology => 9271,
            Self::NationalSecurityAndInternationalAffairs => 9281,
            Self::UnclassifiedEstablishments => 9999,
        }
    }

    /// Attempts to convert a string representing a NAICS code into a NaicsCategory
    /// Returns Some(NaicsCategory) if successful, None otherwise
    pub fn from_code(code_str: &str) -> Option<Self> {
        // Try to parse the input string to i64
        let code = match code_str.parse::<i64>() {
            Ok(num) => num,
            Err(_) => return None,
        };

        // Match the code against known NAICS codes
        let category = match code {
            1111 => Self::OilseedAndGrainFarming,
            1112 => Self::VegetableAndMelonFarming,
            1113 => Self::FruitAndTreeNutFarming,
            1114 => Self::GreenhouseNurseryAndFloricultureProduction,
            1119 => Self::OtherCropFarming,
            1121 => Self::CattleRanchingAndFarming,
            1122 => Self::HogAndPigFarming,
            1123 => Self::PoultryAndEggProduction,
            1124 => Self::SheepAndGoatFarming,
            1125 => Self::Aquaculture,
            1129 => Self::OtherAnimalProduction,
            1131 => Self::TimberTractOperations,
            1132 => Self::ForestNurseriesAndGatheringOfForestProducts,
            1133 => Self::Logging,
            1141 => Self::Fishing,
            1142 => Self::HuntingAndTrapping,
            1151 => Self::SupportActivitiesForCropProduction,
            1152 => Self::SupportActivitiesForAnimalProduction,
            1153 => Self::SupportActivitiesForForestry,
            2111 => Self::OilAndGasExtraction,
            2121 => Self::CoalMining,
            2122 => Self::MetalOreMining,
            2123 => Self::NonmetallicMineralMiningAndQuarrying,
            2131 => Self::SupportActivitiesForMining,
            2211 => Self::ElectricPowerGenerationTransmissionAndDistribution,
            2212 => Self::NaturalGasDistribution,
            2213 => Self::WaterSewageAndOtherSystems,
            2361 => Self::ResidentialBuildingConstruction,
            2362 => Self::NonresidentialBuildingConstruction,
            2371 => Self::UtilitySystemConstruction,
            2372 => Self::LandSubdivision,
            2373 => Self::HighwayStreetAndBridgeConstruction,
            2379 => Self::OtherHeavyAndCivilEngineeringConstruction,
            2381 => Self::FoundationStructureAndBuildingExteriorContractors,
            2382 => Self::BuildingEquipmentContractors,
            2383 => Self::BuildingFinishingContractors,
            2389 => Self::OtherSpecialtyTradeContractors,
            3111 => Self::AnimalFoodManufacturing,
            3112 => Self::GrainAndOilseedMilling,
            3113 => Self::SugarAndConfectioneryProductManufacturing,
            3114 => Self::FruitAndVegetablePreservingAndSpecialtyFoodManufacturing,
            3115 => Self::DairyProductManufacturing,
            3116 => Self::AnimalSlaughteringAndProcessing,
            3117 => Self::SeafoodProductPreparationAndPackaging,
            3118 => Self::BakeriesAndTortillaManufacturing,
            3119 => Self::OtherFoodManufacturing,
            3121 => Self::BeverageManufacturing,
            3122 => Self::TobaccoManufacturing,
            3131 => Self::FiberYarnAndThreadMills,
            3132 => Self::FabricMills,
            3133 => Self::TextileAndFabricFinishingAndFabricCoatingMills,
            3141 => Self::TextileFurnishingsMills,
            3149 => Self::OtherTextileProductMills,
            3151 => Self::ApparelKnittingMills,
            3152 => Self::CutAndSewApparelManufacturing,
            3159 => Self::ApparelAccessoriesAndOtherApparelManufacturing,
            3161 => Self::LeatherAndHideTanningAndFinishing,
            3162 => Self::FootwearManufacturing,
            3169 => Self::OtherLeatherAndAlliedProductManufacturing,
            3211 => Self::SawmillsAndWoodPreservation,
            3212 => Self::VeneerPlywoodAndEngineeredWoodProductManufacturing,
            3219 => Self::OtherWoodProductManufacturing,
            3221 => Self::PulpPaperAndPaperboardMills,
            3222 => Self::ConvertedPaperProductManufacturing,
            3231 => Self::PrintingAndRelatedSupportActivities,
            3241 => Self::PetroleumAndCoalProductsManufacturing,
            3251 => Self::BasicChemicalManufacturing,
            3252 => Self::ResinSyntheticRubberAndArtificialAndSyntheticFibersAndFilamentsManufacturing,
            3253 => Self::PesticideFertilizerAndOtherAgriculturalChemicalManufacturing,
            3254 => Self::PharmaceuticalAndMedicineManufacturing,
            3255 => Self::PaintCoatingAndAdhesiveManufacturing,
            3256 => Self::SoapCleaningCompoundAndToiletPreparationManufacturing,
            3259 => Self::OtherChemicalProductAndPreparationManufacturing,
            3261 => Self::PlasticsProductManufacturing,
            3262 => Self::RubberProductManufacturing,
            3271 => Self::ClayProductAndRefractoryManufacturing,
            3272 => Self::GlassAndGlassProductManufacturing,
            3273 => Self::CementAndConcreteProductManufacturing,
            3274 => Self::LimeAndGypsumProductManufacturing,
            3279 => Self::OtherNonmetallicMineralProductManufacturing,
            3311 => Self::IronAndSteelMillsAndFerroalloyManufacturing,
            3312 => Self::SteelProductManufacturingFromPurchasedSteel,
            3313 => Self::AluminaAndAluminumProductionAndProcessing,
            3314 => Self::NonferrousMetalExceptAluminumProductionAndProcessing,
            3315 => Self::Foundries,
            3321 => Self::ForgingAndStamping,
            3322 => Self::CutleryAndHandtoolManufacturing,
            3323 => Self::ArchitecturalAndStructuralMetalsManufacturing,
            3324 => Self::BoilerTankAndShippingContainerManufacturing,
            3325 => Self::HardwareManufacturing,
            3326 => Self::SpringAndWireProductManufacturing,
            3327 => Self::MachineShopsTurnedProductAndScrewNutAndBoltManufacturing,
            3328 => Self::CoatingEngravingHeatTreatingAndAlliedActivities,
            3329 => Self::OtherFabricatedMetalProductManufacturing,
            3331 => Self::AgricultureConstructionAndMiningMachineryManufacturing,
            3332 => Self::IndustrialMachineryManufacturing,
            3333 => Self::CommercialAndServiceIndustryMachineryManufacturing,
            3334 => Self::VentilationHeatingAirconditioningAndCommercialRefrigerationEquipmentManufacturing,
            3335 => Self::MetalworkingMachineryManufacturing,
            3336 => Self::EngineTurbineAndPowerTransmissionEquipmentManufacturing,
            3339 => Self::OtherGeneralPurposeMachineryManufacturing,
            3341 => Self::ComputerAndPeripheralEquipmentManufacturing,
            3342 => Self::CommunicationsEquipmentManufacturing,
            3343 => Self::AudioAndVideoEquipmentManufacturing,
            3344 => Self::SemiconductorAndOtherElectronicComponentManufacturing,
            3345 => Self::NavigationalMeasuringElectromedicalAndControlInstrumentsManufacturing,
            3346 => Self::ManufacturingAndReproducingMagneticAndOpticalMedia,
            3351 => Self::ElectricLightingEquipmentManufacturing,
            3352 => Self::HouseholdApplianceManufacturing,
            3353 => Self::ElectricalEquipmentManufacturing,
            3359 => Self::OtherElectricalEquipmentAndComponentManufacturing,
            3361 => Self::MotorVehicleManufacturing,
            3362 => Self::MotorVehicleBodyAndTrailerManufacturing,
            3363 => Self::MotorVehiclePartsManufacturing,
            3364 => Self::AerospaceProductAndPartsManufacturing,
            3365 => Self::RailroadRollingStockManufacturing,
            3366 => Self::ShipAndBoatBuilding,
            3369 => Self::OtherTransportationEquipmentManufacturing,
            3371 => Self::HouseholdAndInstitutionalFurnitureAndKitchenCabinetManufacturing,
            3372 => Self::OfficeFurnitureIncludingFixturesManufacturing,
            3379 => Self::OtherFurnitureRelatedProductManufacturing,
            3391 => Self::MedicalEquipmentAndSuppliesManufacturing,
            3399 => Self::OtherMiscellaneousManufacturing,
            4231 => Self::MotorVehicleAndMotorVehiclePartsAndSuppliesMerchantWholesalers,
            4232 => Self::FurnitureAndHomeFurnishingMerchantWholesalers,
            4233 => Self::LumberAndOtherConstructionMaterialsMerchantWholesalers,
            4234 => Self::ProfessionalAndCommercialEquipmentAndSuppliesMerchantWholesalers,
            4235 => Self::MetalAndMineralExceptPetroleumMerchantWholesalers,
            4236 => Self::HouseholdAppliancesAndElectricalAndElectronicGoodsMerchantWholesalers,
            4237 => Self::HardwareAndPlumbingAndHeatingEquipmentAndSuppliesMerchantWholesalers,
            4238 => Self::MachineryEquipmentAndSuppliesMerchantWholesalers,
            4239 => Self::MiscellaneousDurableGoodsMerchantWholesalers,
            4241 => Self::PaperAndPaperProductMerchantWholesalers,
            4242 => Self::DrugsAndDruggistsSundriesMerchantWholesalers,
            4243 => Self::ApparelPieceGoodsAndNotionsMerchantWholesalers,
            4244 => Self::GroceryAndRelatedProductMerchantWholesalers,
            4245 => Self::FarmProductRawMaterialMerchantWholesalers,
            4246 => Self::ChemicalAndAlliedProductsMerchantWholesalers,
            4247 => Self::PetroleumAndPetroleumProductsMerchantWholesalers,
            4248 => Self::BeerWineAndDistilledAlcoholicBeverageMerchantWholesalers,
            4249 => Self::MiscellaneousNondurableGoodsMerchantWholesalers,
            4251 => Self::WholesaleTradeAgentsAndBrokers,
            4411 => Self::AutomobileDealers,
            4412 => Self::OtherMotorVehicleDealers,
            4413 => Self::AutomotivePartsAccessoriesAndTireRetailers,
            4441 => Self::BuildingMaterialAndSuppliesDealers,
            4442 => Self::LawnAndGardenEquipmentAndSuppliesRetailers,
            4451 => Self::GroceryAndConvenienceRetailers,
            4452 => Self::SpecialtyFoodRetailers,
            4453 => Self::BeerWineAndLiquorRetailers,
            4491 => Self::FurnitureAndHomeFurnishingsRetailers,
            4492 => Self::ElectronicsAndApplianceRetailers,
            4551 => Self::DepartmentStores,
            4552 => Self::WarehouseClubsSupercentersAndOtherGeneralMerchandiseRetailers,
            4561 => Self::HealthAndPersonalCareRetailers,
            4571 => Self::GasolineStations,
            4572 => Self::FuelDealers,
            4581 => Self::ClothingAndClothingAccessoriesRetailers,
            4582 => Self::ShoeRetailers,
            4583 => Self::JewelryLuggageAndLeatherGoodsRetailers,
            4591 => Self::SportingGoodsAndMusicalInstrumentRetailers,
            4592 => Self::BookRetailersAndNewsDealers,
            4593 => Self::Florists,
            4594 => Self::OfficeSuppliesStationeryAndGiftRetailers,
            4595 => Self::UsedMerchandiseRetailers,
            4599 => Self::OtherMiscellaneousRetailers,
            4811 => Self::ScheduledAirTransportation,
            4812 => Self::NonscheduledAirTransportation,
            4821 => Self::RailTransportation,
            4831 => Self::DeepSeaCoastalAndGreatLakesWaterTransportation,
            4832 => Self::InlandWaterTransportation,
            4841 => Self::GeneralFreightTrucking,
            4842 => Self::SpecializedFreightTrucking,
            4851 => Self::UrbanTransitSystems,
            4852 => Self::InterurbanAndRuralBusTransportation,
            4853 => Self::TaxiAndLimousineService,
            4854 => Self::SchoolAndEmployeeBusTransportation,
            4855 => Self::CharterBusIndustry,
            4859 => Self::OtherTransitAndGroundPassengerTransportation,
            4861 => Self::PipelineTransportationOfCrudeOil,
            4862 => Self::PipelineTransportationOfNaturalGas,
            4869 => Self::OtherPipelineTransportation,
            4871 => Self::ScenicAndSightseeingTransportationLand,
            4872 => Self::ScenicAndSightseeingTransportationWater,
            4879 => Self::ScenicAndSightseeingTransportationOther,
            4881 => Self::SupportActivitiesForAirTransportation,
            4882 => Self::SupportActivitiesForRailTransportation,
            4883 => Self::SupportActivitiesForWaterTransportation,
            4884 => Self::SupportActivitiesForRoadTransportation,
            4885 => Self::FreightTransportationArrangement,
            4889 => Self::OtherSupportActivitiesForTransportation,
            4911 => Self::PostalService,
            4921 => Self::CouriersAndExpressDeliveryServices,
            4922 => Self::LocalMessengersAndLocalDelivery,
            4931 => Self::WarehousingAndStorage,
            5111 => Self::NewspaperPeriodicalBookAndDirectoryPublishers,
            5112 => Self::SoftwarePublishers,
            5121 => Self::MotionPictureAndVideoIndustries,
            5122 => Self::SoundRecordingIndustries,
            5151 => Self::RadioAndTelevisionBroadcasting,
            5152 => Self::CableAndOtherSubscriptionProgramming,
            5173 => Self::WiredAndWirelessTelecommunicationsCarriers,
            5174 => Self::SatelliteTelecommunications,
            5179 => Self::OtherTelecommunications,
            5182 => Self::ComputingInfrastructureProvidersDataProcessingWebHostingAndRelatedServices,
            5191 => Self::OtherInformationServices,
            5211 => Self::MonetaryAuthoritiesCentralBank,
            5221 => Self::DepositoryCreditIntermediation,
            5222 => Self::NondepositoryCreditIntermediation,
            5223 => Self::ActivitiesRelatedToCreditIntermediation,
            5231 => Self::SecuritiesAndCommodityContractsIntermediationAndBrokerage,
            5232 => Self::SecuritiesAndCommodityExchanges,
            5239 => Self::OtherFinancialInvestmentActivities,
            5241 => Self::InsuranceCarriers,
            5242 => Self::AgenciesBrokeragesAndOtherInsuranceRelatedActivities,
            5251 => Self::InsuranceAndEmployeeBenefitFunds,
            5259 => Self::OtherInvestmentPoolsAndFunds,
            5311 => Self::LessorsOfRealEstate,
            5312 => Self::OfficesOfRealEstateAgentsAndBrokers,
            5313 => Self::ActivitiesRelatedToRealEstate,
            5321 => Self::AutomotiveEquipmentRentalAndLeasing,
            5322 => Self::ConsumerGoodsRental,
            5323 => Self::GeneralRentalCenters,
            5324 => Self::CommercialAndIndustrialMachineryAndEquipmentRentalAndLeasing,
            5331 => Self::LessorsOfNonfinancialIntangibleAssetsExceptCopyrightedWorks,
            5411 => Self::LegalServices,
            5412 => Self::AccountingTaxPreparationBookkeepingAndPayrollServices,
            5413 => Self::ArchitecturalEngineeringAndRelatedServices,
            5414 => Self::SpecializedDesignServices,
            5415 => Self::ComputerSystemsDesignAndRelatedServices,
            5416 => Self::ManagementScientificAndTechnicalConsultingServices,
            5417 => Self::ScientificResearchAndDevelopmentServices,
            5418 => Self::AdvertisingPublicRelationsAndRelatedServices,
            5419 => Self::OtherProfessionalScientificAndTechnicalServices,
            5511 => Self::ManagementOfCompaniesAndEnterprises,
            5611 => Self::OfficeAdministrativeServices,
            5612 => Self::FacilitiesSupportServices,
            5613 => Self::EmploymentServices,
            5614 => Self::BusinessSupportServices,
            5615 => Self::TravelArrangementAndReservationServices,
            5616 => Self::InvestigationAndSecurityServices,
            5617 => Self::ServicesToBuildingsAndDwellings,
            5619 => Self::OtherSupportServices,
            5621 => Self::WasteCollection,
            5622 => Self::WasteTreatmentAndDisposal,
            5629 => Self::RemediationAndOtherWasteManagementServices,
            6111 => Self::ElementaryAndSecondarySchools,
            6112 => Self::JuniorColleges,
            6113 => Self::CollegesUniversitiesAndProfessionalSchools,
            6114 => Self::BusinessSchoolsAndComputerAndManagementTraining,
            6115 => Self::TechnicalAndTradeSchools,
            6116 => Self::OtherSchoolsAndInstruction,
            6117 => Self::EducationalSupportServices,
            6211 => Self::OfficesOfPhysicians,
            6212 => Self::OfficesOfDentists,
            6213 => Self::OfficesOfOtherHealthPractitioners,
            6214 => Self::OutpatientCareCenters,
            6215 => Self::MedicalAndDiagnosticLaboratories,
            6216 => Self::HomeHealthCareServices,
            6219 => Self::OtherAmbulatoryHealthCareServices,
            6221 => Self::GeneralMedicalAndSurgicalHospitals,
            6222 => Self::PsychiatricAndSubstanceAbuseHospitals,
            6223 => Self::SpecialtyExceptPsychiatricAndSubstanceAbuseHospitals,
            6231 => Self::NursingCareFacilitiesSkilledNursingFacilities,
            6232 => Self::ResidentialIntellectualAndDevelopmentalDisabilityMentalHealthAndSubstanceAbuseFacilities,
            6233 => Self::ContinuingCareRetirementCommunitiesAndAssistedLivingFacilitiesForTheElderly,
            6239 => Self::OtherResidentialCareFacilities,
            6241 => Self::IndividualAndFamilyServices,
            6242 => Self::CommunityFoodAndHousingAndEmergencyAndOtherReliefServices,
            6243 => Self::VocationalRehabilitationServices,
            6244 => Self::ChildDayCareServices,
            7111 => Self::PerformingArtsCompanies,
            7112 => Self::SpectatorSports,
            7113 => Self::PromotersOfPerformingArtsSportsAndSimilarEvents,
            7114 => Self::AgentsAndManagersForArtistsAthletesEntertainersAndOtherPublicFigures,
            7115 => Self::IndependentArtistsWritersAndPerformers,
            7121 => Self::MuseumsHistoricalSitesAndSimilarInstitutions,
            7131 => Self::AmusementParksAndArcades,
            7132 => Self::GamblingIndustries,
            7139 => Self::OtherAmusementAndRecreationIndustries,
            7211 => Self::TravelerAccommodation,
            7212 => Self::RvRecreationalVehicleParksAndRecreationalCamps,
            7213 => Self::RoomingAndBoardingHouses,
            7223 => Self::SpecialFoodServices,
            7224 => Self::DrinkingPlacesAlcoholicBeverages,
            7225 => Self::RestaurantsAndOtherEatingPlaces,
            8111 => Self::AutomotiveRepairAndMaintenance,
            8112 => Self::ElectronicAndPrecisionEquipmentRepairAndMaintenance,
            8113 => Self::CommercialAndIndustrialMachineryAndEquipmentExceptAutomotiveAndElectronicRepairAndMaintenance,
            8114 => Self::PersonalAndHouseholdGoodsRepairAndMaintenance,
            8121 => Self::PersonalCareServices,
            8122 => Self::DeathCareServices,
            8123 => Self::DrycleaningAndLaundryServices,
            8129 => Self::OtherPersonalServices,
            8131 => Self::ReligiousOrganizations,
            8132 => Self::GrantmakingAndGivingServices,
            8133 => Self::SocialAdvocacyOrganizations,
            8134 => Self::CivicAndSocialOrganizations,
            8139 => Self::BusinessProfessionalLaborPoliticalAndSimilarOrganizations,
            8141 => Self::PrivateHouseholds,
            9211 => Self::ExecutiveLegislativeAndOtherGeneralGovernmentSupport,
            9221 => Self::JusticePublicOrderAndSafetyActivities,
            9231 => Self::AdministrationOfHumanResourcePrograms,
            9241 => Self::AdministrationOfEnvironmentalQualityPrograms,
            9251 => Self::AdministrationOfHousingProgramsUrbanPlanningAndCommunityDevelopment,
            9261 => Self::AdministrationOfEconomicPrograms,
            9271 => Self::SpaceResearchAndTechnology,
            9281 => Self::NationalSecurityAndInternationalAffairs,
            9999 => Self::UnclassifiedEstablishments,
            _ => return None, // Return None if no matching code is found
        };

        // Wrap the category in Some and return
        Some(category)
    }
}
