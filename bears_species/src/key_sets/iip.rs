use crate::{
    BeaErr, BeaResponse, Component, Data, Dataset, DatasetMissing, DeriveFromStr, Investment,
    IoError, ItaFrequencies, ItaFrequency, NotArray, NotObject, ParameterName, ParameterValueTable,
    SelectionKind, SerdeJson, Set, VariantMissing, Year, date_by_period, map_to_int, map_to_string,
    parse_year,
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
    type_of_investment: Vec<Investment>,
    year: Vec<Year>,
}

impl Iip {
    pub fn iter(&self) -> IipIterator<'_> {
        IipIterator::new(self)
    }
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
                                    let i = Investment::from_str(pf.key()).map_err(|e| {
                                        DeriveFromStr::new(
                                            pf.key().to_owned(),
                                            e,
                                            line!(),
                                            file!().to_owned(),
                                        )
                                    })?;
                                    type_of_investment.push(i);
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

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, derive_setters::Setters)]
#[setters(prefix = "with_", borrow_self, into)]
pub struct IipIterator<'a> {
    #[setters(skip)]
    data: &'a Iip,
    year_selection: SelectionKind,
    #[setters(skip)]
    component_index: usize,
    #[setters(skip)]
    toi_index: usize,
    #[setters(skip)]
    years: Option<Vec<String>>,
    #[setters(skip)]
    year_index: usize,
    #[setters(skip)]
    year_end: bool,
}

impl<'a> IipIterator<'a> {
    pub fn new(data: &'a Iip) -> Self {
        let year_selection = SelectionKind::default();
        let component_index = 0;
        let toi_index = 0;
        let years = None;
        let year_index = 0;
        let year_end = false;
        Self {
            data,
            year_selection,
            component_index,
            toi_index,
            years,
            year_index,
            year_end,
        }
    }
}

