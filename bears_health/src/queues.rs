use bears_ecology::{init_queue, trace_init};
use bears_species::{BeaErr, Dataset};

#[tracing::instrument(skip_all)]
pub async fn inspect_queues() -> Result<(), BeaErr> {
    trace_init()?;
    // let datasets = vec![
    //     Dataset::Nipa,
    //     Dataset::NIUnderlyingDetail,
    //     Dataset::FixedAssets,
    //     Dataset::Mne,
    // ];
    let datasets = vec![Dataset::UnderlyingGDPbyIndustry];
    for dataset in datasets {
        let queue = init_queue(dataset)?;
        tracing::info!("Queue length: {}", queue.len());
    }
    Ok(())
}
