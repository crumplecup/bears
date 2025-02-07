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
    strum::EnumIter,
)]
pub enum Footnotes {
    #[default]
    Yes,
    No,
}

impl Footnotes {
    pub fn params(&self) -> (String, String) {
        let key = "GetFootnotes".to_string();
        let value = match self {
            Self::Yes => "yes".to_string(),
            Self::No => "no".to_string(),
        };
        (key, value)
    }
}
