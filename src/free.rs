use crate::{FromStrError, JsonParseError, JsonParseErrorKind};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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

// pub fn from_str(s: &str) -> Result<String,

#[tracing::instrument]
pub fn json_str(json: &serde_json::Value) -> Result<String, JsonParseError> {
    match json {
        serde_json::Value::String(s) => {
            tracing::info!("String detected: {s}");
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

#[tracing::instrument]
pub fn json_bool(json: &serde_json::Value) -> Result<bool, JsonParseError> {
    match json {
        serde_json::Value::Number(n) => {
            tracing::info!("Number detected: {n}");
            if n.as_u64() == Some(1) {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        serde_json::Value::String(s) => {
            tracing::info!("String detected: {s}");
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

#[tracing::instrument(skip(m))]
pub fn map_to_bool(
    key: &str,
    m: &serde_json::Map<String, serde_json::Value>,
) -> Result<bool, JsonParseError> {
    tracing::info!("Mapping value to bool.");
    if let Some(value) = m.get(key) {
        tracing::info!("Boolean candidate found.");
        let flag = json_bool(value)?;
        Ok(flag)
    } else {
        tracing::warn!("Key missing: {key}");
        let error = JsonParseErrorKind::KeyMissing(key.to_string());
        Err(error.into())
    }
}

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
