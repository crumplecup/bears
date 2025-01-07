use crate::{
    BeaErr, Integer, MneDoi, OwnershipInvalid, ParameterValueTable, ParameterValueTableVariant,
};

#[derive(
    Debug,
    Default,
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
    derive_more::FromStr,
    strum::EnumIter,
)]
pub enum OwnershipKind {
    #[default]
    AllAffiliates,
    MajorityOwnedAffiliates,
}

impl TryFrom<&MneDoi> for OwnershipKind {
    type Error = OwnershipInvalid;
    fn try_from(value: &MneDoi) -> Result<Self, Self::Error> {
        match value.key().as_str() {
            "0" => Ok(Self::MajorityOwnedAffiliates),
            "1" => Ok(Self::AllAffiliates),
            other => {
                let error = OwnershipInvalid::new(other.into(), line!(), file!().to_string());
                Err(error)
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
    serde::Deserialize,
    serde::Serialize,
    derive_new::new,
    derive_getters::Getters,
)]
pub struct OwnershipLevel {
    key: Integer,
    kind: OwnershipKind,
}

impl TryFrom<&MneDoi> for OwnershipLevel {
    type Error = BeaErr;
    fn try_from(value: &MneDoi) -> Result<Self, Self::Error> {
        let key = Integer::try_from(value)?;
        let kind = OwnershipKind::try_from(value)?;
        Ok(Self::new(key, kind))
    }
}

impl TryFrom<&ParameterValueTable> for OwnershipLevel {
    type Error = BeaErr;
    fn try_from(value: &ParameterValueTable) -> Result<Self, Self::Error> {
        match value {
            ParameterValueTable::MneDoi(tab) => Ok(Self::try_from(tab)?),
            other => {
                let error = ParameterValueTableVariant::new(
                    format!("MneDoi needed, found {other:#?}"),
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
    derive_more::FromStr,
    strum::EnumIter,
)]
pub enum AffiliateKind {
    #[default]
    All,
    NonBank,
}

impl TryFrom<&MneDoi> for AffiliateKind {
    type Error = OwnershipInvalid;
    fn try_from(value: &MneDoi) -> Result<Self, Self::Error> {
        match value.key().as_str() {
            "0" => Ok(Self::All),
            "1" => Ok(Self::NonBank),
            other => {
                let error = OwnershipInvalid::new(other.into(), line!(), file!().to_string());
                Err(error)
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
    serde::Deserialize,
    serde::Serialize,
    derive_new::new,
    derive_getters::Getters,
)]
pub struct AffiliateLevel {
    key: Integer,
    kind: AffiliateKind,
}

impl TryFrom<&MneDoi> for AffiliateLevel {
    type Error = BeaErr;
    fn try_from(value: &MneDoi) -> Result<Self, Self::Error> {
        let key = Integer::try_from(value)?;
        let kind = AffiliateKind::try_from(value)?;
        Ok(Self::new(key, kind))
    }
}

impl TryFrom<&ParameterValueTable> for AffiliateLevel {
    type Error = BeaErr;
    fn try_from(value: &ParameterValueTable) -> Result<Self, Self::Error> {
        match value {
            ParameterValueTable::MneDoi(tab) => Ok(Self::try_from(tab)?),
            other => {
                let error = ParameterValueTableVariant::new(
                    format!("MneDoi needed, found {other:#?}"),
                    line!(),
                    file!().to_string(),
                );
                Err(error.into())
            }
        }
    }
}
