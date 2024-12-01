use crate::{map_to_string, JsonParseError, JsonParseErrorKind, RequestParameters};
use serde::{Deserialize, Serialize};

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
)]
#[serde(rename_all = "PascalCase")]
pub struct ParameterFields {
    desc: String,
    key: String,
}

impl ParameterFields {
    #[tracing::instrument(skip_all)]
    pub fn read_json(
        m: &serde_json::Map<String, serde_json::Value>,
    ) -> Result<Self, JsonParseError> {
        let desc = map_to_string("Desc", m)?;
        let key = map_to_string("Key", m)?;
        let param = ParameterFields::new(desc, key);
        Ok(param)
    }
}

impl TryFrom<serde_json::Value> for ParameterFields {
    type Error = JsonParseError;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        tracing::info!("Reading ParameterFields.");
        match value {
            serde_json::Value::Object(m) => {
                let data = Self::read_json(&m)?;
                Ok(data)
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
pub struct ParameterValues(Vec<ParameterFields>);

impl TryFrom<&serde_json::Value> for ParameterValues {
    type Error = JsonParseError;
    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        tracing::info!("Reading ParameterValues");
        match value {
            serde_json::Value::Object(m) => {
                let key = "ParamValue".to_string();
                if let Some(data) = m.get(&key) {
                    match data {
                        serde_json::Value::Array(v) => {
                            tracing::info!("Array found.");
                            let mut parameter = Vec::new();
                            for val in v {
                                let param = ParameterFields::try_from(val.clone())?;
                                parameter.push(param);
                            }
                            tracing::info!("ParameterFields found.");
                            let parameters = Self::new(parameter);
                            Ok(parameters)
                        }
                        _ => {
                            tracing::warn!("Unexpected content: {m:#?}");
                            let error = JsonParseErrorKind::NotArray;
                            Err(error.into())
                        }
                    }
                } else {
                    tracing::warn!("ParameterFields missing.");
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
)]
#[serde(rename_all = "PascalCase")]
pub struct BeaParameterValues {
    request: RequestParameters,
    results: ParameterValues,
}

impl BeaParameterValues {
    #[tracing::instrument(skip_all)]
    pub fn read_json(
        m: &serde_json::Map<String, serde_json::Value>,
    ) -> Result<Self, JsonParseError> {
        let key = "Request".to_string();
        let request = if let Some(value) = m.get(&key) {
            RequestParameters::try_from(value)?
        } else {
            let error = JsonParseErrorKind::KeyMissing(key);
            return Err(error.into());
        };
        let key = "Results".to_string();
        let results = if let Some(value) = m.get(&key) {
            ParameterValues::try_from(value)?
        } else {
            let error = JsonParseErrorKind::KeyMissing(key);
            return Err(error.into());
        };
        let param = Self::new(request, results);
        Ok(param)
    }
}

impl TryFrom<&serde_json::Value> for BeaParameterValues {
    type Error = JsonParseError;
    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        tracing::info!("Reading BeaParameterValues.");
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
