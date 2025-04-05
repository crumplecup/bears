use crate::{
    BeaErr, DeriveFromStr, MneDoi, ParameterFields, ParameterValueTable, ParameterValueTableVariant,
};
use convert_case::Casing;
use std::str::FromStr;

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
    derive_new::new,
    derive_getters::Getters,
)]
pub struct State {
    key: String,
    kind: StateKind,
}

impl TryFrom<&ParameterFields> for State {
    type Error = BeaErr;
    fn try_from(value: &ParameterFields) -> Result<Self, Self::Error> {
        let key = value.key().to_string();
        let kind = StateKind::try_from(value)?;
        Ok(Self::new(key, kind))
    }
}

impl TryFrom<&MneDoi> for State {
    type Error = BeaErr;
    fn try_from(value: &MneDoi) -> Result<Self, Self::Error> {
        let key = value.key().to_string();
        let kind = StateKind::try_from(value)?;
        Ok(Self::new(key, kind))
    }
}

impl TryFrom<&ParameterValueTable> for State {
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
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Deserialize,
    serde::Serialize,
    derive_more::Display,
    derive_more::FromStr,
    strum::EnumIter,
)]
pub enum StateKind {
    #[default]
    All,
    TotalStatesAndAreas,
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    AmericanSomoa,
    California,
    Colorado,
    Connecticut,
    Delaware,
    DistrictOfColumbia,
    Florida,
    Georgia,
    Guam,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    NewHampshire,
    NewJersey,
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    NorthernMarianaIslands,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    PuertoRico,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    TrustTerritories,
    Utah,
    Vermont,
    Virginia,
    VirginIslands,
    Washington,
    WestVirginia,
    Wisconsin,
    Wyoming,
    Foreign,
    OtherUsAreas,
    Unspecified,
    UnspecifiedStateOrArea,
}

impl StateKind {
    /// The `pascal` method converts the variant name to `Pascal` case using
    /// [`convert_case::Case::Pascal`].
    #[tracing::instrument]
    pub fn pascal(&self) -> String {
        self.to_string().to_case(convert_case::Case::Pascal)
    }
}

impl TryFrom<&ParameterFields> for StateKind {
    type Error = DeriveFromStr;
    fn try_from(value: &ParameterFields) -> Result<Self, Self::Error> {
        let pascal = value.desc().to_case(convert_case::Case::Pascal);
        match Self::from_str(&pascal) {
            Ok(state) => Ok(state),
            Err(source) => {
                let input = value.desc().trim();
                match input {
                    "all" => Ok(Self::All),
                    "Total States and Areas" => Ok(Self::TotalStatesAndAreas),
                    "District of Columbia" => Ok(Self::DistrictOfColumbia),
                    "Other U.S. areas" => Ok(Self::OtherUsAreas),
                    "Unspecicied State or Area" => Ok(Self::UnspecifiedStateOrArea),
                    other => {
                        let error = DeriveFromStr::new(
                            other.to_string(),
                            source,
                            line!(),
                            file!().to_string(),
                        );
                        Err(error)
                    }
                }
            }
        }
    }
}

impl TryFrom<&MneDoi> for StateKind {
    type Error = DeriveFromStr;
    fn try_from(value: &MneDoi) -> Result<Self, Self::Error> {
        let pascal = value.desc().to_case(convert_case::Case::Pascal);
        match Self::from_str(&pascal) {
            Ok(state) => Ok(state),
            Err(source) => {
                let input = value.desc().trim();
                match input {
                    "all" => Ok(Self::All),
                    "Total States and Areas" => Ok(Self::TotalStatesAndAreas),
                    "District of Columbia" => Ok(Self::DistrictOfColumbia),
                    "Other U.S. areas" => Ok(Self::OtherUsAreas),
                    "Unspecicied State or Area" => Ok(Self::UnspecifiedStateOrArea),
                    other => {
                        let error = DeriveFromStr::new(
                            other.to_string(),
                            source,
                            line!(),
                            file!().to_string(),
                        );
                        Err(error)
                    }
                }
            }
        }
    }
}
