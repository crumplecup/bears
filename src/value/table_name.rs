use crate::{
    BeaErr, ParameterFields, ParameterValueTable, ParameterValueTableVariant, ParameterValues,
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
#[display("{}", self.name)]
pub struct TableName {
    name: String,
    description: String,
}

impl From<&ParameterFields> for TableName {
    fn from(value: &ParameterFields) -> Self {
        Self::new(value.key().into(), value.desc().into())
    }
}

impl TryFrom<&ParameterValueTable> for TableName {
    type Error = BeaErr;
    fn try_from(value: &ParameterValueTable) -> Result<Self, Self::Error> {
        match value {
            ParameterValueTable::ParameterFields(pf) => Ok(TableName::from(pf)),
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
pub struct TableNameData(Vec<TableName>);

impl TryFrom<&ParameterValues> for TableNameData {
    type Error = BeaErr;
    fn try_from(value: &ParameterValues) -> Result<Self, Self::Error> {
        let mut results = Vec::new();
        for table in value.iter() {
            let item = TableName::try_from(table)?;
            results.push(item);
        }
        Ok(Self::new(results))
    }
}
