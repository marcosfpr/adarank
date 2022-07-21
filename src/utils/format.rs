/// Copyright (c) 2021 Marcos Pontes
/// MIT License
///
///
/// Inspired by the following project: prettytable-rs
use std::{
    io::{Error, ErrorKind, Write},
    iter::repeat,
};

use encode_unicode::Utf8Char;
use unicode_width::UnicodeWidthStr;

#[cfg(any(not(windows), not(feature = "win_crlf")))]
pub static NEWLINE: &[u8] = b"\n";
#[cfg(all(windows, feature = "win_crlf"))]
pub static NEWLINE: &[u8] = b"\r\n";

/// Alignment for tables content
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Alignment {
    /// Align to the left
    Left,
    /// Align to the right
    Right,
    /// Align to the center
    Center,
}

/// Useful configs to logging tables
pub struct TableConfig {
    /// Column widths
    pub col_width: Vec<usize>,
    /// Left and right padding factors
    pub padding: (usize, usize),
    /// Alignment strategy
    pub align: Alignment,
    /// True if the column will be separated
    pub colsep: bool,
    /// True if the left border will be separated
    pub lborder: bool,
    /// True if the right border will be separated
    pub rborder: bool,
}

impl TableConfig {
    /// Create a `TableConfig` based on its arguments
    pub fn new(
        col_width: Vec<usize>,
        padding: (usize, usize),
        align: Alignment,
        colsep: bool,
        lborder: bool,
        rborder: bool,
    ) -> TableConfig {
        TableConfig {
            col_width,
            padding,
            align,
            colsep,
            lborder,
            rborder,
        }
    }

    /// Create a `TableConfig` based on the header values
    pub fn from_header(
        header: &[&str],
        padding: (usize, usize),
        align: Alignment,
        colsep: bool,
        lborder: bool,
        rborder: bool,
    ) -> TableConfig {
        let col_width = header.iter().map(|s| s.width()).collect();
        TableConfig::new(col_width, padding, align, colsep, lborder, rborder)
    }
}

/// Table print simplified
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TableLogger {
    /// Line separator character
    line: char,
    /// Internal junction separator
    junc: char,
    /// Left junction separator
    ljunc: char,
    /// Right junction separator
    rjunc: char,
}

impl TableLogger {
    /// Create a new line separator instance where `line` is the character used to separate 2 lines
    /// and `junc` is the one used for junctions between columns and lines
    pub fn new(line: char, junc: char, ljunc: char, rjunc: char) -> TableLogger {
        TableLogger {
            line,
            junc,
            ljunc,
            rjunc,
        }
    }

    /// Return the display width of a unicode string.
    /// This functions takes ANSI-escaped color codes into account.
    pub fn display_width(text: &str) -> usize {
        let width = UnicodeWidthStr::width(text);
        let mut state = 0;
        let mut hidden = 0;

        for c in text.chars() {
            state = match (state, c) {
                (0, '\u{1b}') => 1,
                (1, '[') => 2,
                (1, _) => 0,
                (2, 'm') => 3,
                _ => state,
            };

            // We don't count escape characters as hidden as
            // UnicodeWidthStr::width already considers them.
            if state > 1 {
                hidden += 1;
            }

            if state == 3 {
                state = 0;
            }
        }

        width - hidden
    }

    /// Align the given string to the given alignment
    fn align<T: Write + ?Sized>(
        out: &mut T,
        align: Alignment,
        text: &str,
        fill: char,
        size: usize,
    ) -> Result<(), Error> {
        let text_len = TableLogger::display_width(text);
        let mut nfill = if text_len < size { size - text_len } else { 0 };

        let n = match align {
            Alignment::Left => 0,
            Alignment::Right => nfill,
            Alignment::Center => nfill / 2,
        };

        if n > 0 {
            out.write_all(repeat(fill).take(n).collect::<String>().as_bytes())?;
            nfill -= n;
        }

        out.write_all(text.as_bytes())?;

        if nfill > 0 {
            out.write_all(repeat(fill).take(nfill).collect::<String>().as_bytes())?;
        }

        Ok(())
    }

