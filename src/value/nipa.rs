use crate::{
    BeaErr, BeaResponse, Dataset, EnvError, Frequencies, Frequency, FrequencyOptions, IoError,
    Millions, MillionsOptions, NipaRange, NipaRanges, ParameterName, ParameterValueTable,
    ParameterValueTableVariant, Queue, Request, SelectionKind, SerdeJson, Set, TableName,
    YearSelection,
};

#[derive(
    Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, derive_getters::Getters,
)]
pub struct Nipa {
    frequency: Frequencies,
    show_millions: Vec<Millions>,
    table_name: Vec<TableName>,
    year: NipaRanges,
}

impl Nipa {
    pub fn frequencies(&self) -> (String, String) {
        self.frequency.params()
    }

    pub fn iter(&self) -> NipaIterator {
        NipaIterator::new(self)
    }

    pub fn queue() -> Result<Queue, BeaErr> {
        let req = Request::Data;
        let mut app = req.init()?;
        let dataset = Dataset::Nipa;
        app.with_dataset(dataset);
        dotenvy::dotenv().ok();
        let bea_data = EnvError::from_env("BEA_DATA")?;
        let path = std::path::PathBuf::from(&bea_data);
        let data = Nipa::try_from(&path)?;
        let mut queue = Vec::new();
        for params in data.iter() {
            tracing::info!("{params:#?}");
            app.with_params(params.clone());
            queue.push(app.clone());
        }
        Ok(Queue::new(queue))
    }
}

impl TryFrom<&std::path::PathBuf> for Nipa {
    type Error = BeaErr;
    fn try_from(value: &std::path::PathBuf) -> Result<Self, Self::Error> {
        let dataset = Dataset::Nipa;
        let names = dataset.names();
        // empty vectors to store values
        let mut frequency = Vec::new();
        let mut show_millions = Vec::new();
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
                    ParameterName::Frequency => {
                        for table in pv.iter() {
                            frequency.push(Frequency::try_from(table)?);
                        }
                    }
                    ParameterName::ShowMillions => {
                        for table in pv.iter() {
                            show_millions.push(Millions::try_from(table)?);
                        }
                    }
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
                    ParameterName::TableID => {
                        tracing::trace!("Table ID has been deprecated by BEA.");
                    }
                    other => return Err(Set::ParameterNameMissing(other.to_string()).into()),
                }
            } else {
                tracing::warn!("Results must be of type ParameterValues");
                return Err(Set::ParameterValuesMissing.into());
            }
        }
        if frequency.is_empty()
            || show_millions.is_empty()
            || table_name.is_empty()
            || year.is_empty()
        {
            tracing::warn!("Value field is empty.");
            Err(Set::Empty.into())
        } else {
            let frequency = Frequencies::new(frequency);
            let year = NipaRanges::new(year);
            let table = Self {
                frequency,
                show_millions,
                table_name,
                year,
            };
            Ok(table)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, derive_setters::Setters)]
#[setters(prefix = "with_", borrow_self, into)]
pub struct NipaIterator<'a> {
    #[setters(skip)]
    nipa: &'a Nipa,
    frequency_options: SelectionKind,
    millions: MillionsOptions,
    year_selection: SelectionKind,
    // index into nipa.table_name
    #[setters(skip)]
    table_index: usize,
    // index into nipa.frequency
    #[setters(skip)]
    frequency_index: usize,
    #[setters(skip)]
    frequency_end: bool,
    // explicit ranges for individual frequencies
    #[setters(skip)]
    range: Option<Vec<Vec<String>>>,
    // index into range
    #[setters(skip)]
    range_index: usize,
    #[setters(skip)]
    range_end: bool,
    #[setters(skip)]
    years: Option<Vec<String>>,
    #[setters(skip)]
    year_index: usize,
    #[setters(skip)]
    year_end: bool,
}

