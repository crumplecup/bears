use crate::{
    parameter_value::ParameterValueTable, BeaErr, GeoFips, LineCode, ParameterValues, TableName,
    Year,
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

// impl TryFrom<&ParameterValues> for Values {
//     type Error = BeaErr;
//     fn try_from(value: &ParameterValues) -> Result<Self, Self::Error> {
//         let mut values = Vec::new();
//         for param in value.iter() {
//             match param {
//                 ParameterValueTable::ParameterFields(fields) => {
//             let v = Value::try_from(param)?;
//             values.push(v);
//
//                 }
//                 _ => {
//                     let error =
//                 }
//             }
//         }
//         Ok(values)
//     }
// }
