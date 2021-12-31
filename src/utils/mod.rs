use std::fmt::Display;
use tracing::{debug, error, info, warn};

pub mod regex;

pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

pub trait Loggable {
    fn log_err<S: Into<String>>(self, message: S, level: LogLevel) -> Self;
}

impl<O, E: Display> Loggable for Result<O, E> {
    fn log_err<S: Into<String>>(self, message: S, level: LogLevel) -> Self {
        let msg: String = message.into();

        self.map_err(|e| {
            match level {
                LogLevel::Debug => {
                    debug!("{}; error: {}", msg, e)
                }
                LogLevel::Info => {
                    info!("{}; error: {}", msg, e)
                }
                LogLevel::Warning => {
                    warn!("{}; error: {}", msg, e)
                }
                LogLevel::Error => {
                    error!("{}; error: {}", msg, e)
                }
            }
            e
        })
    }
}
