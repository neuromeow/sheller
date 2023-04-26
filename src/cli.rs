use std::ffi::OsString;
use std::fmt;
use std::ops::Range;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Build {
        history_file: Option<OsString>,

        #[arg(short, long)]
        output_file: Option<OsString>,

        #[arg(short, long, value_enum, default_value_t = Interpreter::Bash, conflicts_with = "no_header")]
        interpreter: Interpreter,

        #[arg(short, long, default_value_t = String::from("Script Description"), conflicts_with = "no_header")]
        description: String,

        #[arg(long)]
        no_header: bool,

        #[arg(short, long, group = "specified_lines")]
        #[arg(value_parser = parse_specified_lines, use_value_delimiter = true, value_delimiter = ',')]
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

fn parse_specified_lines(
    line_number_or_lines_range: &str,
) -> Result<Range<u32>, std::num::ParseIntError> {
    if line_number_or_lines_range.contains("..") {
        let mut line_range_split = line_number_or_lines_range.split("..");
        let lines_range_start_parsed = line_range_split.next().unwrap().parse::<u32>()?;
        let lines_range_end_parsed = line_range_split.last().unwrap().parse::<u32>()?;
        Ok(lines_range_start_parsed..lines_range_end_parsed)
    } else {
        let line_number_parsed = line_number_or_lines_range.parse::<u32>()?;
        Ok(line_number_parsed..line_number_parsed + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_specified_lines_should_parse_correctly() {
        let values = [
            ("0", 0..1),
            ("0..1", 0..1),
            ("0..3", 0..3),
            ("1..0", 1..0),
            ("2..0", 2..0),
            ("1", 1..2),
            ("1..2", 1..2),
            ("1..5", 1..5),
            ("11..24", 11..24),
            ("2..1", 2..1),
            ("6..1", 6..1),
            ("18..7", 18..7),
        ];
        for (input_value, expected_value) in values {
            let actual_value = parse_specified_lines(input_value).unwrap();
            assert_eq!(actual_value, expected_value);
        }
    }

    #[test]
    fn parse_specified_returns_error_when_string_cannot_be_parsed() {
        let values = [
            "zero", "1..", "..5", "12..", "..", " 6", "9 ", " 17 ", " ", "",
        ];
        for value in values {
            assert!(parse_specified_lines(value).is_err());
        }
    }
}
