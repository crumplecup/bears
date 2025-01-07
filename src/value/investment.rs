use crate::{
    parameter_value::MneDoi, BeaErr, DeriveFromStr, ParameterValueTable, ParameterValueTableVariant,
};
use convert_case::Casing;
use derive_more::FromStr;

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
    strum::EnumIter,
    derive_more::FromStr,
)]
pub enum InvestmentKind {
    #[default]
    Inward,
    Outward,
    Parent,
    State,
}

impl TryFrom<&MneDoi> for InvestmentKind {
    type Error = DeriveFromStr;
    fn try_from(value: &MneDoi) -> Result<Self, Self::Error> {
        let key = value.key().to_case(convert_case::Case::Title);
        match Self::from_str(&key) {
            Ok(kind) => Ok(kind),
            Err(source) => {
                let error = DeriveFromStr::new(key, source, line!(), file!().to_string());
                Err(error)
            }
        }
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
    derive_new::new,
    derive_getters::Getters,
)]
pub struct DirectionOfInvestment {
    description: String,
    key: String,
    kind: InvestmentKind,
}

impl TryFrom<&MneDoi> for DirectionOfInvestment {
    type Error = DeriveFromStr;
    fn try_from(value: &MneDoi) -> Result<Self, Self::Error> {
        let description = value.desc().to_string();
        let key = value.key().to_string();
        let kind = InvestmentKind::try_from(value)?;
        Ok(Self::new(description, key, kind))
    }
}

impl TryFrom<&ParameterValueTable> for DirectionOfInvestment {
    type Error = BeaErr;
    fn try_from(value: &ParameterValueTable) -> Result<Self, Self::Error> {
        match value {
            ParameterValueTable::MneDoi(tab) => Ok(Self::try_from(tab)?),
            _ => {
                let error = ParameterValueTableVariant::new(
                    "MneDoi needed".to_string(),
                    line!(),
                    file!().to_string(),
                );
                Err(error.into())
            }
        }
    }
}
