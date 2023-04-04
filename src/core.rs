use std::error::Error;

use clap::Parser;

use crate::cli::{Cli, Commands};

pub fn run() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Build { history_file, line } => {
            println!("Command 'build' was used.");
            println!("History file: {:?}", history_file);
            println!("Line number: {:?}", line);
        }
    }
    Ok(())
}
