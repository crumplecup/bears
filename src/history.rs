use crate::{
    bea_data, App, BeaErr, Data, Dataset, Event, IoError, Mode, Queue, ResultStatus, SerdeJson,
};
use indicatif::{ParallelProgressIterator, ProgressIterator};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

/// The `History` struct contains a log of [`Event`] data stored as values in a `BTreeMap`, using the `Event`
/// path as the key.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
)]
pub struct History(std::collections::BTreeMap<std::path::PathBuf, Event>);

impl History {
    /// History logs live in the `history` folder of the `BEA_DATA` directory.
    /// The `from_env` method is an internal function used to establish the appropriate directory
    /// for writing and reading log files.
    ///
    /// Called by [`Queue::active_subset`].
    #[tracing::instrument]
    pub fn from_env() -> Result<Self, BeaErr> {
        dotenvy::dotenv().ok();
        let path = bea_data()?;
        let path = path.join("history");
        Self::try_from(&path)
    }

    /// The `is_success` method only returns a bool indicating whether the operation at the
    /// destination of `app` was successful if the event is present in `self`, otherwise returning
    /// `None`.
    ///
    /// Called by [`Queue::successes`].
    #[tracing::instrument(skip_all)]
    pub fn is_success(&self, app: &App) -> Result<Option<bool>, BeaErr> {
        // get the path destination associated with app
        let path = app.destination(false)?;
        if let Some(event) = self.get(&path) {
            // path is present in event history
            match event.status() {
                // event was successful, return true
                ResultStatus::Success(_, _) => Ok(Some(true)),
                // event was not successful, return false
                _ => Ok(Some(false)),
            }
        } else {
            // path is not present in event history
            Ok(None)
        }
    }

    /// The `is_error` method only returns a bool indicating whether the operation at the
    /// destination of `app` was a failure if the event is present in `self`, otherwise returning
    /// `None`.
    ///
    /// Called by [`Queue::errors`] and [`Queue::active_subset`].
    #[tracing::instrument(skip_all)]
    pub fn is_error(&self, app: &App) -> Result<Option<bool>, BeaErr> {
        // get the path destination associated with app
        let path = app.destination(false)?;
        if let Some(event) = self.get(&path) {
            // path is present in event history
            match event.status() {
                // event was a failure, return true
                ResultStatus::Error(_) => Ok(Some(true)),
                // event was not a failure, return false
                _ => Ok(Some(false)),
            }
        } else {
            // path is not present in event history
            Ok(None)
        }
    }

    /// The `with_mode` method filters entries by the provided `mode`, returning the subset of
    /// entries where the mode of the event matches `mode`.
    #[tracing::instrument(skip_all)]
    pub fn with_mode(&self, mode: Mode) -> Self {
        let mut values = self.values().cloned().collect::<Vec<Event>>();
        values.retain(|event| *event.mode() == mode);
        Self::from(values)
    }

    /// The `summary` method prints summary statistics of the event history to the console at the
    /// INFO level.  Includes the number of successes and errors, as well as the total size of
    /// downloaded files.
    #[tracing::instrument(skip_all)]
    pub fn summary(&self) {
        let mut success = 0;
        let mut error = 0;
        let mut total_size = 0;
        for value in self.values() {
            match value.status() {
                ResultStatus::Success(_, _) => {
                    success += 1;
                    if let Some(size) = value.length() {
                        total_size += size;
                    }
                }
                ResultStatus::Error(_) => error += 1,
                _ => {}
            }
        }
        tracing::info!("Successes: {success}");
        tracing::info!("Errors: {error}");
        tracing::info!("Total Size: {}", bytesize::ByteSize::b(total_size));
    }

    /// Returns the number of buckets required to sort the `Queue` into sets of 100 requests, the
    /// maximum number of requests per minute allowed by the BEA API server.
    #[tracing::instrument(skip_all)]
    pub fn buckets(&self) -> usize {
        // minimum chunk size should be one, so we call ceil to round up.
        // How many chunks do we need?
        // convert to f64 to take a ratio
        (self.len() as f64 / 100.).ceil() as usize
    }

