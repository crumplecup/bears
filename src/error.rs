#[derive(Debug, derive_more::Deref, derive_more::DerefMut)]
pub struct BeaErr {
    kind: Box<BeaErrorKind>,
}

impl std::fmt::Display for BeaErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl std::error::Error for BeaErr {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.kind.source()
    }
}

impl From<BeaErrorKind> for BeaErr {
    fn from(value: BeaErrorKind) -> Self {
        let kind = Box::new(value);
        Self { kind }
    }
}

impl From<BincodeError> for BeaErr {
    fn from(value: BincodeError) -> Self {
        let kind = BeaErrorKind::from(value).into();
        Self { kind }
    }
}

impl From<Check> for BeaErr {
    fn from(value: Check) -> Self {
        let kind = BeaErrorKind::from(value).into();
        Self { kind }
    }
}

impl From<DatasetMissing> for BeaErr {
    fn from(value: DatasetMissing) -> Self {
        let kind = BeaErrorKind::from(value).into();
        Self { kind }
    }
}

impl From<DeriveFromStr> for BeaErr {
    fn from(value: DeriveFromStr) -> Self {
        let kind = BeaErrorKind::from(value).into();
        Self { kind }
    }
}

impl From<EnvError> for BeaErr {
    fn from(value: EnvError) -> Self {
        let kind = BeaErrorKind::from(value).into();
        Self { kind }
    }
}

impl From<JsonParseError> for BeaErr {
    fn from(value: JsonParseError) -> Self {
        let kind = BeaErrorKind::from(value).into();
        Self { kind }
    }
}

impl From<ReqwestError> for BeaErr {
    fn from(value: ReqwestError) -> Self {
        let kind = BeaErrorKind::from(value).into();
        Self { kind }
    }
}

impl From<serde_json::Error> for BeaErr {
    fn from(value: serde_json::Error) -> Self {
        let kind = SerdeJson::from(value);
        let kind = BeaErrorKind::from(kind).into();
        Self { kind }
    }
}

impl From<SerdeJson> for BeaErr {
    fn from(value: SerdeJson) -> Self {
        let kind = BeaErrorKind::from(value).into();
        Self { kind }
    }
}

impl From<std::io::Error> for BeaErr {
    fn from(value: std::io::Error) -> Self {
        let kind = Io::from(value);
        let kind = BeaErrorKind::from(kind).into();
        Self { kind }
    }
}

impl From<Io> for BeaErr {
    fn from(value: Io) -> Self {
        let kind = BeaErrorKind::from(value).into();
        Self { kind }
    }
}

impl From<IoError> for BeaErr {
    fn from(value: IoError) -> Self {
        let kind = BeaErrorKind::from(value).into();
        Self { kind }
    }
}

impl From<ParameterValueTableVariant> for BeaErr {
    fn from(value: ParameterValueTableVariant) -> Self {
        let kind = BeaErrorKind::from(value).into();
        Self { kind }
    }
}

impl From<ParseInt> for BeaErr {
    fn from(value: ParseInt) -> Self {
        let kind = BeaErrorKind::from(value).into();
        Self { kind }
    }
}

impl From<UrlParseError> for BeaErr {
    fn from(value: UrlParseError) -> Self {
        let kind = BeaErrorKind::from(value).into();
        Self { kind }
    }
}

#[derive(Debug, derive_more::From)]
#[non_exhaustive]
pub enum BeaErrorKind {
    #[from(BincodeError)]
    Bincode(BincodeError),
    #[from(Check)]
    Check(Check),
    #[from(DatasetMissing)]
    DatasetMissing(DatasetMissing),
    #[from(DeriveFromStr)]
    DeriveFromStr(DeriveFromStr),
    #[from(EnvError)]
    Env(EnvError),
    #[from(IoError)]
    Io(IoError),
    #[from(Io, std::io::Error)]
    IoPass(Io),
    #[from(JsonParseError)]
    JsonParse(JsonParseError),
    #[from(ParameterValueTableVariant)]
    ParameterValueTableVariant(ParameterValueTableVariant),
    #[from(ParseInt)]
    ParseInt(ParseInt),
    #[from(SerdeJson, serde_json::Error)]
    SerdeJson(SerdeJson),
    #[from(ReqwestError)]
    Reqwest(ReqwestError),
    #[from(UrlParseError)]
    UrlParse(UrlParseError),
}

impl std::fmt::Display for BeaErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bincode(e) => {
                write!(f, "{e}")
            }
            Self::Check(e) => {
                write!(f, "{e}")
            }
            Self::DatasetMissing(e) => {
                write!(f, "{e}")
            }
            Self::DeriveFromStr(e) => {
                write!(f, "{e}")
            }
            Self::Env(e) => {
                write!(f, "{e}")
            }
            Self::Io(e) => {
                write!(f, "{e}")
            }
            Self::IoPass(e) => {
                write!(f, "{e}")
            }
            Self::JsonParse(e) => {
                write!(f, "{e}")
            }
            Self::ParameterValueTableVariant(e) => {
                write!(f, "{e}")
            }
            Self::ParseInt(e) => {
                write!(f, "{e}")
            }
            Self::Reqwest(e) => {
                write!(f, "{e}")
            }
            Self::SerdeJson(e) => {
                write!(f, "{e}")
            }
            Self::UrlParse(e) => {
                write!(f, "{e}")
            }
        }
    }
}

