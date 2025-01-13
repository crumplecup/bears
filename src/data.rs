use crate::{
    map_to_float, map_to_int, map_to_string, quarter, BeaErr, JsonParseError, JsonParseErrorKind,
    KeyMissing, NotArray, NotObject,
};
#[derive(
    Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize, derive_more::From,
)]
#[from(Vec<NipaData>)]
pub enum Data {
    #[from(NipaData)]
    Nipa(NipaData),
}

#[derive(Clone, Debug, Default, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
pub struct NipaDatum {
    cl_unit: String,
    data_value: f64,
    line_description: String,
    line_number: i64,
    metric_name: String,
    note_ref: String,
    series_code: String,
    table_name: String,
    time_period: jiff::civil::Date,
    unit_mult: Option<i64>,
}

impl NipaDatum {
    pub fn read_json(m: &serde_json::Map<String, serde_json::Value>) -> Result<Self, BeaErr> {
        let cl_unit = map_to_string("CL_UNIT", m)?;
        let data_value = map_to_float("DataValue", m)?;
        let line_description = map_to_string("LineDescription", m)?;
        let line_number = map_to_int("LineNumber", m)?;
        let metric_name = map_to_string("METRIC_NAME", m)?;
        let note_ref = map_to_string("NoteRef", m)?;
        let series_code = map_to_string("SeriesCode", m)?;
        let table_name = map_to_string("TableName", m)?;
        let time_period = map_to_string("TimePeriod", m)?;
        let time_period = quarter(&time_period)?;
        let unit_mult = map_to_int("UNIT_MULT", m)?;
        let unit_mult = match unit_mult {
            0 => None,
            num => Some(num),
        };
        Ok(Self {
            cl_unit,
            data_value,
            line_description,
            line_number,
            metric_name,
            note_ref,
            series_code,
            table_name,
            time_period,
            unit_mult,
        })
    }
}

impl TryFrom<serde_json::Value> for NipaDatum {
    type Error = BeaErr;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        tracing::trace!("Reading NipaDatum.");
        match value {
            serde_json::Value::Object(m) => {
                let data = Self::read_json(&m)?;
                Ok(data)
            }
            _ => {
                tracing::trace!("Invalid Value: {value:#?}");
                let error = NotObject::new(line!(), file!().to_string());
                let error = JsonParseErrorKind::from(error);
                let error = JsonParseError::from(error);
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
#[from(Vec<NipaDatum>)]
pub struct NipaData(Vec<NipaDatum>);

impl TryFrom<&serde_json::Value> for NipaData {
    type Error = BeaErr;
    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        tracing::info!("Reading NipaData");
        match value {
            serde_json::Value::Object(m) => {
                let key = "Data".to_string();
                if let Some(data) = m.get(&key) {
                    match data {
                        serde_json::Value::Array(v) => {
                            tracing::trace!("Array found for {key}.");
                            let mut data = Vec::new();
                            for val in v {
                                match val {
                                    serde_json::Value::Object(m) => {
                                        let datum = NipaDatum::read_json(m)?;
                                        data.push(datum);
                                    }
                                    _ => {
                                        let error = NotObject::new(line!(), file!().to_string());
                                        let error = JsonParseErrorKind::from(error);
                                        let error = JsonParseError::from(error);
                                        return Err(error.into());
                                    }
                                }
                            }
                            tracing::trace!("Data found: {} records.", data.len());
                            Ok(Self(data))
                        }
                        _ => {
                            tracing::trace!("Unexpected content: {m:#?}");
                            let error = NotArray::new(line!(), file!().to_string());
                            let error = JsonParseErrorKind::from(error);
                            let error = JsonParseError::from(error);
                            Err(error.into())
                        }
                    }
                } else {
                    tracing::trace!("Parameter Value Table missing.");
                    let error = KeyMissing::new(key, line!(), file!().to_string());
                    let error = JsonParseErrorKind::from(error);
                    let error = JsonParseError::from(error);
                    Err(error.into())
                }
            }
            _ => {
                tracing::trace!("Wrong Value type: {value:#?}");
                let error = NotObject::new(line!(), file!().to_string());
                let error = JsonParseErrorKind::from(error);
                let error = JsonParseError::from(error);
                Err(error.into())
            }
        }
    }
}
