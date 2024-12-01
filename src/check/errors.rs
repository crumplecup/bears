use crate::{trace_init, BeaErr, Check, EnvError, IoError};

#[tracing::instrument]
pub fn env() -> Result<(), BeaErr> {
    trace_init();
    match EnvError::from_env("NOT_THERE") {
        Ok(e) => {
            tracing::warn!("Unexpected success: {e:#?}");
            let error = Check::new("read from .env did not fail".to_string());
            Err(error.into())
        }
        Err(env) => {
            tracing::info!("{env}");
            tracing::info!("{}", env.source());
            Ok(())
        }
    }
}

#[tracing::instrument]
pub fn io_read() -> Result<(), BeaErr> {
    trace_init();
    match IoError::read("not_there".into()) {
        Ok(e) => {
            tracing::warn!("Unexpected success: {e:#?}");
            let error = Check::new("io_read did not fail".to_string());
            Err(error.into())
        }
        Err(io) => {
            tracing::info!("{io}");
            tracing::info!("{}", io.source());
            Ok(())
        }
    }
}
