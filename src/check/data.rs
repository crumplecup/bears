use crate::{BeaErr, Dataset, EnvError, IoError, Nipa, ReqwestError};

/// Pings the BEA API.
#[tracing::instrument]
pub async fn data_to_json() -> Result<(), BeaErr> {
    let req = super::Request::Data;
    let mut app = req.init()?;
    let dataset = Dataset::Nipa;
    dotenvy::dotenv().ok();
    let bea_data = EnvError::from_env("BEA_DATA")?;
    let path = std::path::PathBuf::from(&bea_data);
    let mut params = std::collections::HashMap::new();
    let nipa = Nipa::try_from(&path)?;
    if !nipa.table_name().is_empty() {
        params.insert(
            "TableName".to_string(),
            nipa.table_name()[0].name().to_string(),
        );
    }
    let frequency = "A, Q, M".to_string();
    params.insert("DatasetName".to_string(), dataset.to_string());
    params.insert("Frequency".to_string(), frequency);
    params.insert("Year".to_string(), "ALL".to_string());
    app.with_params(params);

    let data = app.get().await?;
    match data.json::<serde_json::Value>().await {
        Ok(json) => {
            let contents = serde_json::to_vec(&json)?;
            let bea_data = EnvError::from_env("BEA_DATA")?;
            let path = std::path::PathBuf::from(&bea_data);
            let path = path.join(format!("data_{dataset}"));
            if std::fs::exists(&path)? {
                tracing::info!("Target directory for {dataset} found.");
            } else {
                std::fs::DirBuilder::new().create(&path)?;
                tracing::info!("Target directory for {dataset} created.");
            }
            let path = path.join(format!("data_{dataset}.json"));
            match std::fs::write(&path, contents) {
                Ok(()) => Ok(()),
                Err(source) => {
                    let error = IoError::new(path, source, line!(), file!().to_string());
                    Err(error.into())
                }
            }
        }
        Err(source) => {
            let url = app.url().to_string();
            let method = "get".to_string();
            let error = ReqwestError::new(url, method, source, line!(), file!().to_string());
            Err(error.into())
        }
    }
}
