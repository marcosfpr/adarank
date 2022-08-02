use colored::{ColoredString, Colorize};
use ltrs::utils::{
    format::{consts::DEFAULT_INNER_TABLE_LOGGER, Alignment, TableConfig, TableLogger},
    logging::{self, *},
};

use log::{debug, error, info, trace, warn};

fn main() {
    init_logger();

    debug!("{}", "Debug message");
    info!("{}", "Info message");
    warn!("{}", "Warning message");
    error!("{}", "Error message");
    trace!("{}", "Trace message");

    let config = TableConfig::new(
        vec![7, 8, 9, 9, 9, 9, 9],
        (2, 2),
        Alignment::Center,
        true,
        true,
        true,
    );

    log::debug!(
        "{}",
        logging::log_table_header(
            vec![
                "#Iter",
                "Feature",
                "MAP-T",
                "Improve-T",
                "MAP-V",
                "Improve-V",
                "Status",
            ],
            &config,
        )
    );
}
