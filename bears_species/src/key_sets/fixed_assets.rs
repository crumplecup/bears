use crate::{
    BeaErr, BeaResponse, Data, Dataset, DatasetMissing, IoError, NipaRange, NipaRanges, NotArray,
    NotObject, ParameterName, ParameterValueTable, ParameterValueTableVariant, SerdeJson, Set,
    TableName, VariantMissing, date_by_period, map_to_float, map_to_int, map_to_string,
    result_to_data,
};

#[derive(
    Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, derive_getters::Getters,
)]
pub struct FixedAssets {
    table_name: Vec<TableName>,
    year: NipaRanges,
}

impl FixedAssets {
    pub fn iter_tables(&self) -> FixedAssetsTables<'_> {
        FixedAssetsTables::new(self)
    }

    // pub fn queue() -> Result<Queue, BeaErr> {
    //     let req = Request::Data;
    //     let mut app = req.init()?;
    //     let dataset = Dataset::FixedAssets;
    //     app.with_dataset(dataset);
    //     dotenvy::dotenv().ok();
    //     let path = bea_data()?;
    //     let data = FixedAssets::try_from(&path)?;
    //     let mut queue = Vec::new();
    //     for params in data.iter() {
    //         app.with_params(params.clone());
    //         queue.push(app.clone());
    //     }
    //     Ok(Queue::new(queue))
    // }
}

impl TryFrom<&std::path::PathBuf> for FixedAssets {
    type Error = BeaErr;
    fn try_from(value: &std::path::PathBuf) -> Result<Self, Self::Error> {
        let dataset = Dataset::FixedAssets;
        let names = dataset.names();
        // empty vectors to store values
        let mut table_name = Vec::new();
        let mut year = std::collections::BTreeMap::new();
        // For each parameter in dataset
        for name in names {
            // open the file at the expected storage location, error if missing
            let path = value.join(format!(
                "parameter_values/{dataset}_{name}_parameter_values.json"
            ));
            let file = std::fs::File::open(&path)
                .map_err(|e| IoError::new(path, e, line!(), file!().into()))?;
            // read the file to json
            let rdr = std::io::BufReader::new(file);
            let res: serde_json::Value = serde_json::from_reader(rdr)
                .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
            // parse to internal bea response format
            let data = BeaResponse::try_from(&res)?;
            let results = data.results();
            // access parameter values from response
            if let Some(pv) = results.into_parameter_values() {
                // type of vector varies by parameter name
                match name {
                    ParameterName::TableName => {
                        for table in pv.iter() {
                            table_name.push(TableName::try_from(table)?);
                        }
                    }
                    ParameterName::Year => {
                        for table in pv.iter() {
                            let table_name = match table {
                                ParameterValueTable::NipaYear(nipa_year) => {
                                    nipa_year.table_name().clone()
                                }
                                _ => {
                                    let error = ParameterValueTableVariant::new(
                                        "NipaYear needed".to_string(),
                                        line!(),
                                        file!().to_string(),
                                    );
                                    return Err(error.into());
                                }
                            };
                            let nipa_range = NipaRange::try_from(table)?;
                            year.insert(table_name, nipa_range);
                        }
                    }
                    other => return Err(Set::ParameterNameMissing(other.to_string()).into()),
                }
            } else {
                tracing::warn!("Results must be of type ParameterValues");
                return Err(Set::ParameterValuesMissing.into());
            }
        }
        if table_name.is_empty() || year.is_empty() {
            tracing::warn!("Value field is empty.");
            Err(Set::Empty.into())
        } else {
            let year = NipaRanges::new(year);
            let table = Self { table_name, year };
            Ok(table)
        }
    }
}

/// Returns an iterator over investments in the `Iip` struct.
/// Used to create API calls with Year set to "ALL".
#[derive(Debug, Clone)]
pub struct FixedAssetsTables<'a> {
    table_names: std::slice::Iter<'a, TableName>,
}

impl<'a> FixedAssetsTables<'a> {
    /// Creates an iterator over table names in the provided `FixedAssets` struct.
    pub fn new(data: &'a FixedAssets) -> Self {
        let table_names = data.table_name().iter();
        Self { table_names }
    }
}

