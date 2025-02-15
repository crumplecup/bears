use crate::{
    BeaErr, BeaResponse, Dataset, EnvError, IoError, NipaRange, NipaRanges, ParameterName,
    ParameterValueTable, ParameterValueTableVariant, Queue, Request, SelectionKind, SerdeJson, Set,
    TableName,
};

#[derive(
    Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, derive_getters::Getters,
)]
pub struct FixedAssets {
    table_name: Vec<TableName>,
    year: NipaRanges,
}

impl FixedAssets {
    pub fn iter(&self) -> FixedAssetsIterator {
        FixedAssetsIterator::new(self)
    }

    pub fn queue() -> Result<Queue, BeaErr> {
        let req = Request::Data;
        let mut app = req.init()?;
        let dataset = Dataset::FixedAssets;
        app.with_dataset(dataset);
        dotenvy::dotenv().ok();
        let bea_data = EnvError::from_env("BEA_DATA")?;
        let path = std::path::PathBuf::from(&bea_data);
        let data = FixedAssets::try_from(&path)?;
        let mut queue = Vec::new();
        for params in data.iter() {
            tracing::trace!("{params:#?}");
            app.with_params(params.clone());
            queue.push(app.clone());
        }
        Ok(Queue::new(queue))
    }
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

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, derive_setters::Setters)]
#[setters(prefix = "with_", borrow_self, into)]
pub struct FixedAssetsIterator<'a> {
    #[setters(skip)]
    data: &'a FixedAssets,
    year_selection: SelectionKind,
    // index into data.table_name
    #[setters(skip)]
    table_index: usize,
    #[setters(skip)]
    years: Option<Vec<String>>,
    #[setters(skip)]
    year_index: usize,
    #[setters(skip)]
    year_end: bool,
}

impl<'a> FixedAssetsIterator<'a> {
    pub fn new(data: &'a FixedAssets) -> Self {
        let year_selection = SelectionKind::default();
        let table_index = 0;
        let first_table = data.table_name[table_index].name();
        // set years if needed
        let years = match year_selection {
            // only needed for individual
            SelectionKind::All => None,
            SelectionKind::Individual => {
                if let Some(rng) = data.year.get(first_table) {
                    // get the range for the current table
                    if let Some(annual) = rng.annual() {
                        Some(annual.keys())
                    } else {
                        tracing::error!("Values for annual years not found.");
                        None
                    }
                } else {
                    tracing::error!("Range for {first_table} not found.");
                    None
                }
            }
            // TODO: unimplemented
            SelectionKind::Multiple => None,
        };
        Self {
            data,
            year_selection,
            table_index,
            years,
            year_index: 0,
            year_end: false,
        }
    }
}

impl Iterator for FixedAssetsIterator<'_> {
    type Item = std::collections::BTreeMap<String, String>;

    fn next(&mut self) -> Option<Self::Item> {
        // advance state
        if self.year_end {
            // no more years for this table, move to next table
            if self.table_index < self.data.table_name.len() - 1 {
                // increment the table index
                self.table_index += 1;
            } else {
                // no more tables, end iteration
                return None;
            }
        }

        // empty parameters dictionary
        let mut params = std::collections::BTreeMap::new();
        // set table name
        let key = ParameterName::TableName.to_string();
        let table_name = self.data.table_name[self.table_index].to_string();
        params.insert(key, table_name.clone());
        // set year values
        let key = ParameterName::Year.to_string();
        match self.year_selection {
            // single key value pair is sufficient
            SelectionKind::All => {
                let value = "ALL".to_string();
                params.insert(key, value);
                // move to next table
                self.year_end = true;
            }
            // pull next year from years
            SelectionKind::Individual => {
                if let Some(years) = &self.years {
                    let value = years[self.year_index].clone();
                    params.insert(key, value);
                    // advance state
                    if self.year_index < years.len() - 1 {
                        // increment year index
                        self.year_index += 1;
                    } else {
                        // move to next table
                        self.year_end = true;
                    }
                } else {
                    tracing::warn!("Years should not be None.");
                    // move to next year range or table
                    self.year_end = true;
                }
            }
            // TODO: unimplemented
            SelectionKind::Multiple => {}
        }
        Some(params)
    }
}
