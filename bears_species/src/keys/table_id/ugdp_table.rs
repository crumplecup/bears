use crate::{Code, Describe};

/// Underlying GDP tables from the Bureau of Economic Analysis
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
/// Underlying GDP tables from the Bureau of Economic Analysis
pub enum UgdpTable {
    /// U.Chain-Type Price Indexes for Gross Output by Industry [2017=100]
    ChainTypePriceIndexesForGrossOutputByIndustry,
    /// U.Chain-Type Price Indexes for Intermediate Inputs by Industry [2017=100]
    ChainTypePriceIndexesForIntermediateInputsByIndustry,
    /// U.Chain-Type Price Indexes for Value Added by Industry [2017=100]
    ChainTypePriceIndexesForValueAddedByIndustry,
    /// U.Chain-Type Quantity Indexes for Gross Output by Industry [2017=100]
    ChainTypeQuantityIndexesForGrossOutputByIndustry,
    /// U.Chain-Type Quantity Indexes for Intermediate Inputs by Industry [2017=100]
    ChainTypeQuantityIndexesForIntermediateInputsByIndustry,
    /// U.Chain-Type Quantity Indexes for Value Added by Industry [2017=100]
    ChainTypeQuantityIndexesForValueAddedByIndustry,
    /// U.Contributions to Percent Change in Real Gross Domestic Product by Industry [Percent and percentage points]
    ContributionsToPercentChangeInRealGrossDomesticProductByIndustry,
    /// U.Contributions to Percent Change in the Chain-Type Price Index for Gross Domestic Product by Industry [Percent and percentage points]
    ContributionsToPercentChangeInTheChainTypePriceIndexForGrossDomesticProductByIndustry,
    /// U.Gross Output by Industry [Billions of dollars]
    GrossOutputByIndustry,
    /// U.Intermediate Inputs by Industry [Billions of dollars]
    IntermediateInputsByIndustry,
    /// U.Percent Changes in Chain-Type Price Indexes for Gross Output by Industry [Percent change]
    PercentChangesInChainTypePriceIndexesForGrossOutputByIndustry,
    /// U.Percent Changes in Chain-Type Price Indexes for Intermediate Inputs by Industry [Percent change]
    PercentChangesInChainTypePriceIndexesForIntermediateInputsByIndustry,
    /// U.Percent Changes in Chain-Type Price Indexes for Value Added by Industry [Percent change]
    PercentChangesInChainTypePriceIndexesForValueAddedByIndustry,
    /// U.Percent Changes in Chain-Type Quantity Indexes for Gross Output by Industry [Percent change]
    PercentChangesInChainTypeQuantityIndexesForGrossOutputByIndustry,
    /// U.Percent Changes in Chain-Type Quantity Indexes for Intermediate Inputs by Industry [Percent change]
    PercentChangesInChainTypeQuantityIndexesForIntermediateInputsByIndustry,
    /// U.Percent Changes in Chain-Type Quantity Indexes for Value Added by Industry [Percent change]
    PercentChangesInChainTypeQuantityIndexesForValueAddedByIndustry,
    /// U.Real Gross Output by Industry [Billions of 2017 chain dollars]
    RealGrossOutputByIndustry,
    /// U.Real Intermediate Inputs by Industry [Billions of 2017 chain dollars]
    RealIntermediateInputsByIndustry,
    /// U.Real Value Added by Industry [Billions of 2017 chain dollars]
    RealValueAddedByIndustry,
    /// U.Value Added by Industry [Billions of dollars]
    ValueAddedByIndustry,
    /// U.Value added by Industry as a Percentage of Gross Domestic Product [Percent]
    ValueAddedByIndustryAsAPercentageOfGrossDomesticProduct,
}

impl Describe for UgdpTable {
    fn description(&self) -> &'static str {
        match self {
            Self::ChainTypePriceIndexesForGrossOutputByIndustry => "U.Chain-Type Price Indexes for Gross Output by Industry [2017=100]",
            Self::ChainTypePriceIndexesForIntermediateInputsByIndustry => "U.Chain-Type Price Indexes for Intermediate Inputs by Industry [2017=100]",
            Self::ChainTypePriceIndexesForValueAddedByIndustry => "U.Chain-Type Price Indexes for Value Added by Industry [2017=100]",
            Self::ChainTypeQuantityIndexesForGrossOutputByIndustry => "U.Chain-Type Quantity Indexes for Gross Output by Industry [2017=100]",
            Self::ChainTypeQuantityIndexesForIntermediateInputsByIndustry => "U.Chain-Type Quantity Indexes for Intermediate Inputs by Industry [2017=100]",
            Self::ChainTypeQuantityIndexesForValueAddedByIndustry => "U.Chain-Type Quantity Indexes for Value Added by Industry [2017=100]",
            Self::ContributionsToPercentChangeInRealGrossDomesticProductByIndustry => "U.Contributions to Percent Change in Real Gross Domestic Product by Industry [Percent and percentage points]",
            Self::ContributionsToPercentChangeInTheChainTypePriceIndexForGrossDomesticProductByIndustry => "U.Contributions to Percent Change in the Chain-Type Price Index for Gross Domestic Product by Industry [Percent and percentage points]",
            Self::GrossOutputByIndustry => "U.Gross Output by Industry [Billions of dollars]",
            Self::IntermediateInputsByIndustry => "U.Intermediate Inputs by Industry [Billions of dollars]",
            Self::PercentChangesInChainTypePriceIndexesForGrossOutputByIndustry => "U.Percent Changes in Chain-Type Price Indexes for Gross Output by Industry [Percent change]",
            Self::PercentChangesInChainTypePriceIndexesForIntermediateInputsByIndustry => "U.Percent Changes in Chain-Type Price Indexes for Intermediate Inputs by Industry [Percent change]",
            Self::PercentChangesInChainTypePriceIndexesForValueAddedByIndustry => "U.Percent Changes in Chain-Type Price Indexes for Value Added by Industry [Percent change]",
            Self::PercentChangesInChainTypeQuantityIndexesForGrossOutputByIndustry => "U.Percent Changes in Chain-Type Quantity Indexes for Gross Output by Industry [Percent change]",
            Self::PercentChangesInChainTypeQuantityIndexesForIntermediateInputsByIndustry => "U.Percent Changes in Chain-Type Quantity Indexes for Intermediate Inputs by Industry [Percent change]",
            Self::PercentChangesInChainTypeQuantityIndexesForValueAddedByIndustry => "U.Percent Changes in Chain-Type Quantity Indexes for Value Added by Industry [Percent change]",
            Self::RealGrossOutputByIndustry => "U.Real Gross Output by Industry [Billions of 2017 chain dollars]",
            Self::RealIntermediateInputsByIndustry => "U.Real Intermediate Inputs by Industry [Billions of 2017 chain dollars]",
            Self::RealValueAddedByIndustry => "U.Real Value Added by Industry [Billions of 2017 chain dollars]",
            Self::ValueAddedByIndustry => "U.Value Added by Industry [Billions of dollars]",
            Self::ValueAddedByIndustryAsAPercentageOfGrossDomesticProduct => "U.Value added by Industry as a Percentage of Gross Domestic Product [Percent]",
        }
    }
}

