use crate::{Code, Describe};

/// GDP tables from the Bureau of Economic Analysis.
///
/// This enum represents the various GDP-related data tables published by the BEA,
/// covering value added, gross output, intermediate inputs, and various price and
/// quantity indexes by industry.
#[derive(
    Debug,
    Copy,
    Clone,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Deserialize,
    serde::Serialize,
    strum::EnumIter,
    derive_more::FromStr,
    derive_more::Display,
)]
/// GDP tables from the Bureau of Economic Analysis.
///
/// This enum represents the various GDP-related data tables published by the BEA,
/// covering value added, gross output, intermediate inputs, and various price and
/// quantity indexes by industry.
pub enum GdpTable {
    #[default]
    /// Chain-Type Price Indexes for Energy Inputs by Industry [2017=100]
    ChainTypePriceIndexesForEnergyInputsByIndustry,
    /// Chain-Type Price Indexes for Gross Output by Industry [2017=100]
    ChainTypePriceIndexesForGrossOutputByIndustry,
    /// Chain-Type Price Indexes for Intermediate Inputs by Industry [2017=100]
    ChainTypePriceIndexesForIntermediateInputsByIndustry,
    /// Chain-Type Price Indexes for Materials Inputs by Industry [2017=100]
    ChainTypePriceIndexesForMaterialsInputsByIndustry,
    /// Chain-Type Price Indexes for Purchased Service Inputs by Industry [2017=100]
    ChainTypePriceIndexesForPurchasedServiceInputsByIndustry,
    /// Chain-Type Price Indexes for Value Added by Industry [2017=100]
    ChainTypePriceIndexesForValueAddedByIndustry,
    /// Chain-Type Quantity Indexes for Energy Inputs by Industry [2017=100]
    ChainTypeQuantityIndexesForEnergyInputsByIndustry,
    /// Chain-Type Quantity Indexes for Gross Output by Industry [2017=100]
    ChainTypeQuantityIndexesForGrossOutputByIndustry,
    /// Chain-Type Quantity Indexes for Intermediate Inputs by Industry [2017=100]
    ChainTypeQuantityIndexesForIntermediateInputsByIndustry,
    /// Chain-Type Quantity Indexes for Materials Inputs by Industry [2017=100]
    ChainTypeQuantityIndexesForMaterialsInputsByIndustry,
    /// Chain-Type Quantity Indexes for Purchased Service Inputs by Industry [2017=100]
    ChainTypeQuantityIndexesForPurchasedServiceInputsByIndustry,
    /// Chain-Type Quantity Indexes for Value Added by Industry [2017=100]
    ChainTypeQuantityIndexesForValueAddedByIndustry,
    /// Components of Value Added by Industry [Billions of dollars]
    ComponentsOfValueAddedByIndustry,
    /// Components of Value Added by Industry as a Percentage of Value Added [Percent]
    ComponentsOfValueAddedByIndustryAsPercentageOfValueAdded,
    /// Composition of Gross Output by Industry [Billions of dollars]
    CompositionOfGrossOutputByIndustry,
    /// Contributions to Percent Change by Industry in the Chain-Type Price Index for All Industries Energy Inputs
    ContributionsToPercentChangeByIndustryChainTypePriceIndexForAllIndustriesEnergyInputs,
    /// Contributions to Percent Change by Industry in the Chain-Type Price Index for All Industries Materials Inputs
    ContributionsToPercentChangeByIndustryChainTypePriceIndexForAllIndustriesMaterialsInputs,
    /// Contributions to Percent Change by Industry in the Chain-Type Price Index for All Industries Purchased Service Inputs
    ContributionsToPercentChangeByIndustryChainTypePriceIndexForAllIndustriesPurchasedServiceInputs,
    /// Contributions to Percent Change by Industry in the Chain-Type Quantity Index for All Industries Energy Inputs
    ContributionsToPercentChangeByIndustryChainTypeQuantityIndexForAllIndustriesEnergyInputs,
    /// Contributions to Percent Change by Industry in the Chain-Type Quantity Index for All Industries Materials Inputs
    ContributionsToPercentChangeByIndustryChainTypeQuantityIndexForAllIndustriesMaterialsInputs,
    /// Contributions to Percent Change by Industry in the Chain-Type Quantity Index for All Industries Purchased Service Inputs
    ContributionsToPercentChangeByIndustryChainTypeQuantityIndexForAllIndustriesPurchasedServiceInputs,
    /// Contributions to Percent Change in the Chain-Type Price Index for Gross Domestic Product by Industry [Percent and percentage points]
    ContributionsToPercentChangeChainTypePriceIndexForGrossDomesticProductByIndustry,
    /// Contributions to Percent Change in Real Gross Domestic Product by Industry [Percent and percentage points]
    ContributionsToPercentChangeRealGrossDomesticProductByIndustry,
    /// Contributions to Percent Changes in Chain-Type Price Indexes for Gross Output by Industry Group [Percent and percentage points]
    ContributionsToPercentChangesChainTypePriceIndexesForGrossOutputByIndustryGroup,
    /// Contributions to Percent Changes in Chain-Type Quantity Indexes for Gross Output by Industry Group [Percent and percentage points]
    ContributionsToPercentChangesChainTypeQuantityIndexesForGrossOutputByIndustryGroup,
    /// Gross Output by Industry [Billions of dollars]
    GrossOutputByIndustry,
    /// Intermediate Inputs by Industry [Billions of dollars]
    IntermediateInputsByIndustry,
    /// Percent Changes in Chain-Type Price Indexes for Gross Output by Industry [Percent change]
    PercentChangesChainTypePriceIndexesForGrossOutputByIndustry,
    /// Percent Changes in Chain-Type Price Indexes for Intermediate Inputs by Industry [Percent change]
    PercentChangesChainTypePriceIndexesForIntermediateInputsByIndustry,
    /// Percent Changes in Chain-Type Price Indexes for Value Added by Industry [Percent change]
    PercentChangesChainTypePriceIndexesForValueAddedByIndustry,
    /// Percent Changes in Chain-Type Quantity Indexes for Gross Output by Industry [Percent change]
    PercentChangesChainTypeQuantityIndexesForGrossOutputByIndustry,
    /// Percent Changes in Chain-Type Quantity Indexes for Intermediate Inputs by Industry [Percent change]
    PercentChangesChainTypeQuantityIndexesForIntermediateInputsByIndustry,
    /// Percent Changes in Chain-Type Quantity Indexes for Value Added by Industry [Percent change]
    PercentChangesChainTypeQuantityIndexesForValueAddedByIndustry,
    /// Real Gross Output by Industry [Billions of 2017 chain dollars]
    RealGrossOutputByIndustry,
    /// Real Intermediate Inputs by Industry [Billions of 2017 chain dollars]
    RealIntermediateInputsByIndustry,
    /// Real Value Added by Industry [Billions of 2017 chain dollars]
    RealValueAddedByIndustry,
    /// Shares of Gross Output by Industry [Percent]
    SharesOfGrossOutputByIndustry,
    /// Value Added by Industry [Billions of dollars]
    ValueAddedByIndustry,
    /// Value added by Industry as a Percentage of Gross Domestic Product [Percent]
    ValueAddedByIndustryAsPercentageOfGrossDomesticProduct,
}

