/// North American Industry Classification System (NAICS) industry codes
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
pub enum NaicsIndustry {
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
    /// Oilseed and Grain Combination Farming
    OilseedAndGrainCombinationFarming,
    /// All Other Grain Farming
    AllOtherGrainFarming,
    /// Potato Farming
    PotatoFarming,
    /// Other Vegetable (except Potato) and Melon Farming
    OtherVegetableExceptPotatoAndMelonFarming,
    /// Orange Groves
    OrangeGroves,
    /// Citrus (except Orange) Groves
    CitrusExceptOrangeGroves,
    /// Apple Orchards
    AppleOrchards,
    /// Grape Vineyards
    GrapeVineyards,
    /// Strawberry Farming
    StrawberryFarming,
    /// Berry (except Strawberry) Farming
    BerryExceptStrawberryFarming,
    /// Tree Nut Farming
    TreeNutFarming,
    /// Fruit and Tree Nut Combination Farming
    FruitAndTreeNutCombinationFarming,
    /// Other Noncitrus Fruit Farming
    OtherNoncitrusFruitFarming,
    /// Mushroom Production
    MushroomProduction,
    /// Other Food Crops Grown Under Cover
    OtherFoodCropsGrownUnderCover,
    /// Nursery and Tree Production
    NurseryAndTreeProduction,
    /// Floriculture Production
    FloricultureProduction,
    /// Tobacco Farming
    TobaccoFarming,
    /// Cotton Farming
    CottonFarming,
    /// Sugarcane Farming
    SugarcaneFarming,
    /// Hay Farming
    HayFarming,
    /// Sugar Beet Farming
    SugarBeetFarming,
    /// Peanut Farming
    PeanutFarming,
    /// All Other Miscellaneous Crop Farming
    AllOtherMiscellaneousCropFarming,
    /// Beef Cattle Ranching and Farming
    BeefCattleRanchingAndFarming,
    /// Cattle Feedlots
    CattleFeedlots,
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
    /// Finfish Farming and Fish Hatcheries
    FinfishFarmingAndFishHatcheries,
    /// Shellfish Farming
    ShellfishFarming,
    /// Other Aquaculture
    OtherAquaculture,
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
    /// Finfish Fishing
    FinfishFishing,
    /// Shellfish Fishing
    ShellfishFishing,
    /// Other Marine Fishing
    OtherMarineFishing,
    /// Hunting and Trapping
    HuntingAndTrapping,
    /// Cotton Ginning
    CottonGinning,
    /// Soil Preparation, Planting, and Cultivating
    SoilPreparationPlantingAndCultivating,
    /// Crop Harvesting, Primarily by Machine
    CropHarvestingPrimarilyByMachine,
    /// Postharvest Crop Activities (except Cotton Ginning)
    PostharvestCropActivitiesExceptCottonGinning,
    /// Farm Labor Contractors and Crew Leaders
    FarmLaborContractorsAndCrewLeaders,
    /// Farm Management Services
    FarmManagementServices,
    /// Support Activities for Animal Production
    SupportActivitiesForAnimalProduction,
    /// Support Activities for Forestry
    SupportActivitiesForForestry,
    /// Crude Petroleum Extraction
    CrudePetroleumExtraction,
    /// Natural Gas Extraction
    NaturalGasExtraction,
    /// Surface Coal Mining
    SurfaceCoalMining,
    /// Underground Coal Mining
    UndergroundCoalMining,
    /// Iron Ore Mining
    IronOreMining,
    /// Gold Ore and Silver Ore Mining
    GoldOreAndSilverOreMining,
    /// Copper, Nickel, Lead, and Zinc Mining
    CopperNickelLeadAndZincMining,
    /// Other Metal Ore Mining
    OtherMetalOreMining,
    /// Dimension Stone Mining and Quarrying
    DimensionStoneMiningAndQuarrying,
    /// Crushed and Broken Limestone Mining and Quarrying
    CrushedAndBrokenLimestoneMiningAndQuarrying,
    /// Crushed and Broken Granite Mining and Quarrying
    CrushedAndBrokenGraniteMiningAndQuarrying,
    /// Other Crushed and Broken Stone Mining and Quarrying
    OtherCrushedAndBrokenStoneMiningAndQuarrying,
    /// Construction Sand and Gravel Mining
    ConstructionSandAndGravelMining,
    /// Industrial Sand Mining
    IndustrialSandMining,
    /// Kaolin, Clay, and Ceramic and Refractory Minerals Mining
    KaolinClayAndCeramicAndRefractoryMineralsMining,
    /// Other Nonmetallic Mineral Mining and Quarrying
    OtherNonmetallicMineralMiningAndQuarrying,
    /// Drilling Oil and Gas Wells
    DrillingOilAndGasWells,
    /// Support Activities for Oil and Gas Operations
    SupportActivitiesForOilAndGasOperations,
    /// Support Activities for Coal Mining
    SupportActivitiesForCoalMining,
    /// Support Activities for Metal Mining
    SupportActivitiesForMetalMining,
    /// Support Activities for Nonmetallic Minerals (except Fuels) Mining
    SupportActivitiesForNonmetallicMineralsExceptFuelsMining,
    /// Hydroelectric Power Generation
    HydroelectricPowerGeneration,
    /// Fossil Fuel Electric Power Generation
    FossilFuelElectricPowerGeneration,
    /// Nuclear Electric Power Generation
    NuclearElectricPowerGeneration,
    /// Solar Electric Power Generation
    SolarElectricPowerGeneration,
    /// Wind Electric Power Generation
    WindElectricPowerGeneration,
    /// Geothermal Electric Power Generation
    GeothermalElectricPowerGeneration,
    /// Biomass Electric Power Generation
    BiomassElectricPowerGeneration,
    /// Other Electric Power Generation
    OtherElectricPowerGeneration,
    /// Electric Bulk Power Transmission and Control
    ElectricBulkPowerTransmissionAndControl,
    /// Electric Power Distribution
    ElectricPowerDistribution,
    /// Natural Gas Distribution
    NaturalGasDistribution,
    /// Water Supply and Irrigation Systems
    WaterSupplyAndIrrigationSystems,
    /// Sewage Treatment Facilities
    SewageTreatmentFacilities,
    /// Steam and Air-Conditioning Supply
    SteamAndAirconditioningSupply,
    /// New Single-Family Housing Construction (except For-Sale Builders)
    NewSinglefamilyHousingConstructionExceptForsaleBuilders,
    /// New Multifamily Housing Construction (except For-Sale Builders)
    NewMultifamilyHousingConstructionExceptForsaleBuilders,
    /// New Housing For-Sale Builders
    NewHousingForsaleBuilders,
    /// Residential Remodelers
    ResidentialRemodelers,
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
    /// Dog and Cat Food Manufacturing
    DogAndCatFoodManufacturing,
    /// Other Animal Food Manufacturing
    OtherAnimalFoodManufacturing,
    /// Flour Milling
    FlourMilling,
    /// Rice Milling
    RiceMilling,
    /// Malt Manufacturing
    MaltManufacturing,
    /// Wet Corn Milling and Starch Manufacturing
    WetCornMillingAndStarchManufacturing,
    /// Soybean and Other Oilseed Processing
    SoybeanAndOtherOilseedProcessing,
    /// Fats and Oils Refining and Blending
    FatsAndOilsRefiningAndBlending,
    /// Breakfast Cereal Manufacturing
    BreakfastCerealManufacturing,
    /// Beet Sugar Manufacturing
    BeetSugarManufacturing,
    /// Cane Sugar Manufacturing
    CaneSugarManufacturing,
    /// Nonchocolate Confectionery Manufacturing
    NonchocolateConfectioneryManufacturing,
    /// Chocolate and Confectionery Manufacturing from Cacao Beans
    ChocolateAndConfectioneryManufacturingFromCacaoBeans,
    /// Confectionery Manufacturing from Purchased Chocolate
    ConfectioneryManufacturingFromPurchasedChocolate,
    /// Frozen Fruit, Juice, and Vegetable Manufacturing
    FrozenFruitJuiceAndVegetableManufacturing,
    /// Frozen Specialty Food Manufacturing
    FrozenSpecialtyFoodManufacturing,
    /// Fruit and Vegetable Canning
    FruitAndVegetableCanning,
    /// Specialty Canning
    SpecialtyCanning,
    /// Dried and Dehydrated Food Manufacturing
    DriedAndDehydratedFoodManufacturing,
    /// Fluid Milk Manufacturing
    FluidMilkManufacturing,
    /// Creamery Butter Manufacturing
    CreameryButterManufacturing,
    /// Cheese Manufacturing
    CheeseManufacturing,
    /// Dry, Condensed, and Evaporated Dairy Product Manufacturing
    DryCondensedAndEvaporatedDairyProductManufacturing,
    /// Ice Cream and Frozen Dessert Manufacturing
    IceCreamAndFrozenDessertManufacturing,
    /// Animal (except Poultry) Slaughtering
    AnimalExceptPoultrySlaughtering,
    /// Meat Processed from Carcasses
    MeatProcessedFromCarcasses,
    /// Rendering and Meat Byproduct Processing
    RenderingAndMeatByproductProcessing,
    /// Poultry Processing
    PoultryProcessing,
    /// Seafood Product Preparation and Packaging
    SeafoodProductPreparationAndPackaging,
    /// Retail Bakeries
    RetailBakeries,
    /// Commercial Bakeries
    CommercialBakeries,
    /// Frozen Cakes, Pies, and Other Pastries Manufacturing
    FrozenCakesPiesAndOtherPastriesManufacturing,
    /// Cookie and Cracker Manufacturing
    CookieAndCrackerManufacturing,
    /// Dry Pasta, Dough, and Flour Mixes Manufacturing from Purchased Flour
    DryPastaDoughAndFlourMixesManufacturingFromPurchasedFlour,
    /// Tortilla Manufacturing
    TortillaManufacturing,
    /// Roasted Nuts and Peanut Butter Manufacturing
    RoastedNutsAndPeanutButterManufacturing,
    /// Other Snack Food Manufacturing
    OtherSnackFoodManufacturing,
    /// Coffee and Tea Manufacturing
    CoffeeAndTeaManufacturing,
    /// Flavoring Syrup and Concentrate Manufacturing
    FlavoringSyrupAndConcentrateManufacturing,
    /// Mayonnaise, Dressing, and Other Prepared Sauce Manufacturing
    MayonnaiseDressingAndOtherPreparedSauceManufacturing,
    /// Spice and Extract Manufacturing
    SpiceAndExtractManufacturing,
    /// Perishable Prepared Food Manufacturing
    PerishablePreparedFoodManufacturing,
    /// All Other Miscellaneous Food Manufacturing
    AllOtherMiscellaneousFoodManufacturing,
    /// Soft Drink Manufacturing
    SoftDrinkManufacturing,
    /// Bottled Water Manufacturing
    BottledWaterManufacturing,
    /// Ice Manufacturing
    IceManufacturing,
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
    /// Rope, Cordage, Twine, Tire Cord, and Tire Fabric Mills
    RopeCordageTwineTireCordAndTireFabricMills,
    /// All Other Miscellaneous Textile Product Mills
    AllOtherMiscellaneousTextileProductMills,
    /// Apparel Knitting Mills
    ApparelKnittingMills,
    /// Cut and Sew Apparel Contractors
    CutAndSewApparelContractors,
    /// Cut and Sew Apparel Manufacturing (except Contractors)
    CutAndSewApparelManufacturingExceptContractors,
    /// Apparel Accessories and Other Apparel Manufacturing
    ApparelAccessoriesAndOtherApparelManufacturing,
    /// Leather and Hide Tanning and Finishing
    LeatherAndHideTanningAndFinishing,
    /// Footwear Manufacturing
    FootwearManufacturing,
    /// Other Leather and Allied Product Manufacturing
    OtherLeatherAndAlliedProductManufacturing,
    /// Sawmills
    Sawmills,
    /// Wood Preservation
    WoodPreservation,
    /// Hardwood Veneer and Plywood Manufacturing
    HardwoodVeneerAndPlywoodManufacturing,
    /// Softwood Veneer and Plywood Manufacturing
    SoftwoodVeneerAndPlywoodManufacturing,
    /// Engineered Wood Member Manufacturing
    EngineeredWoodMemberManufacturing,
    /// Reconstituted Wood Product Manufacturing
    ReconstitutedWoodProductManufacturing,
    /// Wood Window and Door Manufacturing
    WoodWindowAndDoorManufacturing,
    /// Cut Stock, Resawing Lumber, and Planing
    CutStockResawingLumberAndPlaning,
    /// Other Millwork (including Flooring)
    OtherMillworkIncludingFlooring,
    /// Wood Container and Pallet Manufacturing
    WoodContainerAndPalletManufacturing,
    /// Manufactured Home (Mobile Home) Manufacturing
    ManufacturedHomeMobileHomeManufacturing,
    /// Prefabricated Wood Building Manufacturing
    PrefabricatedWoodBuildingManufacturing,
    /// All Other Miscellaneous Wood Product Manufacturing
    AllOtherMiscellaneousWoodProductManufacturing,
    /// Pulp Mills
    PulpMills,
    /// Paper Mills
    PaperMills,
    /// Paperboard Mills
    PaperboardMills,
    /// Corrugated and Solid Fiber Box Manufacturing
    CorrugatedAndSolidFiberBoxManufacturing,
    /// Folding Paperboard Box Manufacturing
    FoldingPaperboardBoxManufacturing,
    /// Other Paperboard Container Manufacturing
    OtherPaperboardContainerManufacturing,
    /// Paper Bag and Coated and Treated Paper Manufacturing
    PaperBagAndCoatedAndTreatedPaperManufacturing,
    /// Stationery Product Manufacturing
    StationeryProductManufacturing,
    /// Sanitary Paper Product Manufacturing
    SanitaryPaperProductManufacturing,
    /// All Other Converted Paper Product Manufacturing
    AllOtherConvertedPaperProductManufacturing,
    /// Commercial Printing (except Screen and Books)
    CommercialPrintingExceptScreenAndBooks,
    /// Commercial Screen Printing
    CommercialScreenPrinting,
    /// Books Printing
    BooksPrinting,
    /// Support Activities for Printing
    SupportActivitiesForPrinting,
    /// Petroleum Refineries
    PetroleumRefineries,
    /// Asphalt Paving Mixture and Block Manufacturing
    AsphaltPavingMixtureAndBlockManufacturing,
    /// Asphalt Shingle and Coating Materials Manufacturing
    AsphaltShingleAndCoatingMaterialsManufacturing,
    /// Petroleum Lubricating Oil and Grease Manufacturing
    PetroleumLubricatingOilAndGreaseManufacturing,
    /// All Other Petroleum and Coal Products Manufacturing
    AllOtherPetroleumAndCoalProductsManufacturing,
    /// Petrochemical Manufacturing
    PetrochemicalManufacturing,
    /// Industrial Gas Manufacturing
    IndustrialGasManufacturing,
    /// Synthetic Dye and Pigment Manufacturing
    SyntheticDyeAndPigmentManufacturing,
    /// Other Basic Inorganic Chemical Manufacturing
    OtherBasicInorganicChemicalManufacturing,
    /// Ethyl Alcohol Manufacturing
    EthylAlcoholManufacturing,
    /// Cyclic Crude, Intermediate, and Gum and Wood Chemical Manufacturing
    CyclicCrudeIntermediateAndGumAndWoodChemicalManufacturing,
    /// All Other Basic Organic Chemical Manufacturing
    AllOtherBasicOrganicChemicalManufacturing,
    /// Plastics Material and Resin Manufacturing
    PlasticsMaterialAndResinManufacturing,
    /// Synthetic Rubber Manufacturing
    SyntheticRubberManufacturing,
    /// Artificial and Synthetic Fibers and Filaments Manufacturing
    ArtificialAndSyntheticFibersAndFilamentsManufacturing,
    /// Nitrogenous Fertilizer Manufacturing
    NitrogenousFertilizerManufacturing,
    /// Phosphatic Fertilizer Manufacturing
    PhosphaticFertilizerManufacturing,
    /// Fertilizer (Mixing Only) Manufacturing
    FertilizerMixingOnlyManufacturing,
    /// Compost Manufacturing
    CompostManufacturing,
    /// Pesticide and Other Agricultural Chemical Manufacturing
    PesticideAndOtherAgriculturalChemicalManufacturing,
    /// Medicinal and Botanical Manufacturing
    MedicinalAndBotanicalManufacturing,
    /// Pharmaceutical Preparation Manufacturing
    PharmaceuticalPreparationManufacturing,
    /// In-Vitro Diagnostic Substance Manufacturing
    InvitroDiagnosticSubstanceManufacturing,
    /// Biological Product (except Diagnostic) Manufacturing
    BiologicalProductExceptDiagnosticManufacturing,
    /// Paint and Coating Manufacturing
    PaintAndCoatingManufacturing,
    /// Adhesive Manufacturing
    AdhesiveManufacturing,
    /// Soap and Other Detergent Manufacturing
    SoapAndOtherDetergentManufacturing,
    /// Polish and Other Sanitation Good Manufacturing
    PolishAndOtherSanitationGoodManufacturing,
    /// Surface Active Agent Manufacturing
    SurfaceActiveAgentManufacturing,
    /// Toilet Preparation Manufacturing
    ToiletPreparationManufacturing,
    /// Printing Ink Manufacturing
    PrintingInkManufacturing,
    /// Explosives Manufacturing
    ExplosivesManufacturing,
    /// Custom Compounding of Purchased Resins
    CustomCompoundingOfPurchasedResins,
    /// Photographic Film, Paper, Plate, Chemical, and Copy Toner Manufacturing
    PhotographicFilmPaperPlateChemicalAndCopyTonerManufacturing,
    /// All Other Miscellaneous Chemical Product and Preparation Manufacturing
    AllOtherMiscellaneousChemicalProductAndPreparationManufacturing,
    /// Plastics Bag and Pouch Manufacturing
    PlasticsBagAndPouchManufacturing,
    /// Plastics Packaging Film and Sheet (including Laminated) Manufacturing
    PlasticsPackagingFilmAndSheetIncludingLaminatedManufacturing,
    /// Unlaminated Plastics Film and Sheet (except Packaging) Manufacturing
    UnlaminatedPlasticsFilmAndSheetExceptPackagingManufacturing,
    /// Unlaminated Plastics Profile Shape Manufacturing
    UnlaminatedPlasticsProfileShapeManufacturing,
    /// Plastics Pipe and Pipe Fitting Manufacturing
    PlasticsPipeAndPipeFittingManufacturing,
    /// Laminated Plastics Plate, Sheet (except Packaging), and Shape Manufacturing
    LaminatedPlasticsPlateSheetExceptPackagingAndShapeManufacturing,
    /// Polystyrene Foam Product Manufacturing
    PolystyreneFoamProductManufacturing,
    /// Urethane and Other Foam Product (except Polystyrene) Manufacturing
    UrethaneAndOtherFoamProductExceptPolystyreneManufacturing,
    /// Plastics Bottle Manufacturing
    PlasticsBottleManufacturing,
    /// Plastics Plumbing Fixture Manufacturing
    PlasticsPlumbingFixtureManufacturing,
    /// All Other Plastics Product Manufacturing
    AllOtherPlasticsProductManufacturing,
    /// Tire Manufacturing (except Retreading)
    TireManufacturingExceptRetreading,
    /// Tire Retreading
    TireRetreading,
    /// Rubber and Plastics Hoses and Belting Manufacturing
    RubberAndPlasticsHosesAndBeltingManufacturing,
    /// Rubber Product Manufacturing for Mechanical Use
    RubberProductManufacturingForMechanicalUse,
    /// All Other Rubber Product Manufacturing
    AllOtherRubberProductManufacturing,
    /// Pottery, Ceramics, and Plumbing Fixture Manufacturing
    PotteryCeramicsAndPlumbingFixtureManufacturing,
    /// Clay Building Material and Refractories Manufacturing
    ClayBuildingMaterialAndRefractoriesManufacturing,
    /// Flat Glass Manufacturing
    FlatGlassManufacturing,
    /// Other Pressed and Blown Glass and Glassware Manufacturing
    OtherPressedAndBlownGlassAndGlasswareManufacturing,
    /// Glass Container Manufacturing
    GlassContainerManufacturing,
    /// Glass Product Manufacturing Made of Purchased Glass
    GlassProductManufacturingMadeOfPurchasedGlass,
    /// Cement Manufacturing
    CementManufacturing,
    /// Ready-Mix Concrete Manufacturing
    ReadymixConcreteManufacturing,
    /// Concrete Block and Brick Manufacturing
    ConcreteBlockAndBrickManufacturing,
    /// Concrete Pipe Manufacturing
    ConcretePipeManufacturing,
    /// Other Concrete Product Manufacturing
    OtherConcreteProductManufacturing,
    /// Lime Manufacturing
    LimeManufacturing,
    /// Gypsum Product Manufacturing
    GypsumProductManufacturing,
    /// Abrasive Product Manufacturing
    AbrasiveProductManufacturing,
    /// Cut Stone and Stone Product Manufacturing
    CutStoneAndStoneProductManufacturing,
    /// Ground or Treated Mineral and Earth Manufacturing
    GroundOrTreatedMineralAndEarthManufacturing,
    /// Mineral Wool Manufacturing
    MineralWoolManufacturing,
    /// All Other Miscellaneous Nonmetallic Mineral Product Manufacturing
    AllOtherMiscellaneousNonmetallicMineralProductManufacturing,
    /// Iron and Steel Mills and Ferroalloy Manufacturing
    IronAndSteelMillsAndFerroalloyManufacturing,
    /// Iron and Steel Pipe and Tube Manufacturing from Purchased Steel
    IronAndSteelPipeAndTubeManufacturingFromPurchasedSteel,
    /// Rolled Steel Shape Manufacturing
    RolledSteelShapeManufacturing,
    /// Steel Wire Drawing
    SteelWireDrawing,
    /// Alumina Refining and Primary Aluminum Production
    AluminaRefiningAndPrimaryAluminumProduction,
    /// Secondary Smelting and Alloying of Aluminum
    SecondarySmeltingAndAlloyingOfAluminum,
    /// Aluminum Sheet, Plate, and Foil Manufacturing
    AluminumSheetPlateAndFoilManufacturing,
    /// Other Aluminum Rolling, Drawing, and Extruding
    OtherAluminumRollingDrawingAndExtruding,
    /// Nonferrous Metal (except Aluminum) Smelting and Refining
    NonferrousMetalExceptAluminumSmeltingAndRefining,
    /// Copper Rolling, Drawing, Extruding, and Alloying
    CopperRollingDrawingExtrudingAndAlloying,
    /// Nonferrous Metal (except Copper and Aluminum) Rolling, Drawing, and Extruding
    NonferrousMetalExceptCopperAndAluminumRollingDrawingAndExtruding,
    /// Secondary Smelting, Refining, and Alloying of Nonferrous Metal (except Copper and Aluminum)
    SecondarySmeltingRefiningAndAlloyingOfNonferrousMetalExceptCopperAndAluminum,
    /// Iron Foundries
    IronFoundries,
    /// Steel Investment Foundries
    SteelInvestmentFoundries,
    /// Steel Foundries (except Investment)
    SteelFoundriesExceptInvestment,
    /// Nonferrous Metal Die-Casting Foundries
    NonferrousMetalDiecastingFoundries,
    /// Aluminum Foundries (except Die-Casting)
    AluminumFoundriesExceptDiecasting,
    /// Other Nonferrous Metal Foundries (except Die-Casting)
    OtherNonferrousMetalFoundriesExceptDiecasting,
    /// Iron and Steel Forging
    IronAndSteelForging,
    /// Nonferrous Forging
    NonferrousForging,
    /// Custom Roll Forming
    CustomRollForming,
    /// Powder Metallurgy Part Manufacturing
    PowderMetallurgyPartManufacturing,
    /// Metal Crown, Closure, and Other Metal Stamping (except Automotive)
    MetalCrownClosureAndOtherMetalStampingExceptAutomotive,
    /// Metal Kitchen Cookware, Utensil, Cutlery, and Flatware (except Precious) Manufacturing
    MetalKitchenCookwareUtensilCutleryAndFlatwareExceptPreciousManufacturing,
    /// Saw Blade and Handtool Manufacturing
    SawBladeAndHandtoolManufacturing,
    /// Prefabricated Metal Building and Component Manufacturing
    PrefabricatedMetalBuildingAndComponentManufacturing,
    /// Fabricated Structural Metal Manufacturing
    FabricatedStructuralMetalManufacturing,
    /// Plate Work Manufacturing
    PlateWorkManufacturing,
    /// Metal Window and Door Manufacturing
    MetalWindowAndDoorManufacturing,
    /// Sheet Metal Work Manufacturing
    SheetMetalWorkManufacturing,
    /// Ornamental and Architectural Metal Work Manufacturing
    OrnamentalAndArchitecturalMetalWorkManufacturing,
    /// Power Boiler and Heat Exchanger Manufacturing
    PowerBoilerAndHeatExchangerManufacturing,
    /// Metal Tank (Heavy Gauge) Manufacturing
    MetalTankHeavyGaugeManufacturing,
    /// Metal Can Manufacturing
    MetalCanManufacturing,
    /// Other Metal Container Manufacturing
    OtherMetalContainerManufacturing,
    /// Hardware Manufacturing
    HardwareManufacturing,
    /// Spring Manufacturing
    SpringManufacturing,
    /// Other Fabricated Wire Product Manufacturing
    OtherFabricatedWireProductManufacturing,
    /// Machine Shops
    MachineShops,
    /// Precision Turned Product Manufacturing
    PrecisionTurnedProductManufacturing,
    /// Bolt, Nut, Screw, Rivet, and Washer Manufacturing
    BoltNutScrewRivetAndWasherManufacturing,
    /// Metal Heat Treating
    MetalHeatTreating,
    /// Metal Coating, Engraving (except Jewelry and Silverware), and Allied Services to Manufacturers
    MetalCoatingEngravingExceptJewelryAndSilverwareAndAlliedServicesToManufacturers,
    /// Electroplating, Plating, Polishing, Anodizing, and Coloring
    ElectroplatingPlatingPolishingAnodizingAndColoring,
    /// Industrial Valve Manufacturing
    IndustrialValveManufacturing,
    /// Fluid Power Valve and Hose Fitting Manufacturing
    FluidPowerValveAndHoseFittingManufacturing,
    /// Plumbing Fixture Fitting and Trim Manufacturing
    PlumbingFixtureFittingAndTrimManufacturing,
    /// Other Metal Valve and Pipe Fitting Manufacturing
    OtherMetalValveAndPipeFittingManufacturing,
    /// Ball and Roller Bearing Manufacturing
    BallAndRollerBearingManufacturing,
    /// Small Arms Ammunition Manufacturing
    SmallArmsAmmunitionManufacturing,
    /// Ammunition (except Small Arms) Manufacturing
    AmmunitionExceptSmallArmsManufacturing,
    /// Small Arms, Ordnance, and Ordnance Accessories Manufacturing
    SmallArmsOrdnanceAndOrdnanceAccessoriesManufacturing,
    /// Fabricated Pipe and Pipe Fitting Manufacturing
    FabricatedPipeAndPipeFittingManufacturing,
    /// All Other Miscellaneous Fabricated Metal Product Manufacturing
    AllOtherMiscellaneousFabricatedMetalProductManufacturing,
    /// Farm Machinery and Equipment Manufacturing
    FarmMachineryAndEquipmentManufacturing,
    /// Lawn and Garden Tractor and Home Lawn and Garden Equipment Manufacturing
    LawnAndGardenTractorAndHomeLawnAndGardenEquipmentManufacturing,
    /// Construction Machinery Manufacturing
    ConstructionMachineryManufacturing,
    /// Mining Machinery and Equipment Manufacturing
    MiningMachineryAndEquipmentManufacturing,
    /// Oil and Gas Field Machinery and Equipment Manufacturing
    OilAndGasFieldMachineryAndEquipmentManufacturing,
    /// Food Product Machinery Manufacturing
    FoodProductMachineryManufacturing,
    /// Semiconductor Machinery Manufacturing
    SemiconductorMachineryManufacturing,
    /// Sawmill, Woodworking, and Paper Machinery Manufacturing
    SawmillWoodworkingAndPaperMachineryManufacturing,
    /// All Other Industrial Machinery Manufacturing
    AllOtherIndustrialMachineryManufacturing,
    /// Commercial and Service Industry Machinery Manufacturing
    CommercialAndServiceIndustryMachineryManufacturing,
    /// Industrial and Commercial Fan and Blower and Air Purification Equipment Manufacturing
    IndustrialAndCommercialFanAndBlowerAndAirPurificationEquipmentManufacturing,
    /// Heating Equipment (except Warm Air Furnaces) Manufacturing
    HeatingEquipmentExceptWarmAirFurnacesManufacturing,
    /// Air-Conditioning and Warm Air Heating Equipment and Commercial and Industrial Refrigeration Equipment Manufacturing
    AirconditioningAndWarmAirHeatingEquipmentAndCommercialAndIndustrialRefrigerationEquipmentManufacturing,
    /// Industrial Mold Manufacturing
    IndustrialMoldManufacturing,
    /// Special Die and Tool, Die Set, Jig, and Fixture Manufacturing
    SpecialDieAndToolDieSetJigAndFixtureManufacturing,
    /// Cutting Tool and Machine Tool Accessory Manufacturing
    CuttingToolAndMachineToolAccessoryManufacturing,
    /// Machine Tool Manufacturing
    MachineToolManufacturing,
    /// Rolling Mill and Other Metalworking Machinery Manufacturing
    RollingMillAndOtherMetalworkingMachineryManufacturing,
    /// Turbine and Turbine Generator Set Units Manufacturing
    TurbineAndTurbineGeneratorSetUnitsManufacturing,
    /// Speed Changer, Industrial High-Speed Drive, and Gear Manufacturing
    SpeedChangerIndustrialHighspeedDriveAndGearManufacturing,
    /// Mechanical Power Transmission Equipment Manufacturing
    MechanicalPowerTransmissionEquipmentManufacturing,
    /// Other Engine Equipment Manufacturing
    OtherEngineEquipmentManufacturing,
    /// Air and Gas Compressor Manufacturing
    AirAndGasCompressorManufacturing,
    /// Measuring, Dispensing, and Other Pumping Equipment Manufacturing
    MeasuringDispensingAndOtherPumpingEquipmentManufacturing,
    /// Elevator and Moving Stairway Manufacturing
    ElevatorAndMovingStairwayManufacturing,
    /// Conveyor and Conveying Equipment Manufacturing
    ConveyorAndConveyingEquipmentManufacturing,
    /// Overhead Traveling Crane, Hoist, and Monorail System Manufacturing
    OverheadTravelingCraneHoistAndMonorailSystemManufacturing,
    /// Industrial Truck, Tractor, Trailer, and Stacker Machinery Manufacturing
    IndustrialTruckTractorTrailerAndStackerMachineryManufacturing,
    /// Power-Driven Handtool Manufacturing
    PowerdrivenHandtoolManufacturing,
    /// Welding and Soldering Equipment Manufacturing
    WeldingAndSolderingEquipmentManufacturing,
    /// Packaging Machinery Manufacturing
    PackagingMachineryManufacturing,
    /// Industrial Process Furnace and Oven Manufacturing
    IndustrialProcessFurnaceAndOvenManufacturing,
    /// Fluid Power Cylinder and Actuator Manufacturing
    FluidPowerCylinderAndActuatorManufacturing,
    /// Fluid Power Pump and Motor Manufacturing
    FluidPowerPumpAndMotorManufacturing,
    /// All Other Miscellaneous General Purpose Machinery Manufacturing
    AllOtherMiscellaneousGeneralPurposeMachineryManufacturing,
    /// Electronic Computer Manufacturing
    ElectronicComputerManufacturing,
    /// Computer Storage Device Manufacturing
    ComputerStorageDeviceManufacturing,
    /// Computer Terminal and Other Computer Peripheral Equipment Manufacturing
    ComputerTerminalAndOtherComputerPeripheralEquipmentManufacturing,
    /// Telephone Apparatus Manufacturing
    TelephoneApparatusManufacturing,
    /// Radio and Television Broadcasting and Wireless Communications Equipment Manufacturing
    RadioAndTelevisionBroadcastingAndWirelessCommunicationsEquipmentManufacturing,
    /// Other Communications Equipment Manufacturing
    OtherCommunicationsEquipmentManufacturing,
    /// Audio and Video Equipment Manufacturing
    AudioAndVideoEquipmentManufacturing,
    /// Bare Printed Circuit Board Manufacturing  
    BarePrintedCircuitBoardManufacturing,
    /// Semiconductor and Related Device Manufacturing
    SemiconductorAndRelatedDeviceManufacturing,
    /// Capacitor, Resistor, Coil, Transformer, and Other Inductor Manufacturing
    CapacitorResistorCoilTransformerAndOtherInductorManufacturing,
    /// Electronic Connector Manufacturing
    ElectronicConnectorManufacturing,
    /// Printed Circuit Assembly (Electronic Assembly) Manufacturing
    PrintedCircuitAssemblyElectronicAssemblyManufacturing,
    /// Other Electronic Component Manufacturing
    OtherElectronicComponentManufacturing,
    /// Electromedical and Electrotherapeutic Apparatus Manufacturing
    ElectromedicalAndElectrotherapeuticApparatusManufacturing,
    /// Search, Detection, Navigation, Guidance, Aeronautical, and Nautical System and Instrument Manufacturing
    SearchDetectionNavigationGuidanceAeronauticalAndNauticalSystemAndInstrumentManufacturing,
    /// Automatic Environmental Control Manufacturing for Residential, Commercial, and Appliance Use
    AutomaticEnvironmentalControlManufacturingForResidentialCommercialAndApplianceUse,
    /// Instruments and Related Products Manufacturing for Measuring, Displaying, and Controlling Industrial Process Variables
    InstrumentsAndRelatedProductsManufacturingForMeasuringDisplayingAndControllingIndustrialProcessVariables,
    /// Totalizing Fluid Meter and Counting Device Manufacturing
    TotalizingFluidMeterAndCountingDeviceManufacturing,
    /// Instrument Manufacturing for Measuring and Testing Electricity and Electrical Signals
    InstrumentManufacturingForMeasuringAndTestingElectricityAndElectricalSignals,
    /// Analytical Laboratory Instrument Manufacturing
    AnalyticalLaboratoryInstrumentManufacturing,
    /// Irradiation Apparatus Manufacturing
    IrradiationApparatusManufacturing,
    /// Other Measuring and Controlling Device Manufacturing
    OtherMeasuringAndControllingDeviceManufacturing,
    /// Manufacturing and Reproducing Magnetic and Optical Media
    ManufacturingAndReproducingMagneticAndOpticalMedia,
    /// Residential Electric Lighting Fixture Manufacturing
    ResidentialElectricLightingFixtureManufacturing,
    /// Commercial, Industrial, and Institutional Electric Lighting Fixture Manufacturing
    CommercialIndustrialAndInstitutionalElectricLightingFixtureManufacturing,
    /// Electric Lamp Bulb and Other Lighting Equipment Manufacturing
    ElectricLampBulbAndOtherLightingEquipmentManufacturing,
    /// Small Electrical Appliance Manufacturing
    SmallElectricalApplianceManufacturing,
    /// Major Household Appliance Manufacturing
    MajorHouseholdApplianceManufacturing,
    /// Power, Distribution, and Specialty Transformer Manufacturing
    PowerDistributionAndSpecialtyTransformerManufacturing,
    /// Motor and Generator Manufacturing
    MotorAndGeneratorManufacturing,
    /// Switchgear and Switchboard Apparatus Manufacturing
    SwitchgearAndSwitchboardApparatusManufacturing,
    /// Relay and Industrial Control Manufacturing
    RelayAndIndustrialControlManufacturing,
    /// Battery Manufacturing
    BatteryManufacturing,
    /// Fiber Optic Cable Manufacturing
    FiberOpticCableManufacturing,
    /// Other Communication and Energy Wire Manufacturing
    OtherCommunicationAndEnergyWireManufacturing,
    /// Current-Carrying Wiring Device Manufacturing
    CurrentcarryingWiringDeviceManufacturing,
    /// Noncurrent-Carrying Wiring Device Manufacturing
    NoncurrentcarryingWiringDeviceManufacturing,
    /// Carbon and Graphite Product Manufacturing
    CarbonAndGraphiteProductManufacturing,
    /// All Other Miscellaneous Electrical Equipment and Component Manufacturing
    AllOtherMiscellaneousElectricalEquipmentAndComponentManufacturing,
    /// Automobile and Light Duty Motor Vehicle Manufacturing
    AutomobileAndLightDutyMotorVehicleManufacturing,
    /// Heavy Duty Truck Manufacturing
    HeavyDutyTruckManufacturing,
    /// Motor Vehicle Body Manufacturing
    MotorVehicleBodyManufacturing,
    /// Truck Trailer Manufacturing
    TruckTrailerManufacturing,
    /// Motor Home Manufacturing
    MotorHomeManufacturing,
    /// Travel Trailer and Camper Manufacturing
    TravelTrailerAndCamperManufacturing,
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
    /// Aircraft Manufacturing
    AircraftManufacturing,
    /// Aircraft Engine and Engine Parts Manufacturing
    AircraftEngineAndEnginePartsManufacturing,
    /// Other Aircraft Parts and Auxiliary Equipment Manufacturing
    OtherAircraftPartsAndAuxiliaryEquipmentManufacturing,
    /// Guided Missile and Space Vehicle Manufacturing
    GuidedMissileAndSpaceVehicleManufacturing,
    /// Guided Missile and Space Vehicle Propulsion Unit and Propulsion Unit Parts Manufacturing
    GuidedMissileAndSpaceVehiclePropulsionUnitAndPropulsionUnitPartsManufacturing,
    /// Other Guided Missile and Space Vehicle Parts and Auxiliary Equipment Manufacturing
    OtherGuidedMissileAndSpaceVehiclePartsAndAuxiliaryEquipmentManufacturing,
    /// Railroad Rolling Stock Manufacturing
    RailroadRollingStockManufacturing,
    /// Ship Building and Repairing
    ShipBuildingAndRepairing,
    /// Boat Building
    BoatBuilding,
    /// Motorcycle, Bicycle, and Parts Manufacturing
    MotorcycleBicycleAndPartsManufacturing,
    /// Military Armored Vehicle, Tank, and Tank Component Manufacturing
    MilitaryArmoredVehicleTankAndTankComponentManufacturing,
    /// All Other Transportation Equipment Manufacturing
    AllOtherTransportationEquipmentManufacturing,
    /// Wood Kitchen Cabinet and Countertop Manufacturing
    WoodKitchenCabinetAndCountertopManufacturing,
    /// Upholstered Household Furniture Manufacturing
    UpholsteredHouseholdFurnitureManufacturing,
    /// Nonupholstered Wood Household Furniture Manufacturing
    NonupholsteredWoodHouseholdFurnitureManufacturing,
    /// Household Furniture (except Wood and Upholstered) Manufacturing
    HouseholdFurnitureExceptWoodAndUpholsteredManufacturing,
    /// Institutional Furniture Manufacturing
    InstitutionalFurnitureManufacturing,
    /// Wood Office Furniture Manufacturing
    WoodOfficeFurnitureManufacturing,
    /// Custom Architectural Woodwork and Millwork Manufacturing
    CustomArchitecturalWoodworkAndMillworkManufacturing,
    /// Office Furniture (except Wood) Manufacturing
    OfficeFurnitureExceptWoodManufacturing,
    /// Showcase, Partition, Shelving, and Locker Manufacturing
    ShowcasePartitionShelvingAndLockerManufacturing,
    /// Mattress Manufacturing
    MattressManufacturing,
    /// Blind and Shade Manufacturing
    BlindAndShadeManufacturing,
    /// Surgical and Medical Instrument Manufacturing
    SurgicalAndMedicalInstrumentManufacturing,
    /// Surgical Appliance and Supplies Manufacturing
    SurgicalApplianceAndSuppliesManufacturing,
    /// Dental Equipment and Supplies Manufacturing
    DentalEquipmentAndSuppliesManufacturing,
    /// Ophthalmic Goods Manufacturing
    OphthalmicGoodsManufacturing,
    /// Dental Laboratories
    DentalLaboratories,
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
    /// Gasket, Packing, and Sealing Device Manufacturing
    GasketPackingAndSealingDeviceManufacturing,
    /// Musical Instrument Manufacturing
    MusicalInstrumentManufacturing,
    /// Fastener, Button, Needle, and Pin Manufacturing
    FastenerButtonNeedleAndPinManufacturing,
    /// Broom, Brush, and Mop Manufacturing
    BroomBrushAndMopManufacturing,
    /// Burial Casket Manufacturing
    BurialCasketManufacturing,
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
    /// Boat Dealers
    BoatDealers,
    /// Motorcycle, ATV, and All Other Motor Vehicle Dealers
    MotorcycleAtvAndAllOtherMotorVehicleDealers,
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
    /// Convenience Retailers
    ConvenienceRetailers,
    /// Vending Machine Operators
    VendingMachineOperators,
    /// Fruit and Vegetable Retailers
    FruitAndVegetableRetailers,
    /// Meat Retailers
    MeatRetailers,
    /// Fish and Seafood Retailers
    FishAndSeafoodRetailers,
    /// Baked Goods Retailers
    BakedGoodsRetailers,
    /// Confectionery and Nut Retailers
    ConfectioneryAndNutRetailers,
    /// All Other Specialty Food Retailers
    AllOtherSpecialtyFoodRetailers,
    /// Beer, Wine, and Liquor Retailers
    BeerWineAndLiquorRetailers,
    /// Furniture Retailers
    FurnitureRetailers,
    /// Floor Covering Retailers
    FloorCoveringRetailers,
    /// Window Treatment Retailers
    WindowTreatmentRetailers,
    /// All Other Home Furnishings Retailers
    AllOtherHomeFurnishingsRetailers,
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
    /// Food (Health) Supplement Retailers
    FoodHealthSupplementRetailers,
    /// All Other Health and Personal Care Retailers
    AllOtherHealthAndPersonalCareRetailers,
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
    /// Tobacco, Electronic Cigarette, and Other Smoking Supplies Retailers
    TobaccoElectronicCigaretteAndOtherSmokingSuppliesRetailers,
    /// All Other Miscellaneous Retailers
    AllOtherMiscellaneousRetailers,
    /// Scheduled Passenger Air Transportation
    ScheduledPassengerAirTransportation,
    /// Scheduled Freight Air Transportation
    ScheduledFreightAirTransportation,
    /// Nonscheduled Chartered Passenger Air Transportation
    NonscheduledCharteredPassengerAirTransportation,
    /// Nonscheduled Chartered Freight Air Transportation
    NonscheduledCharteredFreightAirTransportation,
    /// Other Nonscheduled Air Transportation
    OtherNonscheduledAirTransportation,
    /// Line-Haul Railroads
    LineHaulRailroads,
    /// Short Line Railroads
    ShortLineRailroads,
    /// Deep Sea Freight Transportation
    DeepSeaFreightTransportation,
    /// Deep Sea Passenger Transportation
    DeepSeaPassengerTransportation,
    /// Coastal and Great Lakes Freight Transportation
    CoastalAndGreatLakesFreightTransportation,
    /// Coastal and Great Lakes Passenger Transportation
    CoastalAndGreatLakesPassengerTransportation,
    /// Inland Water Freight Transportation
    InlandWaterFreightTransportation,
    /// Inland Water Passenger Transportation
    InlandWaterPassengerTransportation,
    /// General Freight Trucking, Local
    GeneralFreightTruckingLocal,
    /// General Freight Trucking, Long-Distance, Truckload
    GeneralFreightTruckingLongdistanceTruckload,
    /// General Freight Trucking, Long-Distance, Less Than Truckload
    GeneralFreightTruckingLongdistanceLessThanTruckload,
    /// Used Household and Office Goods Moving
    UsedHouseholdAndOfficeGoodsMoving,
    /// Specialized Freight (except Used Goods) Trucking, Local
    SpecializedFreightExceptUsedGoodsTruckingLocal,
    /// Specialized Freight (except Used Goods) Trucking, Long-Distance
    SpecializedFreightExceptUsedGoodsTruckingLongdistance,
    /// Mixed Mode Transit Systems
    MixedModeTransitSystems,
    /// Commuter Rail Systems
    CommuterRailSystems,
    /// Bus and Other Motor Vehicle Transit Systems
    BusAndOtherMotorVehicleTransitSystems,
    /// Street Railroads
    StreetRailroads,
    /// Light Rail Transit Systems
    LightRailTransitSystems,
    /// Other Urban Transit Systems
    OtherUrbanTransitSystems,
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
    /// Special Needs Transportation
    SpecialNeedsTransportation,
    /// All Other Transit and Ground Passenger Transportation
    AllOtherTransitAndGroundPassengerTransportation,
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
    /// Air Traffic Control
    AirTrafficControl,
    /// Other Airport Operations
    OtherAirportOperations,
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
    /// Packing and Crating
    PackingAndCrating,
    /// All Other Support Activities for Transportation
    AllOtherSupportActivitiesForTransportation,
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
    /// Greeting Card Publishers
    GreetingCardPublishers,
    /// All Other Publishers
    AllOtherPublishers,
    /// Software Publishers
    SoftwarePublishers,
    /// Motion Picture and Video Production
    MotionPictureAndVideoProduction,
    /// Motion Picture and Video Distribution
    MotionPictureAndVideoDistribution,
    /// Motion Picture Theaters (except Drive-Ins)
    MotionPictureTheatersExceptDriveins,
    /// Drive-In Motion Picture Theaters
    DriveInMotionPictureTheaters,
    /// Teleproduction and Other Postproduction Services
    TeleproductionAndOtherPostproductionServices,
    /// Other Motion Picture and Video Industries
    OtherMotionPictureAndVideoIndustries,
    /// Music Publishers
    MusicPublishers,
    /// Sound Recording Studios
    SoundRecordingStudios,
    /// Record Production and Distribution
    RecordProductionAndDistribution,
    /// Other Sound Recording Industries
    OtherSoundRecordingIndustries,
    /// Radio Networks
    RadioNetworks,
    /// Radio Stations
    RadioStations,
    /// Television Broadcasting
    TelevisionBroadcasting,
    /// Cable and Other Subscription Programming
    CableAndOtherSubscriptionProgramming,
    /// Wired Telecommunications Carriers
    WiredTelecommunicationsCarriers,
    /// Wireless Telecommunications Carriers (except Satellite)
    WirelessTelecommunicationsCarriersExceptSatellite,
    /// Satellite Telecommunications
    SatelliteTelecommunications,
    /// Telecommunications Resellers
    TelecommunicationsResellers,
    /// All Other Telecommunications
    AllOtherTelecommunications,
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
    /// Consumer Lending
    ConsumerLending,
    /// Real Estate Credit
    RealEstateCredit,
    /// All Other Nondepository Credit Intermediation
    AllOtherNondepositoryCreditIntermediation,
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
    /// Trust, Fiduciary, and Custody Activities
    TrustFiduciaryAndCustodyActivities,
    /// Miscellaneous Financial Investment Activities
    MiscellaneousFinancialInvestmentActivities,
    /// Direct Life Insurance Carriers
    DirectLifeInsuranceCarriers,
    /// Direct Health and Medical Insurance Carriers
    DirectHealthAndMedicalInsuranceCarriers,
    /// Direct Property and Casualty Insurance Carriers
    DirectPropertyAndCasualtyInsuranceCarriers,
    /// Direct Title Insurance Carriers
    DirectTitleInsuranceCarriers,
    /// Other Direct Insurance (except Life, Health, and Medical) Carriers
    OtherDirectInsuranceExceptLifeHealthAndMedicalCarriers,
    /// Reinsurance Carriers
    ReinsuranceCarriers,
    /// Insurance Agencies and Brokerages
    InsuranceAgenciesAndBrokerages,
    /// Claims Adjusting
    ClaimsAdjusting,
    /// Third Party Administration of Insurance and Pension Funds
    ThirdPartyAdministrationOfInsuranceAndPensionFunds,
    /// All Other Insurance Related Activities
    AllOtherInsuranceRelatedActivities,
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
    /// Residential Property Managers
    ResidentialPropertyManagers,
    /// Nonresidential Property Managers
    NonresidentialPropertyManagers,
    /// Offices of Real Estate Appraisers
    OfficesOfRealEstateAppraisers,
    /// Other Activities Related to Real Estate
    OtherActivitiesRelatedToRealEstate,
    /// Passenger Car Rental
    PassengerCarRental,
    /// Passenger Car Leasing
    PassengerCarLeasing,
    /// Truck, Utility Trailer, and RV (Recreational Vehicle) Rental and Leasing
    TruckUtilityTrailerAndRvRecreationalVehicleRentalAndLeasing,
    /// Consumer Electronics and Appliances Rental
    ConsumerElectronicsAndAppliancesRental,
    /// Formal Wear and Costume Rental
    FormalWearAndCostumeRental,
    /// Video Tape and Disc Rental
    VideoTapeAndDiscRental,
    /// Recreational Goods Rental Parent
    RecreationalGoodsRentalParent,
    /// Medical Equipment and Supplies Rental
    MedicalEquipmentAndSuppliesRental,
    /// Home Health Equipment and Supplies Rental
    HomeHealthEquipmentAndSuppliesRental,
    /// Recreational Goods Rental
    RecreationalGoodsRental,
    /// All Other Consumer Goods Rental
    AllOtherConsumerGoodsRental,
    /// General Rental Centers
    GeneralRentalCenters,
    /// Commercial Air, Rail, and Water Transportation Equipment Rental and Leasing
    CommercialAirRailAndWaterTransportationEquipmentRentalAndLeasing,
    /// Construction, Mining, and Forestry Machinery and Equipment Rental and Leasing
    ConstructionMiningAndForestryMachineryAndEquipmentRentalAndLeasing,
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
    /// Title Abstract and Settlement Offices
    TitleAbstractAndSettlementOffices,
    /// All Other Legal Services
    AllOtherLegalServices,
    /// Offices of Certified Public Accountants
    OfficesOfCertifiedPublicAccountants,
    /// Tax Preparation Services
    TaxPreparationServices,
    /// Payroll Services
    PayrollServices,
    /// Other Accounting Services
    OtherAccountingServices,
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
    /// Custom Computer Programming Services
    CustomComputerProgrammingServices,
    /// Computer Systems Design Services
    ComputerSystemsDesignServices,
    /// Computer Facilities Management Services
    ComputerFacilitiesManagementServices,
    /// Other Computer Related Services
    OtherComputerRelatedServices,
    /// Administrative Management and General Management Consulting Services
    AdministrativeManagementAndGeneralManagementConsultingServices,
    /// Human Resources Consulting Services
    HumanResourcesConsultingServices,
    /// Marketing Consulting Services
    MarketingConsultingServices,
    /// Process, Physical Distribution, and Logistics Consulting Services
    ProcessPhysicalDistributionAndLogisticsConsultingServices,
    /// Other Management Consulting Services
    OtherManagementConsultingServices,
    /// Environmental Consulting Services
    EnvironmentalConsultingServices,
    /// Other Scientific and Technical Consulting Services
    OtherScientificAndTechnicalConsultingServices,
    /// Research and Development in Nanotechnology
    ResearchAndDevelopmentInNanotechnology,
    /// Research and Development in Biotechnology (except Nanobiotechnology)
    ResearchAndDevelopmentInBiotechnologyExceptNanobiotechnology,
    /// Research and Development in the Physical, Engineering, and Life Sciences (except Nanotechnology and Biotechnology)
    ResearchAndDevelopmentInThePhysicalEngineeringAndLifeSciencesExceptNanotechnologyAndBiotechnology,
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
    /// Photography Studios, Portrait
    PhotographyStudiosPortrait,
    /// Commercial Photography
    CommercialPhotography,
    /// Translation and Interpretation Services
    TranslationAndInterpretationServices,
    /// Veterinary Services
    VeterinaryServices,
    /// All Other Professional, Scientific, and Technical Services
    AllOtherProfessionalScientificAndTechnicalServices,
    /// Offices of Bank Holding Companies
    OfficesOfBankHoldingCompanies,
    /// Offices of Other Holding Companies
    OfficesOfOtherHoldingCompanies,
    /// Corporate, Subsidiary, and Regional Managing Offices Parent
    CorporateSubsidiaryAndRegionalManagingOfficesParent,
    /// Corporate, Subsidiary, and Regional Managing Offices
    CorporateSubsidiaryAndRegionalManagingOffices,
    /// Office Administrative Services
    OfficeAdministrativeServices,
    /// Facilities Support Services
    FacilitiesSupportServices,
    /// Employment Placement Agencies
    EmploymentPlacementAgencies,
    /// Executive Search Services
    ExecutiveSearchServices,
    /// Temporary Help Services
    TemporaryHelpServices,
    /// Professional Employer Organizations
    ProfessionalEmployerOrganizations,
    /// Document Preparation Services
    DocumentPreparationServices,
    /// Telephone Answering Services
    TelephoneAnsweringServices,
    /// Telemarketing Bureaus and Other Contact Centers
    TelemarketingBureausAndOtherContactCenters,
    /// Private Mail Centers
    PrivateMailCenters,
    /// Other Business Service Centers (including Copy Shops)
    OtherBusinessServiceCentersIncludingCopyShops,
    /// Collection Agencies
    CollectionAgencies,
    /// Credit Bureaus
    CreditBureaus,
    /// Repossession Services
    RepossessionServices,
    /// Court Reporting and Stenotype Services
    CourtReportingAndStenotypeServices,
    /// All Other Business Support Services
    AllOtherBusinessSupportServices,
    /// Travel Agencies
    TravelAgencies,
    /// Tour Operators
    TourOperators,
    /// Convention and Visitors Bureaus
    ConventionAndVisitorsBureaus,
    /// All Other Travel Arrangement and Reservation Services
    AllOtherTravelArrangementAndReservationServices,
    /// Investigation Services
    InvestigationServices,
    /// Security Guards and Patrol Services
    SecurityGuardsAndPatrolServices,
    /// Armored Car Services
    ArmoredCarServices,
    /// Security Systems Services (except Locksmiths)
    SecuritySystemsServicesExceptLocksmiths,
    /// Locksmiths
    Locksmiths,
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
    /// Solid Waste Collection
    SolidWasteCollection,
    /// Hazardous Waste Collection
    HazardousWasteCollection,
    /// Other Waste Collection
    OtherWasteCollection,
    /// Hazardous Waste Treatment and Disposal
    HazardousWasteTreatmentAndDisposal,
    /// Solid Waste Landfill
    SolidWasteLandfill,
    /// Solid Waste Combustors and Incinerators
    SolidWasteCombustorsAndIncinerators,
    /// Other Nonhazardous Waste Treatment and Disposal
    OtherNonhazardousWasteTreatmentAndDisposal,
    /// Remediation Services
    RemediationServices,
    /// Materials Recovery Facilities
    MaterialsRecoveryFacilities,
    /// Septic Tank and Related Services
    SepticTankAndRelatedServices,
    /// All Other Miscellaneous Waste Management Services
    AllOtherMiscellaneousWasteManagementServices,
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
    /// Cosmetology and Barber Schools
    CosmetologyAndBarberSchools,
    /// Flight Training
    FlightTraining,
    /// Apprenticeship Training
    ApprenticeshipTraining,
    /// Other Technical and Trade Schools
    OtherTechnicalAndTradeSchools,
    /// Fine Arts Schools
    FineArtsSchools,
    /// Sports and Recreation Instruction
    SportsAndRecreationInstruction,
    /// Language Schools
    LanguageSchools,
    /// Exam Preparation and Tutoring
    ExamPreparationAndTutoring,
    /// Automobile Driving Schools
    AutomobileDrivingSchools,
    /// All Other Miscellaneous Schools and Instruction
    AllOtherMiscellaneousSchoolsAndInstruction,
    /// Educational Support Services
    EducationalSupportServices,
    /// Offices of Physicians (except Mental Health Specialists)
    OfficesOfPhysiciansExceptMentalHealthSpecialists,
    /// Offices of Physicians, Mental Health Specialists
    OfficesOfPhysiciansMentalHealthSpecialists,
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
    /// Offices of Podiatrists
    OfficesOfPodiatrists,
    /// Offices of All Other Miscellaneous Health Practitioners
    OfficesOfAllOtherMiscellaneousHealthPractitioners,
    /// Family Planning Centers
    FamilyPlanningCenters,
    /// Outpatient Mental Health and Substance Abuse Centers
    OutpatientMentalHealthAndSubstanceAbuseCenters,
    /// HMO Medical Centers
    HmoMedicalCenters,
    /// Kidney Dialysis Centers
    KidneyDialysisCenters,
    /// Freestanding Ambulatory Surgical and Emergency Centers
    FreestandingAmbulatorySurgicalAndEmergencyCenters,
    /// All Other Outpatient Care Centers
    AllOtherOutpatientCareCenters,
    /// Medical Laboratories
    MedicalLaboratories,
    /// Diagnostic Imaging Centers
    DiagnosticImagingCenters,
    /// Home Health Care Services
    HomeHealthCareServices,
    /// Ambulance Services
    AmbulanceServices,
    /// Blood and Organ Banks
    BloodAndOrganBanks,
    /// All Other Miscellaneous Ambulatory Health Care Services
    AllOtherMiscellaneousAmbulatoryHealthCareServices,
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
    /// Continuing Care Retirement Communities
    ContinuingCareRetirementCommunities,
    /// Assisted Living Facilities for the Elderly
    AssistedLivingFacilitiesForTheElderly,
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
    /// Temporary Shelters
    TemporaryShelters,
    /// Other Community Housing Services
    OtherCommunityHousingServices,
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
    /// Sports Teams and Clubs
    SportsTeamsAndClubs,
    /// Racetracks
    Racetracks,
    /// Other Spectator Sports
    OtherSpectatorSports,
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
    /// Bed-and-Breakfast Inns
    BedAndBreakfastInns,
    /// All Other Traveler Accommodation
    AllOtherTravelerAccommodation,
    /// RV (Recreational Vehicle) Parks and Campgrounds
    RvRecreationalVehicleParksAndCampgrounds,
    /// Recreational and Vacation Camps (except Campgrounds)
    RecreationalAndVacationCampsExceptCampgrounds,
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
    /// Full-Service Restaurants
    FullServiceRestaurants,
    /// Limited-Service Restaurants
    LimitedServiceRestaurants,
    /// Cafeterias, Grill Buffets, and Buffets
    CafeteriasGrillBuffetsAndBuffets,
    /// Snack and Nonalcoholic Beverage Bars
    SnackAndNonalcoholicBeverageBars,
    /// General Automotive Repair
    GeneralAutomotiveRepair,
    /// Automotive Exhaust System Repair
    AutomotiveExhaustSystemRepair,
    /// Automotive Transmission Repair
    AutomotiveTransmissionRepair,
    /// Other Automotive Mechanical and Electrical Repair and Maintenance
    OtherAutomotiveMechanicalAndElectricalRepairAndMaintenance,
    /// Automotive Body, Paint, and Interior Repair and Maintenance
    AutomotiveBodyPaintAndInteriorRepairAndMaintenance,
    /// Automotive Glass Replacement Shops
    AutomotiveGlassReplacementShops,
    /// Automotive Oil Change and Lubrication Shops
    AutomotiveOilChangeAndLubricationShops,
    /// Car Washes
    CarWashes,
    /// All Other Automotive Repair and Maintenance
    AllOtherAutomotiveRepairAndMaintenance,
    /// Consumer Electronics Repair and Maintenance
    ConsumerElectronicsRepairAndMaintenance,
    /// Computer and Office Machine Repair and Maintenance
    ComputerAndOfficeMachineRepairAndMaintenance,
    /// Communication Equipment Repair and Maintenance
    CommunicationEquipmentRepairAndMaintenance,
    /// Other Electronic and Precision Equipment Repair and Maintenance
    OtherElectronicAndPrecisionEquipmentRepairAndMaintenance,
    /// Commercial and Industrial Machinery and Equipment (except Automotive and Electronic) Repair and Maintenance
    CommercialAndIndustrialMachineryAndEquipmentExceptAutomotiveAndElectronicRepairAndMaintenance,
    /// Home and Garden Equipment Repair and Maintenance
    HomeAndGardenEquipmentRepairAndMaintenance,
    /// Appliance Repair and Maintenance
    ApplianceRepairAndMaintenance,
    /// Reupholstery and Furniture Repair
    ReupholsteryAndFurnitureRepair,
    /// Footwear and Leather Goods Repair
    FootwearAndLeatherGoodsRepair,
    /// Other Personal and Household Goods Repair and Maintenance
    OtherPersonalAndHouseholdGoodsRepairAndMaintenance,
    /// Barber Shops
    BarberShops,
    /// Beauty Salons
    BeautySalons,
    /// Nail Salons
    NailSalons,
    /// Diet and Weight Reducing Centers
    DietAndWeightReducingCenters,
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
    /// Linen Supply
    LinenSupply,
    /// Industrial Launderers
    IndustrialLaunderers,
    /// Pet Care (except Veterinary) Services
    PetCareExceptVeterinaryServices,
    /// Photofinishing Laboratories (except One-Hour)
    PhotofinishingLaboratoriesExceptOneHour,
    /// One-Hour Photofinishing
    OneHourPhotofinishing,
    /// Parking Lots and Garages
    ParkingLotsAndGarages,
    /// All Other Personal Services
    AllOtherPersonalServices,
    /// Religious Organizations
    ReligiousOrganizations,
    /// Grantmaking Foundations
    GrantmakingFoundations,
    /// Voluntary Health Organizations
    VoluntaryHealthOrganizations,
    /// Other Grantmaking and Giving Services
    OtherGrantmakingAndGivingServices,
    /// Human Rights Organizations
    HumanRightsOrganizations,
    /// Environment, Conservation and Wildlife Organizations
    EnvironmentConservationAndWildlifeOrganizations,
    /// Other Social Advocacy Organizations
    OtherSocialAdvocacyOrganizations,
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

impl NaicsIndustry {
    /// Returns the industry description corresponding to this NAICS code
    pub fn description(&self) -> &'static str {
        match self {
            Self::SoybeanFarming => "Soybean Farming",
            Self::OilseedExceptSoybeanFarming => "Oilseed (except Soybean) Farming",
            Self::DryPeaAndBeanFarming => "Dry Pea and Bean Farming",
            Self::WheatFarming => "Wheat Farming",
            Self::CornFarming => "Corn Farming",
            Self::RiceFarming => "Rice Farming",
            Self::OilseedAndGrainCombinationFarming => "Oilseed and Grain Combination Farming",
            Self::AllOtherGrainFarming => "All Other Grain Farming",
            Self::PotatoFarming => "Potato Farming",
            Self::OtherVegetableExceptPotatoAndMelonFarming => {
                "Other Vegetable (except Potato) and Melon Farming"
            }
            Self::OrangeGroves => "Orange Groves",
            Self::CitrusExceptOrangeGroves => "Citrus (except Orange) Groves",
            Self::AppleOrchards => "Apple Orchards",
            Self::GrapeVineyards => "Grape Vineyards",
            Self::StrawberryFarming => "Strawberry Farming",
            Self::BerryExceptStrawberryFarming => "Berry (except Strawberry) Farming",
            Self::TreeNutFarming => "Tree Nut Farming",
            Self::FruitAndTreeNutCombinationFarming => "Fruit and Tree Nut Combination Farming",
            Self::OtherNoncitrusFruitFarming => "Other Noncitrus Fruit Farming",
            Self::MushroomProduction => "Mushroom Production",
            Self::OtherFoodCropsGrownUnderCover => "Other Food Crops Grown Under Cover",
            Self::NurseryAndTreeProduction => "Nursery and Tree Production",
            Self::FloricultureProduction => "Floriculture Production",
            Self::TobaccoFarming => "Tobacco Farming",
            Self::CottonFarming => "Cotton Farming",
            Self::SugarcaneFarming => "Sugarcane Farming",
            Self::HayFarming => "Hay Farming",
            Self::SugarBeetFarming => "Sugar Beet Farming",
            Self::PeanutFarming => "Peanut Farming",
            Self::AllOtherMiscellaneousCropFarming => "All Other Miscellaneous Crop Farming",
            Self::BeefCattleRanchingAndFarming => "Beef Cattle Ranching and Farming",
            Self::CattleFeedlots => "Cattle Feedlots",
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
            Self::FinfishFarmingAndFishHatcheries => "Finfish Farming and Fish Hatcheries",
            Self::ShellfishFarming => "Shellfish Farming",
            Self::OtherAquaculture => "Other Aquaculture",
            Self::Apiculture => "Apiculture",
            Self::HorsesAndOtherEquineProduction => "Horses and Other Equine Production",
            Self::FurbearingAnimalAndRabbitProduction => "Fur-Bearing Animal and Rabbit Production",
            Self::AllOtherAnimalProduction => "All Other Animal Production",
            Self::TimberTractOperations => "Timber Tract Operations",
            Self::ForestNurseriesAndGatheringOfForestProducts => {
                "Forest Nurseries and Gathering of Forest Products"
            }
            Self::Logging => "Logging",
            Self::FinfishFishing => "Finfish Fishing",
            Self::ShellfishFishing => "Shellfish Fishing",
            Self::OtherMarineFishing => "Other Marine Fishing",
            Self::HuntingAndTrapping => "Hunting and Trapping",
            Self::CottonGinning => "Cotton Ginning",
            Self::SoilPreparationPlantingAndCultivating => {
                "Soil Preparation, Planting, and Cultivating"
            }
            Self::CropHarvestingPrimarilyByMachine => "Crop Harvesting, Primarily by Machine",
            Self::PostharvestCropActivitiesExceptCottonGinning => {
                "Postharvest Crop Activities (except Cotton Ginning)"
            }
            Self::FarmLaborContractorsAndCrewLeaders => "Farm Labor Contractors and Crew Leaders",
            Self::FarmManagementServices => "Farm Management Services",
            Self::SupportActivitiesForAnimalProduction => {
                "Support Activities for Animal Production"
            }
            Self::SupportActivitiesForForestry => "Support Activities for Forestry",
            Self::CrudePetroleumExtraction => "Crude Petroleum Extraction",
            Self::NaturalGasExtraction => "Natural Gas Extraction",
            Self::SurfaceCoalMining => "Surface Coal Mining",
            Self::UndergroundCoalMining => "Underground Coal Mining",
            Self::IronOreMining => "Iron Ore Mining",
            Self::GoldOreAndSilverOreMining => "Gold Ore and Silver Ore Mining",
            Self::CopperNickelLeadAndZincMining => "Copper, Nickel, Lead, and Zinc Mining",
            Self::OtherMetalOreMining => "Other Metal Ore Mining",
            Self::DimensionStoneMiningAndQuarrying => "Dimension Stone Mining and Quarrying",
            Self::CrushedAndBrokenLimestoneMiningAndQuarrying => {
                "Crushed and Broken Limestone Mining and Quarrying"
            }
            Self::CrushedAndBrokenGraniteMiningAndQuarrying => {
                "Crushed and Broken Granite Mining and Quarrying"
            }
            Self::OtherCrushedAndBrokenStoneMiningAndQuarrying => {
                "Other Crushed and Broken Stone Mining and Quarrying"
            }
            Self::ConstructionSandAndGravelMining => "Construction Sand and Gravel Mining",
            Self::IndustrialSandMining => "Industrial Sand Mining",
            Self::KaolinClayAndCeramicAndRefractoryMineralsMining => {
                "Kaolin, Clay, and Ceramic and Refractory Minerals Mining"
            }
            Self::OtherNonmetallicMineralMiningAndQuarrying => {
                "Other Nonmetallic Mineral Mining and Quarrying"
            }
            Self::DrillingOilAndGasWells => "Drilling Oil and Gas Wells",
            Self::SupportActivitiesForOilAndGasOperations => {
                "Support Activities for Oil and Gas Operations"
            }
            Self::SupportActivitiesForCoalMining => "Support Activities for Coal Mining",
            Self::SupportActivitiesForMetalMining => "Support Activities for Metal Mining",
            Self::SupportActivitiesForNonmetallicMineralsExceptFuelsMining => {
                "Support Activities for Nonmetallic Minerals (except Fuels) Mining"
            }
            Self::HydroelectricPowerGeneration => "Hydroelectric Power Generation",
            Self::FossilFuelElectricPowerGeneration => "Fossil Fuel Electric Power Generation",
            Self::NuclearElectricPowerGeneration => "Nuclear Electric Power Generation",
            Self::SolarElectricPowerGeneration => "Solar Electric Power Generation",
            Self::WindElectricPowerGeneration => "Wind Electric Power Generation",
            Self::GeothermalElectricPowerGeneration => "Geothermal Electric Power Generation",
            Self::BiomassElectricPowerGeneration => "Biomass Electric Power Generation",
            Self::OtherElectricPowerGeneration => "Other Electric Power Generation",
            Self::ElectricBulkPowerTransmissionAndControl => {
                "Electric Bulk Power Transmission and Control"
            }
            Self::ElectricPowerDistribution => "Electric Power Distribution",
            Self::NaturalGasDistribution => "Natural Gas Distribution",
            Self::WaterSupplyAndIrrigationSystems => "Water Supply and Irrigation Systems",
            Self::SewageTreatmentFacilities => "Sewage Treatment Facilities",
            Self::SteamAndAirconditioningSupply => "Steam and Air-Conditioning Supply",
            Self::NewSinglefamilyHousingConstructionExceptForsaleBuilders => {
                "New Single-Family Housing Construction (except For-Sale Builders)"
            }
            Self::NewMultifamilyHousingConstructionExceptForsaleBuilders => {
                "New Multifamily Housing Construction (except For-Sale Builders)"
            }
            Self::NewHousingForsaleBuilders => "New Housing For-Sale Builders",
            Self::ResidentialRemodelers => "Residential Remodelers",
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
                "Poured Concrete Foundation and Structure Contractors"
            }
            Self::StructuralSteelAndPrecastConcreteContractors => {
                "Structural Steel and Precast Concrete Contractors"
            }
            Self::FramingContractors => "Framing Contractors",
            Self::MasonryContractors => "Masonry Contractors",
            Self::GlassAndGlazingContractors => "Glass and Glazing Contractors",
            Self::RoofingContractors => "Roofing Contractors",
            Self::SidingContractors => "Siding Contractors",
            Self::OtherFoundationStructureAndBuildingExteriorContractors => {
                "Other Foundation, Structure, and Building Exterior Contractors"
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
            Self::DogAndCatFoodManufacturing => "Dog and Cat Food Manufacturing",
            Self::OtherAnimalFoodManufacturing => "Other Animal Food Manufacturing",
            Self::FlourMilling => "Flour Milling",
            Self::RiceMilling => "Rice Milling",
            Self::MaltManufacturing => "Malt Manufacturing",
            Self::WetCornMillingAndStarchManufacturing => {
                "Wet Corn Milling and Starch Manufacturing"
            }
            Self::SoybeanAndOtherOilseedProcessing => "Soybean and Other Oilseed Processing",
            Self::FatsAndOilsRefiningAndBlending => "Fats and Oils Refining and Blending",
            Self::BreakfastCerealManufacturing => "Breakfast Cereal Manufacturing",
            Self::BeetSugarManufacturing => "Beet Sugar Manufacturing",
            Self::CaneSugarManufacturing => "Cane Sugar Manufacturing",
            Self::NonchocolateConfectioneryManufacturing => {
                "Nonchocolate Confectionery Manufacturing"
            }
            Self::ChocolateAndConfectioneryManufacturingFromCacaoBeans => {
                "Chocolate and Confectionery Manufacturing from Cacao Beans"
            }
            Self::ConfectioneryManufacturingFromPurchasedChocolate => {
                "Confectionery Manufacturing from Purchased Chocolate"
            }
            Self::FrozenFruitJuiceAndVegetableManufacturing => {
                "Frozen Fruit, Juice, and Vegetable Manufacturing"
            }
            Self::FrozenSpecialtyFoodManufacturing => "Frozen Specialty Food Manufacturing",
            Self::FruitAndVegetableCanning => "Fruit and Vegetable Canning",
            Self::SpecialtyCanning => "Specialty Canning",
            Self::DriedAndDehydratedFoodManufacturing => "Dried and Dehydrated Food Manufacturing",
            Self::FluidMilkManufacturing => "Fluid Milk Manufacturing",
            Self::CreameryButterManufacturing => "Creamery Butter Manufacturing",
            Self::CheeseManufacturing => "Cheese Manufacturing",
            Self::DryCondensedAndEvaporatedDairyProductManufacturing => {
                "Dry, Condensed, and Evaporated Dairy Product Manufacturing"
            }
            Self::IceCreamAndFrozenDessertManufacturing => {
                "Ice Cream and Frozen Dessert Manufacturing"
            }
            Self::AnimalExceptPoultrySlaughtering => "Animal (except Poultry) Slaughtering",
            Self::MeatProcessedFromCarcasses => "Meat Processed from Carcasses",
            Self::RenderingAndMeatByproductProcessing => "Rendering and Meat Byproduct Processing",
            Self::PoultryProcessing => "Poultry Processing",
            Self::SeafoodProductPreparationAndPackaging => {
                "Seafood Product Preparation and Packaging"
            }
            Self::RetailBakeries => "Retail Bakeries",
            Self::CommercialBakeries => "Commercial Bakeries",
            Self::FrozenCakesPiesAndOtherPastriesManufacturing => {
                "Frozen Cakes, Pies, and Other Pastries Manufacturing"
            }
            Self::CookieAndCrackerManufacturing => "Cookie and Cracker Manufacturing",
            Self::DryPastaDoughAndFlourMixesManufacturingFromPurchasedFlour => {
                "Dry Pasta, Dough, and Flour Mixes Manufacturing from Purchased Flour"
            }
            Self::TortillaManufacturing => "Tortilla Manufacturing",
            Self::RoastedNutsAndPeanutButterManufacturing => {
                "Roasted Nuts and Peanut Butter Manufacturing"
            }
            Self::OtherSnackFoodManufacturing => "Other Snack Food Manufacturing",
            Self::CoffeeAndTeaManufacturing => "Coffee and Tea Manufacturing",
            Self::FlavoringSyrupAndConcentrateManufacturing => {
                "Flavoring Syrup and Concentrate Manufacturing"
            }
            Self::MayonnaiseDressingAndOtherPreparedSauceManufacturing => {
                "Mayonnaise, Dressing, and Other Prepared Sauce Manufacturing"
            }
            Self::SpiceAndExtractManufacturing => "Spice and Extract Manufacturing",
            Self::PerishablePreparedFoodManufacturing => "Perishable Prepared Food Manufacturing",
            Self::AllOtherMiscellaneousFoodManufacturing => {
                "All Other Miscellaneous Food Manufacturing"
            }
            Self::SoftDrinkManufacturing => "Soft Drink Manufacturing",
            Self::BottledWaterManufacturing => "Bottled Water Manufacturing",
            Self::IceManufacturing => "Ice Manufacturing",
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
            Self::RopeCordageTwineTireCordAndTireFabricMills => {
                "Rope, Cordage, Twine, Tire Cord, and Tire Fabric Mills"
            }
            Self::AllOtherMiscellaneousTextileProductMills => {
                "All Other Miscellaneous Textile Product Mills"
            }
            Self::ApparelKnittingMills => "Apparel Knitting Mills",
            Self::CutAndSewApparelContractors => "Cut and Sew Apparel Contractors",
            Self::CutAndSewApparelManufacturingExceptContractors => {
                "Cut and Sew Apparel Manufacturing (except Contractors)"
            }
            Self::ApparelAccessoriesAndOtherApparelManufacturing => {
                "Apparel Accessories and Other Apparel Manufacturing"
            }
            Self::LeatherAndHideTanningAndFinishing => "Leather and Hide Tanning and Finishing",
            Self::FootwearManufacturing => "Footwear Manufacturing",
            Self::OtherLeatherAndAlliedProductManufacturing => {
                "Other Leather and Allied Product Manufacturing"
            }
            Self::Sawmills => "Sawmills",
            Self::WoodPreservation => "Wood Preservation",
            Self::HardwoodVeneerAndPlywoodManufacturing => {
                "Hardwood Veneer and Plywood Manufacturing"
            }
            Self::SoftwoodVeneerAndPlywoodManufacturing => {
                "Softwood Veneer and Plywood Manufacturing"
            }
            Self::EngineeredWoodMemberManufacturing => "Engineered Wood Member Manufacturing",
            Self::ReconstitutedWoodProductManufacturing => {
                "Reconstituted Wood Product Manufacturing"
            }
            Self::WoodWindowAndDoorManufacturing => "Wood Window and Door Manufacturing",
            Self::CutStockResawingLumberAndPlaning => "Cut Stock, Resawing Lumber, and Planing",
            Self::OtherMillworkIncludingFlooring => "Other Millwork (including Flooring)",
            Self::WoodContainerAndPalletManufacturing => "Wood Container and Pallet Manufacturing",
            Self::ManufacturedHomeMobileHomeManufacturing => {
                "Manufactured Home (Mobile Home) Manufacturing"
            }
            Self::PrefabricatedWoodBuildingManufacturing => {
                "Prefabricated Wood Building Manufacturing"
            }
            Self::AllOtherMiscellaneousWoodProductManufacturing => {
                "All Other Miscellaneous Wood Product Manufacturing"
            }
            Self::PulpMills => "Pulp Mills",
            Self::PaperMills => "Paper Mills",
            Self::PaperboardMills => "Paperboard Mills",
            Self::CorrugatedAndSolidFiberBoxManufacturing => {
                "Corrugated and Solid Fiber Box Manufacturing"
            }
            Self::FoldingPaperboardBoxManufacturing => "Folding Paperboard Box Manufacturing",
            Self::OtherPaperboardContainerManufacturing => {
                "Other Paperboard Container Manufacturing"
            }
            Self::PaperBagAndCoatedAndTreatedPaperManufacturing => {
                "Paper Bag and Coated and Treated Paper Manufacturing"
            }
            Self::StationeryProductManufacturing => "Stationery Product Manufacturing",
            Self::SanitaryPaperProductManufacturing => "Sanitary Paper Product Manufacturing",
            Self::AllOtherConvertedPaperProductManufacturing => {
                "All Other Converted Paper Product Manufacturing"
            }
            Self::CommercialPrintingExceptScreenAndBooks => {
                "Commercial Printing (except Screen and Books)"
            }
            Self::CommercialScreenPrinting => "Commercial Screen Printing",
            Self::BooksPrinting => "Books Printing",
            Self::SupportActivitiesForPrinting => "Support Activities for Printing",
            Self::PetroleumRefineries => "Petroleum Refineries",
            Self::AsphaltPavingMixtureAndBlockManufacturing => {
                "Asphalt Paving Mixture and Block Manufacturing"
            }
            Self::AsphaltShingleAndCoatingMaterialsManufacturing => {
                "Asphalt Shingle and Coating Materials Manufacturing"
            }
            Self::PetroleumLubricatingOilAndGreaseManufacturing => {
                "Petroleum Lubricating Oil and Grease Manufacturing"
            }
            Self::AllOtherPetroleumAndCoalProductsManufacturing => {
                "All Other Petroleum and Coal Products Manufacturing"
            }
            Self::PetrochemicalManufacturing => "Petrochemical Manufacturing",
            Self::IndustrialGasManufacturing => "Industrial Gas Manufacturing",
            Self::SyntheticDyeAndPigmentManufacturing => "Synthetic Dye and Pigment Manufacturing",
            Self::OtherBasicInorganicChemicalManufacturing => {
                "Other Basic Inorganic Chemical Manufacturing"
            }
            Self::EthylAlcoholManufacturing => "Ethyl Alcohol Manufacturing",
            Self::CyclicCrudeIntermediateAndGumAndWoodChemicalManufacturing => {
                "Cyclic Crude, Intermediate, and Gum and Wood Chemical Manufacturing "
            }
            Self::AllOtherBasicOrganicChemicalManufacturing => {
                "All Other Basic Organic Chemical Manufacturing "
            }
            Self::PlasticsMaterialAndResinManufacturing => {
                "Plastics Material and Resin Manufacturing "
            }
            Self::SyntheticRubberManufacturing => "Synthetic Rubber Manufacturing ",
            Self::ArtificialAndSyntheticFibersAndFilamentsManufacturing => {
                "Artificial and Synthetic Fibers and Filaments Manufacturing"
            }
            Self::NitrogenousFertilizerManufacturing => "Nitrogenous Fertilizer Manufacturing ",
            Self::PhosphaticFertilizerManufacturing => "Phosphatic Fertilizer Manufacturing ",
            Self::FertilizerMixingOnlyManufacturing => "Fertilizer (Mixing Only) Manufacturing ",
            Self::CompostManufacturing => "Compost Manufacturing",
            Self::PesticideAndOtherAgriculturalChemicalManufacturing => {
                "Pesticide and Other Agricultural Chemical Manufacturing"
            }
            Self::MedicinalAndBotanicalManufacturing => "Medicinal and Botanical Manufacturing ",
            Self::PharmaceuticalPreparationManufacturing => {
                "Pharmaceutical Preparation Manufacturing "
            }
            Self::InvitroDiagnosticSubstanceManufacturing => {
                "In-Vitro Diagnostic Substance Manufacturing "
            }
            Self::BiologicalProductExceptDiagnosticManufacturing => {
                "Biological Product (except Diagnostic) Manufacturing "
            }
            Self::PaintAndCoatingManufacturing => "Paint and Coating Manufacturing",
            Self::AdhesiveManufacturing => "Adhesive Manufacturing",
            Self::SoapAndOtherDetergentManufacturing => "Soap and Other Detergent Manufacturing ",
            Self::PolishAndOtherSanitationGoodManufacturing => {
                "Polish and Other Sanitation Good Manufacturing "
            }
            Self::SurfaceActiveAgentManufacturing => "Surface Active Agent Manufacturing ",
            Self::ToiletPreparationManufacturing => "Toilet Preparation Manufacturing",
            Self::PrintingInkManufacturing => "Printing Ink Manufacturing",
            Self::ExplosivesManufacturing => "Explosives Manufacturing",
            Self::CustomCompoundingOfPurchasedResins => "Custom Compounding of Purchased Resins ",
            Self::PhotographicFilmPaperPlateChemicalAndCopyTonerManufacturing => {
                "Photographic Film, Paper, Plate, Chemical, and Copy Toner Manufacturing "
            }
            Self::AllOtherMiscellaneousChemicalProductAndPreparationManufacturing => {
                "All Other Miscellaneous Chemical Product and Preparation Manufacturing "
            }
            Self::PlasticsBagAndPouchManufacturing => "Plastics Bag and Pouch Manufacturing ",
            Self::PlasticsPackagingFilmAndSheetIncludingLaminatedManufacturing => {
                "Plastics Packaging Film and Sheet (including Laminated) Manufacturing "
            }
            Self::UnlaminatedPlasticsFilmAndSheetExceptPackagingManufacturing => {
                "Unlaminated Plastics Film and Sheet (except Packaging) Manufacturing "
            }
            Self::UnlaminatedPlasticsProfileShapeManufacturing => {
                "Unlaminated Plastics Profile Shape Manufacturing "
            }
            Self::PlasticsPipeAndPipeFittingManufacturing => {
                "Plastics Pipe and Pipe Fitting Manufacturing "
            }
            Self::LaminatedPlasticsPlateSheetExceptPackagingAndShapeManufacturing => {
                "Laminated Plastics Plate, Sheet (except Packaging), and Shape Manufacturing"
            }
            Self::PolystyreneFoamProductManufacturing => "Polystyrene Foam Product Manufacturing",
            Self::UrethaneAndOtherFoamProductExceptPolystyreneManufacturing => {
                "Urethane and Other Foam Product (except Polystyrene) Manufacturing"
            }
            Self::PlasticsBottleManufacturing => "Plastics Bottle Manufacturing",
            Self::PlasticsPlumbingFixtureManufacturing => {
                "Plastics Plumbing Fixture Manufacturing "
            }
            Self::AllOtherPlasticsProductManufacturing => {
                "All Other Plastics Product Manufacturing "
            }
            Self::TireManufacturingExceptRetreading => "Tire Manufacturing (except Retreading) ",
            Self::TireRetreading => "Tire Retreading ",
            Self::RubberAndPlasticsHosesAndBeltingManufacturing => {
                "Rubber and Plastics Hoses and Belting Manufacturing"
            }
            Self::RubberProductManufacturingForMechanicalUse => {
                "Rubber Product Manufacturing for Mechanical Use "
            }
            Self::AllOtherRubberProductManufacturing => "All Other Rubber Product Manufacturing ",
            Self::PotteryCeramicsAndPlumbingFixtureManufacturing => {
                "Pottery, Ceramics, and Plumbing Fixture Manufacturing "
            }
            Self::ClayBuildingMaterialAndRefractoriesManufacturing => {
                "Clay Building Material and Refractories Manufacturing "
            }
            Self::FlatGlassManufacturing => "Flat Glass Manufacturing ",
            Self::OtherPressedAndBlownGlassAndGlasswareManufacturing => {
                "Other Pressed and Blown Glass and Glassware Manufacturing "
            }
            Self::GlassContainerManufacturing => "Glass Container Manufacturing ",
            Self::GlassProductManufacturingMadeOfPurchasedGlass => {
                "Glass Product Manufacturing Made of Purchased Glass "
            }
            Self::CementManufacturing => "Cement Manufacturing",
            Self::ReadymixConcreteManufacturing => "Ready-Mix Concrete Manufacturing",
            Self::ConcreteBlockAndBrickManufacturing => "Concrete Block and Brick Manufacturing ",
            Self::ConcretePipeManufacturing => "Concrete Pipe Manufacturing ",
            Self::OtherConcreteProductManufacturing => "Other Concrete Product Manufacturing ",
            Self::LimeManufacturing => "Lime Manufacturing",
            Self::GypsumProductManufacturing => "Gypsum Product Manufacturing",
            Self::AbrasiveProductManufacturing => "Abrasive Product Manufacturing",
            Self::CutStoneAndStoneProductManufacturing => "Cut Stone and Stone Product Manufacturing",
            Self::GroundOrTreatedMineralAndEarthManufacturing => "Ground or Treated Mineral and Earth Manufacturing",
            Self::MineralWoolManufacturing => "Mineral Wool Manufacturing",
            Self::AllOtherMiscellaneousNonmetallicMineralProductManufacturing => "All Other Miscellaneous Nonmetallic Mineral Product Manufacturing",
            Self::IronAndSteelMillsAndFerroalloyManufacturing => "Iron and Steel Mills and Ferroalloy Manufacturing",
            Self::IronAndSteelPipeAndTubeManufacturingFromPurchasedSteel => "Iron and Steel Pipe and Tube Manufacturing from Purchased Steel",
            Self::RolledSteelShapeManufacturing => "Rolled Steel Shape Manufacturing",
            Self::SteelWireDrawing => "Steel Wire Drawing",
            Self::AluminaRefiningAndPrimaryAluminumProduction => "Alumina Refining and Primary Aluminum Production",
            Self::SecondarySmeltingAndAlloyingOfAluminum => "Secondary Smelting and Alloying of Aluminum",
            Self::AluminumSheetPlateAndFoilManufacturing => "Aluminum Sheet, Plate, and Foil Manufacturing",
            Self::OtherAluminumRollingDrawingAndExtruding => "Other Aluminum Rolling, Drawing, and Extruding",
            Self::NonferrousMetalExceptAluminumSmeltingAndRefining => "Nonferrous Metal (except Aluminum) Smelting and Refining",
            Self::CopperRollingDrawingExtrudingAndAlloying => "Copper Rolling, Drawing, Extruding, and Alloying",
            Self::NonferrousMetalExceptCopperAndAluminumRollingDrawingAndExtruding => "Nonferrous Metal (except Copper and Aluminum) Rolling, Drawing, and Extruding",
            Self::SecondarySmeltingRefiningAndAlloyingOfNonferrousMetalExceptCopperAndAluminum => "Secondary Smelting, Refining, and Alloying of Nonferrous Metal (except Copper and Aluminum)",
            Self::IronFoundries => "Iron Foundries",
            Self::SteelInvestmentFoundries => "Steel Investment Foundries",
            Self::SteelFoundriesExceptInvestment => "Steel Foundries (except Investment)",
            Self::NonferrousMetalDiecastingFoundries => "Nonferrous Metal Die-Casting Foundries",
            Self::AluminumFoundriesExceptDiecasting => "Aluminum Foundries (except Die-Casting)",
            Self::OtherNonferrousMetalFoundriesExceptDiecasting => "Other Nonferrous Metal Foundries (except Die-Casting)",
            Self::IronAndSteelForging => "Iron and Steel Forging",
            Self::NonferrousForging => "Nonferrous Forging",
            Self::CustomRollForming => "Custom Roll Forming",
            Self::PowderMetallurgyPartManufacturing => "Powder Metallurgy Part Manufacturing",
            Self::MetalCrownClosureAndOtherMetalStampingExceptAutomotive => "Metal Crown, Closure, and Other Metal Stamping (except Automotive)",
            Self::MetalKitchenCookwareUtensilCutleryAndFlatwareExceptPreciousManufacturing => "Metal Kitchen Cookware, Utensil, Cutlery, and Flatware (except Precious) Manufacturing",
            Self::SawBladeAndHandtoolManufacturing => "Saw Blade and Handtool Manufacturing",
            Self::PrefabricatedMetalBuildingAndComponentManufacturing => "Prefabricated Metal Building and Component Manufacturing",
            Self::FabricatedStructuralMetalManufacturing => "Fabricated Structural Metal Manufacturing",
            Self::PlateWorkManufacturing => "Plate Work Manufacturing",
            Self::MetalWindowAndDoorManufacturing => "Metal Window and Door Manufacturing",
            Self::SheetMetalWorkManufacturing => "Sheet Metal Work Manufacturing",
            Self::OrnamentalAndArchitecturalMetalWorkManufacturing => "Ornamental and Architectural Metal Work Manufacturing",
            Self::PowerBoilerAndHeatExchangerManufacturing => "Power Boiler and Heat Exchanger Manufacturing",
            Self::MetalTankHeavyGaugeManufacturing => "Metal Tank (Heavy Gauge) Manufacturing",
            Self::MetalCanManufacturing => "Metal Can Manufacturing",
            Self::OtherMetalContainerManufacturing => "Other Metal Container Manufacturing",
            Self::HardwareManufacturing => "Hardware Manufacturing",
            Self::SpringManufacturing => "Spring Manufacturing",
            Self::OtherFabricatedWireProductManufacturing => "Other Fabricated Wire Product Manufacturing",
            Self::MachineShops => "Machine Shops",
            Self::PrecisionTurnedProductManufacturing => "Precision Turned Product Manufacturing",
            Self::BoltNutScrewRivetAndWasherManufacturing => "Bolt, Nut, Screw, Rivet, and Washer Manufacturing",
            Self::MetalHeatTreating => "Metal Heat Treating",
            Self::MetalCoatingEngravingExceptJewelryAndSilverwareAndAlliedServicesToManufacturers => "Metal Coating, Engraving (except Jewelry and Silverware), and Allied Services to Manufacturers",
            Self::ElectroplatingPlatingPolishingAnodizingAndColoring => "Electroplating, Plating, Polishing, Anodizing, and Coloring",
            Self::IndustrialValveManufacturing => "Industrial Valve Manufacturing",
            Self::FluidPowerValveAndHoseFittingManufacturing => "Fluid Power Valve and Hose Fitting Manufacturing",
            Self::PlumbingFixtureFittingAndTrimManufacturing => "Plumbing Fixture Fitting and Trim Manufacturing",
            Self::OtherMetalValveAndPipeFittingManufacturing => "Other Metal Valve and Pipe Fitting Manufacturing",
            Self::BallAndRollerBearingManufacturing => "Ball and Roller Bearing Manufacturing",
            Self::SmallArmsAmmunitionManufacturing => "Small Arms Ammunition Manufacturing",
            Self::AmmunitionExceptSmallArmsManufacturing => "Ammunition (except Small Arms) Manufacturing",
            Self::SmallArmsOrdnanceAndOrdnanceAccessoriesManufacturing => "Small Arms, Ordnance, and Ordnance Accessories Manufacturing",
            Self::FabricatedPipeAndPipeFittingManufacturing => "Fabricated Pipe and Pipe Fitting Manufacturing",
            Self::AllOtherMiscellaneousFabricatedMetalProductManufacturing => "All Other Miscellaneous Fabricated Metal Product Manufacturing",
            Self::FarmMachineryAndEquipmentManufacturing => "Farm Machinery and Equipment Manufacturing",
            Self::LawnAndGardenTractorAndHomeLawnAndGardenEquipmentManufacturing => "Lawn and Garden Tractor and Home Lawn and Garden Equipment Manufacturing",
            Self::ConstructionMachineryManufacturing => "Construction Machinery Manufacturing",
            Self::MiningMachineryAndEquipmentManufacturing => "Mining Machinery and Equipment Manufacturing",
            Self::OilAndGasFieldMachineryAndEquipmentManufacturing => "Oil and Gas Field Machinery and Equipment Manufacturing",
            Self::FoodProductMachineryManufacturing => "Food Product Machinery Manufacturing",
            Self::SemiconductorMachineryManufacturing => "Semiconductor Machinery Manufacturing",
            Self::SawmillWoodworkingAndPaperMachineryManufacturing => "Sawmill, Woodworking, and Paper Machinery Manufacturing",
            Self::AllOtherIndustrialMachineryManufacturing => "All Other Industrial Machinery Manufacturing",
            Self::CommercialAndServiceIndustryMachineryManufacturing => "Commercial and Service Industry Machinery Manufacturing",
            Self::IndustrialAndCommercialFanAndBlowerAndAirPurificationEquipmentManufacturing => "Industrial and Commercial Fan and Blower and Air Purification Equipment Manufacturing",
            Self::HeatingEquipmentExceptWarmAirFurnacesManufacturing => "Heating Equipment (except Warm Air Furnaces) Manufacturing",
            Self::AirconditioningAndWarmAirHeatingEquipmentAndCommercialAndIndustrialRefrigerationEquipmentManufacturing => "Air-Conditioning and Warm Air Heating Equipment and Commercial and Industrial Refrigeration Equipment Manufacturing",
            Self::IndustrialMoldManufacturing => "Industrial Mold Manufacturing",
            Self::SpecialDieAndToolDieSetJigAndFixtureManufacturing => "Special Die and Tool, Die Set, Jig, and Fixture Manufacturing",
            Self::CuttingToolAndMachineToolAccessoryManufacturing => "Cutting Tool and Machine Tool Accessory Manufacturing",
            Self::MachineToolManufacturing => "Machine Tool Manufacturing",
            Self::RollingMillAndOtherMetalworkingMachineryManufacturing => "Rolling Mill and Other Metalworking Machinery Manufacturing",
            Self::TurbineAndTurbineGeneratorSetUnitsManufacturing => "Turbine and Turbine Generator Set Units Manufacturing",
            Self::SpeedChangerIndustrialHighspeedDriveAndGearManufacturing => "Speed Changer, Industrial High-Speed Drive, and Gear Manufacturing",
            Self::MechanicalPowerTransmissionEquipmentManufacturing => "Mechanical Power Transmission Equipment Manufacturing",
            Self::OtherEngineEquipmentManufacturing => "Other Engine Equipment Manufacturing",
            Self::AirAndGasCompressorManufacturing => "Air and Gas Compressor Manufacturing",
            Self::MeasuringDispensingAndOtherPumpingEquipmentManufacturing => "Measuring, Dispensing, and Other Pumping Equipment Manufacturing",
            Self::ElevatorAndMovingStairwayManufacturing => "Elevator and Moving Stairway Manufacturing",
            Self::ConveyorAndConveyingEquipmentManufacturing => "Conveyor and Conveying Equipment Manufacturing",
            Self::OverheadTravelingCraneHoistAndMonorailSystemManufacturing => "Overhead Traveling Crane, Hoist, and Monorail System Manufacturing",
            Self::IndustrialTruckTractorTrailerAndStackerMachineryManufacturing => "Industrial Truck, Tractor, Trailer, and Stacker Machinery Manufacturing",
            Self::PowerdrivenHandtoolManufacturing => "Power-Driven Handtool Manufacturing",
            Self::WeldingAndSolderingEquipmentManufacturing => "Welding and Soldering Equipment Manufacturing",
            Self::PackagingMachineryManufacturing => "Packaging Machinery Manufacturing",
            Self::IndustrialProcessFurnaceAndOvenManufacturing => "Industrial Process Furnace and Oven Manufacturing",
            Self::FluidPowerCylinderAndActuatorManufacturing => "Fluid Power Cylinder and Actuator Manufacturing",
            Self::FluidPowerPumpAndMotorManufacturing => "Fluid Power Pump and Motor Manufacturing",
            Self::AllOtherMiscellaneousGeneralPurposeMachineryManufacturing => "All Other Miscellaneous General Purpose Machinery Manufacturing",
            Self::ElectronicComputerManufacturing => "Electronic Computer Manufacturing",
            Self::ComputerStorageDeviceManufacturing => "Computer Storage Device Manufacturing",
            Self::ComputerTerminalAndOtherComputerPeripheralEquipmentManufacturing => "Computer Terminal and Other Computer Peripheral Equipment Manufacturing",
            Self::TelephoneApparatusManufacturing => "Telephone Apparatus Manufacturing",
            Self::RadioAndTelevisionBroadcastingAndWirelessCommunicationsEquipmentManufacturing => "Radio and Television Broadcasting and Wireless Communications Equipment Manufacturing",
            Self::OtherCommunicationsEquipmentManufacturing => "Other Communications Equipment Manufacturing",
            Self::AudioAndVideoEquipmentManufacturing => "Audio and Video Equipment Manufacturing",
            Self::BarePrintedCircuitBoardManufacturing => "Bare Printed Circuit Board Manufacturing  ",
            Self::SemiconductorAndRelatedDeviceManufacturing => "Semiconductor and Related Device Manufacturing ",
            Self::CapacitorResistorCoilTransformerAndOtherInductorManufacturing => "Capacitor, Resistor, Coil, Transformer, and Other Inductor Manufacturing ",
            Self::ElectronicConnectorManufacturing => "Electronic Connector Manufacturing ",
            Self::PrintedCircuitAssemblyElectronicAssemblyManufacturing => "Printed Circuit Assembly (Electronic Assembly) Manufacturing ",
            Self::OtherElectronicComponentManufacturing => "Other Electronic Component Manufacturing ",
            Self::ElectromedicalAndElectrotherapeuticApparatusManufacturing => "Electromedical and Electrotherapeutic Apparatus Manufacturing ",
            Self::SearchDetectionNavigationGuidanceAeronauticalAndNauticalSystemAndInstrumentManufacturing => "Search, Detection, Navigation, Guidance, Aeronautical, and Nautical System and Instrument Manufacturing ",
            Self::AutomaticEnvironmentalControlManufacturingForResidentialCommercialAndApplianceUse => "Automatic Environmental Control Manufacturing for Residential, Commercial, and Appliance Use ",
            Self::InstrumentsAndRelatedProductsManufacturingForMeasuringDisplayingAndControllingIndustrialProcessVariables => "Instruments and Related Products Manufacturing for Measuring, Displaying, and Controlling Industrial Process Variables ",
            Self::TotalizingFluidMeterAndCountingDeviceManufacturing => "Totalizing Fluid Meter and Counting Device Manufacturing ",
            Self::InstrumentManufacturingForMeasuringAndTestingElectricityAndElectricalSignals => "Instrument Manufacturing for Measuring and Testing Electricity and Electrical Signals ",
            Self::AnalyticalLaboratoryInstrumentManufacturing => "Analytical Laboratory Instrument Manufacturing ",
            Self::IrradiationApparatusManufacturing => "Irradiation Apparatus Manufacturing ",
            Self::OtherMeasuringAndControllingDeviceManufacturing => "Other Measuring and Controlling Device Manufacturing ",
            Self::ManufacturingAndReproducingMagneticAndOpticalMedia => "Manufacturing and Reproducing Magnetic and Optical Media ",
            Self::ResidentialElectricLightingFixtureManufacturing => "Residential Electric Lighting Fixture Manufacturing ",
            Self::CommercialIndustrialAndInstitutionalElectricLightingFixtureManufacturing => "Commercial, Industrial, and Institutional Electric Lighting Fixture Manufacturing ",
            Self::ElectricLampBulbAndOtherLightingEquipmentManufacturing => "Electric Lamp Bulb and Other Lighting Equipment Manufacturing ",
            Self::SmallElectricalApplianceManufacturing => "Small Electrical Appliance Manufacturing",
            Self::MajorHouseholdApplianceManufacturing => "Major Household Appliance Manufacturing ",
            Self::PowerDistributionAndSpecialtyTransformerManufacturing => "Power, Distribution, and Specialty Transformer Manufacturing ",
            Self::MotorAndGeneratorManufacturing => "Motor and Generator Manufacturing ",
            Self::SwitchgearAndSwitchboardApparatusManufacturing => "Switchgear and Switchboard Apparatus Manufacturing ",
            Self::RelayAndIndustrialControlManufacturing => "Relay and Industrial Control Manufacturing ",
            Self::BatteryManufacturing => "Battery Manufacturing ",
            Self::FiberOpticCableManufacturing => "Fiber Optic Cable Manufacturing ",
            Self::OtherCommunicationAndEnergyWireManufacturing => "Other Communication and Energy Wire Manufacturing ",
            Self::CurrentcarryingWiringDeviceManufacturing => "Current-Carrying Wiring Device Manufacturing ",
            Self::NoncurrentcarryingWiringDeviceManufacturing => "Noncurrent-Carrying Wiring Device Manufacturing ",
            Self::CarbonAndGraphiteProductManufacturing => "Carbon and Graphite Product Manufacturing ",
            Self::AllOtherMiscellaneousElectricalEquipmentAndComponentManufacturing => "All Other Miscellaneous Electrical Equipment and Component Manufacturing ",
            Self::AutomobileAndLightDutyMotorVehicleManufacturing => "Automobile and Light Duty Motor Vehicle Manufacturing ",
            Self::HeavyDutyTruckManufacturing => "Heavy Duty Truck Manufacturing",
            Self::MotorVehicleBodyManufacturing => "Motor Vehicle Body Manufacturing ",
            Self::TruckTrailerManufacturing => "Truck Trailer Manufacturing ",
            Self::MotorHomeManufacturing => "Motor Home Manufacturing ",
            Self::TravelTrailerAndCamperManufacturing => "Travel Trailer and Camper Manufacturing ",
            Self::MotorVehicleGasolineEngineAndEnginePartsManufacturing => "Motor Vehicle Gasoline Engine and Engine Parts Manufacturing",
            Self::MotorVehicleElectricalAndElectronicEquipmentManufacturing => "Motor Vehicle Electrical and Electronic Equipment Manufacturing",
            Self::MotorVehicleSteeringAndSuspensionComponentsExceptSpringManufacturing => "Motor Vehicle Steering and Suspension Components (except Spring) Manufacturing",
            Self::MotorVehicleBrakeSystemManufacturing => "Motor Vehicle Brake System Manufacturing",
            Self::MotorVehicleTransmissionAndPowerTrainPartsManufacturing => "Motor Vehicle Transmission and Power Train Parts Manufacturing",
            Self::MotorVehicleSeatingAndInteriorTrimManufacturing => "Motor Vehicle Seating and Interior Trim Manufacturing",
            Self::MotorVehicleMetalStamping => "Motor Vehicle Metal Stamping",
            Self::OtherMotorVehiclePartsManufacturing => "Other Motor Vehicle Parts Manufacturing",
            Self::AircraftManufacturing => "Aircraft Manufacturing ",
            Self::AircraftEngineAndEnginePartsManufacturing => "Aircraft Engine and Engine Parts Manufacturing ",
            Self::OtherAircraftPartsAndAuxiliaryEquipmentManufacturing => "Other Aircraft Parts and Auxiliary Equipment Manufacturing ",
            Self::GuidedMissileAndSpaceVehicleManufacturing => "Guided Missile and Space Vehicle Manufacturing ",
            Self::GuidedMissileAndSpaceVehiclePropulsionUnitAndPropulsionUnitPartsManufacturing => "Guided Missile and Space Vehicle Propulsion Unit and Propulsion Unit Parts Manufacturing ",
            Self::OtherGuidedMissileAndSpaceVehiclePartsAndAuxiliaryEquipmentManufacturing => "Other Guided Missile and Space Vehicle Parts and Auxiliary Equipment Manufacturing ",
            Self::RailroadRollingStockManufacturing => "Railroad Rolling Stock Manufacturing",
            Self::ShipBuildingAndRepairing => "Ship Building and Repairing ",
            Self::BoatBuilding => "Boat Building ",
            Self::MotorcycleBicycleAndPartsManufacturing => "Motorcycle, Bicycle, and Parts Manufacturing ",
            Self::MilitaryArmoredVehicleTankAndTankComponentManufacturing => "Military Armored Vehicle, Tank, and Tank Component Manufacturing ",
            Self::AllOtherTransportationEquipmentManufacturing => "All Other Transportation Equipment Manufacturing ",
            Self::WoodKitchenCabinetAndCountertopManufacturing => "Wood Kitchen Cabinet and Countertop Manufacturing",
            Self::UpholsteredHouseholdFurnitureManufacturing => "Upholstered Household Furniture Manufacturing ",
            Self::NonupholsteredWoodHouseholdFurnitureManufacturing => "Nonupholstered Wood Household Furniture Manufacturing ",
            Self::HouseholdFurnitureExceptWoodAndUpholsteredManufacturing => "Household Furniture (except Wood and Upholstered) Manufacturing ",
            Self::InstitutionalFurnitureManufacturing => "Institutional Furniture Manufacturing ",
            Self::WoodOfficeFurnitureManufacturing => "Wood Office Furniture Manufacturing ",
            Self::CustomArchitecturalWoodworkAndMillworkManufacturing => "Custom Architectural Woodwork and Millwork Manufacturing ",
            Self::OfficeFurnitureExceptWoodManufacturing => "Office Furniture (except Wood) Manufacturing ",
            Self::ShowcasePartitionShelvingAndLockerManufacturing => "Showcase, Partition, Shelving, and Locker Manufacturing ",
            Self::MattressManufacturing => "Mattress Manufacturing",
            Self::BlindAndShadeManufacturing => "Blind and Shade Manufacturing",
            Self::SurgicalAndMedicalInstrumentManufacturing => "Surgical and Medical Instrument Manufacturing ",
            Self::SurgicalApplianceAndSuppliesManufacturing => "Surgical Appliance and Supplies Manufacturing ",
            Self::DentalEquipmentAndSuppliesManufacturing => "Dental Equipment and Supplies Manufacturing ",
            Self::OphthalmicGoodsManufacturing => "Ophthalmic Goods Manufacturing ",
            Self::DentalLaboratories => "Dental Laboratories ",
            Self::JewelryAndSilverwareManufacturing => "Jewelry and Silverware Manufacturing ",
            Self::SportingAndAthleticGoodsManufacturing => "Sporting and Athletic Goods Manufacturing",
            Self::DollToyAndGameManufacturing => "Doll, Toy, and Game Manufacturing",
            Self::OfficeSuppliesExceptPaperManufacturing => "Office Supplies (except Paper) Manufacturing",
            Self::SignManufacturing => "Sign Manufacturing",
            Self::GasketPackingAndSealingDeviceManufacturing => "Gasket, Packing, and Sealing Device Manufacturing ",
            Self::MusicalInstrumentManufacturing => "Musical Instrument Manufacturing ",
            Self::FastenerButtonNeedleAndPinManufacturing => "Fastener, Button, Needle, and Pin Manufacturing ",
            Self::BroomBrushAndMopManufacturing => "Broom, Brush, and Mop Manufacturing ",
            Self::BurialCasketManufacturing => "Burial Casket Manufacturing ",
            Self::AllOtherMiscellaneousManufacturing => "All Other Miscellaneous Manufacturing ",
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
            Self::MedicalDentalAndHospitalEquipmentAndSuppliesMerchantWholesalers => 
                "Medical, Dental, and Hospital Equipment and Supplies Merchant Wholesalers",
            Self::OphthalmicGoodsMerchantWholesalers => 
                "Ophthalmic Goods Merchant Wholesalers",
            Self::OtherProfessionalEquipmentAndSuppliesMerchantWholesalers => 
                "Other Professional Equipment and Supplies Merchant Wholesalers",
            Self::MetalServiceCentersAndOtherMetalMerchantWholesalers => 
                "Metal Service Centers and Other Metal Merchant Wholesalers",
            Self::CoalAndOtherMineralAndOreMerchantWholesalers => 
                "Coal and Other Mineral and Ore Merchant Wholesalers",
            Self::ElectricalApparatusAndEquipmentWiringSuppliesAndRelatedEquipmentMerchantWholesalers => 
                "Electrical Apparatus and Equipment, Wiring Supplies, and Related Equipment Merchant Wholesalers",
            Self::HouseholdAppliancesElectricHousewaresAndConsumerElectronicsMerchantWholesalers => 
                "Household Appliances, Electric Housewares, and Consumer Electronics Merchant Wholesalers",
            Self::OtherElectronicPartsAndEquipmentMerchantWholesalers => 
                "Other Electronic Parts and Equipment Merchant Wholesalers",
            Self::HardwareMerchantWholesalers => 
                "Hardware Merchant Wholesalers",
            Self::PlumbingAndHeatingEquipmentAndSuppliesHydronicsMerchantWholesalers => 
                "Plumbing and Heating Equipment and Supplies (Hydronics) Merchant Wholesalers",
            Self::WarmAirHeatingAndAirconditioningEquipmentAndSuppliesMerchantWholesalers => 
                "Warm Air Heating and Air-Conditioning Equipment and Supplies Merchant Wholesalers",
            Self::RefrigerationEquipmentAndSuppliesMerchantWholesalers => 
                "Refrigeration Equipment and Supplies Merchant Wholesalers",
            Self::ConstructionAndMiningExceptOilWellMachineryAndEquipmentMerchantWholesalers => 
                "Construction and Mining (except Oil Well) Machinery and Equipment Merchant Wholesalers",
            Self::FarmAndGardenMachineryAndEquipmentMerchantWholesalers => 
                "Farm and Garden Machinery and Equipment Merchant Wholesalers",
            Self::IndustrialMachineryAndEquipmentMerchantWholesalers => 
                "Industrial Machinery and Equipment Merchant Wholesalers",
            Self::IndustrialSuppliesMerchantWholesalers => 
                "Industrial Supplies Merchant Wholesalers",
            Self::ServiceEstablishmentEquipmentAndSuppliesMerchantWholesalers => 
                "Service Establishment Equipment and Supplies Merchant Wholesalers",
            Self::TransportationEquipmentAndSuppliesExceptMotorVehicleMerchantWholesalers => 
                "Transportation Equipment and Supplies (except Motor Vehicle) Merchant Wholesalers",
            Self::SportingAndRecreationalGoodsAndSuppliesMerchantWholesalers => 
                "Sporting and Recreational Goods and Supplies Merchant Wholesalers",
            Self::ToyAndHobbyGoodsAndSuppliesMerchantWholesalers => 
                "Toy and Hobby Goods and Supplies Merchant Wholesalers",
            Self::RecyclableMaterialMerchantWholesalers => 
                "Recyclable Material Merchant Wholesalers",
            Self::JewelryWatchPreciousStoneAndPreciousMetalMerchantWholesalers => 
                "Jewelry, Watch, Precious Stone, and Precious Metal Merchant Wholesalers",
            Self::OtherMiscellaneousDurableGoodsMerchantWholesalers => 
                "Other Miscellaneous Durable Goods Merchant Wholesalers",
            Self::PrintingAndWritingPaperMerchantWholesalers => 
                "Printing and Writing Paper Merchant Wholesalers",
            Self::StationeryAndOfficeSuppliesMerchantWholesalers => 
                "Stationery and Office Supplies Merchant Wholesalers",
            Self::IndustrialAndPersonalServicePaperMerchantWholesalers => 
                "Industrial and Personal Service Paper Merchant Wholesalers",
            Self::DrugsAndDruggistsSundriesMerchantWholesalers => 
                "Drugs and Druggists' Sundries Merchant Wholesalers",
            Self::PieceGoodsNotionsAndOtherDryGoodsMerchantWholesalers => 
                "Piece Goods, Notions, and Other Dry Goods Merchant Wholesalers",
            Self::FootwearMerchantWholesalers => 
                "Footwear Merchant Wholesalers",
            Self::ClothingAndClothingAccessoriesMerchantWholesalers => 
                "Clothing and Clothing Accessories Merchant Wholesalers",
            Self::GeneralLineGroceryMerchantWholesalers => 
                "General Line Grocery Merchant Wholesalers",
            Self::PackagedFrozenFoodMerchantWholesalers => 
                "Packaged Frozen Food Merchant Wholesalers",
            Self::DairyProductExceptDriedOrCannedMerchantWholesalers => 
                "Dairy Product (except Dried or Canned) Merchant Wholesalers",
            Self::PoultryAndPoultryProductMerchantWholesalers => 
                "Poultry and Poultry Product Merchant Wholesalers",
            Self::ConfectioneryMerchantWholesalers => 
                "Confectionery Merchant Wholesalers",
            Self::FishAndSeafoodMerchantWholesalers => 
                "Fish and Seafood Merchant Wholesalers",
            Self::MeatAndMeatProductMerchantWholesalers => 
                "Meat and Meat Product Merchant Wholesalers",
            Self::FreshFruitAndVegetableMerchantWholesalers => 
                "Fresh Fruit and Vegetable Merchant Wholesalers",
            Self::OtherGroceryAndRelatedProductsMerchantWholesalers => 
                "Other Grocery and Related Products Merchant Wholesalers",
            Self::GrainAndFieldBeanMerchantWholesalers => 
                "Grain and Field Bean Merchant Wholesalers",
            Self::LivestockMerchantWholesalers => 
                "Livestock Merchant Wholesalers",
            Self::OtherFarmProductRawMaterialMerchantWholesalers => 
                "Other Farm Product Raw Material Merchant Wholesalers",
            Self::PlasticsMaterialsAndBasicFormsAndShapesMerchantWholesalers => 
                "Plastics Materials and Basic Forms and Shapes Merchant Wholesalers",
            Self::OtherChemicalAndAlliedProductsMerchantWholesalers => 
                "Other Chemical and Allied Products Merchant Wholesalers",
            Self::PetroleumBulkStationsAndTerminals => 
                "Petroleum Bulk Stations and Terminals",
            Self::PetroleumAndPetroleumProductsMerchantWholesalersExceptBulkStationsAndTerminals => 
                "Petroleum and Petroleum Products Merchant Wholesalers (except Bulk Stations and Terminals)",
            Self::BeerAndAleMerchantWholesalers => 
                "Beer and Ale Merchant Wholesalers",
            Self::WineAndDistilledAlcoholicBeverageMerchantWholesalers => 
                "Wine and Distilled Alcoholic Beverage Merchant Wholesalers",
            Self::FarmSuppliesMerchantWholesalers => 
                "Farm Supplies Merchant Wholesalers",
            Self::BookPeriodicalAndNewspaperMerchantWholesalers => 
                "Book, Periodical, and Newspaper Merchant Wholesalers",
            Self::FlowerNurseryStockAndFloristsSuppliesMerchantWholesalers => "Flower, Nursery Stock, and Florists' Supplies Merchant Wholesalers",
            Self::TobaccoProductAndElectronicCigaretteMerchantWholesalers => "Tobacco Product and Electronic Cigarette Merchant Wholesalers",
            Self::PaintVarnishAndSuppliesMerchantWholesalers => "Paint, Varnish, and Supplies Merchant Wholesalers",
            Self::OtherMiscellaneousNondurableGoodsMerchantWholesalers => "Other Miscellaneous Nondurable Goods Merchant Wholesalers",
            Self::WholesaleTradeAgentsAndBrokers => "Wholesale Trade Agents and Brokers",
            Self::NewCarDealers => "New Car Dealers",
            Self::UsedCarDealers => "Used Car Dealers",
            Self::RecreationalVehicleDealers => "Recreational Vehicle Dealers",
            Self::BoatDealers => "Boat Dealers",
            Self::MotorcycleAtvAndAllOtherMotorVehicleDealers => "Motorcycle, ATV, and All Other Motor Vehicle Dealers",
            Self::AutomotivePartsAndAccessoriesRetailers => "Automotive Parts and Accessories Retailers",
            Self::TireDealers => "Tire Dealers",
            Self::HomeCenters => "Home Centers",
            Self::PaintAndWallpaperRetailers => "Paint and Wallpaper Retailers",
            Self::HardwareRetailers => "Hardware Retailers",
            Self::OtherBuildingMaterialDealers => "Other Building Material Dealers",
            Self::OutdoorPowerEquipmentRetailers => "Outdoor Power Equipment Retailers",
            Self::NurseryGardenCenterAndFarmSupplyRetailers => "Nursery, Garden Center, and Farm Supply Retailers",
            Self::SupermarketsAndOtherGroceryRetailersExceptConvenienceRetailers => "Supermarkets and Other Grocery Retailers (except Convenience Retailers)",
            Self::ConvenienceRetailers => "Convenience Retailers",
            Self::VendingMachineOperators => "Vending Machine Operators",
            Self::FruitAndVegetableRetailers => "Fruit and Vegetable Retailers",
            Self::MeatRetailers => "Meat Retailers",
            Self::FishAndSeafoodRetailers => "Fish and Seafood Retailers",
            Self::BakedGoodsRetailers => "Baked Goods Retailers",
            Self::ConfectioneryAndNutRetailers => "Confectionery and Nut Retailers",
            Self::AllOtherSpecialtyFoodRetailers => "All Other Specialty Food Retailers",
            Self::BeerWineAndLiquorRetailers => "Beer, Wine, and Liquor Retailers",
            Self::FurnitureRetailers => "Furniture Retailers",
            Self::FloorCoveringRetailers => "Floor Covering Retailers",
            Self::WindowTreatmentRetailers => "Window Treatment Retailers",
            Self::AllOtherHomeFurnishingsRetailers => "All Other Home Furnishings Retailers",
            Self::ElectronicsAndApplianceRetailers => "Electronics and Appliance Retailers",
            Self::DepartmentStores => "Department Stores",
            Self::WarehouseClubsAndSupercenters => "Warehouse Clubs and Supercenters",
            Self::AllOtherGeneralMerchandiseRetailers => "All Other General Merchandise Retailers",
            Self::PharmaciesAndDrugRetailers => "Pharmacies and Drug Retailers",
            Self::CosmeticsBeautySuppliesAndPerfumeRetailers => "Cosmetics, Beauty Supplies, and Perfume Retailers",
            Self::OpticalGoodsRetailers => "Optical Goods Retailers",
            Self::FoodHealthSupplementRetailers => "Food (Health) Supplement Retailers",
            Self::AllOtherHealthAndPersonalCareRetailers => "All Other Health and Personal Care Retailers",
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
            Self::TobaccoElectronicCigaretteAndOtherSmokingSuppliesRetailers => "Tobacco, Electronic Cigarette, and Other Smoking Supplies Retailers",
            Self::AllOtherMiscellaneousRetailers => "All Other Miscellaneous Retailers",
            Self::ScheduledPassengerAirTransportation => "Scheduled Passenger Air Transportation",
            Self::ScheduledFreightAirTransportation => "Scheduled Freight Air Transportation",
            Self::NonscheduledCharteredPassengerAirTransportation => "Nonscheduled Chartered Passenger Air Transportation",
            Self::NonscheduledCharteredFreightAirTransportation => "Nonscheduled Chartered Freight Air Transportation",
            Self::OtherNonscheduledAirTransportation => "Other Nonscheduled Air Transportation",
            Self::LineHaulRailroads => "Line-Haul Railroads",
            Self::ShortLineRailroads => "Short Line Railroads",
            Self::DeepSeaFreightTransportation => "Deep Sea Freight Transportation",
            Self::DeepSeaPassengerTransportation => "Deep Sea Passenger Transportation",
            Self::CoastalAndGreatLakesFreightTransportation => "Coastal and Great Lakes Freight Transportation",
            Self::CoastalAndGreatLakesPassengerTransportation => "Coastal and Great Lakes Passenger Transportation",
            Self::InlandWaterFreightTransportation => "Inland Water Freight Transportation",
            Self::InlandWaterPassengerTransportation => "Inland Water Passenger Transportation",
            Self::GeneralFreightTruckingLocal => "General Freight Trucking, Local",
            Self::GeneralFreightTruckingLongdistanceTruckload => "General Freight Trucking, Long-Distance, Truckload",
            Self::GeneralFreightTruckingLongdistanceLessThanTruckload => "General Freight Trucking, Long-Distance, Less Than Truckload",
            Self::UsedHouseholdAndOfficeGoodsMoving => "Used Household and Office Goods Moving",
            Self::SpecializedFreightExceptUsedGoodsTruckingLocal => "Specialized Freight (except Used Goods) Trucking, Local",
            Self::SpecializedFreightExceptUsedGoodsTruckingLongdistance => "Specialized Freight (except Used Goods) Trucking, Long-Distance",
            Self::MixedModeTransitSystems => "Mixed Mode Transit Systems",
            Self::CommuterRailSystems => "Commuter Rail Systems",
            Self::BusAndOtherMotorVehicleTransitSystems => "Bus and Other Motor Vehicle Transit Systems",
            Self::StreetRailroads => "Street Railroads",
            Self::LightRailTransitSystems => "Light Rail Transit Systems",
            Self::OtherUrbanTransitSystems => "Other Urban Transit Systems",
            Self::InterurbanAndRuralBusTransportation => "Interurban and Rural Bus Transportation",
            Self::TaxiService => "Taxi Service",
            Self::LimousineService => "Limousine Service",
            Self::SchoolAndEmployeeBusTransportation => "School and Employee Bus Transportation",
            Self::CharterBusIndustry => "Charter Bus Industry",
            Self::SpecialNeedsTransportation => "Special Needs Transportation",
            Self::AllOtherTransitAndGroundPassengerTransportation => "All Other Transit and Ground Passenger Transportation",
            Self::PipelineTransportationOfCrudeOil => "Pipeline Transportation of Crude Oil",
            Self::PipelineTransportationOfNaturalGas => "Pipeline Transportation of Natural Gas",
            Self::PipelineTransportationOfRefinedPetroleumProducts => "Pipeline Transportation of Refined Petroleum Products",
            Self::AllOtherPipelineTransportation => "All Other Pipeline Transportation",
            Self::ScenicAndSightseeingTransportationLand => "Scenic and Sightseeing Transportation, Land",
            Self::ScenicAndSightseeingTransportationWater => "Scenic and Sightseeing Transportation, Water",
            Self::ScenicAndSightseeingTransportationOther => "Scenic and Sightseeing Transportation, Other",
            Self::AirTrafficControl => "Air Traffic Control",
            Self::OtherAirportOperations => "Other Airport Operations",
            Self::OtherSupportActivitiesForAirTransportation => "Other Support Activities for Air Transportation",
            Self::SupportActivitiesForRailTransportation => "Support Activities for Rail Transportation",
            Self::PortAndHarborOperations => "Port and Harbor Operations",
            Self::MarineCargoHandling => "Marine Cargo Handling",
            Self::NavigationalServicesToShipping => "Navigational Services to Shipping",
            Self::OtherSupportActivitiesForWaterTransportation => "Other Support Activities for Water Transportation",
            Self::MotorVehicleTowing => "Motor Vehicle Towing",
            Self::OtherSupportActivitiesForRoadTransportation => "Other Support Activities for Road Transportation",
            Self::FreightTransportationArrangement => "Freight Transportation Arrangement",
            Self::PackingAndCrating => "Packing and Crating",
            Self::AllOtherSupportActivitiesForTransportation => "All Other Support Activities for Transportation",
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
            Self::GreetingCardPublishers => "Greeting Card Publishers",
            Self::AllOtherPublishers => "All Other Publishers",
            Self::SoftwarePublishers => "Software Publishers",
            Self::MotionPictureAndVideoProduction => "Motion Picture and Video Production",
            Self::MotionPictureAndVideoDistribution => "Motion Picture and Video Distribution",
            Self::MotionPictureTheatersExceptDriveins => "Motion Picture Theaters (except Drive-Ins)",
            Self::DriveInMotionPictureTheaters => "Drive-In Motion Picture Theaters",
            Self::TeleproductionAndOtherPostproductionServices => "Teleproduction and Other Postproduction Services",
            Self::OtherMotionPictureAndVideoIndustries => "Other Motion Picture and Video Industries",
            Self::MusicPublishers => "Music Publishers",
            Self::SoundRecordingStudios => "Sound Recording Studios",
            Self::RecordProductionAndDistribution => "Record Production and Distribution",
            Self::OtherSoundRecordingIndustries => "Other Sound Recording Industries",
            Self::RadioNetworks => "Radio Networks",
            Self::RadioStations => "Radio Stations",
            Self::TelevisionBroadcasting => "Television Broadcasting",
            Self::CableAndOtherSubscriptionProgramming => "Cable and Other Subscription Programming",
            Self::WiredTelecommunicationsCarriers => "Wired Telecommunications Carriers",
            Self::WirelessTelecommunicationsCarriersExceptSatellite => "Wireless Telecommunications Carriers (except Satellite)",
            Self::SatelliteTelecommunications => "Satellite Telecommunications",
            Self::TelecommunicationsResellers => "Telecommunications Resellers",
            Self::AllOtherTelecommunications => "All Other Telecommunications",
            Self::ComputingInfrastructureProvidersDataProcessingWebHostingAndRelatedServices => 
                "Computing Infrastructure Providers, Data Processing, Web Hosting, and Related Services",
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
            Self::ConsumerLending => "Consumer Lending",
            Self::RealEstateCredit => "Real Estate Credit",
            Self::AllOtherNondepositoryCreditIntermediation => "All Other Nondepository Credit Intermediation",
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
            Self::TrustFiduciaryAndCustodyActivities => "Trust, Fiduciary, and Custody Activities",
            Self::MiscellaneousFinancialInvestmentActivities => "Miscellaneous Financial Investment Activities",
            Self::DirectLifeInsuranceCarriers => "Direct Life Insurance Carriers",
            Self::DirectHealthAndMedicalInsuranceCarriers => "Direct Health and Medical Insurance Carriers",
            Self::DirectPropertyAndCasualtyInsuranceCarriers => "Direct Property and Casualty Insurance Carriers",
            Self::DirectTitleInsuranceCarriers => "Direct Title Insurance Carriers",
            Self::OtherDirectInsuranceExceptLifeHealthAndMedicalCarriers => "Other Direct Insurance (except Life, Health, and Medical) Carriers",
            Self::ReinsuranceCarriers => "Reinsurance Carriers",
            Self::InsuranceAgenciesAndBrokerages => "Insurance Agencies and Brokerages",
            Self::ClaimsAdjusting => "Claims Adjusting",
            Self::ThirdPartyAdministrationOfInsuranceAndPensionFunds => "Third Party Administration of Insurance and Pension Funds",
            Self::AllOtherInsuranceRelatedActivities => "All Other Insurance Related Activities",
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
            Self::ResidentialPropertyManagers => "Residential Property Managers",
            Self::NonresidentialPropertyManagers => "Nonresidential Property Managers",
            Self::OfficesOfRealEstateAppraisers => "Offices of Real Estate Appraisers",
            Self::OtherActivitiesRelatedToRealEstate => "Other Activities Related to Real Estate",
            Self::PassengerCarRental => "Passenger Car Rental",
            Self::PassengerCarLeasing => "Passenger Car Leasing",
            Self::TruckUtilityTrailerAndRvRecreationalVehicleRentalAndLeasing => "Truck, Utility Trailer, and RV (Recreational Vehicle) Rental and Leasing",
            Self::ConsumerElectronicsAndAppliancesRental => "Consumer Electronics and Appliances Rental",
            Self::FormalWearAndCostumeRental => "Formal Wear and Costume Rental",
            Self::VideoTapeAndDiscRental => "Video Tape and Disc Rental",
            Self::RecreationalGoodsRentalParent => "Recreational Goods Rental Parent",
            Self::MedicalEquipmentAndSuppliesRental => "Medical Equipment and Supplies Rental",
            Self::HomeHealthEquipmentAndSuppliesRental => "Home Health Equipment and Supplies Rental",
            Self::RecreationalGoodsRental => "Recreational Goods Rental",
            Self::AllOtherConsumerGoodsRental => "All Other Consumer Goods Rental",
            Self::GeneralRentalCenters => "General Rental Centers",
            Self::CommercialAirRailAndWaterTransportationEquipmentRentalAndLeasing => "Commercial Air, Rail, and Water Transportation Equipment Rental and Leasing",
            Self::ConstructionMiningAndForestryMachineryAndEquipmentRentalAndLeasing => "Construction, Mining, and Forestry Machinery and Equipment Rental and Leasing",
            Self::OfficeMachineryAndEquipmentRentalAndLeasing => "Office Machinery and Equipment Rental and Leasing",
            Self::OtherCommercialAndIndustrialMachineryAndEquipmentRentalAndLeasing => "Other Commercial and Industrial Machinery and Equipment Rental and Leasing",
            Self::LessorsOfNonfinancialIntangibleAssetsExceptCopyrightedWorks => "Lessors of Nonfinancial Intangible Assets (except Copyrighted Works)",
            Self::OfficesOfLawyers => "Offices of Lawyers",
            Self::OfficesOfNotaries => "Offices of Notaries",
            Self::TitleAbstractAndSettlementOffices => "Title Abstract and Settlement Offices",
            Self::AllOtherLegalServices => "All Other Legal Services",
            Self::OfficesOfCertifiedPublicAccountants => "Offices of Certified Public Accountants",
            Self::TaxPreparationServices => "Tax Preparation Services",
            Self::PayrollServices => "Payroll Services",
            Self::OtherAccountingServices => "Other Accounting Services",
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
            Self::CustomComputerProgrammingServices => "Custom Computer Programming Services",
            Self::ComputerSystemsDesignServices => "Computer Systems Design Services",
            Self::ComputerFacilitiesManagementServices => "Computer Facilities Management Services",
            Self::OtherComputerRelatedServices => "Other Computer Related Services",
            Self::AdministrativeManagementAndGeneralManagementConsultingServices => "Administrative Management and General Management Consulting Services",
            Self::HumanResourcesConsultingServices => "Human Resources Consulting Services",
            Self::MarketingConsultingServices => "Marketing Consulting Services",
            Self::ProcessPhysicalDistributionAndLogisticsConsultingServices => "Process, Physical Distribution, and Logistics Consulting Services",
            Self::OtherManagementConsultingServices => "Other Management Consulting Services",
            Self::EnvironmentalConsultingServices => "Environmental Consulting Services",
            Self::OtherScientificAndTechnicalConsultingServices => "Other Scientific and Technical Consulting Services",
            Self::ResearchAndDevelopmentInNanotechnology => "Research and Development in Nanotechnology",
            Self::ResearchAndDevelopmentInBiotechnologyExceptNanobiotechnology => "Research and Development in Biotechnology (except Nanobiotechnology)",
            Self::ResearchAndDevelopmentInThePhysicalEngineeringAndLifeSciencesExceptNanotechnologyAndBiotechnology => "Research and Development in the Physical, Engineering, and Life Sciences (except Nanotechnology and Biotechnology)",
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
            Self::PhotographyStudiosPortrait => "Photography Studios, Portrait",
            Self::CommercialPhotography => "Commercial Photography",
            Self::TranslationAndInterpretationServices => "Translation and Interpretation Services",
            Self::VeterinaryServices => "Veterinary Services",
            Self::AllOtherProfessionalScientificAndTechnicalServices => "All Other Professional, Scientific, and Technical Services",
            Self::OfficesOfBankHoldingCompanies => "Offices of Bank Holding Companies",
            Self::OfficesOfOtherHoldingCompanies => "Offices of Other Holding Companies",
            Self::CorporateSubsidiaryAndRegionalManagingOfficesParent => "Corporate, Subsidiary, and Regional Managing Offices Parent",
            Self::CorporateSubsidiaryAndRegionalManagingOffices => "Corporate, Subsidiary, and Regional Managing Offices",
            Self::OfficeAdministrativeServices => "Office Administrative Services",
            Self::FacilitiesSupportServices => "Facilities Support Services",
            Self::EmploymentPlacementAgencies => "Employment Placement Agencies",
            Self::ExecutiveSearchServices => "Executive Search Services",
            Self::TemporaryHelpServices => "Temporary Help Services",
            Self::ProfessionalEmployerOrganizations => "Professional Employer Organizations",
            Self::DocumentPreparationServices => "Document Preparation Services",
            Self::TelephoneAnsweringServices => "Telephone Answering Services",
            Self::TelemarketingBureausAndOtherContactCenters => "Telemarketing Bureaus and Other Contact Centers",
            Self::PrivateMailCenters => "Private Mail Centers",
            Self::OtherBusinessServiceCentersIncludingCopyShops => "Other Business Service Centers (including Copy Shops)",
            Self::CollectionAgencies => "Collection Agencies",
            Self::CreditBureaus => "Credit Bureaus",
            Self::RepossessionServices => "Repossession Services",
            Self::CourtReportingAndStenotypeServices => "Court Reporting and Stenotype Services",
            Self::AllOtherBusinessSupportServices => "All Other Business Support Services",
            Self::TravelAgencies => "Travel Agencies",
            Self::TourOperators => "Tour Operators",
            Self::ConventionAndVisitorsBureaus => "Convention and Visitors Bureaus",
            Self::AllOtherTravelArrangementAndReservationServices => "All Other Travel Arrangement and Reservation Services",
            Self::InvestigationServices => "Investigation Services",
            Self::SecurityGuardsAndPatrolServices => "Security Guards and Patrol Services",
            Self::ArmoredCarServices => "Armored Car Services",
            Self::SecuritySystemsServicesExceptLocksmiths => "Security Systems Services (except Locksmiths)",
            Self::Locksmiths => "Locksmiths",
            Self::ExterminatingAndPestControlServices => "Exterminating and Pest Control Services",
            Self::JanitorialServices => "Janitorial Services",
            Self::LandscapingServices => "Landscaping Services",
            Self::CarpetAndUpholsteryCleaningServices => "Carpet and Upholstery Cleaning Services",
            Self::OtherServicesToBuildingsAndDwellings => "Other Services to Buildings and Dwellings",
            Self::PackagingAndLabelingServices => "Packaging and Labeling Services",
            Self::ConventionAndTradeShowOrganizers => "Convention and Trade Show Organizers",
            Self::AllOtherSupportServices => "All Other Support Services",
            Self::SolidWasteCollection => "Solid Waste Collection",
            Self::HazardousWasteCollection => "Hazardous Waste Collection",
            Self::OtherWasteCollection => "Other Waste Collection",
            Self::HazardousWasteTreatmentAndDisposal => "Hazardous Waste Treatment and Disposal",
            Self::SolidWasteLandfill => "Solid Waste Landfill",
            Self::SolidWasteCombustorsAndIncinerators => "Solid Waste Combustors and Incinerators",
            Self::OtherNonhazardousWasteTreatmentAndDisposal => "Other Nonhazardous Waste Treatment and Disposal",
            Self::RemediationServices => "Remediation Services",
            Self::MaterialsRecoveryFacilities => "Materials Recovery Facilities",
            Self::SepticTankAndRelatedServices => "Septic Tank and Related Services",
            Self::AllOtherMiscellaneousWasteManagementServices => "All Other Miscellaneous Waste Management Services",
            Self::ElementaryAndSecondarySchools => "Elementary and Secondary Schools",
            Self::JuniorColleges => "Junior Colleges",
            Self::CollegesUniversitiesAndProfessionalSchools => "Colleges, Universities, and Professional Schools",
            Self::BusinessAndSecretarialSchools => "Business and Secretarial Schools",
            Self::ComputerTraining => "Computer Training",
            Self::ProfessionalAndManagementDevelopmentTraining => "Professional and Management Development Training",
            Self::CosmetologyAndBarberSchools => "Cosmetology and Barber Schools",
            Self::FlightTraining => "Flight Training",
            Self::ApprenticeshipTraining => "Apprenticeship Training",
            Self::OtherTechnicalAndTradeSchools => "Other Technical and Trade Schools",
            Self::FineArtsSchools => "Fine Arts Schools",
            Self::SportsAndRecreationInstruction => "Sports and Recreation Instruction",
            Self::LanguageSchools => "Language Schools",
            Self::ExamPreparationAndTutoring => "Exam Preparation and Tutoring",
            Self::AutomobileDrivingSchools => "Automobile Driving Schools",
            Self::AllOtherMiscellaneousSchoolsAndInstruction => "All Other Miscellaneous Schools and Instruction",
            Self::EducationalSupportServices => "Educational Support Services",
            Self::OfficesOfPhysiciansExceptMentalHealthSpecialists => "Offices of Physicians (except Mental Health Specialists)",
            Self::OfficesOfPhysiciansMentalHealthSpecialists => "Offices of Physicians, Mental Health Specialists",
            Self::OfficesOfDentists => "Offices of Dentists",
            Self::OfficesOfChiropractors => "Offices of Chiropractors",
            Self::OfficesOfOptometrists => "Offices of Optometrists",
            Self::OfficesOfMentalHealthPractitionersExceptPhysicians => "Offices of Mental Health Practitioners (except Physicians)",
            Self::OfficesOfPhysicalOccupationalAndSpeechTherapistsAndAudiologists => "Offices of Physical, Occupational and Speech Therapists, and Audiologists",
            Self::OfficesOfPodiatrists => "Offices of Podiatrists",
            Self::OfficesOfAllOtherMiscellaneousHealthPractitioners => "Offices of All Other Miscellaneous Health Practitioners",
            Self::FamilyPlanningCenters => "Family Planning Centers",
            Self::OutpatientMentalHealthAndSubstanceAbuseCenters => "Outpatient Mental Health and Substance Abuse Centers",
            Self::HmoMedicalCenters => "HMO Medical Centers",
            Self::KidneyDialysisCenters => "Kidney Dialysis Centers",
            Self::FreestandingAmbulatorySurgicalAndEmergencyCenters => "Freestanding Ambulatory Surgical and Emergency Centers",
            Self::AllOtherOutpatientCareCenters => "All Other Outpatient Care Centers",
            Self::MedicalLaboratories => "Medical Laboratories",
            Self::DiagnosticImagingCenters => "Diagnostic Imaging Centers",
            Self::HomeHealthCareServices => "Home Health Care Services",
            Self::AmbulanceServices => "Ambulance Services",
            Self::BloodAndOrganBanks => "Blood and Organ Banks",
            Self::AllOtherMiscellaneousAmbulatoryHealthCareServices => "All Other Miscellaneous Ambulatory Health Care Services",
            Self::GeneralMedicalAndSurgicalHospitals => "General Medical and Surgical Hospitals",
            Self::PsychiatricAndSubstanceAbuseHospitals => "Psychiatric and Substance Abuse Hospitals",
            Self::SpecialtyExceptPsychiatricAndSubstanceAbuseHospitals => "Specialty (except Psychiatric and Substance Abuse) Hospitals",
            Self::NursingCareFacilitiesSkilledNursingFacilities => "Nursing Care Facilities (Skilled Nursing Facilities)",
            Self::ResidentialIntellectualAndDevelopmentalDisabilityFacilities => "Residential Intellectual and Developmental Disability Facilities",
            Self::ResidentialMentalHealthAndSubstanceAbuseFacilities => "Residential Mental Health and Substance Abuse Facilities",
            Self::ContinuingCareRetirementCommunities => "Continuing Care Retirement Communities",
            Self::AssistedLivingFacilitiesForTheElderly => "Assisted Living Facilities for the Elderly",
            Self::OtherResidentialCareFacilities => "Other Residential Care Facilities",
            Self::ChildAndYouthServices => "Child and Youth Services",
            Self::ServicesForTheElderlyAndPersonsWithDisabilities => "Services for the Elderly and Persons with Disabilities",
            Self::OtherIndividualAndFamilyServices => "Other Individual and Family Services",
            Self::CommunityFoodServices => "Community Food Services",
            Self::TemporaryShelters => "Temporary Shelters",
            Self::OtherCommunityHousingServices => "Other Community Housing Services",
            Self::EmergencyAndOtherReliefServices => "Emergency and Other Relief Services",
            Self::VocationalRehabilitationServices => "Vocational Rehabilitation Services",
            Self::ChildDayCareServices => "Child Day Care Services",
            Self::TheaterCompaniesAndDinnerTheaters => "Theater Companies and Dinner Theaters",
            Self::DanceCompanies => "Dance Companies",
            Self::MusicalGroupsAndArtists => "Musical Groups and Artists",
            Self::OtherPerformingArtsCompanies => "Other Performing Arts Companies",
            Self::SportsTeamsAndClubs => "Sports Teams and Clubs",
            Self::Racetracks => "Racetracks",
            Self::OtherSpectatorSports => "Other Spectator Sports",
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
            Self::BedAndBreakfastInns => "Bed-and-Breakfast Inns",
            Self::AllOtherTravelerAccommodation => "All Other Traveler Accommodation",
            Self::RvRecreationalVehicleParksAndCampgrounds => "RV (Recreational Vehicle) Parks and Campgrounds",
            Self::RecreationalAndVacationCampsExceptCampgrounds => "Recreational and Vacation Camps (except Campgrounds)",
            Self::RoomingAndBoardingHouses => "Rooming and Boarding Houses",
            Self::FoodServiceContractors => "Food Service Contractors",
            Self::Caterers => "Caterers",
            Self::MobileFoodServices => "Mobile Food Services",
            Self::DrinkingPlacesAlcoholicBeverages => "Drinking Places (Alcoholic Beverages)",
            Self::FullServiceRestaurants => "Full-Service Restaurants",
            Self::LimitedServiceRestaurants => "Limited-Service Restaurants",
            Self::CafeteriasGrillBuffetsAndBuffets => "Cafeterias, Grill Buffets, and Buffets",
            Self::SnackAndNonalcoholicBeverageBars => "Snack and Nonalcoholic Beverage Bars",
            Self::GeneralAutomotiveRepair => "General Automotive Repair",
            Self::AutomotiveExhaustSystemRepair => "Automotive Exhaust System Repair",
            Self::AutomotiveTransmissionRepair => "Automotive Transmission Repair",
            Self::OtherAutomotiveMechanicalAndElectricalRepairAndMaintenance => "Other Automotive Mechanical and Electrical Repair and Maintenance",
            Self::AutomotiveBodyPaintAndInteriorRepairAndMaintenance => "Automotive Body, Paint, and Interior Repair and Maintenance",
            Self::AutomotiveGlassReplacementShops => "Automotive Glass Replacement Shops",
            Self::AutomotiveOilChangeAndLubricationShops => "Automotive Oil Change and Lubrication Shops",
            Self::CarWashes => "Car Washes",
            Self::AllOtherAutomotiveRepairAndMaintenance => "All Other Automotive Repair and Maintenance",
            Self::ConsumerElectronicsRepairAndMaintenance => "Consumer Electronics Repair and Maintenance",
            Self::ComputerAndOfficeMachineRepairAndMaintenance => "Computer and Office Machine Repair and Maintenance",
            Self::CommunicationEquipmentRepairAndMaintenance => "Communication Equipment Repair and Maintenance",
            Self::OtherElectronicAndPrecisionEquipmentRepairAndMaintenance => "Other Electronic and Precision Equipment Repair and Maintenance",
            Self::CommercialAndIndustrialMachineryAndEquipmentExceptAutomotiveAndElectronicRepairAndMaintenance => "Commercial and Industrial Machinery and Equipment (except Automotive and Electronic) Repair and Maintenance",
            Self::HomeAndGardenEquipmentRepairAndMaintenance => "Home and Garden Equipment Repair and Maintenance",
            Self::ApplianceRepairAndMaintenance => "Appliance Repair and Maintenance",
            Self::ReupholsteryAndFurnitureRepair => "Reupholstery and Furniture Repair",
            Self::FootwearAndLeatherGoodsRepair => "Footwear and Leather Goods Repair",
            Self::OtherPersonalAndHouseholdGoodsRepairAndMaintenance => "Other Personal and Household Goods Repair and Maintenance",
            Self::BarberShops => "Barber Shops",
            Self::BeautySalons => "Beauty Salons",
            Self::NailSalons => "Nail Salons",
            Self::DietAndWeightReducingCenters => "Diet and Weight Reducing Centers",
            Self::OtherPersonalCareServices => "Other Personal Care Services",
            Self::FuneralHomesAndFuneralServices => "Funeral Homes and Funeral Services",
            Self::CemeteriesAndCrematories => "Cemeteries and Crematories",
            Self::CoinOperatedLaundriesAndDrycleaners => "Coin-Operated Laundries and Drycleaners",
            Self::DrycleaningAndLaundryServicesExceptCoinOperated => "Drycleaning and Laundry Services (except Coin-Operated)",
            Self::LinenSupply => "Linen Supply",
            Self::IndustrialLaunderers => "Industrial Launderers",
            Self::PetCareExceptVeterinaryServices => "Pet Care (except Veterinary) Services",
            Self::PhotofinishingLaboratoriesExceptOneHour => "Photofinishing Laboratories (except One-Hour)",
            Self::OneHourPhotofinishing => "One-Hour Photofinishing",
            Self::ParkingLotsAndGarages => "Parking Lots and Garages",
            Self::AllOtherPersonalServices => "All Other Personal Services",
            Self::ReligiousOrganizations => "Religious Organizations",
            Self::GrantmakingFoundations => "Grantmaking Foundations",
            Self::VoluntaryHealthOrganizations => "Voluntary Health Organizations",
            Self::OtherGrantmakingAndGivingServices => "Other Grantmaking and Giving Services",
            Self::HumanRightsOrganizations => "Human Rights Organizations",
            Self::EnvironmentConservationAndWildlifeOrganizations => "Environment, Conservation and Wildlife Organizations",
            Self::OtherSocialAdvocacyOrganizations => "Other Social Advocacy Organizations",
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

    /// Returns the NAICS code for this industry as an i64
    pub fn code(&self) -> i64 {
        match self {
            Self::SoybeanFarming => 111110,
            Self::OilseedExceptSoybeanFarming => 111120,
            Self::DryPeaAndBeanFarming => 111130,
            Self::WheatFarming => 111140,
            Self::CornFarming => 111150,
            Self::RiceFarming => 111160,
            Self::OilseedAndGrainCombinationFarming => 111191,
            Self::AllOtherGrainFarming => 111199,
            Self::PotatoFarming => 111211,
            Self::OtherVegetableExceptPotatoAndMelonFarming => 111219,
            Self::OrangeGroves => 111310,
            Self::CitrusExceptOrangeGroves => 111320,
            Self::AppleOrchards => 111331,
            Self::GrapeVineyards => 111332,
            Self::StrawberryFarming => 111333,
            Self::BerryExceptStrawberryFarming => 111334,
            Self::TreeNutFarming => 111335,
            Self::FruitAndTreeNutCombinationFarming => 111336,
            Self::OtherNoncitrusFruitFarming => 111339,
            Self::MushroomProduction => 111411,
            Self::OtherFoodCropsGrownUnderCover => 111419,
            Self::NurseryAndTreeProduction => 111421,
            Self::FloricultureProduction => 111422,
            Self::TobaccoFarming => 111910,
            Self::CottonFarming => 111920,
            Self::SugarcaneFarming => 111930,
            Self::HayFarming => 111940,
            Self::SugarBeetFarming => 111991,
            Self::PeanutFarming => 111992,
            Self::AllOtherMiscellaneousCropFarming => 111998,
            Self::BeefCattleRanchingAndFarming => 112111,
            Self::CattleFeedlots => 112112,
            Self::DairyCattleAndMilkProduction => 112120,
            Self::DualpurposeCattleRanchingAndFarming => 112130,
            Self::HogAndPigFarming => 112210,
            Self::ChickenEggProduction => 112310,
            Self::BroilersAndOtherMeatTypeChickenProduction => 112320,
            Self::TurkeyProduction => 112330,
            Self::PoultryHatcheries => 112340,
            Self::OtherPoultryProduction => 112390,
            Self::SheepFarming => 112410,
            Self::GoatFarming => 112420,
            Self::FinfishFarmingAndFishHatcheries => 112511,
            Self::ShellfishFarming => 112512,
            Self::OtherAquaculture => 112519,
            Self::Apiculture => 112910,
            Self::HorsesAndOtherEquineProduction => 112920,
            Self::FurbearingAnimalAndRabbitProduction => 112930,
            Self::AllOtherAnimalProduction => 112990,
            Self::TimberTractOperations => 113110,
            Self::ForestNurseriesAndGatheringOfForestProducts => 113210,
            Self::Logging => 113310,
            Self::FinfishFishing => 114111,
            Self::ShellfishFishing => 114112,
            Self::OtherMarineFishing => 114119,
            Self::HuntingAndTrapping => 114210,
            Self::CottonGinning => 115111,
            Self::SoilPreparationPlantingAndCultivating => 115112,
            Self::CropHarvestingPrimarilyByMachine => 115113,
            Self::PostharvestCropActivitiesExceptCottonGinning => 115114,
            Self::FarmLaborContractorsAndCrewLeaders => 115115,
            Self::FarmManagementServices => 115116,
            Self::SupportActivitiesForAnimalProduction => 115210,
            Self::SupportActivitiesForForestry => 115310,
            Self::CrudePetroleumExtraction => 211120,
            Self::NaturalGasExtraction => 211130,
            Self::SurfaceCoalMining => 212114,
            Self::UndergroundCoalMining => 212115,
            Self::IronOreMining => 212210,
            Self::GoldOreAndSilverOreMining => 212220,
            Self::CopperNickelLeadAndZincMining => 212230,
            Self::OtherMetalOreMining => 212290,
            Self::DimensionStoneMiningAndQuarrying => 212311,
            Self::CrushedAndBrokenLimestoneMiningAndQuarrying => 212312,
            Self::CrushedAndBrokenGraniteMiningAndQuarrying => 212313,
            Self::OtherCrushedAndBrokenStoneMiningAndQuarrying => 212319,
            Self::ConstructionSandAndGravelMining => 212321,
            Self::IndustrialSandMining => 212322,
            Self::KaolinClayAndCeramicAndRefractoryMineralsMining => 212323,
            Self::OtherNonmetallicMineralMiningAndQuarrying => 212390,
            Self::DrillingOilAndGasWells => 213111,
            Self::SupportActivitiesForOilAndGasOperations => 213112,
            Self::SupportActivitiesForCoalMining => 213113,
            Self::SupportActivitiesForMetalMining => 213114,
            Self::SupportActivitiesForNonmetallicMineralsExceptFuelsMining => 213115,
            Self::HydroelectricPowerGeneration => 221111,
            Self::FossilFuelElectricPowerGeneration => 221112,
            Self::NuclearElectricPowerGeneration => 221113,
            Self::SolarElectricPowerGeneration => 221114,
            Self::WindElectricPowerGeneration => 221115,
            Self::GeothermalElectricPowerGeneration => 221116,
            Self::BiomassElectricPowerGeneration => 221117,
            Self::OtherElectricPowerGeneration => 221118,
            Self::ElectricBulkPowerTransmissionAndControl => 221121,
            Self::ElectricPowerDistribution => 221122,
            Self::NaturalGasDistribution => 221210,
            Self::WaterSupplyAndIrrigationSystems => 221310,
            Self::SewageTreatmentFacilities => 221320,
            Self::SteamAndAirconditioningSupply => 221330,
            Self::NewSinglefamilyHousingConstructionExceptForsaleBuilders => 236115,
            Self::NewMultifamilyHousingConstructionExceptForsaleBuilders => 236116,
            Self::NewHousingForsaleBuilders => 236117,
            Self::ResidentialRemodelers => 236118,
            Self::IndustrialBuildingConstruction => 236210,
            Self::CommercialAndInstitutionalBuildingConstruction => 236220,
            Self::WaterAndSewerLineAndRelatedStructuresConstruction => 237110,
            Self::OilAndGasPipelineAndRelatedStructuresConstruction => 237120,
            Self::PowerAndCommunicationLineAndRelatedStructuresConstruction => 237130,
            Self::LandSubdivision => 237210,
            Self::HighwayStreetAndBridgeConstruction => 237310,
            Self::OtherHeavyAndCivilEngineeringConstruction => 237990,
            Self::PouredConcreteFoundationAndStructureContractors => 238110,
            Self::StructuralSteelAndPrecastConcreteContractors => 238120,
            Self::FramingContractors => 238130,
            Self::MasonryContractors => 238140,
            Self::GlassAndGlazingContractors => 238150,
            Self::RoofingContractors => 238160,
            Self::SidingContractors => 238170,
            Self::OtherFoundationStructureAndBuildingExteriorContractors => 238190,
            Self::ElectricalContractorsAndOtherWiringInstallationContractors => 238210,
            Self::PlumbingHeatingAndAirconditioningContractors => 238220,
            Self::OtherBuildingEquipmentContractors => 238290,
            Self::DrywallAndInsulationContractors => 238310,
            Self::PaintingAndWallCoveringContractors => 238320,
            Self::FlooringContractors => 238330,
            Self::TileAndTerrazzoContractors => 238340,
            Self::FinishCarpentryContractors => 238350,
            Self::OtherBuildingFinishingContractors => 238390,
            Self::SitePreparationContractors => 238910,
            Self::AllOtherSpecialtyTradeContractors => 238990,
            Self::DogAndCatFoodManufacturing => 311111,
            Self::OtherAnimalFoodManufacturing => 311119,
            Self::FlourMilling => 311211,
            Self::RiceMilling => 311212,
            Self::MaltManufacturing => 311213,
            Self::WetCornMillingAndStarchManufacturing => 311221,
            Self::SoybeanAndOtherOilseedProcessing => 311224,
            Self::FatsAndOilsRefiningAndBlending => 311225,
            Self::BreakfastCerealManufacturing => 311230,
            Self::BeetSugarManufacturing => 311313,
            Self::CaneSugarManufacturing => 311314,
            Self::NonchocolateConfectioneryManufacturing => 311340,
            Self::ChocolateAndConfectioneryManufacturingFromCacaoBeans => 311351,
            Self::ConfectioneryManufacturingFromPurchasedChocolate => 311352,
            Self::FrozenFruitJuiceAndVegetableManufacturing => 311411,
            Self::FrozenSpecialtyFoodManufacturing => 311412,
            Self::FruitAndVegetableCanning => 311421,
            Self::SpecialtyCanning => 311422,
            Self::DriedAndDehydratedFoodManufacturing => 311423,
            Self::FluidMilkManufacturing => 311511,
            Self::CreameryButterManufacturing => 311512,
            Self::CheeseManufacturing => 311513,
            Self::DryCondensedAndEvaporatedDairyProductManufacturing => 311514,
            Self::IceCreamAndFrozenDessertManufacturing => 311520,
            Self::AnimalExceptPoultrySlaughtering => 311611,
            Self::MeatProcessedFromCarcasses => 311612,
            Self::RenderingAndMeatByproductProcessing => 311613,
            Self::PoultryProcessing => 311615,
            Self::SeafoodProductPreparationAndPackaging => 311710,
            Self::RetailBakeries => 311811,
            Self::CommercialBakeries => 311812,
            Self::FrozenCakesPiesAndOtherPastriesManufacturing => 311813,
            Self::CookieAndCrackerManufacturing => 311821,
            Self::DryPastaDoughAndFlourMixesManufacturingFromPurchasedFlour => 311824,
            Self::TortillaManufacturing => 311830,
            Self::RoastedNutsAndPeanutButterManufacturing => 311911,
            Self::OtherSnackFoodManufacturing => 311919,
            Self::CoffeeAndTeaManufacturing => 311920,
            Self::FlavoringSyrupAndConcentrateManufacturing => 311930,
            Self::MayonnaiseDressingAndOtherPreparedSauceManufacturing => 311941,
            Self::SpiceAndExtractManufacturing => 311942,
            Self::PerishablePreparedFoodManufacturing => 311991,
            Self::AllOtherMiscellaneousFoodManufacturing => 311999,
            Self::SoftDrinkManufacturing => 312111,
            Self::BottledWaterManufacturing => 312112,
            Self::IceManufacturing => 312113,
            Self::Breweries => 312120,
            Self::Wineries => 312130,
            Self::Distilleries => 312140,
            Self::TobaccoManufacturing => 312230,
            Self::FiberYarnAndThreadMills => 313110,
            Self::BroadwovenFabricMills => 313210,
            Self::NarrowFabricMillsAndSchiffliMachineEmbroidery => 313220,
            Self::NonwovenFabricMills => 313230,
            Self::KnitFabricMills => 313240,
            Self::TextileAndFabricFinishingMills => 313310,
            Self::FabricCoatingMills => 313320,
            Self::CarpetAndRugMills => 314110,
            Self::CurtainAndLinenMills => 314120,
            Self::TextileBagAndCanvasMills => 314910,
            Self::RopeCordageTwineTireCordAndTireFabricMills => 314994,
            Self::AllOtherMiscellaneousTextileProductMills => 314999,
            Self::ApparelKnittingMills => 315120,
            Self::CutAndSewApparelContractors => 315210,
            Self::CutAndSewApparelManufacturingExceptContractors => 315250,
            Self::ApparelAccessoriesAndOtherApparelManufacturing => 315990,
            Self::LeatherAndHideTanningAndFinishing => 316110,
            Self::FootwearManufacturing => 316210,
            Self::OtherLeatherAndAlliedProductManufacturing => 316990,
            Self::Sawmills => 321113,
            Self::WoodPreservation => 321114,
            Self::HardwoodVeneerAndPlywoodManufacturing => 321211,
            Self::SoftwoodVeneerAndPlywoodManufacturing => 321212,
            Self::EngineeredWoodMemberManufacturing => 321215,
            Self::ReconstitutedWoodProductManufacturing => 321219,
            Self::WoodWindowAndDoorManufacturing => 321911,
            Self::CutStockResawingLumberAndPlaning => 321912,
            Self::OtherMillworkIncludingFlooring => 321918,
            Self::WoodContainerAndPalletManufacturing => 321920,
            Self::ManufacturedHomeMobileHomeManufacturing => 321991,
            Self::PrefabricatedWoodBuildingManufacturing => 321992,
            Self::AllOtherMiscellaneousWoodProductManufacturing => 321999,
            Self::PulpMills => 322110,
            Self::PaperMills => 322120,
            Self::PaperboardMills => 322130,
            Self::CorrugatedAndSolidFiberBoxManufacturing => 322211,
            Self::FoldingPaperboardBoxManufacturing => 322212,
            Self::OtherPaperboardContainerManufacturing => 322219,
            Self::PaperBagAndCoatedAndTreatedPaperManufacturing => 322220,
            Self::StationeryProductManufacturing => 322230,
            Self::SanitaryPaperProductManufacturing => 322291,
            Self::AllOtherConvertedPaperProductManufacturing => 322299,
            Self::CommercialPrintingExceptScreenAndBooks => 323111,
            Self::CommercialScreenPrinting => 323113,
            Self::BooksPrinting => 323117,
            Self::SupportActivitiesForPrinting => 323120,
            Self::PetroleumRefineries => 324110,
            Self::AsphaltPavingMixtureAndBlockManufacturing => 324121,
            Self::AsphaltShingleAndCoatingMaterialsManufacturing => 324122,
            Self::PetroleumLubricatingOilAndGreaseManufacturing => 324191,
            Self::AllOtherPetroleumAndCoalProductsManufacturing => 324199,
            Self::PetrochemicalManufacturing => 325110,
            Self::IndustrialGasManufacturing => 325120,
            Self::SyntheticDyeAndPigmentManufacturing => 325130,
            Self::OtherBasicInorganicChemicalManufacturing => 325180,
            Self::EthylAlcoholManufacturing => 325193,
            Self::CyclicCrudeIntermediateAndGumAndWoodChemicalManufacturing => 325194,
            Self::AllOtherBasicOrganicChemicalManufacturing => 325199,
            Self::PlasticsMaterialAndResinManufacturing => 325211,
            Self::SyntheticRubberManufacturing => 325212,
            Self::ArtificialAndSyntheticFibersAndFilamentsManufacturing => 325220,
            Self::NitrogenousFertilizerManufacturing => 325311,
            Self::PhosphaticFertilizerManufacturing => 325312,
            Self::FertilizerMixingOnlyManufacturing => 325314,
            Self::CompostManufacturing => 325315,
            Self::PesticideAndOtherAgriculturalChemicalManufacturing => 325320,
            Self::MedicinalAndBotanicalManufacturing => 325411,
            Self::PharmaceuticalPreparationManufacturing => 325412,
            Self::InvitroDiagnosticSubstanceManufacturing => 325413,
            Self::BiologicalProductExceptDiagnosticManufacturing => 325414,
            Self::PaintAndCoatingManufacturing => 325510,
            Self::AdhesiveManufacturing => 325520,
            Self::SoapAndOtherDetergentManufacturing => 325611,
            Self::PolishAndOtherSanitationGoodManufacturing => 325612,
            Self::SurfaceActiveAgentManufacturing => 325613,
            Self::ToiletPreparationManufacturing => 325620,
            Self::PrintingInkManufacturing => 325910,
            Self::ExplosivesManufacturing => 325920,
            Self::CustomCompoundingOfPurchasedResins => 325991,
            Self::PhotographicFilmPaperPlateChemicalAndCopyTonerManufacturing => 325992,
            Self::AllOtherMiscellaneousChemicalProductAndPreparationManufacturing => 325998,
            Self::PlasticsBagAndPouchManufacturing => 326111,
            Self::PlasticsPackagingFilmAndSheetIncludingLaminatedManufacturing => 326112,
            Self::UnlaminatedPlasticsFilmAndSheetExceptPackagingManufacturing => 326113,
            Self::UnlaminatedPlasticsProfileShapeManufacturing => 326121,
            Self::PlasticsPipeAndPipeFittingManufacturing => 326122,
            Self::LaminatedPlasticsPlateSheetExceptPackagingAndShapeManufacturing => 326130,
            Self::PolystyreneFoamProductManufacturing => 326140,
            Self::UrethaneAndOtherFoamProductExceptPolystyreneManufacturing => 326150,
            Self::PlasticsBottleManufacturing => 326160,
            Self::PlasticsPlumbingFixtureManufacturing => 326191,
            Self::AllOtherPlasticsProductManufacturing => 326199,
            Self::TireManufacturingExceptRetreading => 326211,
            Self::TireRetreading => 326212,
            Self::RubberAndPlasticsHosesAndBeltingManufacturing => 326220,
            Self::RubberProductManufacturingForMechanicalUse => 326291,
            Self::AllOtherRubberProductManufacturing => 326299,
            Self::PotteryCeramicsAndPlumbingFixtureManufacturing => 327110,
            Self::ClayBuildingMaterialAndRefractoriesManufacturing => 327120,
            Self::FlatGlassManufacturing => 327211,
            Self::OtherPressedAndBlownGlassAndGlasswareManufacturing => 327212,
            Self::GlassContainerManufacturing => 327213,
            Self::GlassProductManufacturingMadeOfPurchasedGlass => 327215,
            Self::CementManufacturing => 327310,
            Self::ReadymixConcreteManufacturing => 327320,
            Self::ConcreteBlockAndBrickManufacturing => 327331,
            Self::ConcretePipeManufacturing => 327332,
            Self::OtherConcreteProductManufacturing => 327390,
            Self::LimeManufacturing => 327410,
            Self::GypsumProductManufacturing => 327420,
            Self::AbrasiveProductManufacturing => 327910,
            Self::CutStoneAndStoneProductManufacturing => 327991,
            Self::GroundOrTreatedMineralAndEarthManufacturing => 327992,
            Self::MineralWoolManufacturing => 327993,
            Self::AllOtherMiscellaneousNonmetallicMineralProductManufacturing => 327999,
            Self::IronAndSteelMillsAndFerroalloyManufacturing => 331110,
            Self::IronAndSteelPipeAndTubeManufacturingFromPurchasedSteel => 331210,
            Self::RolledSteelShapeManufacturing => 331221,
            Self::SteelWireDrawing => 331222,
            Self::AluminaRefiningAndPrimaryAluminumProduction => 331313,
            Self::SecondarySmeltingAndAlloyingOfAluminum => 331314,
            Self::AluminumSheetPlateAndFoilManufacturing => 331315,
            Self::OtherAluminumRollingDrawingAndExtruding => 331318,
            Self::NonferrousMetalExceptAluminumSmeltingAndRefining => 331410,
            Self::CopperRollingDrawingExtrudingAndAlloying => 331420,
            Self::NonferrousMetalExceptCopperAndAluminumRollingDrawingAndExtruding => 331491,
            Self::SecondarySmeltingRefiningAndAlloyingOfNonferrousMetalExceptCopperAndAluminum => 331492,
            Self::IronFoundries => 331511,
            Self::SteelInvestmentFoundries => 331512,
            Self::SteelFoundriesExceptInvestment => 331513,
            Self::NonferrousMetalDiecastingFoundries => 331523,
            Self::AluminumFoundriesExceptDiecasting => 331524,
            Self::OtherNonferrousMetalFoundriesExceptDiecasting => 331529,
            Self::IronAndSteelForging => 332111,
            Self::NonferrousForging => 332112,
            Self::CustomRollForming => 332114,
            Self::PowderMetallurgyPartManufacturing => 332117,
            Self::MetalCrownClosureAndOtherMetalStampingExceptAutomotive => 332119,
            Self::MetalKitchenCookwareUtensilCutleryAndFlatwareExceptPreciousManufacturing => 332215,
            Self::SawBladeAndHandtoolManufacturing => 332216,
            Self::PrefabricatedMetalBuildingAndComponentManufacturing => 332311,
            Self::FabricatedStructuralMetalManufacturing => 332312,
            Self::PlateWorkManufacturing => 332313,
            Self::MetalWindowAndDoorManufacturing => 332321,
            Self::SheetMetalWorkManufacturing => 332322,
            Self::OrnamentalAndArchitecturalMetalWorkManufacturing => 332323,
            Self::PowerBoilerAndHeatExchangerManufacturing => 332410,
            Self::MetalTankHeavyGaugeManufacturing => 332420,
            Self::MetalCanManufacturing => 332431,
            Self::OtherMetalContainerManufacturing => 332439,
            Self::HardwareManufacturing => 332510,
            Self::SpringManufacturing => 332613,
            Self::OtherFabricatedWireProductManufacturing => 332618,
            Self::MachineShops => 332710,
            Self::PrecisionTurnedProductManufacturing => 332721,
            Self::BoltNutScrewRivetAndWasherManufacturing => 332722,
            Self::MetalHeatTreating => 332811,
            Self::MetalCoatingEngravingExceptJewelryAndSilverwareAndAlliedServicesToManufacturers => 332812,
            Self::ElectroplatingPlatingPolishingAnodizingAndColoring => 332813,
            Self::IndustrialValveManufacturing => 332911,
            Self::FluidPowerValveAndHoseFittingManufacturing => 332912,
            Self::PlumbingFixtureFittingAndTrimManufacturing => 332913,
            Self::OtherMetalValveAndPipeFittingManufacturing => 332919,
            Self::BallAndRollerBearingManufacturing => 332991,
            Self::SmallArmsAmmunitionManufacturing => 332992,
            Self::AmmunitionExceptSmallArmsManufacturing => 332993,
            Self::SmallArmsOrdnanceAndOrdnanceAccessoriesManufacturing => 332994,
            Self::FabricatedPipeAndPipeFittingManufacturing => 332996,
            Self::AllOtherMiscellaneousFabricatedMetalProductManufacturing => 332999,
            Self::FarmMachineryAndEquipmentManufacturing => 333111,
            Self::LawnAndGardenTractorAndHomeLawnAndGardenEquipmentManufacturing => 333112,
            Self::ConstructionMachineryManufacturing => 333120,
            Self::MiningMachineryAndEquipmentManufacturing => 333131,
            Self::OilAndGasFieldMachineryAndEquipmentManufacturing => 333132,
            Self::FoodProductMachineryManufacturing => 333241,
            Self::SemiconductorMachineryManufacturing => 333242,
            Self::SawmillWoodworkingAndPaperMachineryManufacturing => 333243,
            Self::AllOtherIndustrialMachineryManufacturing => 333248,
            Self::CommercialAndServiceIndustryMachineryManufacturing => 333310,
            Self::IndustrialAndCommercialFanAndBlowerAndAirPurificationEquipmentManufacturing => 333413,
            Self::HeatingEquipmentExceptWarmAirFurnacesManufacturing => 333414,
            Self::AirconditioningAndWarmAirHeatingEquipmentAndCommercialAndIndustrialRefrigerationEquipmentManufacturing => 333415,
            Self::IndustrialMoldManufacturing => 333511,
            Self::SpecialDieAndToolDieSetJigAndFixtureManufacturing => 333514,
            Self::CuttingToolAndMachineToolAccessoryManufacturing => 333515,
            Self::MachineToolManufacturing => 333517,
            Self::RollingMillAndOtherMetalworkingMachineryManufacturing => 333519,
            Self::TurbineAndTurbineGeneratorSetUnitsManufacturing => 333611,
            Self::SpeedChangerIndustrialHighspeedDriveAndGearManufacturing => 333612,
            Self::MechanicalPowerTransmissionEquipmentManufacturing => 333613,
            Self::OtherEngineEquipmentManufacturing => 333618,
            Self::AirAndGasCompressorManufacturing => 333912,
            Self::MeasuringDispensingAndOtherPumpingEquipmentManufacturing => 333914,
            Self::ElevatorAndMovingStairwayManufacturing => 333921,
            Self::ConveyorAndConveyingEquipmentManufacturing => 333922,
            Self::OverheadTravelingCraneHoistAndMonorailSystemManufacturing => 333923,
            Self::IndustrialTruckTractorTrailerAndStackerMachineryManufacturing => 333924,
            Self::PowerdrivenHandtoolManufacturing => 333991,
            Self::WeldingAndSolderingEquipmentManufacturing => 333992,
            Self::PackagingMachineryManufacturing => 333993,
            Self::IndustrialProcessFurnaceAndOvenManufacturing => 333994,
            Self::FluidPowerCylinderAndActuatorManufacturing => 333995,
            Self::FluidPowerPumpAndMotorManufacturing => 333996,
            Self::AllOtherMiscellaneousGeneralPurposeMachineryManufacturing => 333998,
            Self::ElectronicComputerManufacturing => 334111,
            Self::ComputerStorageDeviceManufacturing => 334112,
            Self::ComputerTerminalAndOtherComputerPeripheralEquipmentManufacturing => 334118,
            Self::TelephoneApparatusManufacturing => 334210,
            Self::RadioAndTelevisionBroadcastingAndWirelessCommunicationsEquipmentManufacturing => 334220,
            Self::OtherCommunicationsEquipmentManufacturing => 334290,
            Self::AudioAndVideoEquipmentManufacturing => 334310,
            Self::BarePrintedCircuitBoardManufacturing => 334412,
            Self::SemiconductorAndRelatedDeviceManufacturing => 334413,
            Self::CapacitorResistorCoilTransformerAndOtherInductorManufacturing => 334416,
            Self::ElectronicConnectorManufacturing => 334417,
            Self::PrintedCircuitAssemblyElectronicAssemblyManufacturing => 334418,
            Self::OtherElectronicComponentManufacturing => 334419,
            Self::ElectromedicalAndElectrotherapeuticApparatusManufacturing => 334510,
            Self::SearchDetectionNavigationGuidanceAeronauticalAndNauticalSystemAndInstrumentManufacturing => 334511,
            Self::AutomaticEnvironmentalControlManufacturingForResidentialCommercialAndApplianceUse => 334512,
            Self::InstrumentsAndRelatedProductsManufacturingForMeasuringDisplayingAndControllingIndustrialProcessVariables => 334513,
            Self::TotalizingFluidMeterAndCountingDeviceManufacturing => 334514,
            Self::InstrumentManufacturingForMeasuringAndTestingElectricityAndElectricalSignals => 334515,
            Self::AnalyticalLaboratoryInstrumentManufacturing => 334516,
            Self::IrradiationApparatusManufacturing => 334517,
            Self::OtherMeasuringAndControllingDeviceManufacturing => 334519,
            Self::ManufacturingAndReproducingMagneticAndOpticalMedia => 334610,
            Self::ResidentialElectricLightingFixtureManufacturing => 335131,
            Self::CommercialIndustrialAndInstitutionalElectricLightingFixtureManufacturing => 335132,
            Self::ElectricLampBulbAndOtherLightingEquipmentManufacturing => 335139,
            Self::SmallElectricalApplianceManufacturing => 335210,
            Self::MajorHouseholdApplianceManufacturing => 335220,
            Self::PowerDistributionAndSpecialtyTransformerManufacturing => 335311,
            Self::MotorAndGeneratorManufacturing => 335312,
            Self::SwitchgearAndSwitchboardApparatusManufacturing => 335313,
            Self::RelayAndIndustrialControlManufacturing => 335314,
            Self::BatteryManufacturing => 335910,
            Self::FiberOpticCableManufacturing => 335921,
            Self::OtherCommunicationAndEnergyWireManufacturing => 335929,
            Self::CurrentcarryingWiringDeviceManufacturing => 335931,
            Self::NoncurrentcarryingWiringDeviceManufacturing => 335932,
            Self::CarbonAndGraphiteProductManufacturing => 335991,
            Self::AllOtherMiscellaneousElectricalEquipmentAndComponentManufacturing => 335999,
            Self::AutomobileAndLightDutyMotorVehicleManufacturing => 336110,
            Self::HeavyDutyTruckManufacturing => 336120,
            Self::MotorVehicleBodyManufacturing => 336211,
            Self::TruckTrailerManufacturing => 336212,
            Self::MotorHomeManufacturing => 336213,
            Self::TravelTrailerAndCamperManufacturing => 336214,
            Self::MotorVehicleGasolineEngineAndEnginePartsManufacturing => 336310,
            Self::MotorVehicleElectricalAndElectronicEquipmentManufacturing => 336320,
            Self::MotorVehicleSteeringAndSuspensionComponentsExceptSpringManufacturing => 336330,
            Self::MotorVehicleBrakeSystemManufacturing => 336340,
            Self::MotorVehicleTransmissionAndPowerTrainPartsManufacturing => 336350,
            Self::MotorVehicleSeatingAndInteriorTrimManufacturing => 336360,
            Self::MotorVehicleMetalStamping => 336370,
            Self::OtherMotorVehiclePartsManufacturing => 336390,
            Self::AircraftManufacturing => 336411,
            Self::AircraftEngineAndEnginePartsManufacturing => 336412,
            Self::OtherAircraftPartsAndAuxiliaryEquipmentManufacturing => 336413,
            Self::GuidedMissileAndSpaceVehicleManufacturing => 336414,
            Self::GuidedMissileAndSpaceVehiclePropulsionUnitAndPropulsionUnitPartsManufacturing => 336415,
            Self::OtherGuidedMissileAndSpaceVehiclePartsAndAuxiliaryEquipmentManufacturing => 336419,
            Self::RailroadRollingStockManufacturing => 336510,
            Self::ShipBuildingAndRepairing => 336611,
            Self::BoatBuilding => 336612,
            Self::MotorcycleBicycleAndPartsManufacturing => 336991,
            Self::MilitaryArmoredVehicleTankAndTankComponentManufacturing => 336992,
            Self::AllOtherTransportationEquipmentManufacturing => 336999,
            Self::WoodKitchenCabinetAndCountertopManufacturing => 337110,
            Self::UpholsteredHouseholdFurnitureManufacturing => 337121,
            Self::NonupholsteredWoodHouseholdFurnitureManufacturing => 337122,
            Self::HouseholdFurnitureExceptWoodAndUpholsteredManufacturing => 337126,
            Self::InstitutionalFurnitureManufacturing => 337127,
            Self::WoodOfficeFurnitureManufacturing => 337211,
            Self::CustomArchitecturalWoodworkAndMillworkManufacturing => 337212,
            Self::OfficeFurnitureExceptWoodManufacturing => 337214,
            Self::ShowcasePartitionShelvingAndLockerManufacturing => 337215,
            Self::MattressManufacturing => 337910,
            Self::BlindAndShadeManufacturing => 337920,
            Self::SurgicalAndMedicalInstrumentManufacturing => 339112,
            Self::SurgicalApplianceAndSuppliesManufacturing => 339113,
            Self::DentalEquipmentAndSuppliesManufacturing => 339114,
            Self::OphthalmicGoodsManufacturing => 339115,
            Self::DentalLaboratories => 339116,
            Self::JewelryAndSilverwareManufacturing => 339910,
            Self::SportingAndAthleticGoodsManufacturing => 339920,
            Self::DollToyAndGameManufacturing => 339930,
            Self::OfficeSuppliesExceptPaperManufacturing => 339940,
            Self::SignManufacturing => 339950,
            Self::GasketPackingAndSealingDeviceManufacturing => 339991,
            Self::MusicalInstrumentManufacturing => 339992,
            Self::FastenerButtonNeedleAndPinManufacturing => 339993,
            Self::BroomBrushAndMopManufacturing => 339994,
            Self::BurialCasketManufacturing => 339995,
            Self::AllOtherMiscellaneousManufacturing => 339999,
            Self::AutomobileAndOtherMotorVehicleMerchantWholesalers => 423110,
            Self::MotorVehicleSuppliesAndNewPartsMerchantWholesalers => 423120,
            Self::TireAndTubeMerchantWholesalers => 423130,
            Self::MotorVehiclePartsUsedMerchantWholesalers => 423140,
            Self::FurnitureMerchantWholesalers => 423210,
            Self::HomeFurnishingMerchantWholesalers => 423220,
            Self::LumberPlywoodMillworkAndWoodPanelMerchantWholesalers => 423310,
            Self::BrickStoneAndRelatedConstructionMaterialMerchantWholesalers => 423320,
            Self::RoofingSidingAndInsulationMaterialMerchantWholesalers => 423330,
            Self::OtherConstructionMaterialMerchantWholesalers => 423390,
            Self::PhotographicEquipmentAndSuppliesMerchantWholesalers => 423410,
            Self::OfficeEquipmentMerchantWholesalers => 423420,
            Self::ComputerAndComputerPeripheralEquipmentAndSoftwareMerchantWholesalers => 423430,
            Self::OtherCommercialEquipmentMerchantWholesalers => 423440,
            Self::MedicalDentalAndHospitalEquipmentAndSuppliesMerchantWholesalers => 423450,
            Self::OphthalmicGoodsMerchantWholesalers => 423460,
            Self::OtherProfessionalEquipmentAndSuppliesMerchantWholesalers => 423490,
            Self::MetalServiceCentersAndOtherMetalMerchantWholesalers => 423510,
            Self::CoalAndOtherMineralAndOreMerchantWholesalers => 423520,
            Self::ElectricalApparatusAndEquipmentWiringSuppliesAndRelatedEquipmentMerchantWholesalers => 423610,
            Self::HouseholdAppliancesElectricHousewaresAndConsumerElectronicsMerchantWholesalers => 423620,
            Self::OtherElectronicPartsAndEquipmentMerchantWholesalers => 423690,
            Self::HardwareMerchantWholesalers => 423710,
            Self::PlumbingAndHeatingEquipmentAndSuppliesHydronicsMerchantWholesalers => 423720,
            Self::WarmAirHeatingAndAirconditioningEquipmentAndSuppliesMerchantWholesalers => 423730,
            Self::RefrigerationEquipmentAndSuppliesMerchantWholesalers => 423740,
            Self::ConstructionAndMiningExceptOilWellMachineryAndEquipmentMerchantWholesalers => 423810,
            Self::FarmAndGardenMachineryAndEquipmentMerchantWholesalers => 423820,
            Self::IndustrialMachineryAndEquipmentMerchantWholesalers => 423830,
            Self::IndustrialSuppliesMerchantWholesalers => 423840,
            Self::ServiceEstablishmentEquipmentAndSuppliesMerchantWholesalers => 423850,
            Self::TransportationEquipmentAndSuppliesExceptMotorVehicleMerchantWholesalers => 423860,
            Self::SportingAndRecreationalGoodsAndSuppliesMerchantWholesalers => 423910,
            Self::ToyAndHobbyGoodsAndSuppliesMerchantWholesalers => 423920,
            Self::RecyclableMaterialMerchantWholesalers => 423930,
            Self::JewelryWatchPreciousStoneAndPreciousMetalMerchantWholesalers => 423940,
            Self::OtherMiscellaneousDurableGoodsMerchantWholesalers => 423990,
            Self::PrintingAndWritingPaperMerchantWholesalers => 424110,
            Self::StationeryAndOfficeSuppliesMerchantWholesalers => 424120,
            Self::IndustrialAndPersonalServicePaperMerchantWholesalers => 424130,
            Self::DrugsAndDruggistsSundriesMerchantWholesalers => 424210,
            Self::PieceGoodsNotionsAndOtherDryGoodsMerchantWholesalers => 424310,
            Self::FootwearMerchantWholesalers => 424340,
            Self::ClothingAndClothingAccessoriesMerchantWholesalers => 424350,
            Self::GeneralLineGroceryMerchantWholesalers => 424410,
            Self::PackagedFrozenFoodMerchantWholesalers => 424420,
            Self::DairyProductExceptDriedOrCannedMerchantWholesalers => 424430,
            Self::PoultryAndPoultryProductMerchantWholesalers => 424440,
            Self::ConfectioneryMerchantWholesalers => 424450,
            Self::FishAndSeafoodMerchantWholesalers => 424460,
            Self::MeatAndMeatProductMerchantWholesalers => 424470,
            Self::FreshFruitAndVegetableMerchantWholesalers => 424480,
            Self::OtherGroceryAndRelatedProductsMerchantWholesalers => 424490,
            Self::GrainAndFieldBeanMerchantWholesalers => 424510,
            Self::LivestockMerchantWholesalers => 424520,
            Self::OtherFarmProductRawMaterialMerchantWholesalers => 424590,
            Self::PlasticsMaterialsAndBasicFormsAndShapesMerchantWholesalers => 424610,
            Self::OtherChemicalAndAlliedProductsMerchantWholesalers => 424690,
            Self::PetroleumBulkStationsAndTerminals => 424710,
            Self::PetroleumAndPetroleumProductsMerchantWholesalersExceptBulkStationsAndTerminals => 424720,
            Self::BeerAndAleMerchantWholesalers => 424810,
            Self::WineAndDistilledAlcoholicBeverageMerchantWholesalers => 424820,
            Self::FarmSuppliesMerchantWholesalers => 424910,
            Self::BookPeriodicalAndNewspaperMerchantWholesalers => 424920,
            Self::FlowerNurseryStockAndFloristsSuppliesMerchantWholesalers => 424930,
            Self::TobaccoProductAndElectronicCigaretteMerchantWholesalers => 424940,
            Self::PaintVarnishAndSuppliesMerchantWholesalers => 424950,
            Self::OtherMiscellaneousNondurableGoodsMerchantWholesalers => 424990,
            Self::WholesaleTradeAgentsAndBrokers => 425120,
            Self::NewCarDealers => 441110,
            Self::UsedCarDealers => 441120,
            Self::RecreationalVehicleDealers => 441210,
            Self::BoatDealers => 441222,
            Self::MotorcycleAtvAndAllOtherMotorVehicleDealers => 441227,
            Self::AutomotivePartsAndAccessoriesRetailers => 441330,
            Self::TireDealers => 441340,
            Self::HomeCenters => 444110,
            Self::PaintAndWallpaperRetailers => 444120,
            Self::HardwareRetailers => 444140,
            Self::OtherBuildingMaterialDealers => 444180,
            Self::OutdoorPowerEquipmentRetailers => 444230,
            Self::NurseryGardenCenterAndFarmSupplyRetailers => 444240,
            Self::SupermarketsAndOtherGroceryRetailersExceptConvenienceRetailers => 445110,
            Self::ConvenienceRetailers => 445131,
            Self::VendingMachineOperators => 445132,
            Self::FruitAndVegetableRetailers => 445230,
            Self::MeatRetailers => 445240,
            Self::FishAndSeafoodRetailers => 445250,
            Self::BakedGoodsRetailers => 445291,
            Self::ConfectioneryAndNutRetailers => 445292,
            Self::AllOtherSpecialtyFoodRetailers => 445298,
            Self::BeerWineAndLiquorRetailers => 445320,
            Self::FurnitureRetailers => 449110,
            Self::FloorCoveringRetailers => 449121,
            Self::WindowTreatmentRetailers => 449122,
            Self::AllOtherHomeFurnishingsRetailers => 449129,
            Self::ElectronicsAndApplianceRetailers => 449210,
            Self::DepartmentStores => 455110,
            Self::WarehouseClubsAndSupercenters => 455211,
            Self::AllOtherGeneralMerchandiseRetailers => 455219,
            Self::PharmaciesAndDrugRetailers => 456110,
            Self::CosmeticsBeautySuppliesAndPerfumeRetailers => 456120,
            Self::OpticalGoodsRetailers => 456130,
            Self::FoodHealthSupplementRetailers => 456191,
            Self::AllOtherHealthAndPersonalCareRetailers => 456199,
            Self::GasolineStationsWithConvenienceStores => 457110,
            Self::OtherGasolineStations => 457120,
            Self::FuelDealers => 457210,
            Self::ClothingRetailers => 458110,
            Self::ClothingAccessoriesRetailers => 458120,
            Self::ShoeRetailers => 458210,
            Self::JewelryRetailers => 458310,
            Self::LuggageAndLeatherGoodsRetailers => 458320,
            Self::SportingGoodsRetailers => 459110,
            Self::HobbyToyAndGameRetailers => 459120,
            Self::SewingNeedleworkAndPieceGoodsRetailers => 459130,
            Self::MusicalInstrumentAndSuppliesRetailers => 459140,
            Self::BookRetailersAndNewsDealers => 459210,
            Self::Florists => 459310,
            Self::OfficeSuppliesAndStationeryRetailers => 459410,
            Self::GiftNoveltyAndSouvenirRetailers => 459420,
            Self::UsedMerchandiseRetailers => 459510,
            Self::PetAndPetSuppliesRetailers => 459910,
            Self::ArtDealers => 459920,
            Self::ManufacturedMobileHomeDealers => 459930,
            Self::TobaccoElectronicCigaretteAndOtherSmokingSuppliesRetailers => 459991,
            Self::AllOtherMiscellaneousRetailers => 459999,
            Self::ScheduledPassengerAirTransportation => 481111,
            Self::ScheduledFreightAirTransportation => 481112,
            Self::NonscheduledCharteredPassengerAirTransportation => 481211,
            Self::NonscheduledCharteredFreightAirTransportation => 481212,
            Self::OtherNonscheduledAirTransportation => 481219,
            Self::LineHaulRailroads => 482111,
            Self::ShortLineRailroads => 482112,
            Self::DeepSeaFreightTransportation => 483111,
            Self::DeepSeaPassengerTransportation => 483112,
            Self::CoastalAndGreatLakesFreightTransportation => 483113,
            Self::CoastalAndGreatLakesPassengerTransportation => 483114,
            Self::InlandWaterFreightTransportation => 483211,
            Self::InlandWaterPassengerTransportation => 483212,
            Self::GeneralFreightTruckingLocal => 484110,
            Self::GeneralFreightTruckingLongdistanceTruckload => 484121,
            Self::GeneralFreightTruckingLongdistanceLessThanTruckload => 484122,
            Self::UsedHouseholdAndOfficeGoodsMoving => 484210,
            Self::SpecializedFreightExceptUsedGoodsTruckingLocal => 484220,
            Self::SpecializedFreightExceptUsedGoodsTruckingLongdistance => 484230,
            Self::MixedModeTransitSystems => 485111,
            Self::CommuterRailSystems => 485112,
            Self::BusAndOtherMotorVehicleTransitSystems => 485113,
            Self::StreetRailroads => 485114,
            Self::LightRailTransitSystems => 485115,
            Self::OtherUrbanTransitSystems => 485119,
            Self::InterurbanAndRuralBusTransportation => 485210,
            Self::TaxiService => 485310,
            Self::LimousineService => 485320,
            Self::SchoolAndEmployeeBusTransportation => 485410,
            Self::CharterBusIndustry => 485510,
            Self::SpecialNeedsTransportation => 485991,
            Self::AllOtherTransitAndGroundPassengerTransportation => 485999,
            Self::PipelineTransportationOfCrudeOil => 486110,
            Self::PipelineTransportationOfNaturalGas => 486210,
            Self::PipelineTransportationOfRefinedPetroleumProducts => 486910,
            Self::AllOtherPipelineTransportation => 486990,
            Self::ScenicAndSightseeingTransportationLand => 487110,
            Self::ScenicAndSightseeingTransportationWater => 487210,
            Self::ScenicAndSightseeingTransportationOther => 487990,
            Self::AirTrafficControl => 488111,
            Self::OtherAirportOperations => 488119,
            Self::OtherSupportActivitiesForAirTransportation => 488190,
            Self::SupportActivitiesForRailTransportation => 488210,
            Self::PortAndHarborOperations => 488310,
            Self::MarineCargoHandling => 488320,
            Self::NavigationalServicesToShipping => 488330,
            Self::OtherSupportActivitiesForWaterTransportation => 488390,
            Self::MotorVehicleTowing => 488410,
            Self::OtherSupportActivitiesForRoadTransportation => 488490,
            Self::FreightTransportationArrangement => 488510,
            Self::PackingAndCrating => 488991,
            Self::AllOtherSupportActivitiesForTransportation => 488999,
            Self::PostalService => 491110,
            Self::CouriersAndExpressDeliveryServices => 492110,
            Self::LocalMessengersAndLocalDelivery => 492210,
            Self::GeneralWarehousingAndStorage => 493110,
            Self::RefrigeratedWarehousingAndStorage => 493120,
            Self::FarmProductWarehousingAndStorage => 493130,
            Self::OtherWarehousingAndStorage => 493190,
            Self::NewspaperPublishers => 511110,
            Self::PeriodicalPublishers => 511120,
            Self::BookPublishers => 511130,
            Self::DirectoryAndMailingListPublishers => 511140,
            Self::GreetingCardPublishers => 511191,
            Self::AllOtherPublishers => 511199,
            Self::SoftwarePublishers => 511210,
            Self::MotionPictureAndVideoProduction => 512110,
            Self::MotionPictureAndVideoDistribution => 512120,
            Self::MotionPictureTheatersExceptDriveins => 512131,
            Self::DriveInMotionPictureTheaters => 512132,
            Self::TeleproductionAndOtherPostproductionServices => 512191,
            Self::OtherMotionPictureAndVideoIndustries => 512199,
            Self::MusicPublishers => 512230,
            Self::SoundRecordingStudios => 512240,
            Self::RecordProductionAndDistribution => 512250,
            Self::OtherSoundRecordingIndustries => 512290,
            Self::RadioNetworks => 515111,
            Self::RadioStations => 515112,
            Self::TelevisionBroadcasting => 515120,
            Self::CableAndOtherSubscriptionProgramming => 515210,
            Self::WiredTelecommunicationsCarriers => 517311,
            Self::WirelessTelecommunicationsCarriersExceptSatellite => 517312,
            Self::SatelliteTelecommunications => 517410,
            Self::TelecommunicationsResellers => 517911,
            Self::AllOtherTelecommunications => 517919,
            Self::ComputingInfrastructureProvidersDataProcessingWebHostingAndRelatedServices => 518210,
            Self::NewsSyndicates => 519110,
            Self::LibrariesAndArchives => 519120,
            Self::InternetPublishingAndBroadcastingAndWebSearchPortals => 519130,
            Self::AllOtherInformationServices => 519190,
            Self::MonetaryAuthoritiesCentralBank => 521110,
            Self::CommercialBanking => 522110,
            Self::SavingsInstitutions => 522120,
            Self::CreditUnions => 522130,
            Self::OtherDepositoryCreditIntermediation => 522190,
            Self::CreditCardIssuing => 522210,
            Self::SalesFinancing => 522220,
            Self::ConsumerLending => 522291,
            Self::RealEstateCredit => 522292,
            Self::AllOtherNondepositoryCreditIntermediation => 522298,
            Self::MortgageAndNonmortgageLoanBrokers => 522310,
            Self::FinancialTransactionsProcessingReserveAndClearinghouseActivities => 522320,
            Self::OtherActivitiesRelatedToCreditIntermediation => 522390,
            Self::InvestmentBankingAndSecuritiesDealing => 523110,
            Self::SecuritiesBrokerage => 523120,
            Self::CommodityContractsDealing => 523130,
            Self::CommodityContractsBrokerage => 523140,
            Self::SecuritiesAndCommodityExchanges => 523210,
            Self::MiscellaneousIntermediation => 523910,
            Self::PortfolioManagement => 523920,
            Self::InvestmentAdvice => 523930,
            Self::TrustFiduciaryAndCustodyActivities => 523991,
            Self::MiscellaneousFinancialInvestmentActivities => 523999,
            Self::DirectLifeInsuranceCarriers => 524113,
            Self::DirectHealthAndMedicalInsuranceCarriers => 524114,
            Self::DirectPropertyAndCasualtyInsuranceCarriers => 524126,
            Self::DirectTitleInsuranceCarriers => 524127,
            Self::OtherDirectInsuranceExceptLifeHealthAndMedicalCarriers => 524128,
            Self::ReinsuranceCarriers => 524130,
            Self::InsuranceAgenciesAndBrokerages => 524210,
            Self::ClaimsAdjusting => 524291,
            Self::ThirdPartyAdministrationOfInsuranceAndPensionFunds => 524292,
            Self::AllOtherInsuranceRelatedActivities => 524298,
            Self::PensionFunds => 525110,
            Self::HealthAndWelfareFunds => 525120,
            Self::OtherInsuranceFunds => 525190,
            Self::OpenEndInvestmentFunds => 525910,
            Self::TrustsEstatesAndAgencyAccounts => 525920,
            Self::RealEstateInvestmentTrusts => 525930,
            Self::OtherFinancialVehicles => 525990,
            Self::LessorsOfResidentialBuildingsAndDwellings => 531110,
            Self::LessorsOfNonresidentialBuildingsExceptMiniwarehouses => 531120,
            Self::LessorsOfMiniwarehousesAndSelfStorageUnits => 531130,
            Self::LessorsOfOtherRealEstateProperty => 531190,
            Self::OfficesOfRealEstateAgentsAndBrokers => 531210,
            Self::ResidentialPropertyManagers => 531311,
            Self::NonresidentialPropertyManagers => 531312,
            Self::OfficesOfRealEstateAppraisers => 531320,
            Self::OtherActivitiesRelatedToRealEstate => 531390,
            Self::PassengerCarRental => 532111,
            Self::PassengerCarLeasing => 532112,
            Self::TruckUtilityTrailerAndRvRecreationalVehicleRentalAndLeasing => 532120,
            Self::ConsumerElectronicsAndAppliancesRental => 532210,
            Self::FormalWearAndCostumeRental => 532220,
            Self::VideoTapeAndDiscRental => 532230,
            Self::RecreationalGoodsRentalParent => 532281,
            Self::MedicalEquipmentAndSuppliesRental => 532282,
            Self::HomeHealthEquipmentAndSuppliesRental => 532283,
            Self::RecreationalGoodsRental => 532284,
            Self::AllOtherConsumerGoodsRental => 532289,
            Self::GeneralRentalCenters => 532310,
            Self::CommercialAirRailAndWaterTransportationEquipmentRentalAndLeasing => 532411,
            Self::ConstructionMiningAndForestryMachineryAndEquipmentRentalAndLeasing => 532412,
            Self::OfficeMachineryAndEquipmentRentalAndLeasing => 532420,
            Self::OtherCommercialAndIndustrialMachineryAndEquipmentRentalAndLeasing => 532490,
            Self::LessorsOfNonfinancialIntangibleAssetsExceptCopyrightedWorks => 533110,
            Self::OfficesOfLawyers => 541110,
            Self::OfficesOfNotaries => 541120,
            Self::TitleAbstractAndSettlementOffices => 541191,
            Self::AllOtherLegalServices => 541199,
            Self::OfficesOfCertifiedPublicAccountants => 541211,
            Self::TaxPreparationServices => 541213,
            Self::PayrollServices => 541214,
            Self::OtherAccountingServices => 541219,
            Self::ArchitecturalServices => 541310,
            Self::LandscapeArchitecturalServices => 541320,
            Self::EngineeringServices => 541330,
            Self::DraftingServices => 541340,
            Self::BuildingInspectionServices => 541350,
            Self::GeophysicalSurveyingAndMappingServices => 541360,
            Self::SurveyingAndMappingExceptGeophysicalServices => 541370,
            Self::TestingLaboratories => 541380,
            Self::InteriorDesignServices => 541410,
            Self::IndustrialDesignServices => 541420,
            Self::GraphicDesignServices => 541430,
            Self::OtherSpecializedDesignServices => 541490,
            Self::CustomComputerProgrammingServices => 541511,
            Self::ComputerSystemsDesignServices => 541512,
            Self::ComputerFacilitiesManagementServices => 541513,
            Self::OtherComputerRelatedServices => 541519,
            Self::AdministrativeManagementAndGeneralManagementConsultingServices => 541611,
            Self::HumanResourcesConsultingServices => 541612,
            Self::MarketingConsultingServices => 541613,
            Self::ProcessPhysicalDistributionAndLogisticsConsultingServices => 541614,
            Self::OtherManagementConsultingServices => 541618,
            Self::EnvironmentalConsultingServices => 541620,
            Self::OtherScientificAndTechnicalConsultingServices => 541690,
            Self::ResearchAndDevelopmentInNanotechnology => 541713,
            Self::ResearchAndDevelopmentInBiotechnologyExceptNanobiotechnology => 541714,
            Self::ResearchAndDevelopmentInThePhysicalEngineeringAndLifeSciencesExceptNanotechnologyAndBiotechnology => 541715,
            Self::ResearchAndDevelopmentInTheSocialSciencesAndHumanities => 541720,
            Self::AdvertisingAgencies => 541810,
            Self::PublicRelationsAgencies => 541820,
            Self::MediaBuyingAgencies => 541830,
            Self::MediaRepresentatives => 541840,
            Self::OutdoorAdvertising => 541850,
            Self::DirectMailAdvertising => 541860,
            Self::AdvertisingMaterialDistributionServices => 541870,
            Self::OtherServicesRelatedToAdvertising => 541890,
            Self::MarketingResearchAndPublicOpinionPolling => 541910,
            Self::PhotographyStudiosPortrait => 541921,
            Self::CommercialPhotography => 541922,
            Self::TranslationAndInterpretationServices => 541930,
            Self::VeterinaryServices => 541940,
            Self::AllOtherProfessionalScientificAndTechnicalServices => 541990,
            Self::OfficesOfBankHoldingCompanies => 551111,
            Self::OfficesOfOtherHoldingCompanies => 551112,
            Self::CorporateSubsidiaryAndRegionalManagingOfficesParent => 551113,
            Self::CorporateSubsidiaryAndRegionalManagingOffices => 551114,
            Self::OfficeAdministrativeServices => 561110,
            Self::FacilitiesSupportServices => 561210,
            Self::EmploymentPlacementAgencies => 561311,
            Self::ExecutiveSearchServices => 561312,
            Self::TemporaryHelpServices => 561320,
            Self::ProfessionalEmployerOrganizations => 561330,
            Self::DocumentPreparationServices => 561410,
            Self::TelephoneAnsweringServices => 561421,
            Self::TelemarketingBureausAndOtherContactCenters => 561422,
            Self::PrivateMailCenters => 561431,
            Self::OtherBusinessServiceCentersIncludingCopyShops => 561439,
            Self::CollectionAgencies => 561440,
            Self::CreditBureaus => 561450,
            Self::RepossessionServices => 561491,
            Self::CourtReportingAndStenotypeServices => 561492,
            Self::AllOtherBusinessSupportServices => 561499,
            Self::TravelAgencies => 561510,
            Self::TourOperators => 561520,
            Self::ConventionAndVisitorsBureaus => 561591,
            Self::AllOtherTravelArrangementAndReservationServices => 561599,
            Self::InvestigationServices => 561611,
            Self::SecurityGuardsAndPatrolServices => 561612,
            Self::ArmoredCarServices => 561613,
            Self::SecuritySystemsServicesExceptLocksmiths => 561621,
            Self::Locksmiths => 561622,
            Self::ExterminatingAndPestControlServices => 561710,
            Self::JanitorialServices => 561720,
            Self::LandscapingServices => 561730,
            Self::CarpetAndUpholsteryCleaningServices => 561740,
            Self::OtherServicesToBuildingsAndDwellings => 561790,
            Self::PackagingAndLabelingServices => 561910,
            Self::ConventionAndTradeShowOrganizers => 561920,
            Self::AllOtherSupportServices => 561990,
            Self::SolidWasteCollection => 562111,
            Self::HazardousWasteCollection => 562112,
            Self::OtherWasteCollection => 562119,
            Self::HazardousWasteTreatmentAndDisposal => 562211,
            Self::SolidWasteLandfill => 562212,
            Self::SolidWasteCombustorsAndIncinerators => 562213,
            Self::OtherNonhazardousWasteTreatmentAndDisposal => 562219,
            Self::RemediationServices => 562910,
            Self::MaterialsRecoveryFacilities => 562920,
            Self::SepticTankAndRelatedServices => 562991,
            Self::AllOtherMiscellaneousWasteManagementServices => 562998,
            Self::ElementaryAndSecondarySchools => 611110,
            Self::JuniorColleges => 611210,
            Self::CollegesUniversitiesAndProfessionalSchools => 611310,
            Self::BusinessAndSecretarialSchools => 611410,
            Self::ComputerTraining => 611420,
            Self::ProfessionalAndManagementDevelopmentTraining => 611430,
            Self::CosmetologyAndBarberSchools => 611511,
            Self::FlightTraining => 611512,
            Self::ApprenticeshipTraining => 611513,
            Self::OtherTechnicalAndTradeSchools => 611519,
            Self::FineArtsSchools => 611610,
            Self::SportsAndRecreationInstruction => 611620,
            Self::LanguageSchools => 611630,
            Self::ExamPreparationAndTutoring => 611691,
            Self::AutomobileDrivingSchools => 611692,
            Self::AllOtherMiscellaneousSchoolsAndInstruction => 611699,
            Self::EducationalSupportServices => 611710,
            Self::OfficesOfPhysiciansExceptMentalHealthSpecialists => 621111,
            Self::OfficesOfPhysiciansMentalHealthSpecialists => 621112,
            Self::OfficesOfDentists => 621210,
            Self::OfficesOfChiropractors => 621310,
            Self::OfficesOfOptometrists => 621320,
            Self::OfficesOfMentalHealthPractitionersExceptPhysicians => 621330,
            Self::OfficesOfPhysicalOccupationalAndSpeechTherapistsAndAudiologists => 621340,
            Self::OfficesOfPodiatrists => 621391,
            Self::OfficesOfAllOtherMiscellaneousHealthPractitioners => 621399,
            Self::FamilyPlanningCenters => 621410,
            Self::OutpatientMentalHealthAndSubstanceAbuseCenters => 621420,
            Self::HmoMedicalCenters => 621491,
            Self::KidneyDialysisCenters => 621492,
            Self::FreestandingAmbulatorySurgicalAndEmergencyCenters => 621493,
            Self::AllOtherOutpatientCareCenters => 621498,
            Self::MedicalLaboratories => 621511,
            Self::DiagnosticImagingCenters => 621512,
            Self::HomeHealthCareServices => 621610,
            Self::AmbulanceServices => 621910,
            Self::BloodAndOrganBanks => 621991,
            Self::AllOtherMiscellaneousAmbulatoryHealthCareServices => 621999,
            Self::GeneralMedicalAndSurgicalHospitals => 622110,
            Self::PsychiatricAndSubstanceAbuseHospitals => 622210,
            Self::SpecialtyExceptPsychiatricAndSubstanceAbuseHospitals => 622310,
            Self::NursingCareFacilitiesSkilledNursingFacilities => 623110,
            Self::ResidentialIntellectualAndDevelopmentalDisabilityFacilities => 623210,
            Self::ResidentialMentalHealthAndSubstanceAbuseFacilities => 623220,
            Self::ContinuingCareRetirementCommunities => 623311,
            Self::AssistedLivingFacilitiesForTheElderly => 623312,
            Self::OtherResidentialCareFacilities => 623990,
            Self::ChildAndYouthServices => 624110,
            Self::ServicesForTheElderlyAndPersonsWithDisabilities => 624120,
            Self::OtherIndividualAndFamilyServices => 624190,
            Self::CommunityFoodServices => 624210,
            Self::TemporaryShelters => 624221,
            Self::OtherCommunityHousingServices => 624229,
            Self::EmergencyAndOtherReliefServices => 624230,
            Self::VocationalRehabilitationServices => 624310,
            Self::ChildDayCareServices => 624410,
            Self::TheaterCompaniesAndDinnerTheaters => 711110,
            Self::DanceCompanies => 711120,
            Self::MusicalGroupsAndArtists => 711130,
            Self::OtherPerformingArtsCompanies => 711190,
            Self::SportsTeamsAndClubs => 711211,
            Self::Racetracks => 711212,
            Self::OtherSpectatorSports => 711219,
            Self::PromotersOfPerformingArtsSportsAndSimilarEventsWithFacilities => 711310,
            Self::PromotersOfPerformingArtsSportsAndSimilarEventsWithoutFacilities => 711320,
            Self::AgentsAndManagersForArtistsAthletesEntertainersAndOtherPublicFigures => 711410,
            Self::IndependentArtistsWritersAndPerformers => 711510,
            Self::Museums => 712110,
            Self::HistoricalSites => 712120,
            Self::ZoosAndBotanicalGardens => 712130,
            Self::NatureParksAndOtherSimilarInstitutions => 712190,
            Self::AmusementAndThemeParks => 713110,
            Self::AmusementArcades => 713120,
            Self::CasinosExceptCasinoHotels => 713210,
            Self::OtherGamblingIndustries => 713290,
            Self::GolfCoursesAndCountryClubs => 713910,
            Self::SkiingFacilities => 713920,
            Self::Marinas => 713930,
            Self::FitnessAndRecreationalSportsCenters => 713940,
            Self::BowlingCenters => 713950,
            Self::AllOtherAmusementAndRecreationIndustries => 713990,
            Self::HotelsExceptCasinoHotelsAndMotels => 721110,
            Self::CasinoHotels => 721120,
            Self::BedAndBreakfastInns => 721191,
            Self::AllOtherTravelerAccommodation => 721199,
            Self::RvRecreationalVehicleParksAndCampgrounds => 721211,
            Self::RecreationalAndVacationCampsExceptCampgrounds => 721214,
            Self::RoomingAndBoardingHouses => 721310,
            Self::FoodServiceContractors => 722310,
            Self::Caterers => 722320,
            Self::MobileFoodServices => 722330,
            Self::DrinkingPlacesAlcoholicBeverages => 722410,
            Self::FullServiceRestaurants => 722511,
            Self::LimitedServiceRestaurants => 722513,
            Self::CafeteriasGrillBuffetsAndBuffets => 722514,
            Self::SnackAndNonalcoholicBeverageBars => 722515,
            Self::GeneralAutomotiveRepair => 811111,
            Self::AutomotiveExhaustSystemRepair => 811112,
            Self::AutomotiveTransmissionRepair => 811113,
            Self::OtherAutomotiveMechanicalAndElectricalRepairAndMaintenance => 811118,
            Self::AutomotiveBodyPaintAndInteriorRepairAndMaintenance => 811121,
            Self::AutomotiveGlassReplacementShops => 811122,
            Self::AutomotiveOilChangeAndLubricationShops => 811191,
            Self::CarWashes => 811192,
            Self::AllOtherAutomotiveRepairAndMaintenance => 811198,
            Self::ConsumerElectronicsRepairAndMaintenance => 811211,
            Self::ComputerAndOfficeMachineRepairAndMaintenance => 811212,
            Self::CommunicationEquipmentRepairAndMaintenance => 811213,
            Self::OtherElectronicAndPrecisionEquipmentRepairAndMaintenance => 811219,
            Self::CommercialAndIndustrialMachineryAndEquipmentExceptAutomotiveAndElectronicRepairAndMaintenance => 811310,
            Self::HomeAndGardenEquipmentRepairAndMaintenance => 811411,
            Self::ApplianceRepairAndMaintenance => 811412,
            Self::ReupholsteryAndFurnitureRepair => 811420,
            Self::FootwearAndLeatherGoodsRepair => 811430,
            Self::OtherPersonalAndHouseholdGoodsRepairAndMaintenance => 811490,
            Self::BarberShops => 812111,
            Self::BeautySalons => 812112,
            Self::NailSalons => 812113,
            Self::DietAndWeightReducingCenters => 812191,
            Self::OtherPersonalCareServices => 812199,
            Self::FuneralHomesAndFuneralServices => 812210,
            Self::CemeteriesAndCrematories => 812220,
            Self::CoinOperatedLaundriesAndDrycleaners => 812310,
            Self::DrycleaningAndLaundryServicesExceptCoinOperated => 812320,
            Self::LinenSupply => 812331,
            Self::IndustrialLaunderers => 812332,
            Self::PetCareExceptVeterinaryServices => 812910,
            Self::PhotofinishingLaboratoriesExceptOneHour => 812921,
            Self::OneHourPhotofinishing => 812922,
            Self::ParkingLotsAndGarages => 812930,
            Self::AllOtherPersonalServices => 812990,
            Self::ReligiousOrganizations => 813110,
            Self::GrantmakingFoundations => 813211,
            Self::VoluntaryHealthOrganizations => 813212,
            Self::OtherGrantmakingAndGivingServices => 813219,
            Self::HumanRightsOrganizations => 813311,
            Self::EnvironmentConservationAndWildlifeOrganizations => 813312,
            Self::OtherSocialAdvocacyOrganizations => 813319,
            Self::CivicAndSocialOrganizations => 813410,
            Self::BusinessAssociations => 813910,
            Self::ProfessionalOrganizations => 813920,
            Self::LaborUnionsAndSimilarLaborOrganizations => 813930,
            Self::PoliticalOrganizations => 813940,
            Self::OtherSimilarOrganizationsExceptBusinessProfessionalLaborAndPoliticalOrganizations => 813990,
            Self::PrivateHouseholds => 814110,
            Self::ExecutiveOffices => 921110,
            Self::LegislativeBodies => 921120,
            Self::PublicFinanceActivities => 921130,
            Self::ExecutiveAndLegislativeOfficesCombined => 921140,
            Self::AmericanIndianAndAlaskaNativeTribalGovernments => 921150,
            Self::OtherGeneralGovernmentSupport => 921190,
            Self::Courts => 922110,
            Self::PoliceProtection => 922120,
            Self::LegalCounselAndProsecution => 922130,
            Self::CorrectionalInstitutions => 922140,
            Self::ParoleOfficesAndProbationOffices => 922150,
            Self::FireProtection => 922160,
            Self::OtherJusticePublicOrderAndSafetyActivities => 922190,
            Self::AdministrationOfEducationPrograms => 923110,
            Self::AdministrationOfPublicHealthPrograms => 923120,
            Self::AdministrationOfHumanResourceProgramsExceptEducationPublicHealthAndVeteransAffairsPrograms => 923130,
            Self::AdministrationOfVeteransAffairs => 923140,
            Self::AdministrationOfAirAndWaterResourceAndSolidWasteManagementPrograms => 924110,
            Self::AdministrationOfConservationPrograms => 924120,
            Self::AdministrationOfHousingPrograms => 925110,
            Self::AdministrationOfUrbanPlanningAndCommunityAndRuralDevelopment => 925120,
            Self::AdministrationOfGeneralEconomicPrograms => 926110,
            Self::RegulationAndAdministrationOfTransportationPrograms => 926120,
            Self::RegulationAndAdministrationOfCommunicationsElectricGasAndOtherUtilities => 926130,
            Self::RegulationOfAgriculturalMarketingAndCommodities => 926140,
            Self::RegulationLicensingAndInspectionOfMiscellaneousCommercialSectors => 926150,
            Self::SpaceResearchAndTechnology => 927110,
            Self::NationalSecurity => 928110,
            Self::InternationalAffairs => 928120,
            Self::UnclassifiedEstablishments => 999999,
        }
    }

    /// Creates a new NaicsIndustry from a NAICS code string
    /// Returns Some(NaicsIndustry) if the code matches a known industry, or None if not found
    pub fn from_code(code_str: &str) -> Option<Self> {
        // Parse the string to an i64
        let code = match code_str.parse::<i64>() {
            Ok(num) => num,
            Err(_) => return None,
        };

        // Match the code to an industry
        let industry = match code {
            111110 => Self::SoybeanFarming,
            111120 => Self::OilseedExceptSoybeanFarming,
            111130 => Self::DryPeaAndBeanFarming,
            111140 => Self::WheatFarming,
            111150 => Self::CornFarming,
            111160 => Self::RiceFarming,
            111191 => Self::OilseedAndGrainCombinationFarming,
            111199 => Self::AllOtherGrainFarming,
            111211 => Self::PotatoFarming,
            111219 => Self::OtherVegetableExceptPotatoAndMelonFarming,
            111310 => Self::OrangeGroves,
            111320 => Self::CitrusExceptOrangeGroves,
            111331 => Self::AppleOrchards,
            111332 => Self::GrapeVineyards,
            111333 => Self::StrawberryFarming,
            111334 => Self::BerryExceptStrawberryFarming,
            111335 => Self::TreeNutFarming,
            111336 => Self::FruitAndTreeNutCombinationFarming,
            111339 => Self::OtherNoncitrusFruitFarming,
            111411 => Self::MushroomProduction,
            111419 => Self::OtherFoodCropsGrownUnderCover,
            111421 => Self::NurseryAndTreeProduction,
            111422 => Self::FloricultureProduction,
            111910 => Self::TobaccoFarming,
            111920 => Self::CottonFarming,
            111930 => Self::SugarcaneFarming,
            111940 => Self::HayFarming,
            111991 => Self::SugarBeetFarming,
            111992 => Self::PeanutFarming,
            111998 => Self::AllOtherMiscellaneousCropFarming,
            112111 => Self::BeefCattleRanchingAndFarming,
            112112 => Self::CattleFeedlots,
            112120 => Self::DairyCattleAndMilkProduction,
            112130 => Self::DualpurposeCattleRanchingAndFarming,
            112210 => Self::HogAndPigFarming,
            112310 => Self::ChickenEggProduction,
            112320 => Self::BroilersAndOtherMeatTypeChickenProduction,
            112330 => Self::TurkeyProduction,
            112340 => Self::PoultryHatcheries,
            112390 => Self::OtherPoultryProduction,
            112410 => Self::SheepFarming,
            112420 => Self::GoatFarming,
            112511 => Self::FinfishFarmingAndFishHatcheries,
            112512 => Self::ShellfishFarming,
            112519 => Self::OtherAquaculture,
            112910 => Self::Apiculture,
            112920 => Self::HorsesAndOtherEquineProduction,
            112930 => Self::FurbearingAnimalAndRabbitProduction,
            112990 => Self::AllOtherAnimalProduction,
            113110 => Self::TimberTractOperations,
            113210 => Self::ForestNurseriesAndGatheringOfForestProducts,
            113310 => Self::Logging,
            114111 => Self::FinfishFishing,
            114112 => Self::ShellfishFishing,
            114119 => Self::OtherMarineFishing,
            114210 => Self::HuntingAndTrapping,
            115111 => Self::CottonGinning,
            115112 => Self::SoilPreparationPlantingAndCultivating,
            115113 => Self::CropHarvestingPrimarilyByMachine,
            115114 => Self::PostharvestCropActivitiesExceptCottonGinning,
            115115 => Self::FarmLaborContractorsAndCrewLeaders,
            115116 => Self::FarmManagementServices,
            115210 => Self::SupportActivitiesForAnimalProduction,
            115310 => Self::SupportActivitiesForForestry,
            211120 => Self::CrudePetroleumExtraction,
            211130 => Self::NaturalGasExtraction,
            212114 => Self::SurfaceCoalMining,
            212115 => Self::UndergroundCoalMining,
            212210 => Self::IronOreMining,
            212220 => Self::GoldOreAndSilverOreMining,
            212230 => Self::CopperNickelLeadAndZincMining,
            212290 => Self::OtherMetalOreMining,
            212311 => Self::DimensionStoneMiningAndQuarrying,
            212312 => Self::CrushedAndBrokenLimestoneMiningAndQuarrying,
            212313 => Self::CrushedAndBrokenGraniteMiningAndQuarrying,
            212319 => Self::OtherCrushedAndBrokenStoneMiningAndQuarrying,
            212321 => Self::ConstructionSandAndGravelMining,
            212322 => Self::IndustrialSandMining,
            212323 => Self::KaolinClayAndCeramicAndRefractoryMineralsMining,
            212390 => Self::OtherNonmetallicMineralMiningAndQuarrying,
            213111 => Self::DrillingOilAndGasWells,
            213112 => Self::SupportActivitiesForOilAndGasOperations,
            213113 => Self::SupportActivitiesForCoalMining,
            213114 => Self::SupportActivitiesForMetalMining,
            213115 => Self::SupportActivitiesForNonmetallicMineralsExceptFuelsMining,
            221111 => Self::HydroelectricPowerGeneration,
            221112 => Self::FossilFuelElectricPowerGeneration,
            221113 => Self::NuclearElectricPowerGeneration,
            221114 => Self::SolarElectricPowerGeneration,
            221115 => Self::WindElectricPowerGeneration,
            221116 => Self::GeothermalElectricPowerGeneration,
            221117 => Self::BiomassElectricPowerGeneration,
            221118 => Self::OtherElectricPowerGeneration,
            221121 => Self::ElectricBulkPowerTransmissionAndControl,
            221122 => Self::ElectricPowerDistribution,
            221210 => Self::NaturalGasDistribution,
            221310 => Self::WaterSupplyAndIrrigationSystems,
            221320 => Self::SewageTreatmentFacilities,
            221330 => Self::SteamAndAirconditioningSupply,
            236115 => Self::NewSinglefamilyHousingConstructionExceptForsaleBuilders,
            236116 => Self::NewMultifamilyHousingConstructionExceptForsaleBuilders,
            236117 => Self::NewHousingForsaleBuilders,
            236118 => Self::ResidentialRemodelers,
            236210 => Self::IndustrialBuildingConstruction,
            236220 => Self::CommercialAndInstitutionalBuildingConstruction,
            237110 => Self::WaterAndSewerLineAndRelatedStructuresConstruction,
            237120 => Self::OilAndGasPipelineAndRelatedStructuresConstruction,
            237130 => Self::PowerAndCommunicationLineAndRelatedStructuresConstruction,
            237210 => Self::LandSubdivision,
            237310 => Self::HighwayStreetAndBridgeConstruction,
            237990 => Self::OtherHeavyAndCivilEngineeringConstruction,
            238110 => Self::PouredConcreteFoundationAndStructureContractors,
            238120 => Self::StructuralSteelAndPrecastConcreteContractors,
            238130 => Self::FramingContractors,
            238140 => Self::MasonryContractors,
            238150 => Self::GlassAndGlazingContractors,
            238160 => Self::RoofingContractors,
            238170 => Self::SidingContractors,
            238190 => Self::OtherFoundationStructureAndBuildingExteriorContractors,
            238210 => Self::ElectricalContractorsAndOtherWiringInstallationContractors,
            238220 => Self::PlumbingHeatingAndAirconditioningContractors,
            238290 => Self::OtherBuildingEquipmentContractors,
            238310 => Self::DrywallAndInsulationContractors,
            238320 => Self::PaintingAndWallCoveringContractors,
            238330 => Self::FlooringContractors,
            238340 => Self::TileAndTerrazzoContractors,
            238350 => Self::FinishCarpentryContractors,
            238390 => Self::OtherBuildingFinishingContractors,
            238910 => Self::SitePreparationContractors,
            238990 => Self::AllOtherSpecialtyTradeContractors,
            311111 => Self::DogAndCatFoodManufacturing,
            311119 => Self::OtherAnimalFoodManufacturing,
            311211 => Self::FlourMilling,
            311212 => Self::RiceMilling,
            311213 => Self::MaltManufacturing,
            311221 => Self::WetCornMillingAndStarchManufacturing,
            311224 => Self::SoybeanAndOtherOilseedProcessing,
            311225 => Self::FatsAndOilsRefiningAndBlending,
            311230 => Self::BreakfastCerealManufacturing,
            311313 => Self::BeetSugarManufacturing,
            311314 => Self::CaneSugarManufacturing,
            311340 => Self::NonchocolateConfectioneryManufacturing,
            311351 => Self::ChocolateAndConfectioneryManufacturingFromCacaoBeans,
            311352 => Self::ConfectioneryManufacturingFromPurchasedChocolate,
            311411 => Self::FrozenFruitJuiceAndVegetableManufacturing,
            311412 => Self::FrozenSpecialtyFoodManufacturing,
            311421 => Self::FruitAndVegetableCanning,
            311422 => Self::SpecialtyCanning,
            311423 => Self::DriedAndDehydratedFoodManufacturing,
            311511 => Self::FluidMilkManufacturing,
            311512 => Self::CreameryButterManufacturing,
            311513 => Self::CheeseManufacturing,
            311514 => Self::DryCondensedAndEvaporatedDairyProductManufacturing,
            311520 => Self::IceCreamAndFrozenDessertManufacturing,
            311611 => Self::AnimalExceptPoultrySlaughtering,
            311612 => Self::MeatProcessedFromCarcasses,
            311613 => Self::RenderingAndMeatByproductProcessing,
            311615 => Self::PoultryProcessing,
            311710 => Self::SeafoodProductPreparationAndPackaging,
            311811 => Self::RetailBakeries,
            311812 => Self::CommercialBakeries,
            311813 => Self::FrozenCakesPiesAndOtherPastriesManufacturing,
            311821 => Self::CookieAndCrackerManufacturing,
            311824 => Self::DryPastaDoughAndFlourMixesManufacturingFromPurchasedFlour,
            311830 => Self::TortillaManufacturing,
            311911 => Self::RoastedNutsAndPeanutButterManufacturing,
            311919 => Self::OtherSnackFoodManufacturing,
            311920 => Self::CoffeeAndTeaManufacturing,
            311930 => Self::FlavoringSyrupAndConcentrateManufacturing,
            311941 => Self::MayonnaiseDressingAndOtherPreparedSauceManufacturing,
            311942 => Self::SpiceAndExtractManufacturing,
            311991 => Self::PerishablePreparedFoodManufacturing,
            311999 => Self::AllOtherMiscellaneousFoodManufacturing,
            312111 => Self::SoftDrinkManufacturing,
            312112 => Self::BottledWaterManufacturing,
            312113 => Self::IceManufacturing,
            312120 => Self::Breweries,
            312130 => Self::Wineries,
            312140 => Self::Distilleries,
            312230 => Self::TobaccoManufacturing,
            313110 => Self::FiberYarnAndThreadMills,
            313210 => Self::BroadwovenFabricMills,
            313220 => Self::NarrowFabricMillsAndSchiffliMachineEmbroidery,
            313230 => Self::NonwovenFabricMills,
            313240 => Self::KnitFabricMills,
            313310 => Self::TextileAndFabricFinishingMills,
            313320 => Self::FabricCoatingMills,
            314110 => Self::CarpetAndRugMills,
            314120 => Self::CurtainAndLinenMills,
            314910 => Self::TextileBagAndCanvasMills,
            314994 => Self::RopeCordageTwineTireCordAndTireFabricMills,
            314999 => Self::AllOtherMiscellaneousTextileProductMills,
            315120 => Self::ApparelKnittingMills,
            315210 => Self::CutAndSewApparelContractors,
            315250 => Self::CutAndSewApparelManufacturingExceptContractors,
            315990 => Self::ApparelAccessoriesAndOtherApparelManufacturing,
            316110 => Self::LeatherAndHideTanningAndFinishing,
            316210 => Self::FootwearManufacturing,
            316990 => Self::OtherLeatherAndAlliedProductManufacturing,
            321113 => Self::Sawmills,
            321114 => Self::WoodPreservation,
            321211 => Self::HardwoodVeneerAndPlywoodManufacturing,
            321212 => Self::SoftwoodVeneerAndPlywoodManufacturing,
            321215 => Self::EngineeredWoodMemberManufacturing,
            321219 => Self::ReconstitutedWoodProductManufacturing,
            321911 => Self::WoodWindowAndDoorManufacturing,
            321912 => Self::CutStockResawingLumberAndPlaning,
            321918 => Self::OtherMillworkIncludingFlooring,
            321920 => Self::WoodContainerAndPalletManufacturing,
            321991 => Self::ManufacturedHomeMobileHomeManufacturing,
            321992 => Self::PrefabricatedWoodBuildingManufacturing,
            321999 => Self::AllOtherMiscellaneousWoodProductManufacturing,
            322110 => Self::PulpMills,
            322120 => Self::PaperMills,
            322130 => Self::PaperboardMills,
            322211 => Self::CorrugatedAndSolidFiberBoxManufacturing,
            322212 => Self::FoldingPaperboardBoxManufacturing,
            322219 => Self::OtherPaperboardContainerManufacturing,
            322220 => Self::PaperBagAndCoatedAndTreatedPaperManufacturing,
            322230 => Self::StationeryProductManufacturing,
            322291 => Self::SanitaryPaperProductManufacturing,
            322299 => Self::AllOtherConvertedPaperProductManufacturing,
            323111 => Self::CommercialPrintingExceptScreenAndBooks,
            323113 => Self::CommercialScreenPrinting,
            323117 => Self::BooksPrinting,
            323120 => Self::SupportActivitiesForPrinting,
            324110 => Self::PetroleumRefineries,
            324121 => Self::AsphaltPavingMixtureAndBlockManufacturing,
            324122 => Self::AsphaltShingleAndCoatingMaterialsManufacturing,
            324191 => Self::PetroleumLubricatingOilAndGreaseManufacturing,
            324199 => Self::AllOtherPetroleumAndCoalProductsManufacturing,
            325110 => Self::PetrochemicalManufacturing,
            325120 => Self::IndustrialGasManufacturing,
            325130 => Self::SyntheticDyeAndPigmentManufacturing,
            325180 => Self::OtherBasicInorganicChemicalManufacturing,
            325193 => Self::EthylAlcoholManufacturing,
            325194 => Self::CyclicCrudeIntermediateAndGumAndWoodChemicalManufacturing,
            325199 => Self::AllOtherBasicOrganicChemicalManufacturing,
            325211 => Self::PlasticsMaterialAndResinManufacturing,
            325212 => Self::SyntheticRubberManufacturing,
            325220 => Self::ArtificialAndSyntheticFibersAndFilamentsManufacturing,
            325311 => Self::NitrogenousFertilizerManufacturing,
            325312 => Self::PhosphaticFertilizerManufacturing,
            325314 => Self::FertilizerMixingOnlyManufacturing,
            325315 => Self::CompostManufacturing,
            325320 => Self::PesticideAndOtherAgriculturalChemicalManufacturing,
            325411 => Self::MedicinalAndBotanicalManufacturing,
            325412 => Self::PharmaceuticalPreparationManufacturing,
            325413 => Self::InvitroDiagnosticSubstanceManufacturing,
            325414 => Self::BiologicalProductExceptDiagnosticManufacturing,
            325510 => Self::PaintAndCoatingManufacturing,
            325520 => Self::AdhesiveManufacturing,
            325611 => Self::SoapAndOtherDetergentManufacturing,
            325612 => Self::PolishAndOtherSanitationGoodManufacturing,
            325613 => Self::SurfaceActiveAgentManufacturing,
            325620 => Self::ToiletPreparationManufacturing,
            325910 => Self::PrintingInkManufacturing,
            325920 => Self::ExplosivesManufacturing,
            325991 => Self::CustomCompoundingOfPurchasedResins,
            325992 => Self::PhotographicFilmPaperPlateChemicalAndCopyTonerManufacturing,
            325998 => Self::AllOtherMiscellaneousChemicalProductAndPreparationManufacturing,
            326111 => Self::PlasticsBagAndPouchManufacturing,
            326112 => Self::PlasticsPackagingFilmAndSheetIncludingLaminatedManufacturing,
            326113 => Self::UnlaminatedPlasticsFilmAndSheetExceptPackagingManufacturing,
            326121 => Self::UnlaminatedPlasticsProfileShapeManufacturing,
            326122 => Self::PlasticsPipeAndPipeFittingManufacturing,
            326130 => Self::LaminatedPlasticsPlateSheetExceptPackagingAndShapeManufacturing,
            326140 => Self::PolystyreneFoamProductManufacturing,
            326150 => Self::UrethaneAndOtherFoamProductExceptPolystyreneManufacturing,
            326160 => Self::PlasticsBottleManufacturing,
            326191 => Self::PlasticsPlumbingFixtureManufacturing,
            326199 => Self::AllOtherPlasticsProductManufacturing,
            326211 => Self::TireManufacturingExceptRetreading,
            326212 => Self::TireRetreading,
            326220 => Self::RubberAndPlasticsHosesAndBeltingManufacturing,
            326291 => Self::RubberProductManufacturingForMechanicalUse,
            326299 => Self::AllOtherRubberProductManufacturing,
            327110 => Self::PotteryCeramicsAndPlumbingFixtureManufacturing,
            327120 => Self::ClayBuildingMaterialAndRefractoriesManufacturing,
            327211 => Self::FlatGlassManufacturing,
            327212 => Self::OtherPressedAndBlownGlassAndGlasswareManufacturing,
            327213 => Self::GlassContainerManufacturing,
            327215 => Self::GlassProductManufacturingMadeOfPurchasedGlass,
            327310 => Self::CementManufacturing,
            327320 => Self::ReadymixConcreteManufacturing,
            327331 => Self::ConcreteBlockAndBrickManufacturing,
            327332 => Self::ConcretePipeManufacturing,
            327390 => Self::OtherConcreteProductManufacturing,
            327410 => Self::LimeManufacturing,
            327420 => Self::GypsumProductManufacturing,
            327910 => Self::AbrasiveProductManufacturing,
            327991 => Self::CutStoneAndStoneProductManufacturing,
            327992 => Self::GroundOrTreatedMineralAndEarthManufacturing,
            327993 => Self::MineralWoolManufacturing,
            327999 => Self::AllOtherMiscellaneousNonmetallicMineralProductManufacturing,
            331110 => Self::IronAndSteelMillsAndFerroalloyManufacturing,
            331210 => Self::IronAndSteelPipeAndTubeManufacturingFromPurchasedSteel,
            331221 => Self::RolledSteelShapeManufacturing,
            331222 => Self::SteelWireDrawing,
            331313 => Self::AluminaRefiningAndPrimaryAluminumProduction,
            331314 => Self::SecondarySmeltingAndAlloyingOfAluminum,
            331315 => Self::AluminumSheetPlateAndFoilManufacturing,
            331318 => Self::OtherAluminumRollingDrawingAndExtruding,
            331410 => Self::NonferrousMetalExceptAluminumSmeltingAndRefining,
            331420 => Self::CopperRollingDrawingExtrudingAndAlloying,
            331491 => Self::NonferrousMetalExceptCopperAndAluminumRollingDrawingAndExtruding,
            331492 => Self::SecondarySmeltingRefiningAndAlloyingOfNonferrousMetalExceptCopperAndAluminum,
            331511 => Self::IronFoundries,
            331512 => Self::SteelInvestmentFoundries,
            331513 => Self::SteelFoundriesExceptInvestment,
            331523 => Self::NonferrousMetalDiecastingFoundries,
            331524 => Self::AluminumFoundriesExceptDiecasting,
            331529 => Self::OtherNonferrousMetalFoundriesExceptDiecasting,
            332111 => Self::IronAndSteelForging,
            332112 => Self::NonferrousForging,
            332114 => Self::CustomRollForming,
            332117 => Self::PowderMetallurgyPartManufacturing,
            332119 => Self::MetalCrownClosureAndOtherMetalStampingExceptAutomotive,
            332215 => Self::MetalKitchenCookwareUtensilCutleryAndFlatwareExceptPreciousManufacturing,
            332216 => Self::SawBladeAndHandtoolManufacturing,
            332311 => Self::PrefabricatedMetalBuildingAndComponentManufacturing,
            332312 => Self::FabricatedStructuralMetalManufacturing,
            332313 => Self::PlateWorkManufacturing,
            332321 => Self::MetalWindowAndDoorManufacturing,
            332322 => Self::SheetMetalWorkManufacturing,
            332323 => Self::OrnamentalAndArchitecturalMetalWorkManufacturing,
            332410 => Self::PowerBoilerAndHeatExchangerManufacturing,
            332420 => Self::MetalTankHeavyGaugeManufacturing,
            332431 => Self::MetalCanManufacturing,
            332439 => Self::OtherMetalContainerManufacturing,
            332510 => Self::HardwareManufacturing,
            332613 => Self::SpringManufacturing,
            332618 => Self::OtherFabricatedWireProductManufacturing,
            332710 => Self::MachineShops,
            332721 => Self::PrecisionTurnedProductManufacturing,
            332722 => Self::BoltNutScrewRivetAndWasherManufacturing,
            332811 => Self::MetalHeatTreating,
            332812 => Self::MetalCoatingEngravingExceptJewelryAndSilverwareAndAlliedServicesToManufacturers,
            332813 => Self::ElectroplatingPlatingPolishingAnodizingAndColoring,
            332911 => Self::IndustrialValveManufacturing,
            332912 => Self::FluidPowerValveAndHoseFittingManufacturing,
            332913 => Self::PlumbingFixtureFittingAndTrimManufacturing,
            332919 => Self::OtherMetalValveAndPipeFittingManufacturing,
            332991 => Self::BallAndRollerBearingManufacturing,
            332992 => Self::SmallArmsAmmunitionManufacturing,
            332993 => Self::AmmunitionExceptSmallArmsManufacturing,
            332994 => Self::SmallArmsOrdnanceAndOrdnanceAccessoriesManufacturing,
            332996 => Self::FabricatedPipeAndPipeFittingManufacturing,
            332999 => Self::AllOtherMiscellaneousFabricatedMetalProductManufacturing,
            333111 => Self::FarmMachineryAndEquipmentManufacturing,
            333112 => Self::LawnAndGardenTractorAndHomeLawnAndGardenEquipmentManufacturing,
            333120 => Self::ConstructionMachineryManufacturing,
            333131 => Self::MiningMachineryAndEquipmentManufacturing,
            333132 => Self::OilAndGasFieldMachineryAndEquipmentManufacturing,
            333241 => Self::FoodProductMachineryManufacturing,
            333242 => Self::SemiconductorMachineryManufacturing,
            333243 => Self::SawmillWoodworkingAndPaperMachineryManufacturing,
            333248 => Self::AllOtherIndustrialMachineryManufacturing,
            333310 => Self::CommercialAndServiceIndustryMachineryManufacturing,
            333413 => Self::IndustrialAndCommercialFanAndBlowerAndAirPurificationEquipmentManufacturing,
            333414 => Self::HeatingEquipmentExceptWarmAirFurnacesManufacturing,
            333415 => Self::AirconditioningAndWarmAirHeatingEquipmentAndCommercialAndIndustrialRefrigerationEquipmentManufacturing,
            333511 => Self::IndustrialMoldManufacturing,
            333514 => Self::SpecialDieAndToolDieSetJigAndFixtureManufacturing,
            333515 => Self::CuttingToolAndMachineToolAccessoryManufacturing,
            333517 => Self::MachineToolManufacturing,
            333519 => Self::RollingMillAndOtherMetalworkingMachineryManufacturing,
            333611 => Self::TurbineAndTurbineGeneratorSetUnitsManufacturing,
            333612 => Self::SpeedChangerIndustrialHighspeedDriveAndGearManufacturing,
            333613 => Self::MechanicalPowerTransmissionEquipmentManufacturing,
            333618 => Self::OtherEngineEquipmentManufacturing,
            333912 => Self::AirAndGasCompressorManufacturing,
            333914 => Self::MeasuringDispensingAndOtherPumpingEquipmentManufacturing,
            333921 => Self::ElevatorAndMovingStairwayManufacturing,
            333922 => Self::ConveyorAndConveyingEquipmentManufacturing,
            333923 => Self::OverheadTravelingCraneHoistAndMonorailSystemManufacturing,
            333924 => Self::IndustrialTruckTractorTrailerAndStackerMachineryManufacturing,
            333991 => Self::PowerdrivenHandtoolManufacturing,
            333992 => Self::WeldingAndSolderingEquipmentManufacturing,
            333993 => Self::PackagingMachineryManufacturing,
            333994 => Self::IndustrialProcessFurnaceAndOvenManufacturing,
            333995 => Self::FluidPowerCylinderAndActuatorManufacturing,
            333996 => Self::FluidPowerPumpAndMotorManufacturing,
            333998 => Self::AllOtherMiscellaneousGeneralPurposeMachineryManufacturing,
            334111 => Self::ElectronicComputerManufacturing,
            334112 => Self::ComputerStorageDeviceManufacturing,
            334118 => Self::ComputerTerminalAndOtherComputerPeripheralEquipmentManufacturing,
            334210 => Self::TelephoneApparatusManufacturing,
            334220 => Self::RadioAndTelevisionBroadcastingAndWirelessCommunicationsEquipmentManufacturing,
            334290 => Self::OtherCommunicationsEquipmentManufacturing,
            334310 => Self::AudioAndVideoEquipmentManufacturing,
            334412 => Self::BarePrintedCircuitBoardManufacturing,
            334413 => Self::SemiconductorAndRelatedDeviceManufacturing,
            334416 => Self::CapacitorResistorCoilTransformerAndOtherInductorManufacturing,
            334417 => Self::ElectronicConnectorManufacturing,
            334418 => Self::PrintedCircuitAssemblyElectronicAssemblyManufacturing,
            334419 => Self::OtherElectronicComponentManufacturing,
            334510 => Self::ElectromedicalAndElectrotherapeuticApparatusManufacturing,
            334511 => Self::SearchDetectionNavigationGuidanceAeronauticalAndNauticalSystemAndInstrumentManufacturing,
            334512 => Self::AutomaticEnvironmentalControlManufacturingForResidentialCommercialAndApplianceUse,
            334513 => Self::InstrumentsAndRelatedProductsManufacturingForMeasuringDisplayingAndControllingIndustrialProcessVariables,
            334514 => Self::TotalizingFluidMeterAndCountingDeviceManufacturing,
            334515 => Self::InstrumentManufacturingForMeasuringAndTestingElectricityAndElectricalSignals,
            334516 => Self::AnalyticalLaboratoryInstrumentManufacturing,
            334517 => Self::IrradiationApparatusManufacturing,
            334519 => Self::OtherMeasuringAndControllingDeviceManufacturing,
            334610 => Self::ManufacturingAndReproducingMagneticAndOpticalMedia,
            335131 => Self::ResidentialElectricLightingFixtureManufacturing,
            335132 => Self::CommercialIndustrialAndInstitutionalElectricLightingFixtureManufacturing,
            335139 => Self::ElectricLampBulbAndOtherLightingEquipmentManufacturing,
            335210 => Self::SmallElectricalApplianceManufacturing,
            335220 => Self::MajorHouseholdApplianceManufacturing,
            335311 => Self::PowerDistributionAndSpecialtyTransformerManufacturing,
            335312 => Self::MotorAndGeneratorManufacturing,
            335313 => Self::SwitchgearAndSwitchboardApparatusManufacturing,
            335314 => Self::RelayAndIndustrialControlManufacturing,
            335910 => Self::BatteryManufacturing,
            335921 => Self::FiberOpticCableManufacturing,
            335929 => Self::OtherCommunicationAndEnergyWireManufacturing,
            335931 => Self::CurrentcarryingWiringDeviceManufacturing,
            335932 => Self::NoncurrentcarryingWiringDeviceManufacturing,
            335991 => Self::CarbonAndGraphiteProductManufacturing,
            335999 => Self::AllOtherMiscellaneousElectricalEquipmentAndComponentManufacturing,
            336110 => Self::AutomobileAndLightDutyMotorVehicleManufacturing,
            336120 => Self::HeavyDutyTruckManufacturing,
            336211 => Self::MotorVehicleBodyManufacturing,
            336212 => Self::TruckTrailerManufacturing,
            336213 => Self::MotorHomeManufacturing,
            336214 => Self::TravelTrailerAndCamperManufacturing,
            336310 => Self::MotorVehicleGasolineEngineAndEnginePartsManufacturing,
            336320 => Self::MotorVehicleElectricalAndElectronicEquipmentManufacturing,
            336330 => Self::MotorVehicleSteeringAndSuspensionComponentsExceptSpringManufacturing,
            336340 => Self::MotorVehicleBrakeSystemManufacturing,
            336350 => Self::MotorVehicleTransmissionAndPowerTrainPartsManufacturing,
            336360 => Self::MotorVehicleSeatingAndInteriorTrimManufacturing,
            336370 => Self::MotorVehicleMetalStamping,
            336390 => Self::OtherMotorVehiclePartsManufacturing,
            336411 => Self::AircraftManufacturing,
            336412 => Self::AircraftEngineAndEnginePartsManufacturing,
            336413 => Self::OtherAircraftPartsAndAuxiliaryEquipmentManufacturing,
            336414 => Self::GuidedMissileAndSpaceVehicleManufacturing,
            336415 => Self::GuidedMissileAndSpaceVehiclePropulsionUnitAndPropulsionUnitPartsManufacturing,
            336419 => Self::OtherGuidedMissileAndSpaceVehiclePartsAndAuxiliaryEquipmentManufacturing,
            336510 => Self::RailroadRollingStockManufacturing,
            336611 => Self::ShipBuildingAndRepairing,
            336612 => Self::BoatBuilding,
            336991 => Self::MotorcycleBicycleAndPartsManufacturing,
            336992 => Self::MilitaryArmoredVehicleTankAndTankComponentManufacturing,
            336999 => Self::AllOtherTransportationEquipmentManufacturing,
            337110 => Self::WoodKitchenCabinetAndCountertopManufacturing,
            337121 => Self::UpholsteredHouseholdFurnitureManufacturing,
            337122 => Self::NonupholsteredWoodHouseholdFurnitureManufacturing,
            337126 => Self::HouseholdFurnitureExceptWoodAndUpholsteredManufacturing,
            337127 => Self::InstitutionalFurnitureManufacturing,
            337211 => Self::WoodOfficeFurnitureManufacturing,
            337212 => Self::CustomArchitecturalWoodworkAndMillworkManufacturing,
            337214 => Self::OfficeFurnitureExceptWoodManufacturing,
            337215 => Self::ShowcasePartitionShelvingAndLockerManufacturing,
            337910 => Self::MattressManufacturing,
            337920 => Self::BlindAndShadeManufacturing,
            339112 => Self::SurgicalAndMedicalInstrumentManufacturing,
            339113 => Self::SurgicalApplianceAndSuppliesManufacturing,
            339114 => Self::DentalEquipmentAndSuppliesManufacturing,
            339115 => Self::OphthalmicGoodsManufacturing,
            339116 => Self::DentalLaboratories,
            339910 => Self::JewelryAndSilverwareManufacturing,
            339920 => Self::SportingAndAthleticGoodsManufacturing,
            339930 => Self::DollToyAndGameManufacturing,
            339940 => Self::OfficeSuppliesExceptPaperManufacturing,
            339950 => Self::SignManufacturing,
            339991 => Self::GasketPackingAndSealingDeviceManufacturing,
            339992 => Self::MusicalInstrumentManufacturing,
            339993 => Self::FastenerButtonNeedleAndPinManufacturing,
            339994 => Self::BroomBrushAndMopManufacturing,
            339995 => Self::BurialCasketManufacturing,
            339999 => Self::AllOtherMiscellaneousManufacturing,
            423110 => Self::AutomobileAndOtherMotorVehicleMerchantWholesalers,
            423120 => Self::MotorVehicleSuppliesAndNewPartsMerchantWholesalers,
            423130 => Self::TireAndTubeMerchantWholesalers,
            423140 => Self::MotorVehiclePartsUsedMerchantWholesalers,
            423210 => Self::FurnitureMerchantWholesalers,
            423220 => Self::HomeFurnishingMerchantWholesalers,
            423310 => Self::LumberPlywoodMillworkAndWoodPanelMerchantWholesalers,
            423320 => Self::BrickStoneAndRelatedConstructionMaterialMerchantWholesalers,
            423330 => Self::RoofingSidingAndInsulationMaterialMerchantWholesalers,
            423390 => Self::OtherConstructionMaterialMerchantWholesalers,
            423410 => Self::PhotographicEquipmentAndSuppliesMerchantWholesalers,
            423420 => Self::OfficeEquipmentMerchantWholesalers,
            423430 => Self::ComputerAndComputerPeripheralEquipmentAndSoftwareMerchantWholesalers,
            423440 => Self::OtherCommercialEquipmentMerchantWholesalers,
            423450 => Self::MedicalDentalAndHospitalEquipmentAndSuppliesMerchantWholesalers,
            423460 => Self::OphthalmicGoodsMerchantWholesalers,
            423490 => Self::OtherProfessionalEquipmentAndSuppliesMerchantWholesalers,
            423510 => Self::MetalServiceCentersAndOtherMetalMerchantWholesalers,
            423520 => Self::CoalAndOtherMineralAndOreMerchantWholesalers,
            423610 => Self::ElectricalApparatusAndEquipmentWiringSuppliesAndRelatedEquipmentMerchantWholesalers,
            423620 => Self::HouseholdAppliancesElectricHousewaresAndConsumerElectronicsMerchantWholesalers,
            423690 => Self::OtherElectronicPartsAndEquipmentMerchantWholesalers,
            423710 => Self::HardwareMerchantWholesalers,
            423720 => Self::PlumbingAndHeatingEquipmentAndSuppliesHydronicsMerchantWholesalers,
            423730 => Self::WarmAirHeatingAndAirconditioningEquipmentAndSuppliesMerchantWholesalers,
            423740 => Self::RefrigerationEquipmentAndSuppliesMerchantWholesalers,
            423810 => Self::ConstructionAndMiningExceptOilWellMachineryAndEquipmentMerchantWholesalers,
            423820 => Self::FarmAndGardenMachineryAndEquipmentMerchantWholesalers,
            423830 => Self::IndustrialMachineryAndEquipmentMerchantWholesalers,
            423840 => Self::IndustrialSuppliesMerchantWholesalers,
            423850 => Self::ServiceEstablishmentEquipmentAndSuppliesMerchantWholesalers,
            423860 => Self::TransportationEquipmentAndSuppliesExceptMotorVehicleMerchantWholesalers,
            423910 => Self::SportingAndRecreationalGoodsAndSuppliesMerchantWholesalers,
            423920 => Self::ToyAndHobbyGoodsAndSuppliesMerchantWholesalers,
            423930 => Self::RecyclableMaterialMerchantWholesalers,
            423940 => Self::JewelryWatchPreciousStoneAndPreciousMetalMerchantWholesalers,
            423990 => Self::OtherMiscellaneousDurableGoodsMerchantWholesalers,
            424110 => Self::PrintingAndWritingPaperMerchantWholesalers,
            424120 => Self::StationeryAndOfficeSuppliesMerchantWholesalers,
            424130 => Self::IndustrialAndPersonalServicePaperMerchantWholesalers,
            424210 => Self::DrugsAndDruggistsSundriesMerchantWholesalers,
            424310 => Self::PieceGoodsNotionsAndOtherDryGoodsMerchantWholesalers,
            424340 => Self::FootwearMerchantWholesalers,
            424350 => Self::ClothingAndClothingAccessoriesMerchantWholesalers,
            424410 => Self::GeneralLineGroceryMerchantWholesalers,
            424420 => Self::PackagedFrozenFoodMerchantWholesalers,
            424430 => Self::DairyProductExceptDriedOrCannedMerchantWholesalers,
            424440 => Self::PoultryAndPoultryProductMerchantWholesalers,
            424450 => Self::ConfectioneryMerchantWholesalers,
            424460 => Self::FishAndSeafoodMerchantWholesalers,
            424470 => Self::MeatAndMeatProductMerchantWholesalers,
            424480 => Self::FreshFruitAndVegetableMerchantWholesalers,
            424490 => Self::OtherGroceryAndRelatedProductsMerchantWholesalers,
            424510 => Self::GrainAndFieldBeanMerchantWholesalers,
            424520 => Self::LivestockMerchantWholesalers,
            424590 => Self::OtherFarmProductRawMaterialMerchantWholesalers,
            424610 => Self::PlasticsMaterialsAndBasicFormsAndShapesMerchantWholesalers,
            424690 => Self::OtherChemicalAndAlliedProductsMerchantWholesalers,
            424710 => Self::PetroleumBulkStationsAndTerminals,
            424720 => Self::PetroleumAndPetroleumProductsMerchantWholesalersExceptBulkStationsAndTerminals,
            424810 => Self::BeerAndAleMerchantWholesalers,
            424820 => Self::WineAndDistilledAlcoholicBeverageMerchantWholesalers,
            424910 => Self::FarmSuppliesMerchantWholesalers,
            424920 => Self::BookPeriodicalAndNewspaperMerchantWholesalers,
            424930 => Self::FlowerNurseryStockAndFloristsSuppliesMerchantWholesalers,
            424940 => Self::TobaccoProductAndElectronicCigaretteMerchantWholesalers,
            424950 => Self::PaintVarnishAndSuppliesMerchantWholesalers,
            424990 => Self::OtherMiscellaneousNondurableGoodsMerchantWholesalers,
            425120 => Self::WholesaleTradeAgentsAndBrokers,
            441110 => Self::NewCarDealers,
            441120 => Self::UsedCarDealers,
            441210 => Self::RecreationalVehicleDealers,
            441222 => Self::BoatDealers,
            441227 => Self::MotorcycleAtvAndAllOtherMotorVehicleDealers,
            441330 => Self::AutomotivePartsAndAccessoriesRetailers,
            441340 => Self::TireDealers,
            444110 => Self::HomeCenters,
            444120 => Self::PaintAndWallpaperRetailers,
            444140 => Self::HardwareRetailers,
            444180 => Self::OtherBuildingMaterialDealers,
            444230 => Self::OutdoorPowerEquipmentRetailers,
            444240 => Self::NurseryGardenCenterAndFarmSupplyRetailers,
            445110 => Self::SupermarketsAndOtherGroceryRetailersExceptConvenienceRetailers,
            445131 => Self::ConvenienceRetailers,
            445132 => Self::VendingMachineOperators,
            445230 => Self::FruitAndVegetableRetailers,
            445240 => Self::MeatRetailers,
            445250 => Self::FishAndSeafoodRetailers,
            445291 => Self::BakedGoodsRetailers,
            445292 => Self::ConfectioneryAndNutRetailers,
            445298 => Self::AllOtherSpecialtyFoodRetailers,
            445320 => Self::BeerWineAndLiquorRetailers,
            449110 => Self::FurnitureRetailers,
            449121 => Self::FloorCoveringRetailers,
            449122 => Self::WindowTreatmentRetailers,
            449129 => Self::AllOtherHomeFurnishingsRetailers,
            449210 => Self::ElectronicsAndApplianceRetailers,
            455110 => Self::DepartmentStores,
            455211 => Self::WarehouseClubsAndSupercenters,
            455219 => Self::AllOtherGeneralMerchandiseRetailers,
            456110 => Self::PharmaciesAndDrugRetailers,
            456120 => Self::CosmeticsBeautySuppliesAndPerfumeRetailers,
            456130 => Self::OpticalGoodsRetailers,
            456191 => Self::FoodHealthSupplementRetailers,
            456199 => Self::AllOtherHealthAndPersonalCareRetailers,
            457110 => Self::GasolineStationsWithConvenienceStores,
            457120 => Self::OtherGasolineStations,
            457210 => Self::FuelDealers,
            458110 => Self::ClothingRetailers,
            458120 => Self::ClothingAccessoriesRetailers,
            458210 => Self::ShoeRetailers,
            458310 => Self::JewelryRetailers,
            458320 => Self::LuggageAndLeatherGoodsRetailers,
            459110 => Self::SportingGoodsRetailers,
            459120 => Self::HobbyToyAndGameRetailers,
            459130 => Self::SewingNeedleworkAndPieceGoodsRetailers,
            459140 => Self::MusicalInstrumentAndSuppliesRetailers,
            459210 => Self::BookRetailersAndNewsDealers,
            459310 => Self::Florists,
            459410 => Self::OfficeSuppliesAndStationeryRetailers,
            459420 => Self::GiftNoveltyAndSouvenirRetailers,
            459510 => Self::UsedMerchandiseRetailers,
            459910 => Self::PetAndPetSuppliesRetailers,
            459920 => Self::ArtDealers,
            459930 => Self::ManufacturedMobileHomeDealers,
            459991 => Self::TobaccoElectronicCigaretteAndOtherSmokingSuppliesRetailers,
            459999 => Self::AllOtherMiscellaneousRetailers,
            481111 => Self::ScheduledPassengerAirTransportation,
            481112 => Self::ScheduledFreightAirTransportation,
            481211 => Self::NonscheduledCharteredPassengerAirTransportation,
            481212 => Self::NonscheduledCharteredFreightAirTransportation,
            481219 => Self::OtherNonscheduledAirTransportation,
            482111 => Self::LineHaulRailroads,
            482112 => Self::ShortLineRailroads,
            483111 => Self::DeepSeaFreightTransportation,
            483112 => Self::DeepSeaPassengerTransportation,
            483113 => Self::CoastalAndGreatLakesFreightTransportation,
            483114 => Self::CoastalAndGreatLakesPassengerTransportation,
            483211 => Self::InlandWaterFreightTransportation,
            483212 => Self::InlandWaterPassengerTransportation,
            484110 => Self::GeneralFreightTruckingLocal,
            484121 => Self::GeneralFreightTruckingLongdistanceTruckload,
            484122 => Self::GeneralFreightTruckingLongdistanceLessThanTruckload,
            484210 => Self::UsedHouseholdAndOfficeGoodsMoving,
            484220 => Self::SpecializedFreightExceptUsedGoodsTruckingLocal,
            484230 => Self::SpecializedFreightExceptUsedGoodsTruckingLongdistance,
            485111 => Self::MixedModeTransitSystems,
            485112 => Self::CommuterRailSystems,
            485113 => Self::BusAndOtherMotorVehicleTransitSystems,
            485114 => Self::StreetRailroads,
            485115 => Self::LightRailTransitSystems,
            485119 => Self::OtherUrbanTransitSystems,
            485210 => Self::InterurbanAndRuralBusTransportation,
            485310 => Self::TaxiService,
            485320 => Self::LimousineService,
            485410 => Self::SchoolAndEmployeeBusTransportation,
            485510 => Self::CharterBusIndustry,
            485991 => Self::SpecialNeedsTransportation,
            485999 => Self::AllOtherTransitAndGroundPassengerTransportation,
            486110 => Self::PipelineTransportationOfCrudeOil,
            486210 => Self::PipelineTransportationOfNaturalGas,
            486910 => Self::PipelineTransportationOfRefinedPetroleumProducts,
            486990 => Self::AllOtherPipelineTransportation,
            487110 => Self::ScenicAndSightseeingTransportationLand,
            487210 => Self::ScenicAndSightseeingTransportationWater,
            487990 => Self::ScenicAndSightseeingTransportationOther,
            488111 => Self::AirTrafficControl,
            488119 => Self::OtherAirportOperations,
            488190 => Self::OtherSupportActivitiesForAirTransportation,
            488210 => Self::SupportActivitiesForRailTransportation,
            488310 => Self::PortAndHarborOperations,
            488320 => Self::MarineCargoHandling,
            488330 => Self::NavigationalServicesToShipping,
            488390 => Self::OtherSupportActivitiesForWaterTransportation,
            488410 => Self::MotorVehicleTowing,
            488490 => Self::OtherSupportActivitiesForRoadTransportation,
            488510 => Self::FreightTransportationArrangement,
            488991 => Self::PackingAndCrating,
            488999 => Self::AllOtherSupportActivitiesForTransportation,
            491110 => Self::PostalService,
            492110 => Self::CouriersAndExpressDeliveryServices,
            492210 => Self::LocalMessengersAndLocalDelivery,
            493110 => Self::GeneralWarehousingAndStorage,
            493120 => Self::RefrigeratedWarehousingAndStorage,
            493130 => Self::FarmProductWarehousingAndStorage,
            493190 => Self::OtherWarehousingAndStorage,
            511110 => Self::NewspaperPublishers,
            511120 => Self::PeriodicalPublishers,
            511130 => Self::BookPublishers,
            511140 => Self::DirectoryAndMailingListPublishers,
            511191 => Self::GreetingCardPublishers,
            511199 => Self::AllOtherPublishers,
            511210 => Self::SoftwarePublishers,
            512110 => Self::MotionPictureAndVideoProduction,
            512120 => Self::MotionPictureAndVideoDistribution,
            512131 => Self::MotionPictureTheatersExceptDriveins,
            512132 => Self::DriveInMotionPictureTheaters,
            512191 => Self::TeleproductionAndOtherPostproductionServices,
            512199 => Self::OtherMotionPictureAndVideoIndustries,
            512230 => Self::MusicPublishers,
            512240 => Self::SoundRecordingStudios,
            512250 => Self::RecordProductionAndDistribution,
            512290 => Self::OtherSoundRecordingIndustries,
            515111 => Self::RadioNetworks,
            515112 => Self::RadioStations,
            515120 => Self::TelevisionBroadcasting,
            515210 => Self::CableAndOtherSubscriptionProgramming,
            517311 => Self::WiredTelecommunicationsCarriers,
            517312 => Self::WirelessTelecommunicationsCarriersExceptSatellite,
            517410 => Self::SatelliteTelecommunications,
            517911 => Self::TelecommunicationsResellers,
            517919 => Self::AllOtherTelecommunications,
            518210 => Self::ComputingInfrastructureProvidersDataProcessingWebHostingAndRelatedServices,
            519110 => Self::NewsSyndicates,
            519120 => Self::LibrariesAndArchives,
            519130 => Self::InternetPublishingAndBroadcastingAndWebSearchPortals,
            519190 => Self::AllOtherInformationServices,
            521110 => Self::MonetaryAuthoritiesCentralBank,
            522110 => Self::CommercialBanking,
            522120 => Self::SavingsInstitutions,
            522130 => Self::CreditUnions,
            522190 => Self::OtherDepositoryCreditIntermediation,
            522210 => Self::CreditCardIssuing,
            522220 => Self::SalesFinancing,
            522291 => Self::ConsumerLending,
            522292 => Self::RealEstateCredit,
            522298 => Self::AllOtherNondepositoryCreditIntermediation,
            522310 => Self::MortgageAndNonmortgageLoanBrokers,
            522320 => Self::FinancialTransactionsProcessingReserveAndClearinghouseActivities,
            522390 => Self::OtherActivitiesRelatedToCreditIntermediation,
            523110 => Self::InvestmentBankingAndSecuritiesDealing,
            523120 => Self::SecuritiesBrokerage,
            523130 => Self::CommodityContractsDealing,
            523140 => Self::CommodityContractsBrokerage,
            523210 => Self::SecuritiesAndCommodityExchanges,
            523910 => Self::MiscellaneousIntermediation,
            523920 => Self::PortfolioManagement,
            523930 => Self::InvestmentAdvice,
            523991 => Self::TrustFiduciaryAndCustodyActivities,
            523999 => Self::MiscellaneousFinancialInvestmentActivities,
            524113 => Self::DirectLifeInsuranceCarriers,
            524114 => Self::DirectHealthAndMedicalInsuranceCarriers,
            524126 => Self::DirectPropertyAndCasualtyInsuranceCarriers,
            524127 => Self::DirectTitleInsuranceCarriers,
            524128 => Self::OtherDirectInsuranceExceptLifeHealthAndMedicalCarriers,
            524130 => Self::ReinsuranceCarriers,
            524210 => Self::InsuranceAgenciesAndBrokerages,
            524291 => Self::ClaimsAdjusting,
            524292 => Self::ThirdPartyAdministrationOfInsuranceAndPensionFunds,
            524298 => Self::AllOtherInsuranceRelatedActivities,
            525110 => Self::PensionFunds,
            525120 => Self::HealthAndWelfareFunds,
            525190 => Self::OtherInsuranceFunds,
            525910 => Self::OpenEndInvestmentFunds,
            525920 => Self::TrustsEstatesAndAgencyAccounts,
            525930 => Self::RealEstateInvestmentTrusts,
            525990 => Self::OtherFinancialVehicles,
            531110 => Self::LessorsOfResidentialBuildingsAndDwellings,
            531120 => Self::LessorsOfNonresidentialBuildingsExceptMiniwarehouses,
            531130 => Self::LessorsOfMiniwarehousesAndSelfStorageUnits,
            531190 => Self::LessorsOfOtherRealEstateProperty,
            531210 => Self::OfficesOfRealEstateAgentsAndBrokers,
            531311 => Self::ResidentialPropertyManagers,
            531312 => Self::NonresidentialPropertyManagers,
            531320 => Self::OfficesOfRealEstateAppraisers,
            531390 => Self::OtherActivitiesRelatedToRealEstate,
            532111 => Self::PassengerCarRental,
            532112 => Self::PassengerCarLeasing,
            532120 => Self::TruckUtilityTrailerAndRvRecreationalVehicleRentalAndLeasing,
            532210 => Self::ConsumerElectronicsAndAppliancesRental,
            532220 => Self::FormalWearAndCostumeRental,
            532230 => Self::VideoTapeAndDiscRental,
            532281 => Self::RecreationalGoodsRentalParent,
            532282 => Self::MedicalEquipmentAndSuppliesRental,
            532283 => Self::HomeHealthEquipmentAndSuppliesRental,
            532284 => Self::RecreationalGoodsRental,
            532289 => Self::AllOtherConsumerGoodsRental,
            532310 => Self::GeneralRentalCenters,
            532411 => Self::CommercialAirRailAndWaterTransportationEquipmentRentalAndLeasing,
            532412 => Self::ConstructionMiningAndForestryMachineryAndEquipmentRentalAndLeasing,
            532420 => Self::OfficeMachineryAndEquipmentRentalAndLeasing,
            532490 => Self::OtherCommercialAndIndustrialMachineryAndEquipmentRentalAndLeasing,
            533110 => Self::LessorsOfNonfinancialIntangibleAssetsExceptCopyrightedWorks,
            541110 => Self::OfficesOfLawyers,
            541120 => Self::OfficesOfNotaries,
            541191 => Self::TitleAbstractAndSettlementOffices,
            541199 => Self::AllOtherLegalServices,
            541211 => Self::OfficesOfCertifiedPublicAccountants,
            541213 => Self::TaxPreparationServices,
            541214 => Self::PayrollServices,
            541219 => Self::OtherAccountingServices,
            541310 => Self::ArchitecturalServices,
            541320 => Self::LandscapeArchitecturalServices,
            541330 => Self::EngineeringServices,
            541340 => Self::DraftingServices,
            541350 => Self::BuildingInspectionServices,
            541360 => Self::GeophysicalSurveyingAndMappingServices,
            541370 => Self::SurveyingAndMappingExceptGeophysicalServices,
            541380 => Self::TestingLaboratories,
            541410 => Self::InteriorDesignServices,
            541420 => Self::IndustrialDesignServices,
            541430 => Self::GraphicDesignServices,
            541490 => Self::OtherSpecializedDesignServices,
            541511 => Self::CustomComputerProgrammingServices,
            541512 => Self::ComputerSystemsDesignServices,
            541513 => Self::ComputerFacilitiesManagementServices,
            541519 => Self::OtherComputerRelatedServices,
            541611 => Self::AdministrativeManagementAndGeneralManagementConsultingServices,
            541612 => Self::HumanResourcesConsultingServices,
            541613 => Self::MarketingConsultingServices,
            541614 => Self::ProcessPhysicalDistributionAndLogisticsConsultingServices,
            541618 => Self::OtherManagementConsultingServices,
            541620 => Self::EnvironmentalConsultingServices,
            541690 => Self::OtherScientificAndTechnicalConsultingServices,
            541713 => Self::ResearchAndDevelopmentInNanotechnology,
            541714 => Self::ResearchAndDevelopmentInBiotechnologyExceptNanobiotechnology,
            541715 => Self::ResearchAndDevelopmentInThePhysicalEngineeringAndLifeSciencesExceptNanotechnologyAndBiotechnology,
            541720 => Self::ResearchAndDevelopmentInTheSocialSciencesAndHumanities,
            541810 => Self::AdvertisingAgencies,
            541820 => Self::PublicRelationsAgencies,
            541830 => Self::MediaBuyingAgencies,
            541840 => Self::MediaRepresentatives,
            541850 => Self::OutdoorAdvertising,
            541860 => Self::DirectMailAdvertising,
            541870 => Self::AdvertisingMaterialDistributionServices,
            541890 => Self::OtherServicesRelatedToAdvertising,
            541910 => Self::MarketingResearchAndPublicOpinionPolling,
            541921 => Self::PhotographyStudiosPortrait,
            541922 => Self::CommercialPhotography,
            541930 => Self::TranslationAndInterpretationServices,
            541940 => Self::VeterinaryServices,
            541990 => Self::AllOtherProfessionalScientificAndTechnicalServices,
            551111 => Self::OfficesOfBankHoldingCompanies,
            551112 => Self::OfficesOfOtherHoldingCompanies,
            551113 => Self::CorporateSubsidiaryAndRegionalManagingOfficesParent,
            551114 => Self::CorporateSubsidiaryAndRegionalManagingOffices,
            561110 => Self::OfficeAdministrativeServices,
            561210 => Self::FacilitiesSupportServices,
            561311 => Self::EmploymentPlacementAgencies,
            561312 => Self::ExecutiveSearchServices,
            561320 => Self::TemporaryHelpServices,
            561330 => Self::ProfessionalEmployerOrganizations,
            561410 => Self::DocumentPreparationServices,
            561421 => Self::TelephoneAnsweringServices,
            561422 => Self::TelemarketingBureausAndOtherContactCenters,
            561431 => Self::PrivateMailCenters,
            561439 => Self::OtherBusinessServiceCentersIncludingCopyShops,
            561440 => Self::CollectionAgencies,
            561450 => Self::CreditBureaus,
            561491 => Self::RepossessionServices,
            561492 => Self::CourtReportingAndStenotypeServices,
            561499 => Self::AllOtherBusinessSupportServices,
            561510 => Self::TravelAgencies,
            561520 => Self::TourOperators,
            561591 => Self::ConventionAndVisitorsBureaus,
            561599 => Self::AllOtherTravelArrangementAndReservationServices,
            561611 => Self::InvestigationServices,
            561612 => Self::SecurityGuardsAndPatrolServices,
            561613 => Self::ArmoredCarServices,
            561621 => Self::SecuritySystemsServicesExceptLocksmiths,
            561622 => Self::Locksmiths,
            561710 => Self::ExterminatingAndPestControlServices,
            561720 => Self::JanitorialServices,
            561730 => Self::LandscapingServices,
            561740 => Self::CarpetAndUpholsteryCleaningServices,
            561790 => Self::OtherServicesToBuildingsAndDwellings,
            561910 => Self::PackagingAndLabelingServices,
            561920 => Self::ConventionAndTradeShowOrganizers,
            561990 => Self::AllOtherSupportServices,
            562111 => Self::SolidWasteCollection,
            562112 => Self::HazardousWasteCollection,
            562119 => Self::OtherWasteCollection,
            562211 => Self::HazardousWasteTreatmentAndDisposal,
            562212 => Self::SolidWasteLandfill,
            562213 => Self::SolidWasteCombustorsAndIncinerators,
            562219 => Self::OtherNonhazardousWasteTreatmentAndDisposal,
            562910 => Self::RemediationServices,
            562920 => Self::MaterialsRecoveryFacilities,
            562991 => Self::SepticTankAndRelatedServices,
            562998 => Self::AllOtherMiscellaneousWasteManagementServices,
            611110 => Self::ElementaryAndSecondarySchools,
            611210 => Self::JuniorColleges,
            611310 => Self::CollegesUniversitiesAndProfessionalSchools,
            611410 => Self::BusinessAndSecretarialSchools,
            611420 => Self::ComputerTraining,
            611430 => Self::ProfessionalAndManagementDevelopmentTraining,
            611511 => Self::CosmetologyAndBarberSchools,
            611512 => Self::FlightTraining,
            611513 => Self::ApprenticeshipTraining,
            611519 => Self::OtherTechnicalAndTradeSchools,
            611610 => Self::FineArtsSchools,
            611620 => Self::SportsAndRecreationInstruction,
            611630 => Self::LanguageSchools,
            611691 => Self::ExamPreparationAndTutoring,
            611692 => Self::AutomobileDrivingSchools,
            611699 => Self::AllOtherMiscellaneousSchoolsAndInstruction,
            611710 => Self::EducationalSupportServices,
            621111 => Self::OfficesOfPhysiciansExceptMentalHealthSpecialists,
            621112 => Self::OfficesOfPhysiciansMentalHealthSpecialists,
            621210 => Self::OfficesOfDentists,
            621310 => Self::OfficesOfChiropractors,
            621320 => Self::OfficesOfOptometrists,
            621330 => Self::OfficesOfMentalHealthPractitionersExceptPhysicians,
            621340 => Self::OfficesOfPhysicalOccupationalAndSpeechTherapistsAndAudiologists,
            621391 => Self::OfficesOfPodiatrists,
            621399 => Self::OfficesOfAllOtherMiscellaneousHealthPractitioners,
            621410 => Self::FamilyPlanningCenters,
            621420 => Self::OutpatientMentalHealthAndSubstanceAbuseCenters,
            621491 => Self::HmoMedicalCenters,
            621492 => Self::KidneyDialysisCenters,
            621493 => Self::FreestandingAmbulatorySurgicalAndEmergencyCenters,
            621498 => Self::AllOtherOutpatientCareCenters,
            621511 => Self::MedicalLaboratories,
            621512 => Self::DiagnosticImagingCenters,
            621610 => Self::HomeHealthCareServices,
            621910 => Self::AmbulanceServices,
            621991 => Self::BloodAndOrganBanks,
            621999 => Self::AllOtherMiscellaneousAmbulatoryHealthCareServices,
            622110 => Self::GeneralMedicalAndSurgicalHospitals,
            622210 => Self::PsychiatricAndSubstanceAbuseHospitals,
            622310 => Self::SpecialtyExceptPsychiatricAndSubstanceAbuseHospitals,
            623110 => Self::NursingCareFacilitiesSkilledNursingFacilities,
            623210 => Self::ResidentialIntellectualAndDevelopmentalDisabilityFacilities,
            623220 => Self::ResidentialMentalHealthAndSubstanceAbuseFacilities,
            623311 => Self::ContinuingCareRetirementCommunities,
            623312 => Self::AssistedLivingFacilitiesForTheElderly,
            623990 => Self::OtherResidentialCareFacilities,
            624110 => Self::ChildAndYouthServices,
            624120 => Self::ServicesForTheElderlyAndPersonsWithDisabilities,
            624190 => Self::OtherIndividualAndFamilyServices,
            624210 => Self::CommunityFoodServices,
            624221 => Self::TemporaryShelters,
            624229 => Self::OtherCommunityHousingServices,
            624230 => Self::EmergencyAndOtherReliefServices,
            624310 => Self::VocationalRehabilitationServices,
            624410 => Self::ChildDayCareServices,
            711110 => Self::TheaterCompaniesAndDinnerTheaters,
            711120 => Self::DanceCompanies,
            711130 => Self::MusicalGroupsAndArtists,
            711190 => Self::OtherPerformingArtsCompanies,
            711211 => Self::SportsTeamsAndClubs,
            711212 => Self::Racetracks,
            711219 => Self::OtherSpectatorSports,
            711310 => Self::PromotersOfPerformingArtsSportsAndSimilarEventsWithFacilities,
            711320 => Self::PromotersOfPerformingArtsSportsAndSimilarEventsWithoutFacilities,
            711410 => Self::AgentsAndManagersForArtistsAthletesEntertainersAndOtherPublicFigures,
            711510 => Self::IndependentArtistsWritersAndPerformers,
            712110 => Self::Museums,
            712120 => Self::HistoricalSites,
            712130 => Self::ZoosAndBotanicalGardens,
            712190 => Self::NatureParksAndOtherSimilarInstitutions,
            713110 => Self::AmusementAndThemeParks,
            713120 => Self::AmusementArcades,
            713210 => Self::CasinosExceptCasinoHotels,
            713290 => Self::OtherGamblingIndustries,
            713910 => Self::GolfCoursesAndCountryClubs,
            713920 => Self::SkiingFacilities,
            713930 => Self::Marinas,
            713940 => Self::FitnessAndRecreationalSportsCenters,
            713950 => Self::BowlingCenters,
            713990 => Self::AllOtherAmusementAndRecreationIndustries,
            721110 => Self::HotelsExceptCasinoHotelsAndMotels,
            721120 => Self::CasinoHotels,
            721191 => Self::BedAndBreakfastInns,
            721199 => Self::AllOtherTravelerAccommodation,
            721211 => Self::RvRecreationalVehicleParksAndCampgrounds,
            721214 => Self::RecreationalAndVacationCampsExceptCampgrounds,
            721310 => Self::RoomingAndBoardingHouses,
            722310 => Self::FoodServiceContractors,
            722320 => Self::Caterers,
            722330 => Self::MobileFoodServices,
            722410 => Self::DrinkingPlacesAlcoholicBeverages,
            722511 => Self::FullServiceRestaurants,
            722513 => Self::LimitedServiceRestaurants,
            722514 => Self::CafeteriasGrillBuffetsAndBuffets,
            722515 => Self::SnackAndNonalcoholicBeverageBars,
            811111 => Self::GeneralAutomotiveRepair,
            811112 => Self::AutomotiveExhaustSystemRepair,
            811113 => Self::AutomotiveTransmissionRepair,
            811118 => Self::OtherAutomotiveMechanicalAndElectricalRepairAndMaintenance,
            811121 => Self::AutomotiveBodyPaintAndInteriorRepairAndMaintenance,
            811122 => Self::AutomotiveGlassReplacementShops,
            811191 => Self::AutomotiveOilChangeAndLubricationShops,
            811192 => Self::CarWashes,
            811198 => Self::AllOtherAutomotiveRepairAndMaintenance,
            811211 => Self::ConsumerElectronicsRepairAndMaintenance,
            811212 => Self::ComputerAndOfficeMachineRepairAndMaintenance,
            811213 => Self::CommunicationEquipmentRepairAndMaintenance,
            811219 => Self::OtherElectronicAndPrecisionEquipmentRepairAndMaintenance,
            811310 => Self::CommercialAndIndustrialMachineryAndEquipmentExceptAutomotiveAndElectronicRepairAndMaintenance,
            811411 => Self::HomeAndGardenEquipmentRepairAndMaintenance,
            811412 => Self::ApplianceRepairAndMaintenance,
            811420 => Self::ReupholsteryAndFurnitureRepair,
            811430 => Self::FootwearAndLeatherGoodsRepair,
            811490 => Self::OtherPersonalAndHouseholdGoodsRepairAndMaintenance,
            812111 => Self::BarberShops,
            812112 => Self::BeautySalons,
            812113 => Self::NailSalons,
            812191 => Self::DietAndWeightReducingCenters,
            812199 => Self::OtherPersonalCareServices,
            812210 => Self::FuneralHomesAndFuneralServices,
            812220 => Self::CemeteriesAndCrematories,
            812310 => Self::CoinOperatedLaundriesAndDrycleaners,
            812320 => Self::DrycleaningAndLaundryServicesExceptCoinOperated,
            812331 => Self::LinenSupply,
            812332 => Self::IndustrialLaunderers,
            812910 => Self::PetCareExceptVeterinaryServices,
            812921 => Self::PhotofinishingLaboratoriesExceptOneHour,
            812922 => Self::OneHourPhotofinishing,
            812930 => Self::ParkingLotsAndGarages,
            812990 => Self::AllOtherPersonalServices,
            813110 => Self::ReligiousOrganizations,
            813211 => Self::GrantmakingFoundations,
            813212 => Self::VoluntaryHealthOrganizations,
            813219 => Self::OtherGrantmakingAndGivingServices,
            813311 => Self::HumanRightsOrganizations,
            813312 => Self::EnvironmentConservationAndWildlifeOrganizations,
            813319 => Self::OtherSocialAdvocacyOrganizations,
            813410 => Self::CivicAndSocialOrganizations,
            813910 => Self::BusinessAssociations,
            813920 => Self::ProfessionalOrganizations,
            813930 => Self::LaborUnionsAndSimilarLaborOrganizations,
            813940 => Self::PoliticalOrganizations,
            813990 => Self::OtherSimilarOrganizationsExceptBusinessProfessionalLaborAndPoliticalOrganizations,
            814110 => Self::PrivateHouseholds,
            921110 => Self::ExecutiveOffices,
            921120 => Self::LegislativeBodies,
            921130 => Self::PublicFinanceActivities,
            921140 => Self::ExecutiveAndLegislativeOfficesCombined,
            921150 => Self::AmericanIndianAndAlaskaNativeTribalGovernments,
            921190 => Self::OtherGeneralGovernmentSupport,
            922110 => Self::Courts,
            922120 => Self::PoliceProtection,
            922130 => Self::LegalCounselAndProsecution,
            922140 => Self::CorrectionalInstitutions,
            922150 => Self::ParoleOfficesAndProbationOffices,
            922160 => Self::FireProtection,
            922190 => Self::OtherJusticePublicOrderAndSafetyActivities,
            923110 => Self::AdministrationOfEducationPrograms,
            923120 => Self::AdministrationOfPublicHealthPrograms,
            923130 => Self::AdministrationOfHumanResourceProgramsExceptEducationPublicHealthAndVeteransAffairsPrograms,
            923140 => Self::AdministrationOfVeteransAffairs,
            924110 => Self::AdministrationOfAirAndWaterResourceAndSolidWasteManagementPrograms,
            924120 => Self::AdministrationOfConservationPrograms,
            925110 => Self::AdministrationOfHousingPrograms,
            925120 => Self::AdministrationOfUrbanPlanningAndCommunityAndRuralDevelopment,
            926110 => Self::AdministrationOfGeneralEconomicPrograms,
            926120 => Self::RegulationAndAdministrationOfTransportationPrograms,
            926130 => Self::RegulationAndAdministrationOfCommunicationsElectricGasAndOtherUtilities,
            926140 => Self::RegulationOfAgriculturalMarketingAndCommodities,
            926150 => Self::RegulationLicensingAndInspectionOfMiscellaneousCommercialSectors,
            927110 => Self::SpaceResearchAndTechnology,
            928110 => Self::NationalSecurity,
            928120 => Self::InternationalAffairs,
            999999 => Self::UnclassifiedEstablishments,
            _ => return None,
        };

        Some(industry)
    }
}
