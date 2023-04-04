use std::ffi::OsString;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Build {
        history_file: OsString,
        #[arg(short, long, required = true)]
        line: u32,
    },
}
