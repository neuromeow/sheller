use std::error::Error;

use clap::Parser;

use crate::cli::{Cli, Commands};
use crate::util::build_script_file_with_multiple_line_ranges;
use crate::util::print_passed_parameters;

pub fn run() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Build { history_file, lines, force } => {
            print_passed_parameters(lines, history_file, force)?;
            build_script_file_with_multiple_line_ranges(lines, history_file)?;
        }
    }
    Ok(())
}