impl Describe for GdpTable {
    fn description(&self) -> &'static str {
        match self {
            Self::ChainTypePriceIndexesForEnergyInputsByIndustry => "Chain-Type Price Indexes for Energy Inputs by Industry [2017=100]",
            Self::ChainTypePriceIndexesForGrossOutputByIndustry => "Chain-Type Price Indexes for Gross Output by Industry [2017=100]",
            Self::ChainTypePriceIndexesForIntermediateInputsByIndustry => "Chain-Type Price Indexes for Intermediate Inputs by Industry [2017=100]",
            Self::ChainTypePriceIndexesForMaterialsInputsByIndustry => "Chain-Type Price Indexes for Materials Inputs by Industry [2017=100]",
            Self::ChainTypePriceIndexesForPurchasedServiceInputsByIndustry => "Chain-Type Price Indexes for Purchased Service Inputs by Industry [2017=100]",
            Self::ChainTypePriceIndexesForValueAddedByIndustry => "Chain-Type Price Indexes for Value Added by Industry [2017=100]",
            Self::ChainTypeQuantityIndexesForEnergyInputsByIndustry => "Chain-Type Quantity Indexes for Energy Inputs by Industry [2017=100]",
            Self::ChainTypeQuantityIndexesForGrossOutputByIndustry => "Chain-Type Quantity Indexes for Gross Output by Industry [2017=100]",
            Self::ChainTypeQuantityIndexesForIntermediateInputsByIndustry => "Chain-Type Quantity Indexes for Intermediate Inputs by Industry [2017=100]",
            Self::ChainTypeQuantityIndexesForMaterialsInputsByIndustry => "Chain-Type Quantity Indexes for Materials Inputs by Industry [2017=100]",
            Self::ChainTypeQuantityIndexesForPurchasedServiceInputsByIndustry => "Chain-Type Quantity Indexes for Purchased Service Inputs by Industry [2017=100]",
            Self::ChainTypeQuantityIndexesForValueAddedByIndustry => "Chain-Type Quantity Indexes for Value Added by Industry [2017=100]",
            Self::ComponentsOfValueAddedByIndustry => "Components of Value Added by Industry [Billions of dollars]",
            Self::ComponentsOfValueAddedByIndustryAsPercentageOfValueAdded => "Components of Value Added by Industry as a Percentage of Value Added [Percent]",
            Self::CompositionOfGrossOutputByIndustry => "Composition of Gross Output by Industry [Billions of dollars]",
            Self::ContributionsToPercentChangeByIndustryChainTypePriceIndexForAllIndustriesEnergyInputs => "Contributions to Percent Change by Industry in the Chain-Type Price Index for All Industries Energy Inputs ",
            Self::ContributionsToPercentChangeByIndustryChainTypePriceIndexForAllIndustriesMaterialsInputs => "Contributions to Percent Change by Industry in the Chain-Type Price Index for All Industries Materials Inputs ",
            Self::ContributionsToPercentChangeByIndustryChainTypePriceIndexForAllIndustriesPurchasedServiceInputs => "Contributions to Percent Change by Industry in the Chain-Type Price Index for All Industries Purchased Service Inputs ",
            Self::ContributionsToPercentChangeByIndustryChainTypeQuantityIndexForAllIndustriesEnergyInputs => "Contributions to Percent Change by Industry in the Chain-Type Quantity Index for All Industries Energy Inputs ",
            Self::ContributionsToPercentChangeByIndustryChainTypeQuantityIndexForAllIndustriesMaterialsInputs => "Contributions to Percent Change by Industry in the Chain-Type Quantity Index for All Industries Materials Inputs ",
            Self::ContributionsToPercentChangeByIndustryChainTypeQuantityIndexForAllIndustriesPurchasedServiceInputs => "Contributions to Percent Change by Industry in the Chain-Type Quantity Index for All Industries Purchased Service Inputs ",
            Self::ContributionsToPercentChangeChainTypePriceIndexForGrossDomesticProductByIndustry => "Contributions to Percent Change in the Chain-Type Price Index for Gross Domestic Product by Industry [Percent and percentage points]",
            Self::ContributionsToPercentChangeRealGrossDomesticProductByIndustry => "Contributions to Percent Change in Real Gross Domestic Product by Industry [Percent and percentage points]",
            Self::ContributionsToPercentChangesChainTypePriceIndexesForGrossOutputByIndustryGroup => "Contributions to Percent Changes in Chain-Type Price Indexes for Gross Output by Industry Group [Percent and percentage points]",
            Self::ContributionsToPercentChangesChainTypeQuantityIndexesForGrossOutputByIndustryGroup => "Contributions to Percent Changes in Chain-Type Quantity Indexes for Gross Output by Industry Group [Percent and percentage points]",
            Self::GrossOutputByIndustry => "Gross Output by Industry [Billions of dollars]",
            Self::IntermediateInputsByIndustry => "Intermediate Inputs by Industry [Billions of dollars]",
            Self::PercentChangesChainTypePriceIndexesForGrossOutputByIndustry => "Percent Changes in Chain-Type Price Indexes for Gross Output by Industry [Percent change]",
            Self::PercentChangesChainTypePriceIndexesForIntermediateInputsByIndustry => "Percent Changes in Chain-Type Price Indexes for Intermediate Inputs by Industry [Percent change]",
            Self::PercentChangesChainTypePriceIndexesForValueAddedByIndustry => "Percent Changes in Chain-Type Price Indexes for Value Added by Industry [Percent change]",
            Self::PercentChangesChainTypeQuantityIndexesForGrossOutputByIndustry => "Percent Changes in Chain-Type Quantity Indexes for Gross Output by Industry [Percent change]",
            Self::PercentChangesChainTypeQuantityIndexesForIntermediateInputsByIndustry => "Percent Changes in Chain-Type Quantity Indexes for Intermediate Inputs by Industry [Percent change]",
            Self::PercentChangesChainTypeQuantityIndexesForValueAddedByIndustry => "Percent Changes in Chain-Type Quantity Indexes for Value Added by Industry [Percent change]",
            Self::RealGrossOutputByIndustry => "Real Gross Output by Industry [Billions of 2017 chain dollars]",
            Self::RealIntermediateInputsByIndustry => "Real Intermediate Inputs by Industry [Billions of 2017 chain dollars]",
            Self::RealValueAddedByIndustry => "Real Value Added by Industry [Billions of 2017 chain dollars]",
            Self::SharesOfGrossOutputByIndustry => "Shares of Gross Output by Industry [Percent]",
            Self::ValueAddedByIndustry => "Value Added by Industry [Billions of dollars]",
            Self::ValueAddedByIndustryAsPercentageOfGrossDomesticProduct => "Value added by Industry as a Percentage of Gross Domestic Product [Percent]",
        }
    }
}

