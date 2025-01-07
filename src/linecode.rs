use crate::{Config, RequestParameters, ReqwestError};
use serde::{Deserialize, Serialize};
use tracing::info;

#[tracing::instrument(skip_all)]
pub async fn get_line_codes(config: &Config) -> Result<BeaLineCodes, ReqwestError> {
    let mut body = config.body();
    body.push_str("&method=GetParameterValuesFiltered");
    body.push_str("&TargetParameter=LineCode");
    let url = body.clone();
    let client = reqwest::Client::new();
    match client.get(body).send().await {
        Ok(res) => match res.json::<BeaLineCodes>().await {
            Ok(data) => Ok(data),
            Err(source) => {
                let error =
                    ReqwestError::new(url, "get".to_string(), source, line!(), file!().to_string());
                Err(error)
            }
        },
        Err(source) => {
            let error =
                ReqwestError::new(url, "get".to_string(), source, line!(), file!().to_string());
            Err(error)
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LineCode {
    key: String,
    desc: String,
}

impl LineCode {
    #[tracing::instrument(skip_all)]
    pub fn report(&self) {
        info!("Key: {}, Desc: {}", self.key, self.desc);
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
pub struct LineCodes {
    param_value: Vec<LineCode>,
}

impl LineCodes {
    #[tracing::instrument(skip_all)]
    pub fn report(&self) {
        self.iter().map(|c| c.report()).for_each(drop);
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LineCodeResults {
    request: RequestParameters,
    results: LineCodes,
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
pub struct BeaLineCodes {
    beaapi: LineCodeResults,
}

impl BeaLineCodes {
    #[tracing::instrument(skip_all)]
    pub fn results(&self) -> LineCodes {
        self.beaapi.results.clone()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct LineCodeTask {
    key: String,
    processed: bool,
}

impl From<&LineCode> for LineCodeTask {
    fn from(linecode: &LineCode) -> Self {
        LineCodeTask {
            key: linecode.key.clone(),
            processed: false,
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
    derive_more::Deref,
    derive_more::DerefMut,
)]
pub struct LineCodeTasks(Vec<LineCodeTask>);

impl From<&LineCodes> for LineCodeTasks {
    fn from(linecodes: &LineCodes) -> Self {
        let mut tasks = Vec::new();
        for code in linecodes.iter() {
            tasks.push(LineCodeTask::from(code));
        }
        LineCodeTasks(tasks)
    }
}
