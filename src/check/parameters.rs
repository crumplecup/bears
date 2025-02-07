use crate::{
    trace_init, App, BeaErr, BeaResponse, Dataset, EnvError, IoError, Json, ParameterName, Request,
    ReqwestError,
};
use derive_more::FromStr;
use strum::IntoEnumIterator;

/// For each variant of [`Dataset`], request the parameters.
/// Write the results in JSON format to the BEA_DATA directory.
#[tracing::instrument]
pub async fn parameters_to_json() -> Result<(), BeaErr> {
    Dataset::parameters().await
}

#[tracing::instrument(skip_all)]
pub fn parameter_from_json(path: std::path::PathBuf) -> Result<(), BeaErr> {
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
    tracing::info!("Parameters read.");
    tracing::trace!("Response: {data:#?}");
    Ok(())
}

#[tracing::instrument(skip_all)]
pub fn parameter_from_bin(path: std::path::PathBuf) -> Result<(), BeaErr> {
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
pub fn parameters_from_file() -> Result<(), BeaErr> {
    trace_init()?;
    dotenvy::dotenv().ok();
    let datasets: Vec<Dataset> = Dataset::iter().collect();
    let bea_data = EnvError::from_env("BEA_DATA")?;
    for dataset in datasets {
        tracing::info!("Response pass.");
        let path =
            std::path::PathBuf::from(&format!("{bea_data}/parameters/{dataset}_parameters.json"));
        parameter_from_json(path)?;

        tracing::info!("Native pass.");
        let path =
            std::path::PathBuf::from(&format!("{bea_data}/parameters/{dataset}_parameters.bin"));
        tracing::info!("Reading from {path:?}");
        parameter_from_bin(path)?;
    }
    Ok(())
}

#[tracing::instrument]
pub fn parameters_json_to_bin() -> Result<(), BeaErr> {
    trace_init()?;
    dotenvy::dotenv().ok();
    let datasets: Vec<Dataset> = Dataset::iter().collect();
    let bea_data = EnvError::from_env("BEA_DATA")?;
    for dataset in datasets {
        // Set path for json file.
        let path = format!("{bea_data}/parameters/{dataset}_parameters.json");
        let path = std::path::PathBuf::from(path);
        let file = match std::fs::File::open(&path) {
            Ok(f) => f,
            Err(source) => {
                let error = IoError::new(path, source, line!(), file!().to_string());
                return Err(error.into());
            }
        };
        // Create reader from path.
        let rdr = std::io::BufReader::new(file);
        // Deserialize to serde_json::Value.
        let res: serde_json::Value = serde_json::from_reader(rdr)?;
        // Serialize to binary.
        let contents = serde_json::to_vec(&res)?;
        // Set path for binary file.
        let path = format!("{bea_data}/parameters/{dataset}_parameters.bin");
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
    Ok(())
}

/// Deserialize directly from response.
#[tracing::instrument]
pub async fn deserialize_parameters() -> Result<(), BeaErr> {
    let req = Request::Parameter;
    let mut app = req.init()?;
    let datasets: Vec<Dataset> = Dataset::iter().collect();
    for dataset in datasets {
        deserialize_parameter(&mut app, dataset).await?;
    }
    Ok(())
}

/// Deserialize directly from response.
#[tracing::instrument]
pub async fn deserialize_parameter(app: &mut App, dataset: Dataset) -> Result<(), BeaErr> {
    let mut opts = app.options().clone();
    opts.with_dataset(dataset);
    app.add_options(opts);

    let data = app.get().await?;
    match data.json::<serde_json::Value>().await {
        Ok(json) => {
            let params = BeaResponse::try_from(&json)?;
            tracing::trace!("Result: {:#?}", params);
            let contents = BeaResponse::serialize(&params)?;
            dotenvy::dotenv().ok();
            let bea_data = EnvError::from_env("BEA_DATA")?;
            let path = format!("{bea_data}/parameters/{dataset}_parameters.bin");
            tracing::info!("Writing to path {}", path);
            let path = std::path::PathBuf::from(&path);
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
pub fn diff_parameters() -> Result<(), BeaErr> {
    trace_init()?;
    let datasets: Vec<Dataset> = Dataset::iter().collect();
    for dataset in datasets {
        Json::diff_parameters(dataset)?;
    }
    Ok(())
}

#[tracing::instrument]
pub fn parameter_names() -> Result<(), BeaErr> {
    trace_init()?;
    let names = ParameterName::iter()
        .map(|p| p.to_string())
        .collect::<Vec<String>>();
    tracing::info!("Names: {names:?}");
    let mut params = Vec::new();
    let mut fails = Vec::new();
    for name in names {
        if let Ok(param) = ParameterName::from_str(&name) {
            params.push(param);
        } else {
            fails.push(name);
        }
    }
    tracing::info!("Successes: {params:?}");
    tracing::info!("Failures: {fails:?}");

    Ok(())
}