impl<'a> NipaIterator<'a> {
    pub fn new(nipa: &'a Nipa) -> Self {
        let frequency_options = SelectionKind::default();
        let millions = MillionsOptions::default();
        let year_selection = SelectionKind::default();
        let table_index = 0;
        let frequency_index = 0;
        let first_table = nipa.table_name[table_index].name();
        // Create the range values by iteratoring over the NipaRange for the table name
        let range = match frequency_options {
            SelectionKind::All => {
                if let Some(rng) = nipa.year.get(first_table) {
                    let vec = rng.iter().collect::<Vec<Vec<String>>>();
                    Some(vec)
                } else {
                    None
                }
            }
            // Unimplemented
            SelectionKind::Multiple => None,
            SelectionKind::Individual => None,
        };
        let range_index = 0;
        // set years if needed
        let years = match year_selection {
            // only needed for individual
            SelectionKind::All => None,
            // Unimplemented
            SelectionKind::Multiple => None,
            // years depend on frequencies selected
            SelectionKind::Individual => {
                match frequency_options {
                    // pull from current range
                    SelectionKind::All => {
                        if let Some(rng) = &range {
                            Some(rng[range_index].clone())
                        } else {
                            tracing::error!("Range should be set if frequency is all.");
                            None
                        }
                    }
                    // only pull the current frequency range
                    SelectionKind::Individual => {
                        if let Some(rng) = nipa.year.get(first_table) {
                            // get the range for the current table
                            match nipa.frequency[frequency_index] {
                                Frequency::Annual => {
                                    if let Some(annual) = rng.annual() {
                                        Some(annual.keys())
                                    } else {
                                        tracing::error!("Values for annual years not found.");
                                        None
                                    }
                                }
                                Frequency::Monthly => {
                                    if let Some(monthly) = rng.monthly() {
                                        Some(monthly.keys())
                                    } else {
                                        tracing::error!("Values for monthly years not found.");
                                        None
                                    }
                                }
                                Frequency::Quarterly => {
                                    if let Some(quarterly) = rng.quarterly() {
                                        Some(quarterly.keys())
                                    } else {
                                        tracing::error!("Values for quarterly years not found.");
                                        None
                                    }
                                }
                            }
                        } else {
                            tracing::error!("Range for {first_table} not found.");
                            None
                        }
                    }
                    // Unimplemented
                    SelectionKind::Multiple => None,
                }
            }
        };
        Self {
            nipa,
            frequency_options,
            millions,
            year_selection,
            table_index,
            frequency_index,
            frequency_end: false,
            range,
            range_index,
            range_end: false,
            years,
            year_index: 0,
            year_end: false,
        }
    }
}

impl Iterator for NipaIterator<'_> {
    type Item = std::collections::BTreeMap<String, String>;

    fn next(&mut self) -> Option<Self::Item> {
        // advance state
        // primary driver year
        // secondary driver frequency
        if self.year_end {
            // no more year values in self.years
            match self.frequency_options {
                // get all ranges
                SelectionKind::All => {
                    if let Some(range) = &self.range {
                        if self.range_index < range.len() - 1 {
                            // increment the range index
                            self.range_index += 1;
                            self.year_index = 0;
                            self.year_end = false;
                        } else {
                            // no more ranges, move to next table
                            self.range_end = true;
                        }
                    }
                }
                // get all frequencies for a given table
                SelectionKind::Individual => {
                    // more frequencies remain
                    if self.frequency_index < self.nipa.frequency.len() - 1 {
                        // increment the frequency index
                        self.frequency_index += 1;
                        self.year_index = 0;
                        self.year_end = false;
                    } else {
                        // no more frequencies, move to next table
                        self.frequency_end = true;
                    }
                }
                SelectionKind::Multiple => {}
            }
        }

        // no more years for this table, move to next table
        if self.frequency_end || self.range_end {
            if self.table_index < self.nipa.table_name.len() - 1 {
                // increment the table index
                self.table_index += 1;
                // Reset dependent drivers
                self.frequency_index = 0;
                self.frequency_end = false;
                // range has been invalidated and must be rebuilt
                let table_name = self.nipa.table_name[self.table_index].name();
                self.range = match self.frequency_options {
                    SelectionKind::All => {
                        if let Some(rng) = self.nipa.year.get(table_name) {
                            self.range_index = 0;
                            self.range_end = false;
                            let vec = rng.iter().collect::<Vec<Vec<String>>>();
                            Some(vec)
                        } else {
                            // When do we hit this condition?
                            None
                        }
                    }
                    // Unimplemented
                    SelectionKind::Multiple => None,
                    SelectionKind::Individual => None,
                };
            } else {
                // no more tables, end iteration
                return None;
            }
        }

        // empty parameters dictionary
        let mut params = std::collections::BTreeMap::new();
        // set show millions
        let (key, value) = self.millions.params();
        params.insert(key, value);
        // set table name
        let key = ParameterName::TableName.to_string();
        let table_name = self.nipa.table_name[self.table_index].to_string();
        params.insert(key, table_name.clone());
        // set frequency
        match self.frequency_options {
            // only a single key value pair needed
            SelectionKind::All => {
                let (key, value) = self.nipa.frequencies();
                params.insert(key, value);
                self.frequency_end = true;
            }
            // step through the available frequencies
            SelectionKind::Individual => {
                let (key, value) = self.nipa.frequency[self.frequency_index].params();
                params.insert(key, value);
            }
            SelectionKind::Multiple => {}
        }
        // set year values
        let key = ParameterName::Year.to_string();
        match self.year_selection {
            // single key value pair is sufficient
            SelectionKind::All => {
                let value = "ALL".to_string();
                params.insert(key, value);
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
                        // move to next year range or table
                        self.year_end = true;
                    }
                } else {
                    tracing::warn!("Years should not be None.");
                    // move to next year range or table
                    self.year_end = true;
                }
            }
            SelectionKind::Multiple => {}
        }
        Some(params)
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, derive_getters::Getters,
)]
pub struct NiUnderlyingDetail {
    frequency: Frequencies,
    table_name: Vec<TableName>,
    year: NipaRanges,
}

