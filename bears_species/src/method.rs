use convert_case::Casing;

/// The variants of the `Method` enum represent different valid inputs for the `METHOD` parameter
/// when calling into the BEA REST API.
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
    strum::EnumIter,
    derive_more::Display,
    derive_more::FromStr,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum Method {
    /// Primary data retrieval method.
    #[default]
    GetData,
    /// Available BEA datasets, corresponding to variants of [`Dataset`](crate::Dataset).
    GetDataSetList,
    /// Valid [`ParameterName`](crate::ParameterName) values for a given
    /// [`Dataset`](crate::Dataset).
    GetParameterList,
    GetParameterValues,
    GetParameterValuesFiltered,
}

impl Method {
    /// The `upper` method converts the variant name to `UPPERCASE` case using
    /// [`convert_case::Case::UpperFlat`].
    /// Conforms to the user manual for the BEA API.
    #[tracing::instrument(skip_all)]
    pub fn upper(&self) -> String {
        self.to_string().to_case(convert_case::Case::UpperFlat)
    }
}
