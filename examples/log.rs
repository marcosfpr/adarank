use ltrs::utils::{
    format::{Alignment, TableConfig},
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

    let config = TableConfig::new(vec![10, 8, 5], (2, 2), Alignment::Center, true, true, true);

    info!("{}", log_table_header(vec!["foo", "bah", "zoo"], &config));
    debug!("{}", log_table_row(vec![1, 2, 3], &config));
    debug!("{}", log_table_row(vec![4, 5, 63], &config));
    info!("{}", log_table_row(vec![1, 22, 33], &config));
    info!("{}", log_table_row(vec![11, 200, 3], &config));
}

