use bears_ecology::{bea_data, trace_init};
use bears_species::{AocSta, BeaErr, BeaResponse, IoError, KeyMissing, SerdeJson};
use strum::IntoEnumIterator;

/// Checks that each "AreaOrCountry" parameter value matches an enum variant in
/// [`AocSta`].
/// Returns an error if a value does not match a known variant.
///
/// Does not test that all variants in `AocSta` are in active use.
#[tracing::instrument]
pub fn check_aoc_sta() -> Result<(), BeaErr> {
    trace_init()?;
    dotenvy::dotenv().ok();
    // Load Indicator parameter values into the `BeaResponse` type.
    let path = bea_data()?;
    let path = path.join("parameter_values");
    let path = path.join("IntlServSTA_AreaOrCountry_values.json");
    let file =
        std::fs::File::open(&path).map_err(|e| IoError::new(path, e, line!(), file!().into()))?;
    let rdr = std::io::BufReader::new(file);
    let json: serde_json::Value = serde_json::from_reader(rdr)
        .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
    let data = BeaResponse::try_from(&json)?;
    // tracing::trace!("Response: {data:#?}");
    // Create vector of variants.
    let sets: Vec<String> = AocSta::iter()
        .map(|d| d.to_string().to_lowercase())
        .collect();
    tracing::info!("Sets: {:#?}", sets);

    // extract `ParameterValues` variant from the `BeaResponse` `Result` type.
    if let Some(values) = data.parameter_values() {
        // for each element, is the name in the set of known values?
        for value in values.iter() {
            if let Some(fields) = value.parameter_fields() {
                tracing::trace!("Fields: {fields:?}");
                match AocSta::from_key(fields.key()) {
                    Ok(_) => {
                        tracing::trace!("{} detected.", fields.key());
                    }
                    Err(_) => {
                        // report the error
                        tracing::warn!("{} not a variant of AocSta.", fields.key());
                        let error =
                            KeyMissing::new(fields.key().to_owned(), line!(), file!().to_string());
                        return Err(error.into());
                    }
                }
            }
        }
    }
    // all datasets accounted for
    Ok(())
}
