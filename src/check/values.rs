use crate::{
    trace_init, ApiMetadata, BeaErr, BeaResponse, Dataset, EnvError, FixedAssets, GdpByIndustry,
    Iip, InputOutput, IntlServSta, IntlServTrade, IoError, Ita, Mne, NiUnderlyingDetail, Nipa,
    ParameterName, Regional, ReqwestError, Results, UnderlyingGdpByIndustry,
};
use strum::IntoEnumIterator;

/// Calls a known bad combination of parameters to generate an API Error as a response.
/// Writes the JSON representation of the error to the BEA_DATA directory.
#[tracing::instrument]
pub fn api_error() -> Result<(), BeaErr> {
    trace_init();
    dotenvy::dotenv().ok();
    let bea_data = EnvError::from_env("BEA_DATA")?;
    let path = std::path::PathBuf::from(&format!("{bea_data}/values_api_error.json"));
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
    let req = super::Request::ParameterValueFilter;
    let mut app = req.init()?;
    let datasets: Vec<Dataset> = Dataset::iter().collect();
    for dataset in &datasets {
        let names = dataset.names();
        for name in names {
            let mut options = app.options().clone();
            options.with_dataset(*dataset);
            options.with_target(name);
            app.add_options(options.clone());
            let data = app.get().await?;
            tracing::info!("{data:#?}");
            match data.json::<serde_json::Value>().await {
                Ok(json) => {
                    tracing::info!("{json:#?}");
                    let bea = BeaResponse::try_from(&json)?;
                    match bea.results() {
                        Results::ApiError(e) => {
                            // TODO: Map api error codes to an enum.
                            tracing::info!("{e}");
                        }
                        Results::ParameterValues(_) => {
                            let contents = serde_json::to_vec(&json)?;
                            dotenvy::dotenv().ok();
                            let bea_data = EnvError::from_env("BEA_DATA")?;
                            let path =
                                std::path::PathBuf::from(&format!("{bea_data}/parameter_values"));
                            if !std::fs::exists(&path)? {
                                std::fs::DirBuilder::new().create(&path)?;
                                tracing::info!("Target directory for Parameter Values created.");
                            }
                            let path = path.join(format!("{dataset}_{name}_values.json"));
                            match std::fs::write(&path, contents) {
                                Ok(()) => {}
                                Err(source) => {
                                    let error =
                                        IoError::new(path, source, line!(), file!().to_string());
                                    return Err(error.into());
                                }
                            }
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
                    let mut error =
                        ReqwestError::new(url, method, source, line!(), file!().to_string());
                    error.with_body(body);
                    return Err(error.into());
                }
            }
        }
    }
    Ok(())
}

/// For each variant of [`Dataset`], request the valid range of values for each parameter name.
/// The `subset` variant of this method only requests data for datasets where the BEA has
/// implemented a response for each parameter name associated with the dataset.
#[tracing::instrument]
pub async fn values_filtered_subset() -> Result<(), BeaErr> {
    let req = super::Request::ParameterValueFilter;
    let mut app = req.init()?;
    let datasets = vec![
        Dataset::APIDatasetMetadata,
        Dataset::Iip,
        Dataset::Ita,
        Dataset::InputOutput,
        Dataset::IntlServSTA,
        Dataset::IntlServTrade,
        Dataset::Regional,
    ];
    for dataset in &datasets {
        let names = dataset.names();
        for name in names {
            let mut options = app.options().clone();
            options.with_dataset(*dataset);
            options.with_target(name);
            app.add_options(options.clone());
            let data = app.get().await?;
            tracing::info!("{data:#?}");
            match data.json::<serde_json::Value>().await {
                Ok(json) => {
                    tracing::info!("{json:#?}");
                    let bea = BeaResponse::try_from(&json)?;
                    match bea.results() {
                        Results::ApiError(e) => {
                            // In the subset we do not expect an error.
                            tracing::warn!("{e}");
                        }
                        Results::ParameterValues(_) => {
                            let contents = serde_json::to_vec(&json)?;
                            dotenvy::dotenv().ok();
                            let bea_data = EnvError::from_env("BEA_DATA")?;
                            let path = std::path::PathBuf::from(&format!(
                                "{bea_data}/parameter_values/{dataset}_{name}_values.json"
                            ));
                            match std::fs::write(&path, contents) {
                                Ok(()) => {}
                                Err(source) => {
                                    let error =
                                        IoError::new(path, source, line!(), file!().to_string());
                                    return Err(error.into());
                                }
                            }
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
                    let mut error =
                        ReqwestError::new(url, method, source, line!(), file!().to_string());
                    error.with_body(body);
                    return Err(error.into());
                }
            }
        }
    }
    Ok(())
}

/// Two parameters in the GdpByIndustry dataset have valid input sets that vary by table_id, namely
/// Year and Industry.  Obtain table ids using [`Method::GetParameterValues`] prior to running this
/// check. For these two parameters, we obtain a response for each table_id and write the result to
/// a folder in the BEA_DATA directory.
///
/// Due to the nested call to [`GdpByIndustry::read_table_id`], we have seperate checks for GDP and
/// Underlying GDP.  Less dry but somewhat clearer to write and read.
#[tracing::instrument]
pub async fn values_gdp_filtered() -> Result<(), BeaErr> {
    let req = super::Request::ParameterValueFilter;
    let mut app = req.init()?;
    let dataset = Dataset::GDPbyIndustry;
    let names = dataset.names();
    for name in names {
        let mut options = app.options().clone();
        options.with_dataset(dataset);
        options.with_target(name);
        match name {
            ParameterName::Industry => {
                dotenvy::dotenv().ok();
                let bea_data = EnvError::from_env("BEA_DATA")?;
                let path = std::path::PathBuf::from(&format!("{bea_data}/parameter_values"));
                if !std::fs::exists(&path)? {
                    std::fs::DirBuilder::new().create(&path)?;
                    tracing::info!("Target directory for Parameter Values created.");
                }
                let path = path.join(format!("{dataset}_{name}"));
                if !std::fs::exists(&path)? {
                    std::fs::DirBuilder::new().create(&path)?;
                    tracing::info!("Target directory for Parameter Values created.");
                }
                let table_id = GdpByIndustry::read_table_id(&bea_data)?;
                for id in table_id {
                    options.with_table_id(*id.value());
                    app.add_options(options.clone());
                    let data = app.get().await?;
                    tracing::info!("{data:#?}");
                    match data.json::<serde_json::Value>().await {
                        Ok(json) => {
                            let contents = serde_json::to_vec(&json)?;
                            let path = path.join(format!(
                                "{dataset}_{name}_byTableId_{}_values.json",
                                id.value()
                            ));
                            tracing::info!("Current target path: {path:?}");
                            match std::fs::write(&path, contents) {
                                Ok(()) => {}
                                Err(source) => {
                                    let error =
                                        IoError::new(path, source, line!(), file!().to_string());
                                    return Err(error.into());
                                }
                            }
                        }
                        Err(source) => {
                            let url = app.url().to_string();
                            let method = "get".to_string();
                            let body = app.params().into_iter().collect::<Vec<(String, String)>>();
                            let mut error = ReqwestError::new(
                                url,
                                method,
                                source,
                                line!(),
                                file!().to_string(),
                            );
                            error.with_body(body);
                            return Err(error.into());
                        }
                    }
                }
            }
            ParameterName::Year => {
                dotenvy::dotenv().ok();
                let bea_data = EnvError::from_env("BEA_DATA")?;
                let path = std::path::PathBuf::from(&format!("{bea_data}/parameter_values"));
                if !std::fs::exists(&path)? {
                    std::fs::DirBuilder::new().create(&path)?;
                    tracing::info!("Target directory for Parameter Values created.");
                }
                let path = path.join(format!("{dataset}_{name}"));
                if !std::fs::exists(&path)? {
                    std::fs::DirBuilder::new().create(&path)?;
                    tracing::info!("Target directory for Parameter Values created.");
                }
                let table_id = GdpByIndustry::read_table_id(&bea_data)?;
                for id in table_id {
                    options.with_table_id(*id.value());
                    app.add_options(options.clone());
                    let data = app.get().await?;
                    tracing::info!("{data:#?}");
                    match data.json::<serde_json::Value>().await {
                        Ok(json) => {
                            // tracing::info!("{json:#?}");
                            let contents = serde_json::to_vec(&json)?;
                            let path = path.join(format!(
                                "{dataset}_{name}_byTableId_{}_values.json",
                                id.value()
                            ));
                            tracing::info!("Current target path: {path:?}");
                            match std::fs::write(&path, contents) {
                                Ok(()) => {}
                                Err(source) => {
                                    let error =
                                        IoError::new(path, source, line!(), file!().to_string());
                                    return Err(error.into());
                                }
                            }
                        }
                        Err(source) => {
                            let url = app.url().to_string();
                            let method = "get".to_string();
                            let body = app.params().into_iter().collect::<Vec<(String, String)>>();
                            let mut error = ReqwestError::new(
                                url,
                                method,
                                source,
                                line!(),
                                file!().to_string(),
                            );
                            error.with_body(body);
                            return Err(error.into());
                        }
                    }
                }
            }
            _ => {
                // read table_id using Method::GetParameterValues
                // frequency listed in BEA USER GUIDE pg. 37
                tracing::info!("Unimplemented.");
            }
        };
    }
    Ok(())
}

/// Two parameters in the UnderlyingGdpByIndustry dataset have valid input sets that vary by table_id, namely
/// Year and Industry.  Obtain table ids using [`Method::GetParameterValues`] prior to running this
/// check. For these two parameters, we obtain a response for each table_id and write the result to
/// a folder in the BEA_DATA directory.
///
/// Due to the nested call to [`UnderlyingGdpByIndustry::read_table_id`], we have seperate checks for GDP and
/// Underlying GDP.  Less dry but somewhat clearer to write and read.
#[tracing::instrument]
pub async fn values_ugdp_filtered() -> Result<(), BeaErr> {
    let req = super::Request::ParameterValueFilter;
    let mut app = req.init()?;
    let dataset = Dataset::UnderlyingGDPbyIndustry;
    let names = dataset.names();
    for name in names {
        let mut options = app.options().clone();
        options.with_dataset(dataset);
        options.with_target(name);
        match name {
            ParameterName::Industry => {
                dotenvy::dotenv().ok();
                let bea_data = EnvError::from_env("BEA_DATA")?;
                let path = std::path::PathBuf::from(&format!("{bea_data}/parameter_values"));
                if !std::fs::exists(&path)? {
                    std::fs::DirBuilder::new().create(&path)?;
                    tracing::info!("Target directory for Parameter Values created.");
                }
                let path = path.join(format!("{dataset}_{name}"));
                if !std::fs::exists(&path)? {
                    std::fs::DirBuilder::new().create(&path)?;
                    tracing::info!(
                        "Target directory for Parameter Values created for {name} in {dataset}."
                    );
                }
                let table_id = UnderlyingGdpByIndustry::read_table_id(&bea_data)?;
                for id in table_id {
                    options.with_table_id(*id.value());
                    app.add_options(options.clone());
                    let data = app.get().await?;
                    tracing::info!("{data:#?}");
                    match data.json::<serde_json::Value>().await {
                        Ok(json) => {
                            let contents = serde_json::to_vec(&json)?;
                            let path = path.join(format!(
                                "{dataset}_{name}_byTableId_{}_values.json",
                                id.value()
                            ));
                            tracing::info!("Current target path: {path:?}");
                            match std::fs::write(&path, contents) {
                                Ok(()) => {}
                                Err(source) => {
                                    let error =
                                        IoError::new(path, source, line!(), file!().to_string());
                                    return Err(error.into());
                                }
                            }
                        }
                        Err(source) => {
                            let url = app.url().to_string();
                            let method = "get".to_string();
                            let body = app.params().into_iter().collect::<Vec<(String, String)>>();
                            let mut error = ReqwestError::new(
                                url,
                                method,
                                source,
                                line!(),
                                file!().to_string(),
                            );
                            error.with_body(body);
                            return Err(error.into());
                        }
                    }
                }
            }
            ParameterName::Year => {
                dotenvy::dotenv().ok();
                let bea_data = EnvError::from_env("BEA_DATA")?;
                let path = std::path::PathBuf::from(&format!("{bea_data}/parameter_values"));
                if !std::fs::exists(&path)? {
                    std::fs::DirBuilder::new().create(&path)?;
                    tracing::info!("Target directory for Parameter Values created.");
                }
                let path = path.join(format!("{dataset}_{name}"));
                if !std::fs::exists(&path)? {
                    std::fs::DirBuilder::new().create(&path)?;
                    tracing::info!(
                        "Target directory for Parameter Values created for {name} in {dataset}."
                    );
                }
                let table_id = UnderlyingGdpByIndustry::read_table_id(&bea_data)?;
                for id in table_id {
                    options.with_table_id(*id.value());
                    app.add_options(options.clone());
                    let data = app.get().await?;
                    tracing::info!("{data:#?}");
                    match data.json::<serde_json::Value>().await {
                        Ok(json) => {
                            // tracing::info!("{json:#?}");
                            let contents = serde_json::to_vec(&json)?;
                            let path = path.join(format!(
                                "{dataset}_{name}_byTableId_{}_values.json",
                                id.value()
                            ));
                            tracing::info!("Current target path: {path:?}");
                            match std::fs::write(&path, contents) {
                                Ok(()) => {}
                                Err(source) => {
                                    let error =
                                        IoError::new(path, source, line!(), file!().to_string());
                                    return Err(error.into());
                                }
                            }
                        }
                        Err(source) => {
                            let url = app.url().to_string();
                            let method = "get".to_string();
                            let body = app.params().into_iter().collect::<Vec<(String, String)>>();
                            let mut error = ReqwestError::new(
                                url,
                                method,
                                source,
                                line!(),
                                file!().to_string(),
                            );
                            error.with_body(body);
                            return Err(error.into());
                        }
                    }
                }
            }
            _ => {
                // read table_id using Method::GetParameterValues
                // frequency listed in BEA USER GUIDE pg. 50
                tracing::info!("Unimplemented.");
            }
        };
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
        match dataset {
            Dataset::APIDatasetMetadata => {
                let set = ApiMetadata::try_from(&path)?;
                tracing::info!("{dataset} values read.");
                tracing::info!("{dataset} has {} Metadata values.", set.len());
            }
            Dataset::FixedAssets => {
                let set = FixedAssets::try_from(&path)?;
                tracing::info!("{dataset} values read.");
                tracing::info!("{dataset} has {} TableName values.", set.table_name().len());
                tracing::info!(
                    "{dataset} has {} tables of Year Ranges.",
                    set.year().keys().len()
                );
            }
            Dataset::GDPbyIndustry => {
                let _set = GdpByIndustry::from_file(&path)?;
                tracing::info!("{dataset} values read.");
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
            Dataset::Nipa => {
                let set = Nipa::try_from(&path)?;
                tracing::info!("{dataset} values read.");
                tracing::info!("{dataset} has {} Frequency values.", set.frequency().len());
                tracing::info!(
                    "{dataset} has {} ShowMillions values.",
                    set.show_millions().len()
                );
                tracing::info!("{dataset} has {} TableName values.", set.table_name().len());
                tracing::info!(
                    "{dataset} has {} tables of Year Ranges.",
                    set.year().keys().len()
                );
            }
            Dataset::Mne => {
                let set = Mne::try_from(&path)?;
                tracing::info!("{dataset} values read.");
                tracing::info!(
                    "{dataset} has {} Classification values.",
                    set.classification().len()
                );
                tracing::info!("{dataset} has {} Country values.", set.country().len());
                tracing::info!(
                    "{dataset} has {} DirectionOfInvestment values.",
                    set.direction_of_investment().len()
                );
                tracing::info!(
                    "{dataset} has {} GetFootnotes values.",
                    set.get_footnotes().len()
                );
                tracing::info!("{dataset} has {} Industry values.", set.industry().len());
                tracing::info!(
                    "{dataset} has {} Investment values.",
                    set.investment().len()
                );
                tracing::info!(
                    "{dataset} has {} NonbankAffiliatesOnly values.",
                    set.nonbank_affiliates_only().len()
                );
                tracing::info!(
                    "{dataset} has {} OwnershipLevel values.",
                    set.ownership_level().len()
                );
                tracing::info!(
                    "{dataset} has {} ParentInvestment values.",
                    set.parent_investment().len()
                );
                tracing::info!("{dataset} has {} YearOptions values.", set.year().len());
            }
            Dataset::NIUnderlyingDetail => {
                let set = NiUnderlyingDetail::try_from(&path)?;
                tracing::info!("{dataset} values read.");
                tracing::info!("{dataset} has {} Frequency values.", set.frequency().len());
                tracing::info!("{dataset} has {} TableName values.", set.table_name().len());
                tracing::info!(
                    "{dataset} has {} tables of Year Ranges.",
                    set.year().keys().len()
                );
            }
            Dataset::Regional => {
                let set = Regional::try_from(&path)?;
                tracing::info!("{dataset} values read.");
                tracing::info!("{dataset} has {} GeoFips values.", set.geo_fips().len());
                tracing::info!("{dataset} has {} LineCode values.", set.line_code().len());
                tracing::info!("{dataset} has {} TableName values.", set.table_name().len());
                tracing::info!("{dataset} has {} Year values.", set.year().len());
            }
            Dataset::UnderlyingGDPbyIndustry => {
                let _set = UnderlyingGdpByIndustry::from_file(&path)?;
                tracing::info!("{dataset} values read.");
            }
        }
    }
    Ok(())
}
