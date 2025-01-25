use std::{
    io::{Error, ErrorKind, Write},
    iter::repeat,
    str::FromStr,
};

use colored::{Color, Colorize};

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

/// Useful configs to render tables
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableConfig {
    /// Column widths
    pub col_width: Vec<usize>,
    /// Left and right padding factors
    pub padding: (usize, usize),
    /// Alignment strategy
    pub align: Alignment,
}

impl TableConfig {
    /// Create a `TableConfig` based on its arguments
    pub fn new(col_width: Vec<usize>, padding: (usize, usize), align: Alignment) -> TableConfig {
        TableConfig {
            col_width,
            padding,
            align,
        }
    }
}

/// Table print simplified
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Table {
    config: TableConfig,
}

impl Table {
    /// Create a table based on the configurations.
    pub fn new(config: TableConfig) -> Table {
        Table { config }
    }

    /// Return the display width of a unicode string.
    /// This functions takes ANSI-escaped color codes into account.
    pub fn display_width(text: &str) -> usize {
        let width = text.len();
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
        let text_len = Table::display_width(text);
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

    /// Print a value to `out`. `col_width` is a slice containing the width of each column.
    fn render_value_inner<T: Write + ?Sized, F: ToString>(
        &self,
        out: &mut T,
        value: Vec<F>,
        col_width: Vec<usize>,
        padding: (usize, usize),
        align: Alignment,
        colored: bool,
    ) -> Result<usize, Error> {
        assert_eq!(value.len(), col_width.len());

        let col_width = if colored {
            col_width.iter().map(|x| x + 2).collect()
        } else {
            col_width
        };

        let mut iter = value.iter().zip(col_width.iter()).peekable();

        while let Some((v, w)) = iter.next() {
            Table::align(out, align, &v.to_string(), ' ', *w + padding.0 + padding.1)?;
        }

        Ok(1)
    }

    /// Render a value to `out` based on a `TableConfig` specification.
    fn render_value<T: Write + ?Sized, F: ToString>(
        &self,
        out: &mut T,
        value: Vec<F>,
        colored: bool,
    ) -> Result<usize, Error> {
        self.render_value_inner(
            out,
            value,
            self.config.col_width.clone(),
            self.config.padding,
            self.config.align,
            colored,
        )
    }

    /// Return a formatted string for a set of values
    pub fn render(&self, values: Vec<&str>, color: Option<Color>) -> String {
        let mut writer = StringWriter::new();

        if let Some(c) = color {
            let values = match c {
                Color::Blue => values.iter().map(|v| v.bold().blue()).collect(),
                Color::Cyan => values.iter().map(|v| v.bold().blue()).collect(),
                Color::Red => values.iter().map(|v| v.bold().red()).collect(),
                Color::Green => values.iter().map(|v| v.bold().green()).collect(),
                _ => values.iter().map(|v| v.bold()).collect(),
            };

            self.render_value(&mut writer, values, true).unwrap();
        } else {
            self.render_value(&mut writer, values, false).unwrap();
        }

        String::from_str(writer.as_string()).unwrap()
    }
}

/// Internal utility for writing data into a string
struct StringWriter {
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
