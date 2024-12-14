use crate::{
    BeaErr, BeaResponse, Dataset, IoError, Metadata, ParameterFields, ParameterName,
    ParameterValueTable, Set,
};

use super::{Integer, TableName, Year};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    derive_more::From,
)]
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

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
    derive_more::AsRef,
    derive_more::AsMut,
)]
pub struct ValueSets(Vec<ValueSet>);

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    derive_new::new,
    derive_more::Deref,
    derive_more::DerefMut,
    derive_more::AsRef,
    derive_more::AsMut,
)]
pub struct ApiMetadata(Vec<Metadata>);

impl TryFrom<&std::path::PathBuf> for ApiMetadata {
    type Error = BeaErr;
    fn try_from(value: &std::path::PathBuf) -> Result<Self, Self::Error> {
        let dataset = Dataset::APIDatasetMetadata;
        let names = dataset.names();
        // empty vectors to store values
        let mut metadata = Vec::new();
        // For each parameter in dataset
        for name in names {
            // open the file at the expected storage location, error if missing
            let path = value.join(format!("values_{dataset}_{name}.json"));
            let file = IoError::open(path)?;
            // read the file to json
            let rdr = std::io::BufReader::new(file);
            let res: serde_json::Value = serde_json::from_reader(rdr)?;
            // parse to internal bea response format
            let data = BeaResponse::try_from(&res)?;
            let results = data.results();
            // access parameter values from response
            if let Some(pf) = results.into_parameter_values() {
                for table in pf.iter() {
                    match table {
                        ParameterValueTable::Metadata(item) => {
                            metadata.push(item.clone());
                        }
                        _ => unreachable!(),
                    }
                }
            } else {
                tracing::warn!("Results must be of type ParameterValues");
                return Err(Set::ParameterValuesMissing.into());
            }
        }
        if metadata.is_empty() {
            tracing::warn!("Value field is empty.");
            Err(Set::Empty.into())
        } else {
            let table = Self::new(metadata);
            Ok(table)
        }
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct FixedAssets {
    table_name: Vec<TableName>,
    year: Vec<Year>,
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct GdpByIndustry {
    frequency: Vec<ParameterFields>,
    industry: Vec<ParameterFields>,
    table_name: Vec<TableName>,
    year: Vec<Year>,
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
pub struct Iip {
    component: Vec<ParameterFields>,
    frequency: Vec<ParameterFields>,
    type_of_investment: Vec<ParameterFields>,
    year: Vec<Year>,
}

impl TryFrom<&std::path::PathBuf> for Iip {
    type Error = BeaErr;
    fn try_from(value: &std::path::PathBuf) -> Result<Self, Self::Error> {
        let dataset = Dataset::Iip;
        let names = dataset.names();
        // empty vectors to store values
        let mut component = Vec::new();
        let mut frequency = Vec::new();
        let mut type_of_investment = Vec::new();
        let mut year = Vec::new();
        // For each parameter in dataset
        for name in names {
            // open the file at the expected storage location, error if missing
            let path = value.join(format!("values_{dataset}_{name}.json"));
            let file = IoError::open(path)?;
            // read the file to json
            let rdr = std::io::BufReader::new(file);
            let res: serde_json::Value = serde_json::from_reader(rdr)?;
            // parse to internal bea response format
            let data = BeaResponse::try_from(&res)?;
            let results = data.results();
            // access parameter values from response
            if let Some(pf) = results.into_parameter_values() {
                // type of vector varies by parameter name
                match name {
                    ParameterName::Component => {
                        for table in pf.iter() {
                            match table {
                                ParameterValueTable::ParameterFields(pf) => {
                                    component.push(pf.clone());
                                }
                                _ => {
                                    return Err(Set::ParameterFieldsMissing.into());
                                }
                            }
                        }
                    }
                    ParameterName::Frequency => {
                        for table in pf.iter() {
                            match table {
                                ParameterValueTable::ParameterFields(pf) => {
                                    frequency.push(pf.clone());
                                }
                                _ => {
                                    return Err(Set::ParameterFieldsMissing.into());
                                }
                            }
                        }
                    }
                    ParameterName::TypeOfInvestment => {
                        for table in pf.iter() {
                            match table {
                                ParameterValueTable::ParameterFields(pf) => {
                                    type_of_investment.push(pf.clone());
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
        if component.is_empty()
            || frequency.is_empty()
            || type_of_investment.is_empty()
            || year.is_empty()
        {
            tracing::warn!("Value field is empty.");
            Err(Set::Empty.into())
        } else {
            let table = Self {
                component,
                frequency,
                type_of_investment,
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
pub struct InputOutput {
    table_id: Vec<Integer>,
    year: Vec<Year>,
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
            let path = value.join(format!("values_{dataset}_{name}.json"));
            let file = IoError::open(path)?;
            // read the file to json
            let rdr = std::io::BufReader::new(file);
            let res: serde_json::Value = serde_json::from_reader(rdr)?;
            // parse to internal bea response format
            let data = BeaResponse::try_from(&res)?;
            let results = data.results();
            // access parameter values from response
            if let Some(pf) = results.into_parameter_values() {
                // type of vector varies by parameter name
                match name {
                    ParameterName::TableID => {
                        for table in pf.iter() {
                            table_id.push(Integer::try_from(table)?);
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
pub struct Ita {
    area_or_country: Vec<ParameterFields>,
    frequency: Vec<ParameterFields>,
    indicator: Vec<ParameterFields>,
    year: Vec<Year>,
}

impl TryFrom<&std::path::PathBuf> for Ita {
    type Error = BeaErr;
    fn try_from(value: &std::path::PathBuf) -> Result<Self, Self::Error> {
        let dataset = Dataset::Ita;
        let names = dataset.names();
        // empty vectors to store values
        let mut area_or_country = Vec::new();
        let mut frequency = Vec::new();
        let mut indicator = Vec::new();
        let mut year = Vec::new();
        // For each parameter in dataset
        for name in names {
            // open the file at the expected storage location, error if missing
            let path = value.join(format!("values_{dataset}_{name}.json"));
            let file = IoError::open(path)?;
            // read the file to json
            let rdr = std::io::BufReader::new(file);
            let res: serde_json::Value = serde_json::from_reader(rdr)?;
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
                    ParameterName::Frequency => {
                        for table in pf.iter() {
                            match table {
                                ParameterValueTable::ParameterFields(pf) => {
                                    frequency.push(pf.clone());
                                }
                                _ => {
                                    return Err(Set::ParameterFieldsMissing.into());
                                }
                            }
                        }
                    }
                    ParameterName::Indicator => {
                        for table in pf.iter() {
                            match table {
                                ParameterValueTable::ParameterFields(pf) => {
                                    indicator.push(pf.clone());
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
            || frequency.is_empty()
            || indicator.is_empty()
            || year.is_empty()
        {
            tracing::warn!("Value field is empty.");
            Err(Set::Empty.into())
        } else {
            let table = Self {
                area_or_country,
                frequency,
                indicator,
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
            let path = value.join(format!("values_{dataset}_{name}.json"));
            let file = IoError::open(path)?;
            // read the file to json
            let rdr = std::io::BufReader::new(file);
            let res: serde_json::Value = serde_json::from_reader(rdr)?;
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
            let path = value.join(format!("values_{dataset}_{name}.json"));
            let file = IoError::open(path)?;
            // read the file to json
            let rdr = std::io::BufReader::new(file);
            let res: serde_json::Value = serde_json::from_reader(rdr)?;
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
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct Mne {
    classification: Vec<ParameterFields>,
    country: Vec<ParameterFields>,
    direction_of_investment: Vec<ParameterFields>,
    get_footnotes: Vec<ParameterFields>,
    industry: Vec<ParameterFields>,
    investment: Vec<ParameterFields>,
    nonbank_affiliates_only: Vec<ParameterFields>,
    onwership_level: Vec<ParameterFields>,
    parent_investment: Vec<ParameterFields>,
    seried_id: Vec<Integer>,
    state: Vec<ParameterFields>,
    year: Vec<Year>,
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct Nipa {
    frequency: Vec<ParameterFields>,
    show_millions: Vec<ParameterFields>,
    table_id: Vec<Integer>,
    table_name: Vec<TableName>,
    year: Vec<Year>,
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct NiUnderlyingDetail {
    frequency: Vec<ParameterFields>,
    table_id: Vec<Integer>,
    table_name: Vec<TableName>,
    year: Vec<Year>,
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
            let path = value.join(format!("values_{dataset}_{name}.json"));
            let file = IoError::open(path)?;
            // read the file to json
            let rdr = std::io::BufReader::new(file);
            let res: serde_json::Value = serde_json::from_reader(rdr)?;
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

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct UnderlyingGdpByIndustry {
    frequency: Vec<ParameterFields>,
    industry: Vec<ParameterFields>,
    table_id: Vec<Integer>,
    year: Vec<Year>,
}
