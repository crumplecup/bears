use bears_species::{
    ApiMetadata, BeaErr, BeaResponse, Dataset, FixedAssets, GdpByIndustry, Iip, InputOutput,
    Integer, IoError, Ita, Mne, NiUnderlyingDetail, Nipa, ParameterFields, ParameterName,
    ParameterValueTable, SerdeJson, Set, TableName, UnderlyingGdpByIndustry, Year,
};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, derive_more::From)]
pub enum ValueSet {
    #[from(ApiMetadata)]
    APIDatasetMetadata(ApiMetadata),
    #[from(FixedAssets)]
    FixedAssets(FixedAssets),
    #[from(Iip)]
    Iip(Iip),
    #[from(InputOutput)]
    InputOutput(InputOutput),
    #[from(IntlServTrade)]
    IntlServTrade(IntlServTrade),
    #[from(IntlServSta)]
    IntlServSTA(IntlServSta),
    #[from(Ita)]
    Ita(Ita),
    #[from(GdpByIndustry)]
    GDPbyIndustry(GdpByIndustry),
    #[from(Mne)]
    Mne(Mne),
    #[from(Nipa)]
    Nipa(Nipa),
    #[from(NiUnderlyingDetail)]
    NIUnderlyingDetail(NiUnderlyingDetail),
    #[from(Regional)]
    Regional(Regional),
    #[from(UnderlyingGdpByIndustry)]
    UnderlyingGDPbyIndustry(UnderlyingGdpByIndustry),
}

