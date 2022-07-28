/// Copyright (c) 2021 Marcos Pontes
/// MIT License
///
use std::{
    io::{ErrorKind, Write},
    str::FromStr,
};

use colored::{ColoredString, Colorize};
use env_logger;
use log::Level;

use super::format::{
    consts::{DEFAULT_INNER_TABLE_LOGGER, DEFAULT_TABLE_LOGGER},
    TableConfig,
};

/// Internal utility for writing data into a string
pub struct StringWriter {
    string: String,
}

impl StringWriter {
    /// Create a new `StringWriter`
    pub fn new() -> StringWriter {
        StringWriter {
            string: String::new(),
        }
    }

    /// Return a reference to the internally written `String`
    pub fn as_string(&self) -> &str {
        &self.string
    }
}

impl Write for StringWriter {
    fn write(&mut self, data: &[u8]) -> Result<usize, std::io::Error> {
        let string = match std::str::from_utf8(data) {
            Ok(s) => s,
            Err(e) => {
                return Err(std::io::Error::new(
                    ErrorKind::Other,
                    format!("Cannot decode utf8 string : {}", e),
                ))
            }
        };
        self.string.push_str(string);
        Ok(data.len())
    }

    fn flush(&mut self) -> Result<(), std::io::Error> {
        // Nothing to do here
        Ok(())
    }
}

/// Internal utility for writing data into a string.
/// This value is totally dependant on the `init_logger` Formatter implementation.
const LOG_PREFIX_SIZE: usize = 28;

fn shift_prefix(writer: &mut StringWriter) {
    // Write LOG_PREFIX_SIZE spaces to the beginning of the line
    for _ in 0..LOG_PREFIX_SIZE {
        writer.write(b" ").unwrap();
    }
}

///
/// Initializes the logger with the environment variable `RUST_LOG`.
/// If the variable is not set, the default logger (err) is used.
///
pub fn init_logger() {
    env_logger::builder()
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}] [{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                color_by_level(record.level()),
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
        Level::Error => "ERR".bold().red(),
        Level::Warn => "WAR".yellow(),
        Level::Info => "INF".blue(),
        Level::Debug => "DBG".green(),
        Level::Trace => "TRC".magenta(),
    }
}

///
/// Utility function that generates a log header based on the `TableConfig` specifications.
///
pub fn log_table_header(header: Vec<&str>, config: &TableConfig) -> String {
    let mut writer = StringWriter::new();

    DEFAULT_TABLE_LOGGER
        .log_separator_with_config(&mut writer, &config, true)
        .unwrap();

    shift_prefix(&mut writer);

    DEFAULT_INNER_TABLE_LOGGER
        .log_value_with_config(
            &mut writer,
            header.iter().map(|h| h.bold().cyan()).collect(),
            &config,
        )
        .unwrap();

    shift_prefix(&mut writer);

    DEFAULT_TABLE_LOGGER
        .log_separator_with_config(&mut writer, &config, false)
        .unwrap();

    String::from_str(writer.as_string()).unwrap()
}

///
/// Utility function that generates a log row based on the `TableConfig` specifications.
/// It appends a line separator after the row.
///
pub fn log_table_row<F: ToString>(row: Vec<F>, config: &TableConfig) -> String {
    let mut writer = StringWriter::new();

    DEFAULT_INNER_TABLE_LOGGER
        .log_value_with_config(&mut writer, row, &config)
        .unwrap();

    shift_prefix(&mut writer);

    DEFAULT_TABLE_LOGGER
        .log_separator_with_config(&mut writer, &config, false)
        .unwrap();

    String::from_str(writer.as_string()).unwrap()
}

///
/// Utility function that generates a log row based on the `TableConfig` specifications.
/// It appends a line separator after the row.
///
pub fn log_shifted_table_row<F: ToString>(row: Vec<F>, config: &TableConfig) -> String {
    let mut writer = StringWriter::new();
    
    shift_prefix(&mut writer);

    writer.write_all(log_table_row(row, config).as_bytes()).unwrap();

    String::from_str(writer.as_string()).unwrap()
}
