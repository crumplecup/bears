use crate::{
    BeaErr, KeyMissing, ParameterFields, ParameterValueTable, ParameterValueTableVariant, ParseInt,
};

/// Represents different types of input-output tables in an economic analysis framework.
///
/// These tables are categorized by their relationship type (Commodity-by-Commodity,
/// Industry-by-Commodity, Industry-by-Industry) and their level of detail (Summary or Sector).
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Deserialize,
    serde::Serialize,
    strum::EnumIter,
    derive_more::Display,
    derive_more::FromStr,
)]
pub enum InputOutputTable {
    /// Commodity-by-Commodity Total Requirements, After Redefinitions - Summary (Key: 59)
    CommodityTotalRequirementsSummary,
    /// Commodity-by-Commodity Total Requirements, After Redefinitions - Sector (Key: 58)
    CommodityTotalRequirementsSector,
    /// Industry-by-Commodity Total Requirements, After Redefinitions - Summary (Key: 57)
    IndustryByCommodityRequirementsSummary,
    /// Industry-by-Commodity Total Requirements, After Redefinitions - Sector (Key: 56)
    IndustryByCommodityRequirementsSector,
    /// Industry-by-Industry Total Requirements, After Redefinitions - Summary (Key: 61)
    IndustryTotalRequirementsSummary,
    /// Industry-by-Industry Total Requirements, After Redefinitions - Sector (Key: 60)
    IndustryTotalRequirementsSector,
    /// The Domestic Supply of Commodities by Industries - Summary (Key: 262)
    DomesticSupplySummary,
    /// The Domestic Supply of Commodities by Industries - Sector (Key: 261)
    DomesticSupplySector,
    /// The Use of Commodities by Industries - Summary (Key: 259)
    CommodityUseSummary,
    /// The Use of Commodities by Industries - Sector (Key: 258)
    CommodityUseSector,
}

impl InputOutputTable {
    /// Returns the full description of the input-output table type.
    ///
    /// # Returns
    ///
    /// A static string containing the official description of the table type.
    pub fn description(&self) -> &'static str {
        match self {
            Self::CommodityTotalRequirementsSummary => {
                "Commodity-by-Commodity Total Requirements, After Redefinitions - Summary"
            }
            Self::CommodityTotalRequirementsSector => {
                "Commodity-by-Commodity Total Requirements, After Redefinitions - Sector"
            }
            Self::IndustryByCommodityRequirementsSummary => {
                "Industry-by-Commodity Total Requirements, After Redefinitions - Summary"
            }
            Self::IndustryByCommodityRequirementsSector => {
                "Industry-by-Commodity Total Requirements, After Redefinitions - Sector"
            }
            Self::IndustryTotalRequirementsSummary => {
                "Industry-by-Industry Total Requirements, After Redefinitions - Summary"
            }
            Self::IndustryTotalRequirementsSector => {
                "Industry-by-Industry Total Requirements, After Redefinitions - Sector"
            }
            Self::DomesticSupplySummary => {
                "The Domestic Supply of Commodities by Industries - Summary"
            }
            Self::DomesticSupplySector => {
                "The Domestic Supply of Commodities by Industries - Sector"
            }
            Self::CommodityUseSummary => "The Use of Commodities by Industries - Summary",
            Self::CommodityUseSector => "The Use of Commodities by Industries - Sector",
        }
    }

    /// Returns the numeric key associated with the input-output table type.
    ///
    /// # Returns
    ///
    /// An integer representing the key identifier for the table type.
    pub fn key(&self) -> i64 {
        match self {
            Self::CommodityTotalRequirementsSummary => 59,
            Self::CommodityTotalRequirementsSector => 58,
            Self::IndustryByCommodityRequirementsSummary => 57,
            Self::IndustryByCommodityRequirementsSector => 56,
            Self::IndustryTotalRequirementsSummary => 61,
            Self::IndustryTotalRequirementsSector => 60,
            Self::DomesticSupplySummary => 262,
            Self::DomesticSupplySector => 261,
            Self::CommodityUseSummary => 259,
            Self::CommodityUseSector => 258,
        }
    }

    /// Creates an InputOutputTable instance by parsing a string key.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice containing the key to parse
    ///
    /// # Returns
    ///
    /// * `Ok(InputOutputTable)` - If the key is valid and matches a known table type
    /// * `Err(BeaErr)` - If the key cannot be parsed as i64 (ParseInt error) or if the key is not found (KeyMissing error)
    ///
    /// # Errors
    ///
    /// Returns a `ParseInt` error if the string cannot be parsed as i64.
    /// Returns a `KeyMissing` error if the parsed integer does not match any known key.
    pub fn from_key(key: &str) -> Result<Self, BeaErr> {
        let result = match key
            .parse::<i64>()
            .map_err(|e| ParseInt::new(key.to_owned(), e, line!(), file!().to_string()))?
        {
            59 => Self::CommodityTotalRequirementsSummary,
            58 => Self::CommodityTotalRequirementsSector,
            57 => Self::IndustryByCommodityRequirementsSummary,
            56 => Self::IndustryByCommodityRequirementsSector,
            61 => Self::IndustryTotalRequirementsSummary,
            60 => Self::IndustryTotalRequirementsSector,
            262 => Self::DomesticSupplySummary,
            261 => Self::DomesticSupplySector,
            259 => Self::CommodityUseSummary,
            258 => Self::CommodityUseSector,
            _ => return Err(KeyMissing::new(key.to_owned(), line!(), file!().to_owned()).into()),
        };
        Ok(result)
    }
}

impl TryFrom<&ParameterFields> for InputOutputTable {
    type Error = BeaErr;
    fn try_from(value: &ParameterFields) -> Result<Self, Self::Error> {
        Self::from_key(value.key())
    }
}

impl TryFrom<&ParameterValueTable> for InputOutputTable {
    type Error = BeaErr;
    fn try_from(value: &ParameterValueTable) -> Result<Self, Self::Error> {
        match value {
            ParameterValueTable::ParameterFields(pf) => Ok(Self::try_from(pf)?),
            _ => {
                let msg = format!("ParameterFields needed, found {value:#?}");
                let error = ParameterValueTableVariant::new(msg, line!(), file!().to_owned());
                Err(error.into())
            }
        }
    }
}
