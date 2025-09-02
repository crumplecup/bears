use crate::{
    AffiliateLevel, BeaErr, BeaResponse, BoolOptions, Dataset, DirectionOfInvestment, Footnotes,
    Integer, IntegerKind, IntegerOptions, IoError, MneDoi, OwnershipLevel, ParameterName,
    ParameterValueTable, ParameterValueTableVariant, SelectionKind, SerdeJson, Set, State,
    YearKind, YearOptions,
};
use strum::IntoEnumIterator;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_getters::Getters,
)]
pub struct Mne {
    classification: Vec<MneDoi>,
    country: Vec<IntegerOptions>,
    direction_of_investment: Vec<DirectionOfInvestment>,
    get_footnotes: Vec<BoolOptions>,
    industry: Vec<IntegerOptions>,
    investment: Vec<IntegerOptions>,
    nonbank_affiliates_only: Vec<AffiliateLevel>,
    ownership_level: Vec<OwnershipLevel>,
    parent_investment: Vec<IntegerOptions>,
    series_id: Vec<Integer>,
    state: Vec<State>,
    year: Vec<YearOptions>,
}

impl Mne {
    pub fn iter(&self) -> MneIterator<'_> {
        let series_options = SelectionKind::default();
        let industry_options = SelectionKind::default();
        let country_options = SelectionKind::Individual;
        let year_options = SelectionKind::default();
        let footnotes = Footnotes::default();
        MneIterator::new(
            self,
            series_options,
            industry_options,
            country_options,
            year_options,
            footnotes,
        )
    }

    // pub fn queue() -> Result<Queue, BeaErr> {
    //     let req = Request::Data;
    //     let mut app = req.init()?;
    //     let dataset = Dataset::Mne;
    //     app.with_dataset(dataset);
    //     dotenvy::dotenv().ok();
    //     let path = bea_data()?;
    //     let data = Mne::try_from(&path)?;
    //     let mut queue = Vec::new();
    //     for params in data.iter() {
    //         tracing::trace!("{params:#?}");
    //         app.with_params(params.clone());
    //         queue.push(app.clone());
    //     }
    //     Ok(Queue::new(queue))
    // }
}

impl TryFrom<&std::path::PathBuf> for Mne {
    type Error = BeaErr;
    fn try_from(value: &std::path::PathBuf) -> Result<Self, Self::Error> {
        let dataset = Dataset::Mne;
        let names = dataset.names();
        // empty vectors to store values
        let mut classification = Vec::new();
        let mut country = Vec::new();
        let mut direction_of_investment = Vec::new();
        let mut get_footnotes = Vec::new();
        let mut industry = Vec::new();
        let mut investment = Vec::new();
        let mut nonbank_affiliates_only = Vec::new();
        let mut ownership_level = Vec::new();
        let mut parent_investment = Vec::new();
        let mut series_id = Vec::new();
        let mut state = Vec::new();
        let mut year = Vec::new();
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
            if let Some(pv) = results.into_parameter_values() {
                // type of vector varies by parameter name
                match name {
                    ParameterName::Classification => {
                        for table in pv.iter() {
                            match table {
                                ParameterValueTable::MneDoi(tab) => {
                                    classification.push(tab.clone());
                                }
                                other => {
                                    let error = ParameterValueTableVariant::new(
                                        format!("MneDoi needed, found {other:#?}"),
                                        line!(),
                                        file!().to_string(),
                                    );
                                    return Err(error.into());
                                }
                            }
                        }
                    }
                    ParameterName::Country => {
                        for table in pv.iter() {
                            country.push(IntegerOptions::try_from(table)?);
                        }
                    }
                    ParameterName::DirectionOfInvestment => {
                        for table in pv.iter() {
                            direction_of_investment.push(DirectionOfInvestment::try_from(table)?);
                        }
                    }
                    ParameterName::GetFootnotes => {
                        for table in pv.iter() {
                            get_footnotes.push(BoolOptions::try_from(table)?);
                        }
                    }
                    ParameterName::Industry => {
                        for table in pv.iter() {
                            industry.push(IntegerOptions::try_from(table)?);
                        }
                    }
                    ParameterName::Investment => {
                        for table in pv.iter() {
                            investment.push(IntegerOptions::try_from(table)?);
                        }
                    }
                    ParameterName::NonbankAffiliatesOnly => {
                        for table in pv.iter() {
                            nonbank_affiliates_only.push(AffiliateLevel::try_from(table)?);
                        }
                    }
                    ParameterName::OwnershipLevel => {
                        for table in pv.iter() {
                            ownership_level.push(OwnershipLevel::try_from(table)?);
                        }
                    }
                    ParameterName::ParentInvestment => {
                        for table in pv.iter() {
                            parent_investment.push(IntegerOptions::try_from(table)?);
                        }
                    }
                    ParameterName::SeriesID => {
                        for table in pv.iter() {
                            series_id.push(Integer::try_from(table)?);
                        }
                    }
                    ParameterName::State => {
                        tracing::trace!("Building State values.");
                        for table in pv.iter() {
                            state.push(State::try_from(table)?);
                        }
                    }
                    ParameterName::Year => {
                        tracing::trace!("Building Year values.");
                        for table in pv.iter() {
                            year.push(YearOptions::try_from(table)?);
                        }
                    }
                    other => return Err(Set::ParameterNameMissing(other.to_string()).into()),
                }
            } else {
                tracing::warn!("Results must be of type ParameterValues");
                return Err(Set::ParameterValuesMissing.into());
            }
        }
        if classification.is_empty()
            || country.is_empty()
            || direction_of_investment.is_empty()
            || get_footnotes.is_empty()
            || industry.is_empty()
            || investment.is_empty()
            || nonbank_affiliates_only.is_empty()
            || ownership_level.is_empty()
            || parent_investment.is_empty()
            || series_id.is_empty()
            || state.is_empty()
            || year.is_empty()
        {
            tracing::warn!("Value field is empty.");
            Err(Set::Empty.into())
        } else {
            let table = Self {
                classification,
                country,
                direction_of_investment,
                get_footnotes,
                industry,
                investment,
                nonbank_affiliates_only,
                ownership_level,
                parent_investment,
                series_id,
                state,
                year,
            };
            Ok(table)
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
    serde::Serialize,
    serde::Deserialize,
    strum::EnumIter,
)]
pub enum MneKind {
    #[default]
    Amne,
    Di,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, derive_setters::Setters)]
