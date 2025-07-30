use crate::{NaicsCategory, NaicsIndustry, NaicsSector, NaicsSubcategory, NaicsSubsector};

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
}

impl Naics {
    pub fn from_code(key: &str) -> Option<Self> {
        let code = match key.parse::<i64>() {
            Ok(num) => num,
            Err(_) => return None,
        };

        match code {
            11..100 => NaicsSector::from_code(key).map(|naics| naics.into()),
            111..1000 => NaicsSubsector::from_code(key).map(|naics| naics.into()),
            1111..10_000 => NaicsCategory::from_code(key).map(|naics| naics.into()),
            11111..100_000 => NaicsSubcategory::from_code(key).map(|naics| naics.into()),
            111110..1_000_000 => NaicsIndustry::from_code(key).map(|naics| naics.into()),
            _ => None,
        }
    }
}
