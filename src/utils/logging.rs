/// Copyright (c) 2021 Marcos Pontes
/// MIT License
///
use std::{io::{ErrorKind, Write}, str::FromStr};

use colored::{ColoredString, Colorize};
use env_logger;
use log::Level;

use prettytable::{
    color,
    format::{self, TableFormat},
    row::Row,
    Attr,
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

///
/// Receives a row and produces a header str.
///
/// Example:
/// ```
/// let header = log_header([row!["Title 1", "Title 2"]]);
/// ```
pub fn table_header(row: Row) -> String {
    let f = format::FormatBuilder::new()
        .column_separator('|')
        .borders('|')
        .separators(
            &[format::LinePosition::Top, format::LinePosition::Bottom],
            format::LineSeparator::new('-', '+', '+', '+'),
        )
        .padding(2, 2)
        .build();

    let mut writer = StringWriter::new();

    let height: usize = row.get_height();
    let length: usize = row.len();

    // &[height, height, ...] length times
    let heights: Vec<usize> = (0..length).map(|_| 1).collect();

    f.print_line_separator(&mut writer, &heights, format::LinePosition::Top)
        .unwrap();
    row.print(&mut writer, &f, &heights).unwrap();
    f.print_line_separator(&mut writer, &heights, format::LinePosition::Bottom)
        .unwrap();
    String::from_str(writer.as_string()).unwrap()
}

pub fn table_log(msg: Vec<&str>) -> String {
    todo!()
}

macro_rules! log_row {
    () => {};
}
