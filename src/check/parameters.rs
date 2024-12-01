use crate::{
    trace_init, App, BeaErr, BeaParameters, Check, Dataset, EnvError, IoError, Json, ParameterName,
    ReqwestError,
};
use derive_more::FromStr;
use strum::IntoEnumIterator;

/// For each variant of [`Dataset`], request the parameters.
/// Write the results in JSON format to the BEA_DATA directory.
#[tracing::instrument]
pub async fn parameters_to_json() -> Result<(), BeaErr> {
    let req = super::Request::Parameter;
    let mut app = req.init()?;
    let datasets: Vec<Dataset> = Dataset::iter().collect();
    for dataset in datasets {
        parameter_to_json(&mut app, dataset).await?;
    }
    Ok(())
}

/// Reads response to json using serde_json.
/// Prints the output to the terminal.
/// Saves the result to the `BEA_DATA` directory.
#[tracing::instrument(skip_all)]
pub async fn parameter_to_json(app: &mut App, dataset: Dataset) -> Result<(), BeaErr> {
    let mut opts = app.options().clone();
    opts.with_dataset(dataset);
    app.with_options(opts);
    let data = app.get().await?;
    match data.json::<serde_json::Value>().await {
        Ok(json) => {
            let contents = serde_json::to_vec(&json)?;
            dotenvy::dotenv().ok();
            let bea_data = EnvError::from_env("BEA_DATA")?;
            let path = std::path::PathBuf::from(&format!("{bea_data}/parameters_{dataset}.json"));
            IoError::write(&contents, path)?;
            Ok(())
        }
        Err(source) => {
            let url = app.url().to_string();
            let method = "get".to_string();
            let error = ReqwestError::new(url, method, source);
            Err(error.into())
        }
    }
}

#[tracing::instrument(skip_all)]
pub fn parameter_from_json(path: std::path::PathBuf) -> Result<(), BeaErr> {
    let file = IoError::open(path)?;
    let rdr = std::io::BufReader::new(file);
    let res: serde_json::Value = serde_json::from_reader(rdr)?;
    let data = BeaParameters::try_from(&res)?;
    tracing::info!("Parameters read.");
    tracing::trace!("Response: {data:#?}");
    Ok(())
}

#[tracing::instrument(skip_all)]
pub fn parameter_from_bin(path: std::path::PathBuf) -> Result<(), BeaErr> {
    let decode = IoError::read(path)?;
    tracing::info!("Path read.");
    let data = BeaParameters::deserialize(&decode)?;
    tracing::info!("Native: {data:#?}");
    Ok(())
}

/// reads response and native format from file
/// avoids making api calls to bea
/// used to test internal parsing of responses
#[tracing::instrument]
pub fn parameters_from_file() -> Result<(), BeaErr> {
    trace_init();
    dotenvy::dotenv().ok();
    let datasets: Vec<Dataset> = Dataset::iter().collect();
    let bea_data = EnvError::from_env("BEA_DATA")?;
    let fails_res = 0;
    // let mut fails_res = 0;
    let mut fails_nat = 0;
    for dataset in datasets {
        tracing::info!("Response pass.");
        let path = std::path::PathBuf::from(&format!("{bea_data}/parameters_{dataset}.json"));
        parameter_from_json(path)?;
        // if parameter_from_json(path).is_err() {
        //     fails_res += 1;
        // }

        tracing::info!("Native pass.");
        let path = std::path::PathBuf::from(&format!("{bea_data}/parameters_{dataset}.bin"));
        tracing::info!("Reading from {path:?}");
        // parameter_from_bin(path)?;
        if parameter_from_bin(path).is_err() {
            fails_nat += 1;
        }
    }
    if fails_res + fails_nat > 0 {
        tracing::warn!("Fails from response: {fails_res}");
        tracing::warn!("Fails from native: {fails_nat}");

        let desc = format!("response: {fails_res}, native: {fails_nat}");
        let error = Check::new(desc);
        Err(error.into())
    } else {
        Ok(())
    }
}

pub fn json_to_bin() -> Result<(), BeaErr> {
    trace_init();
    dotenvy::dotenv().ok();
    let datasets: Vec<Dataset> = Dataset::iter().collect();
    let bea_data = EnvError::from_env("BEA_DATA")?;
    for dataset in datasets {
        // Set path for json file.
        let path = format!("{bea_data}/parameters_{dataset}.json");
        let path = std::path::PathBuf::from(path);
        // Create reader from path.
        let file = IoError::open(path)?;
        let rdr = std::io::BufReader::new(file);
        // Deserialize to serde_json::Value.
        let res: serde_json::Value = serde_json::from_reader(rdr)?;
        // Parse to BeaParameters.
        let data = BeaParameters::try_from(&res)?;
        // Serialize to binary.
        let contents = data.serialize()?;
        // Set path for binary file.
        let path = format!("{bea_data}/parameters_{dataset}.bin");
        let path = std::path::PathBuf::from(path);
        // Write binary to file.
        IoError::write(&contents, path)?;
    }
    Ok(())
}

/// Deserialize directly from response.
#[tracing::instrument]
pub async fn deserialize_parameters() -> Result<(), BeaErr> {
    let req = super::Request::Parameter;
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
            let params = BeaParameters::try_from(&json)?;
            tracing::trace!("Result: {:#?}", params);
            let encode = BeaParameters::serialize(&params)?;
            dotenvy::dotenv().ok();
            let bea_data = EnvError::from_env("BEA_DATA")?;
            let path = format!("{bea_data}/parameters_{dataset}.bin");
            tracing::info!("Writing to path {}", path);
            let path = std::path::PathBuf::from(&path);
            IoError::write(&encode, path)?;
            Ok(())
        }
        Err(source) => {
            let url = app.url().to_string();
            let method = "get".to_string();
            let body = app.params().into_iter().collect::<Vec<(String, String)>>();
            let mut error = ReqwestError::new(url, method, source);
            error.with_body(body);
            Err(error.into())
        }
    }
}

#[tracing::instrument]
pub fn diff_parameters() -> Result<(), BeaErr> {
    trace_init();
    let datasets: Vec<Dataset> = Dataset::iter().collect();
    for dataset in datasets {
        Json::diff_parameters(dataset)?;
    }
    Ok(())
}

#[tracing::instrument]
pub fn parameter_names() -> Result<(), BeaErr> {
    trace_init();
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
