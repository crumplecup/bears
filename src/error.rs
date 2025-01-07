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

impl From<BoolInvalid> for BeaErr {
    fn from(value: BoolInvalid) -> Self {
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

impl From<Jiff> for BeaErr {
    fn from(value: Jiff) -> Self {
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

impl From<Set> for BeaErr {
    fn from(value: Set) -> Self {
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

impl From<IntegerInvalid> for BeaErr {
    fn from(value: IntegerInvalid) -> Self {
        let kind = BeaErrorKind::from(value).into();
        Self { kind }
    }
}

impl From<InvestmentInvalid> for BeaErr {
    fn from(value: InvestmentInvalid) -> Self {
        let kind = BeaErrorKind::from(value).into();
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

impl From<OwnershipInvalid> for BeaErr {
    fn from(value: OwnershipInvalid) -> Self {
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

impl From<VariantMissing> for BeaErr {
    fn from(value: VariantMissing) -> Self {
        let kind = BeaErrorKind::from(value).into();
        Self { kind }
    }
}

impl From<YearInvalid> for BeaErr {
    fn from(value: YearInvalid) -> Self {
        let kind = BeaErrorKind::from(value).into();
        Self { kind }
    }
}

#[derive(Debug, derive_more::From)]
pub enum BeaErrorKind {
    #[from(BincodeError)]
    Bincode(BincodeError),
    #[from(BoolInvalid)]
    BoolInvalid(BoolInvalid),
    #[from(Check)]
    Check(Check),
    #[from(DatasetMissing)]
    DatasetMissing(DatasetMissing),
    #[from(DeriveFromStr)]
    DeriveFromStr(DeriveFromStr),
    #[from(EnvError)]
    Env(EnvError),
    #[from(IntegerInvalid)]
    IntegerInvalid(IntegerInvalid),
    #[from(InvestmentInvalid)]
    InvestmentInvalid(InvestmentInvalid),
    #[from(IoError)]
    Io(IoError),
    #[from(Io, std::io::Error)]
    IoPass(Io),
    #[from(Jiff)]
    Jiff(Jiff),
    #[from(JsonParseError)]
    JsonParse(JsonParseError),
    #[from(OwnershipInvalid)]
    OwnershipInvalid(OwnershipInvalid),
    #[from(ParameterValueTableVariant)]
    ParameterValueTableVariant(ParameterValueTableVariant),
    #[from(ParseInt)]
    ParseInt(ParseInt),
    #[from(ReqwestError)]
    Reqwest(ReqwestError),
    #[from(Set)]
    Set(Set),
    #[from(SerdeJson, serde_json::Error)]
    SerdeJson(SerdeJson),
    #[from(UrlParseError)]
    UrlParse(UrlParseError),
    #[from(VariantMissing)]
    VariantMissing(VariantMissing),
    #[from(YearInvalid)]
    YearInvalid(YearInvalid),
}

impl std::fmt::Display for BeaErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bincode(e) => {
                write!(f, "{e}")
            }
            Self::BoolInvalid(e) => {
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
            Self::IntegerInvalid(e) => {
                write!(f, "{e}")
            }
            Self::InvestmentInvalid(e) => {
                write!(f, "{e}")
            }
            Self::Io(e) => {
                write!(f, "{e}")
            }
            Self::IoPass(e) => {
                write!(f, "{e}")
            }
            Self::Jiff(e) => {
                write!(f, "{e}")
            }
            Self::JsonParse(e) => {
                write!(f, "{e}")
            }
            Self::OwnershipInvalid(e) => {
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
            Self::Set(e) => {
                write!(f, "{e}")
            }
            Self::SerdeJson(e) => {
                write!(f, "{e}")
            }
            Self::UrlParse(e) => {
                write!(f, "{e}")
            }
            Self::VariantMissing(e) => {
                write!(f, "{e}")
            }
            Self::YearInvalid(e) => {
                write!(f, "{e}")
            }
        }
    }
}

impl std::error::Error for BeaErrorKind {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Bincode(e) => Some(e.source()),
            Self::BoolInvalid(e) => e.source(),
            Self::Check(e) => e.source(),
            Self::DatasetMissing(e) => e.source(),
            Self::DeriveFromStr(e) => e.source(),
            Self::Env(e) => Some(e.source()),
            Self::IntegerInvalid(e) => e.source(),
            Self::InvestmentInvalid(e) => e.source(),
            Self::Io(e) => Some(e.source()),
            Self::IoPass(e) => Some(e.source()),
            Self::Jiff(e) => e.source(),
            Self::JsonParse(e) => e.source(),
            Self::OwnershipInvalid(e) => e.source(),
            Self::ParameterValueTableVariant(e) => e.source(),
            Self::ParseInt(e) => Some(e.source()),
            Self::Reqwest(e) => Some(e.source()),
            Self::Set(e) => e.source(),
            Self::SerdeJson(e) => Some(e.source()),
            Self::UrlParse(e) => Some(e.source()),
            Self::VariantMissing(e) => e.source(),
            Self::YearInvalid(e) => e.source(),
        }
    }
}

#[derive(Debug, derive_more::From)]
pub enum JsonParseErrorKind {
    #[from(FromStrError)]
    FromStr(FromStrError),
    KeyMissing(String),
    #[from(NotArray)]
    NotArray(NotArray),
    NotBool,
    #[from(NotFloat)]
    NotFloat(NotFloat),
    #[from(NotInteger)]
    NotInteger(NotInteger),
    #[from(NotObject)]
    NotObject(NotObject),
    #[from(NotParameterName)]
    NotParameterName(NotParameterName),
    NotString,
    #[from(ParseFloat)]
    ParseFloat(ParseFloat),
    #[from(ParseInteger)]
    ParseInteger(ParseInteger),
}

impl std::fmt::Display for JsonParseErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FromStr(e) => write!(f, "problem converting str to json: {e}"),
            Self::KeyMissing(s) => write!(f, "missing key {s}"),
            Self::NotArray(e) => write!(f, "{e}"),
            Self::NotBool => write!(f, "serde_json::Value is not a Number or String variant"),
            Self::NotFloat(e) => write!(f, "{e}"),
            Self::NotInteger(e) => write!(f, "{e}"),
            Self::NotParameterName(e) => write!(f, "{e}"),
            Self::NotObject(e) => write!(f, "{e}"),
            Self::NotString => write!(f, "serde_json::Value is not a String variant"),
            Self::ParseFloat(e) => write!(f, "{e}"),
            Self::ParseInteger(e) => write!(f, "{e}"),
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
    line: u32,
    file: String,
}

impl ReqwestError {
    #[tracing::instrument(skip_all)]
    pub fn new<S: std::fmt::Display>(
        url: S,
        method: S,
        source: reqwest::Error,
        line: u32,
        file: String,
    ) -> Self {
        let url = url.to_string();
        let method = method.to_string();
        Self {
            url,
            method,
            headers: None,
            body: None,
            form: None,
            source,
            line,
            file,
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
#[display("io error at path {path:?} in line {line} of {file}")]
#[setters(prefix = "with_", borrow_self)]
pub struct IoError {
    pub path: std::path::PathBuf,
    pub source: std::io::Error,
    line: u32,
    file: String,
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
pub struct ParseInt {
    input: String,
    source: std::num::ParseIntError,
    line: u32,
    file: String,
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
#[display("{} at line {} in {}", self.issue, self.line, self.file)]
#[setters(prefix = "with_", borrow_self)]
pub struct ParameterValueTableVariant {
    issue: String,
    line: u32,
    file: String,
}

impl std::error::Error for ParameterValueTableVariant {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone, derive_new::new, derive_more::Display, derive_more::Error)]
#[display("could not parse from {} into target value on line {} in {}", self.value, self.line, self.file)]
pub struct DeriveFromStr {
    value: String,
    source: derive_more::FromStrError,
    line: u32,
    file: String,
}

#[derive(Debug, Clone, derive_new::new, derive_more::Display, derive_more::Error)]
#[display("could not parse from {} into target type", self.value)]
pub struct Jiff {
    value: String,
    source: jiff::Error,
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::Display, strum::EnumIter,
)]
pub enum Set {
    Empty,
    ParameterFieldsMissing,
    ParameterNameMissing(String),
    ParameterValuesMissing,
}

impl std::error::Error for Set {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::Display, derive_new::new,
)]
#[display("{clue} with {input} at line {line} in file {file}")]
pub struct VariantMissing {
    clue: String,
    input: String,
    line: u32,
    file: String,
}

impl std::error::Error for VariantMissing {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::Display, derive_new::new,
)]
#[display("{input} is not a valid Year at line {line} in file {file}")]
pub struct YearInvalid {
    input: String,
    line: u32,
    file: String,
}

impl std::error::Error for YearInvalid {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::Display, derive_new::new,
)]
#[display("{input} is not a valid Integer at line {line} in file {file}")]
pub struct IntegerInvalid {
    input: String,
    line: u32,
    file: String,
}

impl std::error::Error for IntegerInvalid {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::Display, derive_new::new,
)]
#[display("{input} is not a valid Ownership at line {line} in file {file}")]
pub struct OwnershipInvalid {
    input: String,
    line: u32,
    file: String,
}

impl std::error::Error for OwnershipInvalid {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::Display, derive_new::new,
)]
#[display("{input} is not a valid bool at line {line} in file {file}")]
pub struct BoolInvalid {
    input: String,
    line: u32,
    file: String,
}

impl std::error::Error for BoolInvalid {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::Display, derive_new::new,
)]
#[display("{input} is not a valid Investment at line {line} in file {file}")]
pub struct InvestmentInvalid {
    input: String,
    line: u32,
    file: String,
}

impl std::error::Error for InvestmentInvalid {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display, derive_new::new)]
#[display("could not parse {input} to float at line {line} in file {file}")]
pub struct ParseFloat {
    input: String,
    source: std::num::ParseFloatError,
    line: u32,
    file: String,
}

impl std::error::Error for ParseFloat {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display, derive_new::new)]
#[display("could not parse {input} to integer at line {line} in file {file}")]
pub struct ParseInteger {
    input: String,
    source: std::num::ParseIntError,
    line: u32,
    file: String,
}

impl std::error::Error for ParseInteger {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display, derive_new::new)]
#[display("could not parse {input} to float at line {line} in file {file}")]
pub struct NotFloat {
    input: String,
    line: u32,
    file: String,
}

impl std::error::Error for NotFloat {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display, derive_new::new)]
#[display("could not parse {input} to integer at line {line} in file {file}")]
pub struct NotInteger {
    input: String,
    line: u32,
    file: String,
}

impl std::error::Error for NotInteger {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display, derive_new::new)]
#[display("serde::Value::Object variant expected at line {line} in file {file}")]
pub struct NotObject {
    line: u32,
    file: String,
}

impl std::error::Error for NotObject {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display, derive_new::new)]
#[display("serde::Value::Array variant expected at line {line} in file {file}")]
pub struct NotArray {
    line: u32,
    file: String,
}

impl std::error::Error for NotArray {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
