#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Deserialize,
    serde::Serialize,
    strum::EnumIter,
    derive_more::Display,
    derive_more::FromStr,
)]
/// Represents different types of components used for position changes in financial accounts.
pub enum Component {
    /// Change in position
    #[default]
    ChgPos,
    /// Change in position attributable to changes in volume and valuation n.i.e.
    ChgPosNie,
    /// Change in position not attributable to financial-account transactions
    ChgPosOth,
    /// Change in position attributable to price changes
    ChgPosPrice,
    /// Change in position attributable to financial-account transactions
    ChgPosTrans,
    /// Change in position attributable to exchange-rate changes
    ChgPosXRate,
    /// Position
    Pos,
}

impl Component {
    /// Returns the description of the component as a static string.
    pub fn description(&self) -> &'static str {
        match self {
            Component::ChgPos => "Change in position",
            Component::ChgPosNie => {
                "Change in position attributable to changes in volume and valuation n.i.e."
            }
            Component::ChgPosOth => {
                "Change in position not attributable to financial-account transactions"
            }
            Component::ChgPosPrice => "Change in position attributable to price changes",
            Component::ChgPosTrans => {
                "Change in position attributable to financial-account transactions"
            }
            Component::ChgPosXRate => "Change in position attributable to exchange-rate changes",
            Component::Pos => "Position",
        }
    }
}
