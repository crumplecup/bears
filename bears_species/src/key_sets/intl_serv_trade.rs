use crate::{
    BeaErr, BeaResponse, Dataset, IoError, ParameterFields, ParameterName, ParameterValueTable,
    SerdeJson, Set, Year,
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
