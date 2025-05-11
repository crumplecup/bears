use bears_ecology::trace_init;
use bears_species::{
    BeaErr, KeyMissing, NaicsCategory, NaicsIndustry, NaicsItems, NaicsSector, NaicsSubcategory,
    NaicsSubsector,
};
use std::str::FromStr;
use strum::IntoEnumIterator;

/// Checks that each Naics sector code matches a variant of
/// [`NaicsSector`].  The sector code is the leading two digits in the industry code.
/// Returns an error if a value does not match a known variant.
///
/// Does not test that all variants in `NaicsSector` are in active use.
#[tracing::instrument]
pub fn check_naics_sectors() -> Result<(), BeaErr> {
    trace_init()?;
    let path = "cave/naics_sector.csv";
    let naics = NaicsItems::from_csv(path)?;
    // Create vector of variants.
    let sets: Vec<String> = NaicsSector::iter()
        .map(|d| d.to_string())
        // .map(|d| d.to_string().to_lowercase())
        .collect();
    tracing::trace!("Sets: {:#?}", sets);

    for item in naics.iter() {
        let name = item.name();
        if !sets.contains(name) {
            tracing::warn!("{} not a variant of NaicsSector.", name);
            let error = KeyMissing::new(name.to_owned(), line!(), file!().to_string());
            return Err(error.into());
        } else {
            // Ok to unwrap because name is in sets
            let variant = NaicsSector::from_str(name).unwrap();
            // check that the description matches the record for this variant
            let desc = variant.description();
            if desc != item.title().trim() {
                tracing::warn!(
                    "{desc} not the description of {variant}, expected {}.",
                    item.title()
                );
                let error = KeyMissing::new(
                    variant.description().to_owned(),
                    line!(),
                    file!().to_string(),
                );
                return Err(error.into());
            }
            // check that the code matches the record for this variant
            let code = variant.code();
            if code != *item.code() {
                tracing::warn!(
                    "{code} not the code of {variant}, expected {}.",
                    item.code()
                );
                let error =
                    KeyMissing::new(variant.code().to_string(), line!(), file!().to_string());
                return Err(error.into());
            }
            if let Some(comp) = NaicsSector::from_code(&code.to_string()) {
                if variant != comp {
                    tracing::error!("Code {code} parses to variant {comp}, expected {variant}.");
                    let error = KeyMissing::new(variant.to_string(), line!(), file!().to_string());
                    return Err(error.into());
                }
            } else {
                tracing::error!("Code {code} fails to parse to variant {variant}.");
                let error =
                    KeyMissing::new(variant.code().to_string(), line!(), file!().to_string());
                return Err(error.into());
            }
        }
    }
    // all variants accounted for
    Ok(())
}

/// Checks that each Naics subsector code matches a variant of
/// [`NaicsSubsector`]. The subsector code is the leading three digits in the naics industry code.
/// Returns an error if a value does not match a known variant.
///
/// Does not test that all variants in `NaicsSubsector` are in active use.
#[tracing::instrument]
pub fn check_naics_subsectors() -> Result<(), BeaErr> {
    trace_init()?;
    let path = "cave/naics_subsector.csv";
    let naics = NaicsItems::from_csv(path)?;
    // Create vector of variants.
    let sets: Vec<String> = NaicsSubsector::iter()
        .map(|d| d.to_string())
        // .map(|d| d.to_string().to_lowercase())
        .collect();
    tracing::trace!("Sets: {:#?}", sets);

    for (index, item) in naics.iter().enumerate() {
        let name = item.name();
        if !sets.contains(name) {
            tracing::warn!("Row {index}: {} not a variant of NaicsSubsector.", name);
            let error = KeyMissing::new(name.to_owned(), line!(), file!().to_string());
            return Err(error.into());
        } else {
            // Ok to unwrap because name is in sets
            let variant = NaicsSubsector::from_str(name).unwrap();
            // check that the description matches the record for this variant
            let desc = variant.description();
            if desc != item.title().trim() {
                tracing::warn!(
                    "{desc} not the description of {variant}, expected {}.",
                    item.title()
                );
                let error = KeyMissing::new(
                    variant.description().to_owned(),
                    line!(),
                    file!().to_string(),
                );
                return Err(error.into());
            }
            // check that the code matches the record for this variant
            let code = variant.code();
            if code != *item.code() {
                tracing::warn!(
                    "{code} not the code of {variant}, expected {}.",
                    item.code()
                );
                let error =
                    KeyMissing::new(variant.code().to_string(), line!(), file!().to_string());
                return Err(error.into());
            }
            if let Some(comp) = NaicsSubsector::from_code(&code.to_string()) {
                if variant != comp {
                    tracing::error!("Code {code} parses to variant {comp}, expected {variant}.");
                    let error = KeyMissing::new(variant.to_string(), line!(), file!().to_string());
                    return Err(error.into());
                }
            } else {
                tracing::error!("Code {code} fails to parse to variant {variant}.");
                let error =
                    KeyMissing::new(variant.code().to_string(), line!(), file!().to_string());
                return Err(error.into());
            }
        }
    }
    // all variants accounted for
    Ok(())
}

