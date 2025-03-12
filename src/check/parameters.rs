use crate::{
    bea_data, trace_init, BeaErr, BeaResponse, Dataset, IoError, Json, ParameterName, SerdeJson,
};
use std::str::FromStr;
use strum::IntoEnumIterator;

/// For each variant of [`Dataset`], request the parameters.
/// Write the results in JSON format to the BEA_DATA directory.
#[tracing::instrument]
pub async fn parameters_to_json() -> Result<(), BeaErr> {
    Dataset::parameters().await
}

#[tracing::instrument(skip_all)]
pub fn parameter_from_json(path: std::path::PathBuf) -> Result<(), BeaErr> {
    let file =
        std::fs::File::open(&path).map_err(|e| IoError::new(path, e, line!(), file!().into()))?;
    let rdr = std::io::BufReader::new(file);
    let res: serde_json::Value = serde_json::from_reader(rdr)
        .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
    let data = BeaResponse::try_from(&res)?;
    tracing::info!("Parameters read.");
    tracing::trace!("Response: {data:#?}");
    Ok(())
}

#[tracing::instrument(skip_all)]
pub fn parameter_from_bin(path: std::path::PathBuf) -> Result<(), BeaErr> {
    let decode =
        std::fs::read(&path).map_err(|e| IoError::new(path, e, line!(), file!().into()))?;
    tracing::info!("Path read.");
    let data: serde_json::Value = serde_json::from_slice(&decode)
        .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
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
    let path = bea_data()?;
    let path = path.join("parameters");
    for dataset in datasets {
        let res = path.join(format!("{dataset}_parameters.json"));
        parameter_from_json(res)?;
    }
    Ok(())
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
