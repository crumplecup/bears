use bears_ecology::{bea_data, trace_init};
use bears_species::{BeaErr, BeaResponse, Component, IoError, KeyMissing, SerdeJson};
use strum::IntoEnumIterator;

/// Checks that each "Component" parameter value matches an enum variant in
/// [`Component`].
/// Returns an error if a value does not match a known variant.
///
/// Does not test that all variants in `Component` are in active use.
#[tracing::instrument]
pub fn check_components() -> Result<(), BeaErr> {
    trace_init()?;
    dotenvy::dotenv().ok();
    // Load Indicator parameter values into the `BeaResponse` type.
    let path = bea_data()?;
    let path = path.join("parameter_values");
    let path = path.join("IIP_Component_values.json");
    let file =
        std::fs::File::open(&path).map_err(|e| IoError::new(path, e, line!(), file!().into()))?;
    let rdr = std::io::BufReader::new(file);
    let json: serde_json::Value = serde_json::from_reader(rdr)
        .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
    let data = BeaResponse::try_from(&json)?;
    tracing::trace!("Response: {data:#?}");
    // Create vector of variants.
    let sets: Vec<String> = Component::iter()
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
                    tracing::warn!("{name} not a variant of Component.");
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
