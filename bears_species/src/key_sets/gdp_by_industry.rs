use crate::{
    BeaErr, BeaResponse, Dataset, Frequencies, Frequency, Integer, IoError, ParameterFields,
    ParameterName, ParameterValueTable, SelectionKind, SerdeJson, Set, Year,
};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    derive_getters::Getters,
    derive_new::new,
)]
pub struct GdpByIndustry {
    frequency: Frequencies,
    // BTreeMap of table ids to industry names
    industry: std::collections::BTreeMap<Integer, Vec<ParameterFields>>,
    table_id: Vec<Integer>,
    // BTreeMap of table ids to years
    year: std::collections::BTreeMap<Integer, Vec<Year>>,
}

impl GdpByIndustry {
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self, BeaErr> {
        let frequency = Self::frequencies();
        let industry = Self::read_industry(&path)?;
        let table_id = Self::read_table_id(&path)?;
        let year = Self::read_year(&path)?;
        Ok(Self::new(frequency, industry, table_id, year))
    }

    pub fn frequencies() -> Frequencies {
        vec![Frequency::Annual, Frequency::Quarterly].into()
    }

    pub fn iter(&self) -> GdpByIndustryIterator<'_> {
        GdpByIndustryIterator::new(self)
    }

    // pub fn queue() -> Result<Queue, BeaErr> {
    //     let req = Request::Data;
    //     let mut app = req.init()?;
    //     let dataset = Dataset::GDPbyIndustry;
    //     app.with_dataset(dataset);
    //     dotenvy::dotenv().ok();
    //     let path = bea_data()?;
    //     let data = GdpByIndustry::try_from(&path)?;
    //     let mut queue = Vec::new();
    //     for params in data.iter() {
    //         tracing::trace!("{params:#?}");
    //         app.with_params(params.clone());
    //         queue.push(app.clone());
    //     }
    //     Ok(Queue::new(queue))
    // }

    pub fn read_industry<P: AsRef<std::path::Path>>(
        path: P,
    ) -> Result<std::collections::BTreeMap<Integer, Vec<ParameterFields>>, BeaErr> {
        let path = path.as_ref();
        let table_id = Self::read_table_id(path)?;
        let dataset = Dataset::GDPbyIndustry;
        // start with table_id because it is a precondition for other parameter values
        let name = ParameterName::Industry;
        // year values vary by table id
        let path = path.join(format!("parameter_values/{dataset}_{name}"));
        let mut industries = std::collections::BTreeMap::new();
        for id in table_id {
            // open the file at the expected storage location, error if missing
            let path = path.join(format!(
                "{dataset}_{name}_byTableId_{}_values.json",
                id.value()
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
            let mut industry = Vec::new();
            // access parameter values from response
            if let Some(pv) = results.into_parameter_values() {
                for table in pv.iter() {
                    match table {
                        ParameterValueTable::ParameterFields(pf) => {
                            industry.push(pf.clone());
                        }
                        _ => {
                            return Err(Set::ParameterFieldsMissing.into());
                        }
                    }
                }
                tracing::info!("{dataset} contains {} {name} values.", industry.len());
                industries.insert(id, industry);
            } else {
                tracing::warn!("Results must be of type ParameterValues");
                return Err(Set::ParameterValuesMissing.into());
            }
        }
        Ok(industries)
    }

    pub fn read_table_id<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<Integer>, BeaErr> {
        let path = path.as_ref();
        let dataset = Dataset::GDPbyIndustry;
        // start with table_id because it is a precondition for other parameter values
        let name = ParameterName::TableID;
        // open the file at the expected storage location, error if missing
        let path = path.join(format!(
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

        let mut table_id = Vec::new();
        // access parameter values from response
        if let Some(pv) = results.into_parameter_values() {
            for table in pv.iter() {
                table_id.push(Integer::try_from(table)?);
            }
            tracing::info!("{dataset} contains {} {name} values.", table_id.len());
            Ok(table_id)
        } else {
            tracing::warn!("Results must be of type ParameterValues");
            Err(Set::ParameterValuesMissing.into())
        }
    }

    pub fn read_year<P: AsRef<std::path::Path>>(
        path: P,
    ) -> Result<std::collections::BTreeMap<Integer, Vec<Year>>, BeaErr> {
        let path = path.as_ref();
        let table_id = Self::read_table_id(path)?;
        let dataset = Dataset::GDPbyIndustry;
        // start with table_id because it is a precondition for other parameter values
        let name = ParameterName::Year;
        // year values vary by table id
        let path = path.join(format!("parameter_values/{dataset}_{name}"));
        let mut years = std::collections::BTreeMap::new();
        for id in table_id {
            // open the file at the expected storage location, error if missing
            let path = path.join(format!(
                "{dataset}_{name}_byTableId_{}_values.json",
                id.value()
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
            let mut year = Vec::new();
            // access parameter values from response
            if let Some(pv) = results.into_parameter_values() {
                for table in pv.iter() {
                    year.push(Year::try_from(table)?);
                }
                tracing::info!("{dataset} contains {} {name} values.", year.len());
                years.insert(id, year);
            } else {
                tracing::warn!("Results must be of type ParameterValues");
                return Err(Set::ParameterValuesMissing.into());
            }
        }
        Ok(years)
    }
}

impl TryFrom<&std::path::PathBuf> for GdpByIndustry {
    type Error = BeaErr;
    fn try_from(value: &std::path::PathBuf) -> Result<Self, Self::Error> {
        Self::from_file(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, derive_setters::Setters)]
#[setters(prefix = "with_", borrow_self, into)]
pub struct GdpByIndustryIterator<'a> {
    #[setters(skip)]
    data: &'a GdpByIndustry,
    frequency_options: SelectionKind,
    industry_options: SelectionKind,
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
    industry_index: usize,
    #[setters(skip)]
    industry_end: bool,
    #[setters(skip)]
    years: Option<Vec<String>>,
    #[setters(skip)]
    year_index: usize,
    #[setters(skip)]
    year_end: bool,
}

impl<'a> GdpByIndustryIterator<'a> {
    pub fn new(data: &'a GdpByIndustry) -> Self {
        let frequency_options = SelectionKind::default();
        let industry_options = SelectionKind::default();
        let year_selection = SelectionKind::default();
        let table_index = 0;
        let frequency_index = 0;
        let industry_index = 0;
        let industry_end = false;
        Self {
            data,
            frequency_options,
            industry_options,
            year_selection,
            table_index,
            frequency_index,
            frequency_end: false,
            industry_index,
            industry_end,
            years: None,
            year_index: 0,
            year_end: false,
        }
    }
}

impl Iterator for GdpByIndustryIterator<'_> {
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
            let table_id = &self.data.table_id[self.table_index];
            if let Some(industries) = self.data.industry.get(table_id) {
                if self.industry_index < industries.len() - 1 {
                    // increment the index
                    self.industry_index += 1;
                } else {
                    self.industry_end = true;
                }
            } else {
                tracing::error!("Industries should not be None.");
                self.industry_end = true;
            }
        }

        // no more years for this table, move to next table
        if self.industry_end {
            self.industry_index = 0;
            self.industry_end = false;
            if self.table_index < self.data.table_id.len() - 1 {
                // increment the table index
                self.table_index += 1;
            } else {
                // no more tables, end iterator
                return None;
            }
        }

        // empty parameters dictionary
        let mut params = std::collections::BTreeMap::new();
        // set table id
        let key = ParameterName::TableID.to_string();
        let table_id = self.data.table_id[self.table_index].clone();
        params.insert(key, table_id.to_string());

        // set frequency
        match self.frequency_options {
            // only a single key value pair needed
            SelectionKind::All => {
                let (key, value) = self.data.frequency.params();
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

        // set industry
        let key = ParameterName::Industry.to_string();
        match self.industry_options {
            SelectionKind::All => {
                let value = "ALL".to_string();
                params.insert(key, value);
                // move to next set
                self.industry_end = true;
            }
            SelectionKind::Individual => {
                if let Some(industries) = self.data.industry.get(&table_id) {
                    params.insert(key, industries[self.industry_index].key().to_owned());
                } else {
                    tracing::warn!("Industry should not be None.");
                    // move to next set
                    self.industry_end = true;
                }
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
                if let Some(years) = self.data.year.get(&table_id) {
                    params.insert(key, years[self.year_index].to_string());
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
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    derive_getters::Getters,
    derive_new::new,
)]
pub struct UnderlyingGdpByIndustry {
    frequency: Frequencies,
    industry: std::collections::HashMap<Integer, Vec<ParameterFields>>,
    table_id: Vec<Integer>,
    year: std::collections::HashMap<Integer, Vec<Year>>,
}

impl UnderlyingGdpByIndustry {
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self, BeaErr> {
        let frequency = Self::frequencies();
        let industry = Self::read_industry(&path)?;
        let table_id = Self::read_table_id(&path)?;
        let year = Self::read_year(&path)?;
        Ok(Self::new(frequency, industry, table_id, year))
    }

    pub fn frequencies() -> Frequencies {
        vec![Frequency::Annual].into()
    }

    pub fn read_industry<P: AsRef<std::path::Path>>(
        path: P,
    ) -> Result<std::collections::HashMap<Integer, Vec<ParameterFields>>, BeaErr> {
        let path = path.as_ref();
        let table_id = Self::read_table_id(path)?;
        let dataset = Dataset::UnderlyingGDPbyIndustry;
        // start with table_id because it is a precondition for other parameter values
        let name = ParameterName::Industry;
        // year values vary by table id
        let path = path.join(format!("parameter_values/{dataset}_{name}"));
        let mut industries = std::collections::HashMap::new();
        for id in table_id {
            // open the file at the expected storage location, error if missing
            let path = path.join(format!(
                "{dataset}_{name}_byTableId_{}_values.json",
                id.value()
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
            let mut industry = Vec::new();
            // access parameter values from response
            if let Some(pv) = results.into_parameter_values() {
                for table in pv.iter() {
                    match table {
                        ParameterValueTable::ParameterFields(pf) => {
                            industry.push(pf.clone());
                        }
                        _ => {
                            return Err(Set::ParameterFieldsMissing.into());
                        }
                    }
                }
                tracing::info!("{dataset} contains {} {name} values.", industry.len());
                industries.insert(id, industry);
            } else {
                tracing::warn!("Results must be of type ParameterValues");
                return Err(Set::ParameterValuesMissing.into());
            }
        }
        Ok(industries)
    }

    pub fn read_table_id<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<Integer>, BeaErr> {
        let path = path.as_ref();
        let dataset = Dataset::UnderlyingGDPbyIndustry;
        // start with table_id because it is a precondition for other parameter values
        let name = ParameterName::TableID;
        // open the file at the expected storage location, error if missing
        let path = path.join(format!(
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

        let mut table_id = Vec::new();
        // access parameter values from response
        if let Some(pv) = results.into_parameter_values() {
            for table in pv.iter() {
                table_id.push(Integer::try_from(table)?);
            }
            tracing::info!("{dataset} contains {} {name} values.", table_id.len());
            Ok(table_id)
        } else {
            tracing::warn!("Results must be of type ParameterValues");
            Err(Set::ParameterValuesMissing.into())
        }
    }

    pub fn read_year<P: AsRef<std::path::Path>>(
        path: P,
    ) -> Result<std::collections::HashMap<Integer, Vec<Year>>, BeaErr> {
        let path = path.as_ref();
        let table_id = Self::read_table_id(path)?;
        let dataset = Dataset::UnderlyingGDPbyIndustry;
        // start with table_id because it is a precondition for other parameter values
        let name = ParameterName::Year;
        // year values vary by table id
        let path = path.join(format!("parameter_values/{dataset}_{name}"));
        let mut years = std::collections::HashMap::new();
        for id in table_id {
            // open the file at the expected storage location, error if missing
            let path = path.join(format!(
                "{dataset}_{name}_byTableId_{}_values.json",
                id.value()
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
            let mut year = Vec::new();
            // access parameter values from response
            if let Some(pv) = results.into_parameter_values() {
                for table in pv.iter() {
                    year.push(Year::try_from(table)?);
                }
                tracing::info!("{dataset} contains {} {name} values.", year.len());
                years.insert(id, year);
            } else {
                tracing::warn!("Results must be of type ParameterValues");
                return Err(Set::ParameterValuesMissing.into());
            }
        }
        Ok(years)
    }
}
