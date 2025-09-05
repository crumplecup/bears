/// Represents values for the TypeOfService parameter in the IntlServTrade dataset.
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
pub enum Service {
   /// Accounting, auditing, and bookkeeping services 
   AccountAuditBookkeep,
   /// Advertising services
   Advertising,
   /// Advertising and related services
   AdvertisingAndRelated,
   /// Services incidental to agriculture, forestry, and fishing
   AgForAndFish,
   /// All types of service
   AllTypesOfService,
   /// Architectural, engineering, scientific, and other technical services
   ArchEngSciAndOthTech,
   /// Architectural services
   Architectural,
   /// Artistic-related services 
   ArtisticRelated,
   /// Audiovisual services 
   AudioVisual,
   /// Audiovisual originals 
   AudVisOriginals,
   /// Books and sound recordings 
   AudVisOriginalsBooksAndSound,
   /// Movies and television programming 
   AudVisOriginalsMoviesAndTv,
   /// Audiovisual production services 
   AudVisProduction,
   /// Rights to use audiovisual products 
   AudVisRightsToUse,
   /// Books and sound recordings 
   AudVisRightsToUseBooksAndSound,
   /// Movies and television programming 
   AudVisRightsToUseMoviesAndTv,
   /// Business and management consulting and public relations services
   BusMgmtConsPubRel,
   /// Charges for the use of intellectual property n.i.e.
   ChargesForTheUseOfIpNie,
   /// Charges for the use of intellectual property for licenses to reproduce and/or distribute computer software (backcasted for ICT services)
   CipCompSoftIct,
   /// Charges for the use of intellectual property n.i.e.; licenses to reproduce and/or distribute audio-visual and related products
   CipLicensesAudVis,
   /// Charges for the use of intellectual property n.i.e.; books and sound recordings
   CipLicensesBooksSoundRecord,
   /// Charges for the use of intellectual property n.i.e.; broadcasting and recording of live events
   CipLicensesBroadcastLiveRecord,
   /// Charges for the use of intellectual property n.i.e.; licenses to reproduce and/or distribute computer software
   CipLicensesCompSoftware,
   /// Charges for the use of intellectual property n.i.e.; franchise fees
   CipLicensesFranchiseFees,
   /// Charges for the use of intellectual property n.i.e.; franchises and trademarks licensing fees
   CipLicensesFranchisesTrademarks,
   /// Charges for the use of intellectual property n.i.e.; movies and television programming
   CipLicensesMoviesTv,
   /// Charges for the use of intellectual property n.i.e.; licenses for the use of outcomes of research and development
   CipLicensesOutcomesResearchAndDev,
   /// Charges for the use of intellectual property n.i.e.; trademarks
   CipLicensesTrademarks,
   /// Cloud computing and data storage services
   CloudCompAndDataStor,
   /// Computer services
   Comp,
   /// Computer software, including end-user licenses and customization
   CompSoftware,
   /// Construction services
   Const,
   /// Construction services abroad or in the United States
   ConstAbroadUs,
   /// Construction services; contractor's expenditures
   ConstExpend,
   /// Database and other information services
   DatabaseAndOthInfo,
   /// Education services 
   Education,
   /// Engineering services
   Engineering,
   /// Financial advisory and custody services
   FinAdvCust,
   /// Financial services
   Financial,
   /// Credit card and other credit-related services
   FinCredCardOthCredRelated,
   /// Explicitly charged and other financial services
   FinExplicitAndOth,
   /// Financial management services
   FinFinMan,
   /// Financial intermediation services indirectly measured
   FinFisim,
   /// Brokerage and market-making services
   FinSecBrokAndMM,
   /// Securities lending, electronic funds transfer, and other services
   FinSecLendEftOth,
   /// Underwriting and private placement services
   FinUwAndPP,
   /// Government goods and services n.i.e.
   GovtGoodsAndServicesNie,
   /// Health services 
   Health,
   /// Heritage and recreational services 
   HeritageAndRec,
   /// ICT services
   IctServ,
   /// Information services
   Info,
   /// Insurance losses paid
   InsLossesPaid,
   /// Insurance losses paid direct insurance
   InsLossesPaidDirIns,
   /// Insurance losses paid reinsurance
   InsLossesPaidReins,
   /// Insurance losses recovered
   InsLossesRecovered,
   /// Insurance losses recovered direct insurance
   InsLossesRecoveredDirIns,
   /// Insurance losses recovered reinsurance
   InsLossesRecoveredReins,
   /// Insurance premiums paid
   InsPremiumsPaid,
   /// Insurance premiums paid direct insurance
   InsPremiumsPaidDirIns,
   /// Insurance premiums paid reinsurance
   InsPremiumsPaidReins,
   /// Insurance premiums received
   InsPremiumsReceived,
   /// Insurance premiums received direct insurance
   InsPremiumsReceivedDirIns,
   /// Insurance premiums received reinsurance
   InsPremiumsReceivedReins,
   /// Insurance services
   Insurance,
   /// Auxiliary insurance services
   InsuranceAuxIns,
   /// Direct insurance services
   InsuranceDirect,
   /// Reinsurance services
   InsuranceReIns,
   /// Legal services
   Legal,
   /// Legal, accounting, management consulting, and public relations services
   LegalAccountMgmtConsAndPubRel,
   /// Maintenance and repair services n.i.e.
   MaintenanceAndRepairNie,
   /// Manufacturing services on physical inputs owned by others
   Manufacturing,
   /// Market research and public opinion polling services
   MarResAndPubOpinPoll,
   /// Mining services
   Mining,
   /// News agency services
   NewsAgency,
   /// Operating leasing services
   OperatingLeasing,
   /// Other business services n.i.e. 
   OthBusinessNie,
   /// Other computer services
   OthComp,
   /// Other business services
   OtherBusiness,
   /// Other personal, cultural, and recreational services 
   OthPersonalCulturalAndRecreational,
   /// Personal, cultural, and recreational services 
   PersonalCulturalAndRecreational,
   /// Digitally deliverable services
   PotIctEnServ,
   /// Other business services that are digitally deliverable
   PotIctEnServOthBusServ,
   /// Personal, cultural, and recreational services that are digitally deliverable
   PotIctEnServPerCultRec,
   /// Technical, trade-related, and other business services that are digitally deliverable
   PotIctEnServTechTradeRelatedOth,
   /// Professional and management consulting services
   ProfMgmtConsult,
   /// Other research and development services
   RDOthRD,
   /// Provision of customized and non-customized research and development services
   RDProvision,
   /// Sale of proprietary rights arising from research and development
   RDSaleRights,
   /// Work undertaken on a systematic basis to increase the stock of knowledge
   RDStockKnowledge,
   /// Research and development services
   ResearchAndDev,
   /// Scientific and other technical services
   SciAndOthTech,
   /// Technical, trade-related, and other business services
   TechTradeRelatedOth,
   /// Telecommunications services
   Telecom,
   /// Telecommunications, computer, and information services
   TelecomCompAndInfo,
   /// Trade exhibition and sales convention services
   TradeExhAndSalesConv,
   /// Trade-related services
   TradeRelated,
   /// Transport services
   Transport,
   /// Air transport services
   TransportAir,
   /// Air freight services
   TransportAirFreight,
   /// Air passenger services
   TransportAirPass,
   /// Air port services
   TransportAirPort,
   /// Transport services; other modes of transport
   TransportOth,
   /// Transport services; other modes of transport; postal
   TransportPostal,
   /// Transport services; other modes of transport; road and other transport
   TransportRoadAndOth,
   /// Sea transport services
   TransportSea,
   /// Sea freight services
   TransportSeaFreight,
   /// Sea port services
   TransportSeaPort,
   /// Travel services
   Travel,
   /// Business travel services
   TravelBusiness,
   /// Other business travel services
   TravelBusinessOth,
   /// Education-related services
   TravelEducation,
   /// Health-related services
   TravelHealth,
   /// Personal travel services
   TravelPersonal,
   /// Other personal travel services
   TravelPersonalOth,
   /// Expenditures by border, seasonal, and other short-term workers
   TravelShortTermWork,
   /// Waste treatment and de-pollution services
   WasteTreatAndDePol,
   /// Waste treatment and de-pollution, agricultural, and mining services
   WasteTreatAndDePolAgAndMining,
}

