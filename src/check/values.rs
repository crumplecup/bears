use crate::{
    trace_init, BeaErr, BeaResponse, Dataset, EnvError, IoError, ParameterName, ReqwestError,
    Results,
};
use strum::IntoEnumIterator;

#[tracing::instrument]
pub fn api_error() -> Result<(), BeaErr> {
    trace_init();
    dotenvy::dotenv().ok();
    let bea_data = EnvError::from_env("BEA_DATA")?;
    let path = std::path::PathBuf::from(&format!(
        "{bea_data}/values_api_error.json" // "{bea_data}/values_{dataset}_{name}.json"
    ));
    let file = IoError::open(path)?;
    let rdr = std::io::BufReader::new(file);
    let res: serde_json::Value = serde_json::from_reader(rdr)?;
    let data = BeaResponse::try_from(&res)?;
    tracing::info!("Response read.");
    tracing::info!("Response: {data:#?}");

    Ok(())
}

pub async fn values_filtered() -> Result<(), BeaErr> {
    trace_init();
    let req = super::Request::ParameterValueFilter;
    let mut app = req.init()?;
    let datasets: Vec<Dataset> = Dataset::iter().collect();
    for dataset in &datasets {
        let names = dataset.names();
        for name in names {
            if *dataset == Dataset::Nipa && name == ParameterName::Frequency {
                let mut options = app.options().clone();
                options.with_dataset(*dataset);
                options.with_target(name);
                app.with_options(options.clone());
                let data = app.get().await?;
                tracing::info!("{data:#?}");
                match data.json::<serde_json::Value>().await {
                    Ok(json) => {
                        tracing::info!("{json:#?}");
                        let bea = BeaResponse::try_from(&json)?;
                        match bea.results() {
                            Results::ApiError(e) => {
                                tracing::info!("{e}");
                            }
                            Results::ParameterValues(pv) => {
                                let req = bea.request();
                            }
                            unexpected => {
                                tracing::warn!("Unexpected type {unexpected:#?}");
                            }
                        }

                        let contents = serde_json::to_vec(&json)?;
                        dotenvy::dotenv().ok();
                        let bea_data = EnvError::from_env("BEA_DATA")?;
                        let path = std::path::PathBuf::from(&format!(
                            "{bea_data}/values_api_error.json" // "{bea_data}/values_{dataset}_{name}.json"
                        ));
                        IoError::write(&contents, path)?;
                    }
                    Err(source) => {
                        let url = app.url().to_string();
                        let method = "get".to_string();
                        let body = app.params().into_iter().collect::<Vec<(String, String)>>();
                        let mut error = ReqwestError::new(url, method, source);
                        error.with_body(body);
                        return Err(error.into());
                    }
                }
            }
        }
    }
    Ok(())
}
