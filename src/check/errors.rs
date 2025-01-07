use crate::{trace_init, BeaErr, Check, EnvError};

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
