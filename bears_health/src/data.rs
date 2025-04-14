use bears_ecology::{
    download_with_history, init_queue, initial_download, initial_load, retry_load, trace_init,
    History, Mode, Style,
};
use bears_species::{BeaErr, Dataset, GdpData};

/// Pings the BEA API.
#[tracing::instrument]
pub async fn data_to_json() -> Result<(), BeaErr> {
    trace_init()?;
    let datasets = vec![Dataset::GDPbyIndustry];
    for dataset in datasets {
        // let queue = dataset.queue()?;
        let mut queue = init_queue(dataset)?;
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
        let mut queue = init_queue(dataset)?;
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

/// Specialty function for debugging deserialization errors with the GDPbyIndustry tables.
/// Attempts to load each file in the download history.
/// Move the problematic files into the download history (backing up as needed).
pub fn debug_gdpbyindustry() -> Result<(), BeaErr> {
    trace_init()?;
    let dataset = Dataset::GDPbyIndustry;
    let history = History::try_from((dataset, Mode::Download))?;
    for (path, _) in (*history).iter() {
        let data = GdpData::try_from(path)?;
        tracing::info!("Data loaded: {} records.", data.len());
    }

    Ok(())
}

/// Attempt to download all configurations using the associated iterator for the dataset.
/// Cannot meter by file size, susceptible to exceeding the 100MB per minute rate limit of the BEA
/// server.
#[tracing::instrument]
pub async fn datasets_download_initial() -> Result<(), BeaErr> {
    trace_init()?;
    // let datasets = vec![
    //     Dataset::Nipa,
    //     Dataset::NIUnderlyingDetail,
    //     Dataset::FixedAssets,
    //     Dataset::Mne,
    //     Dataset::GDPbyIndustry,
    // ];
    let datasets = vec![Dataset::Ita];
    for dataset in datasets {
        initial_download(dataset).await?;
    }
    Ok(())
}

/// Download existing files of a known size from the download [`History`].
/// Metered to prevent exceeding the 100MB per minute rate limit set by the BEA server.
#[tracing::instrument]
pub async fn datasets_download_with_history() -> Result<(), BeaErr> {
    trace_init()?;
    let styles = Style::try_new()?;
    let style = styles["queue_download"].clone();
    // let datasets = vec![
    //     Dataset::Nipa,
    //     Dataset::NIUnderlyingDetail,
    //     Dataset::FixedAssets,
    //     Dataset::Mne,
    //     Dataset::GDPbyIndustry,
    // ];
    let datasets = vec![Dataset::FixedAssets];
    for dataset in datasets {
        download_with_history(dataset, style.clone()).await?;
    }
    Ok(())
}

/// Attempts to load all files in the download [`History`], without respect to the load `History`.
#[tracing::instrument(skip_all)]
pub async fn datasets_initial_load() -> Result<(), BeaErr> {
    trace_init()?;
    // let datasets = vec![
    //     Dataset::Nipa,
    //     Dataset::NIUnderlyingDetail,
    //     Dataset::FixedAssets,
    //     Dataset::Mne,
    //     Dataset::GDPbyIndustry,
    // ];
    let datasets = vec![Dataset::FixedAssets];
    for dataset in datasets {
        let result = initial_load(dataset, None).await?;
        tracing::info!("{} datasets loaded.", result.len());
    }
    Ok(())
}

/// Attempts to load all files in the download [`History`] that are not yet in the load `History`.
#[tracing::instrument(skip_all)]
pub async fn datasets_initial_load_continued() -> Result<(), BeaErr> {
    trace_init()?;
    let datasets = vec![
        Dataset::Nipa,
        Dataset::NIUnderlyingDetail,
        Dataset::FixedAssets,
        Dataset::Mne,
    ];
    // let datasets = vec![Dataset::Mne];
    for dataset in datasets {
        let loads = History::try_from((dataset, Mode::Load))?;
        let result = initial_load(dataset, Some(&loads)).await?;
        tracing::info!("{} datasets loaded.", result.len());
    }
    Ok(())
}

/// Attempts to reload errors in the load [`History`].
/// Run on `TRACE` level to gather more data on specific a specific file.
#[tracing::instrument(skip_all)]
pub async fn datasets_retry_load() -> Result<(), BeaErr> {
    trace_init()?;
    let datasets = vec![Dataset::Ita];
    for dataset in datasets {
        let result = retry_load(dataset).await?;
        tracing::info!("{} datasets loaded.", result.len());
    }
    Ok(())
}

#[tracing::instrument]
pub fn download_history() -> Result<(), BeaErr> {
    trace_init()?;
    let history = History::from_env()?;
    tracing::info!("History: {history:#?}");
    Ok(())
}
