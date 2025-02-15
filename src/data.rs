use crate::{
    date_by_period, map_to_float, map_to_int, map_to_string, parse_year, AnnotatedInteger, BeaErr,
    BeaResponse, DatasetMissing, IoError, JsonParseError, JsonParseErrorKind, KeyMissing, Naics,
    NotArray, NotObject, RowCode, SerdeJson, VariantMissing,
};
#[derive(
    Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize, derive_more::From,
)]
#[from(Vec<NipaData>)]
pub enum Data {
    #[from(NipaData)]
    Nipa(NipaData),
    #[from(FixedAssetData)]
    FixedAssets(FixedAssetData),
    #[from(MneDiData)]
    MneDi(MneDiData),
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
        tracing::trace!("cl_unit is {cl_unit}.");
        let data_value = map_to_float("DataValue", m)?;
        tracing::trace!("data_value is {data_value}.");
        let line_description = map_to_string("LineDescription", m)?;
        tracing::trace!("line_description is {line_description}.");
        let line_number = map_to_int("LineNumber", m)?;
        tracing::trace!("line_number is {line_number}.");
        let metric_name = map_to_string("METRIC_NAME", m)?;
        tracing::trace!("metric_name is {metric_name}.");
        let note_ref = map_to_string("NoteRef", m)?;
        tracing::trace!("note_ref is {note_ref}.");
        let series_code = map_to_string("SeriesCode", m)?;
        tracing::trace!("series_code is {series_code}.");
        let table_name = map_to_string("TableName", m)?;
        tracing::trace!("table_name is {table_name}.");
        let time_period = map_to_string("TimePeriod", m)?;
        let time_period = date_by_period(&time_period)?;
        tracing::trace!("time_period is {time_period}.");
        let unit_mult = map_to_int("UNIT_MULT", m)?;
        let unit_mult = match unit_mult {
            0 => None,
            num => Some(num),
        };
        tracing::trace!("unit_mult is {unit_mult:?}.");
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

impl TryFrom<&std::path::PathBuf> for NipaData {
    type Error = BeaErr;

    fn try_from(value: &std::path::PathBuf) -> Result<Self, Self::Error> {
        let file = std::fs::File::open(value)
            .map_err(|e| IoError::new(value.into(), e, line!(), file!().into()))?;
        let rdr = std::io::BufReader::new(file);
        let res: serde_json::Value = serde_json::from_reader(rdr)
            .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
        let data = BeaResponse::try_from(&res)?;
        tracing::info!("Response read.");
        tracing::trace!("Response: {data:#?}");
        let results = data.results();
        if let Some(data) = results.into_data() {
            match data {
                Data::Nipa(nipa) => {
                    tracing::info!("{} Nipa records read.", nipa.len());
                    Ok(nipa)
                }
                _ => {
                    tracing::warn!("Not Nipa variant.");
                    let error = DatasetMissing::new(
                        "Nipa variant needed".to_string(),
                        line!(),
                        file!().to_string(),
                    );
                    Err(error.into())
                }
            }
        } else {
            tracing::warn!("Data variant missing.");
            let error = VariantMissing::new(
                "Data variant missing".to_string(),
                "Results".to_string(),
                line!(),
                file!().to_string(),
            );
            Err(error.into())
        }
    }
}

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

#[derive(Clone, Debug, Default, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
pub struct FixedAssetDatum {
    cl_unit: String,
    data_value: f64,
    line_description: String,
    line_number: i64,
    metric_name: String,
    series_code: String,
    table_name: String,
    time_period: jiff::civil::Date,
    unit_mult: Option<i64>,
}

impl FixedAssetDatum {
    pub fn read_json(m: &serde_json::Map<String, serde_json::Value>) -> Result<Self, BeaErr> {
        let cl_unit = map_to_string("CL_UNIT", m)?;
        let data_value = map_to_float("DataValue", m)?;
        let line_description = map_to_string("LineDescription", m)?;
        let line_number = map_to_int("LineNumber", m)?;
        let metric_name = map_to_string("METRIC_NAME", m)?;
        let series_code = map_to_string("SeriesCode", m)?;
        let table_name = map_to_string("TableName", m)?;
        let time_period = map_to_string("TimePeriod", m)?;
        let time_period = date_by_period(&time_period)?;
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
            series_code,
            table_name,
            time_period,
            unit_mult,
        })
    }
}

impl TryFrom<serde_json::Value> for FixedAssetDatum {
    type Error = BeaErr;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        tracing::trace!("Reading FixedAssetDatum.");
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
#[from(Vec<FixedAssetDatum>)]
pub struct FixedAssetData(Vec<FixedAssetDatum>);

impl TryFrom<&std::path::PathBuf> for FixedAssetData {
    type Error = BeaErr;