/// Checks that each Naics category code matches a variant of
/// [`NaicsCategory`]. The category code is the leading four digits in the naics industry code.
/// Returns an error if a value does not match a known variant.
///
/// Does not test that all variants in `NaicsCategory` are in active use.
#[tracing::instrument]
pub fn check_naics_category() -> Result<(), BeaErr> {
    trace_init()?;
    let path = "cave/naics_category.csv";
    let naics = NaicsItems::from_csv(path)?;
    // Create vector of variants.
    let sets: Vec<String> = NaicsCategory::iter()
        .map(|d| d.to_string())
        // .map(|d| d.to_string().to_lowercase())
        .collect();
    tracing::trace!("Sets: {:#?}", sets);

    for (index, item) in naics.iter().enumerate() {
        let name = item.name();
        if !sets.contains(name) {
            tracing::warn!("Row {index}: {} not a variant of NaicsCategory.", name);
            let error = KeyMissing::new(name.to_owned(), line!(), file!().to_string());
            return Err(error.into());
        } else {
            // Ok to unwrap because name is in sets
            let variant = NaicsCategory::from_str(name).unwrap();
            // check that the description matches the record for this variant
            let desc = variant.description();
            if desc != item.title().trim() {
                tracing::warn!(
                    "{desc} not the description of {variant}, expected {}.",
                    item.title()
                );
                let error = KeyMissing::new(
                    variant.description().to_owned(),
                    line!(),
                    file!().to_string(),
                );
                return Err(error.into());
            }
            // check that the code matches the record for this variant
            let code = variant.code();
            if code != *item.code() {
                tracing::warn!(
                    "{code} not the code of {variant}, expected {}.",
                    item.code()
                );
                let error =
                    KeyMissing::new(variant.code().to_string(), line!(), file!().to_string());
                return Err(error.into());
            }
            if let Some(comp) = NaicsCategory::from_code(&code.to_string()) {
                if variant != comp {
                    tracing::error!("Code {code} parses to variant {comp}, expected {variant}.");
                    let error = KeyMissing::new(variant.to_string(), line!(), file!().to_string());
                    return Err(error.into());
                }
            } else {
                tracing::error!("Code {code} fails to parse to variant {variant}.");
                let error =
                    KeyMissing::new(variant.code().to_string(), line!(), file!().to_string());
                return Err(error.into());
            }
        }
    }
    // all variants accounted for
    Ok(())
}