impl Iterator for FixedAssetsTables<'_> {
    type Item = std::collections::BTreeMap<String, String>;

    fn next(&mut self) -> Option<Self::Item> {
        // empty parameters dictionary
        let mut params = std::collections::BTreeMap::new();

        // set table name
        let table_name = self.table_names.next()?;
        let (key, value) = table_name.params();
        params.insert(key, value);

        // set years to all
        let key = ParameterName::Year.to_string();
        let value = "ALL".to_owned();
        params.insert(key, value);

        Some(params)
    }
}

#[derive(Clone, Debug, Default, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
pub struct FixedAssetDatum {
    cl_unit: String,
    data_value: f64,
    line_description: String,
    line_number: i64,
    metric_name: String,
    series_code: String,
    table_name: String,
    time_period: jiff::civil::Date,
    unit_mult: Option<i64>,
}

impl FixedAssetDatum {
    pub fn read_json(m: &serde_json::Map<String, serde_json::Value>) -> Result<Self, BeaErr> {
        let cl_unit = map_to_string("CL_UNIT", m)?;
        let data_value = map_to_float("DataValue", m)?;
        let line_description = map_to_string("LineDescription", m)?;
        let line_number = map_to_int("LineNumber", m)?;
        let metric_name = map_to_string("METRIC_NAME", m)?;
        let series_code = map_to_string("SeriesCode", m)?;
        let table_name = map_to_string("TableName", m)?;
        let time_period = map_to_string("TimePeriod", m)?;
        let time_period = date_by_period(&time_period)?;
        let unit_mult = map_to_int("UNIT_MULT", m)?;
        let unit_mult = match unit_mult {
            0 => None,
            num => Some(num),
        };
        Ok(Self {
            cl_unit,
            data_value,
            line_description,
            line_number,
            metric_name,
            series_code,
            table_name,
            time_period,
            unit_mult,
        })
    }
}

impl TryFrom<serde_json::Value> for FixedAssetDatum {
    type Error = BeaErr;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        tracing::trace!("Reading FixedAssetDatum.");
        match value {
            serde_json::Value::Object(m) => {
                let data = Self::read_json(&m)?;
                Ok(data)
            }
            _ => {
                tracing::trace!("Invalid Value: {value:#?}");
                let error = NotObject::new(line!(), file!().to_string());
                Err(error.into())
            }
        }
    }
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
    derive_more::Deref,
    derive_more::DerefMut,
    derive_more::From,
)]
#[from(Vec<FixedAssetDatum>)]
pub struct FixedAssetData(Vec<FixedAssetDatum>);

impl TryFrom<&std::path::PathBuf> for FixedAssetData {
    type Error = BeaErr;

    fn try_from(value: &std::path::PathBuf) -> Result<Self, Self::Error> {
        let file = std::fs::File::open(value)
            .map_err(|e| IoError::new(value.into(), e, line!(), file!().into()))?;
        let rdr = std::io::BufReader::new(file);
        let res: serde_json::Value = serde_json::from_reader(rdr)
            .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
        let data = BeaResponse::try_from(&res)?;
        tracing::trace!("Response read.");
        tracing::trace!("Response: {data:#?}");
        let results = data.results();
        if let Some(data) = results.into_data() {
            match data {
                Data::FixedAssets(value) => {
                    tracing::trace!("{} FixedAsset records read.", value.len());
                    Ok(value)
                }
                _ => {
                    let error = DatasetMissing::new(
                        "FixedAssets".to_string(),
                        line!(),
                        file!().to_string(),
                    );
                    Err(error.into())
                }
            }
        } else {
            tracing::warn!("Data variant missing.");
            let error = VariantMissing::new(
                "Data variant missing".to_string(),
                "Results".to_string(),
                line!(),
                file!().to_string(),
            );
            Err(error.into())
        }
    }
}

impl TryFrom<&serde_json::Value> for FixedAssetData {
    type Error = BeaErr;
    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        tracing::trace!("Reading FixedAssetData");
        match result_to_data(value)? {
            serde_json::Value::Array(v) => {
                let mut data = Vec::new();
                for val in v {
                    match val {
                        serde_json::Value::Object(m) => {
                            let datum = FixedAssetDatum::read_json(m)?;
                            data.push(datum);
                        }
                        _ => {
                            let error = NotObject::new(line!(), file!().to_string());
                            return Err(error.into());
                        }
                    }
                }
                tracing::trace!("Data found: {} records.", data.len());
                Ok(Self(data))
            }
            _ => {
                let error = NotArray::new(line!(), file!().to_string());
                Err(error.into())
            }
        }
    }
}
