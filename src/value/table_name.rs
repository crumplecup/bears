use crate::ParameterFields;

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
#[display("{}", self.name)]
pub struct TableName {
    name: String,
    description: String,
}

impl From<ParameterFields> for TableName {
    fn from(value: ParameterFields) -> Self {
        Self::new(value.key().into(), value.desc().into())
    }
}
