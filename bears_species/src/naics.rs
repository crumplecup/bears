use crate::{from_csv, IoError};

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_getters::Getters,
)]
pub struct NaicsItem {
    #[serde(rename = "naics_code")]
    code: i64,
    #[serde(rename = "naics_title")]
    title: String,
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_more::From,
    derive_more::Deref,
    derive_more::DerefMut,
)]
#[from(Vec<NaicsItem>)]
pub struct Naics(Vec<NaicsItem>);

impl Naics {
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, IoError> {
        let naics = from_csv(path)?;
        Ok(Self::from(naics))
    }
}
