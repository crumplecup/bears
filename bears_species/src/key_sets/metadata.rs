use crate::{
    BeaErr, BeaResponse, Dataset, IoError, Metadata, ParameterValueTable,
    ParameterValueTableVariant, SerdeJson, Set,
};
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    derive_new::new,
    derive_more::Deref,
    derive_more::DerefMut,
    derive_more::AsRef,
    derive_more::AsMut,
)]
pub struct ApiMetadata(Vec<Metadata>);

impl TryFrom<&std::path::PathBuf> for ApiMetadata {
    type Error = BeaErr;
    fn try_from(value: &std::path::PathBuf) -> Result<Self, Self::Error> {
        let dataset = Dataset::APIDatasetMetadata;
        let names = dataset.names();
        // empty vectors to store values
        let mut metadata = Vec::new();
        // For each parameter in dataset
        for name in names {
            // open the file at the expected storage location, error if missing
            let path = value.join(format!(
                "parameter_values/{dataset}_{name}_parameter_values.json"
            ));
            let file = std::fs::File::open(&path)
                .map_err(|e| IoError::new(path, e, line!(), file!().into()))?;
            // read the file to json
            let rdr = std::io::BufReader::new(file);
            let res: serde_json::Value = serde_json::from_reader(rdr)
                .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
            // parse to internal bea response format
            let data = BeaResponse::try_from(&res)?;
            let results = data.results();
            // access parameter values from response
            if let Some(pf) = results.into_parameter_values() {
                for table in pf.iter() {
                    match table {
                        ParameterValueTable::Metadata(item) => {
                            metadata.push(item.clone());
                        }
                        other => {
                            let error = ParameterValueTableVariant::new(
                                format!("Metadata needed, found {other:#?}"),
                                line!(),
                                file!().to_string(),
                            );
                            return Err(error.into());
                        }
                    }
                }
            } else {
                tracing::warn!("Results must be of type ParameterValues");
                return Err(Set::ParameterValuesMissing.into());
            }
        }
        if metadata.is_empty() {
            tracing::warn!("Value field is empty.");
            Err(Set::Empty.into())
        } else {
            let table = Self::new(metadata);
            Ok(table)
        }
    }
}
