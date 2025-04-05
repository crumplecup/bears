use crate::{
    BeaErr, BeaResponse, Dataset, Integer, IoError, ParameterName, SerdeJson, Set, TableName, Year,
};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_getters::Getters,
)]
pub struct Regional {
    geo_fips: Vec<Integer>,
    line_code: Vec<Integer>,
    table_name: Vec<TableName>,
    year: Vec<Year>,
}

impl TryFrom<&std::path::PathBuf> for Regional {
    type Error = BeaErr;
    fn try_from(value: &std::path::PathBuf) -> Result<Self, Self::Error> {
        let dataset = Dataset::Regional;
        let names = dataset.names();
        // empty vectors to store values
        let mut geo_fips = Vec::new();
        let mut line_code = Vec::new();
        let mut table_name = Vec::new();
        let mut year = Vec::new();
        // For each parameter in dataset
        for name in names {
            // open the file at the expected storage location, error if missing
            let path = value.join(format!("parameter_values/{dataset}_{name}_values.json"));
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
            if let Some(pf) = results.into_parameter_values() {
                // type of vector varies by parameter name
                match name {
                    ParameterName::GeoFips => {
                        for table in pf.iter() {
                            geo_fips.push(Integer::try_from(table)?);
                        }
                    }
                    ParameterName::LineCode => {
                        for table in pf.iter() {
                            line_code.push(Integer::try_from(table)?);
                        }
                    }
                    ParameterName::TableName => {
                        for table in pf.iter() {
                            table_name.push(TableName::try_from(table)?);
                        }
                    }
                    ParameterName::Year => {
                        for table in pf.iter() {
                            year.push(Year::try_from(table)?);
                        }
                    }
                    _ => unreachable!(),
                }
            } else {
                tracing::warn!("Results must be of type ParameterValues");
                return Err(Set::ParameterValuesMissing.into());
            }
        }
        if geo_fips.is_empty() || line_code.is_empty() || table_name.is_empty() || year.is_empty() {
            tracing::warn!("Value field is empty.");
            Err(Set::Empty.into())
        } else {
            let table = Self {
                geo_fips,
                line_code,
                table_name,
                year,
            };
            Ok(table)
        }
    }
}
