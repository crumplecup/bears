use crate::{error::BincodeError, RequestParameters};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GeoFipsItem {
    key: String,
    desc: String,
}

impl GeoFipsItem {
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
pub struct GeoFips(Vec<GeoFipsItem>);

impl GeoFips {
    pub fn report(&self) {
        self.iter().map(|c| c.report()).for_each(drop);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GeoFipsResults {
    request: RequestParameters,
    results: GeoFips,
}

#[derive(
    Clone,
    Debug,
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
pub struct BeaGeoFips(GeoFipsResults);

impl BeaGeoFips {
    pub fn results(&self) -> GeoFips {
        self.results.clone()
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
    derive_getters::Getters,
    derive_setters::Setters,
)]
#[setters(prefix = "with_", into, borrow_self)]
pub struct GeoFipsTask {
    key: String,
    processed: bool,
}

impl GeoFipsTask {
    pub fn report(&self) {
        info!("Key: {}, Processed: {}", self.key, self.processed);
    }
}

impl From<&GeoFipsItem> for GeoFipsTask {
    fn from(geofips: &GeoFipsItem) -> Self {
        GeoFipsTask {
            key: geofips.key.clone(),
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
    derive_new::new,
    derive_more::Deref,
    derive_more::DerefMut,
)]
pub struct GeoFipsTasks(Vec<GeoFipsTask>);

impl GeoFipsTasks {
    #[tracing::instrument(skip_all)]
    pub fn tasks(&self) -> Vec<GeoFipsTask> {
        self.to_vec()
    }

    #[tracing::instrument(skip_all)]
    pub fn tasks_mut(&mut self) -> &mut Vec<GeoFipsTask> {
        &mut *self
    }

    #[tracing::instrument(skip_all)]
    pub fn report(&self) {
        for task in self.iter() {
            task.report();
        }
    }

    #[tracing::instrument(skip_all)]
    pub fn serialize(&self) -> Result<Vec<u8>, BincodeError> {
        match bincode::serialize(self) {
            Ok(data) => Ok(data),
            Err(source) => {
                let error = BincodeError::new("serializing GeoFipsTasks".to_string(), source);
                Err(error)
            }
        }
    }

    #[tracing::instrument(skip_all)]
    pub fn deserialize(bytes: &[u8]) -> Result<Self, BincodeError> {
        match bincode::deserialize(bytes) {
            Ok(data) => Ok(data),
            Err(source) => {
                let error = BincodeError::new("deserializing GeoFipsTasks".to_string(), source);
                Err(error)
            }
        }
    }
}

impl From<&GeoFips> for GeoFipsTasks {
    fn from(geofips: &GeoFips) -> Self {
        let mut tasks = Vec::new();
        for code in geofips.iter() {
            tasks.push(GeoFipsTask::from(code));
        }
        GeoFipsTasks(tasks)
    }
}
