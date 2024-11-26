use crate::BeaError;
use nom::character::complete::alphanumeric1;
use nom::IResult;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Action {
    Checklist,
    Download,
    #[default]
    Unknown,
}

impl std::str::FromStr for Action {
    type Err = BeaError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let action = match s {
            "checklist" => Self::Checklist,
            "download" => Self::Download,
            _ => Self::Unknown,
        };
        Ok(action)
    }
}

pub fn parse_command(command: &str) -> IResult<&str, &str> {
    let (rem, res) = alphanumeric1(command)?;
    Ok((rem, res))
}
