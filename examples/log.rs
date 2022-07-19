#[macro_use]
extern crate ltrs;

use ltrs::utils::{
    format::{Alignment, TableLogger},
    logging::*,
};

use log::{debug, error, info, trace, warn};

fn main() {
    init_logger();

    debug!("{}", "Debug message");
    info!("{}", "Info message");
    warn!("{}", "Warning message");
    error!("{}", "Error message");
    trace!("{}", "Trace message");

    let table = TableLogger::default();
    let mut writer = StringWriter::new();

    table
        .log_separator(&mut writer, &[10, 8, 5], (2, 2), true, true, true)
        .unwrap();

    table
        .log_value(
            &mut writer,
            vec!["foo", "bar", "baz"],
            &[10, 8, 5],
            (2, 2),
            Alignment::Center,
            true,
            true,
            true,
        )
        .unwrap();

    table
        .log_separator(&mut writer, &[10, 8, 5], (2, 2), true, true, true)
        .unwrap();

    info!("\n{}", writer.as_string());
}
