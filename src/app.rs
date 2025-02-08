use crate::{
    BeaErr, BeaResponse, Dataset, DeriveFromStr, EnvError, IoError, JsonParseError,
    JsonParseErrorKind, KeyMissing, Method, Options, ParameterName, RateLimit, ReqwestError,
    Results, SerdeJson, VariantMissing,
};
use std::collections::BTreeMap;
use std::str::FromStr;

/// The `App` struct contains the application state.
///
/// The [`get`](Self::get) method makes calls to the BEA REST API constructing calls from the
/// [`Options`] contained in the `options` field.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_getters::Getters,
    derive_setters::Setters,
    serde::Serialize,
    serde::Deserialize,
)]
#[setters(prefix = "with_", into, borrow_self)]
pub struct App {
    key: String,
    options: Options,
    url: url::Url,
    query: BTreeMap<String, String>,
}

impl App {
    pub fn new(key: String, options: Options, url: url::Url) -> Self {
        let mut query = options.params();
        query.insert("USERID".to_string(), key.clone());
        Self {
            key,
            options,
            url,
            query,
        }
    }

    pub fn add_options(&mut self, options: Options) {
        self.options = options;
        self.query = self.params();
    }

    /// Produces the parameters appended to the REST endpoint.  Used by the [`get`](Self::get)
    /// method to construct REST API calls.
    #[tracing::instrument(skip_all)]
    pub fn params(&self) -> BTreeMap<String, String> {
        let mut params = self.options.params();
        params.insert("USERID".to_string(), self.key.clone());
        params
    }

    /// Append parameters to the cached query.
    #[tracing::instrument(skip_all)]
    pub fn with_params(&mut self, params: BTreeMap<String, String>) {
        self.query.extend(params);
    }

    pub fn options_mut(&mut self) -> &mut Options {
        &mut self.options
    }

    pub fn with_dataset(&mut self, dataset: Dataset) {
        let options = self.options_mut();
        options.with_dataset(dataset);
        self.query = self.params();
    }

    /// Internal library workhorse function for REST API calls.  Configure the desired parameters
    /// of the call using the [`Options`].
    #[tracing::instrument(skip_all)]
    pub async fn get(&self) -> Result<reqwest::Response, ReqwestError> {
        tracing::trace!("Calling get for App.");
        let body = self
            .query
            .clone()
            .into_iter()
            .collect::<Vec<(String, String)>>();
        let client = reqwest::Client::new();
        let req = client.get(self.url.clone()).query(&self.query);
        tracing::trace!("Sending request: {:?}", req);
        match req.send().await {
            Ok(res) => Ok(res),
            Err(source) => {
                let mut error = ReqwestError::new(
                    self.url().to_string(),
                    "get".to_string(),
                    source,
                    line!(),
                    file!().to_string(),
                );
                error.with_body(body);
                Err(error)
            }
        }
    }

    pub fn method(&self) -> Result<Method, DeriveFromStr> {
        let query = self.query();
        let method = query["METHOD"].clone();
        match Method::from_str(&method) {
            Ok(value) => Ok(value),
            Err(source) => {
                let error = DeriveFromStr::new(method, source, line!(), file!().to_string());
                Err(error)
            }
        }
    }

