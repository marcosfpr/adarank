use ltrs::utils::logging::{init_logging, Alignment, TableConfig, TableLogger};

use log::{debug, error, info, trace, warn};

fn main() {
    init_logging();

    debug!("{}", "Debug message");
    info!("{}", "Info message");
    warn!("{}", "Warning message");
    error!("{}", "Error message");
    trace!("{}", "Trace message");

    let config = TableConfig::new(vec![7, 8, 9, 9, 9, 9, 9], (2, 2), Alignment::Center);

    let logger = TableLogger::new(config);
    log::debug!(
        "{}",
        logger.log(
            vec![
                "#Iter",
                "Feature",
                "MAP-T",
                "Improve-T",
                "MAP-V",
                "Improve-V",
                "Status",
            ],
            None
        )
    );
}
