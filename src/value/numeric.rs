use crate::{BeaErr, ParameterFields, ParameterValueTable, ParameterValueTableVariant, ParseInt};

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    derive_more::Display,
    derive_new::new,
    derive_getters::Getters,
    serde::Deserialize,
    serde::Serialize,
)]
#[display("{}", self.value)]
pub struct Integer {
    value: i32,
    description: String,
}

impl TryFrom<&ParameterFields> for Integer {
    type Error = ParseInt;
    fn try_from(value: &ParameterFields) -> Result<Self, Self::Error> {
        match value.key().parse::<i32>() {
            Ok(num) => Ok(Self::new(num, value.desc().into())),
            Err(source) => {
                let error = ParseInt::new(value.key().into(), source);
                Err(error)
            }
        }
    }
}

impl TryFrom<&ParameterValueTable> for Integer {
    type Error = BeaErr;
    fn try_from(value: &ParameterValueTable) -> Result<Self, Self::Error> {
        match value {
            ParameterValueTable::ParameterFields(pf) => Ok(Integer::try_from(pf)?),
            _ => {
                let error = ParameterValueTableVariant::new("ParameterFields needed".to_string());
                Err(error.into())
            }
        }
    }
}
