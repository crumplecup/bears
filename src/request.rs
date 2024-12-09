use crate::{map_to_string, JsonParseError, JsonParseErrorKind};
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
        tracing::trace!("Reading RequestParameter");
        match value {
            serde_json::Value::Object(m) => {
                let parameter_name = map_to_string("ParameterName", &m)?;
                let parameter_value = map_to_string("ParameterValue", &m)?;
                Ok(Self::new(parameter_name, parameter_value))
            }
            _ => {
                tracing::trace!("Invalid Value: {value:#?}");
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
        tracing::trace!("Reading RequestParameters");
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
                    tracing::trace!("Unexpected content: {m:#?}");
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