    /// Based on the `Queue` size, assign an index representing a bucket to requests such that the
    /// size of the cumulative requests in the bucket is less likely to exceed maximum download
    /// size limit set by the BEA API server.
    #[tracing::instrument(skip_all)]
    pub fn chunk_index(&self) -> Vec<usize> {
        let buckets = self.buckets();
        // A vector of indexes for chunks
        // Loop the indexes until there is an index for every event.
        BucketIter::from_len(buckets)
            .take(self.len())
            .collect::<Vec<usize>>()
    }

    /// Clones the `History` and returns a vector of [`Event`] type sorted by the length field (file
    /// size).
    #[tracing::instrument(skip_all)]
    pub fn by_size(&self) -> Vec<Event> {
        // clone to a mutable vector to apply sorting by event field
        let mut values = self.values().cloned().collect::<Vec<Event>>();
        // sort by event length (file size)
        // should not be called on paths with unknown file size (errors)
        // but defaults to zero if done so.
        values.sort_by_key(|event| event.length().unwrap_or(0));
        values
    }

    #[tracing::instrument(skip_all)]
    pub fn iter(&self) -> Chunks {
        Chunks::from(self)
    }
}

impl From<Vec<Event>> for History {
    fn from(value: Vec<Event>) -> Self {
        let tree = value
            .into_iter()
            .map(|event| (event.path().to_owned(), event))
            .collect::<std::collections::BTreeMap<std::path::PathBuf, Event>>();
        Self(tree)
    }
}

impl TryFrom<&std::path::PathBuf> for History {
    type Error = BeaErr;

    fn try_from(path: &std::path::PathBuf) -> Result<Self, Self::Error> {
        let mut events = std::collections::BTreeMap::new();
        let file = std::fs::read_to_string(path)
            .map_err(|e| IoError::new(path.into(), e, line!(), file!().into()))?;
        for line in file.lines() {
            tracing::trace!("String: {line}");
            let json: serde_json::Value = serde_json::from_str(line)
                .map_err(|e| SerdeJson::new(e, line!(), file!().to_string()))?;
            tracing::trace!("Json: {json}");
            let event = Event::try_from(&json)?;
            events.insert(event.path().clone(), event);
        }
        Ok(Self(events))
    }
}

impl TryFrom<(Dataset, Mode)> for History {
    type Error = BeaErr;

    fn try_from(ctx: (Dataset, Mode)) -> Result<Self, Self::Error> {
        dotenvy::dotenv().ok();
        let (dataset, mode) = ctx;
        let path = bea_data()?;
        let path = path.join("history");
        let path = path.join(format!("history_{dataset}_{mode}.log"));
        Self::try_from(&path)
    }
}

pub struct BucketIter {
    start: usize,
    end: usize,
    current: usize,
}

impl BucketIter {
    #[tracing::instrument(skip_all)]
    pub fn from_len(length: usize) -> Self {
        Self {
            start: 0,
            end: length,
            current: 0,
        }
    }
}

impl Iterator for BucketIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current.cmp(&self.end) {
            std::cmp::Ordering::Less => {
                let value = self.current;
                self.current += 1;
                Some(value)
            }
            std::cmp::Ordering::Equal => {
                let value = self.current;
                self.current = self.start;
                Some(value)
            }
            std::cmp::Ordering::Greater => None,
        }
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
    derive_more::From,
)]
#[from(Vec<Event>)]
pub struct Chunk(Vec<Event>);

impl Chunk {
    #[tracing::instrument(skip_all)]
    pub fn with_queue(&self, queue: &Queue) -> Queue {
        let style = indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {'Matching event history to queue.'}",
        )
        .unwrap();
        let queue = self
            .iter()
            .map(|event| {
                let mut q = queue.clone();
                q.with_event(event);
                q.to_vec()
            })
            .progress_with_style(style)
            .fold(Vec::new(), |mut result, apps| {
                result.extend(apps);
                result
            });
        Queue::new(queue)
    }

    #[tracing::instrument(skip_all)]
    pub fn with_queue_par(&self, queue: &Queue) -> Queue {
        let style = indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {'Matching event history to queue.'}",
        )
        .unwrap();
        let queue = self
            .par_iter()
            .map(|event| {
                let mut q = queue.clone();
                q.with_event(event);
                q.to_vec()
            })
            .progress_with_style(style)
            .collect::<Vec<Vec<App>>>();
        let queue = queue.into_iter().fold(Vec::new(), |mut result, apps| {
            result.extend(apps);
            result
        });
        Queue::new(queue)
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
    derive_more::From,
)]
#[from(Vec<Chunk>)]
pub struct Chunks(Vec<Chunk>);

