use crate::{
    bea_data, trace_init, App, BeaErr, BeaResponse, Dataset, IoError, Json, ParameterName, Request,
    ReqwestError, SerdeJson,
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
        tracing::info!("Response pass.");
        let res = path.join(format!("{dataset}_parameters.json"));
        parameter_from_json(res)?;

        tracing::info!("Native pass.");
        let nat = path.join(format!("{dataset}_parameters.json"));
        tracing::info!("Reading from {nat:?}");
        parameter_from_bin(nat)?;
    }
    Ok(())
}

#[tracing::instrument]
pub fn parameters_json_to_bin() -> Result<(), BeaErr> {
    trace_init()?;
    dotenvy::dotenv().ok();
    let datasets: Vec<Dataset> = Dataset::iter().collect();
    let path = bea_data()?;
    let path = path.join("parameters");
    for dataset in datasets {
        // Set path for json file.
        let json = path.join(format!("{dataset}_parameters.json"));
        let file = std::fs::File::open(&json)
            .map_err(|e| IoError::new(json, e, line!(), file!().into()))?;
        // Create reader from path.
        let rdr = std::io::BufReader::new(file);
        // Deserialize to serde_json::Value.
        let res: serde_json::Value = serde_json::from_reader(rdr)
            .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
        // Serialize to binary.
        let contents = serde_json::to_vec(&res)
            .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
        // Set path for binary file.
        let bin = path.join(format!("{dataset}_parameters.bin"));
        // Write binary to file.
        std::fs::write(&bin, contents)
            .map_err(|e| IoError::new(bin, e, line!(), file!().into()))?;
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
    app.with_options(opts);

    let data = app.get().await?;
    match data.json::<serde_json::Value>().await {
        Ok(json) => {
            let params = BeaResponse::try_from(&json)?;
            tracing::trace!("Result: {:#?}", params);
            let contents = BeaResponse::serialize(&params)?;
            dotenvy::dotenv().ok();
            let path = bea_data()?;
            let path = path.join("parameters");
            let path = path.join(format!("{dataset}_parameters.bin"));
            tracing::info!("Writing to path {:?}", path);
            std::fs::write(&path, contents)
                .map_err(|e| IoError::new(path, e, line!(), file!().into()).into())
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
