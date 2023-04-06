use std::ffi::OsString;
use std::ops::Range;

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

#[allow(dead_code)]
fn parse_passed_lines(line_number_or_range: &str) -> Result<Range<u32>, std::num::ParseIntError> {
    let mut line_number_or_range_split = line_number_or_range.split("..");
    let line_range_start = line_number_or_range_split.next().unwrap();
    let line_range_start_parsed = line_range_start.parse::<u32>()?;
    let line_range_end = line_number_or_range_split.last().unwrap_or(line_range_start);
    let line_range_end_parsed = line_range_end.parse::<u32>()?;
    Ok(line_range_start_parsed..line_range_end_parsed)
}
