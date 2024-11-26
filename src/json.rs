use crate::{BeaError, Dataset};
use std::io::BufRead;

pub struct Json;

impl Json {
    pub fn diff_parameters(dataset: Dataset) -> Result<(), BeaError> {
        let path = format!("data/parameters_{dataset}.json");
        let response = std::path::Path::new(&path);
        let path = format!("data/bea_parameters_{dataset}.json");
        let native = std::path::Path::new(&path);
        let response = Json::read_lines(response)?;
        let mut response_lines = Vec::new();
        for line in response {
            response_lines.push(line?);
        }
        let native = Json::read_lines(native)?;
        let mut native_lines = Vec::new();
        for line in native {
            native_lines.push(line?);
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

    pub fn diff_datasets() -> Result<(), BeaError> {
        let path = "data/datasets.json";
        let response = std::path::Path::new(path);
        let path = "data/bea_datasets.json";
        let native = std::path::Path::new(path);
        let response = Json::read_lines(response)?;
        let mut response_lines = Vec::new();
        for line in response {
            response_lines.push(line?);
        }
        let native = Json::read_lines(native)?;
        let mut native_lines = Vec::new();
        for line in native {
            native_lines.push(line?);
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
    // The output is wrapped in a Result to allow matching on errors.
    // Returns an Iterator to the Reader of the lines of the file.
    fn read_lines<P>(path: P) -> std::io::Result<std::io::Lines<std::io::BufReader<std::fs::File>>>
    where
        P: AsRef<std::path::Path>,
    {
        let file = std::fs::File::open(path)?;
        Ok(std::io::BufReader::new(file).lines())
    }
}
