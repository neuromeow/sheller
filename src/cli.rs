use std::ffi::OsString;
use std::fmt;
use std::ops::Range;

use clap::{Parser, Subcommand, ValueEnum};

use crate::names_generator::get_random_name;

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

        #[arg(short, long, default_value_os_t = get_output_file_name())]
        output: OsString,

        #[arg(short, long, value_enum, default_value_t = Interpreter::Bash)]
        interpreter: Interpreter,

        #[arg(short, long, default_value_t = String::from("Script Description"))]
        description: String,

        #[arg(short, long, group = "specified_lines")]
        #[arg(value_parser = parse_passed_lines, use_value_delimiter = true, value_delimiter = ',')]
        lines: Vec<Range<u32>>,

        #[arg(short, long, requires = "specified_lines")]
        force: bool,

        #[arg(long)]
        reverse: bool,

        #[arg(long, requires = "specified_lines")]
        reverse_inner: bool,
    },
}

#[derive(Clone, Debug, ValueEnum)]
pub enum Interpreter {
    // Bourne Shell (sh)
    Sh,
    // C Shell (csh)
    Csh,
    // TENEX C Shell (tcsh)
    Tcsh,
    // KornShell (ksh)
    Ksh,
    // Debian Almquist Shell (dash)
    Dash,
    // Bourne Again Shell (bash)
    Bash,
    // Z Shell (zsh)
    Zsh,
    // Friendly Interactive Shell (fish)
    Fish,
}

impl fmt::Display for Interpreter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

fn get_output_file_name() -> OsString {
    loop {
        let random_basename = get_random_name() + ".sh";
        if !std::path::Path::new(&random_basename).exists() {
            break OsString::from(random_basename);
        }
    }
}

fn parse_passed_lines(line_number_or_range: &str) -> Result<Range<u32>, std::num::ParseIntError> {
    if line_number_or_range.contains("..") {
        let mut line_range_split = line_number_or_range.split("..");
        let line_range_start = line_range_split.next().unwrap();
        let line_range_start_parsed = line_range_start.parse::<u32>()?;
        let line_range_end = line_range_split.last().unwrap();
        let line_range_end_parsed = line_range_end.parse::<u32>()?;
        Ok(line_range_start_parsed..line_range_end_parsed)
    } else {
        let line_number_parsed = line_number_or_range.parse::<u32>()?;
        Ok(line_number_parsed..line_number_parsed + 1)
    }
}
