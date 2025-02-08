use crate::{trace_init, BeaErr, Dataset, History, Naics};

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

        queue.active_subset(false)?;
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
    let history = History::from_file()?;
    let datasets = vec![Dataset::Mne];
    for dataset in datasets {
        // let queue = dataset.queue()?;
        let mut queue = dataset.queue()?;
        // queue.retain(|app| &app.query()["Country"] == "000");
        // queue.retain(|app| &app.query()["DirectionOfInvestment"] == "outward");
        // queue.retain(|app| &app.query()["Classification"] == "Country");
        tracing::info!("Queue length: {}", queue.len());
        queue.dedup();
        tracing::info!("Queue length: {}", queue.len());
        queue.exclude(&history)?;
        tracing::info!("Queue length: {}", queue.len());
        // queue.successes(false)?;
        // tracing::info!("Queue length: {}", queue.len());
        // let mut set = std::collections::HashSet::new();
        // let mut dupes = Vec::new();
        // for app in queue.iter() {
        //     if !set.contains(&app) {
        //         set.insert(app);
        //     } else {
        //         dupes.push(app);
        //     }
        // }
        //
        // tracing::info!("Set length: {}", set.len());
        // tracing::info!("Dupes length: {}", dupes.len());
        // tracing::info!("{dupes:#?}");
        // tracing::info!("{queue:#?}");
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
    let history = History::from_file()?;
    tracing::info!("History: {history:#?}");
    Ok(())
}
