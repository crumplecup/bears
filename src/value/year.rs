use crate::{
    BeaErr, MneDoi, NipaYear, ParameterFields, ParameterValueTable, ParameterValueTableVariant,
    ParseInt, YearInvalid,
};

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    derive_more::Display,
    derive_new::new,
    derive_getters::Getters,
    serde::Deserialize,
    serde::Serialize,
)]
#[display("{}", self.date.year())]
pub struct Year {
    date: jiff::civil::Date,
    description: String,
}

impl TryFrom<&ParameterFields> for Year {
    type Error = ParseInt;
    fn try_from(value: &ParameterFields) -> Result<Self, Self::Error> {
        let date = parse_year(value.key())?;
        Ok(Self::new(date, value.desc().into()))
    }
}

impl TryFrom<&MneDoi> for Year {
    type Error = ParseInt;
    fn try_from(value: &MneDoi) -> Result<Self, Self::Error> {
        let date = parse_year(value.key())?;
        Ok(Self::new(date, value.desc().into()))
    }
}

pub fn parse_year(input: &str) -> Result<jiff::civil::Date, ParseInt> {
    match input.parse::<i16>() {
        Ok(num) => {
            let date = jiff::civil::date(num, 1, 1);
            Ok(date)
        }
        Err(source) => {
            let error = ParseInt::new(input.into(), source, line!(), file!().into());
            Err(error)
        }
    }
}

impl TryFrom<&ParameterValueTable> for Year {
    type Error = BeaErr;
    fn try_from(value: &ParameterValueTable) -> Result<Self, Self::Error> {
        match value {
            ParameterValueTable::ParameterFields(pf) => Ok(Self::try_from(pf)?),
            _ => {
                let error = ParameterValueTableVariant::new(
                    "ParameterFields needed".to_string(),
                    line!(),
                    file!().to_string(),
                );
                Err(error.into())
            }
        }
    }
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    strum::EnumIter,
)]
pub enum YearKind {
    #[default]
    All,
    Year(Year),
}

impl TryFrom<&ParameterFields> for YearKind {
    type Error = BeaErr;
    fn try_from(value: &ParameterFields) -> Result<Self, Self::Error> {
        match Year::try_from(value) {
            Ok(year) => Ok(Self::Year(year)),
            Err(_) => match value.key().as_str() {
                "all" => Ok(Self::All),
                other => {
                    let error = YearInvalid::new(other.into(), line!(), file!().to_string());
                    Err(error.into())
                }
            },
        }
    }
}

impl TryFrom<&MneDoi> for YearKind {
    type Error = BeaErr;
    fn try_from(value: &MneDoi) -> Result<Self, Self::Error> {
        match Year::try_from(value) {
            Ok(year) => Ok(Self::Year(year)),
            Err(_) => match value.key().as_str() {
                "all" => Ok(Self::All),
                other => {
                    let error = YearInvalid::new(other.into(), line!(), file!().to_string());
                    Err(error.into())
                }
            },
        }
    }
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    derive_new::new,
    derive_getters::Getters,
    serde::Deserialize,
    serde::Serialize,
)]
pub struct YearOptions {
    key: String,
    kind: YearKind,
}

impl TryFrom<&ParameterFields> for YearOptions {
    type Error = BeaErr;
    fn try_from(value: &ParameterFields) -> Result<Self, Self::Error> {
        let key = value.key().to_string();
        let kind = YearKind::try_from(value)?;
        Ok(Self::new(key, kind))
    }
}

impl TryFrom<&MneDoi> for YearOptions {
    type Error = BeaErr;
    fn try_from(value: &MneDoi) -> Result<Self, Self::Error> {
        let key = value.key().to_string();
        let kind = YearKind::try_from(value)?;
        Ok(Self::new(key, kind))
    }
}

impl TryFrom<&ParameterValueTable> for YearOptions {
    type Error = BeaErr;
    fn try_from(value: &ParameterValueTable) -> Result<Self, Self::Error> {
        match value {
            ParameterValueTable::ParameterFields(pf) => Ok(Self::try_from(pf)?),
            ParameterValueTable::MneDoi(tab) => Ok(Self::try_from(tab)?),
            other => {
                let error = ParameterValueTableVariant::new(
                    format!("ParameterFields or MneDoi needed, found {other:#?}"),
                    line!(),
                    file!().to_string(),
                );
                Err(error.into())
            }
        }
    }
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    derive_more::Display,
    derive_new::new,
    derive_getters::Getters,
    serde::Deserialize,
    serde::Serialize,
)]
#[display("{}-{}", self.first.year(), self.last.year())]
pub struct YearRange {
    first: jiff::civil::Date,
    last: jiff::civil::Date,
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    derive_new::new,
    derive_getters::Getters,
    derive_setters::Setters,
    serde::Deserialize,
    serde::Serialize,
)]
#[setters(prefix = "with_", strip_option)]
pub struct NipaRange {
    annual: Option<YearRange>,
    monthly: Option<YearRange>,
    quarterly: Option<YearRange>,
}

pub fn year_opt(input: &str) -> Result<Option<jiff::civil::Date>, ParseInt> {
    match input {
        "0" => Ok(None),
        num => Ok(Some(parse_year(num)?)),
    }
}

impl TryFrom<&NipaYear> for NipaRange {
    type Error = ParseInt;
    fn try_from(value: &NipaYear) -> Result<Self, Self::Error> {
        let from = year_opt(value.first_annual_year())?;
        let to = year_opt(value.last_annual_year())?;
        let annual = match (from, to) {
            (Some(first), Some(last)) => Some(YearRange::new(first, last)),
            _ => None,
        };
        let from = year_opt(value.first_monthly_year())?;
        let to = year_opt(value.last_monthly_year())?;
        let monthly = match (from, to) {
            (Some(first), Some(last)) => Some(YearRange::new(first, last)),
            _ => None,
        };
        let from = year_opt(value.first_quarterly_year())?;
        let to = year_opt(value.last_quarterly_year())?;
        let quarterly = match (from, to) {
            (Some(first), Some(last)) => Some(YearRange::new(first, last)),
            _ => None,
        };
        Ok(Self::new(annual, monthly, quarterly))
    }
}

impl TryFrom<&ParameterValueTable> for NipaRange {
    type Error = BeaErr;
    fn try_from(value: &ParameterValueTable) -> Result<Self, Self::Error> {
        match value {
            ParameterValueTable::NipaYear(nipa_year) => Ok(Self::try_from(nipa_year)?),
            _ => {
                let error = ParameterValueTableVariant::new(
                    "NipaFrequency needed".to_string(),
                    line!(),
                    file!().to_string(),
                );
                Err(error.into())
            }
        }
    }
}
