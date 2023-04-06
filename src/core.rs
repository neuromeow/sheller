use std::error::Error;

use clap::Parser;

use crate::cli::{Cli, Commands};
// use crate::util::build_script_file;
use crate::util::print_passed_parameters;

pub fn run() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Build { history_file, line } => {
            print_passed_parameters(*line, history_file)?;
            // build_script_file(*line, history_file)?;
        }
    }
    Ok(())
}