    fn try_from(value: &std::path::PathBuf) -> Result<Self, Self::Error> {
        let file = std::fs::File::open(value)
            .map_err(|e| IoError::new(value.into(), e, line!(), file!().into()))?;
        let rdr = std::io::BufReader::new(file);
        let res: serde_json::Value = serde_json::from_reader(rdr)
            .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
        let data = BeaResponse::try_from(&res)?;
        tracing::info!("Response read.");
        tracing::trace!("Response: {data:#?}");
        let results = data.results();
        if let Some(data) = results.into_data() {
            match data {
                Data::FixedAssets(value) => {
                    tracing::info!("{} FixedAsset records read.", value.len());
                    Ok(value)
                }
                _ => {
                    let error = DatasetMissing::new(
                        "FixedAssets".to_string(),
                        line!(),
                        file!().to_string(),
                    );
                    Err(error.into())
                }
            }
        } else {
            tracing::warn!("Data variant missing.");
            let error = VariantMissing::new(
                "Data variant missing".to_string(),
                "Results".to_string(),
                line!(),
                file!().to_string(),
            );
            Err(error.into())
        }
    }
}

impl TryFrom<&serde_json::Value> for FixedAssetData {
    type Error = BeaErr;
    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        tracing::info!("Reading FixedAssetData");
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
                                        let datum = FixedAssetDatum::read_json(m)?;
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

#[derive(Clone, Debug, Default, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
pub struct MneDiDatum {
    column: String,
    column_code: i64,
    column_g_parent: String,
    column_g_parent_code: i64,
    column_parent: String,
    column_parent_code: i64,
    data_value: String,
    data_value_unformatted: AnnotatedInteger,
    row: String,
    row_code: RowCode,
    series_id: i64,
    series_name: String,
    table_column_display_order: f64,
    table_row_display_order: f64,
    table_scale: String,
    year: jiff::civil::Date,
}

impl MneDiDatum {
    pub fn read_json(
        m: &serde_json::Map<String, serde_json::Value>,
        naics: &Naics,
    ) -> Result<Self, BeaErr> {
        tracing::trace!("Reading MneDiDatum.");
        let column = map_to_string("Column", m)?;
        tracing::trace!("Column: {column}.");
        let column_code = map_to_int("ColumnCode", m)?;
        tracing::trace!("Column Code: {column_code}.");
        let column_g_parent = map_to_string("ColumnGParent", m)?;
        tracing::trace!("Column G Parent: {column_g_parent}.");
        let column_g_parent_code = map_to_int("ColumnGParentCode", m)?;
        tracing::trace!("Column G Parent Code: {column_g_parent_code}.");
        let column_parent = map_to_string("ColumnParent", m)?;
        tracing::trace!("Column Parent: {column_parent}.");
        let column_parent_code = map_to_int("ColumnParentCode", m)?;
        tracing::trace!("Column Parent Code: {column_parent_code}.");
        let data_value = map_to_string("DataValue", m)?;
        tracing::trace!("Data Value: {data_value}.");
        let data_value_unformatted = map_to_string("DataValueUnformatted", m)?;
        let data_value_unformatted = AnnotatedInteger::from_value(&data_value_unformatted)?;
        tracing::trace!(
            "Data Value Unformatted: {}.",
            data_value_unformatted.as_value()
        );
        let row = map_to_string("Row", m)?;
        let row_code = RowCode::from_value(m, &row, naics)?;
        let series_id = map_to_int("SeriesID", m)?;
        let series_name = map_to_string("SeriesName", m)?;
        let table_column_display_order = map_to_float("TableColumnDisplayOrder", m)?;
        let table_row_display_order = map_to_float("TableRowDisplayOrder", m)?;
        let table_scale = map_to_string("TableScale", m)?;
        let year = map_to_string("Year", m)?;
        let year = parse_year(&year)?;
        Ok(Self {
            column,
            column_code,
            column_g_parent,
            column_g_parent_code,
            column_parent,
            column_parent_code,
            data_value,
            data_value_unformatted,
            row,
            row_code,
            series_id,
            series_name,
            table_column_display_order,
            table_row_display_order,
            table_scale,
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
    derive_more::From,
)]
#[from(Vec<MneDiDatum>)]
pub struct MneDiData(Vec<MneDiDatum>);

impl TryFrom<&std::path::PathBuf> for MneDiData {
    type Error = BeaErr;

    fn try_from(value: &std::path::PathBuf) -> Result<Self, Self::Error> {
        let file = std::fs::File::open(value)
            .map_err(|e| IoError::new(value.into(), e, line!(), file!().into()))?;
        let rdr = std::io::BufReader::new(file);
        let res: serde_json::Value = serde_json::from_reader(rdr)
            .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
        let data = BeaResponse::try_from(&res)?;
        tracing::info!("Response read.");
        tracing::trace!("Response: {data:#?}");
        let results = data.results();
        if let Some(data) = results.into_data() {
            match data {
                Data::MneDi(value) => {
                    tracing::info!("{} MneDi records read.", value.len());
                    Ok(value)
                }
                _ => {
                    let error =
                        DatasetMissing::new("MneDi".to_string(), line!(), file!().to_string());
                    Err(error.into())
                }
            }
        } else {
            tracing::warn!("Data variant missing.");
            let error = VariantMissing::new(
                "Data variant missing".to_string(),
                "Results".to_string(),
                line!(),
                file!().to_string(),
            );
            Err(error.into())
        }
    }
}

impl TryFrom<&serde_json::Value> for MneDiData {
    type Error = BeaErr;
    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        tracing::trace!("Reading MneDiData");
        // use naics code to determine missing row codes from the row title
        let naics = Naics::from_csv("data/naics_codes.csv")?;
        match value {
            serde_json::Value::Object(m) => {
                let key = "Data".to_string();
                if let Some(data) = m.get(&key) {
                    tracing::trace!("{key} found.");
                    match data {
                        serde_json::Value::Array(v) => {
                            tracing::trace!("Array found for {key}.");
                            let mut data = Vec::new();
                            for val in v {
                                match val {
                                    serde_json::Value::Object(m) => {
                                        let datum = MneDiDatum::read_json(m, &naics)?;
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
                            tracing::info!("Data found: {} records.", data.len());
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
