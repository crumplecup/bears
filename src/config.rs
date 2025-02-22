use crate::{Dataset, Method, ParameterName};
use std::collections::BTreeMap;

/// The `Options` struct holds values for parameters in a BEA request.
///
/// Each field represents a different parameter field in the request.  Only fields that evaluate to
/// `Some(T)` will be included in the request, and fields set to `None` are ignored.
///
/// Fields include:
///
/// * **dataset** - The `DatasetName` parameter, corresponding to the [`Dataset`] type.
/// * **geofips** - The `Geofips` parameter, represented by an `i32`.
/// * **industry** - The `Industry` parameter, represented by a `String`.
/// * **linecode** - The `LineCode` parameter, represented by an `i32`.
/// * **method** - The `METHOD` parameter, corresponding to the [`Method`] type.
/// * **param_name** - The `ParameterName` parameter, corresponding to the [`ParameterName`] type.
/// * **table** - The `TableName` parameter, represented by a `String`.
/// * **table_id** - The `TableID` parameter, represented by an `i32`.
/// * **target** - The `TargetParameter` parameter, corresponding to the [`ParameterName`] type.
/// * **year** - The `Year` parameter, represented by a `String`.
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
    serde::Serialize,
    serde::Deserialize,
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
    /// Creates a new instance of `Options` using the default implementation, with each field set
    /// to `None`.
    #[tracing::instrument]
    pub fn new() -> Self {
        Default::default()
    }

    #[tracing::instrument(skip_all)]
    pub fn params(&self) -> BTreeMap<String, String> {
        let mut params = BTreeMap::new();
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
