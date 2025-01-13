use crate::{App, BeaErr, DeriveFromStr, EnvError, IoError, Method, ReqwestError};
use derive_more::FromStr;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
    derive_new::new,
)]
pub struct Queue(Vec<App>);

impl Queue {
    pub async fn process(&mut self) -> Result<(), BeaErr> {
        self.iter_mut().map(Self::process_app).for_each(drop);
        Ok(())
    }

    pub async fn process_app(app: &mut App) -> Result<(), BeaErr> {
        let params = app.params();
        let method = params["METHOD"].clone();
        let method = match Method::from_str(&method) {
            Ok(value) => value,
            Err(source) => {
                let error = DeriveFromStr::new(method, source, line!(), file!().to_string());
                return Err(error.into());
            }
        };
        match method {
            Method::GetData => {
                let dataset = params["DatasetName"].clone();
                let name = params["TableName"].clone();
                let data = app.get().await?;
                match data.json::<serde_json::Value>().await {
                    Ok(json) => {
                        let contents = serde_json::to_vec(&json)?;
                        dotenvy::dotenv().ok();
                        let bea_data = EnvError::from_env("BEA_DATA")?;
                        let path = std::path::PathBuf::from(&bea_data);
                        let path = path.join("data");
                        if !std::fs::exists(&path)? {
                            std::fs::DirBuilder::new().create(&path)?;
                            tracing::info!("Target directory for Data created.");
                        }
                        let path = path.join(&dataset);
                        if !std::fs::exists(&path)? {
                            std::fs::DirBuilder::new().create(&path)?;
                            tracing::info!("Target directory for {dataset} created.");
                        }
                        let path = path.join(format!("{dataset}_{name}.json"));
                        match std::fs::write(&path, contents) {
                            Ok(()) => match Throttle::check(&path)? {
                                Throttle::Full => {}
                                Throttle::Ease(minutes) => {
                                    let duration = tokio::time::Duration::from_secs(minutes * 60);
                                    tracing::info!("Pausing for {minutes} minutes.");
                                    tokio::time::sleep(duration).await;
                                }
                                Throttle::Idle => {
                                    let minutes = 60;
                                    let duration = tokio::time::Duration::from_secs(minutes * 60);
                                    tracing::info!("Pausing for {minutes} minutes.");
                                    tokio::time::sleep(duration).await;
                                }
                            },
                            Err(source) => {
                                let error =
                                    IoError::new(path, source, line!(), file!().to_string());
                                return Err(error.into());
                            }
                        }
                    }
                    Err(source) => {
                        let url = app.url().to_string();
                        let method = "get".to_string();
                        let error =
                            ReqwestError::new(url, method, source, line!(), file!().to_string());
                        return Err(error.into());
                    }
                }
            }
            _ => tracing::info!("{method} not implemented."),
        }
        Ok(())
    }
}

/// The `Throttle` will move to idle if the user has exceeded a rate limiting condition from BEA:
///
/// * More than 100 requests per minute.
/// * More than 100MB of data volume retrieved per minute.
/// * More than 30 errors per minute.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Throttle {
    /// No limiting in place.
    #[default]
    Full,
    /// Pause for duration before continuing.
    Ease(u64),
    /// Wait for one hour if user has exceeded wait limits.
    Idle,
}

impl Throttle {
    pub fn check<P: AsRef<std::path::Path>>(path: P) -> Result<Self, BeaErr> {
        let mut throttle = Self::Full;
        // data limit is 100mb per minute BEA User Guide pg. 4
        let data_limit: u64 = 1000000;
        let file = match std::fs::File::open(path.as_ref()) {
            Ok(f) => f,
            Err(source) => {
                let error =
                    IoError::new(path.as_ref().into(), source, line!(), file!().to_string());
                return Err(error.into());
            }
        };
        let metadata = match file.metadata() {
            Ok(data) => data,
            Err(source) => {
                let error =
                    IoError::new(path.as_ref().into(), source, line!(), file!().to_string());
                return Err(error.into());
            }
        };
        tracing::info!("File {:?} is len {}", path.as_ref(), metadata.len());
        if metadata.len() >= data_limit {
            let ratio = metadata.len() as f64 / data_limit as f64;
            let pause = ratio.ceil() as u64;
            tracing::info!("Easing back throttle by {pause} minutes.");
            throttle = Self::Ease(pause);
        }
        Ok(throttle)
    }
}
