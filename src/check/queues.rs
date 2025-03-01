use crate::{trace_init, BeaErr, Dataset};

#[tracing::instrument(skip_all)]
pub async fn inspect_queues() -> Result<(), BeaErr> {
    trace_init()?;
    // let datasets = vec![
    //     Dataset::Nipa,
    //     Dataset::NIUnderlyingDetail,
    //     Dataset::FixedAssets,
    //     Dataset::Mne,
    // ];
    let datasets = vec![Dataset::NIUnderlyingDetail];
    for dataset in datasets {
        let queue = dataset.queue()?;
        tracing::info!("Queue length: {}", queue.len());
    }
    Ok(())
}
