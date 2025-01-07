use crate::{
    map_to_string, BeaErr, BeaResponse, JsonParseError, JsonParseErrorKind, NotObject,
    ParameterName, ReqwestError, User,
};
use convert_case::Casing;
use serde::{Deserialize, Serialize};

/// The `Dataset` enum contains variants for each dataset published by the BEA.
///
/// The enum should technically be marked as exhaustive, but the developer needs to lean on Rust's
/// exhaustive enum matching as a crutch.
///
/// We match the variants against the response from the
/// [`Method::GetDataSetList`](crate::Method::GetDataSetList) in a unit test to detect new
/// additions.
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
    strum::EnumIter,
    derive_more::Display,
    derive_more::FromStr,
    serde::Deserialize,
    serde::Serialize,
)]
pub enum Dataset {
    #[default]
    #[display("NIPA")]
    Nipa,
    NIUnderlyingDetail,
    #[display("MNE")]
    Mne,
    FixedAssets,
    #[display("ITA")]
    Ita,
    #[display("IIP")]
    Iip,
    InputOutput,
    IntlServTrade,
    IntlServSTA,
    GDPbyIndustry,
    Regional,
    UnderlyingGDPbyIndustry,
    APIDatasetMetadata,
}

impl Dataset {
    /// The `lower` method converts the variant name to `lowercase` case using
    /// [`convert_case::Case::Flat`].
    #[tracing::instrument]
    pub fn lower(&self) -> String {
        self.to_string().to_case(convert_case::Case::Flat)
    }

    /// The `names` method returns the vector of valid [`ParameterName`] inputs for a given
    /// `Dataset`.
    ///
    /// We match values for each variant manually against the responses from the
    /// [`Method::GetParameterList`](crate::Method::GetParameterList) call for each [`Dataset`]
    /// variant.
    ///
    /// TODO: Match the output against the responses in a unit test to detect changes or additions.
    pub fn names(&self) -> Vec<ParameterName> {
        match self {
            Self::Nipa => {
                vec![
                    ParameterName::Frequency,
                    ParameterName::ShowMillions,
                    ParameterName::TableID,
                    ParameterName::TableName,
                    ParameterName::Year,
                ]
            }
            Self::NIUnderlyingDetail => {
                vec![
                    ParameterName::Frequency,
                    ParameterName::TableID,
                    ParameterName::TableName,
                    ParameterName::Year,
                ]
            }
            Self::Mne => {
                vec![
                    ParameterName::DirectionOfInvestment,
                    ParameterName::OwnershipLevel,
                    ParameterName::NonbankAffiliatesOnly,
                    ParameterName::Classification,
                    ParameterName::Country,
                    ParameterName::Industry,
                    ParameterName::Year,
                    ParameterName::State,
                    ParameterName::SeriesID,
                    ParameterName::GetFootnotes,
                    ParameterName::Investment,
                    ParameterName::ParentInvestment,
                ]
            }
            Self::FixedAssets => {
                vec![ParameterName::TableName, ParameterName::Year]
            }
            Self::Ita => {
                vec![
                    ParameterName::Indicator,
                    ParameterName::AreaOrCountry,
                    ParameterName::Frequency,
                    ParameterName::Year,
                ]
            }
            Self::Iip => {
                vec![
                    ParameterName::TypeOfInvestment,
                    ParameterName::Component,
                    ParameterName::Frequency,
                    ParameterName::Year,
                ]
            }
            Self::InputOutput => {
                vec![ParameterName::TableID, ParameterName::Year]
            }
            Self::IntlServTrade => {
                vec![
                    ParameterName::TypeOfService,
                    ParameterName::TradeDirection,
                    ParameterName::Affiliation,
                    ParameterName::AreaOrCountry,
                    ParameterName::Year,
                ]
            }
            Self::IntlServSTA => {
                vec![
                    ParameterName::Channel,
                    ParameterName::Destination,
                    ParameterName::Industry,
                    ParameterName::AreaOrCountry,
                    ParameterName::Year,
                ]
            }
            Self::GDPbyIndustry => {
                vec![
                    ParameterName::Frequency,
                    ParameterName::Industry,
                    ParameterName::TableID,
                    ParameterName::Year,
                ]
            }
            Self::Regional => {
                vec![
                    ParameterName::GeoFips,
                    ParameterName::LineCode,
                    ParameterName::TableName,
                    ParameterName::Year,
                ]
            }
            Self::UnderlyingGDPbyIndustry => {
                vec![
                    ParameterName::Frequency,
                    ParameterName::Industry,
                    ParameterName::TableID,
                    ParameterName::Year,
                ]
            }
            Self::APIDatasetMetadata => {
                vec![ParameterName::Dataset]
            }
        }
    }
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Deserialize,
    Serialize,
    derive_new::new,
    derive_getters::Getters,
)]
#[serde(rename_all = "PascalCase")]
pub struct DatasetDetails {
    dataset_description: String,
    dataset_name: String,
}

