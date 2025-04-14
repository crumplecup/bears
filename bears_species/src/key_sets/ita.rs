use crate::{
    AreaOrCountry, BeaErr, BeaResponse, Dataset, DeriveFromStr, Indicator, IoError, ItaFrequencies,
    ItaFrequency, KeyMissing, NotArray, NotObject, ParameterName, ParameterValueTable, SerdeJson,
    Set, Year, date_by_period, map_to_int, map_to_string, parse_year,
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
pub struct Ita {
    area_or_country: Vec<AreaOrCountry>,
    frequency: ItaFrequencies,
    indicator: Vec<Indicator>,
    year: Vec<Year>,
}

impl Ita {
    pub fn iter(&self) -> ItaIterator {
        ItaIterator::new(self)
    }

    // pub fn queue() -> Result<Queue, BeaErr> {
    //     let req = Request::Data;
    //     let mut app = req.init()?;
    //     let dataset = Dataset::Ita;
    //     app.with_dataset(dataset);
    //     dotenvy::dotenv().ok();
    //     let path = bea_data()?;
    //     let data = Ita::try_from(&path)?;
    //     let mut queue = Vec::new();
    //     for params in data.iter() {
    //         app.with_params(params.clone());
    //         queue.push(app.clone());
    //     }
    //     Ok(Queue::new(queue))
    // }
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
                                    if let Ok(aoc) = AreaOrCountry::from_str(pf.key()) {
                                        area_or_country.push(aoc);
                                    }
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
                    ParameterName::Indicator => {
                        for table in pf.iter() {
                            match table {
                                ParameterValueTable::ParameterFields(_) => {
                                    indicator.push(Indicator::try_from(table)?);
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
            let frequency = ItaFrequencies::new(frequency);
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

/// This iterator returns all values for indicator, frequency, and years regardless of whether the
/// user subsets the [`Ita`] struct that generates the iterator.
#[derive(Debug, Clone)]
pub struct ItaIterator<'a> {
    data: &'a Ita,
    aoc: std::slice::Iter<'a, AreaOrCountry>,
}

impl<'a> ItaIterator<'a> {
    pub fn new(data: &'a Ita) -> Self {
        let aoc = data.area_or_country().iter();
        Self { data, aoc }
    }
}

impl Iterator for ItaIterator<'_> {
    type Item = std::collections::BTreeMap<String, String>;

    fn next(&mut self) -> Option<Self::Item> {
        // empty parameters dictionary
        let mut params = std::collections::BTreeMap::new();

        // set aoc
        let aoc = self.aoc.next()?;
        let (key, value) = aoc.params();
        params.insert(key, value);

        // set indicator to all
        let key = "Indicator".to_owned();
        let value = "All".to_owned();
        params.insert(key, value);

        // sets the value to all frequency options in a string vector
        let (key, value) = self.data.frequency.all();
        params.insert(key, value);

        // set years to all
        let key = ParameterName::Year.to_string();
        let value = "All".to_owned();
        params.insert(key, value);

        Some(params)
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
pub struct ItaDatum {
    area_or_country: AreaOrCountry,
    cl_unit: String,
    data_value: Option<i64>,
    frequency: ItaFrequency,
    indicator: String,
    time_period: jiff::civil::Date,
    time_series_description: String,
    time_series_id: String,
    unit_mult: Option<i64>,
    year: jiff::civil::Date,
}

impl ItaDatum {
    pub fn read_json(m: &serde_json::Map<String, serde_json::Value>) -> Result<Self, BeaErr> {
        let area_or_country = map_to_string("AreaOrCountry", m)?;
        let area_or_country = AreaOrCountry::from_str(&area_or_country)
            .map_err(|e| DeriveFromStr::new(area_or_country, e, line!(), file!().to_owned()))?;
        tracing::trace!("area_or_country is {area_or_country}.");
        let cl_unit = map_to_string("CL_UNIT", m)?;
        tracing::trace!("cl_unit is {cl_unit}.");
        let data_value = match map_to_int("DataValue", m) {
            Ok(value) => Some(value),
            Err(_) => {
                let value = map_to_string("DataValue", m)?;
                if value.is_empty() {
                    None
                } else {
                    let error = KeyMissing::new(value, line!(), file!().to_owned());
                    return Err(error.into());
                }
            }
        };
        tracing::trace!("data_value is {data_value:?}.");
        let frequency = map_to_string("Frequency", m)?;
        let frequency = ItaFrequency::from_value(&frequency)?;
        tracing::trace!("frequency is {frequency}.");
        let indicator = map_to_string("Indicator", m)?;
        tracing::trace!("indicator is {indicator}.");
        let time_period = map_to_string("TimePeriod", m)?;
        let time_period = date_by_period(&time_period)?;
        tracing::trace!("time_period is {time_period}.");
        let time_series_description = map_to_string("TimeSeriesDescription", m)?;
        tracing::trace!("time_series_description is {time_series_description}.");
        let time_series_id = map_to_string("TimeSeriesId", m)?;
        tracing::trace!("time_series_id is {time_series_id}.");
        let unit_mult = map_to_int("UNIT_MULT", m)?;
        let unit_mult = match unit_mult {
            0 => None,
            num => Some(num),
        };
        tracing::trace!("unit_mult is {unit_mult:?}.");
        let year = map_to_string("Year", m)?;
        let year = parse_year(&year)?;
        Ok(Self {
            area_or_country,
            cl_unit,
            data_value,
            frequency,
            indicator,
            time_period,
            time_series_description,
            time_series_id,
            unit_mult,
            year,
        })
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
#[from(Vec<ItaDatum>)]
pub struct ItaData(Vec<ItaDatum>);

impl TryFrom<&serde_json::Value> for ItaData {
    type Error = BeaErr;
    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        tracing::trace!("Reading ItaData");
        match crate::data::result_to_data(value)? {
            serde_json::Value::Array(v) => {
                let mut data = Vec::new();
                for val in v {
                    match val {
                        serde_json::Value::Object(m) => {
                            let datum = ItaDatum::read_json(m)?;
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
