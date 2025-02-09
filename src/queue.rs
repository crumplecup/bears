use crate::{
    file_size, map_to_string, App, BeaErr, Data, DeriveFromStr, History, Jiff, JsonParseError,
    JsonParseErrorKind, KeyMissing, NotObject, ParseInt, ResultStatus,
};
use jiff::ToSpan;
use std::str::FromStr;

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
    /// Subset of queue that contains a success status.
    pub fn exclude(&mut self, history: &History) -> Result<(), BeaErr> {
        history.summary();
        self.retain(|app| !history.contains(&app.destination(false).unwrap()));
        Ok(())
    }

    #[tracing::instrument(skip_all)]
    /// Subset of queue that contains a success status.
    pub fn successes(&mut self, strict: bool) -> Result<(), BeaErr> {
        let history = History::from_env()?;
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
    pub fn with_event(&mut self, event: &Event) {
        // Since we are not creating new directories, constructing the destination path should
        // never panic.
        self.retain(|app| app.destination(false).unwrap() == *event.path());
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
                    tracing::info!("Update: {status}.");
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
                let id = event.id;
                let mut slack;
                {
                    // Scoped to release lock before entering while loop
                    let mut tracker = tracker.lock().await;
                    slack = tracker.check_slack();
                }
                while slack == 0 {
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
                    }
                }
                {
                    let mut tracker = tracker.lock().await;
                    tracker.calls.push(event);
                }

                let fut = tokio::spawn(async move {
                    let mut result = ResultStatus::Pass(id);
                    tracing::info!("Calling download for {path:#?}");
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
        let mut handles = Vec::new();
        for app in self.iter() {
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
                    tracing::info!("Calling load for {path:#?}");
                    let status;
                    match app.load() {
                        Ok(response) => {
                            if let Some(dataset) = response.data() {
                                {
                                    // Scoped to release lock immediately after pushing update.
                                    let mut data = data.lock().await;
                                    data.push(dataset);
                                    tracing::info!("Dataset loaded.");
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
    derive_more::Display,
    derive_more::FromStr,
)]
pub enum Mode {
    Download,
    Load,
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct Tracker {
    calls: Vec<Event>,
    errors: Vec<Event>,
    cache: Vec<Event>,
}

impl Tracker {
    #[tracing::instrument(skip_all)]
    pub fn update_status(&mut self, status: ResultStatus, mode: Mode) {
        match status {
            ResultStatus::Success(id, length) => {
                if let Some(event) = self.calls.iter_mut().find(|event| event.id == id) {
                    event.status = status;
                    event.length = Some(length);
                    match mode {
                        Mode::Download => tracing::info!(target: "download_history", "{event}"),
                        Mode::Load => tracing::info!(target: "load_history", "{event}"),
                    }
                } else if let Some(event) = self.cache.iter_mut().find(|event| event.id == id) {
                    event.status = status;
                    event.length = Some(length);
                    match mode {
                        Mode::Download => tracing::info!(target: "download_history", "{event}"),
                        Mode::Load => tracing::info!(target: "load_history", "{event}"),
                    }
                }
            }
            ResultStatus::Error(id) => {
                if let Some(event) = self.calls.iter_mut().find(|event| event.id == id) {
                    event.status = status;
                    match mode {
                        Mode::Download => tracing::info!(target: "download_history", "{event}"),
                        Mode::Load => tracing::info!(target: "load_history", "{event}"),
                    }
                } else if let Some(event) = self.cache.iter_mut().find(|event| event.id == id) {
                    event.status = status;
                    match mode {
                        Mode::Download => tracing::info!(target: "download_history", "{event}"),
                        Mode::Load => tracing::info!(target: "load_history", "{event}"),
                    }
                }
                if let Some(event) = self.calls.iter().find(|event| event.id == id) {
                    self.errors.push(event.clone());
                }
            }
            ResultStatus::Pending | ResultStatus::Abort | ResultStatus::Pass(_) => {}
        }
    }

    #[tracing::instrument(skip_all)]
    pub fn check_slack(&mut self) -> usize {
        tracing::trace!("Checking slack.");
        // Cannot exceed 30 errors per minute.
        // Theory: calls may get ahead of tracker
        // 14 ahead of 14 = 28
        // hitting API rate limit at 14, try 10
        // probably hitting the data upload limit of 100MB per minute
        let error_cap: usize = 10;
        // Cannot exceed 100 calls per minute.
        // 14 ahead of 85 = 99
        let call_cap: usize = 70;
        self.update_count();
        let pending = self
            .calls
            .iter()
            .filter(|c| c.status == ResultStatus::Pending)
            .collect::<Vec<&Event>>()
            .len();
        let pending_slack = error_cap.saturating_sub(pending);
        let error_slack = error_cap.saturating_sub(self.errors.len());
        let call_slack = call_cap.saturating_sub(self.calls.len());
        let slack = error_slack.min(call_slack);
        let slack = slack.min(pending_slack);
        tracing::trace!("Pending slack {pending_slack}.");
        tracing::trace!("Error slack {error_slack}.");
        tracing::trace!("Call slack {call_slack}.");
        tracing::info!("Slack is {slack}.");
        slack
    }

    #[tracing::instrument(skip_all)]
    pub fn update_count(&mut self) {
        tracing::trace!("Updating count.");
        let now = jiff::Timestamp::now();
        if !self.calls.is_empty() {
            let mut old = self.calls.clone();
            old.retain(|call| {
                (now - call.time).total(jiff::Unit::Minute).unwrap()
                    >= 1.minutes().total(jiff::Unit::Minute).unwrap()
            });
            self.cache.extend(old);
            self.calls.retain(|call| {
                (now - call.time).total(jiff::Unit::Minute).unwrap()
                    < 1.minutes().total(jiff::Unit::Minute).unwrap()
            });
        }
        if !self.errors.is_empty() {
            self.errors.retain(|error| {
                (now - error.time).total(jiff::Unit::Minute).unwrap()
                    < 1.minutes().total(jiff::Unit::Minute).unwrap()
            });
        }
        tracing::info!(
            "Calls: {}, Errors: {}.",
            self.calls.len(),
            self.errors.len()
        );
    }

    #[tracing::instrument(skip_all)]
    pub async fn wait(&self) {
        tracing::trace!("Calling wait.");
        let minute = 1.minute().total(jiff::Unit::Millisecond).unwrap();
        let mut pause = minute.ceil() as u64;
        tracing::trace!("Maximum pause is {pause} millis.");
        let now = jiff::Timestamp::now();
        if !self.calls.is_empty() {
            let oldest = self
                .calls
                .iter()
                .map(|c| {
                    let cap = now - 1.minute();
                    (c.time - cap).total(jiff::Unit::Millisecond).unwrap()
                })
                .fold(minute, f64::min);
            let oldest = oldest.ceil() as u64;
            tracing::trace!("Oldest call is {oldest} millis away from expiring.");
            pause = pause.min(oldest);
        }
        if !self.errors.is_empty() {
            let oldest = self
                .errors
                .iter()
                .map(|c| {
                    let cap = now - 1.minute();
                    (c.time - cap).total(jiff::Unit::Millisecond).unwrap()
                })
                .fold(minute, f64::min);
            let oldest = oldest.ceil() as u64;
            tracing::trace!("Oldest call is {oldest} millis away from expiring.");
            pause = pause.min(oldest);
        }
        if pause > 5000 {
            tracing::trace!("Pausing for 5000 millis.");
            tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
        } else {
            tracing::trace!("Pausing for {pause} millis.");
            tokio::time::sleep(tokio::time::Duration::from_millis(pause)).await;
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
    derive_getters::Getters,
)]
pub struct Event {
    id: uuid::Uuid,
    length: Option<u64>,
    mode: Mode,
    path: std::path::PathBuf,
    status: ResultStatus,
    time: jiff::Timestamp,
}

impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id: {}, length: {}, mode: {}, path: {:?}, status: {}, time: {}",
            self.id,
            self.len_as_str(),
            self.mode,
            self.path,
            self.status,
            self.time
        )
    }
}

impl Event {
    pub fn new<P: AsRef<std::path::Path>>(path: P, mode: Mode) -> Self {
        let id = uuid::Uuid::new_v4();
        let path = std::path::PathBuf::from(path.as_ref());
        let status = ResultStatus::Pending;
        let time = jiff::Timestamp::now();
        Self {
            id,
            length: None,
            mode,
            path,
            status,
            time,
        }
    }

    pub fn len_as_str(&self) -> String {
        match self.length {
            Some(num) => num.to_string(),
            None => "None".to_string(),
        }
    }

    pub fn len_from_str(len: &str) -> Result<Option<u64>, BeaErr> {
        match len {
            "None" => Ok(None),
            number => match number.parse::<u64>() {
                Ok(num) => Ok(Some(num)),
                Err(source) => {
                    let error =
                        ParseInt::new(number.to_string(), source, line!(), file!().to_string());
                    Err(error.into())
                }
            },
        }
    }

    pub fn read_json(m: &serde_json::Map<String, serde_json::Value>) -> Result<Self, BeaErr> {
        let msg = map_to_string("message", m)?;
        // skip the key in key: value pair sequence
        // skip the first field "id"
        let result = msg
            .split(",")
            .flat_map(|pair| pair.split(":").skip(1))
            .skip(1)
            .collect::<Vec<&str>>();
        tracing::trace!("Raw Event: {result:#?}");
        let id = uuid::Uuid::new_v4();
        if result.len() < 7 {
            let error = KeyMissing::new("invalid Event".to_string(), line!(), file!().to_string());
            let error = JsonParseErrorKind::from(error);
            let error = JsonParseError::from(error);
            Err(error.into())
        } else {
            let length = Self::len_from_str(result[0].trim())?;
            let mode = result[1].trim().to_string();
            let mode = Mode::from_str(&mode)
                .map_err(|e| DeriveFromStr::new(mode, e, line!(), file!().to_string()))?;
            let path = result[2].trim().trim_matches('"');
            tracing::trace!("Path is {path}");
            let path = std::path::PathBuf::from(path);
            let status = result[3].trim().to_string();
            let status = ResultStatus::from_str(&status)?;
            let time = result[4].trim().to_string();
            let time = format!("{time}:{}:{}", result[5], result[6]);
            let time = match jiff::Timestamp::from_str(&time) {
                Ok(stamp) => stamp,
                Err(source) => {
                    let error = Jiff::new(time, source);
                    return Err(error.into());
                }
            };
            Ok(Self {
                id,
                length,
                mode,
                path,
                status,
                time,
            })
        }
    }
}

impl TryFrom<&serde_json::Value> for Event {
    type Error = BeaErr;

    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        tracing::trace!("Reading Event.");
        match value {
            serde_json::Value::Object(m) => {
                let key = "fields".to_string();
                if let Some(fields) = m.get(&key) {
                    tracing::trace!("Fields present: {fields:#?}");
                    match fields {
                        serde_json::Value::Object(m) => Self::read_json(m),
                        _ => {
                            tracing::trace!("Invalid Value: {value:#?}");
                            let error = NotObject::new(line!(), file!().to_string());
                            let error = JsonParseErrorKind::from(error);
                            let error = JsonParseError::from(error);
                            Err(error.into())
                        }
                    }
                } else {
                    let error = KeyMissing::new(key, line!(), file!().to_string());
                    let error = JsonParseErrorKind::from(error);
                    let error = JsonParseError::from(error);
                    Err(error.into())
                }
            }
            _ => {
                tracing::trace!("Invalid Value: {value:#?}");
                let error = NotObject::new(line!(), file!().to_string());
                let error = JsonParseErrorKind::from(error);
                let error = JsonParseError::from(error);
                Err(error.into())
            }
        }
    }
}
