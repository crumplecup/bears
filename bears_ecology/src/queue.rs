use crate::{file_size, App, Event, History, ResultStatus, SizeEvent, Tracker};
use bears_species::{BeaErr, Data};
use indicatif::ProgressIterator;
use rayon::prelude::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
    derive_new::new,
)]
pub struct Queue(Vec<App>);

impl Queue {
    #[tracing::instrument(skip_all)]
    /// Subset of queue that is not contained within the `history`.
    pub fn exclude(&mut self, history: &History) -> Result<(), BeaErr> {
        history.summary();
        self.retain(|app| !history.contains_key(&app.destination(false).unwrap()));
        Ok(())
    }

    #[tracing::instrument(skip_all)]
    /// Subset of queue that contains a success status.
    pub fn successes(&mut self, history: &History, strict: bool) -> Result<(), BeaErr> {
        history.summary();
        self.retain(|app| history.is_success(app).unwrap_or(None).unwrap_or(!strict));
        Ok(())
    }

    #[tracing::instrument(skip_all)]
    /// Subset of queue that contains an error status.
    pub fn errors(&mut self, history: &History, strict: bool) -> Result<(), BeaErr> {
        self.retain(|app| history.is_error(app).unwrap_or(None).unwrap_or(!strict));
        Ok(())
    }

    #[tracing::instrument(skip_all)]
    pub fn active_subset(&mut self, strict: bool) -> Result<(), BeaErr> {
        let history = History::from_env()?;
        history.summary();
        self.retain(|app| match history.is_error(app) {
            Ok(opt) => match opt {
                Some(val) => !val,
                None => !strict,
            },
            Err(source) => {
                tracing::error!("{source}");
                false
            }
        });
        Ok(())
    }

    /// Filters the `Queue` for members with a destination path matching the [`Event`] path in
    /// 'event'.
    #[tracing::instrument(skip_all)]
    pub fn with_event(&mut self, event: &Event) {
        // Since we are not creating new directories, constructing the destination path should
        // never panic.
        self.retain(|app| app.destination(false).unwrap() == *event.path());
        // Update the `size_hint` field of the app to the event length.
        self.iter_mut()
            .map(|app| app.with_size_hint(*event.length()))
            .for_each(drop);
    }

    /// Filters the `Queue` in parallel for members with a destination path matching the [`Event`] path in
    /// 'event'.
    #[tracing::instrument(skip_all)]
    pub fn with_event_par(&mut self, event: &Event) {
        // Since we are not creating new directories, constructing the destination path should
        // never panic.
        self.retain(|app| app.destination(false).unwrap() == *event.path());
        // Update the `size_hint` field of the app to the event length.
        self.par_iter_mut()
            .map(|app| app.with_size_hint(*event.length()))
            .for_each(drop);
    }

    /// Filters the `Queue` for members with a destination path matching the [`Event`] path in
    /// 'event'.
    #[tracing::instrument(skip_all)]
    pub fn with_event_ref(&mut self, event: &Event) {
        let mut paths = self
            .iter()
            .enumerate()
            .map(|(i, q)| (i, q.destination(false).unwrap()))
            .collect::<Vec<(usize, std::path::PathBuf)>>();
        paths.sort_by_key(|p| p.1.clone());
        let ids = paths.iter().map(|x| x.0).collect::<Vec<usize>>();
        let paths = paths
            .into_iter()
            .map(|x| x.1)
            .collect::<Vec<std::path::PathBuf>>();
        let index = paths.binary_search(event.path()).unwrap();
        // Update the `size_hint` field of the app to the event length.
        self[ids[index]].with_size_hint(*event.length());
    }