impl Code<i64> for GdpTable {
    type Decoded = Self;

    fn code(&self) -> i64 {
        match self {
            Self::ChainTypePriceIndexesForEnergyInputsByIndustry => 33,
            Self::ChainTypePriceIndexesForGrossOutputByIndustry => 18,
            Self::ChainTypePriceIndexesForIntermediateInputsByIndustry => 23,
            Self::ChainTypePriceIndexesForMaterialsInputsByIndustry => 37,
            Self::ChainTypePriceIndexesForPurchasedServiceInputsByIndustry => 41,
            Self::ChainTypePriceIndexesForValueAddedByIndustry => 11,
            Self::ChainTypeQuantityIndexesForEnergyInputsByIndustry => 31,
            Self::ChainTypeQuantityIndexesForGrossOutputByIndustry => 16,
            Self::ChainTypeQuantityIndexesForIntermediateInputsByIndustry => 21,
            Self::ChainTypeQuantityIndexesForMaterialsInputsByIndustry => 35,
            Self::ChainTypeQuantityIndexesForPurchasedServiceInputsByIndustry => 39,
            Self::ChainTypeQuantityIndexesForValueAddedByIndustry => 8,
            Self::ComponentsOfValueAddedByIndustry => 6,
            Self::ComponentsOfValueAddedByIndustryAsPercentageOfValueAdded => 7,
            Self::CompositionOfGrossOutputByIndustry => 25,
            Self::ContributionsToPercentChangeByIndustryChainTypePriceIndexForAllIndustriesEnergyInputs => 34,
            Self::ContributionsToPercentChangeByIndustryChainTypePriceIndexForAllIndustriesMaterialsInputs => 38,
            Self::ContributionsToPercentChangeByIndustryChainTypePriceIndexForAllIndustriesPurchasedServiceInputs => 42,
            Self::ContributionsToPercentChangeByIndustryChainTypeQuantityIndexForAllIndustriesEnergyInputs => 32,
            Self::ContributionsToPercentChangeByIndustryChainTypeQuantityIndexForAllIndustriesMaterialsInputs => 36,
            Self::ContributionsToPercentChangeByIndustryChainTypeQuantityIndexForAllIndustriesPurchasedServiceInputs => 40,
            Self::ContributionsToPercentChangeChainTypePriceIndexForGrossDomesticProductByIndustry => 14,
            Self::ContributionsToPercentChangeRealGrossDomesticProductByIndustry => 13,
            Self::ContributionsToPercentChangesChainTypePriceIndexesForGrossOutputByIndustryGroup => 30,
            Self::ContributionsToPercentChangesChainTypeQuantityIndexesForGrossOutputByIndustryGroup => 29,
            Self::GrossOutputByIndustry => 15,
            Self::IntermediateInputsByIndustry => 20,
            Self::PercentChangesChainTypePriceIndexesForGrossOutputByIndustry => 19,
            Self::PercentChangesChainTypePriceIndexesForIntermediateInputsByIndustry => 24,
            Self::PercentChangesChainTypePriceIndexesForValueAddedByIndustry => 12,
            Self::PercentChangesChainTypeQuantityIndexesForGrossOutputByIndustry => 17,
            Self::PercentChangesChainTypeQuantityIndexesForIntermediateInputsByIndustry => 22,
            Self::PercentChangesChainTypeQuantityIndexesForValueAddedByIndustry => 9,
            Self::RealGrossOutputByIndustry => 208,
            Self::RealIntermediateInputsByIndustry => 209,
            Self::RealValueAddedByIndustry => 10,
            Self::SharesOfGrossOutputByIndustry => 26,
            Self::ValueAddedByIndustry => 1,
            Self::ValueAddedByIndustryAsPercentageOfGrossDomesticProduct => 5,
        }
    }

