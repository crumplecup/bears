/// Trade direction for international services and trade data
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
pub enum TradeDirection {
    /// Balance
    Balance,
    /// Exports,
    Exports,
    /// Imports,
    Imports,
    /// Supplemental detail on insurance transactions
    SupplementalIns,
}

impl TradeDirection {
    /// Returns the description of the trade direction.
    pub fn description(&self) -> &'static str {
        match self {
            Self::Balance => "Balance",
            Self::Exports => "Exports",
            Self::Imports => "Imports",
            Self::SupplementalIns => "Supplemental detail on insurance transactions",
        }
    }
}
