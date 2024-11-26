use crate::{BeaError, BeaParameterValues, RequestParameters, User};
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Parameter {
    #[serde(skip_serializing_if = "Option::is_none")]
    all_value: Option<String>,
    #[serde(
        deserialize_with = "deserialize_bool",
        serialize_with = "serialize_bool"
    )]
    multiple_accepted_flag: bool,
    parameter_data_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter_default_value: Option<String>,
    parameter_description: String,
    parameter_name: String,
    #[serde(
        deserialize_with = "deserialize_bool",
        serialize_with = "serialize_bool"
    )]
    parameter_is_required_flag: bool,
}

impl Parameter {
    pub async fn values(&self, user: &User, dataset: &str) -> Result<BeaParameterValues, BeaError> {
        let mut body = user.body();
        body.push_str("&method=GETPARAMETERVALUES");
        body.push_str(&format!("&datasetname={}", dataset));
        body.push_str(&format!("&ParameterName={}", self.parameter_name));
        let client = reqwest::Client::new();
        let res = client.get(body.clone()).send().await?;
        info!("Response: {}", res.text().await?);
        let res = client.get(body).send().await?;
        info!("Response code: {}.", res.status());
        let data = res.json::<BeaParameterValues>().await?;
        Ok(data)
    }

    pub fn name(&self) -> String {
        self.parameter_name.to_owned()
    }
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Deserialize,
    Serialize,
    derive_more::Deref,
    derive_more::DerefMut,
)]
#[serde(rename_all = "PascalCase")]
pub struct Parameters {
    parameter: Vec<Parameter>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ParameterList {
    request: RequestParameters,
    results: Parameters,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Deserialize,
    Serialize,
    derive_more::Deref,
    derive_more::DerefMut,
)]
#[serde(rename_all = "UPPERCASE")]
pub struct BeaParameters {
    beaapi: ParameterList,
}

impl BeaParameters {
    pub fn results(&self) -> Vec<Parameter> {
        self.beaapi.results.to_vec()
    }
}

/// Custom handling of bool deserialization to match BEA.
pub fn deserialize_bool<'de, D: Deserializer<'de>>(de: D) -> Result<bool, D::Error> {
    let intermediate = Deserialize::deserialize(de)?;
    match intermediate {
        "1" => Ok(true),
        "true" => Ok(true),
        "false" => Ok(false),
        "0" => Ok(false),
        _ => Ok(false),
    }
}

/// Custom handling of bool serialization to match BEA.
pub fn serialize_bool<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::ser::Serializer,
{
    match *value {
        true => serializer.serialize_u8(1),
        false => serializer.serialize_u8(0),
    }
}
