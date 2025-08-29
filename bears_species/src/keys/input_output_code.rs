use crate::{KeyMissing, Naics};

/// Parent enum for types of InputOutput row and column codes.
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
    derive_more::From,
    derive_more::Deref,
    derive_more::DerefMut,
)]
#[from(Naics)]
pub struct InputOutputCode(Naics);

impl InputOutputCode {
    pub fn from_value(value: &str) -> Result<Self, KeyMissing> {
        match Naics::from_code(value) {
            Some(naics) => Ok(naics.into()),
            None => {
                let error = KeyMissing::new(value.to_owned(), line!(), file!().to_owned());
                Err(error)
            }
        }
    }
}