impl std::error::Error for BeaErrorKind {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Bincode(e) => Some(e.source()),
            Self::Check(e) => e.source(),
            Self::DatasetMissing(e) => e.source(),
            Self::DeriveFromStr(e) => e.source(),
            Self::Env(e) => Some(e.source()),
            Self::Io(e) => Some(e.source()),
            Self::IoPass(e) => Some(e.source()),
            Self::JsonParse(e) => e.source(),
            Self::ParameterValueTableVariant(e) => e.source(),
            Self::ParseInt(e) => Some(e.source()),
            Self::Reqwest(e) => Some(e.source()),
            Self::SerdeJson(e) => Some(e.source()),
            Self::UrlParse(e) => Some(e.source()),
        }
    }
}

#[derive(Debug, derive_more::From)]
pub enum JsonParseErrorKind {
    #[from(FromStrError)]
    FromStr(FromStrError),
    KeyMissing(String),
    NotArray,
    NotBool,
    NotObject,
    #[from(NotParameterName)]
    NotParameterName(NotParameterName),
    NotString,
}

impl std::fmt::Display for JsonParseErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FromStr(e) => write!(f, "problem converting str to json: {e}"),
            Self::KeyMissing(s) => write!(f, "missing key {s}"),
            Self::NotArray => write!(f, "serde_json::Value is not an Array or Object variant"),
            Self::NotBool => write!(f, "serde_json::Value is not a Number or String variant"),
            Self::NotParameterName(e) => write!(f, "{e}"),
            Self::NotObject => write!(f, "serde_json::Value is not an Object variant"),
            Self::NotString => write!(f, "serde_json::Value is not a String variant"),
        }
    }
}

impl std::error::Error for JsonParseErrorKind {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl From<JsonParseErrorKind> for JsonParseError {
    fn from(kind: JsonParseErrorKind) -> Self {
        Self { kind }
    }
}

#[derive(Debug, derive_new::new, derive_more::Deref, derive_more::DerefMut)]
#[non_exhaustive]
pub struct JsonParseError {
    kind: JsonParseErrorKind,
}

impl std::fmt::Display for JsonParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error parsing json")
    }
}

impl std::error::Error for JsonParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.kind)
    }
}

#[derive(Debug, derive_new::new)]
pub struct NotParameterName(String);

impl std::fmt::Display for NotParameterName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} is not a parameter name", self.0)
    }
}

impl std::error::Error for NotParameterName {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(Debug, derive_new::new)]
#[non_exhaustive]
pub struct FromStrError {
    value: String,
    source: serde_json::Error,
}

impl std::fmt::Display for FromStrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", self.value)
    }
}

impl std::error::Error for FromStrError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

#[derive(Debug, derive_getters::Getters, derive_setters::Setters)]
#[setters(prefix = "with_", strip_option, borrow_self)]
pub struct ReqwestError {
    pub url: String,
    pub method: String,
    pub headers: Option<Vec<(String, String)>>,
    pub body: Option<Vec<(String, String)>>,
    pub form: Option<Vec<(String, String)>>,
    pub source: reqwest::Error,
}

impl ReqwestError {
    #[tracing::instrument(skip_all)]
    pub fn new<S: std::fmt::Display>(url: S, method: S, source: reqwest::Error) -> Self {
        let url = url.to_string();
        let method = method.to_string();
        Self {
            url,
            method,
            headers: None,
            body: None,
            form: None,
            source,
        }
    }
}

impl std::fmt::Display for ReqwestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = "error in ".to_string();
        msg.push_str(&format!("{} request to {}", self.method, self.url));
        if let Some(headers) = self.headers.clone() {
            msg.push_str(&format!(" including headers {headers:?}"));
        }
        if let Some(body) = self.body.clone() {
            msg.push_str(&format!(" with body {body:?}"));
        }
        if let Some(form) = self.form.clone() {
            msg.push_str(&format!(" with form body {form:?}"));
        }
        write!(f, "{msg}")
    }
}

impl std::error::Error for ReqwestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.source())
    }
}

#[derive(
    Debug, derive_getters::Getters, derive_setters::Setters, derive_more::Display, derive_new::new,
)]
#[display(".env file missing {}", self.target)]
#[setters(prefix = "with_", borrow_self)]
pub struct EnvError {
    pub target: String,
    pub source: std::env::VarError,
}

impl EnvError {
    #[tracing::instrument]
    pub fn from_env(key: &str) -> Result<String, Self> {
        match std::env::var(key) {
            Ok(value) => Ok(value),
            Err(source) => {
                let error = Self::new(key.to_string(), source);
                Err(error)
            }
        }
    }
}

impl std::error::Error for EnvError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

#[derive(
    Debug, derive_getters::Getters, derive_setters::Setters, derive_more::Display, derive_new::new,
)]
#[display("{} failed parse to a valid url", self.target)]
#[setters(prefix = "with_", borrow_self)]
pub struct UrlParseError {
    pub target: String,
    pub source: url::ParseError,
}

