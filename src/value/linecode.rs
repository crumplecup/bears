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
#[display("{}", self.value)]
pub struct LineCode {
    value: i32,
    description: String,
}

impl TryFrom<ParameterFields> for LineCode {
    type Error = ParseInt;
    fn try_from(value: ParameterFields) -> Result<Self, Self::Error> {
        match value.key().parse::<i32>() {
            Ok(num) => Ok(Self::new(num, value.desc().into())),
            Err(source) => {
                let error = ParseInt::new(value.key().into(), source);
                Err(error)
            }
        }
    }
}
