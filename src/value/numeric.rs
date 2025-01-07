use crate::{
    BeaErr, BoolInvalid, IntegerInvalid, MneDoi, ParameterFields, ParameterValueTable,
    ParameterValueTableVariant, ParseInt,
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
#[display("{}", self.value)]
pub struct Integer {
    value: i32,
    description: String,
}

impl TryFrom<&ParameterFields> for Integer {
    type Error = ParseInt;
    fn try_from(value: &ParameterFields) -> Result<Self, Self::Error> {
        match value.key().parse::<i32>() {
            Ok(num) => Ok(Self::new(num, value.desc().into())),
            Err(source) => {
                let error = ParseInt::new(value.key().into(), source, line!(), file!().into());
                Err(error)
            }
        }
    }
}

impl TryFrom<&MneDoi> for Integer {
    type Error = ParseInt;
    fn try_from(value: &MneDoi) -> Result<Self, Self::Error> {
        match value.key().parse::<i32>() {
            Ok(num) => Ok(Self::new(num, value.desc().into())),
            Err(source) => {
                let error = ParseInt::new(value.key().into(), source, line!(), file!().into());
                Err(error)
            }
        }
    }
}

impl TryFrom<&ParameterValueTable> for Integer {
    type Error = BeaErr;
    fn try_from(value: &ParameterValueTable) -> Result<Self, Self::Error> {
        match value {
            ParameterValueTable::ParameterFields(pf) => Ok(Integer::try_from(pf)?),
            ParameterValueTable::MneDoi(tab) => Ok(Integer::try_from(tab)?),
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
    serde::Serialize,
    serde::Deserialize,
    strum::EnumIter,
)]
pub enum IntegerKind {
    #[default]
    All,
    Integer(Integer),
}

impl TryFrom<&ParameterFields> for IntegerKind {
    type Error = BeaErr;
    fn try_from(value: &ParameterFields) -> Result<Self, Self::Error> {
        match Integer::try_from(value) {
            Ok(year) => Ok(Self::Integer(year)),
            Err(_) => match value.key().as_str() {
                "all" => Ok(Self::All),
                other => {
                    let error = IntegerInvalid::new(other.into(), line!(), file!().to_string());
                    Err(error.into())
                }
            },
        }
    }
}

impl TryFrom<&MneDoi> for IntegerKind {
    type Error = BeaErr;
    fn try_from(value: &MneDoi) -> Result<Self, Self::Error> {
        match Integer::try_from(value) {
            Ok(year) => Ok(Self::Integer(year)),
            Err(_) => match value.key().as_str() {
                "all" => Ok(Self::All),
                other => {
                    let error = IntegerInvalid::new(other.into(), line!(), file!().to_string());
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
pub struct IntegerOptions {
    key: String,
    kind: IntegerKind,
}

impl TryFrom<&ParameterFields> for IntegerOptions {
    type Error = BeaErr;
    fn try_from(value: &ParameterFields) -> Result<Self, Self::Error> {
        let key = value.key().to_string();
        let kind = IntegerKind::try_from(value)?;
        Ok(Self::new(key, kind))
    }
}

impl TryFrom<&MneDoi> for IntegerOptions {
    type Error = BeaErr;
    fn try_from(value: &MneDoi) -> Result<Self, Self::Error> {
        let key = value.key().to_string();
        let kind = IntegerKind::try_from(value)?;
        Ok(Self::new(key, kind))
    }
}

impl TryFrom<&ParameterValueTable> for IntegerOptions {
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
    derive_new::new,
    derive_getters::Getters,
    serde::Deserialize,
    serde::Serialize,
)]
pub struct BoolOptions {
    description: String,
    key: bool,
}

impl BoolOptions {
    pub fn to_key(&self) -> String {
        match self.key {
            true => "yes".to_string(),
            false => "no".to_string(),
        }
    }
}

impl TryFrom<&ParameterFields> for BoolOptions {
    type Error = BoolInvalid;
    fn try_from(value: &ParameterFields) -> Result<Self, Self::Error> {
        let key = match value.key().as_str() {
            "yes" => true,
            "no" => false,
            other => {
                let error = BoolInvalid::new(other.into(), line!(), file!().to_string());
                return Err(error);
            }
        };
        let description = value.desc().to_string();
        Ok(Self::new(description, key))
    }
}

impl TryFrom<&MneDoi> for BoolOptions {
    type Error = BoolInvalid;
    fn try_from(value: &MneDoi) -> Result<Self, Self::Error> {
        let key = match value.key().as_str() {
            "yes" => true,
            "no" => false,
            other => {
                let error = BoolInvalid::new(other.into(), line!(), file!().to_string());
                return Err(error);
            }
        };
        let description = value.desc().to_string();
        Ok(Self::new(description, key))
    }
}

impl TryFrom<&ParameterValueTable> for BoolOptions {
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