impl DatasetDetails {
    #[deprecated]
    #[tracing::instrument(skip_all)]
    pub async fn parameters(&self, user: &User) -> Result<BeaResponse, ReqwestError> {
        let mut body = user.body();
        body.push_str("&method=GETPARAMETERLIST");
        body.push_str(&format!("&datasetname={}", self.dataset_name));
        let url = body.clone();
        let client = reqwest::Client::new();
        match client.get(body).send().await {
            Ok(res) => {
                tracing::trace!("Response code: {}.", res.status());
                match res.json::<BeaResponse>().await {
                    Ok(r) => Ok(r),
                    Err(source) => {
                        let error = ReqwestError::new(
                            url,
                            "get".into(),
                            source,
                            line!(),
                            file!().to_string(),
                        );
                        return Err(error);
                    }
                }
            }
            Err(source) => {
                let error =
                    ReqwestError::new(url, "get".into(), source, line!(), file!().to_string());
                return Err(error);
            }
        }
    }

    #[deprecated]
    pub fn name(&self) -> String {
        self.dataset_name.to_owned()
    }
}

impl TryFrom<serde_json::Value> for DatasetDetails {
    type Error = BeaErr;
    // #[tracing::instrument(skip_all)]
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        match value {
            serde_json::Value::Object(m) => {
                let name = map_to_string("DatasetName", &m)?;
                let desc = map_to_string("DatasetDescription", &m)?;
                Ok(Self::new(desc, name))
            }
            _ => {
                tracing::trace!("Invalid Value: {value:#?}");
                let error = NotObject::new(line!(), file!().to_string());
                let error = JsonParseErrorKind::from(error);
                let error = JsonParseError::from(error);
                Err(error.into())
            }
        }
    }
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Deserialize,
    Serialize,
    derive_new::new,
    derive_more::Deref,
    derive_more::DerefMut,
)]
#[serde(rename_all = "PascalCase")]
pub struct Datasets {
    dataset: Vec<DatasetDetails>,
}

impl TryFrom<serde_json::Value> for Datasets {
    type Error = BeaErr;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        tracing::trace!("Reading DatasetDetails");
        match value {
            serde_json::Value::Object(m) => {
                let key = "Dataset".to_string();
                if let Some(serde_json::Value::Array(v)) = m.get(&key) {
                    tracing::trace!("Array found.");
                    let mut dataset = Vec::new();
                    for val in v {
                        let details = DatasetDetails::try_from(val.clone())?;
                        dataset.push(details);
                    }
                    tracing::trace!("Details found.");
                    let datasets = Datasets::new(dataset);
                    Ok(datasets)
                } else {
                    tracing::trace!("Unexpected content: {m:#?}");
                    let error = JsonParseErrorKind::KeyMissing(key);
                    let error = JsonParseError::from(error);
                    Err(error.into())
                }
            }
            _ => {
                tracing::trace!("Wrong Value type: {value:#?}");
                let error = NotObject::new(line!(), file!().to_string());
                let error = JsonParseErrorKind::from(error);
                let error = JsonParseError::from(error);
                Err(error.into())
            }
        }
    }
}
