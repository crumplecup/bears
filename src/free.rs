use crate::{
    App, BeaErr, FromStrError, IoError, JsonParseError, JsonParseErrorKind, KeyMissing, Options,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};

pub fn bea_data() -> Result<std::path::PathBuf, EnvError> {
    let key = "BEA_DATA".to_string();
    let path = std::env::var(&key)
        .map_err(|source| EnvError::new(key, source, line!(), file!().into()))?;
    Ok(std::path::PathBuf::from(&path))
}

/// Initiates a subscriber for the tracing library. Used to instrument internal library functions
/// for debugging and diagnostics.
#[tracing::instrument]
pub fn trace_init() -> Result<(), BeaErr> {
    dotenvy::dotenv().ok();
    let path = bea_data()?;
    let path = path.join("history");
    if !path.exists() {
        std::fs::DirBuilder::new()
            .create(&path)
            .map_err(|e| IoError::new(path.clone(), e, line!(), file!().into()))?;
        tracing::info!("History directory created.");
    }
    let path = path.join("history.log");
    let history = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(&path)
        .map_err(|e| IoError::new(path, e, line!(), file!().into()))?;
    let history = tracing_subscriber::fmt::layer()
        .json()
        .with_writer(std::sync::Arc::new(history))
        .with_filter(tracing_subscriber::filter::filter_fn(|metadata| {
            metadata.target() == "download_history" || metadata.target() == "load_history"
        }));
    if tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "bea=info".into()),
        )
        .with(
            tracing_subscriber::fmt::layer().with_filter(tracing_subscriber::filter::filter_fn(
                |metadata| {
                    metadata.target() != "download_history" && metadata.target() != "load_history"
                },
            )),
        )
        .with(history)
        .try_init()
        .is_ok()
    {};
    tracing::trace!("Loading Bea...");
    Ok(())
}

/// Helper function
/// Initiates logging
/// Reads environmental variables from .env
/// Creates an instance of App
#[tracing::instrument]
pub fn init() -> Result<App, BeaErr> {
    trace_init()?;
    tracing::info!("Test logging initialized.");
    dotenvy::dotenv().ok();
    let url = "BEA_URL".to_string();
    let url = std::env::var(&url)
        .map_err(|source| EnvError::new(url, source, line!(), file!().into()))?;
    let url = url::Url::parse(&url)
        .map_err(|source| UrlParseError::new(url, source, line!(), file!().into()))?;
    let key = "API_KEY".to_string();
    let key = std::env::var(&key)
        .map_err(|source| EnvError::new(key, source, line!(), file!().into()))?;
    let options = Options::default();
    let app = App::new(key, options, url);
    Ok(app)
}

/// Converts a [`serde_json::Value`] to a String.
/// Called by [`map_to_string`].
#[tracing::instrument]
pub fn json_str(json: &serde_json::Value) -> Result<String, JsonParseError> {
    match json {
        serde_json::Value::String(s) => {
            tracing::trace!("String detected: {s}");
            if s.starts_with('"') {
                match serde_json::from_str(s) {
                    Ok(v) => Ok(v),
                    Err(e) => {
                        let error = FromStrError::new(s.to_string(), e);
                        let error: JsonParseErrorKind = error.into();
                        Err(error.into())
                    }
                }
            } else {
                Ok(s.to_string())
            }
        }
        _ => {
            tracing::warn!("Unexpected value.");
            let error = JsonParseErrorKind::NotString;
            Err(error.into())
        }
    }
}

