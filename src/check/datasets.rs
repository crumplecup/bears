use crate::{
    trace_init, BeaDatasets, BeaErr, Dataset, DatasetMissing, EnvError, IoError, ReqwestError,
};
use strum::IntoEnumIterator;

/// Reads response to json using serde_json.
/// Saves the result to the `data` folder.
/// Pings the BEA API.
#[tracing::instrument]
pub async fn dataset_to_json() -> Result<(), BeaErr> {
    let req = super::Request::Dataset;
    let app = req.init()?;
    let data = app.get().await?;
    match data.json::<serde_json::Value>().await {
        Ok(json) => {
            let contents = serde_json::to_vec(&json)?;
            dotenvy::dotenv().ok();
            let bea_data = EnvError::from_env("BEA_DATA")?;
            let path = std::path::PathBuf::from(&format!("{bea_data}/datasets.json"));
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

/// reads response and native format from file
/// avoids making api calls to bea
/// used to test internal parsing of responses
#[tracing::instrument]
pub fn datasets_from_file() -> Result<(), BeaErr> {
    trace_init();
    dotenvy::dotenv().ok();
    let bea_data = EnvError::from_env("BEA_DATA")?;
    let path = std::path::PathBuf::from(&format!("{bea_data}/datasets.json"));
    let file = IoError::open(path)?;
    let rdr = std::io::BufReader::new(file);
    let json: BeaDatasets = serde_json::from_reader(rdr)?;
    tracing::info!("Response: {json:#?}");

    let path = std::path::PathBuf::from(&format!("{bea_data}/datasets.bin"));
    let decode = IoError::read(path)?;
    let data = BeaDatasets::deserialize(&decode)?;
    tracing::info!("Native: {data:#?}");
    Ok(())
}

/// Deserialize directly from response.
/// Write to binary format in the BEA_DATA directory.
/// Pings the BEA API.
#[tracing::instrument]
pub async fn deserialize_datasets() -> Result<(), BeaErr> {
    let req = super::Request::Dataset;
    let app = req.init()?;
    let data = app.get().await?;
    match data.json::<BeaDatasets>().await {
        Ok(datasets) => {
            tracing::info!("Result: {:#?}", datasets);
            let encode = BeaDatasets::serialize(&datasets)?;
            dotenvy::dotenv().ok();
            let bea_data = EnvError::from_env("BEA_DATA")?;
            let path = std::path::PathBuf::from(&format!("{bea_data}/datasets.bin"));
            tracing::info!("Writing to path {:?}", path);
            IoError::write(&encode, path)?;
            Ok(())
        }
        Err(source) => {
            let url = app.url().to_string();
            let method = "get".to_string();
            let body = app.params().into_iter().collect::<Vec<(String, String)>>();
            let mut error = ReqwestError::new(url, method, source);
            if !body.is_empty() {
                error.with_body(body);
            }
            Err(error.into())
        }
    }
}

/// Checks that each dataset returned from the call matches an enum variant in Dataset
/// Returns an error if the datasets do not match.
#[tracing::instrument]
pub fn check_datasets() -> Result<(), BeaErr> {
    trace_init();
    dotenvy::dotenv().ok();
    let bea_data = EnvError::from_env("BEA_DATA")?;
    let path = std::path::PathBuf::from(&format!("{bea_data}/datasets.json"));
    let file = IoError::open(path)?;
    let rdr = std::io::BufReader::new(file);
    let data: BeaDatasets = serde_json::from_reader(rdr)?;
    tracing::trace!("Response: {data:#?}");
    let sets: Vec<String> = Dataset::iter().map(|d| d.lower()).collect();
    tracing::info!("Sets: {:#?}", sets);

    for dataset in data.results().iter() {
        let name = dataset.dataset_name().to_lowercase();
        if !sets.contains(&name) {
            tracing::warn!("{} not in datasets.", dataset.dataset_name());
            let error = DatasetMissing::from(dataset.dataset_name());
            return Err(error.into());
        } else {
            tracing::info!("{} in datasets.", dataset.dataset_name());
        }
    }
    Ok(())
}
