use std::backtrace::Backtrace;
use std::convert::Infallible;
use std::io;
use std::ops::FromResidual;

use crate::utils::fs::FindFileError;
use crate::utils::logging::{log_backtrace, log_multiline_error};
use ltrs::error::LtrError;
use thiserror::Error;
use tracing_subscriber::util::TryInitError;

///
/// Handles default errors that can occur if the CLI commands fail.
#[derive(Error, Debug)]
pub enum CliError {
    #[error("An error has occurred: {0}")]
    Error(String),
    #[error("An internal error has occurred: {0}")]
    InternalError(#[from] LtrError),
    #[error("An I/O error has occurred: {error}")]
    IoError {
        #[from]
        error: std::io::Error,
        backtrace: Backtrace,
    },
    #[error("Failed to initiliaze the logs: {error}")]
    LogError {
        #[from]
        error: LogError,
        backtrace: Backtrace,
    },
    #[error("{error}")]
    InvalidFileError {
        #[from]
        error: FindFileError,
        backtrace: Backtrace,
    },
}

#[derive(Debug, Error)]
pub enum LogError {
    #[error("Failed to initialize logging file: {0}")]
    IoError(#[from] io::Error),
    #[error(transparent)]
    InitError(#[from] TryInitError),
}

pub fn exit_with_error(err: self::CliError) {
    log_multiline_error(err.to_string());

    match err {
        self::CliError::InvalidFileError { backtrace, .. }
        | self::CliError::IoError { backtrace, .. }
        | self::CliError::LogError { backtrace, .. } => {
            log_backtrace(backtrace);
        }
        _ => (),
    }

    std::process::exit(1)
}

// Convert a `Result<Infallible, E>` directly to a `Failure` variant of
// `ApiResponse` when `E` implements `Into<VmeError>`.
impl<E> FromResidual<Result<Infallible, E>> for CliError
where
    E: Into<LtrError>,
{
    fn from_residual(residual: Result<Infallible, E>) -> Self {
        match residual {
            Ok(_) => unreachable!("`Infallible` is always unreachable"),
            Err(e) => CliError::InternalError(e.into()),
        }
    }
}
