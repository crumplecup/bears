/// Channels for service trade and affiliate activity
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
pub enum Channel {
    /// All channels
    AllChannels,
    /// Majority-owned foreign affiliates of U.S. MNEs
    Mofas,
    /// Majority-owned U.S. affiliates of foreign MNEs
    Mousas,
    /// Sales of services by majority-owned foreign affiliates of U.S. MNEs
    SalesMofas,
    /// Sales of services by majority-owned U.S. affiliates of foreign MNEs
    SalesMousas,
    /// Trade (adjusted)
    Trade,
    /// U.S. exports by majority-owned U.S. affiliates of foreign MNEs
    UsExportsByMousas,
    /// U.S. imports from majority-owned foreign affiliates of U.S. MNEs
    UsImportsFromMofas,
}

impl Channel {
    /// Returns the description of the channel
    pub fn description(&self) -> &'static str {
        match self {
            Self::AllChannels => "All channels",
            Self::Mofas => "Majority-owned foreign affiliates of U.S. MNEs",
            Self::Mousas => "Majority-owned U.S. affiliates of foreign MNEs",
            Self::SalesMofas => {
                "Sales of services by majority-owned foreign affiliates of U.S. MNEs"
            }
            Self::SalesMousas => {
                "Sales of services by majority-owned U.S. affiliates of foreign MNEs"
            }
            Self::Trade => "Trade (adjusted)",
            Self::UsExportsByMousas => {
                "U.S. exports by majority-owned U.S. affiliates of foreign MNEs"
            }
            Self::UsImportsFromMofas => {
                "U.S. imports from majority-owned foreign affiliates of U.S. MNEs"
            }
        }
    }
}
