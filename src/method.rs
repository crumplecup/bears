use crate::{BeaDatasets, BeaError, NeoConfig, User};
use convert_case::Casing;
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
pub enum Method {
    #[default]
    GetDataSetList,
    GetParameterList,
    GetParameterValues,
    GetParameterValuesFiltered,
}

impl Method {
    /// The `upper` method converts the variant name to `UPPERCASE` case using
    /// [`convert_case::Case::UpperFlat`].
    /// Conforms to the user manual for the BEA API.
    pub fn upper(&self) -> String {
        self.to_string().to_case(convert_case::Case::UpperFlat)
    }

    pub async fn list_datasets(&self, config: &NeoConfig) -> Result<BeaDatasets, BeaError> {
        let url = config.url();
        let mut params = config.params();
        params.insert("METHOD".to_string(), self.upper());
        params.insert("RESULTFORMAT".to_string(), "JSON".to_string());
        let client = reqwest::Client::new();
        let req = client.get(url.clone()).query(&params);
        info!("Sending request: {:?}", req);
        let res = req.send().await?;
        info!("Response code: {}.", res.status());
        info!("Response: {:#?}", res);
        info!("Body: {:#?}", res.text().await?);
        let req = client.get(url.clone()).query(&params);
        info!("Sending request: {:?}", req);
        let res = req.send().await?;
        let data = res.json::<BeaDatasets>().await?;
        info!("Data: {:#?}", data);
        Ok(data)
    }

    pub async fn get_dataset_list(user: &User) -> Result<BeaDatasets, BeaError> {
        let url = user.url();
        let mut params = user.params();
        params.insert("METHOD".to_string(), "GETDATASETLIST".to_string());
        params.insert("RESULTFORMAT".to_string(), "JSON".to_string());
        let client = reqwest::Client::new();
        let req = client.get(url.clone()).query(&params);
        info!("Sending request: {:?}", req);
        let res = req.send().await?;
        info!("Response code: {}.", res.status());
        info!("Response: {:#?}", res);
        info!("Body: {:#?}", res.text().await?);
        let req = client.get(url.clone()).query(&params);
        info!("Sending request: {:?}", req);
        let res = req.send().await?;
        let data = res.json::<BeaDatasets>().await?;
        info!("Data: {:#?}", data);
        Ok(data)
    }
}
