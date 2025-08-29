use crate::{
    BeaErr, BeaResponse, Dataset, InputOutputCode, InputOutputTable, IoError, NotArray, NotObject,
    ParameterName, SerdeJson, Set, Year, map_to_float, map_to_int, map_to_string, parse_year,
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
pub struct InputOutput {
    table_id: Vec<InputOutputTable>,
    year: Vec<Year>,
}

impl InputOutput {
    pub fn iter(&self) -> InputOutputIterator<'_> {
        InputOutputIterator::new(self)
    }
}

impl TryFrom<&std::path::PathBuf> for InputOutput {
    type Error = BeaErr;
    fn try_from(value: &std::path::PathBuf) -> Result<Self, Self::Error> {
        let dataset = Dataset::InputOutput;
        let names = dataset.names();
        // empty vectors to store values
        let mut table_id = Vec::new();
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
                    ParameterName::TableID => {
                        for table in pf.iter() {
                            table_id.push(InputOutputTable::try_from(table)?);
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
        if table_id.is_empty() || year.is_empty() {
            tracing::warn!("Value field is empty.");
            Err(Set::Empty.into())
        } else {
            let table = Self { table_id, year };
            Ok(table)
        }
    }
}

/// This iterator returns all values for table id and years regardless of whether the
/// user subsets the [`InputOutput`] struct that generates the iterator.
#[derive(Debug, Clone)]
pub struct InputOutputIterator<'a> {
    // data: &'a InputOutput,
    table_ids: std::slice::Iter<'a, InputOutputTable>,
}

impl<'a> InputOutputIterator<'a> {
    pub fn new(data: &'a InputOutput) -> Self {
        let table_ids = data.table_id().iter();
        // Self { data, table_ids }
        Self { table_ids }
    }
}

impl Iterator for InputOutputIterator<'_> {
    type Item = std::collections::BTreeMap<String, String>;

    fn next(&mut self) -> Option<Self::Item> {
        // empty parameters dictionary
        let mut params = std::collections::BTreeMap::new();

        // set table id
        let table_id = self.table_ids.next()?;
        let (key, value) = table_id.params();
        params.insert(key, value);

        // set years to all
        let key = ParameterName::Year.to_string();
        let value = "ALL".to_owned();
        params.insert(key, value);

        Some(params)
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    derive_getters::Getters,
)]
pub struct InputOutputDatum {
    column_code: InputOutputCode,
    column_description: String,
    column_type: String,
    data_value: f64,
    note_ref: Option<String>,
    row_code: InputOutputCode,
    row_description: String,
    row_type: String,
    table_id: i64,
    year: jiff::civil::Date,
}

impl InputOutputDatum {
    pub fn read_json(m: &serde_json::Map<String, serde_json::Value>) -> Result<Self, BeaErr> {
        let column_code = map_to_string("ColCode", m)?;
        let column_code = InputOutputCode::from_value(&column_code)?;
        tracing::trace!("column_code is {}.", column_code.code());
        let column_description = map_to_string("ColDescr", m)?;
        tracing::trace!("column_description is {column_description}.");
        let column_type = map_to_string("ColType", m)?;
        tracing::trace!("column_type is {column_type}.");
        let data_value = map_to_float("DataValue", m)?;
        tracing::trace!("data_value is {data_value}.");
        let note_ref = map_to_string("NoteRef", m)?;
        let note_ref = if note_ref.is_empty() {
            None
        } else {
            Some(note_ref)
        };
        tracing::trace!("note_ref is {note_ref:?}.");
        let row_code = map_to_string("RowCode", m)?;
        let row_code = InputOutputCode::from_value(&row_code)?;
        tracing::trace!("row_code is {}.", row_code.code());
        let row_description = map_to_string("RowDescr", m)?;
        tracing::trace!("row_description is {row_description}.");
        let row_type = map_to_string("RowType", m)?;
        tracing::trace!("row_type is {row_type}.");
        let table_id = map_to_int("TableID", m)?;
        tracing::trace!("table_id is {table_id}.");
        let year = map_to_string("Year", m)?;
        let year = parse_year(&year)?;
        Ok(Self {
            column_code,
            column_description,
            column_type,
            data_value,
            note_ref,
            row_code,
            row_description,
            row_type,
            table_id,
            year,
        })
    }

    pub fn to_column_code(&self) -> (String, String) {
        (
            self.column_code().code(),
            self.column_description().to_owned(),
        )
    }

    pub fn to_row_code(&self) -> (String, String) {
        (self.row_code().code(), self.row_description().to_owned())
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
    derive_more::AsRef,
    derive_more::AsMut,
    derive_more::From,
)]
#[from(Vec<InputOutputDatum>)]
pub struct InputOutputData(Vec<InputOutputDatum>);

impl InputOutputData {
    pub fn column_codes(&self) -> std::collections::BTreeMap<String, String> {
        let mut params = std::collections::BTreeMap::new();
        self.iter()
            .map(|v| {
                let (key, value) = v.to_column_code();
                params.insert(key, value);
            })
            .for_each(drop);
        params
    }

    pub fn row_codes(&self) -> std::collections::BTreeMap<String, String> {
        let mut params = std::collections::BTreeMap::new();
        self.iter()
            .map(|v| {
                let (key, value) = v.to_row_code();
                params.insert(key, value);
            })
            .for_each(drop);
        params
    }
}

impl TryFrom<&serde_json::Value> for InputOutputData {
    type Error = BeaErr;
    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        tracing::trace!("Reading InputOutputData");
        match crate::data::result_to_data(value)? {
            serde_json::Value::Array(v) => {
                let mut data = Vec::new();
                for val in v {
                    match val {
                        serde_json::Value::Object(m) => {
                            let datum = InputOutputDatum::read_json(m)?;
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
