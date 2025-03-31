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
mod key_sets;
mod keys;
mod method;
mod naics;
mod parameter;
mod parameter_value;
mod progress;
mod queue;
mod request;
mod results;
mod validate;
mod value;

pub use app::{App, BTreeKeyMissing, RateLimit, ResultStatus};
pub use config::{Options, ParameterKind};
pub use data::{Data, FixedAssetData, GdpData, GdpDatum, MneDiData, NipaData};
pub use dataset::{Dataset, DatasetDetails, Datasets};
// investment invalid unused
// check unused
pub use error::{
    BeaErr, BeaErrorKind, DatasetMissing, DeriveFromStr, FromStrError, IoError, Jiff,
    JsonParseError, JsonParseErrorKind, KeyMissing, NotArray, NotObject,
    ParameterValueTableVariant, ParseInt, ReqwestError, SerdeJson, Set, VariantMissing,
};
pub use free::{
    bea_data, file_size, from_csv, init, json_bool, json_str, map_to_bool, map_to_float,
    map_to_int, map_to_string, to_csv, trace_init, Csv, EnvError, NotFloat, NotInteger, ParseFloat,
    ParseInteger, UrlParseError,
};
pub use history::{Chunk, Chunks, History};
pub use json::Json;
pub use key_sets::{
    FixedAssets, GdpByIndustry, Ita, ItaData, ItaDatum, Mne, NiUnderlyingDetail, Nipa, NipaIterator,
};
pub use keys::{
    date_by_period, parse_year, roman_numeral_quarter, AffiliateKind, AffiliateLevel,
    AnnotatedInteger, Annotation, AnnotationMissing, AreaOrCountry, BoolInvalid, BoolOptions,
    DirectionOfInvestment, Footnotes, Frequencies, Frequency, FrequencyOptions, Indicator, Integer,
    IntegerInvalid, IntegerKind, IntegerOptions, InvestmentKind, ItaFrequencies, ItaFrequency,
    Millions, MillionsOptions, NipaRange, NipaRangeIterator, NipaRanges, NipaTableName, Nom,
    NotQuarter, OwnershipInvalid, OwnershipKind, OwnershipLevel, RowCode, RowCodeMissing,
    SelectionKind, SelectionSet, State, StateKind, TableName, Year, YearInvalid, YearKind,
    YearOptions, YearRange,
};
pub use method::Method;
pub use naics::Naics;
pub use parameter::{deserialize_bool, NotParameterName, Parameter, ParameterName, Parameters};
pub use parameter_value::{
    Metadata, MneDoi, NipaFrequency, NipaShowMillions, NipaTable, NipaYear, ParameterFields,
    ParameterValueTable, ParameterValues,
};
pub use progress::{Progress, Style};
pub use queue::{Event, Mode, Queue, Tracker};
pub use request::{Request, RequestParameter, RequestParameters};
pub use results::{BeaResponse, Results};
pub use value::{
    ApiMetadata, Iip, InputOutput, IntlServSta, IntlServTrade, Regional, UnderlyingGdpByIndustry,
    ValueSet, ValueSets,
};
