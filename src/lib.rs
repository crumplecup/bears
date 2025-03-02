#![doc(html_playground_url = "https://play.rust-lang.org/")]
mod app;
pub mod check;
pub mod command;
mod config;
mod data;
mod dataset;
mod error;
mod free;
mod history;
mod json;
mod method;
mod naics;
mod parameter;
mod parameter_value;
mod queue;
mod request;
mod results;
mod validate;
mod value;

pub use app::{App, ResultStatus};
pub use config::{Options, ParameterKind};
pub use data::{Data, NipaData};
pub use dataset::{Dataset, DatasetDetails, Datasets};
pub use error::{
    AnnotationMissing, BTreeKeyMissing, BeaErr, BeaErrorKind, BincodeError, BoolInvalid, Check,
    Csv, DatasetMissing, DeriveFromStr, EnvError, FromStrError, IntegerInvalid, InvestmentInvalid,
    IoError, Jiff, JsonParseError, JsonParseErrorKind, KeyMissing, Nom, NotArray, NotFloat,
    NotInteger, NotObject, NotParameterName, NotQuarter, OwnershipInvalid,
    ParameterValueTableVariant, ParseFloat, ParseInt, ParseInteger, RateLimit, ReqwestError,
    RowCodeMissing, SerdeJson, Set, UrlParseError, VariantMissing, YearInvalid,
};
pub use free::{
    bea_data, file_size, from_csv, init, json_bool, json_str, map_to_bool, map_to_float,
    map_to_int, map_to_string, to_csv, trace_init,
};
pub use history::{Chunk, Chunks, History};
pub use json::Json;
pub use method::Method;
pub use naics::Naics;
pub use parameter::{deserialize_bool, Parameter, ParameterName, Parameters};
pub use parameter_value::{
    Metadata, MneDoi, NipaFrequency, NipaShowMillions, NipaTable, NipaYear, ParameterFields,
    ParameterValueTable, ParameterValues,
};
pub use queue::{Event, Mode, Queue, Tracker};
pub use request::{Request, RequestParameter, RequestParameters};
pub use results::{BeaResponse, Results};
pub use value::{
    date_by_period, parse_year, AffiliateKind, AffiliateLevel, AnnotatedInteger, Annotation,
    ApiMetadata, BoolOptions, DirectionOfInvestment, FixedAssets, Footnotes, Frequencies,
    Frequency, FrequencyOptions, GdpByIndustry, Iip, InputOutput, Integer, IntegerKind,
    IntegerOptions, IntlServSta, IntlServTrade, InvestmentKind, Ita, Millions, MillionsOptions,
    Mne, NiUnderlyingDetail, Nipa, NipaIterator, NipaRange, NipaRangeIterator, NipaRanges,
    OwnershipKind, OwnershipLevel, Regional, RowCode, SelectionKind, State, StateKind, TableName,
    UnderlyingGdpByIndustry, ValueSet, ValueSets, Year, YearKind, YearOptions, YearRange,
};
