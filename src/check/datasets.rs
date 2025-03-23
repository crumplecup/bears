use crate::{
    bea_data, trace_init, BeaErr, BeaResponse, Dataset, DatasetMissing, IoError, SerdeJson,
};
use strum::IntoEnumIterator;

/// Queries valid parameter values for the dataset parameter.
/// Reads response to json using serde_json.
/// Saves the result as `dataset.json` in the `BEA_DATA` directory.
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
    Ok(())
}

/// Checks that each dataset returned from the call matches an enum variant in Dataset
/// Returns an error if the datasets do not match.
///
/// Does not test that all variants in `Dataset` are in active use.
#[tracing::instrument]
pub fn check_datasets() -> Result<(), BeaErr> {
    trace_init()?;
    dotenvy::dotenv().ok();
    // Load `datasets.json` into a `BeaResponse` type.
    let path = bea_data()?;
    let path = path.join("datasets.json");
    let file =
        std::fs::File::open(&path).map_err(|e| IoError::new(path, e, line!(), file!().into()))?;
    let rdr = std::io::BufReader::new(file);
    let json: serde_json::Value = serde_json::from_reader(rdr)
        .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
    let data = BeaResponse::try_from(&json)?;
    tracing::trace!("Response: {data:#?}");
    // Create vector of dataset variants.
    let sets: Vec<String> = Dataset::iter().map(|d| d.lower()).collect();
    tracing::info!("Sets: {:#?}", sets);

    // extract `Datasets` variant from `BeaResponse`
    if let Some(datasets) = data.datasets() {
        // for each element, is the name in the set of known datasets?
        for dataset in datasets.iter() {
            // converting both to lowercase avoids the need for special-casing around
            // heterogeneity in capitalization
            let name = dataset.dataset_name().to_lowercase();
            // if not a known name
            if !sets.contains(&name) {
                tracing::warn!("{} not in datasets.", dataset.dataset_name());
                // report the error
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
    // all datasets accounted for
    Ok(())
}
