use crate::utils::logging;

use super::error::CliError;
use super::Cli;

///
/// Handles all the CLI available commands by calling
/// the corresponding command services.
pub fn run(cli: Cli) -> Result<(), CliError> {
    logging::init_logger();

    match cli.command {
        super::Commands::Fit => todo!(),
    }
}