impl ValueSet {
    pub fn from_path(path: &std::path::PathBuf, dataset: Dataset) -> Result<Self, BeaErr> {
        match dataset {
            Dataset::APIDatasetMetadata => {
                let set = ApiMetadata::try_from(path)?;
                tracing::trace!("{dataset} values read.");
                tracing::trace!("{dataset} has {} Metadata values.", set.len());
                Ok(set.into())
            }
            Dataset::FixedAssets => {
                let set = FixedAssets::try_from(path)?;
                tracing::trace!("{dataset} values read.");
                tracing::trace!("{dataset} has {} TableName values.", set.table_name().len());
                tracing::trace!(
                    "{dataset} has {} tables of Year Ranges.",
                    set.year().keys().len()
                );
                Ok(set.into())
            }
            Dataset::GDPbyIndustry => {
                let set = GdpByIndustry::from_file(path)?;
                tracing::trace!("{dataset} values read.");
                Ok(set.into())
            }
            Dataset::Iip => {
                let set = Iip::try_from(path)?;
                tracing::trace!("{dataset} values read.");
                tracing::trace!("{dataset} has {} Component values.", set.component().len());
                tracing::trace!("{dataset} has {} Frequency values.", set.frequency().len());
                tracing::trace!(
                    "{dataset} has {} TypeOfInvestment values.",
                    set.type_of_investment().len()
                );
                tracing::trace!("{dataset} has {} Year values.", set.year().len());
                Ok(set.into())
            }
            Dataset::InputOutput => {
                let set = InputOutput::try_from(path)?;
                tracing::trace!("{dataset} values read.");
                tracing::trace!("{dataset} has {} TableID values.", set.table_id().len());
                tracing::trace!("{dataset} has {} Year values.", set.year().len());
                Ok(set.into())
            }
            Dataset::Ita => {
                let set = Ita::try_from(path)?;
                tracing::trace!("{dataset} values read.");
                tracing::trace!(
                    "{dataset} has {} AreaOrCounty values.",
                    set.area_or_country().len()
                );
                tracing::trace!("{dataset} has {} Frequency values.", set.frequency().len());
                tracing::trace!("{dataset} has {} Indicator values.", set.indicator().len());
                tracing::trace!("{dataset} has {} Year values.", set.year().len());
                Ok(set.into())
            }
            Dataset::IntlServSTA => {
                let set = IntlServSta::try_from(path)?;
                tracing::trace!("{dataset} values read.");
                tracing::trace!(
                    "{dataset} has {} AreaOrCounty values.",
                    set.area_or_country().len()
                );
                tracing::trace!("{dataset} has {} Channel values.", set.channel().len());
                tracing::trace!(
                    "{dataset} has {} Destination values.",
                    set.destination().len()
                );
                tracing::trace!("{dataset} has {} Industry values.", set.industry().len());
                tracing::trace!("{dataset} has {} Year values.", set.year().len());
                Ok(set.into())
            }
            Dataset::IntlServTrade => {
                let set = IntlServTrade::try_from(path)?;
                tracing::trace!("{dataset} values read.");
                tracing::trace!(
                    "{dataset} has {} Affiliation values.",
                    set.affiliation().len()
                );
                tracing::trace!(
                    "{dataset} has {} AreaOrCounty values.",
                    set.area_or_country().len()
                );
                tracing::trace!(
                    "{dataset} has {} TradeDirection values.",
                    set.trade_direction().len()
                );
                tracing::trace!(
                    "{dataset} has {} TypeOfService values.",
                    set.type_of_service().len()
                );
                tracing::trace!("{dataset} has {} Year values.", set.year().len());
                Ok(set.into())
            }
            Dataset::Nipa => {
                let set = Nipa::try_from(path)?;
                tracing::trace!("{dataset} values read.");
                tracing::trace!("{dataset} has {} Frequency values.", set.frequency().len());
                tracing::trace!(
                    "{dataset} has {} ShowMillions values.",
                    set.show_millions().len()
                );
                tracing::trace!("{dataset} has {} TableName values.", set.table_name().len());
                tracing::trace!(
                    "{dataset} has {} tables of Year Ranges.",
                    set.year().keys().len()
                );
                Ok(set.into())
            }
            Dataset::Mne => {
                let set = Mne::try_from(path)?;
                tracing::trace!("{dataset} values read.");
                tracing::trace!(
                    "{dataset} has {} Classification values.",
                    set.classification().len()
                );
                tracing::trace!("{dataset} has {} Country values.", set.country().len());
                tracing::trace!(
                    "{dataset} has {} DirectionOfInvestment values.",
                    set.direction_of_investment().len()
                );
                tracing::trace!(
                    "{dataset} has {} GetFootnotes values.",
                    set.get_footnotes().len()
                );
                tracing::trace!("{dataset} has {} Industry values.", set.industry().len());
                tracing::trace!(
                    "{dataset} has {} Investment values.",
                    set.investment().len()
                );
                tracing::trace!(
                    "{dataset} has {} NonbankAffiliatesOnly values.",
                    set.nonbank_affiliates_only().len()
                );
                tracing::trace!(
                    "{dataset} has {} OwnershipLevel values.",
                    set.ownership_level().len()
                );
                tracing::trace!(
                    "{dataset} has {} ParentInvestment values.",
                    set.parent_investment().len()
                );
                tracing::trace!("{dataset} has {} YearOptions values.", set.year().len());
                Ok(set.into())
            }
            Dataset::NIUnderlyingDetail => {
                let set = NiUnderlyingDetail::try_from(path)?;
                tracing::trace!("{dataset} values read.");
                tracing::trace!("{dataset} has {} Frequency values.", set.frequency().len());
                tracing::trace!("{dataset} has {} TableName values.", set.table_name().len());
                tracing::trace!(
                    "{dataset} has {} tables of Year Ranges.",
                    set.year().keys().len()
                );
                Ok(set.into())
            }
            Dataset::Regional => {
                let set = Regional::try_from(path)?;
                tracing::trace!("{dataset} values read.");
                tracing::trace!("{dataset} has {} GeoFips values.", set.geo_fips().len());
                tracing::trace!("{dataset} has {} LineCode values.", set.line_code().len());
                tracing::trace!("{dataset} has {} TableName values.", set.table_name().len());
                tracing::trace!("{dataset} has {} Year values.", set.year().len());
                Ok(set.into())
            }
            Dataset::UnderlyingGDPbyIndustry => {
                let set = UnderlyingGdpByIndustry::from_file(path)?;
                tracing::trace!("{dataset} values read.");
                Ok(set.into())
            }
        }
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
    derive_more::AsRef,
    derive_more::AsMut,
)]
pub struct ValueSets(Vec<ValueSet>);

impl ValueSets {
    pub fn from_path(path: &std::path::PathBuf, datasets: &[Dataset]) -> Result<Self, BeaErr> {
        let mut sets = Vec::new();
        for dataset in datasets {
            let set = ValueSet::from_path(path, *dataset)?;
            sets.push(set);
        }
        Ok(Self(sets))
    }
}

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
pub struct IntlServSta {
    area_or_country: Vec<ParameterFields>,
    channel: Vec<ParameterFields>,
    destination: Vec<ParameterFields>,
    industry: Vec<ParameterFields>,
    year: Vec<Year>,
}

