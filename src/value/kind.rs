use crate::{
    BeaErr, BeaResponse, GeoFips, JsonParseError, JsonParseErrorKind, LineCode, ParameterName,
    ParameterValueTable, ParameterValueTableVariant, ParameterValues, Results, TableName, Year,
};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    derive_more::Display,
    derive_more::From,
    derive_new::new,
    serde::Deserialize,
    serde::Serialize,
    strum::EnumIter,
)]
pub enum Value {
    #[from(GeoFips)]
    GeoFips(GeoFips),
    #[from(LineCode)]
    LineCode(LineCode),
    #[from(TableName)]
    TableName(TableName),
    #[from(Year)]
    Year(Year),
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
    derive_more::From,
    derive_new::new,
    serde::Deserialize,
    serde::Serialize,
)]
#[from(&[Value])]
pub struct Values(Vec<Value>);

// impl TryFrom<&BeaResponse> for Values {
//     type Error = BeaErr;
//     fn try_from(value: &BeaResponse) -> Result<Self, Self::Error> {
//         let (request, results) = value.into_parts();
//         match request.name()? {
//             ParameterName::GeoFips => match results {
//                 Results::ParameterValues(pv) => {
//                     let mut fips = Vec::new();
//                     for v in pv.iter() {
//                         match *v {
//                             ParameterValueTable::ParameterFields(pf) => {
//                                 let geo = GeoFips::try_from(pf)?;
//                                 let geo = Value::from(geo);
//                                 fips.push(geo);
//                             }
//                             other => {
//                                 tracing::warn!("Unexpected ParameterValueTable variant.");
//                                 let error = ParameterValueTableVariant::new(
//                                     "expected ParamterFields variant".to_string(),
//                                 );
//                                 return Err(error.into());
//                             }
//                         }
//                     }
//                     Ok(Self::new(fips))
//                 }
//                 _ => {
//                     tracing::warn!("Unexpected Results variant.");
//                 }
//             },
//             ParameterName::LineCode => {}
//             ParameterName::TableName => {}
//             ParameterName::Year => {}
//             other => {
//                 tracing::warn!("Unexpected parameter name {other}.");
//                 let error = JsonParseErrorKind::KeyMissing(format!("invalid key {other}"));
//                 let error = JsonParseError::from(error);
//                 Err(error.into())
//             }
//         }
//     }
// }
