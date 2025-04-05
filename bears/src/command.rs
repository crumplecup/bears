use nom::character::complete::alphanumeric1;
use nom::IResult;

/// Variants of the `Action` enum encapsulate the different actions a user can select, exposing the
/// different capabilities of the library.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Action {
    Checklist,
    Download,
    #[default]
    Unknown,
}

impl std::str::FromStr for Action {
    type Err = FromStrError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let action = match s {
            "checklist" => Self::Checklist,
            "download" => Self::Download,
            _ => Self::Unknown,
        };
        if action == Self::Unknown {
            let input = s.to_string();
            let target = "Action".to_string();
            let error = FromStrError::new(input, target);
            Err(error)
        } else {
            Ok(action)
        }
    }
}

/// Context information around the failure to parse a String into a library type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, derive_new::new)]
pub struct FromStrError {
    /// What the user attempted to parse into the `target` type.
    input: String,
    /// Type the user is trying to parse the `input` into.
    target: String,
}

impl std::fmt::Display for FromStrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} does not match variant of {}",
            self.input, self.target
        )
    }
}

impl std::error::Error for FromStrError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[tracing::instrument]
pub fn parse_command(command: &str) -> IResult<&str, &str> {
    let (rem, res) = alphanumeric1(command)?;
    Ok((rem, res))
}
