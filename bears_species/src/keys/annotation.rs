use crate::BeaErr;

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
    #[display("r")]
    R,
    #[display("--")]
    Dash,
    #[display("- - - - -")]
    Dashes,
    #[display("...")]
    Dots,
    #[display("n.a.")]
    NotApplicable,
    #[display("Note 2&#x0D;&#x0A;J")]
    Note,
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
            "r" => Self::R,
            "--" => Self::Dash,
            // the first hyphen gets grabbed by the parser as a minus sign
            "-" => Self::Dash,
            "- - - - -" => Self::Dashes,
            " - - - -" => Self::Dashes,
            "..." => Self::Dots,
            "...." => Self::Dots,
            "……………" => Self::Dots,
            "……………." => Self::Dots,
            "………………" => Self::Dots,
            "…………………" => Self::Dots,
            "n.a." => Self::NotApplicable,
            "Note 2&#x0D;&#x0A;J" => Self::Note,
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
            Self::R => "r",
            Self::Dash => "--",
            Self::Dashes => "- - - - -",
            Self::Dots => "...",
            Self::NotApplicable => "n.a.",
            Self::Note => "Note 2&#x0D;&#x0A;J",
            Self::Star => "(*)",
        };
        anno.to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display, derive_new::new)]
#[display("Annotation missing for {input} at line {line} in {file}")]
pub struct AnnotationMissing {
    input: String,
    line: u32,
    file: String,
}

impl std::error::Error for AnnotationMissing {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