/// Checks that each Naics subcategory code matches a variant of
/// [`NaicsSubcategory`]. The subcategory code is the leading five digits in the naics industry code.
/// Returns an error if a value does not match a known variant.
///
/// Does not test that all variants in `NaicsSubcategory` are in active use.
#[tracing::instrument]
pub fn check_naics_subcategory() -> Result<(), BeaErr> {
    trace_init()?;
    let path = "cave/naics_subcategory.csv";
    let naics = NaicsItems::from_csv(path)?;
    // Create vector of variants.
    let sets: Vec<String> = NaicsSubcategory::iter()
        .map(|d| d.to_string())
        // .map(|d| d.to_string().to_lowercase())
        .collect();
    tracing::trace!("Sets: {:#?}", sets);

    for (index, item) in naics.iter().enumerate() {
        let name = item.name();
        if !sets.contains(name) {
            tracing::warn!("Row {index}: {} not a variant of NaicsSubcategory.", name);
            let error = KeyMissing::new(name.to_owned(), line!(), file!().to_string());
            return Err(error.into());
        } else {
            // Ok to unwrap because name is in sets
            let variant = NaicsSubcategory::from_str(name).unwrap();
            // check that the description matches the record for this variant
            let desc = variant.description();
            if desc.trim() != item.title().trim() {
                tracing::warn!(
                    "{desc} not the description of {variant}, expected {}.",
                    item.title().trim()
                );
                let error = KeyMissing::new(
                    variant.description().to_owned(),
                    line!(),
                    file!().to_string(),
                );
                return Err(error.into());
            }
            // check that the code matches the record for this variant
            let code = variant.code();
            if code != *item.code() {
                tracing::warn!(
                    "{code} not the code of {variant}, expected {}.",
                    item.code()
                );
                let error =
                    KeyMissing::new(variant.code().to_string(), line!(), file!().to_string());
                return Err(error.into());
            }
            if let Some(comp) = NaicsSubcategory::from_code(&code.to_string()) {
                if variant != comp {
                    tracing::error!("Code {code} parses to variant {comp}, expected {variant}.");
                    let error = KeyMissing::new(variant.to_string(), line!(), file!().to_string());
                    return Err(error.into());
                }
            } else {
                tracing::error!("Code {code} fails to parse to variant {variant}.");
                let error =
                    KeyMissing::new(variant.code().to_string(), line!(), file!().to_string());
                return Err(error.into());
            }
        }
    }
    // all variants accounted for
    Ok(())
}

/// Checks that each Naics industry code matches a variant of
/// [`NaicsIndustry`]. The industry code is the full six-digit naics industry code.
/// Returns an error if a value does not match a known variant.
///
/// Does not test that all variants in `NaicsIndustry` are in active use.
#[tracing::instrument]
pub fn check_naics_industry() -> Result<(), BeaErr> {
    trace_init()?;
    let path = "cave/naics_industry.csv";
    let naics = NaicsItems::from_csv(path)?;
    // Create vector of variants.
    let sets: Vec<String> = NaicsIndustry::iter()
        .map(|d| d.to_string())
        // .map(|d| d.to_string().to_lowercase())
        .collect();
    tracing::trace!("Sets: {:#?}", sets);

    for (index, item) in naics.iter().enumerate() {
        let name = item.name();
        if !sets.contains(name) {
            tracing::warn!("Row {index}: {} not a variant of NaicsIndustry.", name);
            let error = KeyMissing::new(name.to_owned(), line!(), file!().to_string());
            return Err(error.into());
        } else {
            // Ok to unwrap because name is in sets
            let variant = NaicsIndustry::from_str(name).unwrap();
            // check that the description matches the record for this variant
            let desc = variant.description();
            if desc.trim() != item.title().trim() {
                tracing::warn!(
                    "{desc} not the description of {variant}, expected {}.",
                    item.title().trim()
                );
                let error = KeyMissing::new(
                    variant.description().to_owned(),
                    line!(),
                    file!().to_string(),
                );
                return Err(error.into());
            }
            // check that the code matches the record for this variant
            let code = variant.code();
            if code != *item.code() {
                tracing::warn!(
                    "{code} not the code of {variant}, expected {}.",
                    item.code()
                );
                let error =
                    KeyMissing::new(variant.code().to_string(), line!(), file!().to_string());
                return Err(error.into());
            }
            if let Some(comp) = NaicsIndustry::from_code(&code.to_string()) {
                if variant != comp {
                    tracing::error!("Code {code} parses to variant {comp}, expected {variant}.");
                    let error = KeyMissing::new(variant.to_string(), line!(), file!().to_string());
                    return Err(error.into());
                }
            } else {
                tracing::error!("Code {code} fails to parse to variant {variant}.");
                let error =
                    KeyMissing::new(variant.code().to_string(), line!(), file!().to_string());
                return Err(error.into());
            }
        }
    }
    // all variants accounted for
    Ok(())
}
