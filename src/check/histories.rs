use crate::{trace_init, BeaErr, Dataset, History, Mode};

#[tracing::instrument]
pub async fn download_summary() -> Result<(), BeaErr> {
    trace_init()?;
    let datasets = vec![
        Dataset::Nipa,
        Dataset::NIUnderlyingDetail,
        Dataset::FixedAssets,
    ];
    for dataset in datasets {
        let history = History::try_from((dataset, Mode::Download))?;
        tracing::info!("Dataset: {dataset}");
        history.summary();
    }
    Ok(())
}
