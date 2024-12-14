use crate::{
    trace_init, ApiMetadata, BeaErr, BeaResponse, Dataset, EnvError, Iip, InputOutput, IntlServSta,
    IntlServTrade, IoError, Ita, ParameterName, Regional, ReqwestError, Results,
};
use strum::IntoEnumIterator;

#[tracing::instrument]
pub fn api_error() -> Result<(), BeaErr> {
    trace_init();
    dotenvy::dotenv().ok();
    let bea_data = EnvError::from_env("BEA_DATA")?;
    let path = std::path::PathBuf::from(&format!("{bea_data}/values_api_error.json"));
    let file = IoError::open(path)?;
    let rdr = std::io::BufReader::new(file);
    let res: serde_json::Value = serde_json::from_reader(rdr)?;
    let data = BeaResponse::try_from(&res)?;
    tracing::info!("Response read.");
    tracing::info!("Response: {data:#?}");

    Ok(())
}

#[tracing::instrument]
pub async fn values_filtered() -> Result<(), BeaErr> {
    let req = super::Request::ParameterValueFilter;
    let mut app = req.init()?;
    let datasets: Vec<Dataset> = Dataset::iter().collect();
    for dataset in &datasets {
        let names = dataset.names();
        for name in names {
            if *dataset == Dataset::GDPbyIndustry && name == ParameterName::Industry
            // && name != ParameterName::Industry
            // || *dataset == Dataset::GDPbyIndustry
            // || *dataset == Dataset::APIDatasetMetadata
            {
                let mut options = app.options().clone();
                options.with_dataset(*dataset);
                options.with_target(name);
                app.with_options(options.clone());
                let data = app.get().await?;
                tracing::info!("{data:#?}");
                match data.json::<serde_json::Value>().await {
                    Ok(json) => {
                        tracing::info!("{json:#?}");
                        let bea = BeaResponse::try_from(&json)?;
                        match bea.results() {
                            Results::ApiError(e) => {
                                tracing::info!("{e}");
                            }
                            Results::ParameterValues(_) => {
                                let contents = serde_json::to_vec(&json)?;
                                dotenvy::dotenv().ok();
                                let bea_data = EnvError::from_env("BEA_DATA")?;
                                let path = std::path::PathBuf::from(&format!(
                                    "{bea_data}/values_{dataset}_{name}.json"
                                ));
                                IoError::write(&contents, path)?;
                            }
                            unexpected => {
                                tracing::warn!("Unexpected type {unexpected:#?}");
                            }
                        }
                    }
                    Err(source) => {
                        let url = app.url().to_string();
                        let method = "get".to_string();
                        let body = app.params().into_iter().collect::<Vec<(String, String)>>();
                        let mut error = ReqwestError::new(url, method, source);
                        error.with_body(body);
                        return Err(error.into());
                    }
                }
            }
        }
    }
    Ok(())
}

#[tracing::instrument]
pub fn value_sets() -> Result<(), BeaErr> {
    trace_init();
    dotenvy::dotenv().ok();
    let bea_data = EnvError::from_env("BEA_DATA")?;
    let path = std::path::PathBuf::from(bea_data);
    let datasets: Vec<Dataset> = Dataset::iter().collect();
    for dataset in &datasets {
        if *dataset == Dataset::APIDatasetMetadata {
            match dataset {
                Dataset::APIDatasetMetadata => {
                    let set = ApiMetadata::try_from(&path)?;
                    tracing::info!("{dataset} values read.");
                    tracing::info!("{dataset} has {} Metadata values.", set.len());
                }
                Dataset::Iip => {
                    let set = Iip::try_from(&path)?;
                    tracing::info!("{dataset} values read.");
                    tracing::info!("{dataset} has {} Component values.", set.component().len());
                    tracing::info!("{dataset} has {} Frequency values.", set.frequency().len());
                    tracing::info!(
                        "{dataset} has {} TypeOfInvestment values.",
                        set.type_of_investment().len()
                    );
                    tracing::info!("{dataset} has {} Year values.", set.year().len());
                }
                Dataset::InputOutput => {
                    let set = InputOutput::try_from(&path)?;
                    tracing::info!("{dataset} values read.");
                    tracing::info!("{dataset} has {} TableID values.", set.table_id().len());
                    tracing::info!("{dataset} has {} Year values.", set.year().len());
                }
                Dataset::Ita => {
                    let set = Ita::try_from(&path)?;
                    tracing::info!("{dataset} values read.");
                    tracing::info!(
                        "{dataset} has {} AreaOrCounty values.",
                        set.area_or_country().len()
                    );
                    tracing::info!("{dataset} has {} Frequency values.", set.frequency().len());
                    tracing::info!("{dataset} has {} Indicator values.", set.indicator().len());
                    tracing::info!("{dataset} has {} Year values.", set.year().len());
                }
                Dataset::IntlServSTA => {
                    let set = IntlServSta::try_from(&path)?;
                    tracing::info!("{dataset} values read.");
                    tracing::info!(
                        "{dataset} has {} AreaOrCounty values.",
                        set.area_or_country().len()
                    );
                    tracing::info!("{dataset} has {} Channel values.", set.channel().len());
                    tracing::info!(
                        "{dataset} has {} Destination values.",
                        set.destination().len()
                    );
                    tracing::info!("{dataset} has {} Industry values.", set.industry().len());
                    tracing::info!("{dataset} has {} Year values.", set.year().len());
                }
                Dataset::IntlServTrade => {
                    let set = IntlServTrade::try_from(&path)?;
                    tracing::info!("{dataset} values read.");
                    tracing::info!(
                        "{dataset} has {} Affiliation values.",
                        set.affiliation().len()
                    );
                    tracing::info!(
                        "{dataset} has {} AreaOrCounty values.",
                        set.area_or_country().len()
                    );
                    tracing::info!(
                        "{dataset} has {} TradeDirection values.",
                        set.trade_direction().len()
                    );
                    tracing::info!(
                        "{dataset} has {} TypeOfService values.",
                        set.type_of_service().len()
                    );
                    tracing::info!("{dataset} has {} Year values.", set.year().len());
                }
                Dataset::Regional => {
                    let set = Regional::try_from(&path)?;
                    tracing::info!("{dataset} values read.");
                    tracing::info!("{dataset} has {} GeoFips values.", set.geo_fips().len());
                    tracing::info!("{dataset} has {} LineCode values.", set.line_code().len());
                    tracing::info!("{dataset} has {} TableName values.", set.table_name().len());
                    tracing::info!("{dataset} has {} Year values.", set.year().len());
                }
                other => {
                    tracing::info!("{other} not implemented.");
                }
            }
        }
    }
    Ok(())
}
