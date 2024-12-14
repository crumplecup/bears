use bea::check;

#[test]
fn parameter_names() -> anyhow::Result<()> {
    check::parameter_names()?;
    Ok(())
}

#[test]
fn io_read() -> anyhow::Result<()> {
    check::io_read()?;
    Ok(())
}

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
    tracing::info!("Serialized into binary.");
    check::datasets_from_file()?;
    tracing::info!("Deserialized from file.");
    check::check_datasets()?;
    tracing::info!("Checked against dataset variants.");
    Ok(())
}

#[tokio::test]
async fn parameters() -> anyhow::Result<()> {
    #[cfg(feature = "api")]
    check::parameters_to_json().await?;
    check::parameters_json_to_bin()?;
    check::parameters_from_file()?;
    Ok(())
}

#[tokio::test]
async fn parameter_values() -> anyhow::Result<()> {
    #[cfg(feature = "api")]
    check::parameter_values_to_json().await?;
    check::parameter_value_json_to_bin()?;
    check::parameter_value_from_file()?;
    Ok(())
}

#[tokio::test]
#[cfg(feature = "api")]
async fn values_filtered() -> anyhow::Result<()> {
    check::values_filtered().await?;
    Ok(())
}

#[tokio::test]
async fn api_error() -> anyhow::Result<()> {
    check::api_error()?;
    Ok(())
}

#[tokio::test]
async fn value_sets() -> anyhow::Result<()> {
    check::value_sets()?;
    Ok(())
}
