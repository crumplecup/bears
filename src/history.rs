use crate::{error::SerdeJson, App, BeaErr, EnvError, Event, IoError, ResultStatus};

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
    pub fn from_file() -> Result<Self, BeaErr> {
        dotenvy::dotenv().ok();
        let path = EnvError::from_env("BEA_DATA")?;
        let path = std::path::PathBuf::from(path);
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
        let file = match std::fs::read_to_string(&path) {
            Ok(file) => file,
            Err(source) => {
                let error = IoError::new(path.to_owned(), source, line!(), file!().to_string());
                return Err(error.into());
            }
        };
        for line in file.lines() {
            tracing::trace!("String: {line}");
            let json: serde_json::Value = match serde_json::from_str(line) {
                Ok(json) => json,
                Err(source) => {
                    let error = SerdeJson::new(source);
                    return Err(error.into());
                }
            };
            tracing::trace!("Json: {json}");
            let event = Event::try_from(&json)?;
            events.insert(event.path().clone(), event);
        }
        Ok(Self(events))
    }
}
