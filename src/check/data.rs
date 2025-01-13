use crate::{
    trace_init, BeaErr, BeaResponse, Data, Dataset, EnvError, IoError, NiUnderlyingDetail, Nipa,
    Request, ReqwestError,
};

/// Pings the BEA API.
#[tracing::instrument]
pub async fn data_to_json() -> Result<(), BeaErr> {
    let req = Request::Data;
    let mut app = req.init()?;
    let dataset = Dataset::NIUnderlyingDetail;
    let mut opts = app.options().clone();
    opts.with_dataset(dataset);
    app.add_options(opts);
    dotenvy::dotenv().ok();
    let bea_data = EnvError::from_env("BEA_DATA")?;
    let path = std::path::PathBuf::from(&bea_data);
    let nipa = NiUnderlyingDetail::try_from(&path)?;
    for params in nipa.iter().skip(16) {
        tracing::info!("{params:#?}");
        app.with_params(params.clone());
        let data = app.get().await?;
        match data.json::<serde_json::Value>().await {
            Ok(json) => {
                let contents = serde_json::to_vec(&json)?;
                let bea_data = EnvError::from_env("BEA_DATA")?;
                let path = std::path::PathBuf::from(&bea_data);
                let path = path.join("data");
                if !std::fs::exists(&path)? {
                    std::fs::DirBuilder::new().create(&path)?;
                    tracing::info!("Target directory for Data created.");
                }
                let path = path.join(format!("{dataset}"));
                if !std::fs::exists(&path)? {
                    std::fs::DirBuilder::new().create(&path)?;
                    tracing::info!("Target directory for {dataset} created.");
                }
                let path = path.join(format!("{dataset}_{}.json", params["TableName"]));
                match std::fs::write(&path, contents) {
                    Ok(()) => {}
                    Err(source) => {
                        let error = IoError::new(path, source, line!(), file!().to_string());
                        return Err(error.into());
                    }
                }
            }
            Err(source) => {
                let url = app.url().to_string();
                let method = "get".to_string();
                let error = ReqwestError::new(url, method, source, line!(), file!().to_string());
                return Err(error.into());
            }
        }
    }
    Ok(())
}

#[tracing::instrument(skip_all)]
pub fn data_from_json() -> Result<(), BeaErr> {
    trace_init();
    dotenvy::dotenv().ok();
    let bea_data = EnvError::from_env("BEA_DATA")?;
    let path = std::path::PathBuf::from(&bea_data);
    let dataset = Dataset::Nipa;
    let path = path.join(format!("data/{dataset}/{dataset}_T10101.json"));
    let file = match std::fs::File::open(&path) {
        Ok(f) => f,
        Err(source) => {
            let error = IoError::new(path, source, line!(), file!().to_string());
            return Err(error.into());
        }
    };
    let rdr = std::io::BufReader::new(file);
    let res: serde_json::Value = serde_json::from_reader(rdr)?;
    let data = BeaResponse::try_from(&res)?;
    tracing::info!("Response read.");
    tracing::trace!("Response: {data:#?}");
    let results = data.results();
    if let Some(data) = results.into_data() {
        match data {
            Data::Nipa(nipa) => {
                tracing::info!("{} Nipa records read.", nipa.len());
            }
        }
    }
    Ok(())
}
