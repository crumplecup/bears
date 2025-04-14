/// Represents NAICS subcategories
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
pub enum NaicsSubcategory {
    /// Soybean Farming
    SoybeanFarming,
    /// Oilseed (except Soybean) Farming
    OilseedExceptSoybeanFarming,
    /// Dry Pea and Bean Farming
    DryPeaAndBeanFarming,
    /// Wheat Farming
    WheatFarming,
    /// Corn Farming
    CornFarming,
    /// Rice Farming
    RiceFarming,
    /// Other Grain Farming
    OtherGrainFarming,
    /// Vegetable and Melon Farming
    VegetableAndMelonFarming,
    /// Orange Groves
    OrangeGroves,
    /// Citrus (except Orange) Groves
    CitrusExceptOrangeGroves,
    /// Noncitrus Fruit and Tree Nut Farming
    NoncitrusFruitAndTreeNutFarming,
    /// Food Crops Grown Under Cover
    FoodCropsGrownUnderCover,
    /// Nursery and Floriculture Production
    NurseryAndFloricultureProduction,
    /// Tobacco Farming
    TobaccoFarming,
    /// Cotton Farming
    CottonFarming,
    /// Sugarcane Farming
    SugarcaneFarming,
    /// Hay Farming
    HayFarming,
    /// All Other Crop Farming
    AllOtherCropFarming,
    /// Beef Cattle Ranching and Farming, including Feedlots
    BeefCattleRanchingAndFarmingIncludingFeedlots,
    /// Dairy Cattle and Milk Production
    DairyCattleAndMilkProduction,
    /// Dual-Purpose Cattle Ranching and Farming
    DualpurposeCattleRanchingAndFarming,
    /// Hog and Pig Farming
    HogAndPigFarming,
    /// Chicken Egg Production
    ChickenEggProduction,
    /// Broilers and Other Meat Type Chicken Production
    BroilersAndOtherMeatTypeChickenProduction,
    /// Turkey Production
    TurkeyProduction,
    /// Poultry Hatcheries
    PoultryHatcheries,
    /// Other Poultry Production
    OtherPoultryProduction,
    /// Sheep Farming
    SheepFarming,
    /// Goat Farming
    GoatFarming,
    /// Aquaculture
    Aquaculture,
    /// Apiculture
    Apiculture,
    /// Horses and Other Equine Production
    HorsesAndOtherEquineProduction,
    /// Fur-Bearing Animal and Rabbit Production
    FurbearingAnimalAndRabbitProduction,
    /// All Other Animal Production
    AllOtherAnimalProduction,
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
    /// Crude Petroleum Extraction
    CrudePetroleumExtraction,
    /// Natural Gas Extraction
    NaturalGasExtraction,
    /// Coal Mining
    CoalMining,
    /// Iron Ore Mining
    IronOreMining,
    /// Gold Ore and Silver Ore Mining
    GoldOreAndSilverOreMining,
    /// Copper, Nickel, Lead, and Zinc Mining
    CopperNickelLeadAndZincMining,
    /// Other Metal Ore Mining
    OtherMetalOreMining,
    /// Stone Mining and Quarrying
    StoneMiningAndQuarrying,
    /// Sand, Gravel, Clay, and Ceramic and Refractory Minerals Mining and Quarrying
    SandGravelClayAndCeramicAndRefractoryMineralsMiningAndQuarrying,
    /// Other Nonmetallic Mineral Mining and Quarrying
    OtherNonmetallicMineralMiningAndQuarrying,
    /// Support Activities for Mining
    SupportActivitiesForMining,
    /// Electric Power Generation
    ElectricPowerGeneration,
    /// Electric Power Transmission, Control, and Distribution
    ElectricPowerTransmissionControlAndDistribution,
    /// Natural Gas Distribution
    NaturalGasDistribution,
    /// Water Supply and Irrigation Systems
    WaterSupplyAndIrrigationSystems,
    /// Sewage Treatment Facilities
    SewageTreatmentFacilities,
    /// Steam and Air-Conditioning Supply
    SteamAndAirconditioningSupply,
    /// Residential Building Construction
    ResidentialBuildingConstruction,
    /// Industrial Building Construction
    IndustrialBuildingConstruction,
    /// Commercial and Institutional Building Construction
    CommercialAndInstitutionalBuildingConstruction,
    /// Water and Sewer Line and Related Structures Construction
    WaterAndSewerLineAndRelatedStructuresConstruction,
    /// Oil and Gas Pipeline and Related Structures Construction
    OilAndGasPipelineAndRelatedStructuresConstruction,
    /// Power and Communication Line and Related Structures Construction
    PowerAndCommunicationLineAndRelatedStructuresConstruction,
    /// Land Subdivision
    LandSubdivision,
    /// Highway, Street, and Bridge Construction
    HighwayStreetAndBridgeConstruction,
    /// Other Heavy and Civil Engineering Construction
    OtherHeavyAndCivilEngineeringConstruction,
    /// Poured Concrete Foundation and Structure Contractors
    PouredConcreteFoundationAndStructureContractors,
    /// Structural Steel and Precast Concrete Contractors
    StructuralSteelAndPrecastConcreteContractors,
    /// Framing Contractors
    FramingContractors,
    /// Masonry Contractors
    MasonryContractors,
    /// Glass and Glazing Contractors
    GlassAndGlazingContractors,
    /// Roofing Contractors
    RoofingContractors,
    /// Siding Contractors
    SidingContractors,
    /// Other Foundation, Structure, and Building Exterior Contractors
    OtherFoundationStructureAndBuildingExteriorContractors,
    /// Electrical Contractors and Other Wiring Installation Contractors
    ElectricalContractorsAndOtherWiringInstallationContractors,
    /// Plumbing, Heating, and Air-Conditioning Contractors
    PlumbingHeatingAndAirconditioningContractors,
    /// Other Building Equipment Contractors
    OtherBuildingEquipmentContractors,
    /// Drywall and Insulation Contractors
    DrywallAndInsulationContractors,
    /// Painting and Wall Covering Contractors
    PaintingAndWallCoveringContractors,
    /// Flooring Contractors
    FlooringContractors,
    /// Tile and Terrazzo Contractors
    TileAndTerrazzoContractors,
    /// Finish Carpentry Contractors
    FinishCarpentryContractors,
    /// Other Building Finishing Contractors
    OtherBuildingFinishingContractors,
    /// Site Preparation Contractors
    SitePreparationContractors,
    /// All Other Specialty Trade Contractors
    AllOtherSpecialtyTradeContractors,
    /// Animal Food Manufacturing
    AnimalFoodManufacturing,
    /// Flour Milling and Malt Manufacturing
    FlourMillingAndMaltManufacturing,
    /// Starch and Vegetable Fats and Oils Manufacturing
    StarchAndVegetableFatsAndOilsManufacturing,
    /// Breakfast Cereal Manufacturing
    BreakfastCerealManufacturing,
    /// Sugar Manufacturing
    SugarManufacturing,
    /// Nonchocolate Confectionery Manufacturing
    NonchocolateConfectioneryManufacturing,
    /// Chocolate and Confectionery Manufacturing
    ChocolateAndConfectioneryManufacturing,
    /// Frozen Food Manufacturing
    FrozenFoodManufacturing,
    /// Fruit and Vegetable Canning, Pickling, and Drying
    FruitAndVegetableCanningPicklingAndDrying,
    /// Dairy Product (except Frozen) Manufacturing
    DairyProductExceptFrozenManufacturing,
    /// Ice Cream and Frozen Dessert Manufacturing
    IceCreamAndFrozenDessertManufacturing,
    /// Animal Slaughtering and Processing
    AnimalSlaughteringAndProcessing,
    /// Seafood Product Preparation and Packaging
    SeafoodProductPreparationAndPackaging,
    /// Bread and Bakery Product Manufacturing
    BreadAndBakeryProductManufacturing,
    /// Cookie, Cracker, and Pasta Manufacturing
    CookieCrackerAndPastaManufacturing,
    /// Tortilla Manufacturing
    TortillaManufacturing,
    /// Snack Food Manufacturing
    SnackFoodManufacturing,
    /// Coffee and Tea Manufacturing
    CoffeeAndTeaManufacturing,
    /// Flavoring Syrup and Concentrate Manufacturing
    FlavoringSyrupAndConcentrateManufacturing,
    /// Seasoning and Dressing Manufacturing
    SeasoningAndDressingManufacturing,
    /// All Other Food Manufacturing
    AllOtherFoodManufacturing,
    /// Soft Drink and Ice Manufacturing
    SoftDrinkAndIceManufacturing,
    /// Breweries
    Breweries,
    /// Wineries
    Wineries,
    /// Distilleries
    Distilleries,
    /// Tobacco Manufacturing
    TobaccoManufacturing,
    /// Fiber, Yarn, and Thread Mills
    FiberYarnAndThreadMills,
    /// Broadwoven Fabric Mills
    BroadwovenFabricMills,
    /// Narrow Fabric Mills and Schiffli Machine Embroidery
    NarrowFabricMillsAndSchiffliMachineEmbroidery,
    /// Nonwoven Fabric Mills
    NonwovenFabricMills,
    /// Knit Fabric Mills
    KnitFabricMills,
    /// Textile and Fabric Finishing Mills
    TextileAndFabricFinishingMills,
    /// Fabric Coating Mills
    FabricCoatingMills,
    /// Carpet and Rug Mills
    CarpetAndRugMills,
    /// Curtain and Linen Mills
    CurtainAndLinenMills,
    /// Textile Bag and Canvas Mills
    TextileBagAndCanvasMills,
    /// All Other Textile Product Mills
    AllOtherTextileProductMills,
    /// Apparel Knitting Mills
    ApparelKnittingMills,
    /// Cut and Sew Apparel Contractors
    CutAndSewApparelContractors,
    /// Cut and Sew Apparel Manufacturing (except Contractors)
    CutAndSewApparelManufacturingExceptContractors,
    /// Men's and Boys' Cut and Sew Apparel Manufacturing
    MensAndBoysCutAndSewApparelManufacturing,
    /// Women's and Girls' Cut and Sew Apparel Manufacturing
    WomensAndGirlsCutAndSewApparelManufacturing,
    /// Other Cut and Sew Apparel Manufacturing
    OtherCutAndSewApparelManufacturing,
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
    /// Millwork
    Millwork,
    /// Wood Container and Pallet Manufacturing
    WoodContainerAndPalletManufacturing,
    /// All Other Wood Product Manufacturing
    AllOtherWoodProductManufacturing,
    /// Pulp Mills
    PulpMills,
    /// Paper Mills
    PaperMills,
    /// Paperboard Mills
    PaperboardMills,
    /// Paperboard Container Manufacturing
    PaperboardContainerManufacturing,
    /// Paper Bag and Coated and Treated Paper Manufacturing
    PaperBagAndCoatedAndTreatedPaperManufacturing,
    /// Stationery Product Manufacturing
    StationeryProductManufacturing,
    /// Other Converted Paper Product Manufacturing
    OtherConvertedPaperProductManufacturing,
    /// Printing
    Printing,
    /// Support Activities for Printing
    SupportActivitiesForPrinting,
    /// Petroleum Refineries
    PetroleumRefineries,
    /// Asphalt Paving, Roofing, and Saturated Materials Manufacturing
    AsphaltPavingRoofingAndSaturatedMaterialsManufacturing,
    /// Other Petroleum and Coal Products Manufacturing
    OtherPetroleumAndCoalProductsManufacturing,
    /// Petrochemical Manufacturing
    PetrochemicalManufacturing,
    /// Industrial Gas Manufacturing
    IndustrialGasManufacturing,
    /// Synthetic Dye and Pigment Manufacturing
    SyntheticDyeAndPigmentManufacturing,
    /// Other Basic Inorganic Chemical Manufacturing
    OtherBasicInorganicChemicalManufacturing,
    /// Other Basic Organic Chemical Manufacturing
    OtherBasicOrganicChemicalManufacturing,
    /// Resin and Synthetic Rubber Manufacturing
    ResinAndSyntheticRubberManufacturing,
    /// Artificial and Synthetic Fibers and Filaments Manufacturing
    ArtificialAndSyntheticFibersAndFilamentsManufacturing,
    /// Fertilizer and Compost Manufacturing
    FertilizerAndCompostManufacturing,
    /// Pesticide and Other Agricultural Chemical Manufacturing
    PesticideAndOtherAgriculturalChemicalManufacturing,
    /// Pharmaceutical and Medicine Manufacturing
    PharmaceuticalAndMedicineManufacturing,
    /// Paint and Coating Manufacturing
    PaintAndCoatingManufacturing,
    /// Adhesive Manufacturing
    AdhesiveManufacturing,
    /// Soap and Cleaning Compound Manufacturing
    SoapAndCleaningCompoundManufacturing,
    /// Toilet Preparation Manufacturing
    ToiletPreparationManufacturing,
    /// Printing Ink Manufacturing
    PrintingInkManufacturing,
    /// Explosives Manufacturing
    ExplosivesManufacturing,
    /// All Other Chemical Product and Preparation Manufacturing
    AllOtherChemicalProductAndPreparationManufacturing,
    /// Plastics Packaging Materials and Unlaminated Film and Sheet Manufacturing
    PlasticsPackagingMaterialsAndUnlaminatedFilmAndSheetManufacturing,
    /// Plastics Pipe, Pipe Fitting, and Unlaminated Profile Shape Manufacturing
    PlasticsPipePipeFittingAndUnlaminatedProfileShapeManufacturing,
    /// Laminated Plastics Plate, Sheet (except Packaging), and Shape Manufacturing
    LaminatedPlasticsPlateSheetExceptPackagingAndShapeManufacturing,
    /// Polystyrene Foam Product Manufacturing
    PolystyreneFoamProductManufacturing,
    /// Urethane and Other Foam Product (except Polystyrene) Manufacturing
    UrethaneAndOtherFoamProductExceptPolystyreneManufacturing,
    /// Plastics Bottle Manufacturing
    PlasticsBottleManufacturing,
    /// Other Plastics Product Manufacturing
    OtherPlasticsProductManufacturing,
    /// Tire Manufacturing
    TireManufacturing,
    /// Rubber and Plastics Hoses and Belting Manufacturing
    RubberAndPlasticsHosesAndBeltingManufacturing,
    /// Other Rubber Product Manufacturing
    OtherRubberProductManufacturing,
    /// Pottery, Ceramics, and Plumbing Fixture Manufacturing
    PotteryCeramicsAndPlumbingFixtureManufacturing,
    /// Clay Building Material and Refractories Manufacturing
    ClayBuildingMaterialAndRefractoriesManufacturing,
    /// Glass and Glass Product Manufacturing
    GlassAndGlassProductManufacturing,
    /// Cement Manufacturing
    CementManufacturing,
    /// Ready-Mix Concrete Manufacturing
    ReadymixConcreteManufacturing,
    /// Concrete Pipe, Brick, and Block Manufacturing
    ConcretePipeBrickAndBlockManufacturing,
    /// Other Concrete Product Manufacturing
    OtherConcreteProductManufacturing,
    /// Lime Manufacturing
    LimeManufacturing,
    /// Gypsum Product Manufacturing
    GypsumProductManufacturing,
    /// Abrasive Product Manufacturing
    AbrasiveProductManufacturing,
    /// All Other Nonmetallic Mineral Product Manufacturing
    AllOtherNonmetallicMineralProductManufacturing,
    /// Iron and Steel Mills and Ferroalloy Manufacturing
    IronAndSteelMillsAndFerroalloyManufacturing,
    /// Iron and Steel Pipe and Tube Manufacturing from Purchased Steel
    IronAndSteelPipeAndTubeManufacturingFromPurchasedSteel,
    /// Rolling and Drawing of Purchased Steel
    RollingAndDrawingOfPurchasedSteel,
    /// Alumina and Aluminum Production and Processing
    AluminaAndAluminumProductionAndProcessing,
    /// Nonferrous Metal (except Aluminum) Smelting and Refining
    NonferrousMetalExceptAluminumSmeltingAndRefining,
    /// Copper Rolling, Drawing, Extruding, and Alloying
    CopperRollingDrawingExtrudingAndAlloying,
    /// Nonferrous Metal (except Copper and Aluminum) Rolling, Drawing, Extruding, and Alloying
    NonferrousMetalExceptCopperAndAluminumRollingDrawingExtrudingAndAlloying,
    /// Ferrous Metal Foundries
    FerrousMetalFoundries,
    /// Nonferrous Metal Foundries
    NonferrousMetalFoundries,
    /// Forging and Stamping
    ForgingAndStamping,
    /// Cutlery and Handtool Manufacturing
    CutleryAndHandtoolManufacturing,
    /// Plate Work and Fabricated Structural Product Manufacturing
    PlateWorkAndFabricatedStructuralProductManufacturing,
    /// Ornamental and Architectural Metal Products Manufacturing
    OrnamentalAndArchitecturalMetalProductsManufacturing,
    /// Power Boiler and Heat Exchanger Manufacturing
    PowerBoilerAndHeatExchangerManufacturing,
    /// Metal Tank (Heavy Gauge) Manufacturing
    MetalTankHeavyGaugeManufacturing,
    /// Metal Can, Box, and Other Metal Container (Light Gauge) Manufacturing
    MetalCanBoxAndOtherMetalContainerLightGaugeManufacturing,
    /// Hardware Manufacturing
    HardwareManufacturing,
    /// Spring and Wire Product Manufacturing
    SpringAndWireProductManufacturing,
    /// Machine Shops
    MachineShops,
    /// Turned Product and Screw, Nut, and Bolt Manufacturing
    TurnedProductAndScrewNutAndBoltManufacturing,
    /// Coating, Engraving, Heat Treating, and Allied Activities
    CoatingEngravingHeatTreatingAndAlliedActivities,
    /// Metal Valve Manufacturing
    MetalValveManufacturing,
    /// All Other Fabricated Metal Product Manufacturing
    AllOtherFabricatedMetalProductManufacturing,
    /// Agricultural Implement Manufacturing
    AgriculturalImplementManufacturing,
    /// Construction Machinery Manufacturing
    ConstructionMachineryManufacturing,
    /// Mining and Oil and Gas Field Machinery Manufacturing
    MiningAndOilAndGasFieldMachineryManufacturing,
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
    /// Pump and Compressor Manufacturing
    PumpAndCompressorManufacturing,
    /// Material Handling Equipment Manufacturing
    MaterialHandlingEquipmentManufacturing,
    /// All Other General Purpose Machinery Manufacturing
    AllOtherGeneralPurposeMachineryManufacturing,
    /// Computer and Peripheral Equipment Manufacturing
    ComputerAndPeripheralEquipmentManufacturing,
    /// Telephone Apparatus Manufacturing
    TelephoneApparatusManufacturing,
    /// Radio and Television Broadcasting and Wireless Communications Equipment Manufacturing
    RadioAndTelevisionBroadcastingAndWirelessCommunicationsEquipmentManufacturing,
    /// Other Communications Equipment Manufacturing
    OtherCommunicationsEquipmentManufacturing,
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
    /// Small Electrical Appliance Manufacturing
    SmallElectricalApplianceManufacturing,
    /// Major Household Appliance Manufacturing
    MajorHouseholdApplianceManufacturing,
    /// Electrical Equipment Manufacturing
    ElectricalEquipmentManufacturing,
    /// Battery Manufacturing
    BatteryManufacturing,
    /// Communication and Energy Wire and Cable Manufacturing
    CommunicationAndEnergyWireAndCableManufacturing,
    /// Wiring Device Manufacturing
    WiringDeviceManufacturing,
    /// All Other Electrical Equipment and Component Manufacturing
    AllOtherElectricalEquipmentAndComponentManufacturing,
    /// Automobile and Light Duty Motor Vehicle Manufacturing
    AutomobileAndLightDutyMotorVehicleManufacturing,
    /// Heavy Duty Truck Manufacturing
    HeavyDutyTruckManufacturing,
    /// Motor Vehicle Body and Trailer Manufacturing
    MotorVehicleBodyAndTrailerManufacturing,
    /// Motor Vehicle Gasoline Engine and Engine Parts Manufacturing
    MotorVehicleGasolineEngineAndEnginePartsManufacturing,
    /// Motor Vehicle Electrical and Electronic Equipment Manufacturing
    MotorVehicleElectricalAndElectronicEquipmentManufacturing,
    /// Motor Vehicle Steering and Suspension Components (except Spring) Manufacturing
    MotorVehicleSteeringAndSuspensionComponentsExceptSpringManufacturing,
    /// Motor Vehicle Brake System Manufacturing
    MotorVehicleBrakeSystemManufacturing,
    /// Motor Vehicle Transmission and Power Train Parts Manufacturing
    MotorVehicleTransmissionAndPowerTrainPartsManufacturing,
    /// Motor Vehicle Seating and Interior Trim Manufacturing
    MotorVehicleSeatingAndInteriorTrimManufacturing,
    /// Motor Vehicle Metal Stamping
    MotorVehicleMetalStamping,
    /// Other Motor Vehicle Parts Manufacturing
    OtherMotorVehiclePartsManufacturing,
    /// Aerospace Product and Parts Manufacturing
    AerospaceProductAndPartsManufacturing,
    /// Railroad Rolling Stock Manufacturing
    RailroadRollingStockManufacturing,
    /// Ship and Boat Building
    ShipAndBoatBuilding,
    /// Other Transportation Equipment Manufacturing
    OtherTransportationEquipmentManufacturing,
    /// Wood Kitchen Cabinet and Countertop Manufacturing
    WoodKitchenCabinetAndCountertopManufacturing,
    /// Household and Institutional Furniture Manufacturing
    HouseholdAndInstitutionalFurnitureManufacturing,
    /// Office Furniture (including Fixtures) Manufacturing
    OfficeFurnitureIncludingFixturesManufacturing,
    /// Mattress Manufacturing
    MattressManufacturing,
    /// Blind and Shade Manufacturing
    BlindAndShadeManufacturing,
    /// Medical Equipment and Supplies Manufacturing
    MedicalEquipmentAndSuppliesManufacturing,
    /// Jewelry and Silverware Manufacturing
    JewelryAndSilverwareManufacturing,
    /// Sporting and Athletic Goods Manufacturing
    SportingAndAthleticGoodsManufacturing,
    /// Doll, Toy, and Game Manufacturing
    DollToyAndGameManufacturing,
    /// Office Supplies (except Paper) Manufacturing
    OfficeSuppliesExceptPaperManufacturing,
    /// Sign Manufacturing
    SignManufacturing,
    /// All Other Miscellaneous Manufacturing
    AllOtherMiscellaneousManufacturing,
    /// Automobile and Other Motor Vehicle Merchant Wholesalers
    AutomobileAndOtherMotorVehicleMerchantWholesalers,
    /// Motor Vehicle Supplies and New Parts Merchant Wholesalers
    MotorVehicleSuppliesAndNewPartsMerchantWholesalers,
    /// Tire and Tube Merchant Wholesalers
    TireAndTubeMerchantWholesalers,
    /// Motor Vehicle Parts (Used) Merchant Wholesalers
    MotorVehiclePartsUsedMerchantWholesalers,
    /// Furniture Merchant Wholesalers
    FurnitureMerchantWholesalers,
    /// Home Furnishing Merchant Wholesalers
    HomeFurnishingMerchantWholesalers,
    /// Lumber, Plywood, Millwork, and Wood Panel Merchant Wholesalers
    LumberPlywoodMillworkAndWoodPanelMerchantWholesalers,
    /// Brick, Stone, and Related Construction Material Merchant Wholesalers
    BrickStoneAndRelatedConstructionMaterialMerchantWholesalers,
    /// Roofing, Siding, and Insulation Material Merchant Wholesalers
    RoofingSidingAndInsulationMaterialMerchantWholesalers,
    /// Other Construction Material Merchant Wholesalers
    OtherConstructionMaterialMerchantWholesalers,
    /// Photographic Equipment and Supplies Merchant Wholesalers
    PhotographicEquipmentAndSuppliesMerchantWholesalers,
    /// Office Equipment Merchant Wholesalers
    OfficeEquipmentMerchantWholesalers,
    /// Computer and Computer Peripheral Equipment and Software Merchant Wholesalers
    ComputerAndComputerPeripheralEquipmentAndSoftwareMerchantWholesalers,
    /// Other Commercial Equipment Merchant Wholesalers
    OtherCommercialEquipmentMerchantWholesalers,
    /// Medical, Dental, and Hospital Equipment and Supplies Merchant Wholesalers
    MedicalDentalAndHospitalEquipmentAndSuppliesMerchantWholesalers,
    /// Ophthalmic Goods Merchant Wholesalers
    OphthalmicGoodsMerchantWholesalers,
    /// Other Professional Equipment and Supplies Merchant Wholesalers
    OtherProfessionalEquipmentAndSuppliesMerchantWholesalers,
    /// Metal Service Centers and Other Metal Merchant Wholesalers
    MetalServiceCentersAndOtherMetalMerchantWholesalers,
    /// Coal and Other Mineral and Ore Merchant Wholesalers
    CoalAndOtherMineralAndOreMerchantWholesalers,
    /// Electrical Apparatus and Equipment, Wiring Supplies, and Related Equipment Merchant Wholesalers
    ElectricalApparatusAndEquipmentWiringSuppliesAndRelatedEquipmentMerchantWholesalers,
    /// Household Appliances, Electric Housewares, and Consumer Electronics Merchant Wholesalers
    HouseholdAppliancesElectricHousewaresAndConsumerElectronicsMerchantWholesalers,
    /// Other Electronic Parts and Equipment Merchant Wholesalers
    OtherElectronicPartsAndEquipmentMerchantWholesalers,
    /// Hardware Merchant Wholesalers
    HardwareMerchantWholesalers,
    /// Plumbing and Heating Equipment and Supplies (Hydronics) Merchant Wholesalers
    PlumbingAndHeatingEquipmentAndSuppliesHydronicsMerchantWholesalers,
    /// Warm Air Heating and Air-Conditioning Equipment and Supplies Merchant Wholesalers
    WarmAirHeatingAndAirconditioningEquipmentAndSuppliesMerchantWholesalers,
    /// Refrigeration Equipment and Supplies Merchant Wholesalers
    RefrigerationEquipmentAndSuppliesMerchantWholesalers,
    /// Construction and Mining (except Oil Well) Machinery and Equipment Merchant Wholesalers
    ConstructionAndMiningExceptOilWellMachineryAndEquipmentMerchantWholesalers,
    /// Farm and Garden Machinery and Equipment Merchant Wholesalers
    FarmAndGardenMachineryAndEquipmentMerchantWholesalers,
    /// Industrial Machinery and Equipment Merchant Wholesalers
    IndustrialMachineryAndEquipmentMerchantWholesalers,
    /// Industrial Supplies Merchant Wholesalers
    IndustrialSuppliesMerchantWholesalers,
    /// Service Establishment Equipment and Supplies Merchant Wholesalers
    ServiceEstablishmentEquipmentAndSuppliesMerchantWholesalers,
    /// Transportation Equipment and Supplies (except Motor Vehicle) Merchant Wholesalers
    TransportationEquipmentAndSuppliesExceptMotorVehicleMerchantWholesalers,
    /// Sporting and Recreational Goods and Supplies Merchant Wholesalers
    SportingAndRecreationalGoodsAndSuppliesMerchantWholesalers,
    /// Toy and Hobby Goods and Supplies Merchant Wholesalers
    ToyAndHobbyGoodsAndSuppliesMerchantWholesalers,
    /// Recyclable Material Merchant Wholesalers
    RecyclableMaterialMerchantWholesalers,
    /// Jewelry, Watch, Precious Stone, and Precious Metal Merchant Wholesalers
    JewelryWatchPreciousStoneAndPreciousMetalMerchantWholesalers,
    /// Other Miscellaneous Durable Goods Merchant Wholesalers
    OtherMiscellaneousDurableGoodsMerchantWholesalers,
    /// Printing and Writing Paper Merchant Wholesalers
    PrintingAndWritingPaperMerchantWholesalers,
    /// Stationery and Office Supplies Merchant Wholesalers
    StationeryAndOfficeSuppliesMerchantWholesalers,
    /// Industrial and Personal Service Paper Merchant Wholesalers
    IndustrialAndPersonalServicePaperMerchantWholesalers,
    /// Drugs and Druggists' Sundries Merchant Wholesalers
    DrugsAndDruggistsSundriesMerchantWholesalers,
    /// Piece Goods, Notions, and Other Dry Goods Merchant Wholesalers
    PieceGoodsNotionsAndOtherDryGoodsMerchantWholesalers,
    /// Footwear Merchant Wholesalers
    FootwearMerchantWholesalers,
    /// Clothing and Clothing Accessories Merchant Wholesalers
    ClothingAndClothingAccessoriesMerchantWholesalers,
    /// General Line Grocery Merchant Wholesalers
    GeneralLineGroceryMerchantWholesalers,
    /// Packaged Frozen Food Merchant Wholesalers
    PackagedFrozenFoodMerchantWholesalers,
    /// Dairy Product (except Dried or Canned) Merchant Wholesalers
    DairyProductExceptDriedOrCannedMerchantWholesalers,
    /// Poultry and Poultry Product Merchant Wholesalers
    PoultryAndPoultryProductMerchantWholesalers,
    /// Confectionery Merchant Wholesalers
    ConfectioneryMerchantWholesalers,
    /// Fish and Seafood Merchant Wholesalers
    FishAndSeafoodMerchantWholesalers,
    /// Meat and Meat Product Merchant Wholesalers
    MeatAndMeatProductMerchantWholesalers,
    /// Fresh Fruit and Vegetable Merchant Wholesalers
    FreshFruitAndVegetableMerchantWholesalers,
    /// Other Grocery and Related Products Merchant Wholesalers
    OtherGroceryAndRelatedProductsMerchantWholesalers,
    /// Grain and Field Bean Merchant Wholesalers
    GrainAndFieldBeanMerchantWholesalers,
    /// Livestock Merchant Wholesalers
    LivestockMerchantWholesalers,
    /// Other Farm Product Raw Material Merchant Wholesalers
    OtherFarmProductRawMaterialMerchantWholesalers,
    /// Plastics Materials and Basic Forms and Shapes Merchant Wholesalers
    PlasticsMaterialsAndBasicFormsAndShapesMerchantWholesalers,
    /// Other Chemical and Allied Products Merchant Wholesalers
    OtherChemicalAndAlliedProductsMerchantWholesalers,
    /// Petroleum Bulk Stations and Terminals
    PetroleumBulkStationsAndTerminals,
    /// Petroleum and Petroleum Products Merchant Wholesalers (except Bulk Stations and Terminals)
    PetroleumAndPetroleumProductsMerchantWholesalersExceptBulkStationsAndTerminals,
    /// Beer and Ale Merchant Wholesalers
    BeerAndAleMerchantWholesalers,
    /// Wine and Distilled Alcoholic Beverage Merchant Wholesalers
    WineAndDistilledAlcoholicBeverageMerchantWholesalers,
    /// Farm Supplies Merchant Wholesalers
    FarmSuppliesMerchantWholesalers,
    /// Book, Periodical, and Newspaper Merchant Wholesalers
    BookPeriodicalAndNewspaperMerchantWholesalers,
    /// Flower, Nursery Stock, and Florists' Supplies Merchant Wholesalers
    FlowerNurseryStockAndFloristsSuppliesMerchantWholesalers,
    /// Tobacco Product and Electronic Cigarette Merchant Wholesalers
    TobaccoProductAndElectronicCigaretteMerchantWholesalers,
    /// Paint, Varnish, and Supplies Merchant Wholesalers
    PaintVarnishAndSuppliesMerchantWholesalers,
    /// Other Miscellaneous Nondurable Goods Merchant Wholesalers
    OtherMiscellaneousNondurableGoodsMerchantWholesalers,
    /// Wholesale Trade Agents and Brokers
    WholesaleTradeAgentsAndBrokers,
    /// New Car Dealers
    NewCarDealers,
    /// Used Car Dealers
    UsedCarDealers,
    /// Recreational Vehicle Dealers
    RecreationalVehicleDealers,
    /// Motorcycle, Boat, and Other Motor Vehicle Dealers
    MotorcycleBoatAndOtherMotorVehicleDealers,
    /// Automotive Parts and Accessories Retailers
    AutomotivePartsAndAccessoriesRetailers,
    /// Tire Dealers
    TireDealers,
    /// Home Centers
    HomeCenters,
    /// Paint and Wallpaper Retailers
    PaintAndWallpaperRetailers,
    /// Hardware Retailers
    HardwareRetailers,
    /// Other Building Material Dealers
    OtherBuildingMaterialDealers,
    /// Outdoor Power Equipment Retailers
    OutdoorPowerEquipmentRetailers,
    /// Nursery, Garden Center, and Farm Supply Retailers
    NurseryGardenCenterAndFarmSupplyRetailers,
    /// Supermarkets and Other Grocery Retailers (except Convenience Retailers)
    SupermarketsAndOtherGroceryRetailersExceptConvenienceRetailers,
    /// Convenience Retailers and Vending Machine Operators
    ConvenienceRetailersAndVendingMachineOperators,
    /// Fruit and Vegetable Retailers
    FruitAndVegetableRetailers,
    /// Meat Retailers
    MeatRetailers,
    /// Fish and Seafood Retailers
    FishAndSeafoodRetailers,
    /// Other Specialty Food Retailers
    OtherSpecialtyFoodRetailers,
    /// Beer, Wine, and Liquor Retailers
    BeerWineAndLiquorRetailers,
    /// Furniture Retailers
    FurnitureRetailers,
    /// Home Furnishings Retailers
    HomeFurnishingsRetailers,
    /// Electronics and Appliance Retailers
    ElectronicsAndApplianceRetailers,
    /// Department Stores
    DepartmentStores,
    /// Warehouse Clubs and Supercenters
    WarehouseClubsAndSupercenters,
    /// All Other General Merchandise Retailers
    AllOtherGeneralMerchandiseRetailers,
    /// Pharmacies and Drug Retailers
    PharmaciesAndDrugRetailers,
    /// Cosmetics, Beauty Supplies, and Perfume Retailers
    CosmeticsBeautySuppliesAndPerfumeRetailers,
    /// Optical Goods Retailers
    OpticalGoodsRetailers,
    /// Other Health and Personal Care Retailers
    OtherHealthAndPersonalCareRetailers,
    /// Gasoline Stations with Convenience Stores
    GasolineStationsWithConvenienceStores,
    /// Other Gasoline Stations
    OtherGasolineStations,
    /// Fuel Dealers
    FuelDealers,
    /// Clothing Retailers
    ClothingRetailers,
    /// Clothing Accessories Retailers
    ClothingAccessoriesRetailers,
    /// Shoe Retailers
    ShoeRetailers,
    /// Jewelry Retailers
    JewelryRetailers,
    /// Luggage and Leather Goods Retailers
    LuggageAndLeatherGoodsRetailers,
    /// Sporting Goods Retailers
    SportingGoodsRetailers,
    /// Hobby, Toy, and Game Retailers
    HobbyToyAndGameRetailers,
    /// Sewing, Needlework, and Piece Goods Retailers
    SewingNeedleworkAndPieceGoodsRetailers,
    /// Musical Instrument and Supplies Retailers
    MusicalInstrumentAndSuppliesRetailers,
    /// Book Retailers and News Dealers
    BookRetailersAndNewsDealers,
    /// Florists
    Florists,
    /// Office Supplies and Stationery Retailers
    OfficeSuppliesAndStationeryRetailers,
    /// Gift, Novelty, and Souvenir Retailers
    GiftNoveltyAndSouvenirRetailers,
    /// Used Merchandise Retailers
    UsedMerchandiseRetailers,
    /// Pet and Pet Supplies Retailers
    PetAndPetSuppliesRetailers,
    /// Art Dealers
    ArtDealers,
    /// Manufactured (Mobile) Home Dealers
    ManufacturedMobileHomeDealers,
    /// All Other Miscellaneous Retailers
    AllOtherMiscellaneousRetailers,
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
    /// General Freight Trucking, Local
    GeneralFreightTruckingLocal,
    /// General Freight Trucking, Long-Distance
    GeneralFreightTruckingLongdistance,
    /// Used Household and Office Goods Moving
    UsedHouseholdAndOfficeGoodsMoving,
    /// Specialized Freight (except Used Goods) Trucking, Local
    SpecializedFreightExceptUsedGoodsTruckingLocal,
    /// Specialized Freight (except Used Goods) Trucking, Long-Distance
    SpecializedFreightExceptUsedGoodsTruckingLongdistance,
    /// Urban Transit Systems
    UrbanTransitSystems,
    /// Interurban and Rural Bus Transportation
    InterurbanAndRuralBusTransportation,
    /// Taxi Service
    TaxiService,
    /// Limousine Service
    LimousineService,
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
    /// Pipeline Transportation of Refined Petroleum Products
    PipelineTransportationOfRefinedPetroleumProducts,
    /// All Other Pipeline Transportation
    AllOtherPipelineTransportation,
    /// Scenic and Sightseeing Transportation, Land
    ScenicAndSightseeingTransportationLand,
    /// Scenic and Sightseeing Transportation, Water
    ScenicAndSightseeingTransportationWater,
    /// Scenic and Sightseeing Transportation, Other
    ScenicAndSightseeingTransportationOther,
    /// Airport Operations
    AirportOperations,
    /// Other Support Activities for Air Transportation
    OtherSupportActivitiesForAirTransportation,
    /// Support Activities for Rail Transportation
    SupportActivitiesForRailTransportation,
    /// Port and Harbor Operations
    PortAndHarborOperations,
    /// Marine Cargo Handling
    MarineCargoHandling,
    /// Navigational Services to Shipping
    NavigationalServicesToShipping,
    /// Other Support Activities for Water Transportation
    OtherSupportActivitiesForWaterTransportation,
    /// Motor Vehicle Towing
    MotorVehicleTowing,
    /// Other Support Activities for Road Transportation
    OtherSupportActivitiesForRoadTransportation,
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
    /// General Warehousing and Storage
    GeneralWarehousingAndStorage,
    /// Refrigerated Warehousing and Storage
    RefrigeratedWarehousingAndStorage,
    /// Farm Product Warehousing and Storage
    FarmProductWarehousingAndStorage,
    /// Other Warehousing and Storage
    OtherWarehousingAndStorage,
    /// Newspaper Publishers
    NewspaperPublishers,
    /// Periodical Publishers
    PeriodicalPublishers,
    /// Book Publishers
    BookPublishers,
    /// Directory and Mailing List Publishers
    DirectoryAndMailingListPublishers,
    /// Other Publishers
    OtherPublishers,
    /// Software Publishers
    SoftwarePublishers,
    /// Motion Picture and Video Production
    MotionPictureAndVideoProduction,
    /// Motion Picture and Video Distribution
    MotionPictureAndVideoDistribution,
    /// Motion Picture and Video Exhibition
    MotionPictureAndVideoExhibition,
    /// Postproduction Services and Other Motion Picture and Video Industries
    PostproductionServicesAndOtherMotionPictureAndVideoIndustries,
    /// Music Publishers
    MusicPublishers,
    /// Sound Recording Studios
    SoundRecordingStudios,
    /// Record Production and Distribution
    RecordProductionAndDistribution,
    /// Other Sound Recording Industries
    OtherSoundRecordingIndustries,
    /// Radio Broadcasting
    RadioBroadcasting,
    /// Television Broadcasting
    TelevisionBroadcasting,
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
    /// News Syndicates
    NewsSyndicates,
    /// Libraries and Archives
    LibrariesAndArchives,
    /// Internet Publishing and Broadcasting and Web Search Portals
    InternetPublishingAndBroadcastingAndWebSearchPortals,
    /// All Other Information Services
    AllOtherInformationServices,
    /// Monetary Authorities-Central Bank
    MonetaryAuthoritiesCentralBank,
    /// Commercial Banking
    CommercialBanking,
    /// Savings Institutions
    SavingsInstitutions,
    /// Credit Unions
    CreditUnions,
    /// Other Depository Credit Intermediation
    OtherDepositoryCreditIntermediation,
    /// Credit Card Issuing
    CreditCardIssuing,
    /// Sales Financing
    SalesFinancing,
    /// Other Nondepository Credit Intermediation
    OtherNondepositoryCreditIntermediation,
    /// Mortgage and Nonmortgage Loan Brokers
    MortgageAndNonmortgageLoanBrokers,
    /// Financial Transactions Processing, Reserve, and Clearinghouse Activities
    FinancialTransactionsProcessingReserveAndClearinghouseActivities,
    /// Other Activities Related to Credit Intermediation
    OtherActivitiesRelatedToCreditIntermediation,
    /// Investment Banking and Securities Dealing
    InvestmentBankingAndSecuritiesDealing,
    /// Securities Brokerage
    SecuritiesBrokerage,
    /// Commodity Contracts Dealing
    CommodityContractsDealing,
    /// Commodity Contracts Brokerage
    CommodityContractsBrokerage,
    /// Securities and Commodity Exchanges
    SecuritiesAndCommodityExchanges,
    /// Miscellaneous Intermediation
    MiscellaneousIntermediation,
    /// Portfolio Management
    PortfolioManagement,
    /// Investment Advice
    InvestmentAdvice,
    /// All Other Financial Investment Activities
    AllOtherFinancialInvestmentActivities,
    /// Direct Life, Health, and Medical Insurance Carriers
    DirectLifeHealthAndMedicalInsuranceCarriers,
    /// Direct Insurance (except Life, Health, and Medical) Carriers
    DirectInsuranceExceptLifeHealthAndMedicalCarriers,
    /// Reinsurance Carriers
    ReinsuranceCarriers,
    /// Insurance Agencies and Brokerages
    InsuranceAgenciesAndBrokerages,
    /// Other Insurance Related Activities
    OtherInsuranceRelatedActivities,
    /// Pension Funds
    PensionFunds,
    /// Health and Welfare Funds
    HealthAndWelfareFunds,
    /// Other Insurance Funds
    OtherInsuranceFunds,
    /// Open-End Investment Funds
    OpenEndInvestmentFunds,
    /// Trusts, Estates, and Agency Accounts
    TrustsEstatesAndAgencyAccounts,
    /// Real Estate Investment Trusts
    RealEstateInvestmentTrusts,
    /// Other Financial Vehicles
    OtherFinancialVehicles,
    /// Lessors of Residential Buildings and Dwellings
    LessorsOfResidentialBuildingsAndDwellings,
    /// Lessors of Nonresidential Buildings (except Miniwarehouses)
    LessorsOfNonresidentialBuildingsExceptMiniwarehouses,
    /// Lessors of Miniwarehouses and Self-Storage Units
    LessorsOfMiniwarehousesAndSelfStorageUnits,
    /// Lessors of Other Real Estate Property
    LessorsOfOtherRealEstateProperty,
    /// Offices of Real Estate Agents and Brokers
    OfficesOfRealEstateAgentsAndBrokers,
    /// Real Estate Property Managers
    RealEstatePropertyManagers,
    /// Offices of Real Estate Appraisers
    OfficesOfRealEstateAppraisers,
    /// Other Activities Related to Real Estate
    OtherActivitiesRelatedToRealEstate,
    /// Passenger Car Rental and Leasing
    PassengerCarRentalAndLeasing,
    /// Truck, Utility Trailer, and RV (Recreational Vehicle) Rental and Leasing
    TruckUtilityTrailerAndRvRecreationalVehicleRentalAndLeasing,
    /// Consumer Electronics and Appliances Rental
    ConsumerElectronicsAndAppliancesRental,
    /// Formal Wear and Costume Rental
    FormalWearAndCostumeRental,
    /// Video Tape and Disc Rental
    VideoTapeAndDiscRental,
    /// Other Consumer Goods Rental
    OtherConsumerGoodsRental,
    /// General Rental Centers
    GeneralRentalCenters,
    /// Construction, Transportation, Mining, and Forestry Machinery and Equipment Rental and Leasing
    ConstructionTransportationMiningAndForestryMachineryAndEquipmentRentalAndLeasing,
    /// Office Machinery and Equipment Rental and Leasing
    OfficeMachineryAndEquipmentRentalAndLeasing,
    /// Other Commercial and Industrial Machinery and Equipment Rental and Leasing
    OtherCommercialAndIndustrialMachineryAndEquipmentRentalAndLeasing,
    /// Lessors of Nonfinancial Intangible Assets (except Copyrighted Works)
    LessorsOfNonfinancialIntangibleAssetsExceptCopyrightedWorks,
    /// Offices of Lawyers
    OfficesOfLawyers,
    /// Offices of Notaries
    OfficesOfNotaries,
    /// Other Legal Services
    OtherLegalServices,
    /// Accounting, Tax Preparation, Bookkeeping, and Payroll Services
    AccountingTaxPreparationBookkeepingAndPayrollServices,
    /// Architectural Services
    ArchitecturalServices,
    /// Landscape Architectural Services
    LandscapeArchitecturalServices,
    /// Engineering Services
    EngineeringServices,
    /// Drafting Services
    DraftingServices,
    /// Building Inspection Services
    BuildingInspectionServices,
    /// Geophysical Surveying and Mapping Services
    GeophysicalSurveyingAndMappingServices,
    /// Surveying and Mapping (except Geophysical) Services
    SurveyingAndMappingExceptGeophysicalServices,
    /// Testing Laboratories
    TestingLaboratories,
    /// Interior Design Services
    InteriorDesignServices,
    /// Industrial Design Services
    IndustrialDesignServices,
    /// Graphic Design Services
    GraphicDesignServices,
    /// Other Specialized Design Services
    OtherSpecializedDesignServices,
    /// Computer Systems Design and Related Services
    ComputerSystemsDesignAndRelatedServices,
    /// Management Consulting Services
    ManagementConsultingServices,
    /// Environmental Consulting Services
    EnvironmentalConsultingServices,
    /// Other Scientific and Technical Consulting Services
    OtherScientificAndTechnicalConsultingServices,
    /// Research and Development in the Physical, Engineering, and Life Sciences
    ResearchAndDevelopmentInThePhysicalEngineeringAndLifeSciences,
    /// Research and Development in the Social Sciences and Humanities
    ResearchAndDevelopmentInTheSocialSciencesAndHumanities,
    /// Advertising Agencies
    AdvertisingAgencies,
    /// Public Relations Agencies
    PublicRelationsAgencies,
    /// Media Buying Agencies
    MediaBuyingAgencies,
    /// Media Representatives
    MediaRepresentatives,
    /// Outdoor Advertising
    OutdoorAdvertising,
    /// Direct Mail Advertising
    DirectMailAdvertising,
    /// Advertising Material Distribution Services
    AdvertisingMaterialDistributionServices,
    /// Other Services Related to Advertising
    OtherServicesRelatedToAdvertising,
    /// Marketing Research and Public Opinion Polling
    MarketingResearchAndPublicOpinionPolling,
    /// Photographic Services
    PhotographicServices,
    /// Translation and Interpretation Services
    TranslationAndInterpretationServices,
    /// Veterinary Services
    VeterinaryServices,
    /// All Other Professional, Scientific, and Technical Services
    AllOtherProfessionalScientificAndTechnicalServices,
    /// Management of Companies and Enterprises
    ManagementOfCompaniesAndEnterprises,
    /// Office Administrative Services
    OfficeAdministrativeServices,
    /// Facilities Support Services
    FacilitiesSupportServices,
    /// Employment Placement Agencies and Executive Search Services
    EmploymentPlacementAgenciesAndExecutiveSearchServices,
    /// Temporary Help Services
    TemporaryHelpServices,
    /// Professional Employer Organizations
    ProfessionalEmployerOrganizations,
    /// Document Preparation Services
    DocumentPreparationServices,
    /// Telephone Call Centers
    TelephoneCallCenters,
    /// Business Service Centers
    BusinessServiceCenters,
    /// Collection Agencies
    CollectionAgencies,
    /// Credit Bureaus
    CreditBureaus,
    /// Other Business Support Services
    OtherBusinessSupportServices,
    /// Travel Agencies
    TravelAgencies,
    /// Tour Operators
    TourOperators,
    /// Other Travel Arrangement and Reservation Services
    OtherTravelArrangementAndReservationServices,
    /// Investigation, Guard, and Armored Car Services
    InvestigationGuardAndArmoredCarServices,
    /// Security Systems Services
    SecuritySystemsServices,
    /// Exterminating and Pest Control Services
    ExterminatingAndPestControlServices,
    /// Janitorial Services
    JanitorialServices,
    /// Landscaping Services
    LandscapingServices,
    /// Carpet and Upholstery Cleaning Services
    CarpetAndUpholsteryCleaningServices,
    /// Other Services to Buildings and Dwellings
    OtherServicesToBuildingsAndDwellings,
    /// Packaging and Labeling Services
    PackagingAndLabelingServices,
    /// Convention and Trade Show Organizers
    ConventionAndTradeShowOrganizers,
    /// All Other Support Services
    AllOtherSupportServices,
    /// Waste Collection
    WasteCollection,
    /// Waste Treatment and Disposal
    WasteTreatmentAndDisposal,
    /// Remediation Services
    RemediationServices,
    /// Materials Recovery Facilities
    MaterialsRecoveryFacilities,
    /// All Other Waste Management Services
    AllOtherWasteManagementServices,
    /// Elementary and Secondary Schools
    ElementaryAndSecondarySchools,
    /// Junior Colleges
    JuniorColleges,
    /// Colleges, Universities, and Professional Schools
    CollegesUniversitiesAndProfessionalSchools,
    /// Business and Secretarial Schools
    BusinessAndSecretarialSchools,
    /// Computer Training
    ComputerTraining,
    /// Professional and Management Development Training
    ProfessionalAndManagementDevelopmentTraining,
    /// Technical and Trade Schools
    TechnicalAndTradeSchools,
    /// Fine Arts Schools
    FineArtsSchools,
    /// Sports and Recreation Instruction
    SportsAndRecreationInstruction,
    /// Language Schools
    LanguageSchools,
    /// All Other Schools and Instruction
    AllOtherSchoolsAndInstruction,
    /// Educational Support Services
    EducationalSupportServices,
    /// Offices of Physicians
    OfficesOfPhysicians,
    /// Offices of Dentists
    OfficesOfDentists,
    /// Offices of Chiropractors
    OfficesOfChiropractors,
    /// Offices of Optometrists
    OfficesOfOptometrists,
    /// Offices of Mental Health Practitioners (except Physicians)
    OfficesOfMentalHealthPractitionersExceptPhysicians,
    /// Offices of Physical, Occupational and Speech Therapists, and Audiologists
    OfficesOfPhysicalOccupationalAndSpeechTherapistsAndAudiologists,
    /// Offices of All Other Health Practitioners
    OfficesOfAllOtherHealthPractitioners,
    /// Family Planning Centers
    FamilyPlanningCenters,
    /// Outpatient Mental Health and Substance Abuse Centers
    OutpatientMentalHealthAndSubstanceAbuseCenters,
    /// Other Outpatient Care Centers
    OtherOutpatientCareCenters,
    /// Medical and Diagnostic Laboratories
    MedicalAndDiagnosticLaboratories,
    /// Home Health Care Services
    HomeHealthCareServices,
    /// Ambulance Services
    AmbulanceServices,
    /// All Other Ambulatory Health Care Services
    AllOtherAmbulatoryHealthCareServices,
    /// General Medical and Surgical Hospitals
    GeneralMedicalAndSurgicalHospitals,
    /// Psychiatric and Substance Abuse Hospitals
    PsychiatricAndSubstanceAbuseHospitals,
    /// Specialty (except Psychiatric and Substance Abuse) Hospitals
    SpecialtyExceptPsychiatricAndSubstanceAbuseHospitals,
    /// Nursing Care Facilities (Skilled Nursing Facilities)
    NursingCareFacilitiesSkilledNursingFacilities,
    /// Residential Intellectual and Developmental Disability Facilities
    ResidentialIntellectualAndDevelopmentalDisabilityFacilities,
    /// Residential Mental Health and Substance Abuse Facilities
    ResidentialMentalHealthAndSubstanceAbuseFacilities,
    /// Continuing Care Retirement Communities and Assisted Living Facilities for the Elderly
    ContinuingCareRetirementCommunitiesAndAssistedLivingFacilitiesForTheElderly,
    /// Other Residential Care Facilities
    OtherResidentialCareFacilities,
    /// Child and Youth Services
    ChildAndYouthServices,
    /// Services for the Elderly and Persons with Disabilities
    ServicesForTheElderlyAndPersonsWithDisabilities,
    /// Other Individual and Family Services
    OtherIndividualAndFamilyServices,
    /// Community Food Services
    CommunityFoodServices,
    /// Community Housing Services
    CommunityHousingServices,
    /// Emergency and Other Relief Services
    EmergencyAndOtherReliefServices,
    /// Vocational Rehabilitation Services
    VocationalRehabilitationServices,
    /// Child Day Care Services
    ChildDayCareServices,
    /// Theater Companies and Dinner Theaters
    TheaterCompaniesAndDinnerTheaters,
    /// Dance Companies
    DanceCompanies,
    /// Musical Groups and Artists
    MusicalGroupsAndArtists,
    /// Other Performing Arts Companies
    OtherPerformingArtsCompanies,
    /// Spectator Sports
    SpectatorSports,
    /// Promoters of Performing Arts, Sports, and Similar Events with Facilities
    PromotersOfPerformingArtsSportsAndSimilarEventsWithFacilities,
    /// Promoters of Performing Arts, Sports, and Similar Events without Facilities
    PromotersOfPerformingArtsSportsAndSimilarEventsWithoutFacilities,
    /// Agents and Managers for Artists, Athletes, Entertainers, and Other Public Figures
    AgentsAndManagersForArtistsAthletesEntertainersAndOtherPublicFigures,
    /// Independent Artists, Writers, and Performers
    IndependentArtistsWritersAndPerformers,
    /// Museums
    Museums,
    /// Historical Sites
    HistoricalSites,
    /// Zoos and Botanical Gardens
    ZoosAndBotanicalGardens,
    /// Nature Parks and Other Similar Institutions
    NatureParksAndOtherSimilarInstitutions,
    /// Amusement and Theme Parks
    AmusementAndThemeParks,
    /// Amusement Arcades
    AmusementArcades,
    /// Casinos (except Casino Hotels)
    CasinosExceptCasinoHotels,
    /// Other Gambling Industries
    OtherGamblingIndustries,
    /// Golf Courses and Country Clubs
    GolfCoursesAndCountryClubs,
    /// Skiing Facilities
    SkiingFacilities,
    /// Marinas
    Marinas,
    /// Fitness and Recreational Sports Centers
    FitnessAndRecreationalSportsCenters,
    /// Bowling Centers
    BowlingCenters,
    /// All Other Amusement and Recreation Industries
    AllOtherAmusementAndRecreationIndustries,
    /// Hotels (except Casino Hotels) and Motels
    HotelsExceptCasinoHotelsAndMotels,
    /// Casino Hotels
    CasinoHotels,
    /// Other Traveler Accommodation
    OtherTravelerAccommodation,
    /// RV (Recreational Vehicle) Parks and Recreational Camps
    RvRecreationalVehicleParksAndRecreationalCamps,
    /// Rooming and Boarding Houses
    RoomingAndBoardingHouses,
    /// Food Service Contractors
    FoodServiceContractors,
    /// Caterers
    Caterers,
    /// Mobile Food Services
    MobileFoodServices,
    /// Drinking Places (Alcoholic Beverages)
    DrinkingPlacesAlcoholicBeverages,
    /// Restaurants and Other Eating Places
    RestaurantsAndOtherEatingPlaces,
    /// Automotive Mechanical and Electrical Repair and Maintenance
    AutomotiveMechanicalAndElectricalRepairAndMaintenance,
    /// Automotive Body, Paint, Interior, and Glass Repair
    AutomotiveBodyPaintInteriorAndGlassRepair,
    /// Other Automotive Repair and Maintenance
    OtherAutomotiveRepairAndMaintenance,
    /// Electronic and Precision Equipment Repair and Maintenance
    ElectronicAndPrecisionEquipmentRepairAndMaintenance,
    /// Commercial and Industrial Machinery and Equipment (except Automotive and Electronic) Repair and Maintenance
    CommercialAndIndustrialMachineryAndEquipmentExceptAutomotiveAndElectronicRepairAndMaintenance,
    /// Home and Garden Equipment and Appliance Repair and Maintenance
    HomeAndGardenEquipmentAndApplianceRepairAndMaintenance,
    /// Reupholstery and Furniture Repair
    ReupholsteryAndFurnitureRepair,
    /// Footwear and Leather Goods Repair
    FootwearAndLeatherGoodsRepair,
    /// Other Personal and Household Goods Repair and Maintenance
    OtherPersonalAndHouseholdGoodsRepairAndMaintenance,
    /// Hair, Nail, and Skin Care Services
    HairNailAndSkinCareServices,
    /// Other Personal Care Services
    OtherPersonalCareServices,
    /// Funeral Homes and Funeral Services
    FuneralHomesAndFuneralServices,
    /// Cemeteries and Crematories
    CemeteriesAndCrematories,
    /// Coin-Operated Laundries and Drycleaners
    CoinOperatedLaundriesAndDrycleaners,
    /// Drycleaning and Laundry Services (except Coin-Operated)
    DrycleaningAndLaundryServicesExceptCoinOperated,
    /// Linen and Uniform Supply
    LinenAndUniformSupply,
    /// Pet Care (except Veterinary) Services
    PetCareExceptVeterinaryServices,
    /// Photofinishing
    Photofinishing,
    /// Parking Lots and Garages
    ParkingLotsAndGarages,
    /// All Other Personal Services
    AllOtherPersonalServices,
    /// Religious Organizations
    ReligiousOrganizations,
    /// Grantmaking and Giving Services
    GrantmakingAndGivingServices,
    /// Social Advocacy Organizations
    SocialAdvocacyOrganizations,
    /// Civic and Social Organizations
    CivicAndSocialOrganizations,
    /// Business Associations
    BusinessAssociations,
    /// Professional Organizations
    ProfessionalOrganizations,
    /// Labor Unions and Similar Labor Organizations
    LaborUnionsAndSimilarLaborOrganizations,
    /// Political Organizations
    PoliticalOrganizations,
    /// Other Similar Organizations (except Business, Professional, Labor, and Political Organizations)
    OtherSimilarOrganizationsExceptBusinessProfessionalLaborAndPoliticalOrganizations,
    /// Private Households
    PrivateHouseholds,
    /// Executive Offices
    ExecutiveOffices,
    /// Legislative Bodies
    LegislativeBodies,
    /// Public Finance Activities
    PublicFinanceActivities,
    /// Executive and Legislative Offices, Combined
    ExecutiveAndLegislativeOfficesCombined,
    /// American Indian and Alaska Native Tribal Governments
    AmericanIndianAndAlaskaNativeTribalGovernments,
    /// Other General Government Support
    OtherGeneralGovernmentSupport,
    /// Courts
    Courts,
    /// Police Protection
    PoliceProtection,
    /// Legal Counsel and Prosecution
    LegalCounselAndProsecution,
    /// Correctional Institutions
    CorrectionalInstitutions,
    /// Parole Offices and Probation Offices
    ParoleOfficesAndProbationOffices,
    /// Fire Protection
    FireProtection,
    /// Other Justice, Public Order, and Safety Activities
    OtherJusticePublicOrderAndSafetyActivities,
    /// Administration of Education Programs
    AdministrationOfEducationPrograms,
    /// Administration of Public Health Programs
    AdministrationOfPublicHealthPrograms,
    /// Administration of Human Resource Programs (except Education, Public Health, and Veterans' Affairs Programs)
    AdministrationOfHumanResourceProgramsExceptEducationPublicHealthAndVeteransAffairsPrograms,
    /// Administration of Veterans' Affairs
    AdministrationOfVeteransAffairs,
    /// Administration of Air and Water Resource and Solid Waste Management Programs
    AdministrationOfAirAndWaterResourceAndSolidWasteManagementPrograms,
    /// Administration of Conservation Programs
    AdministrationOfConservationPrograms,
    /// Administration of Housing Programs
    AdministrationOfHousingPrograms,
    /// Administration of Urban Planning and Community and Rural Development
    AdministrationOfUrbanPlanningAndCommunityAndRuralDevelopment,
    /// Administration of General Economic Programs
    AdministrationOfGeneralEconomicPrograms,
    /// Regulation and Administration of Transportation Programs
    RegulationAndAdministrationOfTransportationPrograms,
    /// Regulation and Administration of Communications, Electric, Gas, and Other Utilities
    RegulationAndAdministrationOfCommunicationsElectricGasAndOtherUtilities,
    /// Regulation of Agricultural Marketing and Commodities
    RegulationOfAgriculturalMarketingAndCommodities,
    /// Regulation, Licensing, and Inspection of Miscellaneous Commercial Sectors
    RegulationLicensingAndInspectionOfMiscellaneousCommercialSectors,
    /// Space Research and Technology
    SpaceResearchAndTechnology,
    /// National Security
    NationalSecurity,
    /// International Affairs
    InternationalAffairs,
    /// Unclassified Establishments
    UnclassifiedEstablishments,
}

