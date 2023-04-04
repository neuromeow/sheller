use std::error::Error;

use clap::Parser;

use crate::cli::{Cli, Commands};
use crate::util::build_script;

pub fn run() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Build { history_file, line } => {
            build_script(*line, history_file.to_str().unwrap())?;
        }
    }
    Ok(())
}
