/// Affiliation for international services and trade data
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
pub enum Affiliation {
    /// Affiliated trade
    Affiliated,
    /// All affiliations
    AllAffiliations,
    /// Unaffiliated trade
    Unaffiliated,
    /// U.S. affiliates' trade with their foreign parent groups
    UsAffiliates,
    /// U.S. parents' trade with their foreign affiliates
    UsParents,
}

impl Affiliation {
    /// Returns the description of the affiliation.
    pub fn description(&self) -> &'static str {
        match self {
            Self::Affiliated => "Affiliated trade",
            Self::AllAffiliations => "All affiliations",
            Self::Unaffiliated => "Unaffiliated trade",
            Self::UsAffiliates => "U.S. affiliates' trade with their foreign parent groups",
            Self::UsParents => "U.S. parents' trade with their foreign affiliates",
        }
    }
}