impl Iterator for IipIterator<'_> {
    type Item = std::collections::BTreeMap<String, String>;

    fn next(&mut self) -> Option<Self::Item> {
        // advance state
        if self.year_end {
            // // no more years for this component, move to next
            // if self.component_index < self.data.component.len() - 1 {
            //     // increment the index
            //     self.component_index += 1;
            // } else {
            //     // no more components, end iteration
            //     return None;
            // }
            // no more years for this investment, move to next
            if self.toi_index < self.data.type_of_investment.len() - 1 {
                // increment the index
                self.toi_index += 1;
            } else {
                // no more investments, end iteration
                return None;
            }
        }

        // empty parameters dictionary
        let mut params = std::collections::BTreeMap::new();
        // set component
        let key = ParameterName::Component.to_string();
        params.insert(key, "ALL".to_string());
        // let component = self.data.component[self.component_index].to_string();
        // params.insert(key, component);

        // set frequency
        let key = ParameterName::Frequency.to_string();
        params.insert(key, "ALL".to_string());

        // set type of investment
        let key = ParameterName::TypeOfInvestment.to_string();
        // params.insert(key, "ALL".to_string());
        let toi = self.data.type_of_investment[self.toi_index].to_string();
        params.insert(key, toi);

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

#[derive(Clone, Debug, Default, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
pub struct IipDatum {
    cl_unit: String,
    component: Component,
    data_value: Option<i64>,
    frequency: ItaFrequency,
    note_ref: Option<String>,
    time_period: jiff::civil::Date,
    time_series_description: String,
    time_series_id: String,
    type_of_investment: Investment,
    unit_mult: Option<i64>,
    year: jiff::civil::Date,
}

impl IipDatum {
    pub fn read_json(m: &serde_json::Map<String, serde_json::Value>) -> Result<Self, BeaErr> {
        let cl_unit = map_to_string("CL_UNIT", m)?;
        tracing::trace!("cl_unit is {cl_unit}.");
        let component = map_to_string("Component", m)?;
        let component = Component::from_str(&component)
            .map_err(|e| DeriveFromStr::new(component, e, line!(), file!().to_string()))?;
        tracing::trace!("component is {component}.");
        let data_value = map_to_string("DataValue", m)?;
        let data_value = if data_value.is_empty() {
            None
        } else {
            Some(map_to_int("DataValue", m)?)
        };
        tracing::trace!("data_value is {data_value:?}.");
        let frequency = map_to_string("Frequency", m)?;
        let frequency = ItaFrequency::from_value(&frequency)?;
        tracing::trace!("frequency is {frequency}.");
        let note_ref = map_to_string("NoteRef", m)?;
        let note_ref = if note_ref.is_empty() {
            None
        } else {
            Some(note_ref)
        };
        tracing::trace!("note_ref is {note_ref:?}.");
        let time_period = map_to_string("TimePeriod", m)?;
        let time_period = date_by_period(&time_period)?;
        tracing::trace!("time_period is {time_period}.");
        let time_series_description = map_to_string("TimeSeriesDescription", m)?;
        tracing::trace!("time_series_description is {time_series_description}.");
        let time_series_id = map_to_string("TimeSeriesId", m)?;
        tracing::trace!("time_series_id is {time_series_id}.");
        let type_of_investment = map_to_string("TypeOfInvestment", m)?;
        let type_of_investment = Investment::from_str(&type_of_investment)
            .map_err(|e| DeriveFromStr::new(type_of_investment, e, line!(), file!().to_string()))?;
        tracing::trace!("type_of_investment is {type_of_investment}.");
        let unit_mult = map_to_int("UNIT_MULT", m)?;
        let unit_mult = match unit_mult {
            0 => None,
            num => Some(num),
        };
        tracing::trace!("unit_mult is {unit_mult:?}.");
        let year = map_to_string("Year", m)?;
        let year = parse_year(&year)?;
        tracing::trace!("year is {year}.");
        Ok(Self {
            cl_unit,
            component,
            data_value,
            frequency,
            note_ref,
            time_period,
            time_series_description,
            time_series_id,
            type_of_investment,
            unit_mult,
            year,
        })
    }
}

impl TryFrom<serde_json::Value> for IipDatum {
    type Error = BeaErr;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        tracing::trace!("Reading IipDatum.");
        match value {
            serde_json::Value::Object(m) => {
                let data = Self::read_json(&m)?;
                Ok(data)
            }
            _ => {
                tracing::trace!("Invalid Value: {value:#?}");
                let error = NotObject::new(line!(), file!().to_string());
                Err(error.into())
            }
        }
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
    derive_more::From,
)]
#[from(Vec<IipDatum>)]
pub struct IipData(Vec<IipDatum>);

impl TryFrom<&std::path::PathBuf> for IipData {
    type Error = BeaErr;

    fn try_from(value: &std::path::PathBuf) -> Result<Self, Self::Error> {
        let file = std::fs::File::open(value)
            .map_err(|e| IoError::new(value.into(), e, line!(), file!().into()))?;
        let rdr = std::io::BufReader::new(file);
        let res: serde_json::Value = serde_json::from_reader(rdr)
            .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
        let data = BeaResponse::try_from(&res)?;
        tracing::trace!("Response read.");
        tracing::trace!("Response: {data:#?}");
        let results = data.results();
        if let Some(data) = results.into_data() {
            match data {
                Data::Iip(iip) => {
                    tracing::trace!("{} IIP records read.", iip.len());
                    Ok(iip)
                }
                _ => {
                    tracing::warn!("Not IIP variant.");
                    let error = DatasetMissing::new(
                        "IIP variant needed".to_string(),
                        line!(),
                        file!().to_string(),
                    );
                    Err(error.into())
                }
            }
        } else {
            let clue = "Data variant missing, expected IIP results".to_string();
            tracing::warn!("{clue}");
            let error =
                VariantMissing::new(clue, "Results".to_string(), line!(), file!().to_string());
            Err(error.into())
        }
    }
}

impl TryFrom<&serde_json::Value> for IipData {
    type Error = BeaErr;
    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        tracing::trace!("Reading ItaData");
        match crate::data::result_to_data(value)? {
            serde_json::Value::Array(v) => {
                let mut data = Vec::new();
                for val in v {
                    match val {
                        serde_json::Value::Object(m) => {
                            let datum = IipDatum::read_json(m)?;
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