impl NaicsSubcategory {
    /// Returns the NAICS title description for this subcategory
    pub fn description(&self) -> &'static str {
        match self {
            Self::SoybeanFarming => "Soybean Farming",
            Self::OilseedExceptSoybeanFarming => "Oilseed (except Soybean) Farming",
            Self::DryPeaAndBeanFarming => "Dry Pea and Bean Farming",
            Self::WheatFarming => "Wheat Farming",
            Self::CornFarming => "Corn Farming",
            Self::RiceFarming => "Rice Farming",
            Self::OtherGrainFarming => "Other Grain Farming",
            Self::VegetableAndMelonFarming => "Vegetable and Melon Farming",
            Self::OrangeGroves => "Orange Groves",
            Self::CitrusExceptOrangeGroves => "Citrus (except Orange) Groves",
            Self::NoncitrusFruitAndTreeNutFarming => "Noncitrus Fruit and Tree Nut Farming",
            Self::FoodCropsGrownUnderCover => "Food Crops Grown Under Cover",
            Self::NurseryAndFloricultureProduction => "Nursery and Floriculture Production",
            Self::TobaccoFarming => "Tobacco Farming",
            Self::CottonFarming => "Cotton Farming",
            Self::SugarcaneFarming => "Sugarcane Farming",
            Self::HayFarming => "Hay Farming",
            Self::AllOtherCropFarming => "All Other Crop Farming",
            Self::BeefCattleRanchingAndFarmingIncludingFeedlots => {
                "Beef Cattle Ranching and Farming, including Feedlots"
            }
            Self::DairyCattleAndMilkProduction => "Dairy Cattle and Milk Production",
            Self::DualpurposeCattleRanchingAndFarming => "Dual-Purpose Cattle Ranching and Farming",
            Self::HogAndPigFarming => "Hog and Pig Farming",
            Self::ChickenEggProduction => "Chicken Egg Production",
            Self::BroilersAndOtherMeatTypeChickenProduction => {
                "Broilers and Other Meat Type Chicken Production"
            }
            Self::TurkeyProduction => "Turkey Production",
            Self::PoultryHatcheries => "Poultry Hatcheries",
            Self::OtherPoultryProduction => "Other Poultry Production",
            Self::SheepFarming => "Sheep Farming",
            Self::GoatFarming => "Goat Farming",
            Self::Aquaculture => "Aquaculture",
            Self::Apiculture => "Apiculture",
            Self::HorsesAndOtherEquineProduction => "Horses and Other Equine Production",
            Self::FurbearingAnimalAndRabbitProduction => "Fur-Bearing Animal and Rabbit Production",
            Self::AllOtherAnimalProduction => "All Other Animal Production",
            Self::TimberTractOperations => "Timber Tract Operations",
            Self::ForestNurseriesAndGatheringOfForestProducts => {
                "Forest Nurseries and Gathering of Forest Products"
            }
            Self::Logging => "Logging",
            Self::Fishing => "Fishing",
            Self::HuntingAndTrapping => "Hunting and Trapping",
            Self::SupportActivitiesForCropProduction => "Support Activities for Crop Production",
            Self::SupportActivitiesForAnimalProduction => {
                "Support Activities for Animal Production"
            }
            Self::SupportActivitiesForForestry => "Support Activities for Forestry",
            Self::CrudePetroleumExtraction => "Crude Petroleum Extraction",
            Self::NaturalGasExtraction => "Natural Gas Extraction",
            Self::CoalMining => "Coal Mining",
            Self::IronOreMining => "Iron Ore Mining",
            Self::GoldOreAndSilverOreMining => "Gold Ore and Silver Ore Mining",
            Self::CopperNickelLeadAndZincMining => "Copper, Nickel, Lead, and Zinc Mining",
            Self::OtherMetalOreMining => "Other Metal Ore Mining",
            Self::StoneMiningAndQuarrying => "Stone Mining and Quarrying",
            Self::SandGravelClayAndCeramicAndRefractoryMineralsMiningAndQuarrying => {
                "Sand, Gravel, Clay, and Ceramic and Refractory Minerals Mining and Quarrying"
            }
            Self::OtherNonmetallicMineralMiningAndQuarrying => {
                "Other Nonmetallic Mineral Mining and Quarrying"
            }
            Self::SupportActivitiesForMining => "Support Activities for Mining",
            Self::ElectricPowerGeneration => "Electric Power Generation ",
            Self::ElectricPowerTransmissionControlAndDistribution => {
                "Electric Power Transmission, Control, and Distribution "
            }
            Self::NaturalGasDistribution => "Natural Gas Distribution ",
            Self::WaterSupplyAndIrrigationSystems => "Water Supply and Irrigation Systems ",
            Self::SewageTreatmentFacilities => "Sewage Treatment Facilities ",
            Self::SteamAndAirconditioningSupply => "Steam and Air-Conditioning Supply ",
            Self::ResidentialBuildingConstruction => "Residential Building Construction",
            Self::IndustrialBuildingConstruction => "Industrial Building Construction",
            Self::CommercialAndInstitutionalBuildingConstruction => {
                "Commercial and Institutional Building Construction"
            }
            Self::WaterAndSewerLineAndRelatedStructuresConstruction => {
                "Water and Sewer Line and Related Structures Construction"
            }
            Self::OilAndGasPipelineAndRelatedStructuresConstruction => {
                "Oil and Gas Pipeline and Related Structures Construction"
            }
            Self::PowerAndCommunicationLineAndRelatedStructuresConstruction => {
                "Power and Communication Line and Related Structures Construction"
            }
            Self::LandSubdivision => "Land Subdivision",
            Self::HighwayStreetAndBridgeConstruction => "Highway, Street, and Bridge Construction",
            Self::OtherHeavyAndCivilEngineeringConstruction => {
                "Other Heavy and Civil Engineering Construction"
            }
            Self::PouredConcreteFoundationAndStructureContractors => {
                "Poured Concrete Foundation and Structure Contractors "
            }
            Self::StructuralSteelAndPrecastConcreteContractors => {
                "Structural Steel and Precast Concrete Contractors "
            }
            Self::FramingContractors => "Framing Contractors ",
            Self::MasonryContractors => "Masonry Contractors ",
            Self::GlassAndGlazingContractors => "Glass and Glazing Contractors ",
            Self::RoofingContractors => "Roofing Contractors ",
            Self::SidingContractors => "Siding Contractors ",
            Self::OtherFoundationStructureAndBuildingExteriorContractors => {
                "Other Foundation, Structure, and Building Exterior Contractors "
            }
            Self::ElectricalContractorsAndOtherWiringInstallationContractors => {
                "Electrical Contractors and Other Wiring Installation Contractors"
            }
            Self::PlumbingHeatingAndAirconditioningContractors => {
                "Plumbing, Heating, and Air-Conditioning Contractors"
            }
            Self::OtherBuildingEquipmentContractors => "Other Building Equipment Contractors",
            Self::DrywallAndInsulationContractors => "Drywall and Insulation Contractors",
            Self::PaintingAndWallCoveringContractors => "Painting and Wall Covering Contractors",
            Self::FlooringContractors => "Flooring Contractors",
            Self::TileAndTerrazzoContractors => "Tile and Terrazzo Contractors",
            Self::FinishCarpentryContractors => "Finish Carpentry Contractors",
            Self::OtherBuildingFinishingContractors => "Other Building Finishing Contractors",
            Self::SitePreparationContractors => "Site Preparation Contractors",
            Self::AllOtherSpecialtyTradeContractors => "All Other Specialty Trade Contractors",
            Self::AnimalFoodManufacturing => "Animal Food Manufacturing",
            Self::FlourMillingAndMaltManufacturing => "Flour Milling and Malt Manufacturing",
            Self::StarchAndVegetableFatsAndOilsManufacturing => {
                "Starch and Vegetable Fats and Oils Manufacturing"
            }
            Self::BreakfastCerealManufacturing => "Breakfast Cereal Manufacturing",
            Self::SugarManufacturing => "Sugar Manufacturing",
            Self::NonchocolateConfectioneryManufacturing => {
                "Nonchocolate Confectionery Manufacturing"
            }
            Self::ChocolateAndConfectioneryManufacturing => {
                "Chocolate and Confectionery Manufacturing"
            }
            Self::FrozenFoodManufacturing => "Frozen Food Manufacturing",
            Self::FruitAndVegetableCanningPicklingAndDrying => {
                "Fruit and Vegetable Canning, Pickling, and Drying"
            }
            Self::DairyProductExceptFrozenManufacturing => {
                "Dairy Product (except Frozen) Manufacturing"
            }
            Self::IceCreamAndFrozenDessertManufacturing => {
                "Ice Cream and Frozen Dessert Manufacturing"
            }
            Self::AnimalSlaughteringAndProcessing => "Animal Slaughtering and Processing",
            Self::SeafoodProductPreparationAndPackaging => {
                "Seafood Product Preparation and Packaging"
            }
            Self::BreadAndBakeryProductManufacturing => "Bread and Bakery Product Manufacturing",
            Self::CookieCrackerAndPastaManufacturing => "Cookie, Cracker, and Pasta Manufacturing",
            Self::TortillaManufacturing => "Tortilla Manufacturing",
            Self::SnackFoodManufacturing => "Snack Food Manufacturing",
            Self::CoffeeAndTeaManufacturing => "Coffee and Tea Manufacturing",
            Self::FlavoringSyrupAndConcentrateManufacturing => {
                "Flavoring Syrup and Concentrate Manufacturing"
            }
            Self::SeasoningAndDressingManufacturing => "Seasoning and Dressing Manufacturing",
            Self::AllOtherFoodManufacturing => "All Other Food Manufacturing",
            Self::SoftDrinkAndIceManufacturing => "Soft Drink and Ice Manufacturing",
            Self::Breweries => "Breweries",
            Self::Wineries => "Wineries",
            Self::Distilleries => "Distilleries",
            Self::TobaccoManufacturing => "Tobacco Manufacturing",
            Self::FiberYarnAndThreadMills => "Fiber, Yarn, and Thread Mills",
            Self::BroadwovenFabricMills => "Broadwoven Fabric Mills",
            Self::NarrowFabricMillsAndSchiffliMachineEmbroidery => {
                "Narrow Fabric Mills and Schiffli Machine Embroidery"
            }
            Self::NonwovenFabricMills => "Nonwoven Fabric Mills",
            Self::KnitFabricMills => "Knit Fabric Mills",
            Self::TextileAndFabricFinishingMills => "Textile and Fabric Finishing Mills",
            Self::FabricCoatingMills => "Fabric Coating Mills",
            Self::CarpetAndRugMills => "Carpet and Rug Mills",
            Self::CurtainAndLinenMills => "Curtain and Linen Mills",
            Self::TextileBagAndCanvasMills => "Textile Bag and Canvas Mills",
            Self::AllOtherTextileProductMills => "All Other Textile Product Mills",
            Self::ApparelKnittingMills => "Apparel Knitting Mills",
            Self::CutAndSewApparelContractors => "Cut and Sew Apparel Contractors",
            Self::CutAndSewApparelManufacturingExceptContractors => {
                "Cut and Sew Apparel Manufacturing (except Contractors)"
            }
            Self::MensAndBoysCutAndSewApparelManufacturing => {
                "Men's and Boys' Cut and Sew Apparel Manufacturing"
            }
            Self::WomensAndGirlsCutAndSewApparelManufacturing => {
                "Women's and Girls' Cut and Sew Apparel Manufacturing"
            }
            Self::OtherCutAndSewApparelManufacturing => "Other Cut and Sew Apparel Manufacturing",
            Self::ApparelAccessoriesAndOtherApparelManufacturing => {
                "Apparel Accessories and Other Apparel Manufacturing"
            }
            Self::LeatherAndHideTanningAndFinishing => "Leather and Hide Tanning and Finishing",
            Self::FootwearManufacturing => "Footwear Manufacturing",
            Self::OtherLeatherAndAlliedProductManufacturing => {
                "Other Leather and Allied Product Manufacturing"
            }
            Self::SawmillsAndWoodPreservation => "Sawmills and Wood Preservation",
            Self::VeneerPlywoodAndEngineeredWoodProductManufacturing => {
                "Veneer, Plywood, and Engineered Wood Product Manufacturing"
            }
            Self::Millwork => "Millwork",
            Self::WoodContainerAndPalletManufacturing => "Wood Container and Pallet Manufacturing",
            Self::AllOtherWoodProductManufacturing => "All Other Wood Product Manufacturing",
            Self::PulpMills => "Pulp Mills",
            Self::PaperMills => "Paper Mills",
            Self::PaperboardMills => "Paperboard Mills",
            Self::PaperboardContainerManufacturing => "Paperboard Container Manufacturing",
            Self::PaperBagAndCoatedAndTreatedPaperManufacturing => {
                "Paper Bag and Coated and Treated Paper Manufacturing"
            }
            Self::StationeryProductManufacturing => "Stationery Product Manufacturing",
            Self::OtherConvertedPaperProductManufacturing => {
                "Other Converted Paper Product Manufacturing"
            }
            Self::Printing => "Printing",
            Self::SupportActivitiesForPrinting => "Support Activities for Printing",
            Self::PetroleumRefineries => "Petroleum Refineries",
            Self::AsphaltPavingRoofingAndSaturatedMaterialsManufacturing => {
                "Asphalt Paving, Roofing, and Saturated Materials Manufacturing"
            }
            Self::OtherPetroleumAndCoalProductsManufacturing => {
                "Other Petroleum and Coal Products Manufacturing"
            }
            Self::PetrochemicalManufacturing => "Petrochemical Manufacturing",
            Self::IndustrialGasManufacturing => "Industrial Gas Manufacturing",
            Self::SyntheticDyeAndPigmentManufacturing => "Synthetic Dye and Pigment Manufacturing",
            Self::OtherBasicInorganicChemicalManufacturing => {
                "Other Basic Inorganic Chemical Manufacturing"
            }
            Self::OtherBasicOrganicChemicalManufacturing => {
                "Other Basic Organic Chemical Manufacturing"
            }
            Self::ResinAndSyntheticRubberManufacturing => {
                "Resin and Synthetic Rubber Manufacturing"
            }
            Self::ArtificialAndSyntheticFibersAndFilamentsManufacturing => {
                "Artificial and Synthetic Fibers and Filaments Manufacturing"
            }
            Self::FertilizerAndCompostManufacturing => "Fertilizer and Compost Manufacturing",
            Self::PesticideAndOtherAgriculturalChemicalManufacturing => {
                "Pesticide and Other Agricultural Chemical Manufacturing"
            }
            Self::PharmaceuticalAndMedicineManufacturing => {
                "Pharmaceutical and Medicine Manufacturing"
            }
            Self::PaintAndCoatingManufacturing => "Paint and Coating Manufacturing",
            Self::AdhesiveManufacturing => "Adhesive Manufacturing",
            Self::SoapAndCleaningCompoundManufacturing => {
                "Soap and Cleaning Compound Manufacturing"
            }
            Self::ToiletPreparationManufacturing => "Toilet Preparation Manufacturing",
            Self::PrintingInkManufacturing => "Printing Ink Manufacturing",
            Self::ExplosivesManufacturing => "Explosives Manufacturing",
            Self::AllOtherChemicalProductAndPreparationManufacturing => {
                "All Other Chemical Product and Preparation Manufacturing"
            }
            Self::PlasticsPackagingMaterialsAndUnlaminatedFilmAndSheetManufacturing => {
                "Plastics Packaging Materials and Unlaminated Film and Sheet Manufacturing"
            }
            Self::PlasticsPipePipeFittingAndUnlaminatedProfileShapeManufacturing => {
                "Plastics Pipe, Pipe Fitting, and Unlaminated Profile Shape Manufacturing"
            }
            Self::LaminatedPlasticsPlateSheetExceptPackagingAndShapeManufacturing => {
                "Laminated Plastics Plate, Sheet (except Packaging), and Shape Manufacturing"
            }
            Self::PolystyreneFoamProductManufacturing => "Polystyrene Foam Product Manufacturing",
            Self::UrethaneAndOtherFoamProductExceptPolystyreneManufacturing => {
                "Urethane and Other Foam Product (except Polystyrene) Manufacturing"
            }
            Self::PlasticsBottleManufacturing => "Plastics Bottle Manufacturing",
            Self::OtherPlasticsProductManufacturing => "Other Plastics Product Manufacturing",
            Self::TireManufacturing => "Tire Manufacturing",
            Self::RubberAndPlasticsHosesAndBeltingManufacturing => {
                "Rubber and Plastics Hoses and Belting Manufacturing"
            }
            Self::OtherRubberProductManufacturing => "Other Rubber Product Manufacturing",
            Self::PotteryCeramicsAndPlumbingFixtureManufacturing => {
                "Pottery, Ceramics, and Plumbing Fixture Manufacturing"
            }
            Self::ClayBuildingMaterialAndRefractoriesManufacturing => {
                "Clay Building Material and Refractories Manufacturing"
            }
            Self::GlassAndGlassProductManufacturing => "Glass and Glass Product Manufacturing",
            Self::CementManufacturing => "Cement Manufacturing",
            Self::ReadymixConcreteManufacturing => "Ready-Mix Concrete Manufacturing",
            Self::ConcretePipeBrickAndBlockManufacturing => {
                "Concrete Pipe, Brick, and Block Manufacturing"
            }
            Self::OtherConcreteProductManufacturing => "Other Concrete Product Manufacturing",
            Self::LimeManufacturing => "Lime Manufacturing",
            Self::GypsumProductManufacturing => "Gypsum Product Manufacturing",
            Self::AbrasiveProductManufacturing => "Abrasive Product Manufacturing",
            Self::AllOtherNonmetallicMineralProductManufacturing => {
                "All Other Nonmetallic Mineral Product Manufacturing"
            }
            Self::IronAndSteelMillsAndFerroalloyManufacturing => {
                "Iron and Steel Mills and Ferroalloy Manufacturing"
            }
            Self::IronAndSteelPipeAndTubeManufacturingFromPurchasedSteel => {
                "Iron and Steel Pipe and Tube Manufacturing from Purchased Steel"
            }
            Self::RollingAndDrawingOfPurchasedSteel => "Rolling and Drawing of Purchased Steel",
            Self::AluminaAndAluminumProductionAndProcessing => {
                "Alumina and Aluminum Production and Processing"
            }
            Self::NonferrousMetalExceptAluminumSmeltingAndRefining => {
                "Nonferrous Metal (except Aluminum) Smelting and Refining"
            }
            Self::CopperRollingDrawingExtrudingAndAlloying => {
                "Copper Rolling, Drawing, Extruding, and Alloying"
            }
            Self::NonferrousMetalExceptCopperAndAluminumRollingDrawingExtrudingAndAlloying => {
                "Nonferrous Metal (except Copper and Aluminum) Rolling, Drawing, Extruding, and Alloying"
            }
            Self::FerrousMetalFoundries => "Ferrous Metal Foundries",
            Self::NonferrousMetalFoundries => "Nonferrous Metal Foundries",
            Self::ForgingAndStamping => "Forging and Stamping",
            Self::CutleryAndHandtoolManufacturing => "Cutlery and Handtool Manufacturing",
            Self::PlateWorkAndFabricatedStructuralProductManufacturing => {
                "Plate Work and Fabricated Structural Product Manufacturing"
            }
            Self::OrnamentalAndArchitecturalMetalProductsManufacturing => {
                "Ornamental and Architectural Metal Products Manufacturing"
            }
            Self::PowerBoilerAndHeatExchangerManufacturing => {
                "Power Boiler and Heat Exchanger Manufacturing"
            }
            Self::MetalTankHeavyGaugeManufacturing => "Metal Tank (Heavy Gauge) Manufacturing",
            Self::MetalCanBoxAndOtherMetalContainerLightGaugeManufacturing => "Metal Can, Box, and Other Metal Container (Light Gauge) Manufacturing",
            Self::HardwareManufacturing => "Hardware Manufacturing",
            Self::SpringAndWireProductManufacturing => "Spring and Wire Product Manufacturing",
            Self::MachineShops => "Machine Shops",
            Self::TurnedProductAndScrewNutAndBoltManufacturing => "Turned Product and Screw, Nut, and Bolt Manufacturing",
            Self::CoatingEngravingHeatTreatingAndAlliedActivities => "Coating, Engraving, Heat Treating, and Allied Activities",
            Self::MetalValveManufacturing => "Metal Valve Manufacturing",
            Self::AllOtherFabricatedMetalProductManufacturing => "All Other Fabricated Metal Product Manufacturing",
            Self::AgriculturalImplementManufacturing => "Agricultural Implement Manufacturing",
            Self::ConstructionMachineryManufacturing => "Construction Machinery Manufacturing",
            Self::MiningAndOilAndGasFieldMachineryManufacturing => "Mining and Oil and Gas Field Machinery Manufacturing",
            Self::IndustrialMachineryManufacturing => "Industrial Machinery Manufacturing",
            Self::CommercialAndServiceIndustryMachineryManufacturing => "Commercial and Service Industry Machinery Manufacturing",
            Self::VentilationHeatingAirconditioningAndCommercialRefrigerationEquipmentManufacturing => "Ventilation, Heating, Air-Conditioning, and Commercial Refrigeration Equipment Manufacturing",
            Self::MetalworkingMachineryManufacturing => "Metalworking Machinery Manufacturing",
            Self::EngineTurbineAndPowerTransmissionEquipmentManufacturing => "Engine, Turbine, and Power Transmission Equipment Manufacturing",
            Self::PumpAndCompressorManufacturing => "Pump and Compressor Manufacturing",
            Self::MaterialHandlingEquipmentManufacturing => "Material Handling Equipment Manufacturing",
            Self::AllOtherGeneralPurposeMachineryManufacturing => "All Other General Purpose Machinery Manufacturing",
            Self::ComputerAndPeripheralEquipmentManufacturing => "Computer and Peripheral Equipment Manufacturing",
            Self::TelephoneApparatusManufacturing => "Telephone Apparatus Manufacturing",
            Self::RadioAndTelevisionBroadcastingAndWirelessCommunicationsEquipmentManufacturing => "Radio and Television Broadcasting and Wireless Communications Equipment Manufacturing",
            Self::OtherCommunicationsEquipmentManufacturing => "Other Communications Equipment Manufacturing",
            Self::AudioAndVideoEquipmentManufacturing => "Audio and Video Equipment Manufacturing",
            Self::SemiconductorAndOtherElectronicComponentManufacturing => "Semiconductor and Other Electronic Component Manufacturing",
            Self::NavigationalMeasuringElectromedicalAndControlInstrumentsManufacturing => "Navigational, Measuring, Electromedical, and Control Instruments Manufacturing",
            Self::ManufacturingAndReproducingMagneticAndOpticalMedia => "Manufacturing and Reproducing Magnetic and Optical Media",
            Self::ElectricLightingEquipmentManufacturing => "Electric Lighting Equipment Manufacturing",
            Self::SmallElectricalApplianceManufacturing => "Small Electrical Appliance Manufacturing",
            Self::MajorHouseholdApplianceManufacturing => "Major Household Appliance Manufacturing ",
            Self::ElectricalEquipmentManufacturing => "Electrical Equipment Manufacturing",
            Self::BatteryManufacturing => "Battery Manufacturing",
            Self::CommunicationAndEnergyWireAndCableManufacturing => "Communication and Energy Wire and Cable Manufacturing",
            Self::WiringDeviceManufacturing => "Wiring Device Manufacturing",
            Self::AllOtherElectricalEquipmentAndComponentManufacturing => "All Other Electrical Equipment and Component Manufacturing",
            Self::AutomobileAndLightDutyMotorVehicleManufacturing => "Automobile and Light Duty Motor Vehicle Manufacturing",
            Self::HeavyDutyTruckManufacturing => "Heavy Duty Truck Manufacturing",
            Self::MotorVehicleBodyAndTrailerManufacturing => "Motor Vehicle Body and Trailer Manufacturing",
            Self::MotorVehicleGasolineEngineAndEnginePartsManufacturing => "Motor Vehicle Gasoline Engine and Engine Parts Manufacturing",
            Self::MotorVehicleElectricalAndElectronicEquipmentManufacturing => "Motor Vehicle Electrical and Electronic Equipment Manufacturing",
            Self::MotorVehicleSteeringAndSuspensionComponentsExceptSpringManufacturing => "Motor Vehicle Steering and Suspension Components (except Spring) Manufacturing",
            Self::MotorVehicleBrakeSystemManufacturing => "Motor Vehicle Brake System Manufacturing",
            Self::MotorVehicleTransmissionAndPowerTrainPartsManufacturing => "Motor Vehicle Transmission and Power Train Parts Manufacturing",
            Self::MotorVehicleSeatingAndInteriorTrimManufacturing => "Motor Vehicle Seating and Interior Trim Manufacturing",
            Self::MotorVehicleMetalStamping => "Motor Vehicle Metal Stamping",
            Self::OtherMotorVehiclePartsManufacturing => "Other Motor Vehicle Parts Manufacturing",
            Self::AerospaceProductAndPartsManufacturing => "Aerospace Product and Parts Manufacturing",
            Self::RailroadRollingStockManufacturing => "Railroad Rolling Stock Manufacturing",
            Self::ShipAndBoatBuilding => "Ship and Boat Building",
            Self::OtherTransportationEquipmentManufacturing => "Other Transportation Equipment Manufacturing",
            Self::WoodKitchenCabinetAndCountertopManufacturing => "Wood Kitchen Cabinet and Countertop Manufacturing",
            Self::HouseholdAndInstitutionalFurnitureManufacturing => "Household and Institutional Furniture Manufacturing",
            Self::OfficeFurnitureIncludingFixturesManufacturing => "Office Furniture (including Fixtures) Manufacturing",
            Self::MattressManufacturing => "Mattress Manufacturing",
            Self::BlindAndShadeManufacturing => "Blind and Shade Manufacturing",
            Self::MedicalEquipmentAndSuppliesManufacturing => "Medical Equipment and Supplies Manufacturing",
            Self::JewelryAndSilverwareManufacturing => "Jewelry and Silverware Manufacturing",
            Self::SportingAndAthleticGoodsManufacturing => "Sporting and Athletic Goods Manufacturing",
            Self::DollToyAndGameManufacturing => "Doll, Toy, and Game Manufacturing",
            Self::OfficeSuppliesExceptPaperManufacturing => "Office Supplies (except Paper) Manufacturing",
            Self::SignManufacturing => "Sign Manufacturing",
            Self::AllOtherMiscellaneousManufacturing => "All Other Miscellaneous Manufacturing",
            Self::AutomobileAndOtherMotorVehicleMerchantWholesalers => "Automobile and Other Motor Vehicle Merchant Wholesalers",
            Self::MotorVehicleSuppliesAndNewPartsMerchantWholesalers => "Motor Vehicle Supplies and New Parts Merchant Wholesalers",
            Self::TireAndTubeMerchantWholesalers => "Tire and Tube Merchant Wholesalers",
            Self::MotorVehiclePartsUsedMerchantWholesalers => "Motor Vehicle Parts (Used) Merchant Wholesalers",
            Self::FurnitureMerchantWholesalers => "Furniture Merchant Wholesalers",
            Self::HomeFurnishingMerchantWholesalers => "Home Furnishing Merchant Wholesalers",
            Self::LumberPlywoodMillworkAndWoodPanelMerchantWholesalers => "Lumber, Plywood, Millwork, and Wood Panel Merchant Wholesalers",
            Self::BrickStoneAndRelatedConstructionMaterialMerchantWholesalers => "Brick, Stone, and Related Construction Material Merchant Wholesalers",
            Self::RoofingSidingAndInsulationMaterialMerchantWholesalers => "Roofing, Siding, and Insulation Material Merchant Wholesalers",
            Self::OtherConstructionMaterialMerchantWholesalers => "Other Construction Material Merchant Wholesalers",
            Self::PhotographicEquipmentAndSuppliesMerchantWholesalers => "Photographic Equipment and Supplies Merchant Wholesalers",
            Self::OfficeEquipmentMerchantWholesalers => "Office Equipment Merchant Wholesalers",
            Self::ComputerAndComputerPeripheralEquipmentAndSoftwareMerchantWholesalers => "Computer and Computer Peripheral Equipment and Software Merchant Wholesalers",
            Self::OtherCommercialEquipmentMerchantWholesalers => "Other Commercial Equipment Merchant Wholesalers",
            Self::MedicalDentalAndHospitalEquipmentAndSuppliesMerchantWholesalers => "Medical, Dental, and Hospital Equipment and Supplies Merchant Wholesalers",
            Self::OphthalmicGoodsMerchantWholesalers => "Ophthalmic Goods Merchant Wholesalers",
            Self::OtherProfessionalEquipmentAndSuppliesMerchantWholesalers => "Other Professional Equipment and Supplies Merchant Wholesalers",
            Self::MetalServiceCentersAndOtherMetalMerchantWholesalers => "Metal Service Centers and Other Metal Merchant Wholesalers",
            Self::CoalAndOtherMineralAndOreMerchantWholesalers => "Coal and Other Mineral and Ore Merchant Wholesalers",
            Self::ElectricalApparatusAndEquipmentWiringSuppliesAndRelatedEquipmentMerchantWholesalers => "Electrical Apparatus and Equipment, Wiring Supplies, and Related Equipment Merchant Wholesalers",
            Self::HouseholdAppliancesElectricHousewaresAndConsumerElectronicsMerchantWholesalers => "Household Appliances, Electric Housewares, and Consumer Electronics Merchant Wholesalers",
            Self::OtherElectronicPartsAndEquipmentMerchantWholesalers => "Other Electronic Parts and Equipment Merchant Wholesalers",
            Self::HardwareMerchantWholesalers => "Hardware Merchant Wholesalers",
            Self::PlumbingAndHeatingEquipmentAndSuppliesHydronicsMerchantWholesalers => "Plumbing and Heating Equipment and Supplies (Hydronics) Merchant Wholesalers",
            Self::WarmAirHeatingAndAirconditioningEquipmentAndSuppliesMerchantWholesalers => "Warm Air Heating and Air-Conditioning Equipment and Supplies Merchant Wholesalers",
            Self::RefrigerationEquipmentAndSuppliesMerchantWholesalers => "Refrigeration Equipment and Supplies Merchant Wholesalers",
            Self::ConstructionAndMiningExceptOilWellMachineryAndEquipmentMerchantWholesalers => "Construction and Mining (except Oil Well) Machinery and Equipment Merchant Wholesalers",
            Self::FarmAndGardenMachineryAndEquipmentMerchantWholesalers => "Farm and Garden Machinery and Equipment Merchant Wholesalers",
            Self::IndustrialMachineryAndEquipmentMerchantWholesalers => "Industrial Machinery and Equipment Merchant Wholesalers",
            Self::IndustrialSuppliesMerchantWholesalers => "Industrial Supplies Merchant Wholesalers",
            Self::ServiceEstablishmentEquipmentAndSuppliesMerchantWholesalers => "Service Establishment Equipment and Supplies Merchant Wholesalers",
            Self::TransportationEquipmentAndSuppliesExceptMotorVehicleMerchantWholesalers => "Transportation Equipment and Supplies (except Motor Vehicle) Merchant Wholesalers",
            Self::SportingAndRecreationalGoodsAndSuppliesMerchantWholesalers => "Sporting and Recreational Goods and Supplies Merchant Wholesalers",
            Self::ToyAndHobbyGoodsAndSuppliesMerchantWholesalers => "Toy and Hobby Goods and Supplies Merchant Wholesalers",
            Self::RecyclableMaterialMerchantWholesalers => "Recyclable Material Merchant Wholesalers",
            Self::JewelryWatchPreciousStoneAndPreciousMetalMerchantWholesalers => "Jewelry, Watch, Precious Stone, and Precious Metal Merchant Wholesalers",
            Self::OtherMiscellaneousDurableGoodsMerchantWholesalers => "Other Miscellaneous Durable Goods Merchant Wholesalers",
            Self::PrintingAndWritingPaperMerchantWholesalers => "Printing and Writing Paper Merchant Wholesalers",
            Self::StationeryAndOfficeSuppliesMerchantWholesalers => "Stationery and Office Supplies Merchant Wholesalers",
            Self::IndustrialAndPersonalServicePaperMerchantWholesalers => "Industrial and Personal Service Paper Merchant Wholesalers",
            Self::DrugsAndDruggistsSundriesMerchantWholesalers => "Drugs and Druggists' Sundries Merchant Wholesalers",
            Self::PieceGoodsNotionsAndOtherDryGoodsMerchantWholesalers => "Piece Goods, Notions, and Other Dry Goods Merchant Wholesalers",
            Self::FootwearMerchantWholesalers => "Footwear Merchant Wholesalers",
            Self::ClothingAndClothingAccessoriesMerchantWholesalers => "Clothing and Clothing Accessories Merchant Wholesalers",
            Self::GeneralLineGroceryMerchantWholesalers => "General Line Grocery Merchant Wholesalers",
            Self::PackagedFrozenFoodMerchantWholesalers => "Packaged Frozen Food Merchant Wholesalers",
            Self::DairyProductExceptDriedOrCannedMerchantWholesalers => "Dairy Product (except Dried or Canned) Merchant Wholesalers",
            Self::PoultryAndPoultryProductMerchantWholesalers => "Poultry and Poultry Product Merchant Wholesalers",
            Self::ConfectioneryMerchantWholesalers => "Confectionery Merchant Wholesalers",
            Self::FishAndSeafoodMerchantWholesalers => "Fish and Seafood Merchant Wholesalers",
            Self::MeatAndMeatProductMerchantWholesalers => "Meat and Meat Product Merchant Wholesalers",
            Self::FreshFruitAndVegetableMerchantWholesalers => "Fresh Fruit and Vegetable Merchant Wholesalers",
            Self::OtherGroceryAndRelatedProductsMerchantWholesalers => "Other Grocery and Related Products Merchant Wholesalers",
            Self::GrainAndFieldBeanMerchantWholesalers => "Grain and Field Bean Merchant Wholesalers",
            Self::LivestockMerchantWholesalers => "Livestock Merchant Wholesalers",
            Self::OtherFarmProductRawMaterialMerchantWholesalers => "Other Farm Product Raw Material Merchant Wholesalers",
            Self::PlasticsMaterialsAndBasicFormsAndShapesMerchantWholesalers => "Plastics Materials and Basic Forms and Shapes Merchant Wholesalers",
            Self::OtherChemicalAndAlliedProductsMerchantWholesalers => "Other Chemical and Allied Products Merchant Wholesalers",
            Self::PetroleumBulkStationsAndTerminals => "Petroleum Bulk Stations and Terminals",
            Self::PetroleumAndPetroleumProductsMerchantWholesalersExceptBulkStationsAndTerminals => "Petroleum and Petroleum Products Merchant Wholesalers (except Bulk Stations and Terminals)",
            Self::BeerAndAleMerchantWholesalers => "Beer and Ale Merchant Wholesalers",
            Self::WineAndDistilledAlcoholicBeverageMerchantWholesalers => "Wine and Distilled Alcoholic Beverage Merchant Wholesalers",
            Self::FarmSuppliesMerchantWholesalers => "Farm Supplies Merchant Wholesalers",
            Self::BookPeriodicalAndNewspaperMerchantWholesalers => "Book, Periodical, and Newspaper Merchant Wholesalers",
            Self::FlowerNurseryStockAndFloristsSuppliesMerchantWholesalers => "Flower, Nursery Stock, and Florists' Supplies Merchant Wholesalers",
            Self::TobaccoProductAndElectronicCigaretteMerchantWholesalers => "Tobacco Product and Electronic Cigarette Merchant Wholesalers",
            Self::PaintVarnishAndSuppliesMerchantWholesalers => "Paint, Varnish, and Supplies Merchant Wholesalers",
            Self::OtherMiscellaneousNondurableGoodsMerchantWholesalers => "Other Miscellaneous Nondurable Goods Merchant Wholesalers",
            Self::WholesaleTradeAgentsAndBrokers => "Wholesale Trade Agents and Brokers",
            Self::NewCarDealers => "New Car Dealers",
            Self::UsedCarDealers => "Used Car Dealers",
            Self::RecreationalVehicleDealers => "Recreational Vehicle Dealers",
            Self::MotorcycleBoatAndOtherMotorVehicleDealers => "Motorcycle, Boat, and Other Motor Vehicle Dealers",
            Self::AutomotivePartsAndAccessoriesRetailers => "Automotive Parts and Accessories Retailers",
            Self::TireDealers => "Tire Dealers",
            Self::HomeCenters => "Home Centers",
            Self::PaintAndWallpaperRetailers => "Paint and Wallpaper Retailers",
            Self::HardwareRetailers => "Hardware Retailers",
            Self::OtherBuildingMaterialDealers => "Other Building Material Dealers",
            Self::OutdoorPowerEquipmentRetailers => "Outdoor Power Equipment Retailers",
            Self::NurseryGardenCenterAndFarmSupplyRetailers => "Nursery, Garden Center, and Farm Supply Retailers",
            Self::SupermarketsAndOtherGroceryRetailersExceptConvenienceRetailers => "Supermarkets and Other Grocery Retailers (except Convenience Retailers)",
            Self::ConvenienceRetailersAndVendingMachineOperators => "Convenience Retailers and Vending Machine Operators",
            Self::FruitAndVegetableRetailers => "Fruit and Vegetable Retailers",
            Self::MeatRetailers => "Meat Retailers",
            Self::FishAndSeafoodRetailers => "Fish and Seafood Retailers",
            Self::OtherSpecialtyFoodRetailers => "Other Specialty Food Retailers",
            Self::BeerWineAndLiquorRetailers => "Beer, Wine, and Liquor Retailers",
            Self::FurnitureRetailers => "Furniture Retailers",
            Self::HomeFurnishingsRetailers => "Home Furnishings Retailers",
            Self::ElectronicsAndApplianceRetailers => "Electronics and Appliance Retailers",
            Self::DepartmentStores => "Department Stores",
            Self::WarehouseClubsAndSupercenters => "Warehouse Clubs and Supercenters",
            Self::AllOtherGeneralMerchandiseRetailers => "All Other General Merchandise Retailers",
            Self::PharmaciesAndDrugRetailers => "Pharmacies and Drug Retailers",
            Self::CosmeticsBeautySuppliesAndPerfumeRetailers => "Cosmetics, Beauty Supplies, and Perfume Retailers",
            Self::OpticalGoodsRetailers => "Optical Goods Retailers",
            Self::OtherHealthAndPersonalCareRetailers => "Other Health and Personal Care Retailers",
            Self::GasolineStationsWithConvenienceStores => "Gasoline Stations with Convenience Stores",
            Self::OtherGasolineStations => "Other Gasoline Stations",
            Self::FuelDealers => "Fuel Dealers",
            Self::ClothingRetailers => "Clothing Retailers",
            Self::ClothingAccessoriesRetailers => "Clothing Accessories Retailers",
            Self::ShoeRetailers => "Shoe Retailers",
            Self::JewelryRetailers => "Jewelry Retailers",
            Self::LuggageAndLeatherGoodsRetailers => "Luggage and Leather Goods Retailers",
            Self::SportingGoodsRetailers => "Sporting Goods Retailers",
            Self::HobbyToyAndGameRetailers => "Hobby, Toy, and Game Retailers",
            Self::SewingNeedleworkAndPieceGoodsRetailers => "Sewing, Needlework, and Piece Goods Retailers",
            Self::MusicalInstrumentAndSuppliesRetailers => "Musical Instrument and Supplies Retailers",
            Self::BookRetailersAndNewsDealers => "Book Retailers and News Dealers",
            Self::Florists => "Florists",
            Self::OfficeSuppliesAndStationeryRetailers => "Office Supplies and Stationery Retailers",
            Self::GiftNoveltyAndSouvenirRetailers => "Gift, Novelty, and Souvenir Retailers",
            Self::UsedMerchandiseRetailers => "Used Merchandise Retailers",
            Self::PetAndPetSuppliesRetailers => "Pet and Pet Supplies Retailers",
            Self::ArtDealers => "Art Dealers",
            Self::ManufacturedMobileHomeDealers => "Manufactured (Mobile) Home Dealers",
            Self::AllOtherMiscellaneousRetailers => "All Other Miscellaneous Retailers",
            Self::ScheduledAirTransportation => "Scheduled Air Transportation",
            Self::NonscheduledAirTransportation => "Nonscheduled Air Transportation",
            Self::RailTransportation => "Rail Transportation",
            Self::DeepSeaCoastalAndGreatLakesWaterTransportation => "Deep Sea, Coastal, and Great Lakes Water Transportation",
            Self::InlandWaterTransportation => "Inland Water Transportation",
            Self::GeneralFreightTruckingLocal => "General Freight Trucking, Local",
            Self::GeneralFreightTruckingLongdistance => "General Freight Trucking, Long-Distance",
            Self::UsedHouseholdAndOfficeGoodsMoving => "Used Household and Office Goods Moving",
            Self::SpecializedFreightExceptUsedGoodsTruckingLocal => "Specialized Freight (except Used Goods) Trucking, Local",
            Self::SpecializedFreightExceptUsedGoodsTruckingLongdistance => "Specialized Freight (except Used Goods) Trucking, Long-Distance",
            Self::UrbanTransitSystems => "Urban Transit Systems",
            Self::InterurbanAndRuralBusTransportation => "Interurban and Rural Bus Transportation",
            Self::TaxiService => "Taxi Service",
            Self::LimousineService => "Limousine Service",
            Self::SchoolAndEmployeeBusTransportation => "School and Employee Bus Transportation",
            Self::CharterBusIndustry => "Charter Bus Industry",
            Self::OtherTransitAndGroundPassengerTransportation => "Other Transit and Ground Passenger Transportation",
            Self::PipelineTransportationOfCrudeOil => "Pipeline Transportation of Crude Oil",
            Self::PipelineTransportationOfNaturalGas => "Pipeline Transportation of Natural Gas",
            Self::PipelineTransportationOfRefinedPetroleumProducts => "Pipeline Transportation of Refined Petroleum Products",
            Self::AllOtherPipelineTransportation => "All Other Pipeline Transportation",
            Self::ScenicAndSightseeingTransportationLand => "Scenic and Sightseeing Transportation, Land",
            Self::ScenicAndSightseeingTransportationWater => "Scenic and Sightseeing Transportation, Water",
            Self::ScenicAndSightseeingTransportationOther => "Scenic and Sightseeing Transportation, Other",
            Self::AirportOperations => "Airport Operations",
            Self::OtherSupportActivitiesForAirTransportation => "Other Support Activities for Air Transportation",
            Self::SupportActivitiesForRailTransportation => "Support Activities for Rail Transportation",
            Self::PortAndHarborOperations => "Port and Harbor Operations",
            Self::MarineCargoHandling => "Marine Cargo Handling",
            Self::NavigationalServicesToShipping => "Navigational Services to Shipping",
            Self::OtherSupportActivitiesForWaterTransportation => "Other Support Activities for Water Transportation",
            Self::MotorVehicleTowing => "Motor Vehicle Towing",
            Self::OtherSupportActivitiesForRoadTransportation => "Other Support Activities for Road Transportation",
            Self::FreightTransportationArrangement => "Freight Transportation Arrangement",
            Self::OtherSupportActivitiesForTransportation => "Other Support Activities for Transportation",
            Self::PostalService => "Postal Service",
            Self::CouriersAndExpressDeliveryServices => "Couriers and Express Delivery Services",
            Self::LocalMessengersAndLocalDelivery => "Local Messengers and Local Delivery",
            Self::GeneralWarehousingAndStorage => "General Warehousing and Storage",
            Self::RefrigeratedWarehousingAndStorage => "Refrigerated Warehousing and Storage",
            Self::FarmProductWarehousingAndStorage => "Farm Product Warehousing and Storage",
            Self::OtherWarehousingAndStorage => "Other Warehousing and Storage",
            Self::NewspaperPublishers => "Newspaper Publishers",
            Self::PeriodicalPublishers => "Periodical Publishers",
            Self::BookPublishers => "Book Publishers",
            Self::DirectoryAndMailingListPublishers => "Directory and Mailing List Publishers",
            Self::OtherPublishers => "Other Publishers",
            Self::SoftwarePublishers => "Software Publishers",
            Self::MotionPictureAndVideoProduction => "Motion Picture and Video Production",
            Self::MotionPictureAndVideoDistribution => "Motion Picture and Video Distribution",
            Self::MotionPictureAndVideoExhibition => "Motion Picture and Video Exhibition",
            Self::PostproductionServicesAndOtherMotionPictureAndVideoIndustries => "Postproduction Services and Other Motion Picture and Video Industries",
            Self::MusicPublishers => "Music Publishers",
            Self::SoundRecordingStudios => "Sound Recording Studios",
            Self::RecordProductionAndDistribution => "Record Production and Distribution",
            Self::OtherSoundRecordingIndustries => "Other Sound Recording Industries",
            Self::RadioBroadcasting => "Radio Broadcasting",
            Self::TelevisionBroadcasting => "Television Broadcasting",
            Self::CableAndOtherSubscriptionProgramming => "Cable and Other Subscription Programming",
            Self::WiredAndWirelessTelecommunicationsCarriers => "Wired and Wireless Telecommunications Carriers",
            Self::SatelliteTelecommunications => "Satellite Telecommunications",
            Self::OtherTelecommunications => "Other Telecommunications",
            Self::ComputingInfrastructureProvidersDataProcessingWebHostingAndRelatedServices => "Computing Infrastructure Providers, Data Processing, Web Hosting, and Related Services",
            Self::NewsSyndicates => "News Syndicates",
            Self::LibrariesAndArchives => "Libraries and Archives",
            Self::InternetPublishingAndBroadcastingAndWebSearchPortals => "Internet Publishing and Broadcasting and Web Search Portals",
            Self::AllOtherInformationServices => "All Other Information Services",
            Self::MonetaryAuthoritiesCentralBank => "Monetary Authorities-Central Bank",
            Self::CommercialBanking => "Commercial Banking",
            Self::SavingsInstitutions => "Savings Institutions",
            Self::CreditUnions => "Credit Unions",
            Self::OtherDepositoryCreditIntermediation => "Other Depository Credit Intermediation",
            Self::CreditCardIssuing => "Credit Card Issuing",
            Self::SalesFinancing => "Sales Financing",
            Self::OtherNondepositoryCreditIntermediation => "Other Nondepository Credit Intermediation",
            Self::MortgageAndNonmortgageLoanBrokers => "Mortgage and Nonmortgage Loan Brokers",
            Self::FinancialTransactionsProcessingReserveAndClearinghouseActivities => "Financial Transactions Processing, Reserve, and Clearinghouse Activities",
            Self::OtherActivitiesRelatedToCreditIntermediation => "Other Activities Related to Credit Intermediation",
            Self::InvestmentBankingAndSecuritiesDealing => "Investment Banking and Securities Dealing",
            Self::SecuritiesBrokerage => "Securities Brokerage",
            Self::CommodityContractsDealing => "Commodity Contracts Dealing",
            Self::CommodityContractsBrokerage => "Commodity Contracts Brokerage",
            Self::SecuritiesAndCommodityExchanges => "Securities and Commodity Exchanges",
            Self::MiscellaneousIntermediation => "Miscellaneous Intermediation",
            Self::PortfolioManagement => "Portfolio Management",
            Self::InvestmentAdvice => "Investment Advice",
            Self::AllOtherFinancialInvestmentActivities => "All Other Financial Investment Activities",
            Self::DirectLifeHealthAndMedicalInsuranceCarriers => "Direct Life, Health, and Medical Insurance Carriers",
            Self::DirectInsuranceExceptLifeHealthAndMedicalCarriers => "Direct Insurance (except Life, Health, and Medical) Carriers",
            Self::ReinsuranceCarriers => "Reinsurance Carriers",
            Self::InsuranceAgenciesAndBrokerages => "Insurance Agencies and Brokerages",
            Self::OtherInsuranceRelatedActivities => "Other Insurance Related Activities",
            Self::PensionFunds => "Pension Funds",
            Self::HealthAndWelfareFunds => "Health and Welfare Funds",
            Self::OtherInsuranceFunds => "Other Insurance Funds",
            Self::OpenEndInvestmentFunds => "Open-End Investment Funds",
            Self::TrustsEstatesAndAgencyAccounts => "Trusts, Estates, and Agency Accounts",
            Self::RealEstateInvestmentTrusts => "Real Estate Investment Trusts",
            Self::OtherFinancialVehicles => "Other Financial Vehicles",
            Self::LessorsOfResidentialBuildingsAndDwellings => "Lessors of Residential Buildings and Dwellings",
            Self::LessorsOfNonresidentialBuildingsExceptMiniwarehouses => "Lessors of Nonresidential Buildings (except Miniwarehouses)",
            Self::LessorsOfMiniwarehousesAndSelfStorageUnits => "Lessors of Miniwarehouses and Self-Storage Units",
            Self::LessorsOfOtherRealEstateProperty => "Lessors of Other Real Estate Property",
            Self::OfficesOfRealEstateAgentsAndBrokers => "Offices of Real Estate Agents and Brokers",
            Self::RealEstatePropertyManagers => "Real Estate Property Managers",
            Self::OfficesOfRealEstateAppraisers => "Offices of Real Estate Appraisers",
            Self::OtherActivitiesRelatedToRealEstate => "Other Activities Related to Real Estate",
            Self::PassengerCarRentalAndLeasing => "Passenger Car Rental and Leasing",
            Self::TruckUtilityTrailerAndRvRecreationalVehicleRentalAndLeasing => "Truck, Utility Trailer, and RV (Recreational Vehicle) Rental and Leasing",
            Self::ConsumerElectronicsAndAppliancesRental => "Consumer Electronics and Appliances Rental",
            Self::FormalWearAndCostumeRental => "Formal Wear and Costume Rental",
            Self::VideoTapeAndDiscRental => "Video Tape and Disc Rental",
            Self::OtherConsumerGoodsRental => "Other Consumer Goods Rental",
            Self::GeneralRentalCenters => "General Rental Centers",
            Self::ConstructionTransportationMiningAndForestryMachineryAndEquipmentRentalAndLeasing => "Construction, Transportation, Mining, and Forestry Machinery and Equipment Rental and Leasing",
            Self::OfficeMachineryAndEquipmentRentalAndLeasing => "Office Machinery and Equipment Rental and Leasing",
            Self::OtherCommercialAndIndustrialMachineryAndEquipmentRentalAndLeasing => "Other Commercial and Industrial Machinery and Equipment Rental and Leasing",
            Self::LessorsOfNonfinancialIntangibleAssetsExceptCopyrightedWorks => "Lessors of Nonfinancial Intangible Assets (except Copyrighted Works)",
            Self::OfficesOfLawyers => "Offices of Lawyers",
            Self::OfficesOfNotaries => "Offices of Notaries",
            Self::OtherLegalServices => "Other Legal Services",
            Self::AccountingTaxPreparationBookkeepingAndPayrollServices => "Accounting, Tax Preparation, Bookkeeping, and Payroll Services",
            Self::ArchitecturalServices => "Architectural Services",
            Self::LandscapeArchitecturalServices => "Landscape Architectural Services",
            Self::EngineeringServices => "Engineering Services",
            Self::DraftingServices => "Drafting Services",
            Self::BuildingInspectionServices => "Building Inspection Services",
            Self::GeophysicalSurveyingAndMappingServices => "Geophysical Surveying and Mapping Services",
            Self::SurveyingAndMappingExceptGeophysicalServices => "Surveying and Mapping (except Geophysical) Services",
            Self::TestingLaboratories => "Testing Laboratories",
            Self::InteriorDesignServices => "Interior Design Services",
            Self::IndustrialDesignServices => "Industrial Design Services",
            Self::GraphicDesignServices => "Graphic Design Services",
            Self::OtherSpecializedDesignServices => "Other Specialized Design Services",
            Self::ComputerSystemsDesignAndRelatedServices => "Computer Systems Design and Related Services",
            Self::ManagementConsultingServices => "Management Consulting Services",
            Self::EnvironmentalConsultingServices => "Environmental Consulting Services",
            Self::OtherScientificAndTechnicalConsultingServices => "Other Scientific and Technical Consulting Services",
            Self::ResearchAndDevelopmentInThePhysicalEngineeringAndLifeSciences => "Research and Development in the Physical, Engineering, and Life Sciences",
            Self::ResearchAndDevelopmentInTheSocialSciencesAndHumanities => "Research and Development in the Social Sciences and Humanities",
            Self::AdvertisingAgencies => "Advertising Agencies",
            Self::PublicRelationsAgencies => "Public Relations Agencies",
            Self::MediaBuyingAgencies => "Media Buying Agencies",
            Self::MediaRepresentatives => "Media Representatives",
            Self::OutdoorAdvertising => "Outdoor Advertising",
            Self::DirectMailAdvertising => "Direct Mail Advertising",
            Self::AdvertisingMaterialDistributionServices => "Advertising Material Distribution Services",
            Self::OtherServicesRelatedToAdvertising => "Other Services Related to Advertising",
            Self::MarketingResearchAndPublicOpinionPolling => "Marketing Research and Public Opinion Polling",
            Self::PhotographicServices => "Photographic Services",
            Self::TranslationAndInterpretationServices => "Translation and Interpretation Services",
            Self::VeterinaryServices => "Veterinary Services",
            Self::AllOtherProfessionalScientificAndTechnicalServices => "All Other Professional, Scientific, and Technical Services",
            Self::ManagementOfCompaniesAndEnterprises => "Management of Companies and Enterprises",
            Self::OfficeAdministrativeServices => "Office Administrative Services",
            Self::FacilitiesSupportServices => "Facilities Support Services",
            Self::EmploymentPlacementAgenciesAndExecutiveSearchServices => "Employment Placement Agencies and Executive Search Services",
            Self::TemporaryHelpServices => "Temporary Help Services",
            Self::ProfessionalEmployerOrganizations => "Professional Employer Organizations",
            Self::DocumentPreparationServices => "Document Preparation Services",
            Self::TelephoneCallCenters => "Telephone Call Centers",
            Self::BusinessServiceCenters => "Business Service Centers",
            Self::CollectionAgencies => "Collection Agencies",
            Self::CreditBureaus => "Credit Bureaus",
            Self::OtherBusinessSupportServices => "Other Business Support Services",
            Self::TravelAgencies => "Travel Agencies",
            Self::TourOperators => "Tour Operators",
            Self::OtherTravelArrangementAndReservationServices => "Other Travel Arrangement and Reservation Services",
            Self::InvestigationGuardAndArmoredCarServices => "Investigation, Guard, and Armored Car Services",
            Self::SecuritySystemsServices => "Security Systems Services",
            Self::ExterminatingAndPestControlServices => "Exterminating and Pest Control Services",
            Self::JanitorialServices => "Janitorial Services",
            Self::LandscapingServices => "Landscaping Services",
            Self::CarpetAndUpholsteryCleaningServices => "Carpet and Upholstery Cleaning Services",
            Self::OtherServicesToBuildingsAndDwellings => "Other Services to Buildings and Dwellings",
            Self::PackagingAndLabelingServices => "Packaging and Labeling Services",
            Self::ConventionAndTradeShowOrganizers => "Convention and Trade Show Organizers",
            Self::AllOtherSupportServices => "All Other Support Services",
            Self::WasteCollection => "Waste Collection",
            Self::WasteTreatmentAndDisposal => "Waste Treatment and Disposal",
            Self::RemediationServices => "Remediation Services",
            Self::MaterialsRecoveryFacilities => "Materials Recovery Facilities",
            Self::AllOtherWasteManagementServices => "All Other Waste Management Services",
            Self::ElementaryAndSecondarySchools => "Elementary and Secondary Schools",
            Self::JuniorColleges => "Junior Colleges",
            Self::CollegesUniversitiesAndProfessionalSchools => "Colleges, Universities, and Professional Schools",
            Self::BusinessAndSecretarialSchools => "Business and Secretarial Schools",
            Self::ComputerTraining => "Computer Training",
            Self::ProfessionalAndManagementDevelopmentTraining => "Professional and Management Development Training",
            Self::TechnicalAndTradeSchools => "Technical and Trade Schools",
            Self::FineArtsSchools => "Fine Arts Schools",
            Self::SportsAndRecreationInstruction => "Sports and Recreation Instruction",
            Self::LanguageSchools => "Language Schools",
            Self::AllOtherSchoolsAndInstruction => "All Other Schools and Instruction",
            Self::EducationalSupportServices => "Educational Support Services",
            Self::OfficesOfPhysicians => "Offices of Physicians",
            Self::OfficesOfDentists => "Offices of Dentists",
            Self::OfficesOfChiropractors => "Offices of Chiropractors",
            Self::OfficesOfOptometrists => "Offices of Optometrists",
            Self::OfficesOfMentalHealthPractitionersExceptPhysicians => "Offices of Mental Health Practitioners (except Physicians)",
            Self::OfficesOfPhysicalOccupationalAndSpeechTherapistsAndAudiologists => "Offices of Physical, Occupational and Speech Therapists, and Audiologists",
            Self::OfficesOfAllOtherHealthPractitioners => "Offices of All Other Health Practitioners",
            Self::FamilyPlanningCenters => "Family Planning Centers",
            Self::OutpatientMentalHealthAndSubstanceAbuseCenters => "Outpatient Mental Health and Substance Abuse Centers",
            Self::OtherOutpatientCareCenters => "Other Outpatient Care Centers",
            Self::MedicalAndDiagnosticLaboratories => "Medical and Diagnostic Laboratories",
            Self::HomeHealthCareServices => "Home Health Care Services",
            Self::AmbulanceServices => "Ambulance Services",
            Self::AllOtherAmbulatoryHealthCareServices => "All Other Ambulatory Health Care Services",
            Self::GeneralMedicalAndSurgicalHospitals => "General Medical and Surgical Hospitals",
            Self::PsychiatricAndSubstanceAbuseHospitals => "Psychiatric and Substance Abuse Hospitals",
            Self::SpecialtyExceptPsychiatricAndSubstanceAbuseHospitals => "Specialty (except Psychiatric and Substance Abuse) Hospitals",
            Self::NursingCareFacilitiesSkilledNursingFacilities => "Nursing Care Facilities (Skilled Nursing Facilities)",
            Self::ResidentialIntellectualAndDevelopmentalDisabilityFacilities => "Residential Intellectual and Developmental Disability Facilities",
            Self::ResidentialMentalHealthAndSubstanceAbuseFacilities => "Residential Mental Health and Substance Abuse Facilities",
            Self::ContinuingCareRetirementCommunitiesAndAssistedLivingFacilitiesForTheElderly => "Continuing Care Retirement Communities and Assisted Living Facilities for the Elderly",
            Self::OtherResidentialCareFacilities => "Other Residential Care Facilities",
            Self::ChildAndYouthServices => "Child and Youth Services",
            Self::ServicesForTheElderlyAndPersonsWithDisabilities => "Services for the Elderly and Persons with Disabilities",
            Self::OtherIndividualAndFamilyServices => "Other Individual and Family Services",
            Self::CommunityFoodServices => "Community Food Services",
            Self::CommunityHousingServices => "Community Housing Services",
            Self::EmergencyAndOtherReliefServices => "Emergency and Other Relief Services",
            Self::VocationalRehabilitationServices => "Vocational Rehabilitation Services",
            Self::ChildDayCareServices => "Child Day Care Services",
            Self::TheaterCompaniesAndDinnerTheaters => "Theater Companies and Dinner Theaters",
            Self::DanceCompanies => "Dance Companies",
            Self::MusicalGroupsAndArtists => "Musical Groups and Artists",
            Self::OtherPerformingArtsCompanies => "Other Performing Arts Companies",
            Self::SpectatorSports => "Spectator Sports",
            Self::PromotersOfPerformingArtsSportsAndSimilarEventsWithFacilities => "Promoters of Performing Arts, Sports, and Similar Events with Facilities",
            Self::PromotersOfPerformingArtsSportsAndSimilarEventsWithoutFacilities => "Promoters of Performing Arts, Sports, and Similar Events without Facilities",
            Self::AgentsAndManagersForArtistsAthletesEntertainersAndOtherPublicFigures => "Agents and Managers for Artists, Athletes, Entertainers, and Other Public Figures",
            Self::IndependentArtistsWritersAndPerformers => "Independent Artists, Writers, and Performers",
            Self::Museums => "Museums",
            Self::HistoricalSites => "Historical Sites",
            Self::ZoosAndBotanicalGardens => "Zoos and Botanical Gardens",
            Self::NatureParksAndOtherSimilarInstitutions => "Nature Parks and Other Similar Institutions",
            Self::AmusementAndThemeParks => "Amusement and Theme Parks",
            Self::AmusementArcades => "Amusement Arcades",
            Self::CasinosExceptCasinoHotels => "Casinos (except Casino Hotels)",
            Self::OtherGamblingIndustries => "Other Gambling Industries",
            Self::GolfCoursesAndCountryClubs => "Golf Courses and Country Clubs",
            Self::SkiingFacilities => "Skiing Facilities",
            Self::Marinas => "Marinas",
            Self::FitnessAndRecreationalSportsCenters => "Fitness and Recreational Sports Centers",
            Self::BowlingCenters => "Bowling Centers",
            Self::AllOtherAmusementAndRecreationIndustries => "All Other Amusement and Recreation Industries",
            Self::HotelsExceptCasinoHotelsAndMotels => "Hotels (except Casino Hotels) and Motels",
            Self::CasinoHotels => "Casino Hotels",
            Self::OtherTravelerAccommodation => "Other Traveler Accommodation",
            Self::RvRecreationalVehicleParksAndRecreationalCamps => "RV (Recreational Vehicle) Parks and Recreational Camps",
            Self::RoomingAndBoardingHouses => "Rooming and Boarding Houses",
            Self::FoodServiceContractors => "Food Service Contractors",
            Self::Caterers => "Caterers",
            Self::MobileFoodServices => "Mobile Food Services",
            Self::DrinkingPlacesAlcoholicBeverages => "Drinking Places (Alcoholic Beverages)",
            Self::RestaurantsAndOtherEatingPlaces => "Restaurants and Other Eating Places",
            Self::AutomotiveMechanicalAndElectricalRepairAndMaintenance => "Automotive Mechanical and Electrical Repair and Maintenance",
            Self::AutomotiveBodyPaintInteriorAndGlassRepair => "Automotive Body, Paint, Interior, and Glass Repair",
            Self::OtherAutomotiveRepairAndMaintenance => "Other Automotive Repair and Maintenance",
            Self::ElectronicAndPrecisionEquipmentRepairAndMaintenance => "Electronic and Precision Equipment Repair and Maintenance",
            Self::CommercialAndIndustrialMachineryAndEquipmentExceptAutomotiveAndElectronicRepairAndMaintenance => "Commercial and Industrial Machinery and Equipment (except Automotive and Electronic) Repair and Maintenance",
            Self::HomeAndGardenEquipmentAndApplianceRepairAndMaintenance => "Home and Garden Equipment and Appliance Repair and Maintenance",
            Self::ReupholsteryAndFurnitureRepair => "Reupholstery and Furniture Repair",
            Self::FootwearAndLeatherGoodsRepair => "Footwear and Leather Goods Repair",
            Self::OtherPersonalAndHouseholdGoodsRepairAndMaintenance => "Other Personal and Household Goods Repair and Maintenance",
            Self::HairNailAndSkinCareServices => "Hair, Nail, and Skin Care Services",
            Self::OtherPersonalCareServices => "Other Personal Care Services",
            Self::FuneralHomesAndFuneralServices => "Funeral Homes and Funeral Services",
            Self::CemeteriesAndCrematories => "Cemeteries and Crematories",
            Self::CoinOperatedLaundriesAndDrycleaners => "Coin-Operated Laundries and Drycleaners",
            Self::DrycleaningAndLaundryServicesExceptCoinOperated => "Drycleaning and Laundry Services (except Coin-Operated)",
            Self::LinenAndUniformSupply => "Linen and Uniform Supply",
            Self::PetCareExceptVeterinaryServices => "Pet Care (except Veterinary) Services",
            Self::Photofinishing => "Photofinishing",
            Self::ParkingLotsAndGarages => "Parking Lots and Garages",
            Self::AllOtherPersonalServices => "All Other Personal Services",
            Self::ReligiousOrganizations => "Religious Organizations",
            Self::GrantmakingAndGivingServices => "Grantmaking and Giving Services",
            Self::SocialAdvocacyOrganizations => "Social Advocacy Organizations",
            Self::CivicAndSocialOrganizations => "Civic and Social Organizations",
            Self::BusinessAssociations => "Business Associations",
            Self::ProfessionalOrganizations => "Professional Organizations",
            Self::LaborUnionsAndSimilarLaborOrganizations => "Labor Unions and Similar Labor Organizations",
            Self::PoliticalOrganizations => "Political Organizations",
            Self::OtherSimilarOrganizationsExceptBusinessProfessionalLaborAndPoliticalOrganizations => "Other Similar Organizations (except Business, Professional, Labor, and Political Organizations)",
            Self::PrivateHouseholds => "Private Households",
            Self::ExecutiveOffices => "Executive Offices",
            Self::LegislativeBodies => "Legislative Bodies",
            Self::PublicFinanceActivities => "Public Finance Activities",
            Self::ExecutiveAndLegislativeOfficesCombined => "Executive and Legislative Offices, Combined",
            Self::AmericanIndianAndAlaskaNativeTribalGovernments => "American Indian and Alaska Native Tribal Governments",
            Self::OtherGeneralGovernmentSupport => "Other General Government Support",
            Self::Courts => "Courts",
            Self::PoliceProtection => "Police Protection",
            Self::LegalCounselAndProsecution => "Legal Counsel and Prosecution",
            Self::CorrectionalInstitutions => "Correctional Institutions",
            Self::ParoleOfficesAndProbationOffices => "Parole Offices and Probation Offices",
            Self::FireProtection => "Fire Protection",
            Self::OtherJusticePublicOrderAndSafetyActivities => "Other Justice, Public Order, and Safety Activities",
            Self::AdministrationOfEducationPrograms => "Administration of Education Programs",
            Self::AdministrationOfPublicHealthPrograms => "Administration of Public Health Programs",
            Self::AdministrationOfHumanResourceProgramsExceptEducationPublicHealthAndVeteransAffairsPrograms => "Administration of Human Resource Programs (except Education, Public Health, and Veterans' Affairs Programs)",
            Self::AdministrationOfVeteransAffairs => "Administration of Veterans' Affairs",
            Self::AdministrationOfAirAndWaterResourceAndSolidWasteManagementPrograms => "Administration of Air and Water Resource and Solid Waste Management Programs",
            Self::AdministrationOfConservationPrograms => "Administration of Conservation Programs",
            Self::AdministrationOfHousingPrograms => "Administration of Housing Programs",
            Self::AdministrationOfUrbanPlanningAndCommunityAndRuralDevelopment => "Administration of Urban Planning and Community and Rural Development",
            Self::AdministrationOfGeneralEconomicPrograms => "Administration of General Economic Programs",
            Self::RegulationAndAdministrationOfTransportationPrograms => "Regulation and Administration of Transportation Programs",
            Self::RegulationAndAdministrationOfCommunicationsElectricGasAndOtherUtilities => "Regulation and Administration of Communications, Electric, Gas, and Other Utilities",
            Self::RegulationOfAgriculturalMarketingAndCommodities => "Regulation of Agricultural Marketing and Commodities",
            Self::RegulationLicensingAndInspectionOfMiscellaneousCommercialSectors => "Regulation, Licensing, and Inspection of Miscellaneous Commercial Sectors",
            Self::SpaceResearchAndTechnology => "Space Research and Technology",
            Self::NationalSecurity => "National Security",
            Self::InternationalAffairs => "International Affairs",
            Self::UnclassifiedEstablishments => "Unclassified Establishments",
        }
    }
    /// Returns the NAICS code for this subcategory as an i64
    pub fn code(&self) -> i64 {
        match self {
            Self::SoybeanFarming => 11111,
            Self::OilseedExceptSoybeanFarming => 11112,
            Self::DryPeaAndBeanFarming => 11113,
            Self::WheatFarming => 11114,
            Self::CornFarming => 11115,
            Self::RiceFarming => 11116,
            Self::OtherGrainFarming => 11119,
            Self::VegetableAndMelonFarming => 11121,
            Self::OrangeGroves => 11131,
            Self::CitrusExceptOrangeGroves => 11132,
            Self::NoncitrusFruitAndTreeNutFarming => 11133,
            Self::FoodCropsGrownUnderCover => 11141,
            Self::NurseryAndFloricultureProduction => 11142,
            Self::TobaccoFarming => 11191,
            Self::CottonFarming => 11192,
            Self::SugarcaneFarming => 11193,
            Self::HayFarming => 11194,
            Self::AllOtherCropFarming => 11199,
            Self::BeefCattleRanchingAndFarmingIncludingFeedlots => 11211,
            Self::DairyCattleAndMilkProduction => 11212,
            Self::DualpurposeCattleRanchingAndFarming => 11213,
            Self::HogAndPigFarming => 11221,
            Self::ChickenEggProduction => 11231,
            Self::BroilersAndOtherMeatTypeChickenProduction => 11232,
            Self::TurkeyProduction => 11233,
            Self::PoultryHatcheries => 11234,
            Self::OtherPoultryProduction => 11239,
            Self::SheepFarming => 11241,
            Self::GoatFarming => 11242,
            Self::Aquaculture => 11251,
            Self::Apiculture => 11291,
            Self::HorsesAndOtherEquineProduction => 11292,
            Self::FurbearingAnimalAndRabbitProduction => 11293,
            Self::AllOtherAnimalProduction => 11299,
            Self::TimberTractOperations => 11311,
            Self::ForestNurseriesAndGatheringOfForestProducts => 11321,
            Self::Logging => 11331,
            Self::Fishing => 11411,
            Self::HuntingAndTrapping => 11421,
            Self::SupportActivitiesForCropProduction => 11511,
            Self::SupportActivitiesForAnimalProduction => 11521,
            Self::SupportActivitiesForForestry => 11531,
            Self::CrudePetroleumExtraction => 21112,
            Self::NaturalGasExtraction => 21113,
            Self::CoalMining => 21211,
            Self::IronOreMining => 21221,
            Self::GoldOreAndSilverOreMining => 21222,
            Self::CopperNickelLeadAndZincMining => 21223,
            Self::OtherMetalOreMining => 21229,
            Self::StoneMiningAndQuarrying => 21231,
            Self::SandGravelClayAndCeramicAndRefractoryMineralsMiningAndQuarrying => 21232,
            Self::OtherNonmetallicMineralMiningAndQuarrying => 21239,
            Self::SupportActivitiesForMining => 21311,
            Self::ElectricPowerGeneration => 22111,
            Self::ElectricPowerTransmissionControlAndDistribution => 22112,
            Self::NaturalGasDistribution => 22121,
            Self::WaterSupplyAndIrrigationSystems => 22131,
            Self::SewageTreatmentFacilities => 22132,
            Self::SteamAndAirconditioningSupply => 22133,
            Self::ResidentialBuildingConstruction => 23611,
            Self::IndustrialBuildingConstruction => 23621,
            Self::CommercialAndInstitutionalBuildingConstruction => 23622,
            Self::WaterAndSewerLineAndRelatedStructuresConstruction => 23711,
            Self::OilAndGasPipelineAndRelatedStructuresConstruction => 23712,
            Self::PowerAndCommunicationLineAndRelatedStructuresConstruction => 23713,
            Self::LandSubdivision => 23721,
            Self::HighwayStreetAndBridgeConstruction => 23731,
            Self::OtherHeavyAndCivilEngineeringConstruction => 23799,
            Self::PouredConcreteFoundationAndStructureContractors => 23811,
            Self::StructuralSteelAndPrecastConcreteContractors => 23812,
            Self::FramingContractors => 23813,
            Self::MasonryContractors => 23814,
            Self::GlassAndGlazingContractors => 23815,
            Self::RoofingContractors => 23816,
            Self::SidingContractors => 23817,
            Self::OtherFoundationStructureAndBuildingExteriorContractors => 23819,
            Self::ElectricalContractorsAndOtherWiringInstallationContractors => 23821,
            Self::PlumbingHeatingAndAirconditioningContractors => 23822,
            Self::OtherBuildingEquipmentContractors => 23829,
            Self::DrywallAndInsulationContractors => 23831,
            Self::PaintingAndWallCoveringContractors => 23832,
            Self::FlooringContractors => 23833,
            Self::TileAndTerrazzoContractors => 23834,
            Self::FinishCarpentryContractors => 23835,
            Self::OtherBuildingFinishingContractors => 23839,
            Self::SitePreparationContractors => 23891,
            Self::AllOtherSpecialtyTradeContractors => 23899,
            Self::AnimalFoodManufacturing => 31111,
            Self::FlourMillingAndMaltManufacturing => 31121,
            Self::StarchAndVegetableFatsAndOilsManufacturing => 31122,
            Self::BreakfastCerealManufacturing => 31123,
            Self::SugarManufacturing => 31131,
            Self::NonchocolateConfectioneryManufacturing => 31134,
            Self::ChocolateAndConfectioneryManufacturing => 31135,
            Self::FrozenFoodManufacturing => 31141,
            Self::FruitAndVegetableCanningPicklingAndDrying => 31142,
            Self::DairyProductExceptFrozenManufacturing => 31151,
            Self::IceCreamAndFrozenDessertManufacturing => 31152,
            Self::AnimalSlaughteringAndProcessing => 31161,
            Self::SeafoodProductPreparationAndPackaging => 31171,
            Self::BreadAndBakeryProductManufacturing => 31181,
            Self::CookieCrackerAndPastaManufacturing => 31182,
            Self::TortillaManufacturing => 31183,
            Self::SnackFoodManufacturing => 31191,
            Self::CoffeeAndTeaManufacturing => 31192,
            Self::FlavoringSyrupAndConcentrateManufacturing => 31193,
            Self::SeasoningAndDressingManufacturing => 31194,
            Self::AllOtherFoodManufacturing => 31199,
            Self::SoftDrinkAndIceManufacturing => 31211,
            Self::Breweries => 31212,
            Self::Wineries => 31213,
            Self::Distilleries => 31214,
            Self::TobaccoManufacturing => 31223,
            Self::FiberYarnAndThreadMills => 31311,
            Self::BroadwovenFabricMills => 31321,
            Self::NarrowFabricMillsAndSchiffliMachineEmbroidery => 31322,
            Self::NonwovenFabricMills => 31323,
            Self::KnitFabricMills => 31324,
            Self::TextileAndFabricFinishingMills => 31331,
            Self::FabricCoatingMills => 31332,
            Self::CarpetAndRugMills => 31411,
            Self::CurtainAndLinenMills => 31412,
            Self::TextileBagAndCanvasMills => 31491,
            Self::AllOtherTextileProductMills => 31499,
            Self::ApparelKnittingMills => 31512,
            Self::CutAndSewApparelContractors => 31521,
            Self::CutAndSewApparelManufacturingExceptContractors => 31525,
            Self::MensAndBoysCutAndSewApparelManufacturing => 31522,
            Self::WomensAndGirlsCutAndSewApparelManufacturing => 31523,
            Self::OtherCutAndSewApparelManufacturing => 31529,
            Self::ApparelAccessoriesAndOtherApparelManufacturing => 31599,
            Self::LeatherAndHideTanningAndFinishing => 31611,
            Self::FootwearManufacturing => 31621,
            Self::OtherLeatherAndAlliedProductManufacturing => 31699,
            Self::SawmillsAndWoodPreservation => 32111,
            Self::VeneerPlywoodAndEngineeredWoodProductManufacturing => 32121,
            Self::Millwork => 32191,
            Self::WoodContainerAndPalletManufacturing => 32192,
            Self::AllOtherWoodProductManufacturing => 32199,
            Self::PulpMills => 32211,
            Self::PaperMills => 32212,
            Self::PaperboardMills => 32213,
            Self::PaperboardContainerManufacturing => 32221,
            Self::PaperBagAndCoatedAndTreatedPaperManufacturing => 32222,
            Self::StationeryProductManufacturing => 32223,
            Self::OtherConvertedPaperProductManufacturing => 32229,
            Self::Printing => 32311,
            Self::SupportActivitiesForPrinting => 32312,
            Self::PetroleumRefineries => 32411,
            Self::AsphaltPavingRoofingAndSaturatedMaterialsManufacturing => 32412,
            Self::OtherPetroleumAndCoalProductsManufacturing => 32419,
            Self::PetrochemicalManufacturing => 32511,
            Self::IndustrialGasManufacturing => 32512,
            Self::SyntheticDyeAndPigmentManufacturing => 32513,
            Self::OtherBasicInorganicChemicalManufacturing => 32518,
            Self::OtherBasicOrganicChemicalManufacturing => 32519,
            Self::ResinAndSyntheticRubberManufacturing => 32521,
            Self::ArtificialAndSyntheticFibersAndFilamentsManufacturing => 32522,
            Self::FertilizerAndCompostManufacturing => 32531,
            Self::PesticideAndOtherAgriculturalChemicalManufacturing => 32532,
            Self::PharmaceuticalAndMedicineManufacturing => 32541,
            Self::PaintAndCoatingManufacturing => 32551,
            Self::AdhesiveManufacturing => 32552,
            Self::SoapAndCleaningCompoundManufacturing => 32561,
            Self::ToiletPreparationManufacturing => 32562,
            Self::PrintingInkManufacturing => 32591,
            Self::ExplosivesManufacturing => 32592,
            Self::AllOtherChemicalProductAndPreparationManufacturing => 32599,
            Self::PlasticsPackagingMaterialsAndUnlaminatedFilmAndSheetManufacturing => 32611,
            Self::PlasticsPipePipeFittingAndUnlaminatedProfileShapeManufacturing => 32612,
            Self::LaminatedPlasticsPlateSheetExceptPackagingAndShapeManufacturing => 32613,
            Self::PolystyreneFoamProductManufacturing => 32614,
            Self::UrethaneAndOtherFoamProductExceptPolystyreneManufacturing => 32615,
            Self::PlasticsBottleManufacturing => 32616,
            Self::OtherPlasticsProductManufacturing => 32619,
            Self::TireManufacturing => 32621,
            Self::RubberAndPlasticsHosesAndBeltingManufacturing => 32622,
            Self::OtherRubberProductManufacturing => 32629,
            Self::PotteryCeramicsAndPlumbingFixtureManufacturing => 32711,
            Self::ClayBuildingMaterialAndRefractoriesManufacturing => 32712,
            Self::GlassAndGlassProductManufacturing => 32721,
            Self::CementManufacturing => 32731,
            Self::ReadymixConcreteManufacturing => 32732,
            Self::ConcretePipeBrickAndBlockManufacturing => 32733,
            Self::OtherConcreteProductManufacturing => 32739,
            Self::LimeManufacturing => 32741,
            Self::GypsumProductManufacturing => 32742,
            Self::AbrasiveProductManufacturing => 32791,
            Self::AllOtherNonmetallicMineralProductManufacturing => 32799,
            Self::IronAndSteelMillsAndFerroalloyManufacturing => 33111,
            Self::IronAndSteelPipeAndTubeManufacturingFromPurchasedSteel => 33121,
            Self::RollingAndDrawingOfPurchasedSteel => 33122,
            Self::AluminaAndAluminumProductionAndProcessing => 33131,
            Self::NonferrousMetalExceptAluminumSmeltingAndRefining => 33141,
            Self::CopperRollingDrawingExtrudingAndAlloying => 33142,
            Self::NonferrousMetalExceptCopperAndAluminumRollingDrawingExtrudingAndAlloying => 33149,
            Self::FerrousMetalFoundries => 33151,
            Self::NonferrousMetalFoundries => 33152,
            Self::ForgingAndStamping => 33211,
            Self::CutleryAndHandtoolManufacturing => 33221,
            Self::PlateWorkAndFabricatedStructuralProductManufacturing => 33231,
            Self::OrnamentalAndArchitecturalMetalProductsManufacturing => 33232,
            Self::PowerBoilerAndHeatExchangerManufacturing => 33241,
            Self::MetalTankHeavyGaugeManufacturing => 33242,
            Self::MetalCanBoxAndOtherMetalContainerLightGaugeManufacturing => 33243,
            Self::HardwareManufacturing => 33251,
            Self::SpringAndWireProductManufacturing => 33261,
            Self::MachineShops => 33271,
            Self::TurnedProductAndScrewNutAndBoltManufacturing => 33272,
            Self::CoatingEngravingHeatTreatingAndAlliedActivities => 33281,
            Self::MetalValveManufacturing => 33291,
            Self::AllOtherFabricatedMetalProductManufacturing => 33299,
            Self::AgriculturalImplementManufacturing => 33311,
            Self::ConstructionMachineryManufacturing => 33312,
            Self::MiningAndOilAndGasFieldMachineryManufacturing => 33313,
            Self::IndustrialMachineryManufacturing => 33324,
            Self::CommercialAndServiceIndustryMachineryManufacturing => 33331,
            Self::VentilationHeatingAirconditioningAndCommercialRefrigerationEquipmentManufacturing => 33341,
            Self::MetalworkingMachineryManufacturing => 33351,
            Self::EngineTurbineAndPowerTransmissionEquipmentManufacturing => 33361,
            Self::PumpAndCompressorManufacturing => 33391,
            Self::MaterialHandlingEquipmentManufacturing => 33392,
            Self::AllOtherGeneralPurposeMachineryManufacturing => 33399,
            Self::ComputerAndPeripheralEquipmentManufacturing => 33411,
            Self::TelephoneApparatusManufacturing => 33421,
            Self::RadioAndTelevisionBroadcastingAndWirelessCommunicationsEquipmentManufacturing => 33422,
            Self::OtherCommunicationsEquipmentManufacturing => 33429,
            Self::AudioAndVideoEquipmentManufacturing => 33431,
            Self::SemiconductorAndOtherElectronicComponentManufacturing => 33441,
            Self::NavigationalMeasuringElectromedicalAndControlInstrumentsManufacturing => 33451,
            Self::ManufacturingAndReproducingMagneticAndOpticalMedia => 33461,
            Self::ElectricLightingEquipmentManufacturing => 33513,
            Self::SmallElectricalApplianceManufacturing => 33521,
            Self::MajorHouseholdApplianceManufacturing => 33522,
            Self::ElectricalEquipmentManufacturing => 33531,
            Self::BatteryManufacturing => 33591,
            Self::CommunicationAndEnergyWireAndCableManufacturing => 33592,
            Self::WiringDeviceManufacturing => 33593,
            Self::AllOtherElectricalEquipmentAndComponentManufacturing => 33599,
            Self::AutomobileAndLightDutyMotorVehicleManufacturing => 33611,
            Self::HeavyDutyTruckManufacturing => 33612,
            Self::MotorVehicleBodyAndTrailerManufacturing => 33621,
            Self::MotorVehicleGasolineEngineAndEnginePartsManufacturing => 33631,
            Self::MotorVehicleElectricalAndElectronicEquipmentManufacturing => 33632,
            Self::MotorVehicleSteeringAndSuspensionComponentsExceptSpringManufacturing => 33633,
            Self::MotorVehicleBrakeSystemManufacturing => 33634,
            Self::MotorVehicleTransmissionAndPowerTrainPartsManufacturing => 33635,
            Self::MotorVehicleSeatingAndInteriorTrimManufacturing => 33636,
            Self::MotorVehicleMetalStamping => 33637,
            Self::OtherMotorVehiclePartsManufacturing => 33639,
            Self::AerospaceProductAndPartsManufacturing => 33641,
            Self::RailroadRollingStockManufacturing => 33651,
            Self::ShipAndBoatBuilding => 33661,
            Self::OtherTransportationEquipmentManufacturing => 33699,
            Self::WoodKitchenCabinetAndCountertopManufacturing => 33711,
            Self::HouseholdAndInstitutionalFurnitureManufacturing => 33712,
            Self::OfficeFurnitureIncludingFixturesManufacturing => 33721,
            Self::MattressManufacturing => 33791,
            Self::BlindAndShadeManufacturing => 33792,
            Self::MedicalEquipmentAndSuppliesManufacturing => 33911,
            Self::JewelryAndSilverwareManufacturing => 33991,
            Self::SportingAndAthleticGoodsManufacturing => 33992,
            Self::DollToyAndGameManufacturing => 33993,
            Self::OfficeSuppliesExceptPaperManufacturing => 33994,
            Self::SignManufacturing => 33995,
            Self::AllOtherMiscellaneousManufacturing => 33999,
            Self::AutomobileAndOtherMotorVehicleMerchantWholesalers => 42311,
            Self::MotorVehicleSuppliesAndNewPartsMerchantWholesalers => 42312,
            Self::TireAndTubeMerchantWholesalers => 42313,
            Self::MotorVehiclePartsUsedMerchantWholesalers => 42314,
            Self::FurnitureMerchantWholesalers => 42321,
            Self::HomeFurnishingMerchantWholesalers => 42322,
            Self::LumberPlywoodMillworkAndWoodPanelMerchantWholesalers => 42331,
            Self::BrickStoneAndRelatedConstructionMaterialMerchantWholesalers => 42332,
            Self::RoofingSidingAndInsulationMaterialMerchantWholesalers => 42333,
            Self::OtherConstructionMaterialMerchantWholesalers => 42339,
            Self::PhotographicEquipmentAndSuppliesMerchantWholesalers => 42341,
            Self::OfficeEquipmentMerchantWholesalers => 42342,
            Self::ComputerAndComputerPeripheralEquipmentAndSoftwareMerchantWholesalers => 42343,
            Self::OtherCommercialEquipmentMerchantWholesalers => 42344,
            Self::MedicalDentalAndHospitalEquipmentAndSuppliesMerchantWholesalers => 42345,
            Self::OphthalmicGoodsMerchantWholesalers => 42346,
            Self::OtherProfessionalEquipmentAndSuppliesMerchantWholesalers => 42349,
            Self::MetalServiceCentersAndOtherMetalMerchantWholesalers => 42351,
            Self::CoalAndOtherMineralAndOreMerchantWholesalers => 42352,
            Self::ElectricalApparatusAndEquipmentWiringSuppliesAndRelatedEquipmentMerchantWholesalers => 42361,
            Self::HouseholdAppliancesElectricHousewaresAndConsumerElectronicsMerchantWholesalers => 42362,
            Self::OtherElectronicPartsAndEquipmentMerchantWholesalers => 42369,
            Self::HardwareMerchantWholesalers => 42371,
            Self::PlumbingAndHeatingEquipmentAndSuppliesHydronicsMerchantWholesalers => 42372,
            Self::WarmAirHeatingAndAirconditioningEquipmentAndSuppliesMerchantWholesalers => 42373,
            Self::RefrigerationEquipmentAndSuppliesMerchantWholesalers => 42374,
            Self::ConstructionAndMiningExceptOilWellMachineryAndEquipmentMerchantWholesalers => 42381,
            Self::FarmAndGardenMachineryAndEquipmentMerchantWholesalers => 42382,
            Self::IndustrialMachineryAndEquipmentMerchantWholesalers => 42383,
            Self::IndustrialSuppliesMerchantWholesalers => 42384,
            Self::ServiceEstablishmentEquipmentAndSuppliesMerchantWholesalers => 42385,
            Self::TransportationEquipmentAndSuppliesExceptMotorVehicleMerchantWholesalers => 42386,
            Self::SportingAndRecreationalGoodsAndSuppliesMerchantWholesalers => 42391,
            Self::ToyAndHobbyGoodsAndSuppliesMerchantWholesalers => 42392,
            Self::RecyclableMaterialMerchantWholesalers => 42393,
            Self::JewelryWatchPreciousStoneAndPreciousMetalMerchantWholesalers => 42394,
            Self::OtherMiscellaneousDurableGoodsMerchantWholesalers => 42399,
            Self::PrintingAndWritingPaperMerchantWholesalers => 42411,
            Self::StationeryAndOfficeSuppliesMerchantWholesalers => 42412,
            Self::IndustrialAndPersonalServicePaperMerchantWholesalers => 42413,
            Self::DrugsAndDruggistsSundriesMerchantWholesalers => 42421,
            Self::PieceGoodsNotionsAndOtherDryGoodsMerchantWholesalers => 42431,
            Self::FootwearMerchantWholesalers => 42434,
            Self::ClothingAndClothingAccessoriesMerchantWholesalers => 42435,
            Self::GeneralLineGroceryMerchantWholesalers => 42441,
            Self::PackagedFrozenFoodMerchantWholesalers => 42442,
            Self::DairyProductExceptDriedOrCannedMerchantWholesalers => 42443,
            Self::PoultryAndPoultryProductMerchantWholesalers => 42444,
            Self::ConfectioneryMerchantWholesalers => 42445,
            Self::FishAndSeafoodMerchantWholesalers => 42446,
            Self::MeatAndMeatProductMerchantWholesalers => 42447,
            Self::FreshFruitAndVegetableMerchantWholesalers => 42448,
            Self::OtherGroceryAndRelatedProductsMerchantWholesalers => 42449,
            Self::GrainAndFieldBeanMerchantWholesalers => 42451,
            Self::LivestockMerchantWholesalers => 42452,
            Self::OtherFarmProductRawMaterialMerchantWholesalers => 42459,
            Self::PlasticsMaterialsAndBasicFormsAndShapesMerchantWholesalers => 42461,
            Self::OtherChemicalAndAlliedProductsMerchantWholesalers => 42469,
            Self::PetroleumBulkStationsAndTerminals => 42471,
            Self::PetroleumAndPetroleumProductsMerchantWholesalersExceptBulkStationsAndTerminals => 42472,
            Self::BeerAndAleMerchantWholesalers => 42481,
            Self::WineAndDistilledAlcoholicBeverageMerchantWholesalers => 42482,
            Self::FarmSuppliesMerchantWholesalers => 42491,
            Self::BookPeriodicalAndNewspaperMerchantWholesalers => 42492,
            Self::FlowerNurseryStockAndFloristsSuppliesMerchantWholesalers => 42493,
            Self::TobaccoProductAndElectronicCigaretteMerchantWholesalers => 42494,
            Self::PaintVarnishAndSuppliesMerchantWholesalers => 42495,
            Self::OtherMiscellaneousNondurableGoodsMerchantWholesalers => 42499,
            Self::WholesaleTradeAgentsAndBrokers => 42512,
            Self::NewCarDealers => 44111,
            Self::UsedCarDealers => 44112,
            Self::RecreationalVehicleDealers => 44121,
            Self::MotorcycleBoatAndOtherMotorVehicleDealers => 44122,
            Self::AutomotivePartsAndAccessoriesRetailers => 44133,
            Self::TireDealers => 44134,
            Self::HomeCenters => 44411,
            Self::PaintAndWallpaperRetailers => 44412,
            Self::HardwareRetailers => 44414,
            Self::OtherBuildingMaterialDealers => 44418,
            Self::OutdoorPowerEquipmentRetailers => 44423,
            Self::NurseryGardenCenterAndFarmSupplyRetailers => 44424,
            Self::SupermarketsAndOtherGroceryRetailersExceptConvenienceRetailers => 44511,
            Self::ConvenienceRetailersAndVendingMachineOperators => 44513,
            Self::FruitAndVegetableRetailers => 44523,
            Self::MeatRetailers => 44524,
            Self::FishAndSeafoodRetailers => 44525,
            Self::OtherSpecialtyFoodRetailers => 44529,
            Self::BeerWineAndLiquorRetailers => 44532,
            Self::FurnitureRetailers => 44911,
            Self::HomeFurnishingsRetailers => 44912,
            Self::ElectronicsAndApplianceRetailers => 44921,
            Self::DepartmentStores => 45511,
            Self::WarehouseClubsAndSupercenters => 45521,
            Self::AllOtherGeneralMerchandiseRetailers => 45522,
            Self::PharmaciesAndDrugRetailers => 45611,
            Self::CosmeticsBeautySuppliesAndPerfumeRetailers => 45612,
            Self::OpticalGoodsRetailers => 45613,
            Self::OtherHealthAndPersonalCareRetailers => 45619,
            Self::GasolineStationsWithConvenienceStores => 45711,
            Self::OtherGasolineStations => 45712,
            Self::FuelDealers => 45721,
            Self::ClothingRetailers => 45811,
            Self::ClothingAccessoriesRetailers => 45812,
            Self::ShoeRetailers => 45821,
            Self::JewelryRetailers => 45831,
            Self::LuggageAndLeatherGoodsRetailers => 45832,
            Self::SportingGoodsRetailers => 45911,
            Self::HobbyToyAndGameRetailers => 45912,
            Self::SewingNeedleworkAndPieceGoodsRetailers => 45913,
            Self::MusicalInstrumentAndSuppliesRetailers => 45914,
            Self::BookRetailersAndNewsDealers => 45921,
            Self::Florists => 45931,
            Self::OfficeSuppliesAndStationeryRetailers => 45941,
            Self::GiftNoveltyAndSouvenirRetailers => 45942,
            Self::UsedMerchandiseRetailers => 45951,
            Self::PetAndPetSuppliesRetailers => 45991,
            Self::ArtDealers => 45992,
            Self::ManufacturedMobileHomeDealers => 45993,
            Self::AllOtherMiscellaneousRetailers => 45999,
            Self::ScheduledAirTransportation => 48111,
            Self::NonscheduledAirTransportation => 48121,
            Self::RailTransportation => 48211,
            Self::DeepSeaCoastalAndGreatLakesWaterTransportation => 48311,
            Self::InlandWaterTransportation => 48321,
            Self::GeneralFreightTruckingLocal => 48411,
            Self::GeneralFreightTruckingLongdistance => 48412,
            Self::UsedHouseholdAndOfficeGoodsMoving => 48421,
            Self::SpecializedFreightExceptUsedGoodsTruckingLocal => 48422,
            Self::SpecializedFreightExceptUsedGoodsTruckingLongdistance => 48423,
            Self::UrbanTransitSystems => 48511,
            Self::InterurbanAndRuralBusTransportation => 48521,
            Self::TaxiService => 48531,
            Self::LimousineService => 48532,
            Self::SchoolAndEmployeeBusTransportation => 48541,
            Self::CharterBusIndustry => 48551,
            Self::OtherTransitAndGroundPassengerTransportation => 48599,
            Self::PipelineTransportationOfCrudeOil => 48611,
            Self::PipelineTransportationOfNaturalGas => 48621,
            Self::PipelineTransportationOfRefinedPetroleumProducts => 48691,
            Self::AllOtherPipelineTransportation => 48699,
            Self::ScenicAndSightseeingTransportationLand => 48711,
            Self::ScenicAndSightseeingTransportationWater => 48721,
            Self::ScenicAndSightseeingTransportationOther => 48799,
            Self::AirportOperations => 48811,
            Self::OtherSupportActivitiesForAirTransportation => 48819,
            Self::SupportActivitiesForRailTransportation => 48821,
            Self::PortAndHarborOperations => 48831,
            Self::MarineCargoHandling => 48832,
            Self::NavigationalServicesToShipping => 48833,
            Self::OtherSupportActivitiesForWaterTransportation => 48839,
            Self::MotorVehicleTowing => 48841,
            Self::OtherSupportActivitiesForRoadTransportation => 48849,
            Self::FreightTransportationArrangement => 48851,
            Self::OtherSupportActivitiesForTransportation => 48899,
            Self::PostalService => 49111,
            Self::CouriersAndExpressDeliveryServices => 49211,
            Self::LocalMessengersAndLocalDelivery => 49221,
            Self::GeneralWarehousingAndStorage => 49311,
            Self::RefrigeratedWarehousingAndStorage => 49312,
            Self::FarmProductWarehousingAndStorage => 49313,
            Self::OtherWarehousingAndStorage => 49319,
            Self::NewspaperPublishers => 51111,
            Self::PeriodicalPublishers => 51112,
            Self::BookPublishers => 51113,
            Self::DirectoryAndMailingListPublishers => 51114,
            Self::OtherPublishers => 51119,
            Self::SoftwarePublishers => 51121,
            Self::MotionPictureAndVideoProduction => 51211,
            Self::MotionPictureAndVideoDistribution => 51212,
            Self::MotionPictureAndVideoExhibition => 51213,
            Self::PostproductionServicesAndOtherMotionPictureAndVideoIndustries => 51219,
            Self::MusicPublishers => 51223,
            Self::SoundRecordingStudios => 51224,
            Self::RecordProductionAndDistribution => 51225,
            Self::OtherSoundRecordingIndustries => 51229,
            Self::RadioBroadcasting => 51511,
            Self::TelevisionBroadcasting => 51512,
            Self::CableAndOtherSubscriptionProgramming => 51521,
            Self::WiredAndWirelessTelecommunicationsCarriers => 51731,
            Self::SatelliteTelecommunications => 51741,
            Self::OtherTelecommunications => 51791,
            Self::ComputingInfrastructureProvidersDataProcessingWebHostingAndRelatedServices => 51821,
            Self::NewsSyndicates => 51911,
            Self::LibrariesAndArchives => 51912,
            Self::InternetPublishingAndBroadcastingAndWebSearchPortals => 51913,
            Self::AllOtherInformationServices => 51919,
            Self::MonetaryAuthoritiesCentralBank => 52111,
            Self::CommercialBanking => 52211,
            Self::SavingsInstitutions => 52212,
            Self::CreditUnions => 52213,
            Self::OtherDepositoryCreditIntermediation => 52219,
            Self::CreditCardIssuing => 52221,
            Self::SalesFinancing => 52222,
            Self::OtherNondepositoryCreditIntermediation => 52229,
            Self::MortgageAndNonmortgageLoanBrokers => 52231,
            Self::FinancialTransactionsProcessingReserveAndClearinghouseActivities => 52232,
            Self::OtherActivitiesRelatedToCreditIntermediation => 52239,
            Self::InvestmentBankingAndSecuritiesDealing => 52311,
            Self::SecuritiesBrokerage => 52312,
            Self::CommodityContractsDealing => 52313,
            Self::CommodityContractsBrokerage => 52314,
            Self::SecuritiesAndCommodityExchanges => 52321,
            Self::MiscellaneousIntermediation => 52391,
            Self::PortfolioManagement => 52392,
            Self::InvestmentAdvice => 52393,
            Self::AllOtherFinancialInvestmentActivities => 52399,
            Self::DirectLifeHealthAndMedicalInsuranceCarriers => 52411,
            Self::DirectInsuranceExceptLifeHealthAndMedicalCarriers => 52412,
            Self::ReinsuranceCarriers => 52413,
            Self::InsuranceAgenciesAndBrokerages => 52421,
            Self::OtherInsuranceRelatedActivities => 52429,
            Self::PensionFunds => 52511,
            Self::HealthAndWelfareFunds => 52512,
            Self::OtherInsuranceFunds => 52519,
            Self::OpenEndInvestmentFunds => 52591,
            Self::TrustsEstatesAndAgencyAccounts => 52592,
            Self::RealEstateInvestmentTrusts => 52593,
            Self::OtherFinancialVehicles => 52599,
            Self::LessorsOfResidentialBuildingsAndDwellings => 53111,
            Self::LessorsOfNonresidentialBuildingsExceptMiniwarehouses => 53112,
            Self::LessorsOfMiniwarehousesAndSelfStorageUnits => 53113,
            Self::LessorsOfOtherRealEstateProperty => 53119,
            Self::OfficesOfRealEstateAgentsAndBrokers => 53121,
            Self::RealEstatePropertyManagers => 53131,
            Self::OfficesOfRealEstateAppraisers => 53132,
            Self::OtherActivitiesRelatedToRealEstate => 53139,
            Self::PassengerCarRentalAndLeasing => 53211,
            Self::TruckUtilityTrailerAndRvRecreationalVehicleRentalAndLeasing => 53212,
            Self::ConsumerElectronicsAndAppliancesRental => 53221,
            Self::FormalWearAndCostumeRental => 53222,
            Self::VideoTapeAndDiscRental => 53223,
            Self::OtherConsumerGoodsRental => 53229,
            Self::GeneralRentalCenters => 53231,
            Self::ConstructionTransportationMiningAndForestryMachineryAndEquipmentRentalAndLeasing => 53241,
            Self::OfficeMachineryAndEquipmentRentalAndLeasing => 53242,
            Self::OtherCommercialAndIndustrialMachineryAndEquipmentRentalAndLeasing => 53249,
            Self::LessorsOfNonfinancialIntangibleAssetsExceptCopyrightedWorks => 53311,
            Self::OfficesOfLawyers => 54111,
            Self::OfficesOfNotaries => 54112,
            Self::OtherLegalServices => 54119,
            Self::AccountingTaxPreparationBookkeepingAndPayrollServices => 54121,
            Self::ArchitecturalServices => 54131,
            Self::LandscapeArchitecturalServices => 54132,
            Self::EngineeringServices => 54133,
            Self::DraftingServices => 54134,
            Self::BuildingInspectionServices => 54135,
            Self::GeophysicalSurveyingAndMappingServices => 54136,
            Self::SurveyingAndMappingExceptGeophysicalServices => 54137,
            Self::TestingLaboratories => 54138,
            Self::InteriorDesignServices => 54141,
            Self::IndustrialDesignServices => 54142,
            Self::GraphicDesignServices => 54143,
            Self::OtherSpecializedDesignServices => 54149,
            Self::ComputerSystemsDesignAndRelatedServices => 54151,
            Self::ManagementConsultingServices => 54161,
            Self::EnvironmentalConsultingServices => 54162,
            Self::OtherScientificAndTechnicalConsultingServices => 54169,
            Self::ResearchAndDevelopmentInThePhysicalEngineeringAndLifeSciences => 54171,
            Self::ResearchAndDevelopmentInTheSocialSciencesAndHumanities => 54172,
            Self::AdvertisingAgencies => 54181,
            Self::PublicRelationsAgencies => 54182,
            Self::MediaBuyingAgencies => 54183,
            Self::MediaRepresentatives => 54184,
            Self::OutdoorAdvertising => 54185,
            Self::DirectMailAdvertising => 54186,
            Self::AdvertisingMaterialDistributionServices => 54187,
            Self::OtherServicesRelatedToAdvertising => 54189,
            Self::MarketingResearchAndPublicOpinionPolling => 54191,
            Self::PhotographicServices => 54192,
            Self::TranslationAndInterpretationServices => 54193,
            Self::VeterinaryServices => 54194,
            Self::AllOtherProfessionalScientificAndTechnicalServices => 54199,
            Self::ManagementOfCompaniesAndEnterprises => 55111,
            Self::OfficeAdministrativeServices => 56111,
            Self::FacilitiesSupportServices => 56121,
            Self::EmploymentPlacementAgenciesAndExecutiveSearchServices => 56131,
            Self::TemporaryHelpServices => 56132,
            Self::ProfessionalEmployerOrganizations => 56133,
            Self::DocumentPreparationServices => 56141,
            Self::TelephoneCallCenters => 56142,
            Self::BusinessServiceCenters => 56143,
            Self::CollectionAgencies => 56144,
            Self::CreditBureaus => 56145,
            Self::OtherBusinessSupportServices => 56149,
            Self::TravelAgencies => 56151,
            Self::TourOperators => 56152,
            Self::OtherTravelArrangementAndReservationServices => 56159,
            Self::InvestigationGuardAndArmoredCarServices => 56161,
            Self::SecuritySystemsServices => 56162,
            Self::ExterminatingAndPestControlServices => 56171,
            Self::JanitorialServices => 56172,
            Self::LandscapingServices => 56173,
            Self::CarpetAndUpholsteryCleaningServices => 56174,
            Self::OtherServicesToBuildingsAndDwellings => 56179,
            Self::PackagingAndLabelingServices => 56191,
            Self::ConventionAndTradeShowOrganizers => 56192,
            Self::AllOtherSupportServices => 56199,
            Self::WasteCollection => 56211,
            Self::WasteTreatmentAndDisposal => 56221,
            Self::RemediationServices => 56291,
            Self::MaterialsRecoveryFacilities => 56292,
            Self::AllOtherWasteManagementServices => 56299,
            Self::ElementaryAndSecondarySchools => 61111,
            Self::JuniorColleges => 61121,
            Self::CollegesUniversitiesAndProfessionalSchools => 61131,
            Self::BusinessAndSecretarialSchools => 61141,
            Self::ComputerTraining => 61142,
            Self::ProfessionalAndManagementDevelopmentTraining => 61143,
            Self::TechnicalAndTradeSchools => 61151,
            Self::FineArtsSchools => 61161,
            Self::SportsAndRecreationInstruction => 61162,
            Self::LanguageSchools => 61163,
            Self::AllOtherSchoolsAndInstruction => 61169,
            Self::EducationalSupportServices => 61171,
            Self::OfficesOfPhysicians => 62111,
            Self::OfficesOfDentists => 62121,
            Self::OfficesOfChiropractors => 62131,
            Self::OfficesOfOptometrists => 62132,
            Self::OfficesOfMentalHealthPractitionersExceptPhysicians => 62133,
            Self::OfficesOfPhysicalOccupationalAndSpeechTherapistsAndAudiologists => 62134,
            Self::OfficesOfAllOtherHealthPractitioners => 62139,
            Self::FamilyPlanningCenters => 62141,
            Self::OutpatientMentalHealthAndSubstanceAbuseCenters => 62142,
            Self::OtherOutpatientCareCenters => 62149,
            Self::MedicalAndDiagnosticLaboratories => 62151,
            Self::HomeHealthCareServices => 62161,
            Self::AmbulanceServices => 62191,
            Self::AllOtherAmbulatoryHealthCareServices => 62199,
            Self::GeneralMedicalAndSurgicalHospitals => 62211,
            Self::PsychiatricAndSubstanceAbuseHospitals => 62221,
            Self::SpecialtyExceptPsychiatricAndSubstanceAbuseHospitals => 62231,
            Self::NursingCareFacilitiesSkilledNursingFacilities => 62311,
            Self::ResidentialIntellectualAndDevelopmentalDisabilityFacilities => 62321,
            Self::ResidentialMentalHealthAndSubstanceAbuseFacilities => 62322,
            Self::ContinuingCareRetirementCommunitiesAndAssistedLivingFacilitiesForTheElderly => 62331,
            Self::OtherResidentialCareFacilities => 62399,
            Self::ChildAndYouthServices => 62411,
            Self::ServicesForTheElderlyAndPersonsWithDisabilities => 62412,
            Self::OtherIndividualAndFamilyServices => 62419,
            Self::CommunityFoodServices => 62421,
            Self::CommunityHousingServices => 62422,
            Self::EmergencyAndOtherReliefServices => 62423,
            Self::VocationalRehabilitationServices => 62431,
            Self::ChildDayCareServices => 62441,
            Self::TheaterCompaniesAndDinnerTheaters => 71111,
            Self::DanceCompanies => 71112,
            Self::MusicalGroupsAndArtists => 71113,
            Self::OtherPerformingArtsCompanies => 71119,
            Self::SpectatorSports => 71121,
            Self::PromotersOfPerformingArtsSportsAndSimilarEventsWithFacilities => 71131,
            Self::PromotersOfPerformingArtsSportsAndSimilarEventsWithoutFacilities => 71132,
            Self::AgentsAndManagersForArtistsAthletesEntertainersAndOtherPublicFigures => 71141,
            Self::IndependentArtistsWritersAndPerformers => 71151,
            Self::Museums => 71211,
            Self::HistoricalSites => 71212,
            Self::ZoosAndBotanicalGardens => 71213,
            Self::NatureParksAndOtherSimilarInstitutions => 71219,
            Self::AmusementAndThemeParks => 71311,
            Self::AmusementArcades => 71312,
            Self::CasinosExceptCasinoHotels => 71321,
            Self::OtherGamblingIndustries => 71329,
            Self::GolfCoursesAndCountryClubs => 71391,
            Self::SkiingFacilities => 71392,
            Self::Marinas => 71393,
            Self::FitnessAndRecreationalSportsCenters => 71394,
            Self::BowlingCenters => 71395,
            Self::AllOtherAmusementAndRecreationIndustries => 71399,
            Self::HotelsExceptCasinoHotelsAndMotels => 72111,
            Self::CasinoHotels => 72112,
            Self::OtherTravelerAccommodation => 72119,
            Self::RvRecreationalVehicleParksAndRecreationalCamps => 72121,
            Self::RoomingAndBoardingHouses => 72131,
            Self::FoodServiceContractors => 72231,
            Self::Caterers => 72232,
            Self::MobileFoodServices => 72233,
            Self::DrinkingPlacesAlcoholicBeverages => 72241,
            Self::RestaurantsAndOtherEatingPlaces => 72251,
            Self::AutomotiveMechanicalAndElectricalRepairAndMaintenance => 81111,
            Self::AutomotiveBodyPaintInteriorAndGlassRepair => 81112,
            Self::OtherAutomotiveRepairAndMaintenance => 81119,
            Self::ElectronicAndPrecisionEquipmentRepairAndMaintenance => 81121,
            Self::CommercialAndIndustrialMachineryAndEquipmentExceptAutomotiveAndElectronicRepairAndMaintenance => 81131,
            Self::HomeAndGardenEquipmentAndApplianceRepairAndMaintenance => 81141,
            Self::ReupholsteryAndFurnitureRepair => 81142,
            Self::FootwearAndLeatherGoodsRepair => 81143,
            Self::OtherPersonalAndHouseholdGoodsRepairAndMaintenance => 81149,
            Self::HairNailAndSkinCareServices => 81211,
            Self::OtherPersonalCareServices => 81219,
            Self::FuneralHomesAndFuneralServices => 81221,
            Self::CemeteriesAndCrematories => 81222,
            Self::CoinOperatedLaundriesAndDrycleaners => 81231,
            Self::DrycleaningAndLaundryServicesExceptCoinOperated => 81232,
            Self::LinenAndUniformSupply => 81233,
            Self::PetCareExceptVeterinaryServices => 81291,
            Self::Photofinishing => 81292,
            Self::ParkingLotsAndGarages => 81293,
            Self::AllOtherPersonalServices => 81299,
            Self::ReligiousOrganizations => 81311,
            Self::GrantmakingAndGivingServices => 81321,
            Self::SocialAdvocacyOrganizations => 81331,
            Self::CivicAndSocialOrganizations => 81341,
            Self::BusinessAssociations => 81391,
            Self::ProfessionalOrganizations => 81392,
            Self::LaborUnionsAndSimilarLaborOrganizations => 81393,
            Self::PoliticalOrganizations => 81394,
            Self::OtherSimilarOrganizationsExceptBusinessProfessionalLaborAndPoliticalOrganizations => 81399,
            Self::PrivateHouseholds => 81411,
            Self::ExecutiveOffices => 92111,
            Self::LegislativeBodies => 92112,
            Self::PublicFinanceActivities => 92113,
            Self::ExecutiveAndLegislativeOfficesCombined => 92114,
            Self::AmericanIndianAndAlaskaNativeTribalGovernments => 92115,
            Self::OtherGeneralGovernmentSupport => 92119,
            Self::Courts => 92211,
            Self::PoliceProtection => 92212,
            Self::LegalCounselAndProsecution => 92213,
            Self::CorrectionalInstitutions => 92214,
            Self::ParoleOfficesAndProbationOffices => 92215,
            Self::FireProtection => 92216,
            Self::OtherJusticePublicOrderAndSafetyActivities => 92219,
            Self::AdministrationOfEducationPrograms => 92311,
            Self::AdministrationOfPublicHealthPrograms => 92312,
            Self::AdministrationOfHumanResourceProgramsExceptEducationPublicHealthAndVeteransAffairsPrograms => 92313,
            Self::AdministrationOfVeteransAffairs => 92314,
            Self::AdministrationOfAirAndWaterResourceAndSolidWasteManagementPrograms => 92411,
            Self::AdministrationOfConservationPrograms => 92412,
            Self::AdministrationOfHousingPrograms => 92511,
            Self::AdministrationOfUrbanPlanningAndCommunityAndRuralDevelopment => 92512,
            Self::AdministrationOfGeneralEconomicPrograms => 92611,
            Self::RegulationAndAdministrationOfTransportationPrograms => 92612,
            Self::RegulationAndAdministrationOfCommunicationsElectricGasAndOtherUtilities => 92613,
            Self::RegulationOfAgriculturalMarketingAndCommodities => 92614,
            Self::RegulationLicensingAndInspectionOfMiscellaneousCommercialSectors => 92615,
            Self::SpaceResearchAndTechnology => 92711,
            Self::NationalSecurity => 92811,
            Self::InternationalAffairs => 92812,
            Self::UnclassifiedEstablishments => 99999,
        }
    }

    /// Tries to create a NaicsSubcategory from a string representing a NAICS code
    /// Returns Some(NaicsSubcategory) if the code is valid, None otherwise
    pub fn from_code(code: &str) -> Option<Self> {
        let code = code.parse::<i64>().ok()?;

        let result = match code {
            11111 => Self::SoybeanFarming,
            11112 => Self::OilseedExceptSoybeanFarming,
            11113 => Self::DryPeaAndBeanFarming,
            11114 => Self::WheatFarming,
            11115 => Self::CornFarming,
            11116 => Self::RiceFarming,
            11119 => Self::OtherGrainFarming,
            11121 => Self::VegetableAndMelonFarming,
            11131 => Self::OrangeGroves,
            11132 => Self::CitrusExceptOrangeGroves,
            11133 => Self::NoncitrusFruitAndTreeNutFarming,
            11141 => Self::FoodCropsGrownUnderCover,
            11142 => Self::NurseryAndFloricultureProduction,
            11191 => Self::TobaccoFarming,
            11192 => Self::CottonFarming,
            11193 => Self::SugarcaneFarming,
            11194 => Self::HayFarming,
            11199 => Self::AllOtherCropFarming,
            11211 => Self::BeefCattleRanchingAndFarmingIncludingFeedlots,
            11212 => Self::DairyCattleAndMilkProduction,
            11213 => Self::DualpurposeCattleRanchingAndFarming,
            11221 => Self::HogAndPigFarming,
            11231 => Self::ChickenEggProduction,
            11232 => Self::BroilersAndOtherMeatTypeChickenProduction,
            11233 => Self::TurkeyProduction,
            11234 => Self::PoultryHatcheries,
            11239 => Self::OtherPoultryProduction,
            11241 => Self::SheepFarming,
            11242 => Self::GoatFarming,
            11251 => Self::Aquaculture,
            11291 => Self::Apiculture,
            11292 => Self::HorsesAndOtherEquineProduction,
            11293 => Self::FurbearingAnimalAndRabbitProduction,
            11299 => Self::AllOtherAnimalProduction,
            11311 => Self::TimberTractOperations,
            11321 => Self::ForestNurseriesAndGatheringOfForestProducts,
            11331 => Self::Logging,
            11411 => Self::Fishing,
            11421 => Self::HuntingAndTrapping,
            11511 => Self::SupportActivitiesForCropProduction,
            11521 => Self::SupportActivitiesForAnimalProduction,
            11531 => Self::SupportActivitiesForForestry,
            21112 => Self::CrudePetroleumExtraction,
            21113 => Self::NaturalGasExtraction,
            21211 => Self::CoalMining,
            21221 => Self::IronOreMining,
            21222 => Self::GoldOreAndSilverOreMining,
            21223 => Self::CopperNickelLeadAndZincMining,
            21229 => Self::OtherMetalOreMining,
            21231 => Self::StoneMiningAndQuarrying,
            21232 => Self::SandGravelClayAndCeramicAndRefractoryMineralsMiningAndQuarrying,
            21239 => Self::OtherNonmetallicMineralMiningAndQuarrying,
            21311 => Self::SupportActivitiesForMining,
            22111 => Self::ElectricPowerGeneration,
            22112 => Self::ElectricPowerTransmissionControlAndDistribution,
            22121 => Self::NaturalGasDistribution,
            22131 => Self::WaterSupplyAndIrrigationSystems,
            22132 => Self::SewageTreatmentFacilities,
            22133 => Self::SteamAndAirconditioningSupply,
            23611 => Self::ResidentialBuildingConstruction,
            23621 => Self::IndustrialBuildingConstruction,
            23622 => Self::CommercialAndInstitutionalBuildingConstruction,
            23711 => Self::WaterAndSewerLineAndRelatedStructuresConstruction,
            23712 => Self::OilAndGasPipelineAndRelatedStructuresConstruction,
            23713 => Self::PowerAndCommunicationLineAndRelatedStructuresConstruction,
            23721 => Self::LandSubdivision,
            23731 => Self::HighwayStreetAndBridgeConstruction,
            23799 => Self::OtherHeavyAndCivilEngineeringConstruction,
            23811 => Self::PouredConcreteFoundationAndStructureContractors,
            23812 => Self::StructuralSteelAndPrecastConcreteContractors,
            23813 => Self::FramingContractors,
            23814 => Self::MasonryContractors,
            23815 => Self::GlassAndGlazingContractors,
            23816 => Self::RoofingContractors,
            23817 => Self::SidingContractors,
            23819 => Self::OtherFoundationStructureAndBuildingExteriorContractors,
            23821 => Self::ElectricalContractorsAndOtherWiringInstallationContractors,
            23822 => Self::PlumbingHeatingAndAirconditioningContractors,
            23829 => Self::OtherBuildingEquipmentContractors,
            23831 => Self::DrywallAndInsulationContractors,
            23832 => Self::PaintingAndWallCoveringContractors,
            23833 => Self::FlooringContractors,
            23834 => Self::TileAndTerrazzoContractors,
            23835 => Self::FinishCarpentryContractors,
            23839 => Self::OtherBuildingFinishingContractors,
            23891 => Self::SitePreparationContractors,
            23899 => Self::AllOtherSpecialtyTradeContractors,
            31111 => Self::AnimalFoodManufacturing,
            31121 => Self::FlourMillingAndMaltManufacturing,
            31122 => Self::StarchAndVegetableFatsAndOilsManufacturing,
            31123 => Self::BreakfastCerealManufacturing,
            31131 => Self::SugarManufacturing,
            31134 => Self::NonchocolateConfectioneryManufacturing,
            31135 => Self::ChocolateAndConfectioneryManufacturing,
            31141 => Self::FrozenFoodManufacturing,
            31142 => Self::FruitAndVegetableCanningPicklingAndDrying,
            31151 => Self::DairyProductExceptFrozenManufacturing,
            31152 => Self::IceCreamAndFrozenDessertManufacturing,
            31161 => Self::AnimalSlaughteringAndProcessing,
            31171 => Self::SeafoodProductPreparationAndPackaging,
            31181 => Self::BreadAndBakeryProductManufacturing,
            31182 => Self::CookieCrackerAndPastaManufacturing,
            31183 => Self::TortillaManufacturing,
            31191 => Self::SnackFoodManufacturing,
            31192 => Self::CoffeeAndTeaManufacturing,
            31193 => Self::FlavoringSyrupAndConcentrateManufacturing,
            31194 => Self::SeasoningAndDressingManufacturing,
            31199 => Self::AllOtherFoodManufacturing,
            31211 => Self::SoftDrinkAndIceManufacturing,
            31212 => Self::Breweries,
            31213 => Self::Wineries,
            31214 => Self::Distilleries,
            31223 => Self::TobaccoManufacturing,
            31311 => Self::FiberYarnAndThreadMills,
            31321 => Self::BroadwovenFabricMills,
            31322 => Self::NarrowFabricMillsAndSchiffliMachineEmbroidery,
            31323 => Self::NonwovenFabricMills,
            31324 => Self::KnitFabricMills,
            31331 => Self::TextileAndFabricFinishingMills,
            31332 => Self::FabricCoatingMills,
            31411 => Self::CarpetAndRugMills,
            31412 => Self::CurtainAndLinenMills,
            31491 => Self::TextileBagAndCanvasMills,
            31499 => Self::AllOtherTextileProductMills,
            31512 => Self::ApparelKnittingMills,
            31521 => Self::CutAndSewApparelContractors,
            31525 => Self::CutAndSewApparelManufacturingExceptContractors,
            31522 => Self::MensAndBoysCutAndSewApparelManufacturing,
            31523 => Self::WomensAndGirlsCutAndSewApparelManufacturing,
            31529 => Self::OtherCutAndSewApparelManufacturing,
            31599 => Self::ApparelAccessoriesAndOtherApparelManufacturing,
            31611 => Self::LeatherAndHideTanningAndFinishing,
            31621 => Self::FootwearManufacturing,
            31699 => Self::OtherLeatherAndAlliedProductManufacturing,
            32111 => Self::SawmillsAndWoodPreservation,
            32121 => Self::VeneerPlywoodAndEngineeredWoodProductManufacturing,
            32191 => Self::Millwork,
            32192 => Self::WoodContainerAndPalletManufacturing,
            32199 => Self::AllOtherWoodProductManufacturing,
            32211 => Self::PulpMills,
            32212 => Self::PaperMills,
            32213 => Self::PaperboardMills,
            32221 => Self::PaperboardContainerManufacturing,
            32222 => Self::PaperBagAndCoatedAndTreatedPaperManufacturing,
            32223 => Self::StationeryProductManufacturing,
            32229 => Self::OtherConvertedPaperProductManufacturing,
            32311 => Self::Printing,
            32312 => Self::SupportActivitiesForPrinting,
            32411 => Self::PetroleumRefineries,
            32412 => Self::AsphaltPavingRoofingAndSaturatedMaterialsManufacturing,
            32419 => Self::OtherPetroleumAndCoalProductsManufacturing,
            32511 => Self::PetrochemicalManufacturing,
            32512 => Self::IndustrialGasManufacturing,
            32513 => Self::SyntheticDyeAndPigmentManufacturing,
            32518 => Self::OtherBasicInorganicChemicalManufacturing,
            32519 => Self::OtherBasicOrganicChemicalManufacturing,
            32521 => Self::ResinAndSyntheticRubberManufacturing,
            32522 => Self::ArtificialAndSyntheticFibersAndFilamentsManufacturing,
            32531 => Self::FertilizerAndCompostManufacturing,
            32532 => Self::PesticideAndOtherAgriculturalChemicalManufacturing,
            32541 => Self::PharmaceuticalAndMedicineManufacturing,
            32551 => Self::PaintAndCoatingManufacturing,
            32552 => Self::AdhesiveManufacturing,
            32561 => Self::SoapAndCleaningCompoundManufacturing,
            32562 => Self::ToiletPreparationManufacturing,
            32591 => Self::PrintingInkManufacturing,
            32592 => Self::ExplosivesManufacturing,
            32599 => Self::AllOtherChemicalProductAndPreparationManufacturing,
            32611 => Self::PlasticsPackagingMaterialsAndUnlaminatedFilmAndSheetManufacturing,
            32612 => Self::PlasticsPipePipeFittingAndUnlaminatedProfileShapeManufacturing,
            32613 => Self::LaminatedPlasticsPlateSheetExceptPackagingAndShapeManufacturing,
            32614 => Self::PolystyreneFoamProductManufacturing,
            32615 => Self::UrethaneAndOtherFoamProductExceptPolystyreneManufacturing,
            32616 => Self::PlasticsBottleManufacturing,
            32619 => Self::OtherPlasticsProductManufacturing,
            32621 => Self::TireManufacturing,
            32622 => Self::RubberAndPlasticsHosesAndBeltingManufacturing,
            32629 => Self::OtherRubberProductManufacturing,
            32711 => Self::PotteryCeramicsAndPlumbingFixtureManufacturing,
            32712 => Self::ClayBuildingMaterialAndRefractoriesManufacturing,
            32721 => Self::GlassAndGlassProductManufacturing,
            32731 => Self::CementManufacturing,
            32732 => Self::ReadymixConcreteManufacturing,
            32733 => Self::ConcretePipeBrickAndBlockManufacturing,
            32739 => Self::OtherConcreteProductManufacturing,
            32741 => Self::LimeManufacturing,
            32742 => Self::GypsumProductManufacturing,
            32791 => Self::AbrasiveProductManufacturing,
            32799 => Self::AllOtherNonmetallicMineralProductManufacturing,
            33111 => Self::IronAndSteelMillsAndFerroalloyManufacturing,
            33121 => Self::IronAndSteelPipeAndTubeManufacturingFromPurchasedSteel,
            33122 => Self::RollingAndDrawingOfPurchasedSteel,
            33131 => Self::AluminaAndAluminumProductionAndProcessing,
            33141 => Self::NonferrousMetalExceptAluminumSmeltingAndRefining,
            33142 => Self::CopperRollingDrawingExtrudingAndAlloying,
            33149 => Self::NonferrousMetalExceptCopperAndAluminumRollingDrawingExtrudingAndAlloying,
            33151 => Self::FerrousMetalFoundries,
            33152 => Self::NonferrousMetalFoundries,
            33211 => Self::ForgingAndStamping,
            33221 => Self::CutleryAndHandtoolManufacturing,
            33231 => Self::PlateWorkAndFabricatedStructuralProductManufacturing,
            33232 => Self::OrnamentalAndArchitecturalMetalProductsManufacturing,
            33241 => Self::PowerBoilerAndHeatExchangerManufacturing,
            33242 => Self::MetalTankHeavyGaugeManufacturing,
            33243 => Self::MetalCanBoxAndOtherMetalContainerLightGaugeManufacturing,
            33251 => Self::HardwareManufacturing,
            33261 => Self::SpringAndWireProductManufacturing,
            33271 => Self::MachineShops,
            33272 => Self::TurnedProductAndScrewNutAndBoltManufacturing,
            33281 => Self::CoatingEngravingHeatTreatingAndAlliedActivities,
            33291 => Self::MetalValveManufacturing,
            33299 => Self::AllOtherFabricatedMetalProductManufacturing,
            33311 => Self::AgriculturalImplementManufacturing,
            33312 => Self::ConstructionMachineryManufacturing,
            33313 => Self::MiningAndOilAndGasFieldMachineryManufacturing,
            33324 => Self::IndustrialMachineryManufacturing,
            33331 => Self::CommercialAndServiceIndustryMachineryManufacturing,
            33341 => Self::VentilationHeatingAirconditioningAndCommercialRefrigerationEquipmentManufacturing,
            33351 => Self::MetalworkingMachineryManufacturing,
            33361 => Self::EngineTurbineAndPowerTransmissionEquipmentManufacturing,
            33391 => Self::PumpAndCompressorManufacturing,
            33392 => Self::MaterialHandlingEquipmentManufacturing,
            33399 => Self::AllOtherGeneralPurposeMachineryManufacturing,
            33411 => Self::ComputerAndPeripheralEquipmentManufacturing,
            33421 => Self::TelephoneApparatusManufacturing,
            33422 => Self::RadioAndTelevisionBroadcastingAndWirelessCommunicationsEquipmentManufacturing,
            33429 => Self::OtherCommunicationsEquipmentManufacturing,
            33431 => Self::AudioAndVideoEquipmentManufacturing,
            33441 => Self::SemiconductorAndOtherElectronicComponentManufacturing,
            33451 => Self::NavigationalMeasuringElectromedicalAndControlInstrumentsManufacturing,
            33461 => Self::ManufacturingAndReproducingMagneticAndOpticalMedia,
            33513 => Self::ElectricLightingEquipmentManufacturing,
            33521 => Self::SmallElectricalApplianceManufacturing,
            33522 => Self::MajorHouseholdApplianceManufacturing,
            33531 => Self::ElectricalEquipmentManufacturing,
            33591 => Self::BatteryManufacturing,
            33592 => Self::CommunicationAndEnergyWireAndCableManufacturing,
            33593 => Self::WiringDeviceManufacturing,
            33599 => Self::AllOtherElectricalEquipmentAndComponentManufacturing,
            33611 => Self::AutomobileAndLightDutyMotorVehicleManufacturing,
            33612 => Self::HeavyDutyTruckManufacturing,
            33621 => Self::MotorVehicleBodyAndTrailerManufacturing,
            33631 => Self::MotorVehicleGasolineEngineAndEnginePartsManufacturing,
            33632 => Self::MotorVehicleElectricalAndElectronicEquipmentManufacturing,
            33633 => Self::MotorVehicleSteeringAndSuspensionComponentsExceptSpringManufacturing,
            33634 => Self::MotorVehicleBrakeSystemManufacturing,
            33635 => Self::MotorVehicleTransmissionAndPowerTrainPartsManufacturing,
            33636 => Self::MotorVehicleSeatingAndInteriorTrimManufacturing,
            33637 => Self::MotorVehicleMetalStamping,
            33639 => Self::OtherMotorVehiclePartsManufacturing,
            33641 => Self::AerospaceProductAndPartsManufacturing,
            33651 => Self::RailroadRollingStockManufacturing,
            33661 => Self::ShipAndBoatBuilding,
            33699 => Self::OtherTransportationEquipmentManufacturing,
            33711 => Self::WoodKitchenCabinetAndCountertopManufacturing,
            33712 => Self::HouseholdAndInstitutionalFurnitureManufacturing,
            33721 => Self::OfficeFurnitureIncludingFixturesManufacturing,
            33791 => Self::MattressManufacturing,
            33792 => Self::BlindAndShadeManufacturing,
            33911 => Self::MedicalEquipmentAndSuppliesManufacturing,
            33991 => Self::JewelryAndSilverwareManufacturing,
            33992 => Self::SportingAndAthleticGoodsManufacturing,
            33993 => Self::DollToyAndGameManufacturing,
            33994 => Self::OfficeSuppliesExceptPaperManufacturing,
            33995 => Self::SignManufacturing,
            33999 => Self::AllOtherMiscellaneousManufacturing,
            42311 => Self::AutomobileAndOtherMotorVehicleMerchantWholesalers,
            42312 => Self::MotorVehicleSuppliesAndNewPartsMerchantWholesalers,
            42313 => Self::TireAndTubeMerchantWholesalers,
            42314 => Self::MotorVehiclePartsUsedMerchantWholesalers,
            42321 => Self::FurnitureMerchantWholesalers,
            42322 => Self::HomeFurnishingMerchantWholesalers,
            42331 => Self::LumberPlywoodMillworkAndWoodPanelMerchantWholesalers,
            42332 => Self::BrickStoneAndRelatedConstructionMaterialMerchantWholesalers,
            42333 => Self::RoofingSidingAndInsulationMaterialMerchantWholesalers,
            42339 => Self::OtherConstructionMaterialMerchantWholesalers,
            42341 => Self::PhotographicEquipmentAndSuppliesMerchantWholesalers,
            42342 => Self::OfficeEquipmentMerchantWholesalers,
            42343 => Self::ComputerAndComputerPeripheralEquipmentAndSoftwareMerchantWholesalers,
            42344 => Self::OtherCommercialEquipmentMerchantWholesalers,
            42345 => Self::MedicalDentalAndHospitalEquipmentAndSuppliesMerchantWholesalers,
            42346 => Self::OphthalmicGoodsMerchantWholesalers,
            42349 => Self::OtherProfessionalEquipmentAndSuppliesMerchantWholesalers,
            42351 => Self::MetalServiceCentersAndOtherMetalMerchantWholesalers,
            42352 => Self::CoalAndOtherMineralAndOreMerchantWholesalers,
            42361 => Self::ElectricalApparatusAndEquipmentWiringSuppliesAndRelatedEquipmentMerchantWholesalers,
            42362 => Self::HouseholdAppliancesElectricHousewaresAndConsumerElectronicsMerchantWholesalers,
            42369 => Self::OtherElectronicPartsAndEquipmentMerchantWholesalers,
            42371 => Self::HardwareMerchantWholesalers,
            42372 => Self::PlumbingAndHeatingEquipmentAndSuppliesHydronicsMerchantWholesalers,
            42373 => Self::WarmAirHeatingAndAirconditioningEquipmentAndSuppliesMerchantWholesalers,
            42374 => Self::RefrigerationEquipmentAndSuppliesMerchantWholesalers,
            42381 => Self::ConstructionAndMiningExceptOilWellMachineryAndEquipmentMerchantWholesalers,
            42382 => Self::FarmAndGardenMachineryAndEquipmentMerchantWholesalers,
            42383 => Self::IndustrialMachineryAndEquipmentMerchantWholesalers,
            42384 => Self::IndustrialSuppliesMerchantWholesalers,
            42385 => Self::ServiceEstablishmentEquipmentAndSuppliesMerchantWholesalers,
            42386 => Self::TransportationEquipmentAndSuppliesExceptMotorVehicleMerchantWholesalers,
            42391 => Self::SportingAndRecreationalGoodsAndSuppliesMerchantWholesalers,
            42392 => Self::ToyAndHobbyGoodsAndSuppliesMerchantWholesalers,
            42393 => Self::RecyclableMaterialMerchantWholesalers,
            42394 => Self::JewelryWatchPreciousStoneAndPreciousMetalMerchantWholesalers,
            42399 => Self::OtherMiscellaneousDurableGoodsMerchantWholesalers,
            42411 => Self::PrintingAndWritingPaperMerchantWholesalers,
            42412 => Self::StationeryAndOfficeSuppliesMerchantWholesalers,
            42413 => Self::IndustrialAndPersonalServicePaperMerchantWholesalers,
            42421 => Self::DrugsAndDruggistsSundriesMerchantWholesalers,
            42431 => Self::PieceGoodsNotionsAndOtherDryGoodsMerchantWholesalers,
            42434 => Self::FootwearMerchantWholesalers,
            42435 => Self::ClothingAndClothingAccessoriesMerchantWholesalers,
            42441 => Self::GeneralLineGroceryMerchantWholesalers,
            42442 => Self::PackagedFrozenFoodMerchantWholesalers,
            42443 => Self::DairyProductExceptDriedOrCannedMerchantWholesalers,
            42444 => Self::PoultryAndPoultryProductMerchantWholesalers,
            42445 => Self::ConfectioneryMerchantWholesalers,
            42446 => Self::FishAndSeafoodMerchantWholesalers,
            42447 => Self::MeatAndMeatProductMerchantWholesalers,
            42448 => Self::FreshFruitAndVegetableMerchantWholesalers,
            42449 => Self::OtherGroceryAndRelatedProductsMerchantWholesalers,
            42451 => Self::GrainAndFieldBeanMerchantWholesalers,
            42452 => Self::LivestockMerchantWholesalers,
            42459 => Self::OtherFarmProductRawMaterialMerchantWholesalers,
            42461 => Self::PlasticsMaterialsAndBasicFormsAndShapesMerchantWholesalers,
            42469 => Self::OtherChemicalAndAlliedProductsMerchantWholesalers,
            42471 => Self::PetroleumBulkStationsAndTerminals,
            42472 => Self::PetroleumAndPetroleumProductsMerchantWholesalersExceptBulkStationsAndTerminals,
            42481 => Self::BeerAndAleMerchantWholesalers,
            42482 => Self::WineAndDistilledAlcoholicBeverageMerchantWholesalers,
            42491 => Self::FarmSuppliesMerchantWholesalers,
            42492 => Self::BookPeriodicalAndNewspaperMerchantWholesalers,
            42493 => Self::FlowerNurseryStockAndFloristsSuppliesMerchantWholesalers,
            42494 => Self::TobaccoProductAndElectronicCigaretteMerchantWholesalers,
            42495 => Self::PaintVarnishAndSuppliesMerchantWholesalers,
            42499 => Self::OtherMiscellaneousNondurableGoodsMerchantWholesalers,
            42512 => Self::WholesaleTradeAgentsAndBrokers,
            44111 => Self::NewCarDealers,
            44112 => Self::UsedCarDealers,
            44121 => Self::RecreationalVehicleDealers,
            44122 => Self::MotorcycleBoatAndOtherMotorVehicleDealers,
            44133 => Self::AutomotivePartsAndAccessoriesRetailers,
            44134 => Self::TireDealers,
            44411 => Self::HomeCenters,
            44412 => Self::PaintAndWallpaperRetailers,
            44414 => Self::HardwareRetailers,
            44418 => Self::OtherBuildingMaterialDealers,
            44423 => Self::OutdoorPowerEquipmentRetailers,
            44424 => Self::NurseryGardenCenterAndFarmSupplyRetailers,
            44511 => Self::SupermarketsAndOtherGroceryRetailersExceptConvenienceRetailers,
            44513 => Self::ConvenienceRetailersAndVendingMachineOperators,
            44523 => Self::FruitAndVegetableRetailers,
            44524 => Self::MeatRetailers,
            44525 => Self::FishAndSeafoodRetailers,
            44529 => Self::OtherSpecialtyFoodRetailers,
            44532 => Self::BeerWineAndLiquorRetailers,
            44911 => Self::FurnitureRetailers,
            44912 => Self::HomeFurnishingsRetailers,
            44921 => Self::ElectronicsAndApplianceRetailers,
            45511 => Self::DepartmentStores,
            45521 => Self::WarehouseClubsAndSupercenters,
            45522 => Self::AllOtherGeneralMerchandiseRetailers,
            45611 => Self::PharmaciesAndDrugRetailers,
            45612 => Self::CosmeticsBeautySuppliesAndPerfumeRetailers,
            45613 => Self::OpticalGoodsRetailers,
            45619 => Self::OtherHealthAndPersonalCareRetailers,
            45711 => Self::GasolineStationsWithConvenienceStores,
            45712 => Self::OtherGasolineStations,
            45721 => Self::FuelDealers,
            45811 => Self::ClothingRetailers,
            45812 => Self::ClothingAccessoriesRetailers,
            45821 => Self::ShoeRetailers,
            45831 => Self::JewelryRetailers,
            45832 => Self::LuggageAndLeatherGoodsRetailers,
            45911 => Self::SportingGoodsRetailers,
            45912 => Self::HobbyToyAndGameRetailers,
            45913 => Self::SewingNeedleworkAndPieceGoodsRetailers,
            45914 => Self::MusicalInstrumentAndSuppliesRetailers,
            45921 => Self::BookRetailersAndNewsDealers,
            45931 => Self::Florists,
            45941 => Self::OfficeSuppliesAndStationeryRetailers,
            45942 => Self::GiftNoveltyAndSouvenirRetailers,
            45951 => Self::UsedMerchandiseRetailers,
            45991 => Self::PetAndPetSuppliesRetailers,
            45992 => Self::ArtDealers,
            45993 => Self::ManufacturedMobileHomeDealers,
            45999 => Self::AllOtherMiscellaneousRetailers,
            48111 => Self::ScheduledAirTransportation,
            48121 => Self::NonscheduledAirTransportation,
            48211 => Self::RailTransportation,
            48311 => Self::DeepSeaCoastalAndGreatLakesWaterTransportation,
            48321 => Self::InlandWaterTransportation,
            48411 => Self::GeneralFreightTruckingLocal,
            48412 => Self::GeneralFreightTruckingLongdistance,
            48421 => Self::UsedHouseholdAndOfficeGoodsMoving,
            48422 => Self::SpecializedFreightExceptUsedGoodsTruckingLocal,
            48423 => Self::SpecializedFreightExceptUsedGoodsTruckingLongdistance,
            48511 => Self::UrbanTransitSystems,
            48521 => Self::InterurbanAndRuralBusTransportation,
            48531 => Self::TaxiService,
            48532 => Self::LimousineService,
            48541 => Self::SchoolAndEmployeeBusTransportation,
            48551 => Self::CharterBusIndustry,
            48599 => Self::OtherTransitAndGroundPassengerTransportation,
            48611 => Self::PipelineTransportationOfCrudeOil,
            48621 => Self::PipelineTransportationOfNaturalGas,
            48691 => Self::PipelineTransportationOfRefinedPetroleumProducts,
            48699 => Self::AllOtherPipelineTransportation,
            48711 => Self::ScenicAndSightseeingTransportationLand,
            48721 => Self::ScenicAndSightseeingTransportationWater,
            48799 => Self::ScenicAndSightseeingTransportationOther,
            48811 => Self::AirportOperations,
            48819 => Self::OtherSupportActivitiesForAirTransportation,
            48821 => Self::SupportActivitiesForRailTransportation,
            48831 => Self::PortAndHarborOperations,
            48832 => Self::MarineCargoHandling,
            48833 => Self::NavigationalServicesToShipping,
            48839 => Self::OtherSupportActivitiesForWaterTransportation,
            48841 => Self::MotorVehicleTowing,
            48849 => Self::OtherSupportActivitiesForRoadTransportation,
            48851 => Self::FreightTransportationArrangement,
            48899 => Self::OtherSupportActivitiesForTransportation,
            49111 => Self::PostalService,
            49211 => Self::CouriersAndExpressDeliveryServices,
            49221 => Self::LocalMessengersAndLocalDelivery,
            49311 => Self::GeneralWarehousingAndStorage,
            49312 => Self::RefrigeratedWarehousingAndStorage,
            49313 => Self::FarmProductWarehousingAndStorage,
            49319 => Self::OtherWarehousingAndStorage,
            51111 => Self::NewspaperPublishers,
            51112 => Self::PeriodicalPublishers,
            51113 => Self::BookPublishers,
            51114 => Self::DirectoryAndMailingListPublishers,
            51119 => Self::OtherPublishers,
            51121 => Self::SoftwarePublishers,
            51211 => Self::MotionPictureAndVideoProduction,
            51212 => Self::MotionPictureAndVideoDistribution,
            51213 => Self::MotionPictureAndVideoExhibition,
            51219 => Self::PostproductionServicesAndOtherMotionPictureAndVideoIndustries,
            51223 => Self::MusicPublishers,
            51224 => Self::SoundRecordingStudios,
            51225 => Self::RecordProductionAndDistribution,
            51229 => Self::OtherSoundRecordingIndustries,
            51511 => Self::RadioBroadcasting,
            51512 => Self::TelevisionBroadcasting,
            51521 => Self::CableAndOtherSubscriptionProgramming,
            51731 => Self::WiredAndWirelessTelecommunicationsCarriers,
            51741 => Self::SatelliteTelecommunications,
            51791 => Self::OtherTelecommunications,
            51821 => Self::ComputingInfrastructureProvidersDataProcessingWebHostingAndRelatedServices,
            51911 => Self::NewsSyndicates,
            51912 => Self::LibrariesAndArchives,
            51913 => Self::InternetPublishingAndBroadcastingAndWebSearchPortals,
            51919 => Self::AllOtherInformationServices,
            52111 => Self::MonetaryAuthoritiesCentralBank,
            52211 => Self::CommercialBanking,
            52212 => Self::SavingsInstitutions,
            52213 => Self::CreditUnions,
            52219 => Self::OtherDepositoryCreditIntermediation,
            52221 => Self::CreditCardIssuing,
            52222 => Self::SalesFinancing,
            52229 => Self::OtherNondepositoryCreditIntermediation,
            52231 => Self::MortgageAndNonmortgageLoanBrokers,
            52232 => Self::FinancialTransactionsProcessingReserveAndClearinghouseActivities,
            52239 => Self::OtherActivitiesRelatedToCreditIntermediation,
            52311 => Self::InvestmentBankingAndSecuritiesDealing,
            52312 => Self::SecuritiesBrokerage,
            52313 => Self::CommodityContractsDealing,
            52314 => Self::CommodityContractsBrokerage,
            52321 => Self::SecuritiesAndCommodityExchanges,
            52391 => Self::MiscellaneousIntermediation,
            52392 => Self::PortfolioManagement,
            52393 => Self::InvestmentAdvice,
            52399 => Self::AllOtherFinancialInvestmentActivities,
            52411 => Self::DirectLifeHealthAndMedicalInsuranceCarriers,
            52412 => Self::DirectInsuranceExceptLifeHealthAndMedicalCarriers,
            52413 => Self::ReinsuranceCarriers,
            52421 => Self::InsuranceAgenciesAndBrokerages,
            52429 => Self::OtherInsuranceRelatedActivities,
            52511 => Self::PensionFunds,
            52512 => Self::HealthAndWelfareFunds,
            52519 => Self::OtherInsuranceFunds,
            52591 => Self::OpenEndInvestmentFunds,
            52592 => Self::TrustsEstatesAndAgencyAccounts,
            52593 => Self::RealEstateInvestmentTrusts,
            52599 => Self::OtherFinancialVehicles,
            53111 => Self::LessorsOfResidentialBuildingsAndDwellings,
            53112 => Self::LessorsOfNonresidentialBuildingsExceptMiniwarehouses,
            53113 => Self::LessorsOfMiniwarehousesAndSelfStorageUnits,
            53119 => Self::LessorsOfOtherRealEstateProperty,
            53121 => Self::OfficesOfRealEstateAgentsAndBrokers,
            53131 => Self::RealEstatePropertyManagers,
            53132 => Self::OfficesOfRealEstateAppraisers,
            53139 => Self::OtherActivitiesRelatedToRealEstate,
            53211 => Self::PassengerCarRentalAndLeasing,
            53212 => Self::TruckUtilityTrailerAndRvRecreationalVehicleRentalAndLeasing,
            53221 => Self::ConsumerElectronicsAndAppliancesRental,
            53222 => Self::FormalWearAndCostumeRental,
            53223 => Self::VideoTapeAndDiscRental,
            53229 => Self::OtherConsumerGoodsRental,
            53231 => Self::GeneralRentalCenters,
            53241 => Self::ConstructionTransportationMiningAndForestryMachineryAndEquipmentRentalAndLeasing,
            53242 => Self::OfficeMachineryAndEquipmentRentalAndLeasing,
            53249 => Self::OtherCommercialAndIndustrialMachineryAndEquipmentRentalAndLeasing,
            53311 => Self::LessorsOfNonfinancialIntangibleAssetsExceptCopyrightedWorks,
            54111 => Self::OfficesOfLawyers,
            54112 => Self::OfficesOfNotaries,
            54119 => Self::OtherLegalServices,
            54121 => Self::AccountingTaxPreparationBookkeepingAndPayrollServices,
            54131 => Self::ArchitecturalServices,
            54132 => Self::LandscapeArchitecturalServices,
            54133 => Self::EngineeringServices,
            54134 => Self::DraftingServices,
            54135 => Self::BuildingInspectionServices,
            54136 => Self::GeophysicalSurveyingAndMappingServices,
            54137 => Self::SurveyingAndMappingExceptGeophysicalServices,
            54138 => Self::TestingLaboratories,
            54141 => Self::InteriorDesignServices,
            54142 => Self::IndustrialDesignServices,
            54143 => Self::GraphicDesignServices,
            54149 => Self::OtherSpecializedDesignServices,
            54151 => Self::ComputerSystemsDesignAndRelatedServices,
            54161 => Self::ManagementConsultingServices,
            54162 => Self::EnvironmentalConsultingServices,
            54169 => Self::OtherScientificAndTechnicalConsultingServices,
            54171 => Self::ResearchAndDevelopmentInThePhysicalEngineeringAndLifeSciences,
            54172 => Self::ResearchAndDevelopmentInTheSocialSciencesAndHumanities,
            54181 => Self::AdvertisingAgencies,
            54182 => Self::PublicRelationsAgencies,
            54183 => Self::MediaBuyingAgencies,
            54184 => Self::MediaRepresentatives,
            54185 => Self::OutdoorAdvertising,
            54186 => Self::DirectMailAdvertising,
            54187 => Self::AdvertisingMaterialDistributionServices,
            54189 => Self::OtherServicesRelatedToAdvertising,
            54191 => Self::MarketingResearchAndPublicOpinionPolling,
            54192 => Self::PhotographicServices,
            54193 => Self::TranslationAndInterpretationServices,
            54194 => Self::VeterinaryServices,
            54199 => Self::AllOtherProfessionalScientificAndTechnicalServices,
            55111 => Self::ManagementOfCompaniesAndEnterprises,
            56111 => Self::OfficeAdministrativeServices,
            56121 => Self::FacilitiesSupportServices,
            56131 => Self::EmploymentPlacementAgenciesAndExecutiveSearchServices,
            56132 => Self::TemporaryHelpServices,
            56133 => Self::ProfessionalEmployerOrganizations,
            56141 => Self::DocumentPreparationServices,
            56142 => Self::TelephoneCallCenters,
            56143 => Self::BusinessServiceCenters,
            56144 => Self::CollectionAgencies,
            56145 => Self::CreditBureaus,
            56149 => Self::OtherBusinessSupportServices,
            56151 => Self::TravelAgencies,
            56152 => Self::TourOperators,
            56159 => Self::OtherTravelArrangementAndReservationServices,
            56161 => Self::InvestigationGuardAndArmoredCarServices,
            56162 => Self::SecuritySystemsServices,
            56171 => Self::ExterminatingAndPestControlServices,
            56172 => Self::JanitorialServices,
            56173 => Self::LandscapingServices,
            56174 => Self::CarpetAndUpholsteryCleaningServices,
            56179 => Self::OtherServicesToBuildingsAndDwellings,
            56191 => Self::PackagingAndLabelingServices,
            56192 => Self::ConventionAndTradeShowOrganizers,
            56199 => Self::AllOtherSupportServices,
            56211 => Self::WasteCollection,
            56221 => Self::WasteTreatmentAndDisposal,
            56291 => Self::RemediationServices,
            56292 => Self::MaterialsRecoveryFacilities,
            56299 => Self::AllOtherWasteManagementServices,
            61111 => Self::ElementaryAndSecondarySchools,
            61121 => Self::JuniorColleges,
            61131 => Self::CollegesUniversitiesAndProfessionalSchools,
            61141 => Self::BusinessAndSecretarialSchools,
            61142 => Self::ComputerTraining,
            61143 => Self::ProfessionalAndManagementDevelopmentTraining,
            61151 => Self::TechnicalAndTradeSchools,
            61161 => Self::FineArtsSchools,
            61162 => Self::SportsAndRecreationInstruction,
            61163 => Self::LanguageSchools,
            61169 => Self::AllOtherSchoolsAndInstruction,
            61171 => Self::EducationalSupportServices,
            62111 => Self::OfficesOfPhysicians,
            62121 => Self::OfficesOfDentists,
            62131 => Self::OfficesOfChiropractors,
            62132 => Self::OfficesOfOptometrists,
            62133 => Self::OfficesOfMentalHealthPractitionersExceptPhysicians,
            62134 => Self::OfficesOfPhysicalOccupationalAndSpeechTherapistsAndAudiologists,
            62139 => Self::OfficesOfAllOtherHealthPractitioners,
            62141 => Self::FamilyPlanningCenters,
            62142 => Self::OutpatientMentalHealthAndSubstanceAbuseCenters,
            62149 => Self::OtherOutpatientCareCenters,
            62151 => Self::MedicalAndDiagnosticLaboratories,
            62161 => Self::HomeHealthCareServices,
            62191 => Self::AmbulanceServices,
            62199 => Self::AllOtherAmbulatoryHealthCareServices,
            62211 => Self::GeneralMedicalAndSurgicalHospitals,
            62221 => Self::PsychiatricAndSubstanceAbuseHospitals,
            62231 => Self::SpecialtyExceptPsychiatricAndSubstanceAbuseHospitals,
            62311 => Self::NursingCareFacilitiesSkilledNursingFacilities,
            62321 => Self::ResidentialIntellectualAndDevelopmentalDisabilityFacilities,
            62322 => Self::ResidentialMentalHealthAndSubstanceAbuseFacilities,
            62331 => Self::ContinuingCareRetirementCommunitiesAndAssistedLivingFacilitiesForTheElderly,
            62399 => Self::OtherResidentialCareFacilities,
            62411 => Self::ChildAndYouthServices,
            62412 => Self::ServicesForTheElderlyAndPersonsWithDisabilities,
            62419 => Self::OtherIndividualAndFamilyServices,
            62421 => Self::CommunityFoodServices,
            62422 => Self::CommunityHousingServices,
            62423 => Self::EmergencyAndOtherReliefServices,
            62431 => Self::VocationalRehabilitationServices,
            62441 => Self::ChildDayCareServices,
            71111 => Self::TheaterCompaniesAndDinnerTheaters,
            71112 => Self::DanceCompanies,
            71113 => Self::MusicalGroupsAndArtists,
            71119 => Self::OtherPerformingArtsCompanies,
            71121 => Self::SpectatorSports,
            71131 => Self::PromotersOfPerformingArtsSportsAndSimilarEventsWithFacilities,
            71132 => Self::PromotersOfPerformingArtsSportsAndSimilarEventsWithoutFacilities,
            71141 => Self::AgentsAndManagersForArtistsAthletesEntertainersAndOtherPublicFigures,
            71151 => Self::IndependentArtistsWritersAndPerformers,
            71211 => Self::Museums,
            71212 => Self::HistoricalSites,
            71213 => Self::ZoosAndBotanicalGardens,
            71219 => Self::NatureParksAndOtherSimilarInstitutions,
            71311 => Self::AmusementAndThemeParks,
            71312 => Self::AmusementArcades,
            71321 => Self::CasinosExceptCasinoHotels,
            71329 => Self::OtherGamblingIndustries,
            71391 => Self::GolfCoursesAndCountryClubs,
            71392 => Self::SkiingFacilities,
            71393 => Self::Marinas,
            71394 => Self::FitnessAndRecreationalSportsCenters,
            71395 => Self::BowlingCenters,
            71399 => Self::AllOtherAmusementAndRecreationIndustries,
            72111 => Self::HotelsExceptCasinoHotelsAndMotels,
            72112 => Self::CasinoHotels,
            72119 => Self::OtherTravelerAccommodation,
            72121 => Self::RvRecreationalVehicleParksAndRecreationalCamps,
            72131 => Self::RoomingAndBoardingHouses,
            72231 => Self::FoodServiceContractors,
            72232 => Self::Caterers,
            72233 => Self::MobileFoodServices,
            72241 => Self::DrinkingPlacesAlcoholicBeverages,
            72251 => Self::RestaurantsAndOtherEatingPlaces,
            81111 => Self::AutomotiveMechanicalAndElectricalRepairAndMaintenance,
            81112 => Self::AutomotiveBodyPaintInteriorAndGlassRepair,
            81119 => Self::OtherAutomotiveRepairAndMaintenance,
            81121 => Self::ElectronicAndPrecisionEquipmentRepairAndMaintenance,
            81131 => Self::CommercialAndIndustrialMachineryAndEquipmentExceptAutomotiveAndElectronicRepairAndMaintenance,
            81141 => Self::HomeAndGardenEquipmentAndApplianceRepairAndMaintenance,
            81142 => Self::ReupholsteryAndFurnitureRepair,
            81143 => Self::FootwearAndLeatherGoodsRepair,
            81149 => Self::OtherPersonalAndHouseholdGoodsRepairAndMaintenance,
            81211 => Self::HairNailAndSkinCareServices,
            81219 => Self::OtherPersonalCareServices,
            81221 => Self::FuneralHomesAndFuneralServices,
            81222 => Self::CemeteriesAndCrematories,
            81231 => Self::CoinOperatedLaundriesAndDrycleaners,
            81232 => Self::DrycleaningAndLaundryServicesExceptCoinOperated,
            81233 => Self::LinenAndUniformSupply,
            81291 => Self::PetCareExceptVeterinaryServices,
            81292 => Self::Photofinishing,
            81293 => Self::ParkingLotsAndGarages,
            81299 => Self::AllOtherPersonalServices,
            81311 => Self::ReligiousOrganizations,
            81321 => Self::GrantmakingAndGivingServices,
            81331 => Self::SocialAdvocacyOrganizations,
            81341 => Self::CivicAndSocialOrganizations,
            81391 => Self::BusinessAssociations,
            81392 => Self::ProfessionalOrganizations,
            81393 => Self::LaborUnionsAndSimilarLaborOrganizations,
            81394 => Self::PoliticalOrganizations,
            81399 => Self::OtherSimilarOrganizationsExceptBusinessProfessionalLaborAndPoliticalOrganizations,
            81411 => Self::PrivateHouseholds,
            92111 => Self::ExecutiveOffices,
            92112 => Self::LegislativeBodies,
            92113 => Self::PublicFinanceActivities,
            92114 => Self::ExecutiveAndLegislativeOfficesCombined,
            92115 => Self::AmericanIndianAndAlaskaNativeTribalGovernments,
            92119 => Self::OtherGeneralGovernmentSupport,
            92211 => Self::Courts,
            92212 => Self::PoliceProtection,
            92213 => Self::LegalCounselAndProsecution,
            92214 => Self::CorrectionalInstitutions,
            92215 => Self::ParoleOfficesAndProbationOffices,
            92216 => Self::FireProtection,
            92219 => Self::OtherJusticePublicOrderAndSafetyActivities,
            92311 => Self::AdministrationOfEducationPrograms,
            92312 => Self::AdministrationOfPublicHealthPrograms,
            92313 => Self::AdministrationOfHumanResourceProgramsExceptEducationPublicHealthAndVeteransAffairsPrograms,
            92314 => Self::AdministrationOfVeteransAffairs,
            92411 => Self::AdministrationOfAirAndWaterResourceAndSolidWasteManagementPrograms,
            92412 => Self::AdministrationOfConservationPrograms,
            92511 => Self::AdministrationOfHousingPrograms,
            92512 => Self::AdministrationOfUrbanPlanningAndCommunityAndRuralDevelopment,
            92611 => Self::AdministrationOfGeneralEconomicPrograms,
            92612 => Self::RegulationAndAdministrationOfTransportationPrograms,
            92613 => Self::RegulationAndAdministrationOfCommunicationsElectricGasAndOtherUtilities,
            92614 => Self::RegulationOfAgriculturalMarketingAndCommodities,
            92615 => Self::RegulationLicensingAndInspectionOfMiscellaneousCommercialSectors,
            92711 => Self::SpaceResearchAndTechnology,
            92811 => Self::NationalSecurity,
            92812 => Self::InternationalAffairs,
            99999 => Self::UnclassifiedEstablishments,
            _ => return None,
        };

        Some(result)
    }
}