#[setters(prefix = "with_", borrow_self, into)]
pub struct MneIterator<'a> {
    #[setters(skip)]
    data: &'a Mne,
    series_options: SelectionKind,
    industry_options: SelectionKind,
    country_options: SelectionKind,
    year_options: SelectionKind,
    footnotes: Footnotes,
    // Kinds of Mne dataset to request (DI or AMNE)
    mne_kinds: Vec<MneKind>,
    #[setters(skip)]
    mne_index: usize,
    #[setters(skip)]
    mne_end: bool,
    // index into data.ownership_level
    #[setters(skip)]
    ownership_index: usize,
    #[setters(skip)]
    ownership_end: bool,
    // index into data.nonbank_affiliates_only
    #[setters(skip)]
    nonbank_index: usize,
    #[setters(skip)]
    nonbank_end: bool,
    // index into data.direction_of_investment
    #[setters(skip)]
    doi_index: usize,
    #[setters(skip)]
    doi_end: bool,
    // index into data.classification
    #[setters(skip)]
    class_index: usize,
    #[setters(skip)]
    class_end: bool,
    // index into data.series_id
    #[setters(skip)]
    series_index: usize,
    #[setters(skip)]
    series_end: bool,
    // index into data.industry
    #[setters(skip)]
    industries: Vec<&'a String>,
    #[setters(skip)]
    industry_index: usize,
    #[setters(skip)]
    industry_end: bool,
    // index into data.country
    #[setters(skip)]
    countries: Vec<&'a String>,
    #[setters(skip)]
    country_index: usize,
    #[setters(skip)]
    country_end: bool,
    // index into year value subset of data.year
    #[setters(skip)]
    years: Vec<&'a String>,
    #[setters(skip)]
    year_index: usize,
    #[setters(skip)]
    year_end: bool,
}

