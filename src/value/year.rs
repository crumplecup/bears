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
#[display("{}", self.date.year())]
pub struct Year {
    date: jiff::civil::Date,
    description: String,
}

impl TryFrom<&ParameterFields> for Year {
    type Error = ParseInt;
    fn try_from(value: &ParameterFields) -> Result<Self, Self::Error> {
        let num = match value.key().parse::<i16>() {
            Ok(num) => num,
            Err(source) => {
                let error = ParseInt::new(value.key().into(), source);
                return Err(error);
            }
        };
        let date = jiff::civil::date(num, 0, 0);
        Ok(Self::new(date, value.desc().into()))
    }
}

impl TryFrom<&ParameterValueTable> for Year {
    type Error = BeaErr;
    fn try_from(value: &ParameterValueTable) -> Result<Self, Self::Error> {
        match value {
            ParameterValueTable::ParameterFields(pf) => Ok(Self::try_from(pf)?),
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
pub struct YearData(Vec<Year>);

impl TryFrom<&ParameterValues> for YearData {
    type Error = BeaErr;
    fn try_from(value: &ParameterValues) -> Result<Self, Self::Error> {
        let mut results = Vec::new();
        for table in value.iter() {
            let item = Year::try_from(table)?;
            results.push(item);
        }
        Ok(Self::new(results))
    }
}
