use crate::{
    BeaErr, DeriveFromStr, JsonParseError, KeyMissing, NipaFrequency, ParameterName,
    ParameterValueTable, ParameterValueTableVariant,
};
use std::str::FromStr;

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
)]
pub enum Frequency {
    #[default]
    Annual,
    Monthly,
    Quarterly,
}

impl Frequency {
    /// Canonical method for converting the given variant into a BEA parameter value for use in a
    /// request.
    pub fn value(&self) -> String {
        let s = match self {
            Self::Annual => "A",
            Self::Monthly => "M",
            Self::Quarterly => "Q",
        };
        s.to_string()
    }

    /// Canonical method for parsing from a BEA parameter value into a variant of `Self`.
    pub fn from_value(value: &str) -> Result<Self, JsonParseError> {
        let frequency = match value {
            "A" => Self::Annual,
            "M" => Self::Monthly,
            "Q" => Self::Quarterly,
            _ => {
                let error = KeyMissing::new(value.to_owned(), line!(), file!().to_owned());
                // Use a JsonParseError until from KeyMissing is impled directly for BeaErr.
                return Err(error.into());
            }
        };
        Ok(frequency)
    }

    /// Formats the given variant into a key:value pair for use in a BEA request.
    pub fn params(&self) -> (String, String) {
        let key = ParameterName::Frequency.to_string();
        let value = self.value();
        (key, value)
    }
}

impl TryFrom<&NipaFrequency> for Frequency {
    type Error = DeriveFromStr;
    fn try_from(value: &NipaFrequency) -> Result<Self, Self::Error> {
        let freq = match Self::from_str(value.description()) {
            Ok(value) => value,
            Err(source) => {
                let error = DeriveFromStr::new(
                    value.description().to_string(),
                    source,
                    line!(),
                    file!().to_string(),
                );
                return Err(error);
            }
        };
        Ok(freq)
    }
}

impl TryFrom<&ParameterValueTable> for Frequency {
    type Error = BeaErr;
    fn try_from(value: &ParameterValueTable) -> Result<Self, Self::Error> {
        match value {
            ParameterValueTable::NipaFrequency(freq) => Ok(Self::try_from(freq)?),
            _ => {
                let error = ParameterValueTableVariant::new(
                    "NipaFrequency needed".to_string(),
                    line!(),
                    file!().to_string(),
                );
                Err(error.into())
            }
        }
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Deserialize,
    serde::Serialize,
    derive_new::new,
    derive_more::Deref,
    derive_more::DerefMut,
    derive_more::AsRef,
    derive_more::AsMut,
    derive_more::From,
)]
#[from(Vec<Frequency>)]
pub struct Frequencies(Vec<Frequency>);

impl Frequencies {
    pub fn value(&self) -> String {
        let v = self.iter().map(|v| v.value()).collect::<Vec<String>>();
        let mut result = String::new();
        let ln = v.len();
        for (index, item) in v.iter().enumerate() {
            result.push_str(item);
            if index < ln - 1 {
                result.push(',');
            }
        }
        result
    }

    pub fn params(&self) -> (String, String) {
        let key = "Frequency".to_string();
        let value = self.value();
        (key, value)
    }
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
    serde::Deserialize,
    serde::Serialize,
)]
pub enum FrequencyOptions {
    #[default]
    All,
    Individual,
}