/// Converts a [`serde_json::Value`] to a [`bool`] based on BEA data conventions.
/// Called by [`map_to_bool`].
#[tracing::instrument]
pub fn json_bool(json: &serde_json::Value) -> Result<bool, JsonParseError> {
    match json {
        serde_json::Value::Number(n) => {
            tracing::trace!("Number detected: {n}");
            if n.as_u64() == Some(1) {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        serde_json::Value::String(s) => {
            tracing::trace!("String detected: {s}");
            if s == "1" {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        _ => {
            tracing::warn!("Unexpected value.");
            let error = JsonParseErrorKind::NotBool;
            Err(error.into())
        }
    }
}

/// Converts a [`serde_json::Value`] to a [`f64`] based on BEA data conventions.
/// Called by [`map_to_float`].
#[tracing::instrument]
pub fn json_float(json: &serde_json::Value) -> Result<f64, JsonParseError> {
    match json {
        serde_json::Value::Number(n) => {
            tracing::trace!("Number detected: {n}");
            if let Some(num) = n.as_f64() {
                Ok(num)
            } else {
                tracing::warn!("Number failed to parse as float.");
                let error = NotFloat::new(
                    format!("Number {n} failed to parse as float"),
                    line!(),
                    file!().to_string(),
                );
                let error = JsonParseErrorKind::from(error);
                Err(error.into())
            }
        }
        serde_json::Value::String(s) => {
            tracing::trace!("String detected: {s}");
            let s = s.replace(",", "");
            match s.parse::<f64>() {
                Ok(num) => Ok(num),
                Err(source) => {
                    let error = ParseFloat::new(s, source, line!(), file!().to_string());
                    let error = JsonParseErrorKind::from(error);
                    Err(error.into())
                }
            }
        }
        _ => {
            tracing::warn!("Unexpected value.");
            let error = NotFloat::new(
                format!("Value {json:#?} is not a Number or String variant of serde_json::Value"),
                line!(),
                file!().to_string(),
            );
            let error = JsonParseErrorKind::from(error);
            Err(error.into())
        }
    }
}

/// Converts a [`serde_json::Value`] to an [`i64`] based on BEA data conventions.
/// Called by [`map_to_int`].
#[tracing::instrument]
pub fn json_int(json: &serde_json::Value) -> Result<i64, JsonParseError> {
    match json {
        serde_json::Value::Number(n) => {
            tracing::trace!("Number detected: {n}");
            if let Some(num) = n.as_i64() {
                Ok(num)
            } else {
                tracing::warn!("Number failed to parse as integer.");
                let error = NotInteger::new(
                    format!("Number {n} failed to parse as float"),
                    line!(),
                    file!().to_string(),
                );
                let error = JsonParseErrorKind::from(error);
                Err(error.into())
            }
        }
        serde_json::Value::String(s) => {
            tracing::trace!("String detected: {s}");
            match s.parse::<i64>() {
                Ok(num) => Ok(num),
                Err(source) => {
                    let error = ParseInteger::new(s.into(), source, line!(), file!().to_string());
                    let error = JsonParseErrorKind::from(error);
                    Err(error.into())
                }
            }
        }
        _ => {
            tracing::warn!("Unexpected value.");
            let error = NotInteger::new(
                format!("Value {json:#?} is not a Number or String variant of serde_json::Value"),
                line!(),
                file!().to_string(),
            );
            let error = JsonParseErrorKind::from(error);
            Err(error.into())
        }
    }
}

/// Convenience function for when we expect a boolean value stored as a JSON string.
#[tracing::instrument(skip(m))]
pub fn map_to_bool(
    key: &str,
    m: &serde_json::Map<String, serde_json::Value>,
) -> Result<bool, JsonParseError> {
    tracing::trace!("Mapping value to bool.");
    if let Some(value) = m.get(key) {
        tracing::trace!("Boolean candidate found.");
        let flag = json_bool(value)?;
        Ok(flag)
    } else {
        tracing::warn!("Key missing: {key}");
        let error = KeyMissing::new(key.to_string(), line!(), file!().to_string());
        let error = JsonParseErrorKind::from(error);
        Err(error.into())
    }
}

/// Convenience function for when we expect a float stored as a JSON string.
#[tracing::instrument(skip(m))]
pub fn map_to_float(
    key: &str,
    m: &serde_json::Map<String, serde_json::Value>,
) -> Result<f64, JsonParseError> {
    tracing::trace!("Mapping value to float.");
    if let Some(value) = m.get(key) {
        tracing::trace!("Float candidate found.");
        let float = json_float(value)?;
        Ok(float)
    } else {
        tracing::warn!("Key missing: {key}");
        let error = KeyMissing::new(key.to_string(), line!(), file!().to_string());
        let error = JsonParseErrorKind::from(error);
        Err(error.into())
    }
}

/// Convenience function for when we expect an integer stored as a JSON string.
#[tracing::instrument(skip(m))]
pub fn map_to_int(
    key: &str,
    m: &serde_json::Map<String, serde_json::Value>,
) -> Result<i64, JsonParseError> {
    tracing::trace!("Mapping value to integer.");
    if let Some(value) = m.get(key) {
        tracing::trace!("Integer candidate found.");
        let flag = json_int(value)?;
        Ok(flag)
    } else {
        tracing::warn!("Key missing: {key}");
        let error = KeyMissing::new(key.to_string(), line!(), file!().to_string());
        let error = JsonParseErrorKind::from(error);
        Err(error.into())
    }
}

/// Convenience function for when we expect a String value stored as a JSON string.
#[tracing::instrument(skip(m))]
pub fn map_to_string(
    key: &str,
    m: &serde_json::Map<String, serde_json::Value>,
) -> Result<String, JsonParseError> {
    if let Some(value) = m.get(key) {
        json_str(value)
    } else {
        let error = KeyMissing::new(key.to_string(), line!(), file!().to_string());
        let error = JsonParseErrorKind::from(error);
        Err(error.into())
    }
}

/// Generic function to serialize data types into a CSV file.  Called by methods to avoid code
/// duplication.
pub fn to_csv<T: serde::Serialize + Clone, P: AsRef<std::path::Path>>(
    item: &mut [T],
    path: P,
) -> Result<(), BeaErr> {
    match csv::Writer::from_path(path.as_ref()) {
        Ok(mut wtr) => {
            for i in item {
                match wtr.serialize(i) {
                    Ok(_) => {}
                    Err(source) => {
                        let path = std::path::PathBuf::from(path.as_ref());
                        return Err(Csv::new(path, source, line!(), file!().to_string()).into());
                    }
                }
            }
            match wtr.flush() {
                Ok(_) => {}
                Err(source) => {
                    let path = std::path::PathBuf::from(path.as_ref());
                    return Err(IoError::new(path, source, line!(), file!().to_string()).into());
                }
            }
            Ok(())
        }
        Err(source) => Err(Csv::new(
            std::path::PathBuf::from(path.as_ref()),
            source,
            line!(),
            file!().to_string(),
        )
        .into()),
    }
}

/// Generic function to deserialize data types from a CSV file.  Called by methods to avoid code
/// duplication.
pub fn from_csv<T: serde::de::DeserializeOwned + Clone, P: AsRef<std::path::Path>>(
    path: P,
) -> Result<Vec<T>, IoError> {
    let mut records = Vec::new();
    match std::fs::File::open(&path) {
        Ok(file) => {
            let mut rdr = csv::Reader::from_reader(file);

            let mut dropped = 0;
            for result in rdr.deserialize() {
                match result {
                    Ok(record) => records.push(record),
                    Err(e) => {
                        tracing::trace!("Dropping: {}", e.to_string());
                        dropped += 1;
                    }
                }
            }
            tracing::trace!("{} records dropped.", dropped);

            Ok(records)
        }
        Err(source) => {
            let path = std::path::PathBuf::from(path.as_ref());
            Err(IoError::new(path, source, line!(), file!().to_string()))
        }
    }
}

pub fn file_size<P: AsRef<std::path::Path>>(path: P) -> Option<u64> {
    let path = path.as_ref();
    if path.exists() {
        let file = match std::fs::File::open(path) {
            Ok(f) => f,
            Err(_) => {
                return None;
            }
        };
        let metadata = match file.metadata() {
            Ok(data) => data,
            Err(_) => return None,
        };
        Some(metadata.len())
    } else {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display, derive_new::new)]
#[display("could not parse {input} to integer at line {line} in file {file}")]
pub struct NotInteger {
    input: String,
    line: u32,
    file: String,
}

impl std::error::Error for NotInteger {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display, derive_new::new)]
#[display("could not parse {input} to float at line {line} in file {file}")]
pub struct NotFloat {
    input: String,
    line: u32,
    file: String,
}

impl std::error::Error for NotFloat {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display, derive_new::new)]
#[display("could not parse {input} to integer at line {line} in file {file}")]
pub struct ParseInteger {
    input: String,
    source: std::num::ParseIntError,
    line: u32,
    file: String,
}

impl std::error::Error for ParseInteger {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display, derive_new::new)]
#[display("could not parse {input} to float at line {line} in file {file}")]
pub struct ParseFloat {
    input: String,
    source: std::num::ParseFloatError,
    line: u32,
    file: String,
}

impl std::error::Error for ParseFloat {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

#[derive(
    Debug,
    derive_getters::Getters,
    derive_setters::Setters,
    derive_more::Display,
    derive_more::Error,
    derive_new::new,
)]
#[display("{target} failed to parse to a valid url at line {line} in {file}")]
#[setters(prefix = "with_", borrow_self)]
pub struct UrlParseError {
    pub target: String,
    pub source: url::ParseError,
    line: u32,
    file: String,
}

#[derive(
    Debug,
    derive_getters::Getters,
    derive_setters::Setters,
    derive_more::Display,
    derive_new::new,
    derive_more::Error,
)]
#[display(".env file missing {target} at line {line} in {file}")]
#[setters(prefix = "with_", borrow_self)]
pub struct EnvError {
    target: String,
    source: std::env::VarError,
    line: u32,
    file: String,
}

/// The `Csv` struct contains error information associated with the `csv` crate.
#[derive(Debug, derive_more::Display, derive_more::Error, derive_new::new)]
#[display("csv error at path {path:?} in line {line} of {file}")]
pub struct Csv {
    path: std::path::PathBuf,
    source: csv::Error,
    line: u32,
    file: String,
}
