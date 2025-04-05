use crate::{
    BeaErr, BeaResponse, Component, Dataset, DeriveFromStr, IoError, ItaFrequencies, ItaFrequency,
    ParameterFields, ParameterName, ParameterValueTable, SerdeJson, Set, Year,
};
use std::str::FromStr;

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
    component: Vec<Component>,
    frequency: ItaFrequencies,
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
                    ParameterName::Component => {
                        for table in pf.iter() {
                            match table {
                                ParameterValueTable::ParameterFields(pf) => {
                                    let c = Component::from_str(pf.key()).map_err(|e| {
                                        DeriveFromStr::new(
                                            pf.key().to_owned(),
                                            e,
                                            line!(),
                                            file!().to_owned(),
                                        )
                                    })?;
                                    component.push(c);
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
                                ParameterValueTable::ParameterFields(_) => {
                                    frequency.push(ItaFrequency::try_from(table)?);
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
            let frequency = ItaFrequencies::new(frequency);
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