impl Chunks {
    /// Constructs a request queue from a download history.
    /// Includes apps in queue where the destination matches the event path.
    #[tracing::instrument(skip_all)]
    pub fn with_queue_single(&self, queue: &Queue) -> Vec<Queue> {
        let style = indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {'Matching event history to chunks.'}",
        )
        .unwrap();
        self.iter()
            .map(|chunk| chunk.with_queue(queue))
            .progress_with_style(style)
            .collect::<Vec<Queue>>()
    }

    /// Constructs a request queue from a download history.
    /// Includes apps in queue where the destination matches the event path.
    #[tracing::instrument(skip_all)]
    pub fn with_queue(&self, queue: &Queue) -> Vec<Queue> {
        let style = indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {'Matching event history to chunks.'}",
        )
        .unwrap();
        self.iter()
            .map(|chunk| queue.with_events(chunk))
            .progress_with_style(style)
            .collect::<Vec<Queue>>()
    }

    /// Constructs a request queue from a download history.
    /// Includes apps in queue where the destination matches the event path.
    #[tracing::instrument(skip_all)]
    pub fn with_queue_par(&self, queue: &Queue) -> Vec<Queue> {
        let style = indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {'Matching event history to chunks.'}",
        )
        .unwrap();
        self.par_iter()
            .map(|chunk| queue.with_events(chunk))
            .progress_with_style(style)
            .collect::<Vec<Queue>>()
    }

    /// Batches requests into bins of max requests per minute (100), averaged over expected file size.
    /// Calls download on each batch in sequence.
    #[tracing::instrument(skip_all)]
    pub async fn download(&self, queue: &Queue, overwrite: bool) -> Result<(), BeaErr> {
        let queues = self.with_queue_par(queue);
        tracing::info!("Chunks to download: {}.", queues.len());
        for (i, queue) in queues.iter().enumerate() {
            tracing::info!("Downloading chunk {i}.");
            queue.download(overwrite).await?;
        }
        Ok(())
    }

    /// Constructs a request queue from a download history.
    /// Batches requests into bins of max requests per minute (100), averaged over expected file size.
    /// Here the batching is unnecessary, but we still want the queue to build from the event
    /// history.
    /// Calls load on each batch in sequence.
    /// Gathers the results into a vector to return to the user.
    #[tracing::instrument(skip_all)]
    pub async fn load(&self, queue: &Queue) -> Result<Vec<Data>, BeaErr> {
        let mut data = Vec::new();
        for queue in self.with_queue_single(queue) {
            let dataset = queue.load().await?;
            let dataset = dataset.lock().await;
            data.extend(dataset.clone())
        }
        Ok(data)
    }
}

impl From<&History> for Chunks {
    fn from(value: &History) -> Self {
        let style = indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {'Chunking history.'}",
        )
        .unwrap();
        let values = value.by_size();
        let chunk_index = value.chunk_index();
        let buckets = value.buckets();
        let bar = indicatif::ProgressBar::new(buckets as u64);
        let bar = indicatif::ProgressBar::with_style(bar, style);
        let mut chunks = Vec::new();
        for i in 0..=buckets {
            let events = values.clone();
            let mut events = events
                .into_iter()
                .zip(chunk_index.clone().into_iter())
                .collect::<Vec<(Event, usize)>>();
            events.retain(|(_, index)| *index == i);
            let events = events
                .into_iter()
                .map(|(event, _)| event)
                .collect::<Vec<Event>>();
            let chunk = Chunk::from(events);
            chunks.push(chunk);
            bar.inc(1);
        }
        bar.finish_and_clear();
        Self::from(chunks)
    }
}
