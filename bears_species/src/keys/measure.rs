/// The `Measure` enum represents variants of the *Classification Unit* in the **FixedAssets** and
/// **IIP** datasets.
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
    serde::Serialize,
    serde::Deserialize,
    derive_more::Display,
    derive_more::FromStr,
    strum::EnumIter,
)]
pub enum Measure {
    #[default]
    /// Proportional data.
    Level,
    /// US dollars.
    Usd,
}
