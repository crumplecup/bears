use crate::{
    map_to_string, BeaErr, BeaErrorKind, DeriveFromStr, JsonParseError, JsonParseErrorKind,
    ParameterName,
};
use derive_more::FromStr;
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

impl RequestParameter {
    /// Indicates `true` if the `parameter_value` field contains the [`ParameterName`] `name`.
    /// Used when reading a response to determine the specific type of data structure corresponding
    /// to the variant of `name`.
    pub fn contains_name(&self, name: ParameterName) -> bool {
        if self.parameter_name == "PARAMETERNAME" || self.parameter_name == "TARGETPARAMETER" {
            if let Ok(value) = ParameterName::from_str(&self.parameter_value) {
                if value == name {
                    true
                } else {
                    // value is a param name but does not match
                    false
                }
            } else {
                // value is not a param name
                false
            }
        } else {
            // not a parameter name
            // this checks the name field for the type "PARAMETERNAME"
            // to screen out invalid values before we run from_str
            false
        }
    }

    pub fn name(&self) -> Result<ParameterName, BeaErr> {
        let key_1 = "PARAMETERNAME".to_string();
        let key_2 = "TARGETPARAMETER".to_string();
        if self.parameter_name == key_1 || self.parameter_name == key_2 {
            match ParameterName::from_str(&self.parameter_value) {
                Ok(name) => Ok(name),
                Err(source) => {
                    let error = DeriveFromStr::new(self.parameter_value.clone(), source);
                    Err(error.into())
                }
            }
        } else {
            let error = JsonParseErrorKind::KeyMissing(format!("neither {key_1} nor {key_2}"));
            let error = JsonParseError::from(error);
            Err(error.into())
        }
    }
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

impl RequestParameters {
    pub fn contains_name(&self, name: ParameterName) -> bool {
        let mut contains = false;
        for item in self.iter() {
            contains |= item.contains_name(name);
        }
        contains
    }

    pub fn name(&self) -> Result<ParameterName, BeaErr> {
        let mut names = Vec::new();
        let mut errs = Vec::new();
        for req in self.iter() {
            match req.name() {
                Ok(name) => names.push(name),
                Err(source) => errs.push(source),
            }
        }
        if !names.is_empty() {
            Ok(names[0])
        } else {
            tracing::warn!("Failed to locate parameter name in request.");
            match &**errs[0] {
                BeaErrorKind::DeriveFromStr(x) => Err(BeaErr::from(x.clone())),
                BeaErrorKind::JsonParse(kind) => match &**kind {
                    JsonParseErrorKind::KeyMissing(key) => {
                        let error = JsonParseErrorKind::KeyMissing(key.clone());
                        let error = JsonParseError::from(error);
                        Err(error.into())
                    }
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }
        }
    }
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
