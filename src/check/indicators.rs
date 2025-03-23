use crate::{bea_data, trace_init, BeaErr, BeaResponse, Indicator, IoError, KeyMissing, SerdeJson};
use strum::IntoEnumIterator;

/// Checks that each "Indicator" parameter value matches an enum variant in
/// [`Indicator`].
/// Returns an error if a value does not match a known variant.
///
/// Does not test that all variants in `Indicator` are in active use.
#[tracing::instrument]
pub fn check_indicators() -> Result<(), BeaErr> {
    trace_init()?;
    dotenvy::dotenv().ok();
    // Load Indicator parameter values into the `BeaResponse` type.
    let path = bea_data()?;
    let path = path.join("parameter_values");
    let path = path.join("ITA_Indicator_values.json");
    let file =
        std::fs::File::open(&path).map_err(|e| IoError::new(path, e, line!(), file!().into()))?;
    let rdr = std::io::BufReader::new(file);
    let json: serde_json::Value = serde_json::from_reader(rdr)
        .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
    let data = BeaResponse::try_from(&json)?;
    tracing::trace!("Response: {data:#?}");
    // Create vector of Indicator variants.
    let sets: Vec<String> = Indicator::iter()
        .map(|d| d.to_string().to_lowercase())
        .collect();
    tracing::info!("Sets: {:#?}", sets);

    // extract `ParameterValues` variant from the `BeaResponse` `Result` type.
    if let Some(values) = data.parameter_values() {
        // for each element, is the name in the set of known values?
        for value in values.iter() {
            if let Some(fields) = value.parameter_fields() {
                tracing::trace!("Fields: {fields:?}");
                // converting both to lowercase avoids the need for special-casing around
                // heterogeneity in capitalization
                let name = fields.key().to_lowercase();
                // if not a known name
                if !sets.contains(&name) {
                    // report the error
                    tracing::warn!("{name} not a variant of Indicator.");
                    let error = KeyMissing::new(name, line!(), file!().to_string());
                    return Err(error.into());
                } else {
                    tracing::trace!("{name} detected.");
                }
            }
        }
    }
    // all datasets accounted for
    Ok(())
}
