use bears_ecology::{bea_data, trace_init};
use bears_species::{BeaErr, BeaResponse, Investment, IoError, KeyMissing, SerdeJson};
use std::str::FromStr;

/// Checks that each "TypeOfInvestment" parameter value matches an enum variant in
/// [`Investment`].
/// Returns an error if a value does not match a known variant.
///
/// Does not test that all variants in `Component` are in active use.
#[tracing::instrument]
pub fn check_investments() -> Result<(), BeaErr> {
    trace_init()?;
    dotenvy::dotenv().ok();
    // Load Indicator parameter values into the `BeaResponse` type.
    let path = bea_data()?;
    let path = path.join("parameter_values");
    let path = path.join("IIP_TypeOfInvestment_values.json");
    let file =
        std::fs::File::open(&path).map_err(|e| IoError::new(path, e, line!(), file!().into()))?;
    let rdr = std::io::BufReader::new(file);
    let json: serde_json::Value = serde_json::from_reader(rdr)
        .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
    let data = BeaResponse::try_from(&json)?;
    tracing::trace!("Response: {data:#?}");

    // extract `ParameterValues` variant from the `BeaResponse` `Result` type.
    if let Some(values) = data.parameter_values() {
        // for each element, is the name in the set of known values?
        for value in values.iter() {
            if let Some(fields) = value.parameter_fields() {
                tracing::trace!("Fields: {fields:?}");
                if let Ok(kind) = Investment::from_str(fields.key()) {
                    if kind.description() != fields.desc() {
                        // report the error
                        tracing::warn!(
                            "Description does not match: {} not equal to {}",
                            kind.description(),
                            fields.desc()
                        );
                        let error = KeyMissing::new(kind.to_string(), line!(), file!().to_string());
                        return Err(error.into());
                    }
                } else {
                    // report the error
                    tracing::warn!("{} not a variant of Investment.", fields.key());
                    let error =
                        KeyMissing::new(fields.key().to_string(), line!(), file!().to_string());
                    return Err(error.into());
                }
            }
        }
    }
    tracing::info!("All elements of the TypeOfInvestment parameter are variants of Investment.");
    Ok(())
}
