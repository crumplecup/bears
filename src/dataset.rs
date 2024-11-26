use crate::{BeaError, BeaParameters, RequestParameters, User};
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
    pub async fn parameters(&self, user: &User) -> Result<BeaParameters, BeaError> {
        let mut body = user.body();
        body.push_str("&method=GETPARAMETERLIST");
        body.push_str(&format!("&datasetname={}", self.dataset_name));
        let client = reqwest::Client::new();
        let res = client.get(body).send().await?;
        info!("Response code: {}.", res.status());
        let data = res.json::<BeaParameters>().await?;
        Ok(data)
    }

    pub fn name(&self) -> String {
        self.dataset_name.to_owned()
    }
}

impl TryFrom<serde_json::Value> for DatasetDetails {
    type Error = BeaError;
    // #[tracing::instrument(skip_all)]
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        match value {
            serde_json::Value::Object(m) => {
                if let Some(name) = m.get("DatasetName") {
                    if let Some(desc) = m.get("DatasetDescription") {
                        let details = DatasetDetails::new(name.to_string(), desc.to_string());
                        Ok(details)
                    } else {
                        tracing::info!("Invalid contents: {m:#?}");
                        Err(Self::Error::ParseError)
                    }
                } else {
                    tracing::info!("Invalid Object: {m:#?}");
                    Err(Self::Error::ParseError)
                }
            }
            _ => {
                tracing::info!("Invalid Value: {value:#?}");
                Err(Self::Error::ParseError)
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
    type Error = BeaError;
    // #[tracing::instrument(skip_all)]
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        tracing::info!("Reading DatasetDetails");
        match value {
            serde_json::Value::Object(m) => {
                if let Some(serde_json::Value::Array(v)) = m.get("Dataset") {
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
                    Err(Self::Error::ParseError)
                }
            }
            _ => {
                tracing::warn!("Wrong Value type: {value:#?}");
                Err(Self::Error::ParseError)
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
    type Error = BeaError;
    // #[tracing::instrument(skip_all)]
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        tracing::info!("Reading DatasetResults");
        match value {
            serde_json::Value::Object(m) => {
                tracing::info!("Object found.");
                if let Some(req) = m.get("Request") {
                    tracing::info!("Request found.");
                    if let Some(res) = m.get("Results") {
                        tracing::info!("Results found.");
                        let request = RequestParameters::try_from(req.clone())?;
                        tracing::info!("Request parameters read.");
                        let results = Datasets::try_from(res.clone())?;
                        tracing::info!("Datasets read.");
                        let dataset = DatasetResults::new(request, results);
                        Ok(dataset)
                    } else {
                        tracing::info!("Invalid contents: {m:#?}");
                        Err(Self::Error::ParseError)
                    }
                } else {
                    tracing::info!("Invalid Object: {m:#?}");
                    Err(Self::Error::ParseError)
                }
            }
            _ => {
                tracing::info!("Invalid Value: {value:#?}");
                Err(Self::Error::ParseError)
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
    // #[tracing::instrument(skip_all)]
    pub async fn get(user: &User) -> Result<Self, BeaError> {
        let mut body = user.body();
        body.push_str("&method=GETDATASETLIST");
        let client = reqwest::Client::new();
        let res = client.get(body).send().await?;
        info!("Response code: {}.", res.status());
        let data = res.json::<BeaDatasets>().await?;
        Ok(data)
    }

    pub fn results(&self) -> Vec<DatasetDetails> {
        self.beaapi.results.to_vec()
    }
}

impl TryFrom<serde_json::Value> for BeaDatasets {
    type Error = BeaError;
    #[tracing::instrument]
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        tracing::info!("Reading BeaDatasets");
        match value {
            serde_json::Value::Object(m) => {
                if let Some(val) = m.get("BEAAPI") {
                    tracing::info!("Val is: {val:#?}");
                    let data = DatasetResults::try_from(val.clone())?;
                    let data = BeaDatasets::new(data);
                    Ok(data)
                } else {
                    tracing::info!("Invalid Object: {m:#?}");
                    Err(Self::Error::ParseError)
                }
            }
            _ => {
                tracing::info!("Invalid Value: {value:#?}");
                Err(Self::Error::ParseError)
            }
        }
    }
}