impl NiUnderlyingDetail {
    pub fn frequencies(&self) -> (String, String) {
        self.frequency.params()
    }

    pub fn iter(&self) -> NiUnderlyingDetailIterator {
        NiUnderlyingDetailIterator::new(self)
    }

    pub fn queue() -> Result<Queue, BeaErr> {
        let req = Request::Data;
        let mut app = req.init()?;
        let dataset = Dataset::NIUnderlyingDetail;
        app.with_dataset(dataset);
        dotenvy::dotenv().ok();
        let bea_data = EnvError::from_env("BEA_DATA")?;
        let path = std::path::PathBuf::from(&bea_data);
        let data = NiUnderlyingDetail::try_from(&path)?;
        let mut queue = Vec::new();
        for params in data.iter() {
            tracing::info!("{params:#?}");
            app.with_params(params.clone());
            queue.push(app.clone());
        }
        Ok(Queue::new(queue))
    }
}

impl TryFrom<&std::path::PathBuf> for NiUnderlyingDetail {
    type Error = BeaErr;
    fn try_from(value: &std::path::PathBuf) -> Result<Self, Self::Error> {
        let dataset = Dataset::NIUnderlyingDetail;
        let names = dataset.names();
        // empty vectors to store values
        let mut frequency = Vec::new();
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
                    ParameterName::Frequency => {
                        for table in pv.iter() {
                            frequency.push(Frequency::try_from(table)?);
                        }
                    }
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
                    ParameterName::TableID => {
                        tracing::trace!("Table ID discontinued by BEA in 2018 (pg. 24)");
                    }
                    other => return Err(Set::ParameterNameMissing(other.to_string()).into()),
                }
            } else {
                tracing::warn!("Results must be of type ParameterValues");
                return Err(Set::ParameterValuesMissing.into());
            }
        }
        if frequency.is_empty() || table_name.is_empty() || year.is_empty() {
            tracing::warn!("Value field is empty.");
            Err(Set::Empty.into())
        } else {
            let frequency = Frequencies::new(frequency);
            let year = NipaRanges::new(year);
            let table = Self {
                frequency,
                table_name,
                year,
            };
            Ok(table)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, derive_setters::Setters)]
#[setters(prefix = "with_", borrow_self, into)]
pub struct NiUnderlyingDetailIterator<'a> {
    #[setters(skip)]
    data: &'a NiUnderlyingDetail,
    frequency_options: FrequencyOptions,
    year_selection: YearSelection,
    // index into data.table_name
    #[setters(skip)]
    table_index: usize,
    // index into data.frequency
    #[setters(skip)]
    frequency_index: usize,
    #[setters(skip)]
    frequency_end: bool,
    // explicit ranges for individual frequencies
    #[setters(skip)]
    range: Option<Vec<Vec<String>>>,
    // index into range
    #[setters(skip)]
    range_index: usize,
    #[setters(skip)]
    range_end: bool,
    #[setters(skip)]
    years: Option<Vec<String>>,
    #[setters(skip)]
    year_index: usize,
    #[setters(skip)]
    year_end: bool,
}

