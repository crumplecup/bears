use crate::{
    json_str, map_to_bool, map_to_string, BeaParameterValues, BincodeError, FromStrError,
    JsonParseError, JsonParseErrorKind, NotParameterName, RequestParameters, ReqwestError, User,
};
use derive_more::FromStr;
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
    parameter_name: ParameterName,
    #[serde(
        deserialize_with = "deserialize_bool",
        serialize_with = "serialize_bool"
    )]
    parameter_is_required_flag: bool,
}

impl Parameter {
    #[tracing::instrument(skip_all)]
    pub async fn values(
        &self,
        user: &User,
        dataset: &str,
    ) -> Result<BeaParameterValues, ReqwestError> {
        let mut body = user.body();
        body.push_str("&method=GETPARAMETERVALUES");
        body.push_str(&format!("&datasetname={}", dataset));
        body.push_str(&format!("&ParameterName={}", self.parameter_name));
        let url = body.clone();
        let client = reqwest::Client::new();
        match client.get(body.clone()).send().await {
            Ok(res) => match res.text().await {
                Ok(text) => tracing::info!("Response: {text}"),
                Err(source) => {
                    let error = ReqwestError::new(url, "get".to_string(), source);
                    return Err(error);
                }
            },
            Err(source) => {
                let error = ReqwestError::new(url, "get".to_string(), source);
                return Err(error);
            }
        }
        match client.get(body).send().await {
            Ok(res) => {
                info!("Response code: {}.", res.status());
                match res.json::<BeaParameterValues>().await {
                    Ok(data) => Ok(data),
                    Err(source) => {
                        let error = ReqwestError::new(url, "get".to_string(), source);
                        return Err(error);
                    }
                }
            }
            Err(source) => {
                let error = ReqwestError::new(url, "get".to_string(), source);
                return Err(error);
            }
        }
    }

    #[tracing::instrument(skip_all)]
    pub fn name(&self) -> String {
        self.parameter_name.to_string()
    }

    #[tracing::instrument(skip_all)]
    pub fn read_json(
        m: &serde_json::Map<String, serde_json::Value>,
    ) -> Result<Self, JsonParseError> {
        let all_value = match map_to_string("AllValue", m) {
            Ok(value) => Some(value),
            Err(_) => None,
        };
        let multiple_accepted_flag = map_to_bool("MultipleAcceptedFlag", m)?;
        let parameter_data_type = map_to_string("ParameterDataType", m)?;
        let parameter_default_value = if let Some(value) = m.get("ParameterDefaultValue") {
            let v = json_str(value)?;
            Some(v)
        } else {
            None
        };
        let parameter_description = map_to_string("ParameterDescription", m)?;
        let parameter_name = ParameterName::from_map("ParameterName", m)?;
        let parameter_is_required_flag = map_to_bool("ParameterIsRequiredFlag", m)?;
        Ok(Self {
            all_value,
            multiple_accepted_flag,
            parameter_data_type,
            parameter_default_value,
            parameter_description,
            parameter_name,
            parameter_is_required_flag,
        })
    }
}

impl TryFrom<serde_json::Value> for Parameter {
    type Error = JsonParseError;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        tracing::info!("Reading Parameter.");
        match value {
            serde_json::Value::Object(m) => {
                let param = Self::read_json(&m)?;
                Ok(param)
            }
            _ => {
                tracing::warn!("Invalid Value: {value:#?}");
                let error = JsonParseErrorKind::NotObject;
                Err(error.into())
            }
        }
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
    derive_new::new,
    derive_more::Deref,
    derive_more::DerefMut,
)]
#[serde(rename_all = "PascalCase")]
pub struct Parameters {
    parameter: Vec<Parameter>,
}

impl TryFrom<&serde_json::Value> for Parameters {
    type Error = JsonParseError;
    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        tracing::info!("Reading Parameters");
        match value {
            serde_json::Value::Object(m) => {
                let key = "Parameter".to_string();
                if let Some(data) = m.get(&key) {
                    match data {
                        serde_json::Value::Array(v) => {
                            tracing::info!("Array found.");
                            let mut parameter = Vec::new();
                            for val in v {
                                let param = Parameter::try_from(val.clone())?;
                                parameter.push(param);
                            }
                            tracing::info!("Parameters found.");
                            let parameters = Self::new(parameter);
                            Ok(parameters)
                        }
                        serde_json::Value::Object(mp) => {
                            tracing::info!("Object found.");
                            let param = Parameter::read_json(mp)?;
                            let params = Parameters::new(vec![param]);
                            Ok(params)
                        }
                        _ => {
                            tracing::warn!("Unexpected content: {m:#?}");
                            let error = JsonParseErrorKind::NotArray;
                            Err(error.into())
                        }
                    }
                } else {
                    tracing::warn!("Parameter missing.");
                    let error = JsonParseErrorKind::KeyMissing(key);
                    Err(error.into())
                }
            }
            _ => {
                tracing::warn!("Wrong Value type: {value:#?}");
                let error = JsonParseErrorKind::NotObject;
                Err(error.into())
            }
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ParameterList {
    request: RequestParameters,
    results: Parameters,
}

impl ParameterList {
    #[tracing::instrument(skip_all)]
    pub fn read_json(
        m: &serde_json::Map<String, serde_json::Value>,
    ) -> Result<Self, JsonParseError> {
        let mut param = Self::default();
        let key = "Request".to_string();
        if let Some(value) = m.get(&key) {
            param.request = RequestParameters::try_from(value)?;
        } else {
            let error = JsonParseErrorKind::KeyMissing(key);
            return Err(error.into());
        }
        let key = "Results".to_string();
        if let Some(value) = m.get(&key) {
            param.results = Parameters::try_from(value)?;
        } else {
            let error = JsonParseErrorKind::KeyMissing(key);
            return Err(error.into());
        }
        Ok(param)
    }
}

impl TryFrom<&serde_json::Value> for ParameterList {
    type Error = JsonParseError;
    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        tracing::info!("Reading ParameterList.");
        match value {
            serde_json::Value::Object(m) => {
                let param = Self::read_json(m)?;
                Ok(param)
            }
            _ => {
                tracing::warn!("Invalid Value: {value:#?}");
                let error = JsonParseErrorKind::NotObject;
                Err(error.into())
            }
        }
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
    derive_new::new,
    derive_more::Deref,
    derive_more::DerefMut,
)]
#[serde(rename_all = "UPPERCASE")]
pub struct BeaParameters {
    beaapi: ParameterList,
}

impl BeaParameters {
    #[tracing::instrument(skip_all)]
    pub fn results(&self) -> Vec<Parameter> {
        self.beaapi.results.to_vec()
    }

