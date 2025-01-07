use crate::{
    trace_init, App, BeaErr, BeaResponse, Dataset, EnvError, IoError, ParameterName, ReqwestError,
};
use strum::IntoEnumIterator;

/// After a successfull response from an API request, the goal is to parse the response into
/// internal library data structures.  The JSON responses can include heavily nested data
/// structures, which makes deserializing directly into Rust types a brittle process.  Instead, we
/// first we deserialize the JSON into serde_json types, and then migrate the results into our
/// internal library types using the [`TryFrom`] trait.  While this is a bit heavier on
/// boilerplate, the errors and logs are easier to consume, providing a clearing path to a correct
/// implementation result during the development process.
///
/// Here we request a parameter values table from the server, parse it into serde_json types, and
/// write the results to the `BEA_DATA` directory.  Later we can attempt to parse the response
/// multiple times into our internal library types, succussfully or unsuccessfully, without making
/// repeated API calls to BEA for the same data.
#[tracing::instrument]
pub async fn parameter_values_to_json() -> Result<(), BeaErr> {
    let req = super::Request::ParameterValue;
    let mut app = req.init()?;
    let datasets: Vec<Dataset> = Dataset::iter().collect();
    for dataset in datasets {
        let names = dataset.names();
        for name in names {
            parameter_value_to_json(&mut app, dataset, name).await?;
        }
    }
    Ok(())
}

#[tracing::instrument(skip_all)]
pub async fn parameter_value_to_json(
    app: &mut App,
    dataset: Dataset,
    name: ParameterName,
) -> Result<(), BeaErr> {
    let mut opts = app.options().clone();
    opts.with_dataset(dataset);
    opts.with_param_name(name);
    app.add_options(opts);
    let data = app.get().await?;
    match data.json::<serde_json::Value>().await {
        Ok(json) => {
            let contents = serde_json::to_vec(&json)?;
            dotenvy::dotenv().ok();
            let bea_data = EnvError::from_env("BEA_DATA")?;
            let path = std::path::PathBuf::from(&format!("{bea_data}/parameter_values"));
            if !std::fs::exists(&path)? {
                std::fs::DirBuilder::new().create(&path)?;
                tracing::info!("Target directory for Parameter Values created.");
            }
            let path = path.join(format!("{dataset}_{name}_parameter_values.json"));
            match std::fs::write(&path, contents) {
                Ok(()) => Ok(()),
                Err(source) => {
                    let error = IoError::new(path, source, line!(), file!().to_string());
                    Err(error.into())
                }
            }
        }
        Err(source) => {
            let url = app.url().to_string();
            let method = "get".to_string();
            let body = app.params().into_iter().collect::<Vec<(String, String)>>();
            let mut error = ReqwestError::new(url, method, source, line!(), file!().to_string());
            error.with_body(body);
            Err(error.into())
        }
    }
}

#[tracing::instrument]
pub fn parameter_value_json_to_bin() -> Result<(), BeaErr> {
    trace_init();
    dotenvy::dotenv().ok();
    let datasets: Vec<Dataset> = Dataset::iter().collect();
    let bea_data = EnvError::from_env("BEA_DATA")?;
    for dataset in datasets {
        let names = dataset.names();
        for name in names {
            // Set path for json file.
            let path =
                format!("{bea_data}/parameter_values/{dataset}_{name}_parameter_values.json");
            let path = std::path::PathBuf::from(path);
            // Create reader from path.
            let file = match std::fs::File::open(&path) {
                Ok(f) => f,
                Err(source) => {
                    let error = IoError::new(path, source, line!(), file!().to_string());
                    return Err(error.into());
                }
            };
            let rdr = std::io::BufReader::new(file);
            // Deserialize to serde_json::Value.
            let res: serde_json::Value = serde_json::from_reader(rdr)?;
            // Serialize to binary.
            let contents = serde_json::to_vec(&res)?;
            // Set path for binary file.
            let path = format!("{bea_data}/parameter_values/{dataset}_{name}_parameter_values.bin");
            let path = std::path::PathBuf::from(path);
            // Write binary to file.
            match std::fs::write(&path, contents) {
                Ok(()) => {}
                Err(source) => {
                    let error = IoError::new(path, source, line!(), file!().to_string());
                    return Err(error.into());
                }
            }
        }
    }
    Ok(())
}

