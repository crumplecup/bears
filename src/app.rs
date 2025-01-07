use crate::{Options, ReqwestError};
use std::collections::HashMap;

/// The `App` struct contains the application state.
///
/// The [`get`](Self::get) method makes calls to the BEA REST API constructing calls from the
/// [`Options`] contained in the `options` field.
#[derive(Debug, Clone, PartialEq, Eq, derive_getters::Getters, derive_setters::Setters)]
#[setters(prefix = "with_", into, borrow_self)]
pub struct App {
    key: String,
    options: Options,
    url: url::Url,
    query: HashMap<String, String>,
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
    pub fn params(&self) -> HashMap<String, String> {
        let mut params = self.options.params();
        params.insert("USERID".to_string(), self.key.clone());
        params
    }

    /// Append parameters to the cached query.
    #[tracing::instrument(skip_all)]
    pub fn with_params(&mut self, params: HashMap<String, String>) {
        self.query.extend(params);
    }

    /// Internal library workhorse function for REST API calls.  Configure the desired parameters
    /// of the call using the [`Options`].
    #[tracing::instrument(skip_all)]
    pub async fn get(&self) -> Result<reqwest::Response, ReqwestError> {
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
}