impl<'a> NiUnderlyingDetailIterator<'a> {
    pub fn new(data: &'a NiUnderlyingDetail) -> Self {
        let frequency_options = FrequencyOptions::default();
        let year_selection = YearSelection::default();
        let table_index = 0;
        let frequency_index = 0;
        let first_table = data.table_name[table_index].name();
        // Create the range values by iteratoring over the NipaRange for the table name
        let range = match frequency_options {
            FrequencyOptions::All => {
                if let Some(rng) = data.year.get(first_table) {
                    let vec = rng.iter().collect::<Vec<Vec<String>>>();
                    Some(vec)
                } else {
                    None
                }
            }
            FrequencyOptions::Individual => None,
        };
        let range_index = 0;
        // set years if needed
        let years = match year_selection {
            // only needed for individual
            YearSelection::All => None,
            // years depend on frequencies selected
            YearSelection::Yearly => {
                match frequency_options {
                    // pull from current range
                    FrequencyOptions::All => {
                        if let Some(rng) = &range {
                            Some(rng[range_index].clone())
                        } else {
                            tracing::error!("Range should be set if frequency is all.");
                            None
                        }
                    }
                    // only pull the current frequency range
                    FrequencyOptions::Individual => {
                        if let Some(rng) = data.year.get(first_table) {
                            // get the range for the current table
                            match data.frequency[frequency_index] {
                                Frequency::Annual => {
                                    if let Some(annual) = rng.annual() {
                                        Some(annual.keys())
                                    } else {
                                        tracing::error!("Values for annual years not found.");
                                        None
                                    }
                                }
                                Frequency::Monthly => {
                                    if let Some(monthly) = rng.annual() {
                                        Some(monthly.keys())
                                    } else {
                                        tracing::error!("Values for monthly years not found.");
                                        None
                                    }
                                }
                                Frequency::Quarterly => {
                                    if let Some(quarterly) = rng.annual() {
                                        Some(quarterly.keys())
                                    } else {
                                        tracing::error!("Values for quarterly years not found.");
                                        None
                                    }
                                }
                            }
                        } else {
                            tracing::error!("Range for {first_table} not found.");
                            None
                        }
                    }
                }
            }
        };
        Self {
            data,
            frequency_options,
            year_selection,
            table_index,
            frequency_index,
            frequency_end: false,
            range,
            range_index,
            range_end: false,
            years,
            year_index: 0,
            year_end: false,
        }
    }
}

impl Iterator for NiUnderlyingDetailIterator<'_> {
    type Item = std::collections::BTreeMap<String, String>;

    fn next(&mut self) -> Option<Self::Item> {
        // advance state
        if self.year_end {
            // no more year values in self.years
            match self.frequency_options {
                // get all ranges
                FrequencyOptions::All => {
                    if let Some(range) = &self.range {
                        if self.range_index < range.len() - 1 {
                            // increment the range index
                            self.range_index += 1;
                        } else {
                            // no more ranges, move to next table
                            self.range_end = true;
                        }
                    }
                }
                // get all frequencies for a given table
                FrequencyOptions::Individual => {
                    // more frequencies remain
                    if self.frequency_index < self.data.frequency.len() - 1 {
                        // increment the frequency index
                        self.frequency_index += 1;
                    } else {
                        // no more frequencies, move to next table
                        self.frequency_end = true;
                    }
                }
            }
        }

        // no more years for this table, move to next table
        if self.frequency_end || self.range_end {
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
        // set frequency
        match self.frequency_options {
            // only a single key value pair needed
            FrequencyOptions::All => {
                let (key, value) = self.data.frequencies();
                params.insert(key, value);
                self.frequency_end = true;
            }
            // step through the available frequencies
            FrequencyOptions::Individual => {
                let (key, value) = self.data.frequency[self.frequency_index].params();
                params.insert(key, value);
            }
        }
        // set year values
        let key = ParameterName::Year.to_string();
        match self.year_selection {
            // single key value pair is sufficient
            YearSelection::All => {
                let value = "ALL".to_string();
                params.insert(key, value);
            }
            // pull next year from years
            YearSelection::Yearly => {
                if let Some(years) = &self.years {
                    let value = years[self.year_index].clone();
                    params.insert(key, value);
                    // advance state
                    if self.year_index < years.len() - 1 {
                        // increment year index
                        self.year_index += 1;
                    } else {
                        // move to next year range or table
                        self.year_end = true;
                    }
                } else {
                    tracing::warn!("Years should not be None.");
                    // move to next year range or table
                    self.year_end = true;
                }
            }
        }
        Some(params)
    }
}
