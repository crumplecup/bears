use bears::check;

#[test]
fn parameter_names() -> anyhow::Result<()> {
    check::parameter_names()?;
    Ok(())
}

// #[test]
// fn io_read() -> anyhow::Result<()> {
//     check::io_read()?;
//     Ok(())
// }

#[test]
fn env() -> anyhow::Result<()> {
    check::env()?;
    Ok(())
}

#[tokio::test]
async fn datasets() -> anyhow::Result<()> {
    #[cfg(feature = "api")]
    check::datasets_to_json().await?;
    #[cfg(feature = "api")]
    tracing::info!("Response received from BEA API.");
    check::datasets_json_to_bin()?;
    tracing::info!("Datasets serialized into binary.");
    check::datasets_from_file()?;
    tracing::info!("Datasets deserialized from file.");
    check::check_datasets()?;
    tracing::info!("Checked against dataset variants.");
    Ok(())
}

#[tokio::test]
async fn parameters() -> anyhow::Result<()> {
    #[cfg(feature = "api")]
    check::parameters_to_json().await?;
    #[cfg(feature = "api")]
    tracing::info!("Response received from BEA API.");
    check::parameters_json_to_bin()?;
    tracing::info!("Parameters serialized into binary.");
    check::parameters_from_file()?;
    tracing::info!("Parameters deserialized from file.");
    Ok(())
}

#[tokio::test]
async fn parameter_values() -> anyhow::Result<()> {
    #[cfg(feature = "api")]
    check::parameter_values_to_json().await?;
    #[cfg(feature = "api")]
    tracing::info!("Response received from BEA API.");
    check::parameter_value_json_to_bin()?;
    tracing::info!("Parameter Values serialized into binary.");
    check::parameter_value_from_file()?;
    tracing::info!("Parameter values deserialized from file.");
    Ok(())
}

// try to retrieve parameter values for all datasets and all parameters
#[tokio::test]
#[cfg(feature = "api")]
async fn values_filtered_search() -> anyhow::Result<()> {
    check::values_filtered().await?;
    Ok(())
}

// retreive parameter values with known valid responses
#[tokio::test]
#[cfg(feature = "api")]
async fn values_filtered_subset() -> anyhow::Result<()> {
    check::values_filtered_subset().await?;
    Ok(())
}

#[tokio::test]
#[cfg(feature = "api")]
async fn values_gdp_filtered() -> anyhow::Result<()> {
    check::values_gdp_filtered().await?;
    tracing::info!("Filtered values for GDP read.");
    check::values_ugdp_filtered().await?;
    tracing::info!("Filtered values for Underlying GDP read.");
    Ok(())
}

#[tokio::test]
async fn api_error() -> anyhow::Result<()> {
    check::api_error()?;
    check::requests_exceeded()?;
    Ok(())
}

#[test]
fn value_sets() -> anyhow::Result<()> {
    check::value_sets()?;
    Ok(())
}

#[tokio::test]
async fn data_to_json() -> anyhow::Result<()> {
    #[cfg(feature = "api")]
    check::data_to_json().await?;
    Ok(())
}

#[tokio::test]
async fn data_from_json() -> anyhow::Result<()> {
    check::data_from_json().await?;
    Ok(())
}

#[test]
fn naics() -> anyhow::Result<()> {
    check::naics()?;
    Ok(())
}

#[test]
fn download_history() -> anyhow::Result<()> {
    check::download_history()?;
    Ok(())
}

#[tokio::test]
async fn download_summary() -> anyhow::Result<()> {
    check::download_summary().await?;
    Ok(())
}