    pub fn destination(&self, create: bool) -> Result<std::path::PathBuf, BeaErr> {
        let query = self.query();
        tracing::trace!("Params are {:#?}", query);
        let method = self.method()?;
        let dataset = query["DatasetName"].clone();
        let datakind = match Dataset::from_str(&dataset) {
            Ok(kind) => kind,
            Err(source) => {
                let error = DeriveFromStr::new(dataset, source, line!(), file!().to_string());
                return Err(error.into());
            }
        };
        dotenvy::dotenv().ok();
        let bea_data = EnvError::from_env("BEA_DATA")?;
        let path = std::path::PathBuf::from(&bea_data);
        match method {
            Method::GetData => {
                let path = path.join("data");
                if !path.exists() && create {
                    std::fs::DirBuilder::new()
                        .create(&path)
                        .map_err(|e| IoError::new(path.clone(), e, line!(), file!().into()))?;
                    tracing::info!("Target directory for Data created.");
                }
                let path = path.join(&dataset);
                if !path.exists() && create {
                    std::fs::DirBuilder::new()
                        .create(&path)
                        .map_err(|e| IoError::new(path.clone(), e, line!(), file!().into()))?;
                    tracing::info!("Target directory for {dataset} created.");
                }
                match datakind {
                    Dataset::Nipa | Dataset::NIUnderlyingDetail | Dataset::FixedAssets => {
                        let name = query["TableName"].clone();
                        Ok(path.join(format!("{dataset}_{name}.json")))
                    }
                    Dataset::Mne => {
                        let country = query["Country"].clone();
                        let doi = query["DirectionOfInvestment"].clone();
                        let class = query["Classification"].clone();
                        if let Some(nonbank) =
                            query.get(ParameterName::NonbankAffiliatesOnly.to_string().as_str())
                        {
                            let path = path.join("AMNE");
                            if !path.exists() && create {
                                std::fs::DirBuilder::new().create(&path).map_err(|e| {
                                    IoError::new(path.clone(), e, line!(), file!().into())
                                })?;
                                tracing::info!("Target directory for AMNE created.");
                            }
                            let path = path.join(country);
                            if !path.exists() && create {
                                std::fs::DirBuilder::new().create(&path).map_err(|e| {
                                    IoError::new(path.clone(), e, line!(), file!().into())
                                })?;
                                tracing::info!("Target directory for {dataset} created.");
                            }
                            let ownership = query["OwnershipLevel"].clone();
                            let path = match (ownership.as_str(), nonbank.as_str()) {
                                ("0", "0") => path.join(format!("{class}_{doi}.json")),
                                ("0", "1") => path.join(format!("{class}_{doi}_nonbank.json")),
                                ("1", "0") => {
                                    path.join(format!("{class}_{doi}_ownership_nonbank.json"))
                                }
                                ("1", "1") => {
                                    path.join(format!("{class}_{doi}_ownership_nonbank.json"))
                                }
                                _ => {
                                    let error = KeyMissing::new(
                                        "0 or 1".to_string(),
                                        line!(),
                                        file!().to_string(),
                                    );
                                    let error = JsonParseErrorKind::from(error);
                                    let error = JsonParseError::from(error);
                                    return Err(error.into());
                                }
                            };
                            Ok(path)
                        } else {
                            let path = path.join("DirectInvestment");
                            if !path.exists() && create {
                                std::fs::DirBuilder::new().create(&path).map_err(|e| {
                                    IoError::new(path.clone(), e, line!(), file!().into())
                                })?;
                                tracing::info!("Target directory for DirectInvestment created.");
                            }
                            let path = path.join(country);
                            if !path.exists() && create {
                                std::fs::DirBuilder::new().create(&path).map_err(|e| {
                                    IoError::new(path.clone(), e, line!(), file!().into())
                                })?;
                                tracing::info!("Target directory for {dataset} created.");
                            }
                            Ok(path.join(format!("{class}_{doi}.json")))
                        }
                    }
                    _ => {
                        tracing::info!("{datakind} not yet implemented.");
                        Ok(path)
                    }
                }
            }
            _ => {
                tracing::info!("Not implemented for {method}.");
                Ok(path)
            }
        }
    }
    #[tracing::instrument(skip_all)]

