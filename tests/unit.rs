use bea::{trace_init, App, BeaDatasets, BeaError, BeaParameters, Dataset, Json, Method, Options};
use std::io::Read;
use strum::IntoEnumIterator;

// Helper function
// Initiates logging
// Reads environmental variables from .env
// Creates an instance of App
fn init() -> Result<App, BeaError> {
    trace_init();
    tracing::info!("Test logging initialized.");
    dotenvy::dotenv().ok();
    let url = std::env::var("BEA_URL")?;
    let url = url::Url::parse(&url)?;
    let key = std::env::var("API_KEY")?;
    let options = Options::default();
    let app = App::new(key, options, url);
    Ok(app)
}

// Tries to retrieve the list of datasets from BEA.
// Compares the names against the known variants.
// Errors if name is not in the list of known variants.
#[tokio::test]
async fn datasets() -> Result<(), BeaError> {
    let mut app = init()?;
    let mut options = Options::default();
    options.with_method(Method::GetDataSetList);
    app.with_options(options);

    dataset_to_json(&mut app).await?;
    let _datasets = dataset_from_json(&mut app).await?;
    let datasets = deserialize_datasets(&mut app).await?;
    check_datasets(&datasets)?;

    Ok(())
}

// Reads response to json using serde_json.
// Prints the output to the terminal.
// Saves the result to the `data` folder.
async fn dataset_to_json(app: &mut App) -> Result<(), BeaError> {
    // initial pass to log to terminal
    let data = app.get().await?;
    let text = data.text().await?;
    let value: serde_json::Value = serde_json::from_str(&text)?;
    tracing::info!("Value: {value:#?}");

    // another pass to convert to json and save to disk
    let data = app.get().await?;
    let text = data.text().await?;
    let json: serde_json::Value = serde_json::from_str(&text)?;
    let contents = serde_json::to_vec(&json)?;
    // let contents = bincode::serialize(&json)?;
    dotenvy::dotenv().ok();
    let test = std::env::var("TEST")?;
    std::fs::write(format!("{test}/datasets.json"), contents)?;
    Ok(())
}

// Deserialize to JSON using serde_json
// Read from serde_json::Value to BeaDatasets
// Write results to json in "data/bea_datasets.json"
async fn dataset_from_json(app: &mut App) -> Result<BeaDatasets, BeaError> {
    let data = app.get().await?;
    let text = data.text().await?;
    let json: BeaDatasets = serde_json::from_str(&text)?;
    let contents = serde_json::to_vec(&json)?;
    dotenvy::dotenv().ok();
    let test = std::env::var("TEST")?;
    std::fs::write(format!("{test}/bea_datasets.json"), contents)?;
    tracing::info!("JSON Result: {:#?}", json);
    Ok(json)
}

// reads response and native format from file
// avoids making api calls to bea
// used to test internal parsing of responses
#[test]
fn dataset_from_file() -> Result<(), BeaError> {
    trace_init();
    dotenvy::dotenv().ok();
    let test = std::env::var("TEST")?;
    let path = format!("{test}/datasets.json");
    let file = std::fs::File::open(path)?;
    let rdr = std::io::BufReader::new(file);
    let json: BeaDatasets = serde_json::from_reader(rdr)?;
    tracing::info!("Response: {json:#?}");

    let path = format!("{test}/bea_datasets.json");
    let file = std::fs::File::open(path)?;
    let rdr = std::io::BufReader::new(file);
    let json: BeaDatasets = serde_json::from_reader(rdr)?;
    tracing::info!("Native: {json:#?}");
    Ok(())
}

// Deserialize directly from response.
async fn deserialize_datasets(app: &mut App) -> Result<BeaDatasets, BeaError> {
    let data = app.get().await?;
    let datasets = data.json::<BeaDatasets>().await?;
    tracing::info!("Result: {:#?}", datasets);
    Ok(datasets)
}

#[test]
fn diff_datasets() -> Result<(), BeaError> {
    trace_init();
    Json::diff_datasets()?;
    Ok(())
}

// Checks that each dataset returned from the call matches an enum variant in Dataset
// Returns an error if the datasets do not match.
fn check_datasets(data: &BeaDatasets) -> Result<(), BeaError> {
    let sets: Vec<String> = Dataset::iter().map(|d| d.lower()).collect();
    tracing::info!("Sets: {:#?}", sets);

    for dataset in data.results().iter() {
        let name = dataset.dataset_name().to_lowercase();
        if !sets.contains(&name) {
            tracing::warn!("{} not in datasets.", dataset.dataset_name());
            return Err(BeaError::MissingDataset);
        } else {
            tracing::info!("{} in datasets.", dataset.dataset_name());
        }
    }
    Ok(())
}

