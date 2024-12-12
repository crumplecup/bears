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
pub struct GeoFips {
    value: i32,
    description: String,
}

impl TryFrom<&ParameterFields> for GeoFips {
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

impl TryFrom<&ParameterValueTable> for GeoFips {
    type Error = BeaErr;
    fn try_from(value: &ParameterValueTable) -> Result<Self, Self::Error> {
        match value {
            ParameterValueTable::ParameterFields(pf) => Ok(GeoFips::try_from(pf)?),
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
pub struct GeoFipsData(Vec<GeoFips>);

impl TryFrom<&ParameterValues> for GeoFipsData {
    type Error = BeaErr;
    fn try_from(value: &ParameterValues) -> Result<Self, Self::Error> {
        let mut geo = Vec::new();
        for table in value.iter() {
            let fips = GeoFips::try_from(table)?;
            geo.push(fips);
        }
        Ok(Self::new(geo))
    }
}
