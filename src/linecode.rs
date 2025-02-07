use crate::RequestParameters;
use serde::{Deserialize, Serialize};
use tracing::info;

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