    pub async fn download(&self, id: uuid::Uuid) -> Result<ResultStatus, BeaErr> {
        tracing::trace!("Calling download.");
        let query = self.query();
        tracing::trace!("Params are {:#?}", query);
        let method = self.method()?;
        match method {
            Method::GetData => {
                let data = self.get().await?;
                let length = data.content_length().unwrap();
                match data.json::<serde_json::Value>().await {
                    Ok(json) => match BeaResponse::try_from(&json) {
                        Ok(response) => match response.results() {
                            Results::ApiError(error) => {
                                tracing::error!("{error}");
                                return Ok(ResultStatus::Error(id));
                            }
                            Results::MneError(error) => {
                                tracing::trace!("{error}");
                                return Ok(ResultStatus::Error(id));
                            }
                            Results::RequestsExceeded(error) => {
                                let error =
                                    RateLimit::new(error.to_string(), line!(), file!().to_string());
                                tracing::error!("{error}");
                                // return Err(error.into());
                                return Ok(ResultStatus::Abort);
                            }
                            _ => {
                                self.save(json).await?;
                                return Ok(ResultStatus::Success(id, length));
                            }
                        },
                        Err(_) => {
                            tracing::warn!("Unrecognized response.");
                            self.save(json).await?;
                            return Ok(ResultStatus::Success(id, length));
                        }
                    },
                    Err(source) => {
                        let url = self.url().to_string();
                        let method = "get".to_string();
                        let error =
                            ReqwestError::new(url, method, source, line!(), file!().to_string());
                        tracing::warn!("{error}");
                        // return Err(error.into());
                        return Ok(ResultStatus::Error(id));
                    }
                }
            }
            _ => {
                tracing::info!("{method} not implemented.");
                Ok(ResultStatus::Error(id))
            }
        }
    }

    pub async fn save(&self, json: serde_json::Value) -> Result<(), BeaErr> {
        tracing::trace!("Calling save.");
        let method = self.method()?;
        match method {
            Method::GetData => {
                let contents = serde_json::to_vec(&json)
                    .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;

                let path = self.destination(true)?;
                std::fs::write(&path, contents)
                    .map_err(|e| IoError::new(path, e, line!(), file!().into()))?;
            }
            _ => {
                tracing::info!("Not implemented for {method}.");
            }
        }
        Ok(())
    }

    pub fn load(&self) -> Result<BeaResponse, BeaErr> {
        tracing::trace!("Calling load.");
        let query = self.query();
        tracing::trace!("Params are {:#?}", query);
        let method = self.method()?;
        match method {
            Method::GetData => {
                let path = self.destination(false)?;
                tracing::info!("Opening {path:?}.");
                // Create reader from path.
                let file = std::fs::File::open(&path)
                    .map_err(|e| IoError::new(path, e, line!(), file!().into()))?;
                let rdr = std::io::BufReader::new(file);
                // Deserialize to serde_json::Value.
                let json: serde_json::Value = serde_json::from_reader(rdr)
                    .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
                BeaResponse::try_from(&json)
            }
            _ => {
                let msg = format!("load not implemented for {method}");
                tracing::info!(msg);
                let error =
                    VariantMissing::new(msg, method.to_string(), line!(), file!().to_string());
                Err(error.into())
            }
        }
    }
}

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum ResultStatus {
    Success(uuid::Uuid, u64),
    Error(uuid::Uuid),
    Pass(uuid::Uuid),
    Pending,
    Abort,
}

impl std::fmt::Display for ResultStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success(_, _) => write!(f, "Success"),
            Self::Error(_) => write!(f, "Error"),
            Self::Pass(_) => write!(f, "Pass"),
            Self::Pending => write!(f, "Pending"),
            Self::Abort => write!(f, "Abort"),
        }
    }
}

impl FromStr for ResultStatus {
    type Err = BeaErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let status = match s {
            "Success" => Self::Success(uuid::Uuid::new_v4(), 0),
            "Error" => Self::Error(uuid::Uuid::new_v4()),
            "Pass" => Self::Pass(uuid::Uuid::new_v4()),
            "Pending" => Self::Pending,
            "Abort" => Self::Abort,
            _ => {
                let error = KeyMissing::new(s.to_string(), line!(), file!().to_string());
                let error = JsonParseErrorKind::from(error);
                let error = JsonParseError::from(error);
                return Err(error.into());
            }
        };
        Ok(status)
    }
}