// For each dataset variant, retrieve the parameter list.
// Save the serde_json::Value version in parameters_{dataset}.json.
// Save the deserialize version in bea_parameters_{dataset}.json.
// Prints to a single line.  Autoformat before running the diff test.
// Save parameters for all datasets in bea_parameters.json.
#[tokio::test]
async fn parameters() -> Result<(), BeaError> {
    let mut app = init()?;
    let mut options = Options::default();
    options.with_method(Method::GetParameterList);
    app.with_options(options.clone());

    let datasets: Vec<Dataset> = Dataset::iter().collect();
    let mut results = Vec::new();
    for dataset in datasets {
        parameters_to_json(&mut app, dataset).await?;
        let _parameters = parameters_from_json(&mut app, dataset).await?;
        let parameters = deserialize_parameters(&mut app, dataset).await?;
        results.push(parameters);
    }
    tracing::info!("Writing results: {:#?}", results);
    let contents = serde_json::to_vec(&results)?;
    dotenvy::dotenv().ok();
    let test = std::env::var("TEST")?;
    std::fs::write(format!("{test}/bea_parameters.json"), contents)?;
    Ok(())
}

// Reads response to json using serde_json.
// Prints the output to the terminal.
// Saves the result to the `data` folder.
async fn parameters_to_json(app: &mut App, dataset: Dataset) -> Result<(), BeaError> {
    let mut opts = app.options().clone();
    opts.with_dataset(dataset);
    app.with_options(opts);
    // initial pass to log to terminal
    let data = app.get().await?;
    let text = data.text().await?;
    let value: serde_json::Value = serde_json::from_str(&text)?;
    tracing::info!("Value: {value:#?}");

    // another pass to convert to json and save to disk
    let data = app.get().await?;
    let text = data.text().await?;
    let json: serde_json::Value = serde_json::from_str(&text)?;
    let contents = serde_json::to_vec(&json)?;
    dotenvy::dotenv().ok();
    let test = std::env::var("TEST")?;
    let path = format!("{test}/parameters_{dataset}.json");
    std::fs::write(path, contents)?;
    Ok(())
}

// Deserialize to JSON using serde_json
// Read from serde_json::Value to BeaParameters
// Write results to json in "data/bea_parameters.json"
async fn parameters_from_json(app: &mut App, dataset: Dataset) -> Result<BeaParameters, BeaError> {
    let mut opts = app.options().clone();
    opts.with_dataset(dataset);
    app.with_options(opts);

    let data = app.get().await?;
    let text = data.text().await?;
    let json: BeaParameters = serde_json::from_str(&text)?;
    let contents = serde_json::to_vec(&json)?;
    dotenvy::dotenv().ok();
    let test = std::env::var("TEST")?;
    let path = format!("{test}/bea_parameters_{dataset}.json");
    std::fs::write(path, contents)?;
    tracing::info!("JSON Result: {:#?}", json);
    Ok(json)
}

// reads response and native format from file
// avoids making api calls to bea
// used to test internal parsing of responses
#[test]
fn parameters_from_file() -> Result<(), BeaError> {
    trace_init();
    dotenvy::dotenv().ok();
    let datasets: Vec<Dataset> = Dataset::iter().collect();
    let test = std::env::var("TEST")?;
    for dataset in datasets {
        // Response pass
        let path = format!("{test}/parameters_{dataset}.json");
        let file = std::fs::File::open(path.clone())?;
        let rdr = std::io::BufReader::new(file);
        tracing::info!("Reading parameters from {path}");
        let json: serde_json::Value = serde_json::from_reader(rdr)?;
        tracing::info!("Response: {json:#?}");

        // Native pass
        let path = format!("data/bea_parameters_{dataset}.json");
        let file = std::fs::File::open(path.clone())?;
        let rdr = std::io::BufReader::new(file);
        tracing::info!("Reading parameters from {path}");
        let json: BeaParameters = serde_json::from_reader(rdr)?;
        tracing::info!("Native: {json:#?}");
    }
    Ok(())
}

// Deserialize directly from response.
async fn deserialize_parameters(
    app: &mut App,
    dataset: Dataset,
) -> Result<BeaParameters, BeaError> {
    let mut opts = app.options().clone();
    opts.with_dataset(dataset);
    app.with_options(opts);

    let data = app.get().await?;
    let parameters = data.json::<BeaParameters>().await?;
    tracing::info!("Result: {:#?}", parameters);
    Ok(parameters)
}

#[test]
fn diff_parameters() -> Result<(), BeaError> {
    trace_init();
    let datasets: Vec<Dataset> = Dataset::iter().collect();
    for dataset in datasets {
        Json::diff_parameters(dataset)?;
    }
    Ok(())
}
