use crate::{ParameterFields, ParseInt};

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

impl TryFrom<ParameterFields> for Year {
    type Error = ParseInt;
    fn try_from(value: ParameterFields) -> Result<Self, Self::Error> {
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