    #[tracing::instrument(skip_all)]
    pub fn serialize(&self) -> Result<Vec<u8>, BincodeError> {
        match bincode::serialize(self) {
            Ok(data) => Ok(data),
            Err(source) => {
                let error = BincodeError::new("serializing BeaParameters".to_string(), source);
                Err(error)
            }
        }
    }

    #[tracing::instrument(skip_all)]
    pub fn deserialize(bytes: &[u8]) -> Result<Self, BincodeError> {
        match bincode::deserialize(bytes) {
            Ok(data) => Ok(data),
            Err(source) => {
                let error = BincodeError::new("deserializing BeaParameters".to_string(), source);
                Err(error)
            }
        }
    }
}

impl TryFrom<&serde_json::Value> for BeaParameters {
    type Error = JsonParseError;
    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        tracing::info!("Reading BeaParameters.");
        match value {
            serde_json::Value::Object(m) => {
                let key = "BEAAPI".to_string();
                if let Some(val) = m.get(&key) {
                    tracing::info!("Val is: {val:#?}");
                    let data = ParameterList::try_from(val)?;
                    let data = BeaParameters::new(data);
                    Ok(data)
                } else {
                    tracing::info!("Invalid Object: {m:#?}");
                    let error = JsonParseErrorKind::KeyMissing(key);
                    Err(error.into())
                }
            }
            _ => {
                tracing::info!("Invalid Value: {value:#?}");
                let error = JsonParseErrorKind::NotObject;
                Err(error.into())
            }
        }
    }
}

/// Custom handling of bool deserialization to match BEA.
#[tracing::instrument(skip_all)]
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
#[tracing::instrument(skip(serializer))]
pub fn serialize_bool<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::ser::Serializer,
{
    match *value {
        true => serializer.serialize_u8(1),
        false => serializer.serialize_u8(0),
    }
}

#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Display,
    derive_more::FromStr,
    strum::EnumIter,
)]
pub enum ParameterName {
    Affiliation,
    AreaOrCountry,
    Channel,
    Classification,
    Component,
    Country,
    Dataset,
    Destination,
    DirectionOfInvestment,
    Frequency,
    GeoFips,
    GetFootnotes,
    Indicator,
    Industry,
    Investment,
    LineCode,
    NonbankAffiliatesOnly,
    OwnershipLevel,
    ParentInvestment,
    SeriesID,
    ShowMillions,
    State,
    TableID,
    #[default]
    TableName,
    TradeDirection,
    TypeOfInvestment,
    TypeOfService,
    Year,
}

impl ParameterName {
    #[tracing::instrument]
    pub fn from_json(json: &str) -> Result<Self, JsonParseError> {
        let mut name = json.to_string();
        if json.starts_with('"') {
            match serde_json::from_str(json) {
                Ok(v) => name = v,
                Err(e) => {
                    let error = FromStrError::new(name, e);
                    let error: JsonParseErrorKind = error.into();
                    return Err(error.into());
                }
            }
        }
        name = name.trim().to_owned();
        if let Ok(p) = ParameterName::from_str(&name) {
            tracing::trace!("Name found: {p}");
            Ok(p)
        } else {
            tracing::warn!("Paramater Variant missing.");
            let error = NotParameterName::new(name);
            let error: JsonParseErrorKind = error.into();
            Err(error.into())
        }
    }

    #[tracing::instrument(skip_all)]
    pub fn from_map(
        key: &str,
        m: &serde_json::Map<String, serde_json::Value>,
    ) -> Result<Self, JsonParseError> {
        if let Some(value) = m.get(key) {
            let name = ParameterName::try_from(value)?;
            Ok(name)
        } else {
            let error = JsonParseErrorKind::KeyMissing(key.to_string());
            Err(error.into())
        }
    }
}

impl TryFrom<&serde_json::Value> for ParameterName {
    type Error = JsonParseError;
    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        match value {
            serde_json::Value::String(s) => {
                let name = ParameterName::from_json(s)?;
                Ok(name)
            }
            _ => {
                tracing::warn!("Unexpected Value variant.");
                let error = JsonParseErrorKind::NotString;
                Err(error.into())
            }
        }
    }
}
