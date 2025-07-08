use bears_ecology::{Request, bea_data, parameter_values, trace_init};
use bears_species::{
    BeaErr, BeaResponse, Dataset, IoError, ParameterName, ReqwestError, SerdeJson,
};
use strum::IntoEnumIterator;

/// Download valid parameter values for each dataset from the BEA server using the
/// `GetParameterValues` method into the `parameter_values` folder of the `BEA_DATA` directory.
#[tracing::instrument]
pub async fn parameter_values_to_json() -> Result<(), BeaErr> {
    parameter_values().await
}

/// Load parameter values from the file at `path` and parse into the
/// [`BeaResponse`](crate::BeaResponse) type.
///
/// Called by [`parameter_values_from_file`].
#[tracing::instrument(skip_all)]
pub fn parameter_value_from_json(path: std::path::PathBuf) -> Result<(), BeaErr> {
    let file =
        std::fs::File::open(&path).map_err(|e| IoError::new(path, e, line!(), file!().into()))?;
    let rdr = std::io::BufReader::new(file);
    let res: serde_json::Value = serde_json::from_reader(rdr)
        .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
    let data = BeaResponse::try_from(&res)?;
    tracing::info!("Response read.");
    tracing::trace!("Response: {data:#?}");
    Ok(())
}

/// Load parameter values from the `parameter_values` folder of the `BEA_DATA` and serialize into
/// the [`BeaResponse`](crate::BeaResponse) type.
#[tracing::instrument]
pub fn parameter_values_from_file() -> Result<(), BeaErr> {
    trace_init()?;
    dotenvy::dotenv().ok();
    let datasets: Vec<Dataset> = Dataset::iter().collect();
    let path = bea_data()?;
    for dataset in datasets {
        let names = dataset.names();
        for name in names {
            tracing::info!("Response pass for {name} in {dataset}");
            let pv = path.join("parameter_values");
            let path = pv.join(format!("{dataset}_{name}_parameter_values.json"));
            parameter_value_from_json(path)?;
        }
    }
    Ok(())
}

#[tracing::instrument]
pub async fn parameter_value_filtered() -> Result<(), BeaErr> {
    trace_init()?;
    let req = Request::ParameterValueFilter;
    let mut app = req.init()?;
    let datasets: Vec<Dataset> = Dataset::iter().collect();
    for dataset in &datasets {
        let names = dataset.names();
        for name in names {
            if *dataset == Dataset::Nipa && name == ParameterName::Frequency {
                let mut options = app.options().clone();
                let _ = options.with_dataset(*dataset);
                let _ = options.with_target(name);
                app.with_options(options.clone());
                let data = app.get().await?;
                tracing::info!("{data:#?}");
                match data.json::<serde_json::Value>().await {
                    Ok(json) => {
                        tracing::info!("{json:#?}");
                        let contents = serde_json::to_vec(&json)
                            .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
                        dotenvy::dotenv().ok();
                        let path = bea_data()?;
                        let path = path.join("values_api_error.json");
                        std::fs::write(&path, contents)
                            .map_err(|e| IoError::new(path, e, line!(), file!().into()))?;
                    }
                    Err(source) => {
                        let url = app.url().to_string();
                        let method = "get".to_string();
                        let body = app.params().into_iter().collect::<Vec<(String, String)>>();
                        let mut error =
                            ReqwestError::new(url, method, source, line!(), file!().to_string());
                        let _ = error.with_body(body);
                        return Err(error.into());
                    }
                }
            }
        }
    }
    Ok(())
}
