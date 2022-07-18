#[macro_use] extern crate prettytable;
#[macro_use] extern crate ltrs;

use ltrs::utils::logging::*;

use log::{debug, error, info, trace, warn};
use prettytable::{Table, Row, Cell};

fn main() {
    
    init_logger();

    debug!("{}", "Debug message");
    info!("{}", "Info message");
    warn!("{}", "Warning message");
    error!("{}", "Error message");
    trace!("{}", "Trace message");

     // Create the table
    let mut table = create_table();

     // Add a row per time
     table.add_row(row!["ABC", "DEFG", "HIJKLMN"]);
     table.add_row(row!["foobar", "bar", "foo"]);
     // A more complicated way to add a row:
     table.add_row(Row::new(vec![
         Cell::new("foobar2"),
         Cell::new("bar2"),
         Cell::new("foo2")]));
 
     // Print the table to stdout
     info!("\n{}", table.to_string())

}
