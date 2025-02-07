use crate::{
    AnnotationMissing, BeaErr, BoolInvalid, IntegerInvalid, MneDoi, Nom, ParameterFields,
    ParameterValueTable, ParameterValueTableVariant, ParseInt,
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
    strum::EnumIter,
    derive_more::Display,
)]
pub enum Annotation {
    #[display("A")]
    A,
    #[display("(D)")]
    D,
    #[display("F")]
    F,
    #[display("G")]
    G,
    #[display("H")]
    H,
    #[display("I")]
    I,
    #[display("J")]
    J,
    #[display("K")]
    K,
    #[display("L")]
    L,
    #[display("M")]
    M,
    #[display("--")]
    Dash,
    #[display("- - - - -")]
    Dashes,
    #[display("...")]
    Dots,
    #[display("n.a.")]
    NotApplicable,
    #[display("(*)")]
    Star,
}

impl Annotation {
    pub fn from_value(value: &str) -> Result<Self, BeaErr> {
        let anno = match value {
            "(*)" => Self::Star,
            "A" => Self::A,
            "(D)" => Self::D,
            "F" => Self::F,
            "G" => Self::G,
            "H" => Self::H,
            "I" => Self::I,
            "J" => Self::J,
            "K" => Self::K,
            "L" => Self::L,
            "M" => Self::M,
            "--" => Self::Dash,
            // the first hyphen gets grabbed by the parser as a minus sign
            "-" => Self::Dash,
            "- - - - -" => Self::Dashes,
            " - - - -" => Self::Dashes,
            "..." => Self::Dots,
            "n.a." => Self::NotApplicable,
            _ => {
                let error = AnnotationMissing::new(value.to_string(), line!(), file!().to_string());
                return Err(error.into());
            }
        };
        Ok(anno)
    }

    pub fn as_value(&self) -> String {
        let anno = match self {
            Self::A => "A",
            Self::D => "(D)",
            Self::F => "F",
            Self::G => "G",
            Self::H => "H",
            Self::I => "I",
            Self::J => "J",
            Self::K => "K",
            Self::L => "L",
            Self::M => "M",
            Self::Dash => "--",
            Self::Dashes => "- - - - -",
            Self::Dots => "...",
            Self::NotApplicable => "n.a.",
            Self::Star => "(*)",
        };
        anno.to_string()
    }
}

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum AnnotatedInteger {
    Annotation(Annotation),
    Integer(i64),
    Both(Annotation, i64),
}

impl AnnotatedInteger {
    pub fn from_value(value: &str) -> Result<Self, BeaErr> {
        let mut value = value;
        let sign =
            match nom::bytes::complete::tag::<&str, &str, nom::error::Error<&str>>("-")(value) {
                Ok((rem, _)) => {
                    value = rem;
                    true
                }
                Err(_) => false,
            };
        match nom::character::complete::digit1::<&str, nom::error::Error<&str>>(value) {
            Ok((rem, res)) => {
                let number = match res.parse::<i64>() {
                    Ok(num) => {
                        if sign {
                            -num
                        } else {
                            num
                        }
                    }
                    Err(source) => {
                        let error =
                            ParseInt::new(res.to_string(), source, line!(), file!().to_string());
                        return Err(error.into());
                    }
                };
                let anno = String::from(rem);
                match Annotation::from_value(&anno) {
                    Ok(anno) => Ok(Self::Both(anno, number)),
                    Err(_) => Ok(Self::Integer(number)),
                }
            }
            Err(source) => match Annotation::from_value(value) {
                Ok(anno) => Ok(Self::Annotation(anno)),
                Err(_) => {
                    let error = Nom::new(
                        value.to_string(),
                        source.to_string(),
                        line!(),
                        file!().to_string(),
                    );
                    Err(error.into())
                }
            },
        }
    }

    pub fn as_value(&self) -> String {
        match self {
            Self::Annotation(anno) => anno.to_string(),
            Self::Integer(num) => num.to_string(),
            Self::Both(anno, num) => format!("{num}{anno}"),
        }
    }
}

impl Default for AnnotatedInteger {
    fn default() -> Self {
        Self::Integer(0)
    }
}
