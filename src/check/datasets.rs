use crate::{
    bea_data, trace_init, BeaErr, BeaResponse, Dataset, DatasetMissing, IoError, Request,
    ReqwestError, SerdeJson,
};
use strum::IntoEnumIterator;

/// Reads response to json using serde_json.
/// Saves the result to the `data` folder.
/// Pings the BEA API.
#[tracing::instrument]
pub async fn datasets_to_json() -> Result<(), BeaErr> {
    Dataset::get().await
}

/// reads response and native format from file
/// avoids making api calls to bea
/// used to test internal parsing of responses
#[tracing::instrument]
pub fn datasets_from_file() -> Result<(), BeaErr> {
    trace_init()?;
    dotenvy::dotenv().ok();
    let bea_data = bea_data()?;
    tracing::info!("BEA_DATA present.");
    let path = bea_data.join("datasets.json");
    let file =
        std::fs::File::open(&path).map_err(|e| IoError::new(path, e, line!(), file!().into()))?;
    tracing::info!("File opened.");
    let rdr = std::io::BufReader::new(file);
    let json: serde_json::Value = serde_json::from_reader(rdr)
        .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
    let bea = BeaResponse::try_from(&json)?;
    tracing::info!("Response: {bea:#?}");

    // TODO: Remove
    let path = bea_data.join("datasets.bin");
    tracing::info!("Reading {path:?}.");
    let decode =
        std::fs::read(&path).map_err(|e| IoError::new(path, e, line!(), file!().into()))?;
    tracing::info!("File read to Vec<u8>.");
    let data: serde_json::Value = serde_json::from_slice(&decode)
        .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
    let data = BeaResponse::try_from(&data)?;
    tracing::info!("Native: {data:#?}");
    Ok(())
}

/// Deserialize directly from response.
/// Write to binary format in the BEA_DATA directory.
/// Pings the BEA API.
/// TODO: Remove
#[tracing::instrument]
pub async fn deserialize_datasets() -> Result<(), BeaErr> {
    let req = Request::Dataset;
    let app = req.init()?;
    let data = app.get().await?;
    match data.json::<BeaResponse>().await {
        Ok(datasets) => {
            tracing::info!("Result: {:#?}", datasets);
            let contents = BeaResponse::serialize(&datasets)?;
            dotenvy::dotenv().ok();
            let path = bea_data()?;
            let path = path.join("datasets.bin");
            tracing::info!("Writing to path {:?}", path);
            std::fs::write(&path, contents)
                .map_err(|e| IoError::new(path, e, line!(), file!().into()).into())
        }
        Err(source) => {
            let url = app.url().to_string();
            let method = "get".to_string();
            let body = app.params().into_iter().collect::<Vec<(String, String)>>();
            let mut error = ReqwestError::new(url, method, source, line!(), file!().to_string());
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
    trace_init()?;
    dotenvy::dotenv().ok();
    let path = bea_data()?;
    let path = path.join("datasets.json");
    let file =
        std::fs::File::open(&path).map_err(|e| IoError::new(path, e, line!(), file!().into()))?;
    let rdr = std::io::BufReader::new(file);
    let json: serde_json::Value = serde_json::from_reader(rdr)
        .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
    let data = BeaResponse::try_from(&json)?;
    tracing::trace!("Response: {data:#?}");
    let sets: Vec<String> = Dataset::iter().map(|d| d.lower()).collect();
    tracing::info!("Sets: {:#?}", sets);

    if let Some(datasets) = data.datasets() {
        for dataset in datasets.iter() {
            let name = dataset.dataset_name().to_lowercase();
            if !sets.contains(&name) {
                tracing::warn!("{} not in datasets.", dataset.dataset_name());
                let error = DatasetMissing::new(
                    dataset.dataset_name().to_string(),
                    line!(),
                    file!().to_string(),
                );
                return Err(error.into());
            } else {
                tracing::info!("{} in datasets.", dataset.dataset_name());
            }
        }
    }
    Ok(())
}

// TODO: Remove
#[tracing::instrument]
pub fn datasets_json_to_bin() -> Result<(), BeaErr> {
    trace_init()?;
    dotenvy::dotenv().ok();
    let bea_data = bea_data()?;
    let path = bea_data.join("datasets.json");
    // Create reader from path.
    let file =
        std::fs::File::open(&path).map_err(|e| IoError::new(path, e, line!(), file!().into()))?;
    let rdr = std::io::BufReader::new(file);
    // Deserialize to serde_json::Value.
    let res: serde_json::Value = serde_json::from_reader(rdr)
        .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
    // Serialize to binary.
    let contents =
        serde_json::to_vec(&res).map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
    // Set path for binary file.
    let path = bea_data.join("datasets.bin");
    // Write binary to file.
    std::fs::write(&path, contents)
        .map_err(|e| IoError::new(path, e, line!(), file!().into()).into())
}