#[tracing::instrument(skip_all)]
pub fn parameter_value_from_json(path: std::path::PathBuf) -> Result<(), BeaErr> {
    let file = match std::fs::File::open(&path) {
        Ok(f) => f,
        Err(source) => {
            let error = IoError::new(path, source, line!(), file!().to_string());
            return Err(error.into());
        }
    };
    let rdr = std::io::BufReader::new(file);
    let res: serde_json::Value = serde_json::from_reader(rdr)?;
    let data = BeaResponse::try_from(&res)?;
    tracing::info!("Response read.");
    tracing::trace!("Response: {data:#?}");
    Ok(())
}

#[tracing::instrument(skip_all)]
pub fn parameter_value_from_bin(path: std::path::PathBuf) -> Result<(), BeaErr> {
    let decode = match std::fs::read(&path) {
        Ok(data) => data,
        Err(source) => {
            let error = IoError::new(path, source, line!(), file!().to_string());
            return Err(error.into());
        }
    };
    tracing::info!("Path read.");
    let data: serde_json::Value = serde_json::from_slice(&decode)?;
    let data = BeaResponse::try_from(&data)?;
    tracing::info!("Native: {data:#?}");
    Ok(())
}

/// reads response and native format from file
/// avoids making api calls to bea
/// used to test internal parsing of responses
#[tracing::instrument]
pub fn parameter_value_from_file() -> Result<(), BeaErr> {
    trace_init();
    dotenvy::dotenv().ok();
    let datasets: Vec<Dataset> = Dataset::iter().collect();
    let bea_data = EnvError::from_env("BEA_DATA")?;
    for dataset in datasets {
        let names = dataset.names();
        for name in names {
            tracing::info!("Response pass for {name} in {dataset}");
            let path = std::path::PathBuf::from(&format!(
                "{bea_data}/parameter_values/{dataset}_{name}_parameter_values.json"
            ));
            parameter_value_from_json(path)?;

            tracing::info!("Native pass for {name} in {dataset}");
            let path = std::path::PathBuf::from(&format!(
                "{bea_data}/parameter_values/{dataset}_{name}_parameter_values.bin"
            ));
            tracing::info!("Reading from {path:?}");
            parameter_value_from_bin(path)?;
        }
    }
    Ok(())
}

#[tracing::instrument]
pub async fn parameter_value_filtered() -> Result<(), BeaErr> {
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
                app.add_options(options.clone());
                let data = app.get().await?;
                tracing::info!("{data:#?}");
                match data.json::<serde_json::Value>().await {
                    Ok(json) => {
                        tracing::info!("{json:#?}");
                        // let bea = BeaResponse::try_from(&json)?;
                        // match bea.results() {
                        //     Results::ParameterValues(pv) => {}
                        //     Results::ApiError(e) => {
                        //         tracing::info!("{e}");
                        //     }
                        //     unexpected => {
                        //         tracing::warn!("Unexpected type {unexpected:#?}");
                        //     }
                        // }

                        let contents = serde_json::to_vec(&json)?;
                        dotenvy::dotenv().ok();
                        let bea_data = EnvError::from_env("BEA_DATA")?;
                        let path = std::path::PathBuf::from(&format!(
                            "{bea_data}/values_api_error.json" // "{bea_data}/values_{dataset}_{name}.json"
                        ));
                        match std::fs::write(&path, contents) {
                            Ok(()) => {}
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
                        let body = app.params().into_iter().collect::<Vec<(String, String)>>();
                        let mut error =
                            ReqwestError::new(url, method, source, line!(), file!().to_string());
                        error.with_body(body);
                        return Err(error.into());
                    }
                }
            }
        }
    }
    Ok(())
}
