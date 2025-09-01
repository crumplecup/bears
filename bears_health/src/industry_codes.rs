use bears_ecology::{bea_data, initial_load, trace_init};
use bears_species::{BeaErr, Data, Dataset, IoError, SerdeJson};

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
    let industry_path = path.join("GDPbyIndustry_IndustryCode.json");

    let industry_file = std::fs::File::create(industry_path.clone())
        .map_err(|e| IoError::new(industry_path.clone(), e, line!(), file!().to_string()))?;
    let industry_writer = std::io::BufWriter::new(industry_file);

    serde_json::to_writer_pretty(industry_writer, &industry_codes)
        .map_err(|e| SerdeJson::new(e, line!(), file!().into()))?;
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
    let industry_path = path.join("UnderlyingGDPbyIndustry_IndustryCode.json");

    let industry_file = std::fs::File::create(industry_path.clone())
        .map_err(|e| IoError::new(industry_path.clone(), e, line!(), file!().to_string()))?;
    let industry_writer = std::io::BufWriter::new(industry_file);

    serde_json::to_writer_pretty(industry_writer, &industry_codes)
        .map_err(|e| SerdeJson::new(e, line!(), file!().into()))?;
    Ok(())
}
