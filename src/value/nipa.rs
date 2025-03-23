use crate::{
    bea_data, BeaErr, BeaResponse, Dataset, Frequencies, Frequency, IoError, Millions,
    MillionsOptions, NipaRange, NipaRanges, ParameterName, ParameterValueTable,
    ParameterValueTableVariant, Queue, Request, SelectionKind, SerdeJson, Set, TableName,
};
use strum::IntoEnumIterator;

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
        let path = bea_data()?;
        let data = Nipa::try_from(&path)?;
        let mut queue = Vec::new();
        for params in data.iter() {
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
    show_millions: Option<MillionsOptions>,
    year_selection: SelectionKind,
    // index into nipa.table_name
    #[setters(skip)]
    table_index: usize,
    table_end: bool,
    // index into nipa.frequency
    #[setters(skip)]
    frequency_index: usize,
    #[setters(skip)]
    frequency_end: bool,
    millions: Vec<MillionsOptions>,
    millions_index: usize,
    millions_end: bool,
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
        let show_millions = None;
        let year_selection = SelectionKind::default();
        let table_index = 0;
        let frequency_index = 0;
        let millions = MillionsOptions::iter().collect::<Vec<MillionsOptions>>();
        let millions_index = 0;
        let millions_end = false;
        let mut nipa_iter = Self {
            nipa,
            frequency_options,
            show_millions,
            year_selection,
            table_index,
            table_end: false,
            frequency_index,
            frequency_end: false,
            millions,
            millions_index,
            millions_end,
            years: None,
            year_index: 0,
            year_end: false,
        };
        nipa_iter.set_years_by_frequency();
        nipa_iter
    }

    pub fn set_years_by_frequency(&mut self) {
        let table = self.nipa.table_name[self.table_index].to_string();
        if let Some(rng) = self.nipa.year.get(&table) {
            match self.nipa.frequency[self.frequency_index] {
                Frequency::Annual => {
                    if let Some(annual) = rng.annual() {
                        self.years = Some(annual.keys());
                    } else {
                        tracing::error!("Values for annual years not found.");
                        self.years = None;
                    }
                }
                Frequency::Monthly => {
                    if let Some(monthly) = rng.monthly() {
                        self.years = Some(monthly.keys());
                    } else {
                        tracing::error!("Values for monthly years not found.");
                        self.years = None;
                    }
                }
                Frequency::Quarterly => {
                    if let Some(quarterly) = rng.quarterly() {
                        self.years = Some(quarterly.keys());
                    } else {
                        tracing::error!("Values for quarterly years not found.");
                        self.years = None;
                    }
                }
            }
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
            // only needs to reset for individual, but doesn't hurt to just reset
            self.year_index = 0;
            self.year_end = false;
            match self.frequency_options {
                SelectionKind::All => self.frequency_end = true,
                SelectionKind::Individual => {
                    // more frequencies remain
                    if self.frequency_index < self.nipa.frequency.len() - 1 {
                        // increment the frequency index
                        self.frequency_index += 1;
                    } else {
                        // no more frequencies, move to next table
                        self.frequency_end = true;
                    }
                }
                // TODO: unimplemented
                SelectionKind::Multiple => {}
            }
        }

        // if years is single and frequency is all, update the years field
        // by incrementing the range index and pulling the next range into years
        // this ensures the values in years match the available years for the frequency

        // no more years for this table, move to next table
        if self.frequency_end {
            self.frequency_index = 0;
            self.frequency_end = false;
            if self.table_index < self.nipa.table_name.len() - 1 {
                // increment the table index
                self.table_index += 1;
            } else {
                self.table_end = true;
            }
        }

        // if the table has changed, the years may have changed
        // if frequency is individual, the index may have changes, so rebuild the years field
        // if the index has not changed, it is zero, and we want years to default to annual, so
        // this is the desired behavior when frequency is all.
        // update the years field based on the current frequency
        self.set_years_by_frequency();

        // no more tables for this ShowMillions option, move to next option
        if self.table_end {
            self.table_index = 0;
            self.table_end = false;
            if self.millions_index < self.nipa.show_millions.len() - 1 {
                self.millions_index += 1;
            } else {
                self.millions_end = true;
            }
        }

        // no more ShowMillions options, iteration is complete
        if self.millions_end {
            return None;
        }

        // empty parameters dictionary
        let mut params = std::collections::BTreeMap::new();
        // set show millions
        let (key, value) = if let Some(preference) = &self.show_millions {
            // user specified a value
            preference.params()
        } else {
            // iterate through available values
            self.millions[self.millions_index].params()
        };
        params.insert(key, value);
        // set table name
        let key = ParameterName::TableName.to_string();
        let table_name = self.nipa.table_name[self.table_index].to_string();
        params.insert(key, table_name.clone());
        // set frequency
        match self.frequency_options {
            // only a single key value pair needed
            SelectionKind::All => {
                // sets the value to all frequency options in a string vector
                let (key, value) = self.nipa.frequencies();
                params.insert(key, value);
            }
            // step through the available frequencies
            SelectionKind::Individual => {
                let (key, value) = self.nipa.frequency[self.frequency_index].params();
                params.insert(key, value);
            }
            // TODO: unimplemented
            SelectionKind::Multiple => {}
        }
        // set year values
        let key = ParameterName::Year.to_string();
        match self.year_selection {
            // single key value pair is sufficient
            SelectionKind::All => {
                let value = "ALL".to_string();
                params.insert(key, value);
                // advance to next table
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
                        // move to next year range or table
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
        let path = bea_data()?;
        let data = NiUnderlyingDetail::try_from(&path)?;
        let mut queue = Vec::new();
        for params in data.iter() {
            tracing::trace!("{params:#?}");
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
    frequency_options: SelectionKind,
    year_selection: SelectionKind,
    // index into data.table_name
    #[setters(skip)]
    table_index: usize,
    // index into data.frequency
    #[setters(skip)]
    frequency_index: usize,
    #[setters(skip)]
    frequency_end: bool,
    #[setters(skip)]
    years: Option<Vec<String>>,
    #[setters(skip)]
    year_index: usize,
    #[setters(skip)]
    year_end: bool,
}

impl<'a> NiUnderlyingDetailIterator<'a> {
    pub fn new(data: &'a NiUnderlyingDetail) -> Self {
        let frequency_options = SelectionKind::default();
        let year_selection = SelectionKind::default();
        let table_index = 0;
        let frequency_index = 0;
        let mut data_iter = Self {
            data,
            frequency_options,
            year_selection,
            table_index,
            frequency_index,
            frequency_end: false,
            years: None,
            year_index: 0,
            year_end: false,
        };
        data_iter.set_years_by_frequency();
        data_iter
    }

    pub fn set_years_by_frequency(&mut self) {
        let table = self.data.table_name[self.table_index].to_string();
        if let Some(rng) = self.data.year.get(&table) {
            match self.data.frequency[self.frequency_index] {
                Frequency::Annual => {
                    if let Some(annual) = rng.annual() {
                        self.years = Some(annual.keys());
                    } else {
                        tracing::error!("Values for annual years not found.");
                        self.years = None;
                    }
                }
                Frequency::Monthly => {
                    if let Some(monthly) = rng.monthly() {
                        self.years = Some(monthly.keys());
                    } else {
                        tracing::error!("Values for monthly years not found.");
                        self.years = None;
                    }
                }
                Frequency::Quarterly => {
                    if let Some(quarterly) = rng.quarterly() {
                        self.years = Some(quarterly.keys());
                    } else {
                        tracing::error!("Values for quarterly years not found.");
                        self.years = None;
                    }
                }
            }
        }
    }
}

impl Iterator for NiUnderlyingDetailIterator<'_> {
    type Item = std::collections::BTreeMap<String, String>;

    fn next(&mut self) -> Option<Self::Item> {
        // advance state
        if self.year_end {
            // no more year values in self.years
            // only needs to reset for individual, but doesn't hurt to just reset
            self.year_index = 0;
            self.year_end = false;
            match self.frequency_options {
                SelectionKind::All => self.frequency_end = true,
                SelectionKind::Individual => {
                    // more frequencies remain
                    if self.frequency_index < self.data.frequency.len() - 1 {
                        // increment the frequency index
                        self.frequency_index += 1;
                    } else {
                        // no more frequencies, move to next table
                        self.frequency_end = true;
                    }
                }
                // TODO: unimplemented
                SelectionKind::Multiple => {}
            }
        }

        // no more years for this table, move to next table
        if self.frequency_end {
            self.frequency_index = 0;
            self.frequency_end = false;
            if self.table_index < self.data.table_name.len() - 1 {
                // increment the table index
                self.table_index += 1;
            } else {
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
            SelectionKind::All => {
                let (key, value) = self.data.frequencies();
                params.insert(key, value);
            }
            // step through the available frequencies
            SelectionKind::Individual => {
                let (key, value) = self.data.frequency[self.frequency_index].params();
                params.insert(key, value);
            }
            // TODO: unimplemented
            SelectionKind::Multiple => {}
        }
        // set year values
        let key = ParameterName::Year.to_string();
        match self.year_selection {
            // single key value pair is sufficient
            SelectionKind::All => {
                let value = "ALL".to_string();
                params.insert(key, value);
                // move to next year range or table
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
                        // move to next year range or table
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
