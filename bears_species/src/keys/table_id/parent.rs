use crate::{Code, Describe, GdpTable, UgdpTable};

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
    derive_more::Display,
    derive_more::From,
)]
pub enum TableId {
    #[from(GdpTable)]
    Gdp(GdpTable),
    #[from(UgdpTable)]
    Ugdp(UgdpTable),
}

impl Describe for TableId {
    fn description(&self) -> &'static str {
        match self {
            Self::Gdp(data) => data.description(),
            Self::Ugdp(data) => data.description(),
        }
    }
}

impl Code<i64> for TableId {
    type Decoded = Self;

    fn code(&self) -> i64 {
        match self {
            Self::Gdp(data) => data.code(),
            Self::Ugdp(data) => data.code(),
        }
    }

    fn from_code(code: i64) -> Option<Self::Decoded> {
        match code < 210 {
            true => GdpTable::from_code(code).map(TableId::from),
            false => UgdpTable::from_code(code).map(TableId::from),
        }
    }
}
