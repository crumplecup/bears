use crate::{
    FromStrError, JsonParseError, JsonParseErrorKind, NotFloat, NotInteger, ParseFloat,
    ParseInteger,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Initiates a subscriber for the tracing library. Used to instrument internal library functions
/// for debugging and diagnostics.
#[tracing::instrument]
pub fn trace_init() {
    if tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "bea=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .is_ok()
    {};
    tracing::trace!("Loading Bea...");
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
            tracing::info!("Number detected: {n}");
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
            tracing::info!("String detected: {s}");
            match s.parse::<f64>() {
                Ok(num) => Ok(num),
                Err(source) => {
                    let error = ParseFloat::new(s.into(), source, line!(), file!().to_string());
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
            tracing::info!("Number detected: {n}");
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
            tracing::info!("String detected: {s}");
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
        let error = JsonParseErrorKind::KeyMissing(key.to_string());
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
        let error = JsonParseErrorKind::KeyMissing(key.to_string());
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
        let error = JsonParseErrorKind::KeyMissing(key.to_string());
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
        let error = JsonParseErrorKind::KeyMissing(key.to_string());
        Err(error.into())
    }
}
