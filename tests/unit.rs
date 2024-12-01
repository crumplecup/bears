use bea::{check, BeaErr};

#[cfg(feature = "api")]
#[tokio::test]
async fn dataset_to_json() -> Result<(), BeaErr> {
    check::dataset_to_json().await
}

#[test]
fn datasets_from_file() -> anyhow::Result<()> {
    check::datasets_from_file()?;
    Ok(())
}

#[cfg(feature = "api")]
#[tokio::test]
async fn deserialize_datasets() -> Result<(), BeaErr> {
    check::deserialize_datasets().await
}

#[test]
fn check_datasets() -> anyhow::Result<()> {
    check::check_datasets()?;
    Ok(())
}

#[cfg(feature = "api")]
#[tokio::test]
async fn parameters_to_json() -> Result<(), BeaErr> {
    check::parameters_to_json().await
}

// reads response and native format from file
// avoids making api calls to bea
// used to test internal parsing of responses
#[test]
fn parameters_from_file() -> anyhow::Result<()> {
    check::parameters_from_file()?;
    Ok(())
}

#[test]
fn parameters_json_to_bin() -> anyhow::Result<()> {
    check::json_to_bin()?;
    Ok(())
}

#[cfg(feature = "api")]
#[tokio::test]
async fn deserialize_parameters() -> Result<(), BeaErr> {
    check::deserialize_parameters().await
}

#[test]
fn diff_parameters() -> anyhow::Result<()> {
    check::diff_parameters()?;
    Ok(())
}

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
