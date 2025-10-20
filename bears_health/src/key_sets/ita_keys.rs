use bears_ecology::initial_load;
use bears_species::{Bull, Data, Dataset, Ita, ItaData, ParameterName};

use crate::params;
pub struct ItaKeys;

impl ItaKeys {
    #[tracing::instrument]
    fn ita_expected<'a, P: AsRef<std::path::Path> + std::fmt::Debug>(path: P) -> Result<Ita, Bull> {
        let path = path.as_ref().to_owned();
        let data = Ita::try_from(&path)?;
        Ok(data)
    }

    #[tracing::instrument(skip_all)]
    async fn ita_observed() -> Result<ItaData, Bull> {
        let dataset = Dataset::Ita;
        let mut data = Vec::new();
        let obs = initial_load(dataset, None).await?;
        tracing::info!("{} datasets loaded for {dataset}.", obs.len());
        obs.iter()
            .map(|v| {
                if let Data::Ita(values) = v {
                    data.append(&mut values.clone());
                }
            })
            .for_each(drop);

        Ok(ItaData::new(data))
    }

    /// Print the value sets from each struct field in the source data to the BEA_DATA directory.
    #[tracing::instrument]
    pub fn print_expected<P: AsRef<std::path::Path> + std::fmt::Debug>(
        path: P,
    ) -> Result<(), Bull> {
        let path = path.as_ref();
        let dataset = Dataset::Ita;
        let kind = "Expected";
        let data = Self::ita_expected(path)?;
        let name = ParameterName::AreaOrCountry;
        params(&data.aocs(), path, dataset, name, kind)?;
        let name = ParameterName::Frequency;
        params(&data.frequencies(), path, dataset, name, kind)?;
        let name = ParameterName::Indicator;
        params(&data.indicators(), path, dataset, name, kind)?;
        let name = ParameterName::Year;
        params(&data.years(), path, dataset, name, kind)?;
        Ok(())
    }

    /// Prints set members of type struct fields from source data to the `BEA_DATA` directory.
    #[tracing::instrument(skip_all)]
    pub async fn print_observed<P: AsRef<std::path::Path>>(path: P) -> Result<(), Bull> {
        let path = path.as_ref();
        let dataset = Dataset::Ita;
        let kind = "Observed";
        let obs = Self::ita_observed().await?;

        let name = ParameterName::AreaOrCountry;
        params(&obs.aocs(), path, dataset, name, kind)?;
        let name = ParameterName::ClUnit;
        params(&obs.cl_units(), path, dataset, name, kind)?;
        let name = ParameterName::Frequency;
        params(&obs.frequencies(), path, dataset, name, kind)?;
        let name = ParameterName::Indicator;
        params(&obs.indicators(), path, dataset, name, kind)?;
        let name = ParameterName::TimePeriod;
        params(&obs.time_periods(), path, dataset, name, kind)?;
        let name = ParameterName::TimeSeriesCode;
        params(&obs.time_series_codes(), path, dataset, name, kind)?;
        let name = ParameterName::UnitMult;
        params(&obs.unit_multipliers(), path, dataset, name, kind)?;
        let name = ParameterName::Year;
        params(&obs.years(), path, dataset, name, kind)?;
        Ok(())
    }
}
