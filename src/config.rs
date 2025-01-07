use crate::{Dataset, Method, ParameterName, User};
use std::collections::HashMap;

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    derive_getters::Getters,
    derive_setters::Setters,
)]
#[setters(prefix = "with_", into, strip_option, borrow_self)]
pub struct Options {
    dataset: Option<Dataset>,
    geofips: Option<i32>,
    industry: Option<String>,
    linecode: Option<i32>,
    method: Option<Method>,
    param_name: Option<ParameterName>,
    table: Option<String>,
    table_id: Option<i32>,
    target: Option<ParameterName>,
    year: Option<String>,
}

impl Options {
    #[tracing::instrument]
    pub fn new() -> Self {
        Default::default()
    }

    #[tracing::instrument(skip_all)]
    pub fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        if let Some(dataset) = self.dataset {
            params.insert("DatasetName".to_string(), dataset.to_string());
        }
        if let Some(geofips) = self.geofips {
            params.insert("GeoFips".to_string(), geofips.to_string());
        }
        if let Some(industry) = &self.industry {
            params.insert("Industry".to_string(), industry.to_string());
        }
        if let Some(linecode) = self.linecode {
            params.insert("LineCode".to_string(), linecode.to_string());
        }
        if let Some(method) = self.method {
            params.insert("METHOD".to_string(), method.to_string());
        }
        if let Some(param_name) = self.param_name {
            params.insert("ParameterName".to_string(), param_name.to_string());
        }
        if let Some(table) = &self.table {
            params.insert("TableName".to_string(), table.to_string());
        }
        if let Some(table_id) = self.table_id {
            params.insert("TableID".to_string(), table_id.to_string());
        }
        if let Some(target) = self.target {
            params.insert("TargetParameter".to_string(), target.to_string());
        }
        if let Some(year) = self.year.clone() {
            params.insert("Year".to_string(), year);
        }
        params.insert("RESULTFORMAT".to_string(), "JSON".to_string());
        params
    }
}

#[deprecated]
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
pub struct NeoConfig {
    key: String,
    options: Options,
    url: url::Url,
}

impl NeoConfig {
    #[tracing::instrument(skip_all)]
    pub fn params(&self) -> HashMap<String, String> {
        let mut params = self.options.params();
        params.insert("USERID".to_string(), self.key.clone());
        params
    }
}

#[deprecated]
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    derive_getters::Getters,
    derive_setters::Setters,
)]
#[setters(prefix = "with_", into, strip_option, borrow_self)]
pub struct Config {
    user: User,
    dataset: String,
    table: Option<String>,
    geofips: Option<String>,
    linecode: Option<String>,
    year: Option<String>,
}

impl Config {
    #[tracing::instrument]
    pub fn new(user: &User, dataset: &str) -> Self {
        Config {
            user: user.clone(),
            dataset: dataset.to_owned(),
            table: None,
            geofips: None,
            linecode: None,
            year: None,
        }
    }

    #[tracing::instrument(skip_all)]
    pub fn body(&self) -> String {
        let mut body = self.user.body();
        body.push_str(&format!("&datasetname={}", self.dataset));
        if let Some(table) = self.table.clone() {
            body.push_str(&format!("&TableName={}", table));
        }
        if let Some(geofips) = self.geofips.clone() {
            body.push_str(&format!("&GeoFips={}", geofips));
        }
        if let Some(linecode) = self.linecode.clone() {
            body.push_str(&format!("&LineCode={}", linecode));
        }
        if let Some(year) = self.year.clone() {
            body.push_str(&format!("&Year={}", year));
        }
        body
    }

    #[tracing::instrument(skip_all)]
    pub fn params(&self) -> HashMap<String, String> {
        let mut params = self.user.params();
        if let Some(table) = self.table.clone() {
            params.insert("TableName".to_string(), table);
        }
        if let Some(geofips) = self.geofips.clone() {
            params.insert("GeoFips".to_string(), geofips);
        }
        if let Some(linecode) = self.linecode.clone() {
            params.insert("LineCode".to_string(), linecode);
        }
        if let Some(year) = self.year.clone() {
            params.insert("Year".to_string(), year);
        }
        params
    }
}
