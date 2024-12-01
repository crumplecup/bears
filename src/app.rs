use crate::{Options, ReqwestError};
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
    #[tracing::instrument(skip_all)]
    pub fn params(&self) -> HashMap<String, String> {
        let mut params = self.options.params();
        params.insert("USERID".to_string(), self.key.clone());
        params
    }

    #[tracing::instrument(skip_all)]
    pub async fn get(&self) -> Result<reqwest::Response, ReqwestError> {
        let params = self.params();
        let body = params
            .clone()
            .into_iter()
            .collect::<Vec<(String, String)>>();
        let client = reqwest::Client::new();
        let req = client.get(self.url.clone()).query(&params);
        tracing::trace!("Sending request: {:?}", req);
        // let res = req.send().await?;
        // tracing::trace!("Response code: {}.", res.status());
        // tracing::trace!("Response: {:#?}", res);
        // tracing::trace!("Body: {:#?}", res.text().await?);
        // let req = client.get(self.url.clone()).query(&params);
        // tracing::trace!("Sending request: {:?}", req);
        match req.send().await {
            Ok(res) => Ok(res),
            Err(source) => {
                let mut error =
                    ReqwestError::new(self.url().to_string(), "get".to_string(), source);
                error.with_body(body);
                Err(error)
            }
        }
    }
}