impl<'a> MneIterator<'a> {
    pub fn new(
        data: &'a Mne,
        series_options: SelectionKind,
        industry_options: SelectionKind,
        country_options: SelectionKind,
        year_options: SelectionKind,
        footnotes: Footnotes,
    ) -> Self {
        // DI needs to come before AMNE
        // so the query stored in app does not retain ownership level or nonbank affiliates
        let mne_kinds = MneKind::iter().rev().collect();
        let mne_index = 0;
        let mne_end = false;
        let ownership_index = 0;
        let ownership_end = false;
        let nonbank_index = 0;
        let nonbank_end = false;
        let doi_index = 0;
        let doi_end = false;
        let class_index = 0;
        let class_end = false;
        let mut years = Vec::new();
        for opt in data.year() {
            match opt.kind() {
                YearKind::Year(_) => {
                    years.push(opt.key());
                }
                _ => {
                    tracing::trace!("Not an individual value.");
                }
            }
        }
        let series_index = 0;
        let series_end = false;
        let mut industries = Vec::new();
        for opt in data.industry() {
            match opt.kind() {
                IntegerKind::All => {}
                IntegerKind::Integer(_) => industries.push(opt.key()),
            }
        }
        let industry_index = 0;
        let industry_end = false;
        let mut countries = Vec::new();
        for opt in data.country() {
            match opt.kind() {
                IntegerKind::All => {}
                IntegerKind::Integer(_) => countries.push(opt.key()),
            }
        }
        let country_index = 0;
        let country_end = false;
        let year_index = 0;
        let year_end = false;
        Self {
            data,
            series_options,
            industry_options,
            country_options,
            year_options,
            footnotes,
            mne_kinds,
            mne_index,
            mne_end,
            ownership_index,
            ownership_end,
            nonbank_index,
            nonbank_end,
            doi_index,
            doi_end,
            class_index,
            class_end,
            series_index,
            series_end,
            industries,
            industry_index,
            industry_end,
            countries,
            country_index,
            country_end,
            years,
            year_index,
            year_end,
        }
    }
}

