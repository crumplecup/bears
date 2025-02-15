use crate::{trace_init, BeaErr, Dataset, History, Mode, Naics};

/// Pings the BEA API.
#[tracing::instrument]
pub async fn data_to_json() -> Result<(), BeaErr> {
    trace_init()?;
    let datasets = vec![Dataset::Mne];
    for dataset in datasets {
        // let queue = dataset.queue()?;
        let mut queue = dataset.queue()?;
        tracing::info!("Queue is length {}", queue.len());
        //
        // queue.retain(|app| {
        //     app.query()
        //         .get(ParameterName::NonbankAffiliatesOnly.to_string().as_str())
        //         == Some(&"1".to_string())
        // });
        // queue.retain(|app| &app.query()["DirectionOfInvestment"] == "outward");
        // queue.retain(|app| {
        //     app.query()
        //         .get(&ParameterName::NonbankAffiliatesOnly.to_string())
        //         .is_none()
        // });
        // queue.retain(|app| app.query().get("OwnershipLevel").is_none());
        // queue.retain(|app| &app.query()["Country"] == "000");
        // queue.retain(|app| &app.query()["Classification"] == "Country");
        // for app in queue.iter().take(10) {
        //     tracing::info!("{app:#?}");
        // }
        queue.dedup();
        tracing::info!("Queue is length {}", queue.len());
        // let mut counter = queue.clone();
        // counter.active_subset(true)?;
        // tracing::info!("Counter is length {}", counter.len());

        // queue.active_subset(false)?;
        let history = History::try_from((dataset, Mode::Download))?;
        queue.exclude(&history)?;
        // queue.errors(&history, false)?;
        tracing::info!("Queue is length {}", queue.len());
        // tracing::info!("{queue:#?}");
        queue.download(false).await?;
        // counter.download(false).await?;
    }
    Ok(())
}

#[tracing::instrument(skip_all)]
pub async fn data_from_json() -> Result<(), BeaErr> {
    trace_init()?;
    let datasets = vec![Dataset::FixedAssets];
    for dataset in datasets {
        let queue = dataset.queue()?;
        // let mut queue = dataset.queue()?;
        // queue.retain(|app| &app.query()["Country"] == "000");
        // queue.retain(|app| &app.query()["DirectionOfInvestment"] == "outward");
        // queue.retain(|app| &app.query()["Classification"] == "Country");
        // queue.retain(|app| &app.query()["ShowMillions"] == "N");
        // tracing::info!("Queue length: {}", queue.len());
        // queue.dedup();
        tracing::info!("Queue length: {}", queue.len());
        // let history = History::try_from((dataset, Mode::Load))?;
        // // strict is true means only download errors included in the event history
        // queue.errors(&history, true)?;
        // tracing::info!("Queue length: {}", queue.len());
        //
        // let queues = history.iter().with_queue(&queue);
        // for q in queues {
        //     let data = q.load().await?;
        //     let data = data.lock().await;
        //     tracing::info!("{} datasets loaded.", data.len());
        // }
        // tracing::info!("Full queue downloaded.");
        // queue.successes(false)?;
        // tracing::info!("Queue length: {}", queue.len());
        let data = queue.load().await?;
        let data = data.lock().await;
        tracing::info!("{} datasets loaded.", data.len());
    }
    Ok(())
}

#[tracing::instrument]
pub fn naics() -> Result<(), BeaErr> {
    trace_init()?;
    let path = "data/naics_codes.csv";
    let naics = Naics::from_csv(path)?;
    tracing::info!("{naics:?}");
    Ok(())
}

#[tracing::instrument]
pub fn download_history() -> Result<(), BeaErr> {
    trace_init()?;
    let history = History::from_env()?;
    tracing::info!("History: {history:#?}");
    Ok(())
}
