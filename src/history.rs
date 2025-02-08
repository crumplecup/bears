use crate::{error::SerdeJson, App, BeaErr, Dataset, EnvError, Event, IoError, Mode, ResultStatus};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
)]
pub struct History(std::collections::HashMap<std::path::PathBuf, Event>);

impl History {
    pub fn from_env() -> Result<Self, BeaErr> {
        dotenvy::dotenv().ok();
        let path = EnvError::from_env("BEA_DATA")?;
        let path = std::path::PathBuf::from(path);
        let path = path.join("history");
        Self::try_from(&path)
    }

    pub fn contains(&self, path: &std::path::PathBuf) -> bool {
        self.contains_key(path)
    }

    pub fn is_success(&self, app: &App) -> Result<Option<bool>, BeaErr> {
        let path = app.destination(false)?;
        if let Some(event) = self.get(&path) {
            match event.status() {
                ResultStatus::Success(_, _) => Ok(Some(true)),
                _ => Ok(Some(false)),
            }
        } else {
            Ok(None)
        }
    }

    pub fn is_error(&self, app: &App) -> Result<Option<bool>, BeaErr> {
        let path = app.destination(false)?;
        if let Some(event) = self.get(&path) {
            match event.status() {
                ResultStatus::Error(_) => Ok(Some(true)),
                _ => Ok(Some(false)),
            }
        } else {
            Ok(None)
        }
    }

    pub fn summary(&self) {
        let mut success = 0;
        let mut error = 0;
        for value in self.values() {
            match value.status() {
                ResultStatus::Success(_, _) => success += 1,
                ResultStatus::Error(_) => error += 1,
                _ => {}
            }
        }
        tracing::info!("Successes: {success}");
        tracing::info!("Errors: {error}");
    }
}

impl TryFrom<&std::path::PathBuf> for History {
    type Error = BeaErr;

    fn try_from(path: &std::path::PathBuf) -> Result<Self, Self::Error> {
        let mut events = std::collections::HashMap::new();
        let path = path.join("history.log");
        let file = std::fs::read_to_string(&path)
            .map_err(|e| IoError::new(path, e, line!(), file!().into()))?;
        for line in file.lines() {
            tracing::trace!("String: {line}");
            let json: serde_json::Value = serde_json::from_str(line)
                .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
            tracing::trace!("Json: {json}");
            let event = Event::try_from(&json)?;
            events.insert(event.path().clone(), event);
        }
        Ok(Self(events))
    }
}

impl TryFrom<(Dataset, Mode)> for History {
    type Error = BeaErr;

    fn try_from(ctx: (Dataset, Mode)) -> Result<Self, Self::Error> {
        dotenvy::dotenv().ok();
        let (dataset, mode) = ctx;
        let key = "BEA_DATA".to_string();
        let bea_data = std::env::var(&key).map_err(|e| EnvError::new(key, e))?;
        let path = std::path::PathBuf::from(&bea_data);
        let path = path.join("history");
        let path = path.join(format!("history_{dataset}_{mode}.log"));
        Self::try_from(&path)
    }
}
