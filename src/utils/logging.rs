/// Copyright (c) 2021 Marcos Pontes
/// MIT License
///
use std::io::Write;

use log::Level;
use colored::{ColoredString, Colorize};
use env_logger;

use crate::utils::table_format;


///
/// Initializes the logger with the environment variable `RUST_LOG`.
/// If the variable is not set, the default logger (err) is used.
///
pub fn init_logger() {
    env_logger::builder()
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}] [{}] [{}] [{}:{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                color_by_level(record.level()),
                record.module_path().unwrap_or(""),
                record.file().unwrap_or(""),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();
}


///
/// Colorize the record level.
///
fn color_by_level(level: Level) -> ColoredString {
    match level {
        Level::Error => "Error".red(),
        Level::Warn => "Warning".yellow(),
        Level::Info => "Info".blue(),
        Level::Debug => "Debug".green(),
        Level::Trace => "Trace".magenta(),
    }
}

fn table_log(msg: Vec<&str>) {
    todo!()
}

macro_rules! log_row {
    () => {
        
    };
}

macro_rules! log_header {
    () => {
        
    };
}