impl Code<i64> for UgdpTable {
    type Decoded = Self;

    fn code(&self) -> i64 {
        match self {
            Self::ChainTypePriceIndexesForGrossOutputByIndustry => 225,
            Self::ChainTypePriceIndexesForIntermediateInputsByIndustry => 232,
            Self::ChainTypePriceIndexesForValueAddedByIndustry => 215,
            Self::ChainTypeQuantityIndexesForGrossOutputByIndustry => 221,
            Self::ChainTypeQuantityIndexesForIntermediateInputsByIndustry => 230,
            Self::ChainTypeQuantityIndexesForValueAddedByIndustry => 212,
            Self::ContributionsToPercentChangeInRealGrossDomesticProductByIndustry => 217,
            Self::ContributionsToPercentChangeInTheChainTypePriceIndexForGrossDomesticProductByIndustry => 218,
            Self::GrossOutputByIndustry => 219,
            Self::IntermediateInputsByIndustry => 229,
            Self::PercentChangesInChainTypePriceIndexesForGrossOutputByIndustry => 227,
            Self::PercentChangesInChainTypePriceIndexesForIntermediateInputsByIndustry => 233,
            Self::PercentChangesInChainTypePriceIndexesForValueAddedByIndustry => 216,
            Self::PercentChangesInChainTypeQuantityIndexesForGrossOutputByIndustry => 223,
            Self::PercentChangesInChainTypeQuantityIndexesForIntermediateInputsByIndustry => 231,
            Self::PercentChangesInChainTypeQuantityIndexesForValueAddedByIndustry => 213,
            Self::RealGrossOutputByIndustry => 234,
            Self::RealIntermediateInputsByIndustry => 236,
            Self::RealValueAddedByIndustry => 214,
            Self::ValueAddedByIndustry => 210,
            Self::ValueAddedByIndustryAsAPercentageOfGrossDomesticProduct => 211,
        }
    }

    fn from_code(code: i64) -> Option<Self> {
        let table = match code {
            210 => Self::ValueAddedByIndustry,
            211 => Self::ValueAddedByIndustryAsAPercentageOfGrossDomesticProduct,
            212 => Self::ChainTypeQuantityIndexesForValueAddedByIndustry,
            213 => Self::PercentChangesInChainTypeQuantityIndexesForValueAddedByIndustry,
            214 => Self::RealValueAddedByIndustry,
            215 => Self::ChainTypePriceIndexesForValueAddedByIndustry,
            216 => Self::PercentChangesInChainTypePriceIndexesForValueAddedByIndustry,
            217 => Self::ContributionsToPercentChangeInRealGrossDomesticProductByIndustry,
            218 => Self::ContributionsToPercentChangeInTheChainTypePriceIndexForGrossDomesticProductByIndustry,
            219 => Self::GrossOutputByIndustry,
            221 => Self::ChainTypeQuantityIndexesForGrossOutputByIndustry,
            223 => Self::PercentChangesInChainTypeQuantityIndexesForGrossOutputByIndustry,
            225 => Self::ChainTypePriceIndexesForGrossOutputByIndustry,
            227 => Self::PercentChangesInChainTypePriceIndexesForGrossOutputByIndustry,
            229 => Self::IntermediateInputsByIndustry,
            230 => Self::ChainTypeQuantityIndexesForIntermediateInputsByIndustry,
            231 => Self::PercentChangesInChainTypeQuantityIndexesForIntermediateInputsByIndustry,
            232 => Self::ChainTypePriceIndexesForIntermediateInputsByIndustry,
            233 => Self::PercentChangesInChainTypePriceIndexesForIntermediateInputsByIndustry,
            234 => Self::RealGrossOutputByIndustry,
            236 => Self::RealIntermediateInputsByIndustry,
            _ => return None,
        };
        Some(table)
    }
}