impl UrlParseError {
    #[tracing::instrument]
    pub fn into_url(s: &str) -> Result<url::Url, Self> {
        match url::Url::parse(s) {
            Ok(value) => Ok(value),
            Err(source) => {
                let error = Self::new(s.to_string(), source);
                Err(error)
            }
        }
    }
}

impl std::error::Error for UrlParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

#[derive(
    Debug, derive_getters::Getters, derive_setters::Setters, derive_more::Display, derive_new::new,
)]
#[display("error {}", self.desc)]
#[setters(prefix = "with_", borrow_self)]
pub struct BincodeError {
    pub desc: String,
    pub source: Box<bincode::ErrorKind>,
}

impl std::error::Error for BincodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

#[derive(
    Debug,
    derive_getters::Getters,
    derive_setters::Setters,
    derive_more::Display,
    derive_more::From,
    derive_new::new,
)]
#[display("io error")]
#[setters(prefix = "with_", borrow_self)]
#[from(std::io::Error)]
pub struct Io {
    pub source: std::io::Error,
}

impl std::error::Error for Io {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

#[derive(
    Debug, derive_getters::Getters, derive_setters::Setters, derive_more::Display, derive_new::new,
)]
#[display("io error at path {:?}", self.path)]
#[setters(prefix = "with_", borrow_self)]
pub struct IoError {
    pub path: std::path::PathBuf,
    pub source: std::io::Error,
}

impl IoError {
    #[tracing::instrument(skip(contents))]
    pub fn write(contents: &Vec<u8>, path: std::path::PathBuf) -> Result<(), Self> {
        match std::fs::write(path.clone(), contents) {
            Ok(()) => Ok(()),
            Err(source) => {
                let error = IoError::new(path, source);
                Err(error)
            }
        }
    }

    #[tracing::instrument]
    pub fn read(path: std::path::PathBuf) -> Result<Vec<u8>, Self> {
        match std::fs::read(path.clone()) {
            Ok(data) => Ok(data),
            Err(source) => {
                let error = Self::new(path, source);
                Err(error)
            }
        }
    }

    #[tracing::instrument]
    pub fn open(path: std::path::PathBuf) -> Result<std::fs::File, Self> {
        match std::fs::File::open(&path) {
            Ok(file) => Ok(file),
            Err(source) => {
                let error = Self::new(path, source);
                Err(error)
            }
        }
    }

    #[tracing::instrument]
    pub fn remove_file(path: std::path::PathBuf) -> Result<(), Self> {
        match std::fs::remove_file(path.clone()) {
            Ok(()) => Ok(()),
            Err(source) => {
                let error = IoError::new(path, source);
                Err(error)
            }
        }
    }
}

impl std::error::Error for IoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

#[derive(
    Debug,
    derive_getters::Getters,
    derive_setters::Setters,
    derive_more::Display,
    derive_new::new,
    derive_more::From,
)]
#[display("error from serde_json library")]
#[setters(prefix = "with_", borrow_self)]
#[from(serde_json::Error)]
pub struct SerdeJson {
    pub source: serde_json::Error,
}

impl std::error::Error for SerdeJson {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

#[derive(
    Debug,
    derive_getters::Getters,
    derive_setters::Setters,
    derive_more::Display,
    derive_new::new,
    derive_more::From,
)]
#[display("variant missing for dataset {}", self.name)]
#[setters(prefix = "with_", borrow_self)]
#[from(String, &String, &str)]
pub struct DatasetMissing {
    pub name: String,
}

impl std::error::Error for DatasetMissing {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(
    Debug,
    derive_getters::Getters,
    derive_setters::Setters,
    derive_more::Display,
    derive_new::new,
    derive_more::From,
)]
#[display("Check failed: {}", self.desc)]
#[setters(prefix = "with_", borrow_self)]
#[from(String, &str)]
pub struct Check {
    pub desc: String,
}

impl std::error::Error for Check {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(
    Debug,
    derive_getters::Getters,
    derive_setters::Setters,
    derive_more::Display,
    derive_new::new,
    derive_more::From,
)]
#[display("error from serde_json library")]
#[setters(prefix = "with_", borrow_self)]
#[from((&str, std::num::ParseIntError))]
pub struct ParseInt {
    pub input: String,
    pub source: std::num::ParseIntError,
}

impl std::error::Error for ParseInt {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

#[derive(
    Debug,
    derive_getters::Getters,
    derive_setters::Setters,
    derive_more::Display,
    derive_new::new,
    derive_more::From,
)]
#[display("{}", self.issue)]
#[setters(prefix = "with_", borrow_self)]
#[from(&str)]
pub struct ParameterValueTableVariant {
    pub issue: String,
}

impl std::error::Error for ParameterValueTableVariant {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone, derive_new::new, derive_more::Display, derive_more::Error)]
#[display("could not parse from {} into target value", self.value)]
pub struct DeriveFromStr {
    value: String,
    source: derive_more::FromStrError,
}
