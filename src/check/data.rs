use crate::{trace_init, BeaErr, Dataset, History, Mode, Naics};

/// Pings the BEA API.
#[tracing::instrument]
pub async fn data_to_json() -> Result<(), BeaErr> {
    trace_init()?;
    let datasets = vec![Dataset::GDPbyIndustry];
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
        // let history = History::try_from((dataset, Mode::Download))?;
        // queue.exclude(&history)?;
        // queue.errors(&history, false)?;
        // tracing::info!("Queue is length {}", queue.len());
        // tracing::info!("{queue:#?}");
        queue.download(false).await?;
        // counter.download(false).await?;
    }
    Ok(())
}

#[tracing::instrument(skip_all)]
pub async fn data_from_json() -> Result<(), BeaErr> {
    trace_init()?;
    let datasets = vec![Dataset::Mne];
    for dataset in datasets {
        // let queue = dataset.queue()?;
        let mut queue = dataset.queue()?;
        // queue.retain(|app| &app.query()["Country"] == "400");
        // queue.retain(|app| &app.query()["DirectionOfInvestment"] == "outward");
        // queue.retain(|app| &app.query()["Classification"] == "Industry");
        // queue.retain(|app| &app.query()["ShowMillions"] == "N");
        tracing::info!("Queue length: {}", queue.len());
        // queue.dedup();
        // tracing::info!("Queue length: {}", queue.len());

        // A fresh queue has been downloaded, try loading the successes
        // let history = History::try_from((dataset, Mode::Download))?;
        // only download successes in history
        // strict = true set to include no others in queue.
        // queue.successes(&history, true)?;
        // tracing::info!("Files downloaded: {}", queue.len());
        // exclude previously loaded files in load history
        // let history = History::try_from((dataset, Mode::Load))?;
        // queue.exclude(&history)?;
        // tracing::info!("Files left to load: {}", queue.len());

        // The load history contains errors, try them again.
        let history = History::try_from((dataset, Mode::Load))?;
        queue.errors(&history, true)?;
        tracing::info!("Files to retry: {}", queue.len());

        // let path = "/home/erik/bea/history/history_MNE_Errors.log";
        // let path = std::path::PathBuf::from(path);
        // let history = History::try_from(&path)?;
        // queue.errors(&history, true)?;
        tracing::info!("Queue length: {}", queue.len());
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

#[tracing::instrument(skip_all)]
pub async fn datasets_download() -> Result<(), BeaErr> {
    trace_init()?;
    // let datasets = vec![
    //     Dataset::Nipa,
    //     Dataset::NIUnderlyingDetail,
    //     Dataset::FixedAssets,
    //     Dataset::Mne,
    // ];
    let datasets = vec![Dataset::Mne];
    for dataset in datasets {
        dataset.download_with_history().await?;
    }
    Ok(())
}

#[tracing::instrument(skip_all)]
pub async fn datasets_initial_load() -> Result<(), BeaErr> {
    trace_init()?;
    let datasets = vec![Dataset::NIUnderlyingDetail];
    for dataset in datasets {
        let result = dataset.initial_load(None).await?;
        tracing::info!("{} datasets loaded.", result.len());
    }
    Ok(())
}

#[tracing::instrument(skip_all)]
pub async fn datasets_initial_load_continued() -> Result<(), BeaErr> {
    trace_init()?;
    let datasets = vec![Dataset::Mne];
    for dataset in datasets {
        let loads = History::try_from((dataset, Mode::Load))?;
        let result = dataset.initial_load(Some(&loads)).await?;
        tracing::info!("{} datasets loaded.", result.len());
    }
    Ok(())
}

#[tracing::instrument(skip_all)]
pub async fn datasets_retry_load() -> Result<(), BeaErr> {
    trace_init()?;
    let datasets = vec![Dataset::Mne];
    for dataset in datasets {
        let result = dataset.retry_load().await?;
        tracing::info!("{} datasets loaded.", result.len());
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
