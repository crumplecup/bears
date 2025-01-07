use crate::{
    BeaErr, DeriveFromStr, NipaFrequency, ParameterValueTable, ParameterValueTableVariant,
};
use derive_more::FromStr;

#[derive(
    Debug,
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
    derive_more::FromStr,
)]
pub enum Frequency {
    Annual,
    Monthly,
    Quarterly,
}

impl Frequency {
    pub fn param(&self) -> String {
        let s = match self {
            Self::Annual => "A",
            Self::Monthly => "M",
            Self::Quarterly => "Q",
        };
        s.to_string()
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
    derive_more::Deref,
    derive_more::DerefMut,
    derive_more::AsRef,
    derive_more::AsMut,
    derive_more::From,
)]
#[from(Vec<Frequency>)]
pub struct Frequencies(Vec<Frequency>);

impl Frequencies {
    pub fn params(&self) -> Vec<String> {
        self.iter().map(|v| v.param()).collect()
    }
}