impl Service {
   pub fn description(&self) -> &'static str {
       match self {
           Self::AccountAuditBookkeep => "Accounting, auditing, and bookkeeping services ",
           Self::Advertising => "Advertising services",
           Self::AdvertisingAndRelated => "Advertising and related services",
           Self::AgForAndFish => "Services incidental to agriculture, forestry, and fishing",
           Self::AllTypesOfService => "All types of service",
           Self::ArchEngSciAndOthTech => "Architectural, engineering, scientific, and other technical services",
           Self::Architectural => "Architectural services",
           Self::ArtisticRelated => "Artistic-related services ",
           Self::AudioVisual => "Audiovisual services ",
           Self::AudVisOriginals => "Audiovisual originals ",
           Self::AudVisOriginalsBooksAndSound => "Books and sound recordings ",
           Self::AudVisOriginalsMoviesAndTv => "Movies and television programming ",
           Self::AudVisProduction => "Audiovisual production services ",
           Self::AudVisRightsToUse => "Rights to use audiovisual products ",
           Self::AudVisRightsToUseBooksAndSound => "Books and sound recordings ",
           Self::AudVisRightsToUseMoviesAndTv => "Movies and television programming ",
           Self::BusMgmtConsPubRel => "Business and management consulting and public relations services",
           Self::ChargesForTheUseOfIpNie => "Charges for the use of intellectual property n.i.e.",
           Self::CipCompSoftIct => "Charges for the use of intellectual property for licenses to reproduce and/or distribute computer software (backcasted for ICT services)",
           Self::CipLicensesAudVis => "Charges for the use of intellectual property n.i.e.; licenses to reproduce and/or distribute audio-visual and related products",
           Self::CipLicensesBooksSoundRecord => "Charges for the use of intellectual property n.i.e.; books and sound recordings",
           Self::CipLicensesBroadcastLiveRecord => "Charges for the use of intellectual property n.i.e.; broadcasting and recording of live events",
           Self::CipLicensesCompSoftware => "Charges for the use of intellectual property n.i.e.; licenses to reproduce and/or distribute computer software",
           Self::CipLicensesFranchiseFees => "Charges for the use of intellectual property n.i.e.; franchise fees",
           Self::CipLicensesFranchisesTrademarks => "Charges for the use of intellectual property n.i.e.; franchises and trademarks licensing fees",
           Self::CipLicensesMoviesTv => "Charges for the use of intellectual property n.i.e.; movies and television programming",
           Self::CipLicensesOutcomesResearchAndDev => "Charges for the use of intellectual property n.i.e.; licenses for the use of outcomes of research and development",
           Self::CipLicensesTrademarks => "Charges for the use of intellectual property n.i.e.; trademarks",
           Self::CloudCompAndDataStor => "Cloud computing and data storage services",
           Self::Comp => "Computer services",
           Self::CompSoftware => "Computer software, including end-user licenses and customization",
           Self::Const => "Construction services",
           Self::ConstAbroadUs => "Construction services abroad or in the United States",
           Self::ConstExpend => "Construction services; contractor's expenditures",
           Self::DatabaseAndOthInfo => "Database and other information services",
           Self::Education => "Education services ",
           Self::Engineering => "Engineering services",
           Self::FinAdvCust => "Financial advisory and custody services",
           Self::Financial => "Financial services",
           Self::FinCredCardOthCredRelated => "Credit card and other credit-related services",
           Self::FinExplicitAndOth => "Explicitly charged and other financial services",
           Self::FinFinMan => "Financial management services",
           Self::FinFisim => "Financial intermediation services indirectly measured",
           Self::FinSecBrokAndMM => "Brokerage and market-making services",
           Self::FinSecLendEftOth => "Securities lending, electronic funds transfer, and other services",
           Self::FinUwAndPP => "Underwriting and private placement services",
           Self::GovtGoodsAndServicesNie => "Government goods and services n.i.e.",
           Self::Health => "Health services ",
           Self::HeritageAndRec => "Heritage and recreational services ",
           Self::IctServ => "ICT services",
           Self::Info => "Information services",
           Self::InsLossesPaid => "Insurance losses paid",
           Self::InsLossesPaidDirIns => "Insurance losses paid direct insurance",
           Self::InsLossesPaidReins => "Insurance losses paid reinsurance",
           Self::InsLossesRecovered => "Insurance losses recovered",
           Self::InsLossesRecoveredDirIns => "Insurance losses recovered direct insurance",
           Self::InsLossesRecoveredReins => "Insurance losses recovered reinsurance",
           Self::InsPremiumsPaid => "Insurance premiums paid",
           Self::InsPremiumsPaidDirIns => "Insurance premiums paid direct insurance",
           Self::InsPremiumsPaidReins => "Insurance premiums paid reinsurance",
           Self::InsPremiumsReceived => "Insurance premiums received",
           Self::InsPremiumsReceivedDirIns => "Insurance premiums received direct insurance",
           Self::InsPremiumsReceivedReins => "Insurance premiums received reinsurance",
           Self::Insurance => "Insurance services",
           Self::InsuranceAuxIns => "Auxiliary insurance services",
           Self::InsuranceDirect => "Direct insurance services",
           Self::InsuranceReIns => "Reinsurance services",
           Self::Legal => "Legal services",
           Self::LegalAccountMgmtConsAndPubRel => "Legal, accounting, management consulting, and public relations services",
           Self::MaintenanceAndRepairNie => "Maintenance and repair services n.i.e.",
           Self::Manufacturing => "Manufacturing services on physical inputs owned by others",
           Self::MarResAndPubOpinPoll => "Market research and public opinion polling services",
           Self::Mining => "Mining services",
           Self::NewsAgency => "News agency services",
           Self::OperatingLeasing => "Operating leasing services",
           Self::OthBusinessNie => "Other business services n.i.e. ",
           Self::OthComp => "Other computer services",
           Self::OtherBusiness => "Other business services",
           Self::OthPersonalCulturalAndRecreational => "Other personal, cultural, and recreational services ",
           Self::PersonalCulturalAndRecreational => "Personal, cultural, and recreational services ",
           Self::PotIctEnServ => "Digitally deliverable services",
           Self::PotIctEnServOthBusServ => "Other business services that are digitally deliverable",
           Self::PotIctEnServPerCultRec => "Personal, cultural, and recreational services that are digitally deliverable",
           Self::PotIctEnServTechTradeRelatedOth => "Technical, trade-related, and other business services that are digitally deliverable",
           Self::ProfMgmtConsult => "Professional and management consulting services",
           Self::RDOthRD => "Other research and development services",
           Self::RDProvision => "Provision of customized and non-customized research and development services",
           Self::RDSaleRights => "Sale of proprietary rights arising from research and development",
           Self::RDStockKnowledge => "Work undertaken on a systematic basis to increase the stock of knowledge",
           Self::ResearchAndDev => "Research and development services",
           Self::SciAndOthTech => "Scientific and other technical services",
           Self::TechTradeRelatedOth => "Technical, trade-related, and other business services",
           Self::Telecom => "Telecommunications services",
           Self::TelecomCompAndInfo => "Telecommunications, computer, and information services",
           Self::TradeExhAndSalesConv => "Trade exhibition and sales convention services",
           Self::TradeRelated => "Trade-related services",
           Self::Transport => "Transport services",
           Self::TransportAir => "Air transport services",
           Self::TransportAirFreight => "Air freight services",
           Self::TransportAirPass => "Air passenger services",
           Self::TransportAirPort => "Air port services",
           Self::TransportOth => "Transport services; other modes of transport",
           Self::TransportPostal => "Transport services; other modes of transport; postal",
           Self::TransportRoadAndOth => "Transport services; other modes of transport; road and other transport",
           Self::TransportSea => "Sea transport services",
           Self::TransportSeaFreight => "Sea freight services",
           Self::TransportSeaPort => "Sea port services",
           Self::Travel => "Travel services",
           Self::TravelBusiness => "Business travel services",
           Self::TravelBusinessOth => "Other business travel services",
           Self::TravelEducation => "Education-related services",
           Self::TravelHealth => "Health-related services",
           Self::TravelPersonal => "Personal travel services",
           Self::TravelPersonalOth => "Other personal travel services",
           Self::TravelShortTermWork => "Expenditures by border, seasonal, and other short-term workers",
           Self::WasteTreatAndDePol => "Waste treatment and de-pollution services",
           Self::WasteTreatAndDePolAgAndMining => "Waste treatment and de-pollution, agricultural, and mining services",
       }
   }
}
