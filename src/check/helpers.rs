use crate::{trace_init, App, BeaErr, EnvError, Method, Options, UrlParseError};

/// Helper function
/// Initiates logging
/// Reads environmental variables from .env
/// Creates an instance of App
pub fn init() -> Result<App, BeaErr> {
    trace_init();
    tracing::info!("Test logging initialized.");
    dotenvy::dotenv().ok();
    let url = EnvError::from_env("BEA_URL")?;
    let url = UrlParseError::into_url(&url)?;
    let key = EnvError::from_env("API_KEY")?;
    let options = Options::default();
    let app = App::new(key, options, url);
    Ok(app)
}

#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    strum::EnumIter,
)]
pub enum Request {
    #[default]
    Dataset,
    Parameter,
    ParameterValue,
    ParameterValueFilter,
}

impl Request {
    #[tracing::instrument(skip_all)]
    pub fn init(&self) -> Result<App, BeaErr> {
        match self {
            Self::Dataset => {
                let mut app = init()?;
                tracing::info!("App initialized.");
                let mut options = Options::default();
                options.with_method(Method::GetDataSetList);
                app.with_options(options);
                tracing::info!("App configured.");
                Ok(app)
            }
            Self::Parameter => {
                let mut app = init()?;
                tracing::info!("App initialized.");
                let mut options = Options::default();
                options.with_method(Method::GetParameterList);
                app.with_options(options.clone());
                tracing::info!("App configured.");
                Ok(app)
            }
            Self::ParameterValue => {
                let mut app = init()?;
                tracing::info!("App initialized.");
                let mut options = Options::default();
                options.with_method(Method::GetParameterValues);
                app.with_options(options.clone());
                tracing::info!("App configured.");
                Ok(app)
            }
            Self::ParameterValueFilter => {
                let mut app = init()?;
                tracing::info!("App initialized.");
                let mut options = Options::default();
                options.with_method(Method::GetParameterValuesFiltered);
                app.with_options(options.clone());
                tracing::info!("App configured.");
                Ok(app)
            }
        }
    }
}