    #[tracing::instrument(skip_all)]
    pub fn with_events(&self, events: &Vec<Event>) -> Self {
        let mut paths = self
            .iter()
            .enumerate()
            .map(|(i, q)| (i, q.destination(false).unwrap()))
            .collect::<Vec<(usize, std::path::PathBuf)>>();
        // perform an initial sort to enable binary search
        paths.sort_by_key(|p| p.1.clone());
        // retain the indexes of the app associated with each path
        let ids = paths.iter().map(|x| x.0).collect::<Vec<usize>>();
        // split the paths off into their own vec for search
        let paths = paths
            .into_iter()
            .map(|x| x.1)
            .collect::<Vec<std::path::PathBuf>>();
        // apps matching events will go into this vector
        let mut apps = Vec::new();
        // match apps to events
        for event in events {
            // get the index of the matching path
            let index = paths.binary_search(event.path()).unwrap();
            // get the app associated with the path index
            let mut app = self[ids[index]].clone();
            // Update the `size_hint` field of the app to the event length.
            app.with_size_hint(*event.length());
            // add to results vector
            apps.push(app);
        }
        Self::new(apps)
    }

    #[tracing::instrument(skip_all)]
    pub async fn download(&self, overwrite: bool) -> Result<(), BeaErr> {
        let tracker = std::sync::Arc::new(tokio::sync::Mutex::new(Tracker::default()));
        let (tx, mut rx) = tokio::sync::mpsc::channel(29);
        let download = self.downloader(tx, tracker.clone(), overwrite);
        let listen = Self::listen(&mut rx, tracker.clone(), Mode::Download);
        let (download_res, listen_res) = tokio::join!(download, listen);
        // listen_res?;
        if let Err(blame) = download_res {
            tracing::warn!("Problem with call: {blame}");
        }
        if let Err(blame) = listen_res {
            tracing::warn!("Probelm with tracking: {blame}");
            return Err(blame);
        }
        Ok(())
    }

    #[tracing::instrument(skip_all)]
    pub async fn listen(
        rx: &mut tokio::sync::mpsc::Receiver<ResultStatus>,
        tracker: std::sync::Arc<tokio::sync::Mutex<Tracker>>,
        mode: Mode,
    ) -> Result<(), BeaErr> {
        while let Some(status) = rx.recv().await {
            match status {
                ResultStatus::Success(_, _) | ResultStatus::Error(_) => {
                    let mut tracker = tracker.lock().await;
                    tracker.update_status(status, mode);
                    tracing::trace!("Update: {status}.");
                }
                ResultStatus::Pass(_) | ResultStatus::Pending => {}
                ResultStatus::Abort => {
                    tracing::info!("Abort detected.");
                    // let error = RateLimit::new(
                    //     "RequestsExceeded".to_string(),
                    //     line!(),
                    //     file!().to_string(),
                    // );
                    // return Err(error.into());
                    panic!("Limit rate exceeded.")
                }
            }
        }
        Ok(())
    }

    /// The download will move to idle if the user has exceeded a rate limiting condition from BEA:
    ///
    /// * More than 100 requests per minute.
    /// * More than 100MB of data volume retrieved per minute.
    /// * More than 30 errors per minute.
    #[tracing::instrument(skip_all)]
    pub async fn downloader(
        &self,
        tx: tokio::sync::mpsc::Sender<ResultStatus>,
        tracker: std::sync::Arc<tokio::sync::Mutex<Tracker>>,
        overwrite: bool,
    ) -> Result<Vec<tokio::task::JoinHandle<()>>, BeaErr> {
        let mut futures = Vec::new();
        for app in self.iter() {
            let app = app.clone();
            let tx = tx.clone();
            let path = app.destination(false)?;
            let path_check = path.exists();
            // tracing::info!("Exists: {path_check} - {path:?}");
            if !path_check || overwrite {
                let event = Event::new(&path, Mode::Download);
                let id = *event.id();
                let mut slack;
                let next_size = app.size_hint().unwrap_or(0);
                tracing::trace!("Next size is {}", bytesize::ByteSize::b(next_size));
                let mut size_available;
                {
                    // Scoped to release lock before entering while loop
                    let mut tracker = tracker.lock().await;
                    slack = tracker.check_slack();
                    size_available = tracker.size_available();
                }
                while slack == 0 || (size_available <= next_size && next_size < 100_000_000) {
                    tracing::trace!("Limiting call rate.");
                    {
                        // Scoped to release lock before checking for slack
                        let tracker = tracker.lock().await;
                        tracker.wait().await;
                    }
                    {
                        // Scoped to release lock before leaving the loop
                        let mut tracker = tracker.lock().await;
                        slack = tracker.check_slack();
                        size_available = tracker.size_available();
                        tracing::trace!("Size available: {size_available}");
                    }
                }
                {
                    let mut tracker = tracker.lock().await;
                    // If the size is known, add it the size events
                    if let Some(size) = app.size_hint() {
                        tracker.size.push(SizeEvent::new(*size));
                    }
                    tracker.calls.push(event);
                }

                let fut = tokio::spawn(async move {
                    let mut result = ResultStatus::Pass(id);
                    tracing::trace!("Calling download for {path:#?}");
                    if let Ok(status) = app.download(id).await {
                        result = status;
                    } else {
                        tracing::error!("Request failure.");
                    }
                    match tx.send(result).await {
                        Ok(_) => {}
                        Err(source) => {
                            tracing::error!("{source}");
                        }
                    }
                });
                futures.push(fut);
            }
        }
        Ok(futures)
    }

