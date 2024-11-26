use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod app;
pub mod command;
mod config;
mod dataset;
mod error;
mod geofips;
pub mod getdata;
pub mod getparameterlist;
pub mod getparametervalues;
mod json;
pub mod linecode;
mod method;
mod parameter;
mod request;
mod user;

pub use app::App;
pub use config::{Config, NeoConfig, Options};
pub use dataset::{BeaDatasets, Dataset};
pub use error::BeaError;
pub use geofips::{get_geofips, BeaGeoFips, GeoFipsTask, GeoFipsTasks};
pub use getparameterlist::{deserialize_bool, BeaParameters};
pub use getparametervalues::BeaParameterValues;
pub use json::Json;
pub use method::Method;
pub use request::{RequestParameter, RequestParameters};
pub use user::User;

pub fn trace_init() {
    if tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "bea=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .is_ok()
    {};
    tracing::trace!("Loading Bea...");
}