impl TryFrom<&std::path::PathBuf> for IntlServSta {
    type Error = BeaErr;
    fn try_from(value: &std::path::PathBuf) -> Result<Self, Self::Error> {
        let dataset = Dataset::IntlServSTA;
        let names = dataset.names();
        // empty vectors to store values
        let mut area_or_country = Vec::new();
        let mut channel = Vec::new();
        let mut destination = Vec::new();
        let mut industry = Vec::new();
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
                    ParameterName::AreaOrCountry => {
                        for table in pf.iter() {
                            match table {
                                ParameterValueTable::ParameterFields(pf) => {
                                    area_or_country.push(pf.clone());
                                }
                                _ => {
                                    return Err(Set::ParameterFieldsMissing.into());
                                }
                            }
                        }
                    }
                    ParameterName::Channel => {
                        for table in pf.iter() {
                            match table {
                                ParameterValueTable::ParameterFields(pf) => {
                                    channel.push(pf.clone());
                                }
                                _ => {
                                    return Err(Set::ParameterFieldsMissing.into());
                                }
                            }
                        }
                    }
                    ParameterName::Destination => {
                        for table in pf.iter() {
                            match table {
                                ParameterValueTable::ParameterFields(pf) => {
                                    destination.push(pf.clone());
                                }
                                _ => {
                                    return Err(Set::ParameterFieldsMissing.into());
                                }
                            }
                        }
                    }
                    ParameterName::Industry => {
                        for table in pf.iter() {
                            match table {
                                ParameterValueTable::ParameterFields(pf) => {
                                    industry.push(pf.clone());
                                }
                                _ => {
                                    return Err(Set::ParameterFieldsMissing.into());
                                }
                            }
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
        if area_or_country.is_empty()
            || channel.is_empty()
            || destination.is_empty()
            || industry.is_empty()
            || year.is_empty()
        {
            tracing::warn!("Value field is empty.");
            Err(Set::Empty.into())
        } else {
            let table = Self {
                area_or_country,
                channel,
                destination,
                industry,
                year,
            };
            Ok(table)
        }
    }
}

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
pub struct IntlServTrade {
    affiliation: Vec<ParameterFields>,
    area_or_country: Vec<ParameterFields>,
    trade_direction: Vec<ParameterFields>,
    type_of_service: Vec<ParameterFields>,
    year: Vec<Year>,
}

impl TryFrom<&std::path::PathBuf> for IntlServTrade {
    type Error = BeaErr;
    fn try_from(value: &std::path::PathBuf) -> Result<Self, Self::Error> {
        let dataset = Dataset::IntlServTrade;
        let names = dataset.names();
        // empty vectors to store values
        let mut area_or_country = Vec::new();
        let mut affiliation = Vec::new();
        let mut trade_direction = Vec::new();
        let mut type_of_service = Vec::new();
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
                    ParameterName::Affiliation => {
                        for table in pf.iter() {
                            match table {
                                ParameterValueTable::ParameterFields(pf) => {
                                    affiliation.push(pf.clone());
                                }
                                _ => {
                                    return Err(Set::ParameterFieldsMissing.into());
                                }
                            }
                        }
                    }
                    ParameterName::AreaOrCountry => {
                        for table in pf.iter() {
                            match table {
                                ParameterValueTable::ParameterFields(pf) => {
                                    area_or_country.push(pf.clone());
                                }
                                _ => {
                                    return Err(Set::ParameterFieldsMissing.into());
                                }
                            }
                        }
                    }
                    ParameterName::TradeDirection => {
                        for table in pf.iter() {
                            match table {
                                ParameterValueTable::ParameterFields(pf) => {
                                    trade_direction.push(pf.clone());
                                }
                                _ => {
                                    return Err(Set::ParameterFieldsMissing.into());
                                }
                            }
                        }
                    }
                    ParameterName::TypeOfService => {
                        for table in pf.iter() {
                            match table {
                                ParameterValueTable::ParameterFields(pf) => {
                                    type_of_service.push(pf.clone());
                                }
                                _ => {
                                    return Err(Set::ParameterFieldsMissing.into());
                                }
                            }
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
        if area_or_country.is_empty()
            || affiliation.is_empty()
            || trade_direction.is_empty()
            || type_of_service.is_empty()
            || year.is_empty()
        {
            tracing::warn!("Value field is empty.");
            Err(Set::Empty.into())
        } else {
            let table = Self {
                affiliation,
                area_or_country,
                trade_direction,
                type_of_service,
                year,
            };
            Ok(table)
        }
    }
}

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
