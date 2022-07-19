#[macro_use] extern crate ltrs;

use ltrs::utils::logging::*;

use log::{debug, error, info, trace, warn};

fn main() {
    
    init_logger();

    debug!("{}", "Debug message");
    info!("{}", "Info message");
    warn!("{}", "Warning message");
    error!("{}", "Error message");
    trace!("{}", "Trace message");

}
