use crate::{JsonParseError, JsonParseErrorKind};
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
pub struct RequestParameter {
    parameter_name: String,
    parameter_value: String,
}

impl TryFrom<serde_json::Value> for RequestParameter {
    type Error = JsonParseError;
    #[tracing::instrument(skip_all)]
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        tracing::info!("Reading RequestParameter");
        match value {
            serde_json::Value::Object(m) => {
                let key_name = "ParameterName".to_string();
                if let Some(name) = m.get(&key_name) {
                    let key_val = "ParameterValue".to_string();
                    if let Some(val) = m.get(&key_val) {
                        let details = RequestParameter::new(name.to_string(), val.to_string());
                        Ok(details)
                    } else {
                        tracing::info!("Invalid contents: {m:#?}");
                        let error = JsonParseErrorKind::KeyMissing(key_val);
                        Err(error.into())
                    }
                } else {
                    tracing::info!("Invalid Object: {m:#?}");
                    let error = JsonParseErrorKind::KeyMissing(key_name);
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
pub struct RequestParameters {
    request_param: Vec<RequestParameter>,
}

impl TryFrom<&serde_json::Value> for RequestParameters {
    type Error = JsonParseError;
    // #[tracing::instrument(skip_all)]
    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        tracing::info!("Reading RequestParameters");
        match value {
            serde_json::Value::Object(m) => {
                let key = "RequestParam".to_string();
                if let Some(serde_json::Value::Array(v)) = m.get(&key) {
                    let mut request_param = Vec::new();
                    for val in v {
                        let contents = RequestParameter::try_from(val.clone())?;
                        request_param.push(contents);
                    }
                    let parameters = RequestParameters::new(request_param);
                    Ok(parameters)
                } else {
                    tracing::info!("Unexpected content: {m:#?}");
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
