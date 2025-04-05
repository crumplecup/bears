use assert_json_diff::assert_json_include;
use bears_species::{BeaErr, Dataset, IoError};
use std::io::BufRead;

pub struct Json;

impl Json {
    #[tracing::instrument]
    pub fn diff_parameters(dataset: Dataset) -> Result<(), BeaErr> {
        let path = format!("data/parameters_{dataset}.json");
        let response = std::path::Path::new(&path);
        let path = format!("data/bea_parameters_{dataset}.json");
        let native = std::path::Path::new(&path);
        let response = Json::read_lines(response)?;
        let mut response_lines = Vec::new();
        for line in response {
            let line = match line {
                Ok(result) => result,
                Err(source) => {
                    let error = IoError::new(path.into(), source, line!(), file!().into());
                    return Err(error.into());
                }
            };
            response_lines.push(line);
        }
        let native = Json::read_lines(native)?;
        let mut native_lines = Vec::new();
        for line in native {
            let line = match line {
                Ok(result) => result,
                Err(source) => {
                    let error = IoError::new(path.into(), source, line!(), file!().into());
                    return Err(error.into());
                }
            };
            native_lines.push(line);
        }

        let fused = response_lines
            .into_iter()
            .zip(native_lines)
            .collect::<Vec<(String, String)>>();
        for (res, nat) in fused {
            let diff = similar::TextDiff::from_lines(res.as_str(), nat.as_str());
            for op in diff.ops() {
                for change in diff.iter_changes(op) {
                    let (sign, style) = match change.tag() {
                        similar::ChangeTag::Delete => ("-", console::Style::new().red()),
                        similar::ChangeTag::Insert => ("+", console::Style::new().green()),
                        similar::ChangeTag::Equal => (" ", console::Style::new()),
                    };
                    tracing::info!("{}{}", style.apply_to(sign).bold(), style.apply_to(change));
                }
            }
        }

        Ok(())
    }

    #[tracing::instrument(skip_all)]
    pub fn datasets_diff(actual: &str, expected: &str) -> Result<(), IoError> {
        assert_json_include!(actual: actual, expected: expected);
        Ok(())
    }

    #[tracing::instrument]
    pub fn diff_datasets(path: &str) -> Result<(), BeaErr> {
        let p = format!("{path}/datasets.json");
        let response = std::path::Path::new(&p);
        tracing::info!("Response path is {response:?}");
        let p = format!("{path}/bea_datasets.json");
        let native = std::path::Path::new(&p);
        tracing::info!("Native path is {native:?}");
        let response = Json::read_lines(response)?;
        let mut response_lines = Vec::new();
        for line in response {
            let line = match line {
                Ok(result) => result,
                Err(source) => {
                    let error = IoError::new(path.into(), source, line!(), file!().into());
                    return Err(error.into());
                }
            };
            response_lines.push(line);
        }
        tracing::info!("Response json read.");
        let native = Json::read_lines(native)?;
        let mut native_lines = Vec::new();
        for line in native {
            let line = match line {
                Ok(result) => result,
                Err(source) => {
                    let error = IoError::new(path.into(), source, line!(), file!().into());
                    return Err(error.into());
                }
            };
            native_lines.push(line);
        }
        tracing::info!("Native json read.");

        let fused = response_lines
            .into_iter()
            .zip(native_lines)
            .collect::<Vec<(String, String)>>();
        for (res, nat) in fused {
            let diff = similar::TextDiff::from_lines(res.as_str(), nat.as_str());
            for op in diff.ops() {
                for change in diff.iter_changes(op) {
                    let (sign, style) = match change.tag() {
                        similar::ChangeTag::Delete => ("-", console::Style::new().red()),
                        similar::ChangeTag::Insert => ("+", console::Style::new().green()),
                        similar::ChangeTag::Equal => (" ", console::Style::new()),
                    };
                    tracing::info!("{}{}", style.apply_to(sign).bold(), style.apply_to(change));
                }
            }
        }

        Ok(())
    }
    // The output is wrapped in a Result to allow matching on errors.
    // Returns an Iterator to the Reader of the lines of the file.
    #[tracing::instrument(skip_all)]
    fn read_lines<P>(path: P) -> Result<std::io::Lines<std::io::BufReader<std::fs::File>>, IoError>
    where
        P: AsRef<std::path::Path>,
    {
        let path = std::path::PathBuf::from(path.as_ref());
        let file = std::fs::File::open(&path)
            .map_err(|e| IoError::new(path, e, line!(), file!().into()))?;
        Ok(std::io::BufReader::new(file).lines())
    }
}
