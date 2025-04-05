use bears_ecology::{bea_data, trace_init, values, values_gdp, values_subset, values_ugdp};
use bears_species::{BeaErr, BeaResponse, IoError, SerdeJson};

/// Calls a known bad combination of parameters to generate an API Error as a response.
/// Writes the JSON representation of the error to the BEA_DATA directory.
#[tracing::instrument]
pub fn api_error() -> Result<(), BeaErr> {
    trace_init()?;
    dotenvy::dotenv().ok();
    let path = bea_data()?;
    let path = path.join("values_api_error.json");
    let file =
        std::fs::File::open(&path).map_err(|e| IoError::new(path, e, line!(), file!().into()))?;
    let rdr = std::io::BufReader::new(file);
    let res: serde_json::Value = serde_json::from_reader(rdr)
        .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
    let data = BeaResponse::try_from(&res)?;
    tracing::info!("Response read.");
    tracing::info!("Response: {data:#?}");

    Ok(())
}

/// Attempts to parse the file `requests_exceeded.json` in the `BEA_DATA` directory to the [`BeaResponse`](crate::BeaResponse)
/// type.  Verifies that the program accurately recognizing this error condition from the BEA
/// server.
#[tracing::instrument]
pub fn requests_exceeded() -> Result<(), BeaErr> {
    trace_init()?;
    dotenvy::dotenv().ok();
    let path = bea_data()?;
    let path = path.join("requests_exceeded.json");
    let file =
        std::fs::File::open(&path).map_err(|e| IoError::new(path, e, line!(), file!().into()))?;
    let rdr = std::io::BufReader::new(file);
    let res: serde_json::Value = serde_json::from_reader(rdr)
        .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
    let data = BeaResponse::try_from(&res)?;
    tracing::info!("Response read.");
    tracing::info!("Response: {data:#?}");

    Ok(())
}

/// For each variant of [`Dataset`], request the valid range of values for each parameter name.
/// BEA has not implemented this method for all parameters, so we expect some calls to fail.
///
/// The GdpByIndustry and UnderlyingGdpByIndustry datasets require additional parameters for some
/// keys.
#[tracing::instrument]
pub async fn values_filtered() -> Result<(), BeaErr> {
    values().await
}

/// For each variant of [`Dataset`], request the valid range of values for each parameter name.
/// The `subset` variant of this method only requests data for datasets where the BEA has
/// implemented a response for each parameter name associated with the dataset.
#[tracing::instrument]
pub async fn values_filtered_subset() -> Result<(), BeaErr> {
    values_subset().await
}

/// Two parameters in the GdpByIndustry dataset have valid input sets that vary by table_id, namely
/// Year and Industry.  Obtain table ids using [`Method::GetParameterValues`](crate::Method::GetParameterValues) prior to running this
/// check. For these two parameters, we obtain a response for each table_id and write the result to
/// a folder in the BEA_DATA directory.
///
/// Due to the nested call to [`GdpByIndustry::read_table_id`], we have seperate checks for GDP and
/// Underlying GDP.  Less dry but somewhat clearer to write and read.
#[tracing::instrument]
pub async fn values_gdp_filtered() -> Result<(), BeaErr> {
    values_gdp().await
}

/// Two parameters in the UnderlyingGdpByIndustry dataset have valid input sets that vary by table_id, namely
/// Year and Industry.  Obtain table ids using [`Method::GetParameterValues`](crate::Method::GetParameterValues) prior to running this
/// check. For these two parameters, we obtain a response for each table_id and write the result to
/// a folder in the BEA_DATA directory.
///
/// Due to the nested call to [`UnderlyingGdpByIndustry::read_table_id`], we have seperate checks for GDP and
/// Underlying GDP.  Less dry but somewhat clearer to write and read.
#[tracing::instrument]
pub async fn values_ugdp_filtered() -> Result<(), BeaErr> {
    values_ugdp().await
}
