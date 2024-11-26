use crate::{BeaDatasets, BeaError, Options};
use std::collections::HashMap;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    derive_new::new,
    derive_getters::Getters,
    derive_setters::Setters,
)]
#[setters(prefix = "with_", into, borrow_self)]
pub struct App {
    key: String,
    options: Options,
    url: url::Url,
}

impl App {
    pub fn params(&self) -> HashMap<String, String> {
        let mut params = self.options.params();
        params.insert("USERID".to_string(), self.key.clone());
        params
    }

    pub async fn get(&self) -> Result<reqwest::Response, BeaError> {
        let params = self.params();
        let client = reqwest::Client::new();
        let req = client.get(self.url.clone()).query(&params);
        tracing::trace!("Sending request: {:?}", req);
        let res = req.send().await?;
        tracing::trace!("Response code: {}.", res.status());
        tracing::trace!("Response: {:#?}", res);
        tracing::trace!("Body: {:#?}", res.text().await?);
        let req = client.get(self.url.clone()).query(&params);
        tracing::trace!("Sending request: {:?}", req);
        let res = req.send().await?;
        // let data = res.json::<BeaDatasets>().await?;
        // tracing::trace!("Data: {:#?}", data);
        // Ok(data)
        Ok(res)
    }
}