    /// Print a full line separator to `out`. `col_width` is a slice containing the width of each column.
    /// Returns the number of printed lines
    pub fn log_separator<T: Write + ?Sized>(
        &self,
        out: &mut T,
        col_width: &[usize],
        padding: (usize, usize),
        colsep: bool,
        lborder: bool,
        rborder: bool,
        newline: bool,
    ) -> Result<usize, Error> {
        if lborder {
            out.write_all(Utf8Char::from(self.ljunc).as_bytes())?;
        }
        let mut iter = col_width.iter().peekable();

        while let Some(w) = iter.next() {
            for _ in 0..w + padding.0 + padding.1 {
                out.write_all(Utf8Char::from(self.line).as_bytes())?;
            }
            if colsep && iter.peek().is_some() {
                out.write_all(Utf8Char::from(self.junc).as_bytes())?;
            }
        }
        if rborder {
            out.write_all(Utf8Char::from(self.rjunc).as_bytes())?;
        }

        if newline {
            out.write_all(NEWLINE)?;
        }

        Ok(1)
    }

    pub fn log_separator_with_config<T: Write + ?Sized>(
        &self,
        out: &mut T,
        config: &TableConfig,
        newline: bool,
    ) -> Result<usize, Error> {
        self.log_separator(
            out,
            &config.col_width,
            config.padding,
            config.colsep,
            config.lborder,
            config.rborder,
            newline,
        )
    }

    /// Print a value to `out`. `col_width` is a slice containing the width of each column.
    pub fn log_value<T: Write + ?Sized, F: ToString>(
        &self,
        out: &mut T,
        value: Vec<F>,
        col_width: &[usize],
        padding: (usize, usize),
        align: Alignment,
        colsep: bool,
        lborder: bool,
        rborder: bool,
    ) -> Result<usize, Error> {
        if lborder {
            out.write_all(Utf8Char::from(self.ljunc).as_bytes())?;
        }

        assert_eq!(value.len(), col_width.len());

        let mut iter = value.iter().zip(col_width.iter()).peekable();

        while let Some((v, w)) = iter.next() {
            let has_next: bool = iter.peek().is_some();
            TableLogger::align(out, align, &v.to_string(), ' ', *w + padding.0 + padding.1)?;
            if colsep && has_next {
                out.write_all(Utf8Char::from(self.junc).as_bytes())?;
            }
        }

        if rborder {
            out.write_all(Utf8Char::from(self.rjunc).as_bytes())?;
        }

        out.write_all(NEWLINE)?;
        Ok(1)
    }

    pub fn log_value_with_config<T: Write + ?Sized, F: ToString>(
        &self,
        out: &mut T,
        value: Vec<F>,
        config: &TableConfig,
    ) -> Result<usize, Error> {
        self.log_value(
            out,
            value,
            &config.col_width,
            config.padding,
            config.align,
            config.colsep,
            config.lborder,
            config.rborder,
        )
    }
}

impl Default for TableLogger {
    fn default() -> Self {
        TableLogger::new('-', '+', '+', '+')
    }
}

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
    fn write(&mut self, data: &[u8]) -> Result<usize, Error> {
        let string = match std::str::from_utf8(data) {
            Ok(s) => s,
            Err(e) => {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("Cannot decode utf8 string : {}", e),
                ))
            }
        };
        self.string.push_str(string);
        Ok(data.len())
    }

    fn flush(&mut self) -> Result<(), Error> {
        // Nothing to do here
        Ok(())
    }
}

pub mod consts {

    lazy_static! {

        /// The default TableLogger
        pub static ref DEFAULT_TABLE_LOGGER: super::TableLogger = super::TableLogger::default();

        /// The default TableLogger for inner values
        pub static ref DEFAULT_INNER_TABLE_LOGGER: super::TableLogger = super::TableLogger::new('-', '|', '+', '+');

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_formatter() {
        let mut writer = StringWriter::new();

        let sep = TableLogger::default();

        sep.log_separator(&mut writer, &[10, 8, 5], (2, 2), true, true, true, true)
            .unwrap();

        assert_eq!(
            writer.as_string(),
            "+--------------+------------+---------+\n"
        );

        sep.log_value(
            &mut writer,
            vec!["foo", "bar", "baz"],
            &[10, 8, 5],
            (2, 2),
            Alignment::Left,
            true,
            true,
            true,
        )
        .unwrap();

        assert_eq!(
            writer.as_string(),
            "+--------------+------------+---------+\n+foo           +bar         +baz      +\n"
        );
    }
}
