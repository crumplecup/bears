use crate::{
    NaicsCategory, NaicsIndustry, NaicsInputOutput, NaicsSector, NaicsSubcategory, NaicsSubsector,
    NaicsSupplement,
};

/// Parent enum for types of NAICS codes.
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_more::From,
)]
pub enum Naics {
    #[from(NaicsSector)]
    Sector(NaicsSector),
    #[from(NaicsSubsector)]
    Subsector(NaicsSubsector),
    #[from(NaicsCategory)]
    Category(NaicsCategory),
    #[from(NaicsSubcategory)]
    Subcategory(NaicsSubcategory),
    #[from(NaicsIndustry)]
    Industry(NaicsIndustry),
    #[from(NaicsSupplement)]
    Supplement(NaicsSupplement),
    #[from(NaicsInputOutput)]
    InputOutput(NaicsInputOutput),
}

impl Naics {
    pub fn from_code(key: &str) -> Option<Self> {
        // check that the key is a number
        let code = match key.parse::<i64>() {
            Ok(num) => num,
            Err(_) => {
                // Check for input output row/column codes, which can include letters
                if let Some(naics) = NaicsInputOutput::from_code(key) {
                    return Some(naics.into());
                } else {
                    // No codes match, return empty-handed
                    return None;
                }
            }
        };

        // check if key is supplement type
        if let Some(supplement) = NaicsSupplement::from_code(key) {
            return Some(Self::from(supplement));
        }

        // match key to naics code by length
        match code {
            11..100 => NaicsSector::from_code(key).map(|naics| naics.into()),
            111..1000 => NaicsSubsector::from_code(key).map(|naics| naics.into()),
            1111..10_000 => NaicsCategory::from_code(key).map(|naics| naics.into()),
            11111..100_000 => NaicsSubcategory::from_code(key).map(|naics| naics.into()),
            111110..1_000_000 => NaicsIndustry::from_code(key).map(|naics| naics.into()),
            _ => None,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Sector(naics) => naics.description(),
            Self::Subsector(naics) => naics.description(),
            Self::Category(naics) => naics.description(),
            Self::Subcategory(naics) => naics.description(),
            Self::Industry(naics) => naics.description(),
            Self::Supplement(naics) => naics.description(),
            Self::InputOutput(naics) => naics.description(),
        }
    }

    pub fn code(&self) -> String {
        let code = match self {
            Self::Sector(naics) => naics.code(),
            Self::Subsector(naics) => naics.code(),
            Self::Category(naics) => naics.code(),
            Self::Subcategory(naics) => naics.code(),
            Self::Industry(naics) => naics.code(),
            Self::Supplement(naics) => naics.code(),
            Self::InputOutput(naics) => return naics.code().to_owned(),
        };
        code.to_string()
    }
}
