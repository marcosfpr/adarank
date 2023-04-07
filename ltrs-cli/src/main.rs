use ltr::{cli, utils};

fn main() {
    utils::logging::setup_backtrace();

    let dmt_cli = cli::Cli::start();

    if let Err(err) = cli::handler::run(dmt_cli) {
        cli::error::exit_with_error(err)
    }
}