impl Iterator for MneIterator<'_> {
    type Item = std::collections::BTreeMap<String, String>;

    fn next(&mut self) -> Option<Self::Item> {
        // advance state
        // primary driver year
        // secondary driver series
        // tertiary driver industry
        // next driver country
        // next driver classification
        // next driver direction of investment
        // next driver nonbank affiliates
        // ultimate driver ownership level
        if self.year_end {
            // no more year values in self.years
            // reset index and flag (necessary or doesn't hurt)
            self.year_index = 0;
            self.year_end = false;
            // advance series
            match self.series_options {
                SelectionKind::All => self.series_end = true,
                SelectionKind::Individual => {
                    if self.series_index < self.data.series_id.len() - 1 {
                        tracing::trace!("Advancing series index.");
                        self.series_index += 1;
                    } else {
                        tracing::trace!("Ending series index.");
                        self.series_end = true;
                    }
                }
                // TODO: unimplemented
                SelectionKind::Multiple => {}
            }
        }

        // set to true when out of years for a given option combo
        if self.series_end {
            // no more series in self.data.series_id
            self.series_index = 0;
            self.series_end = false;
            // advance industry
            match self.industry_options {
                SelectionKind::All => self.industry_end = true,
                SelectionKind::Individual => {
                    if self.industry_index < self.industries.len() - 1 {
                        tracing::trace!("Advancing industy index.");
                        self.industry_index += 1;
                    } else {
                        tracing::trace!("Ending industy index.");
                        self.industry_end = true;
                    }
                }
                // TODO: unimplemented
                SelectionKind::Multiple => {}
            }
        }

        // set to true when out of industries for a given option combo
        if self.industry_end {
            // no more industries in self.data.industry
            self.industry_index = 0;
            self.industry_end = false;
            // advance country
            match self.country_options {
                SelectionKind::All => self.country_end = true,
                SelectionKind::Individual => {
                    if self.country_index < self.countries.len() - 1 {
                        tracing::trace!("Advancing country index.");
                        self.country_index += 1;
                    } else {
                        tracing::trace!("Ending country index.");
                        self.country_end = true;
                    }
                }
                // TODO: unimplemented
                SelectionKind::Multiple => {}
            }
        }

        // set to true when out of countries for a given option combo
        if self.country_end {
            // no more countries in self.data.country
            self.country_index = 0;
            self.country_end = false;
            // advance classification
            if self.class_index < self.data.classification.len() - 1 {
                tracing::trace!("Advancing class index.");
                self.class_index += 1;
            } else {
                tracing::trace!("Ending class index.");
                self.class_end = true;
            }
        }

        if self.class_end {
            self.class_index = 0;
            self.class_end = false;
            let doi_cap = match self.mne_kinds[self.mne_index] {
                MneKind::Amne => self.data.direction_of_investment.len() - 1,
                // DI only has inward and outward directions, indexed at 0 and 1
                MneKind::Di => 1,
            };
            if self.doi_index < doi_cap {
                tracing::trace!("Advancing doi index.");
                self.doi_index += 1;
            } else {
                tracing::trace!("Ending doi index.");
                self.doi_end = true;
            }
        }

        if self.doi_end {
            self.doi_index = 0;
            self.doi_end = false;
            match self.mne_kinds[self.mne_index] {
                MneKind::Amne => {
                    if self.nonbank_index < self.data.nonbank_affiliates_only.len() - 1 {
                        tracing::trace!("Advancing nonbank index.");
                        self.nonbank_index += 1;
                    } else {
                        tracing::trace!("Ending nonbank index.");
                        self.nonbank_end = true;
                    }
                }
                MneKind::Di => {
                    if self.mne_index < self.mne_kinds.len() - 1 {
                        tracing::trace!("Advancing mne index.");
                        self.mne_index += 1;
                    } else {
                        tracing::trace!("Ending mne index.");
                        self.mne_end = true;
                    }
                }
            }
        }

        if self.nonbank_end {
            self.nonbank_index = 0;
            self.nonbank_end = false;
            if self.ownership_index < self.data.ownership_level.len() - 1 {
                tracing::trace!("Advancing ownership index.");
                self.ownership_index += 1;
            } else {
                tracing::trace!("Ending ownership index.");
                self.ownership_end = true;
            }
        }

        if self.ownership_end {
            self.ownership_index = 0;
            self.ownership_end = false;
            if self.mne_index < self.mne_kinds.len() - 1 {
                tracing::trace!("Advancing mne index.");
                self.mne_index += 1;
            } else {
                tracing::trace!("Ending ownership index.");
                self.mne_end = true;
            }
        }

        if self.mne_end {
            return None;
        }

        // empty parameters dictionary
        let mut params = std::collections::BTreeMap::new();
        // set footnotes
        let (key, value) = self.footnotes.params();
        params.insert(key, value);

        // set direction of investment
        let key = ParameterName::DirectionOfInvestment.to_string();
        let value = self.data.direction_of_investment[self.doi_index]
            .key()
            .to_string();
        let doi = value.clone();
        params.insert(key, value);

        if self.mne_kinds[self.mne_index] == MneKind::Amne {
            // set ownership level
            let key = ParameterName::OwnershipLevel.to_string();
            let mut value = self.data.ownership_level[self.ownership_index]
                .key()
                .to_string();
            // if doi is parent, then ownership level must be 1
            if &doi == "parent" && &value == "0" {
                self.ownership_index = 1;
                value = "1".to_string();
            }
            params.insert(key, value);

            // set nonbank affiliates only
            let key = ParameterName::NonbankAffiliatesOnly.to_string();
            let value = self.data.nonbank_affiliates_only[self.nonbank_index]
                .key()
                .to_string();
            params.insert(key, value);
        } else {
            tracing::trace!("Ownership and nonbank not set.");
        }

        // set classification
        let key = ParameterName::Classification.to_string();
        let value = self.data.classification[self.class_index].key().to_string();
        params.insert(key, value);

        // set series id
        let key = ParameterName::SeriesID.to_string();
        let value = match self.series_options {
            SelectionKind::All => "all".to_string(),
            SelectionKind::Individual => self.data.series_id[self.series_index].value().to_string(),
            SelectionKind::Multiple => "all".to_string(),
        };
        params.insert(key, value);

        // set industry
        let key = ParameterName::Industry.to_string();
        let value = match self.industry_options {
            SelectionKind::All => "all",
            SelectionKind::Individual => self.industries[self.industry_index],
            SelectionKind::Multiple => "all",
        };
        params.insert(key, value.to_string());

        // set country
        let key = ParameterName::Country.to_string();
        let value = match self.country_options {
            SelectionKind::All => "all",
            SelectionKind::Individual => self.countries[self.country_index],
            SelectionKind::Multiple => "all",
        };
        params.insert(key, value.to_string());

        // set year
        let key = ParameterName::Year.to_string();
        let value = match self.year_options {
            SelectionKind::All => {
                self.year_end = true;
                "all"
            }
            SelectionKind::Individual => {
                // Pull current year from self.years by self.year_index
                let year = self.years[self.year_index];
                // Check if more years are available
                if self.year_index == self.years.len() - 1 {
                    // No more years, move to next table
                    self.year_end = true;
                } else {
                    // Increment year index
                    self.year_index += 1;
                }
                year
            }
            SelectionKind::Multiple => {
                self.year_end = true;
                "all"
            }
        };
        params.insert(key, value.to_string());
        Some(params)
    }
}
