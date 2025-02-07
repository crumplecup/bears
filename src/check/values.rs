use crate::{
    trace_init, ApiMetadata, BeaErr, BeaResponse, Dataset, EnvError, FixedAssets, GdpByIndustry,
    Iip, InputOutput, IntlServSta, IntlServTrade, IoError, Ita, Mne, NiUnderlyingDetail, Nipa,
    Regional, UnderlyingGdpByIndustry,
};
use strum::IntoEnumIterator;

/// Calls a known bad combination of parameters to generate an API Error as a response.
/// Writes the JSON representation of the error to the BEA_DATA directory.
#[tracing::instrument]
pub fn api_error() -> Result<(), BeaErr> {
    trace_init()?;
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

#[tracing::instrument]
pub fn requests_exceeded() -> Result<(), BeaErr> {
    trace_init()?;
    dotenvy::dotenv().ok();
    let bea_data = EnvError::from_env("BEA_DATA")?;
    let path = std::path::PathBuf::from(&format!("{bea_data}/requests_exceeded.json"));
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
    Dataset::values().await
}

/// For each variant of [`Dataset`], request the valid range of values for each parameter name.
/// The `subset` variant of this method only requests data for datasets where the BEA has
/// implemented a response for each parameter name associated with the dataset.
#[tracing::instrument]
pub async fn values_filtered_subset() -> Result<(), BeaErr> {
    Dataset::values_subset().await
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
    Dataset::values_gdp().await
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
    Dataset::values_ugdp().await
}

#[tracing::instrument]
pub fn value_sets() -> Result<(), BeaErr> {
    trace_init()?;
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
