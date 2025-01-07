use crate::{BeaResponse, ReqwestError, User};
use convert_case::Casing;

/// The variants of the `Method` enum represent different valid inputs for the `METHOD` parameter
/// when calling into the BEA REST API.
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
pub enum Method {
    /// Primary data retrieval method.
    #[default]
    GetData,
    /// Available BEA datasets, corresponding to variants of [`Dataset`].
    GetDataSetList,
    /// Valid [`ParameterName`] values for a given [`Dataset`].
    GetParameterList,
    GetParameterValues,
    GetParameterValuesFiltered,
}

impl Method {
    /// The `upper` method converts the variant name to `UPPERCASE` case using
    /// [`convert_case::Case::UpperFlat`].
    /// Conforms to the user manual for the BEA API.
    #[tracing::instrument(skip_all)]
    pub fn upper(&self) -> String {
        self.to_string().to_case(convert_case::Case::UpperFlat)
    }

    // pub async fn list_datasets(&self, config: &NeoConfig) -> Result<BeaDatasets, ReqwestError> {
    //     let url = config.url();
    //     let mut params = config.params();
    //     params.insert("METHOD".to_string(), self.upper());
    //     params.insert("RESULTFORMAT".to_string(), "JSON".to_string());
    //     let client = reqwest::Client::new();
    //     let req = client.get(url.clone()).query(&params);
    //     info!("Sending request: {:?}", req);
    //     let res = req.send().await?;
    //     info!("Response code: {}.", res.status());
    //     info!("Response: {:#?}", res);
    //     info!("Body: {:#?}", res.text().await?);
    //     let req = client.get(url.clone()).query(&params);
    //     info!("Sending request: {:?}", req);
    //     let res = req.send().await?;
    //     let data = res.json::<BeaDatasets>().await?;
    //     info!("Data: {:#?}", data);
    //     Ok(data)
    // }

    #[tracing::instrument(skip_all)]
    pub async fn get_dataset_list(user: &User) -> Result<BeaResponse, ReqwestError> {
        let url = user.url();
        let mut params = user.params();
        params.insert("METHOD".to_string(), "GETDATASETLIST".to_string());
        params.insert("RESULTFORMAT".to_string(), "JSON".to_string());
        let body = params
            .clone()
            .into_iter()
            .collect::<Vec<(String, String)>>();
        let client = reqwest::Client::new();
        // let req = client.get(url.clone()).query(&params);
        // info!("Sending request: {:?}", req);
        // let res = req.send().await?;
        // info!("Response code: {}.", res.status());
        // info!("Response: {:#?}", res);
        // info!("Body: {:#?}", res.text().await?);
        match client.get(url.clone()).query(&params).send().await {
            Ok(res) => match res.json::<BeaResponse>().await {
                Ok(data) => Ok(data),
                Err(source) => {
                    let mut error = ReqwestError::new(
                        url.to_string(),
                        "get".to_string(),
                        source,
                        line!(),
                        file!().to_string(),
                    );
                    error.with_body(body);
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
                error.with_body(body);
                Err(error)
            }
        }
    }
}
