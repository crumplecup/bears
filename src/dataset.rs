use crate::{
    error::BincodeError, BeaParameters, JsonParseError, JsonParseErrorKind, RequestParameters,
    ReqwestError, User,
};
use convert_case::Casing;
use serde::{Deserialize, Serialize};
use tracing::info;

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
    strum::EnumIter,
    derive_more::Display,
    derive_more::FromStr,
)]
pub enum Dataset {
    #[default]
    #[display("NIPA")]
    Nipa,
    NIUnderlyingDetail,
    #[display("MNE")]
    Mne,
    FixedAssets,
    #[display("ITA")]
    Ita,
    #[display("IIP")]
    Iip,
    InputOutput,
    IntlServTrade,
    IntlServSTA,
    GDPbyIndustry,
    Regional,
    UnderlyingGDPbyIndustry,
    APIDatasetMetadata,
}

impl Dataset {
    /// The `lower` method converts the variant name to `lowercase` case using
    /// [`convert_case::Case::Flat`].
    #[tracing::instrument]
    pub fn lower(&self) -> String {
        self.to_string().to_case(convert_case::Case::Flat)
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
    derive_getters::Getters,
)]
#[serde(rename_all = "PascalCase")]
pub struct DatasetDetails {
    dataset_description: String,
    dataset_name: String,
}

impl DatasetDetails {
    #[tracing::instrument(skip_all)]
    pub async fn parameters(&self, user: &User) -> Result<BeaParameters, ReqwestError> {
        let mut body = user.body();
        body.push_str("&method=GETPARAMETERLIST");
        body.push_str(&format!("&datasetname={}", self.dataset_name));
        let url = body.clone();
        let client = reqwest::Client::new();
        match client.get(body).send().await {
            Ok(res) => {
                info!("Response code: {}.", res.status());
                match res.json::<BeaParameters>().await {
                    Ok(r) => Ok(r),
                    Err(source) => {
                        let error = ReqwestError::new(url, "get".into(), source);
                        return Err(error);
                    }
                }
            }
            Err(source) => {
                let error = ReqwestError::new(url, "get".into(), source);
                return Err(error);
            }
        }
    }

    pub fn name(&self) -> String {
        self.dataset_name.to_owned()
    }
}

impl TryFrom<serde_json::Value> for DatasetDetails {
    type Error = JsonParseError;
    // #[tracing::instrument(skip_all)]
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        match value {
            serde_json::Value::Object(m) => {
                let key_name = "DatasetName".to_string();
                if let Some(name) = m.get(&key_name) {
                    let key_desc = "DatasetDescription".to_string();
                    if let Some(desc) = m.get(&key_desc) {
                        let details = DatasetDetails::new(name.to_string(), desc.to_string());
                        Ok(details)
                    } else {
                        tracing::info!("Invalid contents: {m:#?}");
                        let error = JsonParseErrorKind::KeyMissing(key_desc);
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
pub struct Datasets {
    dataset: Vec<DatasetDetails>,
}

impl TryFrom<serde_json::Value> for Datasets {
    type Error = JsonParseError;
    // #[tracing::instrument(skip_all)]
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        tracing::info!("Reading DatasetDetails");
        match value {
            serde_json::Value::Object(m) => {
                let key = "Dataset".to_string();
                if let Some(serde_json::Value::Array(v)) = m.get(&key) {
                    tracing::info!("Array found.");
                    let mut dataset = Vec::new();
                    for val in v {
                        let details = DatasetDetails::try_from(val.clone())?;
                        dataset.push(details);
                    }
                    tracing::info!("Details found.");
                    let datasets = Datasets::new(dataset);
                    Ok(datasets)
                } else {
                    tracing::warn!("Unexpected content: {m:#?}");
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
pub struct DatasetResults {
    pub request: RequestParameters,
    pub results: Datasets,
}

impl TryFrom<serde_json::Value> for DatasetResults {
    type Error = JsonParseError;
    // #[tracing::instrument(skip_all)]
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        tracing::info!("Reading DatasetResults");
        match value {
            serde_json::Value::Object(m) => {
                tracing::info!("Object found.");
                let key_request = "Request".to_string();
                if let Some(req) = m.get(&key_request) {
                    tracing::info!("Request found.");
                    let key_results = "Results".to_string();
                    if let Some(res) = m.get(&key_results) {
                        tracing::info!("Results found.");
                        let request = RequestParameters::try_from(req)?;
                        tracing::info!("Request parameters read.");
                        let results = Datasets::try_from(res.clone())?;
                        tracing::info!("Datasets read.");
                        let dataset = DatasetResults::new(request, results);
                        Ok(dataset)
                    } else {
                        tracing::info!("Invalid contents: {m:#?}");
                        let error = JsonParseErrorKind::KeyMissing(key_results);
                        Err(error.into())
                    }
                } else {
                    tracing::info!("Invalid Object: {m:#?}");
                    let error = JsonParseErrorKind::KeyMissing(key_request);
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
    derive_getters::Getters,
    derive_more::Deref,
    derive_more::DerefMut,
)]
#[serde(rename_all = "UPPERCASE")]
pub struct BeaDatasets {
    beaapi: DatasetResults,
}

impl BeaDatasets {
    #[tracing::instrument(skip_all)]
    pub async fn get(user: &User) -> Result<Self, ReqwestError> {
        let mut body = user.body();
        body.push_str("&method=GETDATASETLIST");
        let client = reqwest::Client::new();
        let url = body.clone();
        match client.get(body).send().await {
            Ok(res) => {
                info!("Response code: {}.", res.status());
                match res.json::<BeaDatasets>().await {
                    Ok(r) => Ok(r),
                    Err(source) => {
                        let error = ReqwestError::new(url, "get".into(), source);
                        Err(error)
                    }
                }
            }
            Err(source) => {
                let error = ReqwestError::new(url, "get".into(), source);
                Err(error)
            }
        }
    }

    #[tracing::instrument(skip_all)]
    pub fn results(&self) -> Vec<DatasetDetails> {
        self.beaapi.results.to_vec()
    }

    #[tracing::instrument(skip_all)]
    pub fn serialize(&self) -> Result<Vec<u8>, BincodeError> {
        match bincode::serialize(self) {
            Ok(data) => Ok(data),
            Err(source) => {
                let error = BincodeError::new("serializing BeaDatasets".to_string(), source);
                Err(error)
            }
        }
    }

    #[tracing::instrument(skip_all)]
    pub fn deserialize(bytes: &[u8]) -> Result<Self, BincodeError> {
        match bincode::deserialize(bytes) {
            Ok(data) => Ok(data),
            Err(source) => {
                let error = BincodeError::new("deserializing BeaDatasets".to_string(), source);
                Err(error)
            }
        }
    }
}

impl TryFrom<serde_json::Value> for BeaDatasets {
    type Error = JsonParseError;
    #[tracing::instrument]
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        tracing::info!("Reading BeaDatasets");
        match value {
            serde_json::Value::Object(m) => {
                let key = "BEAAPI".to_string();
                if let Some(val) = m.get(&key) {
                    tracing::info!("Val is: {val:#?}");
                    let data = DatasetResults::try_from(val.clone())?;
                    let data = BeaDatasets::new(data);
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
