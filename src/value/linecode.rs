use crate::{
    BeaErr, ParameterFields, ParameterValueTable, ParameterValueTableVariant, ParameterValues,
    ParseInt,
};

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
    serde::Deserialize,
    serde::Serialize,
)]
#[display("{}", self.value)]
pub struct LineCode {
    value: i32,
    description: String,
}

impl TryFrom<&ParameterFields> for LineCode {
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

impl TryFrom<&ParameterValueTable> for LineCode {
    type Error = BeaErr;
    fn try_from(value: &ParameterValueTable) -> Result<Self, Self::Error> {
        match value {
            ParameterValueTable::ParameterFields(pf) => Ok(LineCode::try_from(pf)?),
            _ => {
                let error = ParameterValueTableVariant::new("ParameterFields needed".to_string());
                Err(error.into())
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
    derive_more::Deref,
    derive_more::DerefMut,
    derive_new::new,
    serde::Deserialize,
    serde::Serialize,
)]
pub struct LineCodeData(Vec<LineCode>);

impl TryFrom<&ParameterValues> for LineCodeData {
    type Error = BeaErr;
    fn try_from(value: &ParameterValues) -> Result<Self, Self::Error> {
        let mut results = Vec::new();
        for table in value.iter() {
            let item = LineCode::try_from(table)?;
            results.push(item);
        }
        Ok(Self::new(results))
    }
}
