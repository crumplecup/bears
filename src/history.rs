use crate::{
    App, BeaErr, Data, Dataset, EnvError, Event, IoError, Mode, Queue, ResultStatus, SerdeJson,
};

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
    pub fn from_env() -> Result<Self, BeaErr> {
        dotenvy::dotenv().ok();
        let path = EnvError::from_env("BEA_DATA")?;
        let path = std::path::PathBuf::from(path);
        let path = path.join("history");
        Self::try_from(&path)
    }

    pub fn contains(&self, path: &std::path::PathBuf) -> bool {
        self.contains_key(path)
    }

    pub fn is_success(&self, app: &App) -> Result<Option<bool>, BeaErr> {
        let path = app.destination(false)?;
        if let Some(event) = self.get(&path) {
            match event.status() {
                ResultStatus::Success(_, _) => Ok(Some(true)),
                _ => Ok(Some(false)),
            }
        } else {
            Ok(None)
        }
    }

    pub fn is_error(&self, app: &App) -> Result<Option<bool>, BeaErr> {
        let path = app.destination(false)?;
        if let Some(event) = self.get(&path) {
            match event.status() {
                ResultStatus::Error(_) => Ok(Some(true)),
                _ => Ok(Some(false)),
            }
        } else {
            Ok(None)
        }
    }

    /// The `with_mode` method filters entries by the provided `mode`, returning the subset of
    /// entries where the mode of the event matches `mode`.
    pub fn with_mode(&self, mode: Mode) -> Self {
        let mut values = self.values().cloned().collect::<Vec<Event>>();
        values.retain(|event| *event.mode() == mode);
        Self::from(values)
    }

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
    pub fn buckets(&self) -> usize {
        // minimum chunk size should be one, so we call ceil to round up.
        // How many chunks do we need?
        // convert to f64 to take a ratio
        (self.len() as f64 / 100.).ceil() as usize
    }

    /// Based on the `Queue` size, assign an index representing a bucket to requests such that the
    /// size of the cumulative requests in the bucket is less likely to exceed maximum download
    /// size limit set by the BEA API server.
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
    pub fn by_size(&self) -> Vec<Event> {
        // clone to a mutable vector to apply sorting by event field
        let mut values = self.values().cloned().collect::<Vec<Event>>();
        // sort by event length (file size)
        // should not be called on paths with unknown file size (errors)
        // but defaults to zero if done so.
        values.sort_by_key(|event| event.length().unwrap_or(0));
        values
    }

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
        let key = "BEA_DATA".to_string();
        let bea_data = std::env::var(&key).map_err(|e| EnvError::new(key, e))?;
        let path = std::path::PathBuf::from(&bea_data);
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
    pub fn with_queue(&self, queue: &Queue) -> Queue {
        let queue = self
            .iter()
            .map(|event| {
                let mut q = queue.clone();
                q.with_event(event);
                q.to_vec()
            })
            .fold(Vec::new(), |mut result, apps| {
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
    pub fn with_queue(&self, queue: &Queue) -> Vec<Queue> {
        self.iter()
            .map(|chunk| chunk.with_queue(queue))
            .collect::<Vec<Queue>>()
    }

    /// Batches requests into bins of max requests per minute (100), averaged over expected file size.
    /// Calls download on each batch in sequence.
    pub async fn download(&self, queue: &Queue, overwrite: bool) -> Result<(), BeaErr> {
        for queue in self.with_queue(queue) {
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
    pub async fn load(&self, queue: &Queue) -> Result<Vec<Data>, BeaErr> {
        let mut data = Vec::new();
        for queue in self.with_queue(queue) {
            let dataset = queue.load().await?;
            let dataset = dataset.lock().await;
            data.extend(dataset.clone())
        }
        Ok(data)
    }
}

impl From<&History> for Chunks {
    fn from(value: &History) -> Self {
        let values = value.by_size();
        let chunk_index = value.chunk_index();
        let buckets = value.buckets();
        let mut chunks = Vec::new();
        for i in 0..buckets {
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
        }
        Self::from(chunks)
    }
}
