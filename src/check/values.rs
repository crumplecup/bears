use crate::{
    trace_init, ApiMetadata, BeaErr, BeaResponse, Dataset, EnvError, GdpByIndustry, Iip,
    InputOutput, IntlServSta, IntlServTrade, IoError, Ita, ParameterName, Regional, ReqwestError,
    Results,
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
pub async fn values_gdp_filtered() -> Result<(), BeaErr> {
    let req = super::Request::ParameterValueFilter;
    let mut app = req.init()?;
    let datasets = vec![Dataset::GDPbyIndustry, Dataset::UnderlyingGDPbyIndustry];
    for dataset in &datasets {
        let names = dataset.names();
        for name in names {
            if *dataset == Dataset::GDPbyIndustry && name == ParameterName::Frequency {
                let mut options = app.options().clone();
                options.with_dataset(*dataset);
                options.with_target(name);
                match name {
                    ParameterName::Industry => {
                        dotenvy::dotenv().ok();
                        let bea_data = EnvError::from_env("BEA_DATA")?;
                        let path = std::path::PathBuf::from(&bea_data);
                        let path = path.join(format!("values_{dataset}_{name}"));
                        if std::fs::exists(&path)? {
                            tracing::info!("Target directory for {name} in {dataset} found.");
                        } else {
                            std::fs::DirBuilder::new().create(&path)?;
                            tracing::info!("Target directory for {name} in {dataset} created.");
                        }
                        let table_id = GdpByIndustry::read_table_id(&bea_data)?;
                        // let first = table_id[0].clone();
                        for id in table_id {
                            // if id == first {
                            options.with_table_id(*id.value());
                            app.with_options(options.clone());
                            let data = app.get().await?;
                            tracing::info!("{data:#?}");
                            match data.json::<serde_json::Value>().await {
                                Ok(json) => {
                                    let contents = serde_json::to_vec(&json)?;
                                    // let path = std::path::PathBuf::from(format!("{bea_data}/values_{dataset}_{name}/values_{dataset}_{name}_byTableId_{}.json", id.value()));
                                    let path = path.join(format!(
                                        "values_{dataset}_{name}_byTableId_{}.json",
                                        id.value()
                                    ));
                                    tracing::info!("Current target path: {path:?}");
                                    IoError::write(&contents, path)?;
                                }
                                Err(source) => {
                                    let url = app.url().to_string();
                                    let method = "get".to_string();
                                    let body =
                                        app.params().into_iter().collect::<Vec<(String, String)>>();
                                    let mut error = ReqwestError::new(url, method, source);
                                    error.with_body(body);
                                    return Err(error.into());
                                }
                            }
                            // }
                        }
                    }
                    ParameterName::Frequency => {
                        dotenvy::dotenv().ok();
                        let bea_data = EnvError::from_env("BEA_DATA")?;
                        let path = std::path::PathBuf::from(&bea_data);
                        let path = path.join(format!("values_{dataset}_{name}"));
                        // if std::fs::exists(&path)? {
                        //     tracing::info!("Target directory for {name} in {dataset} found.");
                        // } else {
                        //     std::fs::DirBuilder::new().create(&path)?;
                        //     tracing::info!("Target directory for {name} in {dataset} created.");
                        // }
                        let industry = GdpByIndustry::read_industry(&bea_data)?;
                        let table_id = GdpByIndustry::read_table_id(&bea_data)?;
                        let year = GdpByIndustry::read_year(&bea_data)?;
                        let first = table_id[0].clone();
                        // values vary by table id
                        for id in table_id {
                            // industry values associated with table id
                            if let Some(sectors) = industry.get(&id) {
                                let primo_sector = sectors[0].clone();
                                // year values associated with table id
                                if let Some(years) = year.get(&id) {
                                    let primo_year = years[0].clone();
                                    for sector in sectors {
                                        for yr in years {
                                            if id == first
                                                && *sector == primo_sector
                                                && *yr == primo_year
                                            {
                                                options.with_table_id(*id.value());
                                                options.with_industry(sector.key());
                                                options.with_year(yr.date().year().to_string());
                                                app.with_options(options.clone());
                                                let data = app.get().await?;
                                                tracing::info!("{data:#?}");
                                                match data.json::<serde_json::Value>().await {
                                                    Ok(json) => {
                                                        tracing::info!("{json:#?}");
                                                    }
                                                    Err(source) => {
                                                        tracing::warn!("{source}");
                                                        use std::error::Error;
                                                        if let Some(src) = source.source() {
                                                            tracing::warn!("Caused by: {src}.");
                                                        }
                                                        // let url = app.url().to_string();
                                                        // let method = "get".to_string();
                                                        // let body = app
                                                        //     .params()
                                                        //     .into_iter()
                                                        //     .collect::<Vec<(String, String)>>();
                                                        // let mut error =
                                                        //     ReqwestError::new(url, method, source);
                                                        // error.with_body(body);
                                                        // return Err(error.into());
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ParameterName::Year => {
                        dotenvy::dotenv().ok();
                        let bea_data = EnvError::from_env("BEA_DATA")?;
                        let path = std::path::PathBuf::from(&bea_data);
                        let path = path.join(format!("values_{dataset}_{name}"));
                        if std::fs::exists(&path)? {
                            tracing::info!("Target directory for {name} in {dataset} found.");
                        } else {
                            std::fs::DirBuilder::new().create(&path)?;
                            tracing::info!("Target directory for {name} in {dataset} created.");
                        }
                        let table_id = GdpByIndustry::read_table_id(&bea_data)?;
                        // let first = table_id[0].clone();
                        for id in table_id {
                            // if id == first {
                            options.with_table_id(*id.value());
                            app.with_options(options.clone());
                            let data = app.get().await?;
                            tracing::info!("{data:#?}");
                            match data.json::<serde_json::Value>().await {
                                Ok(json) => {
                                    // tracing::info!("{json:#?}");
                                    let contents = serde_json::to_vec(&json)?;
                                    // let path = std::path::PathBuf::from(format!("{bea_data}/values_{dataset}_{name}/values_{dataset}_{name}_byTableId_{}.json", id.value()));
                                    let path = path.join(format!(
                                        "values_{dataset}_{name}_byTableId_{}.json",
                                        id.value()
                                    ));
                                    tracing::info!("Current target path: {path:?}");
                                    IoError::write(&contents, path)?;
                                }
                                Err(source) => {
                                    let url = app.url().to_string();
                                    let method = "get".to_string();
                                    let body =
                                        app.params().into_iter().collect::<Vec<(String, String)>>();
                                    let mut error = ReqwestError::new(url, method, source);
                                    error.with_body(body);
                                    return Err(error.into());
                                }
                            }
                            // }
                        }
                    }
                    _ => {
                        tracing::info!("Unimplemented.");
                    }
                };
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
        if *dataset == Dataset::GDPbyIndustry {
            match dataset {
                Dataset::APIDatasetMetadata => {
                    let set = ApiMetadata::try_from(&path)?;
                    tracing::info!("{dataset} values read.");
                    tracing::info!("{dataset} has {} Metadata values.", set.len());
                }
                Dataset::GDPbyIndustry => {
                    let ids = GdpByIndustry::read_table_id(&path)?;
                    if !ids.is_empty() {}
                    // let set =
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
