pub mod error;
pub mod handler;
mod help;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(
	author,
	version,
	about,
	override_help = help::HELP
)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

impl Cli {
    ///
    /// Starts the CLI by calling the parse trait.
    pub fn start() -> Self {
        Cli::parse()
    }
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    // todo: wip
    Fit,
}
