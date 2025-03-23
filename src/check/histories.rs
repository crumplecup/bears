use crate::{trace_init, BeaErr, Dataset, History, Mode};

#[tracing::instrument]
/// Prints summary data from the Load [`History`] of the currently implemented datasets.
/// Used for reporting and to assess storage requirements.
pub fn download_summary() -> Result<(), BeaErr> {
    trace_init()?;
    let datasets = vec![
        Dataset::Nipa,
        Dataset::NIUnderlyingDetail,
        Dataset::FixedAssets,
        Dataset::Mne,
        Dataset::GDPbyIndustry,
        Dataset::Ita,
    ];
    for dataset in datasets {
        tracing::info!("Dataset: {dataset}");
        let history = History::try_from((dataset, Mode::Load))?;
        history.summary();
    }

    Ok(())
}