    #[tracing::instrument(skip_all)]
    pub async fn loader(
        &self,
        data: std::sync::Arc<tokio::sync::Mutex<Vec<Data>>>,
        tx: tokio::sync::mpsc::Sender<ResultStatus>,
        tracker: std::sync::Arc<tokio::sync::Mutex<Tracker>>,
    ) -> Result<Vec<tokio::task::JoinHandle<()>>, BeaErr> {
        let style = indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {'Loading files in queue.'}",
        )
        .unwrap();
        let mut handles = Vec::new();
        for app in self.iter().progress_with_style(style) {
            let path = app.destination(false)?;
            if path.exists() {
                let event = Event::new(&path, Mode::Load);
                let id = *event.id();
                {
                    // Log load event
                    let mut tracker = tracker.lock().await;
                    tracker.calls.push(event);
                }
                // Clone arc references to pass to spawn
                let data = data.clone();
                let tx = tx.clone();
                let app = app.clone();
                let handle = tokio::spawn(async move {
                    tracing::trace!("Calling load for {path:#?}");
                    let status;
                    match app.load() {
                        Ok(response) => {
                            if let Some(dataset) = response.data() {
                                {
                                    // Scoped to release lock immediately after pushing update.
                                    let mut data = data.lock().await;
                                    data.push(dataset);
                                    tracing::trace!("Dataset loaded.");
                                }
                                let size = file_size(path).unwrap_or(0);
                                status = ResultStatus::Success(id, size);
                            } else {
                                tracing::error!("Load failure: Not Data type.");
                                status = ResultStatus::Error(id);
                            }
                        }
                        Err(source) => {
                            tracing::error!("{source}");
                            status = ResultStatus::Error(id);
                        }
                    }
                    match tx.send(status).await {
                        Ok(_) => {}
                        Err(source) => {
                            tracing::error!("{source}");
                        }
                    }
                });
                handles.push(handle);
            }
        }
        Ok(handles)
    }

    #[tracing::instrument(skip_all)]
    pub async fn load(&self) -> Result<std::sync::Arc<tokio::sync::Mutex<Vec<Data>>>, BeaErr> {
        let tracker = std::sync::Arc::new(tokio::sync::Mutex::new(Tracker::default()));
        let (tx, mut rx) = tokio::sync::mpsc::channel(100);
        let data = std::sync::Arc::new(tokio::sync::Mutex::new(Vec::new()));
        let load = self.loader(data.clone(), tx, tracker.clone());
        let listen = Self::listen(&mut rx, tracker.clone(), Mode::Load);
        let (load_res, listen_res) = tokio::join!(load, listen);
        if let Err(blame) = load_res {
            tracing::warn!("Problem with load: {blame}");
        }
        if let Err(blame) = listen_res {
            tracing::warn!("Probelm with tracking: {blame}");
            return Err(blame);
        }
        Ok(data)
    }

    #[tracing::instrument(skip_all)]
    pub fn load_par(&self) -> Result<Vec<Data>, BeaErr> {
        let data = self
            .par_iter()
            .map(|app| app.load())
            .filter_map(|r| r.map_err(|_| {}).ok())
            .filter_map(|res| res.data())
            .collect::<Vec<Data>>();
        Ok(data)
    }
}

#[derive(
    Debug,
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
    derive_more::Display,
    derive_more::FromStr,
)]
pub enum Mode {
    Download,
    Load,
}
