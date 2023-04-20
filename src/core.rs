use std::error::Error;

use clap::Parser;

use crate::cli::{Cli, Commands};
use crate::util::build_script_file;
use crate::util::print_passed_parameters;

pub fn run() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Build { history_file, output, interpreter, description, lines, force, reverse, reverse_inner } => {
            print_passed_parameters(history_file, output, interpreter, description, lines, force, reverse, reverse_inner)?;
            build_script_file(history_file, output, interpreter, description, lines, force, reverse)?;
        }
    }
    Ok(())
}
