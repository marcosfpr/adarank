/// Copyright (c) 2021 Marcos Pontes
/// MIT License
///
use std::str::FromStr;

use fern::{log_file, Dispatch, InitError};
use log::{debug, error, info, trace, warn};
// use fern::colors::{Color, ColoredLevelConfig};
use chrono::Local;

///
/// The default debug logging configuration for lt.rs
///
fn build_file_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%H:%M:%S]"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("ltr.log")?)
        .apply()?;
    Ok(())
}

///
/// The default trace logging configuration for lt.rs
///
fn build_stdout_logger(level: log::LevelFilter) -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%H:%M:%S]"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(level)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

/// Initializes the logger based on the environment variable `LOG_LEVEL`.
/// If the environment variable is not set, the default level is `info`.
///
fn init_logger() -> Result<(), fern::InitError> {
    let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    let log_level =
        log::LevelFilter::from_str(&log_level).unwrap_or_else(|_| log::LevelFilter::Info);
    match log_level {
        log::LevelFilter::Trace => build_stdout_logger(log::LevelFilter::Trace),
        log::LevelFilter::Debug => build_stdout_logger(log::LevelFilter::Debug),
        log::LevelFilter::Info => build_stdout_logger(log::LevelFilter::Info),
        log::LevelFilter::Warn => build_stdout_logger(log::LevelFilter::Warn),
        log::LevelFilter::Error => build_stdout_logger(log::LevelFilter::Error),
        log::LevelFilter::Off => build_stdout_logger(log::LevelFilter::Off),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setup_logger() {
        init_logger();

        debug!("Debug message");
    }
}