    fn from_code(code: i64) -> Option<Self> {
        let table = match code {
            1 => Self::ValueAddedByIndustry,
            5 => Self::ValueAddedByIndustryAsPercentageOfGrossDomesticProduct,
            6 => Self::ComponentsOfValueAddedByIndustry,
            7 => Self::ComponentsOfValueAddedByIndustryAsPercentageOfValueAdded,
            8 => Self::ChainTypeQuantityIndexesForValueAddedByIndustry,
            9 => Self::PercentChangesChainTypeQuantityIndexesForValueAddedByIndustry,
            10 => Self::RealValueAddedByIndustry,
            11 => Self::ChainTypePriceIndexesForValueAddedByIndustry,
            12 => Self::PercentChangesChainTypePriceIndexesForValueAddedByIndustry,
            13 => Self::ContributionsToPercentChangeRealGrossDomesticProductByIndustry,
            14 => Self::ContributionsToPercentChangeChainTypePriceIndexForGrossDomesticProductByIndustry,
            15 => Self::GrossOutputByIndustry,
            16 => Self::ChainTypeQuantityIndexesForGrossOutputByIndustry,
            17 => Self::PercentChangesChainTypeQuantityIndexesForGrossOutputByIndustry,
            18 => Self::ChainTypePriceIndexesForGrossOutputByIndustry,
            19 => Self::PercentChangesChainTypePriceIndexesForGrossOutputByIndustry,
            20 => Self::IntermediateInputsByIndustry,
            21 => Self::ChainTypeQuantityIndexesForIntermediateInputsByIndustry,
            22 => Self::PercentChangesChainTypeQuantityIndexesForIntermediateInputsByIndustry,
            23 => Self::ChainTypePriceIndexesForIntermediateInputsByIndustry,
            24 => Self::PercentChangesChainTypePriceIndexesForIntermediateInputsByIndustry,
            25 => Self::CompositionOfGrossOutputByIndustry,
            26 => Self::SharesOfGrossOutputByIndustry,
            29 => Self::ContributionsToPercentChangesChainTypeQuantityIndexesForGrossOutputByIndustryGroup,
            30 => Self::ContributionsToPercentChangesChainTypePriceIndexesForGrossOutputByIndustryGroup,
            31 => Self::ChainTypeQuantityIndexesForEnergyInputsByIndustry,
            32 => Self::ContributionsToPercentChangeByIndustryChainTypeQuantityIndexForAllIndustriesEnergyInputs,
            33 => Self::ChainTypePriceIndexesForEnergyInputsByIndustry,
            34 => Self::ContributionsToPercentChangeByIndustryChainTypePriceIndexForAllIndustriesEnergyInputs,
            35 => Self::ChainTypeQuantityIndexesForMaterialsInputsByIndustry,
            36 => Self::ContributionsToPercentChangeByIndustryChainTypeQuantityIndexForAllIndustriesMaterialsInputs,
            37 => Self::ChainTypePriceIndexesForMaterialsInputsByIndustry,
            38 => Self::ContributionsToPercentChangeByIndustryChainTypePriceIndexForAllIndustriesMaterialsInputs,
            39 => Self::ChainTypeQuantityIndexesForPurchasedServiceInputsByIndustry,
            40 => Self::ContributionsToPercentChangeByIndustryChainTypeQuantityIndexForAllIndustriesPurchasedServiceInputs,
            41 => Self::ChainTypePriceIndexesForPurchasedServiceInputsByIndustry,
            42 => Self::ContributionsToPercentChangeByIndustryChainTypePriceIndexForAllIndustriesPurchasedServiceInputs,
            208 => Self::RealGrossOutputByIndustry,
            209 => Self::RealIntermediateInputsByIndustry,
            _ => return None,
        };
        Some(table)
    }
}
