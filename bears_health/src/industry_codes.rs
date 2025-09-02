use bears_ecology::{bea_data, initial_load, trace_init};
use bears_species::{BeaErr, Data, Dataset, Naics};

use crate::write_json;

/// Attempts to load all files in the download [`History`], without respect to the load `History`.
/// Loads GDPbyIndustry files, converts them to industry codes.
/// Serializes the results to `GDPbyIndustry_IndustryCode.json` in the
/// `BEA_DATA` directory.
#[tracing::instrument(skip_all)]
pub async fn gdp_codes() -> Result<(), BeaErr> {
    trace_init()?;
    let dataset = Dataset::GDPbyIndustry;
    let data = initial_load(dataset, None).await?;
    tracing::info!("{} datasets loaded.", data.len());
    let mut industry_codes = std::collections::BTreeMap::new();
    data.iter()
        .map(|v| {
            if let Data::Gdp(data) = v {
                industry_codes.append(&mut data.industry_codes());
            }
        })
        .for_each(drop);

    let path = bea_data()?;
    let path = path.join("GDPbyIndustry_IndustryCode.json");
    write_json(&industry_codes, path)?;
    Ok(())
}

/// Attempts to load all files in the download [`History`], without respect to the load `History`.
/// Loads GDPbyIndustry files, converts them to industry codes.
/// Checks that all industry codes are present as variants of the Naics::InputOutput enum.
#[tracing::instrument(skip_all)]
pub async fn check_gdp_codes() -> Result<(), BeaErr> {
    trace_init()?;
    let dataset = Dataset::GDPbyIndustry;
    let data = initial_load(dataset, None).await?;
    tracing::info!("{} datasets loaded.", data.len());
    let mut industry_codes = std::collections::BTreeMap::new();
    let mut errors = std::collections::BTreeMap::new();
    data.iter()
        .map(|v| {
            if let Data::Gdp(data) = v {
                industry_codes.append(&mut data.industry_codes());
            }
        })
        .for_each(drop);
    industry_codes
        .iter()
        .map(|(k, v)| {
            if Naics::from_code(k).is_none() {
                errors.insert(k.clone(), v.clone());
            }
        })
        .for_each(drop);

    if !errors.is_empty() {
        tracing::error!("Error file printed to BEA_DATA directory.");
        let path = bea_data()?;
        let path = path.join("GDPbyIndustry_IndustryCode_Missing.json");
        write_json(&errors, path)?;
    } else {
        tracing::info!("All parameters map to variants of the Naics enum.");
    }
    Ok(())
}

/// Attempts to load all files in the download [`History`], without respect to the load `History`.
/// Loads UnderlyingGDPbyIndustry files, converts them to industry codes.
/// Serializes the results to `UnderlyingGDPbyIndustry_IndustryCode.json` in the
/// `BEA_DATA` directory.
#[tracing::instrument(skip_all)]
pub async fn ugdp_codes() -> Result<(), BeaErr> {
    trace_init()?;
    let dataset = Dataset::UnderlyingGDPbyIndustry;
    let data = initial_load(dataset, None).await?;
    tracing::info!("{} datasets loaded.", data.len());
    let mut industry_codes = std::collections::BTreeMap::new();
    data.iter()
        .map(|v| {
            if let Data::UnderlyingGdp(data) = v {
                industry_codes.append(&mut data.industry_codes());
            }
        })
        .for_each(drop);

    let path = bea_data()?;
    let path = path.join("UnderlyingGDPbyIndustry_IndustryCode.json");
    write_json(&industry_codes, path)?;
    Ok(())
}

/// Attempts to load all files in the download [`History`], without respect to the load `History`.
/// Loads UnderlyingGDPbyIndustry files, converts them to industry codes.
/// Checks that all industry codes are present as variants of the Naics::InputOutput enum.
#[tracing::instrument(skip_all)]
pub async fn check_ugdp_codes() -> Result<(), BeaErr> {
    trace_init()?;
    let dataset = Dataset::UnderlyingGDPbyIndustry;
    let data = initial_load(dataset, None).await?;
    tracing::info!("{} datasets loaded.", data.len());
    let mut industry_codes = std::collections::BTreeMap::new();
    let mut errors = std::collections::BTreeMap::new();
    data.iter()
        .map(|v| {
            if let Data::UnderlyingGdp(data) = v {
                industry_codes.append(&mut data.industry_codes());
            }
        })
        .for_each(drop);
    industry_codes
        .iter()
        .map(|(k, v)| {
            if Naics::from_code(k).is_none() {
                errors.insert(k.clone(), v.clone());
            }
        })
        .for_each(drop);

    if !errors.is_empty() {
        tracing::error!("Error file printed to BEA_DATA directory.");
        let path = bea_data()?;
        let path = path.join("UnderlyingGDPbyIndustry_IndustryCode_Missing.json");
        write_json(&errors, path)?;
    } else {
        tracing::info!("All parameters map to variants of the Naics enum.");
    }
    Ok(())
}
