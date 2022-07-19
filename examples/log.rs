#[macro_use]
extern crate ltrs;
#[macro_use]
extern crate prettytable;

use ltrs::utils::logging::*;

use log::{debug, error, info, trace, warn};
use prettytable::{
    format::{self, TableFormat},
    Table,
};

fn main() {
    init_logger();

    debug!("{}", "Debug message");
    info!("{}", "Info message");
    warn!("{}", "Warning message");
    error!("{}", "Error message");
    trace!("{}", "Trace message");

    let mut table = Table::new();

    let r = row!["aa", "bb", "cc"];

    // String that implements Write    let r = row!["Title 1", "Title 2"];
    info!("\n{}", table_header(row!["Title 1", "Title 2"]));
}
