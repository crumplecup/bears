use crate::{deserialize_bool, Config, RequestParameters, ReqwestError};
use serde::{Deserialize, Serialize};
use tracing::info;

#[tracing::instrument(skip_all)]
pub async fn get_data(config: &Config) -> Result<BeaDataResponse, ReqwestError> {
    let mut body = config.body();
    body.push_str("&method=GetData");
    let url = body.clone();
    let client = reqwest::Client::new();
    match client.get(body).send().await {
        Ok(res) => match res.json::<BeaDataResponse>().await {
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
pub struct Dimension {
    name: String,
    data_type: String,
    #[serde(deserialize_with = "deserialize_bool")]
    is_value: bool,
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
pub struct Dimensions(Vec<Dimension>);

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Note {
    note_ref: String,
    note_text: String,
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
pub struct Notes(Vec<Note>);

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Datum {
    code: String,
    geo_fips: String,
    geo_name: String,
    time_period: String,
    description: String,
    #[serde(rename = "CL_UNIT")]
    cl_unit: String,
    #[serde(rename = "UNIT_MULT")]
    unit_mult: String,
    data_value: String,
}

impl Datum {
    #[tracing::instrument(skip_all)]
    pub fn report(&self) {
        info!("Desc: {}, Value: {}", self.description, self.data_value);
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
pub struct Data(Vec<Datum>);

impl Data {
    #[tracing::instrument(skip_all)]
    pub fn new(data: &[Datum]) -> Self {
        Data(data.to_vec())
    }

    #[tracing::instrument(skip(self))]
    pub fn to_csv(&mut self, title: std::path::PathBuf) -> Result<(), std::io::Error> {
        let mut wtr = csv::Writer::from_path(title)?;
        for i in self.iter() {
            wtr.serialize(i)?;
        }
        wtr.flush()?;
        Ok(())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DataResult {
    statistic: String,
    unit_of_measure: String,
    public_table: String,
    #[serde(rename(deserialize = "UTCProductionTime"))]
    utc_production_time: String,
    note_ref: String,
    dimensions: Vec<Dimension>,
    data: Vec<Datum>,
    notes: Vec<Note>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DataResponse {
    request: RequestParameters,
    results: DataResult,
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
pub struct BeaDataResponse(DataResponse);

impl BeaDataResponse {
    #[tracing::instrument(skip_all)]
    pub fn results(&self) -> Vec<Datum> {
        self.0.results.data.clone()
    }

    #[tracing::instrument(skip_all)]
    pub async fn get(config: &Config) -> Result<BeaDataResponse, ReqwestError> {
        let mut params = config.user().params();
        params.insert("method".to_string(), "GetData".to_string());
        let form = params
            .clone()
            .into_iter()
            .collect::<Vec<(String, String)>>();
        let url = config.user().url().clone();
        let client = reqwest::Client::new();
        match client.get(url.clone()).form(&params).send().await {
            Ok(res) => match res.json::<BeaDataResponse>().await {
                Ok(data) => Ok(data),
                Err(source) => {
                    let mut error = ReqwestError::new(
                        url.to_string(),
                        "get".to_string(),
                        source,
                        line!(),
                        file!().to_string(),
                    );
                    error.with_form(form);
                    Err(error)
                }
            },
            Err(source) => {
                let mut error = ReqwestError::new(
                    url.to_string(),
                    "get".to_string(),
                    source,
                    line!(),
                    file!().to_string(),
                );
                error.with_form(form);
                Err(error)
            }
        }
    }
}
