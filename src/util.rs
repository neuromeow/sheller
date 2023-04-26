use std::collections::HashMap;
use std::error::Error;
use std::ffi::OsString;
use std::fs::{File, OpenOptions};
use std::io::{stdin, BufRead, BufReader, BufWriter, Write};
use std::ops::Range;
use std::os::unix::fs::OpenOptionsExt;

use atty::Stream;

use crate::cli::Interpreter;

fn read_lines_from_file_or_stdin(
    file_pathname_or_none: &Option<OsString>,
) -> Result<Vec<String>, Box<dyn Error>> {
    let lines = match file_pathname_or_none {
        Some(file_pathname) => {
            let file = File::open(file_pathname)?;
            let file_bufreader = BufReader::new(file);
            file_bufreader.lines().map(|line| line.unwrap()).collect()
        }
        None => {
            // Solves the problem of hanging when standard input is not passed
            if atty::is(Stream::Stdin) {
                Vec::new()
            } else {
                let stdin = stdin();
                stdin.lock().lines().map(|line| line.unwrap()).collect()
            }
        }
    };
    Ok(lines)
}

fn create_script_file_box_writer(
    script_file_pathname_or_none: &Option<OsString>,
) -> Result<Box<dyn Write>, Box<dyn Error>> {
    let script_file_writer: Box<dyn Write> = match script_file_pathname_or_none {
        Some(script_file_pathname) => {
            let script_file_options = OpenOptions::new()
                .append(true)
                .create_new(true)
                .mode(0o744)
                .open(script_file_pathname)?;
            Box::new(BufWriter::new(script_file_options))
        }
        None => Box::new(std::io::stdout()),
    };
    Ok(script_file_writer)
}

fn create_hashmap_from_ranges(ranges: &Vec<Range<u32>>) -> HashMap<u32, Option<String>> {
    let mut hashmap = HashMap::new();
    for range in ranges {
        let cloned_reversed_or_original_range = if range.start > range.end {
            range.end + 1..range.start + 1
        } else {
            range.clone()
        };
        for number in cloned_reversed_or_original_range {
            hashmap.insert(number, None);
        }
    }
    hashmap
}

fn update_hashmap_by_lines(hashmap: &mut HashMap<u32, Option<String>>, lines: Vec<String>) {
    for (index, line) in lines.into_iter().enumerate() {
        let line_number = (index + 1) as u32;
        if hashmap.contains_key(&line_number) {
            hashmap.insert(line_number, Some(line));
        }
    }
}

fn update_script_file_bufwriter_header(
    script_file_bufwriter: &mut Box<dyn Write>,
    interpreter: &Interpreter,
    description: &String,
) -> Result<(), Box<dyn Error>> {
    let mut header = format!("#!/bin/env {}\n#\n", interpreter);
    let header_description = format!("# {}\n#\n\n", description);
    header.push_str(&header_description);
    script_file_bufwriter.write_all(header.as_bytes())?;
    Ok(())
}

fn update_script_file_bufwriter_body_by_lines(
    script_file_bufwriter: &mut Box<dyn Write>,
    lines: &mut Vec<String>,
    reverse_flag: &bool,
) -> Result<(), Box<dyn Error>> {
    if *reverse_flag == true {
        lines.reverse();
    }
    for line in lines {
        let body_line = line.to_owned() + "\n";
        script_file_bufwriter.write_all(body_line.as_bytes())?;
    }
    Ok(())
}

fn update_script_file_bufwriter_body_by_hashmap(
    script_file_bufwriter: &mut Box<dyn Write>,
    hashmap: HashMap<u32, Option<String>>,
    ranges: &Vec<Range<u32>>,
    reverse_flag: &bool,
    reverse_inner_flag: &bool,
) -> Result<(), Box<dyn Error>> {
    let mut reversed_ranges_or_original = ranges.clone();
    if *reverse_flag == true {
        reversed_ranges_or_original.reverse();
    }
    for range in reversed_ranges_or_original {
        let mut cloned_reversed_or_original_range: Vec<u32> = if range.start > range.end {
            (range.end + 1..range.start + 1).rev().collect()
        } else {
            range.clone().collect()
        };
        // Boolean conditions for two types of ranges: from lower to higher and from higher to lower.
        let is_possible_to_reverse = if range.end > range.start {
            range.end - range.start > 1
        } else {
            range.start - range.end > 1
        };
        if *reverse_inner_flag == true && is_possible_to_reverse {
            cloned_reversed_or_original_range.reverse();
        }
        // If the range contains only one number, then it is no possible to reverse it.
        // Because the boundaries are already fixed and the hashmap with the lines is already collected.
        for number in cloned_reversed_or_original_range.clone() {
            match hashmap.get(&number).unwrap() {
                Some(v) => {
                    let body_line = v.clone() + "\n";
                    script_file_bufwriter.write_all(body_line.as_bytes())?;
                }
                _ => continue,
            }
        }
    }
    Ok(())
}

pub fn build_script_file(
    file_pathname_or_none: &Option<OsString>,
    script_file_pathname_or_none: &Option<OsString>,
    interpreter: &Interpreter,
    description: &String,
    no_header_flag: &bool,
    ranges: &Vec<Range<u32>>,
    force_flag: &bool,
    reverse_flag: &bool,
    reverse_inner_flag: &bool,
) -> Result<(), Box<dyn Error>> {
    let mut lines = read_lines_from_file_or_stdin(file_pathname_or_none)?;
    if lines.is_empty() {
        println!("The specified file (or stdin) contains no content.");
        std::process::exit(1);
    }
    if ranges.is_empty() {
        println!("No specified line ranges. All lines from the specified file will be used.");
        let mut script_file_bufwriter =
            create_script_file_box_writer(script_file_pathname_or_none)?;
        if *no_header_flag == false {
            update_script_file_bufwriter_header(
                &mut script_file_bufwriter,
                interpreter,
                description,
            )?;
        }
        update_script_file_bufwriter_body_by_lines(
            &mut script_file_bufwriter,
            &mut lines,
            reverse_flag,
        )?;
    } else {
        let mut hashmap = create_hashmap_from_ranges(ranges);
        update_hashmap_by_lines(&mut hashmap, lines);
        if hashmap.values().any(|v| v.is_some()) {
            if *force_flag == true || hashmap.values().all(|v| v.is_some()) {
                let mut script_file_bufwriter =
                    create_script_file_box_writer(script_file_pathname_or_none)?;
                if *no_header_flag == false {
                    update_script_file_bufwriter_header(
                        &mut script_file_bufwriter,
                        interpreter,
                        description,
                    )?;
                }
                update_script_file_bufwriter_body_by_hashmap(
                    &mut script_file_bufwriter,
                    hashmap,
                    ranges,
                    reverse_flag,
                    reverse_inner_flag,
                )?;
            } else {
                println!("The specified file (or standard input) doesn't contain one or more lines with the specified numbers.");
                // std::process::exit(1);
            }
        } else {
            println!("The specified file doesn't contain any lines with specified numbers.");
            // std::process::exit(1);
        }
    }
    Ok(())
}

pub fn print_passed_parameters(
    file_pathname_or_none: &Option<OsString>,
    script_file_pathname_or_none: &Option<OsString>,
    interpreter: &Interpreter,
    description: &String,
    no_header_flag: &bool,
    ranges: &Vec<Range<u32>>,
    force_flag: &bool,
    reverse_flag: &bool,
    reverse_inner_flag: &bool,
) -> Result<(), Box<dyn Error>> {
    println!("History file: {:?}", file_pathname_or_none);
    println!("Output file: {:?}", script_file_pathname_or_none);
    println!("Interpreter: {}", interpreter);
    println!("Description: {}", description);
    println!("No header option: {}", no_header_flag);
    println!("Line ranges: {:?}", ranges);
    println!("Force option: {}", force_flag);
    println!("Reverse option: {}", reverse_flag);
    println!("Reverse inner option: {}\n", reverse_inner_flag);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::zip;

    fn get_testing_hashmaps() -> [HashMap<u32, Option<String>>; 8] {
        let testing_hashmaps = [
            HashMap::new(),
            HashMap::from([(1, None)]),
            HashMap::from([(1, None), (2, None)]),
            HashMap::from([(1, None), (2, None), (5, None), (6, None), (7, None)]),
            HashMap::from([
                (1, None),
                (2, None),
                (5, None),
                (6, None),
                (7, None),
                (8, None),
                (9, None),
                (10, None),
                (11, None),
                (12, None),
            ]),
            HashMap::from([(2, None)]),
            HashMap::from([(2, None), (3, None)]),
            HashMap::from([
                (2, None),
                (3, None),
                (6, None),
                (7, None),
                (8, None),
                (9, None),
            ]),
        ];
        testing_hashmaps
    }

    #[test]
    fn create_hashmap_from_ranges_should_create_correctly() {
        let ranges = [
            vec![],
            vec![1..2],
            vec![1..2, 2..3],
            vec![1..3, 5..8],
            vec![1..3, 5..13, 7..9],
            vec![2..1],
            vec![2..1, 3..2],
            vec![3..1, 8..10, 7..5],
        ];
        let expected_values = get_testing_hashmaps();
        for (range, expected_value) in zip(&ranges, expected_values) {
            let actual_value = create_hashmap_from_ranges(range);
            assert_eq!(actual_value, expected_value);
        }
    }

    #[test]
    fn update_hashmap_by_lines_should_update_correctly() {
        let mut hashmaps = get_testing_hashmaps();
        let lines: Vec<String> = vec!["line 1", "line 2", "line 3", "line 4", "line 5", "line 6"]
            .into_iter()
            .map(|line| line.to_string())
            .collect();
        let expected_values: [HashMap<u32, Option<String>>; 8] = [
            HashMap::new(),
            HashMap::from([(1, Some("line 1".to_string()))]),
            HashMap::from([
                (1, Some("line 1".to_string())),
                (2, Some("line 2".to_string())),
            ]),
            HashMap::from([
                (1, Some("line 1".to_string())),
                (2, Some("line 2".to_string())),
                (5, Some("line 5".to_string())),
                (6, Some("line 6".to_string())),
                (7, None),
            ]),
            HashMap::from([
                (1, Some("line 1".to_string())),
                (2, Some("line 2".to_string())),
                (5, Some("line 5".to_string())),
                (6, Some("line 6".to_string())),
                (7, None),
                (8, None),
                (9, None),
                (10, None),
                (11, None),
                (12, None),
            ]),
            HashMap::from([(2, Some("line 2".to_string()))]),
            HashMap::from([
                (2, Some("line 2".to_string())),
                (3, Some("line 3".to_string())),
            ]),
            HashMap::from([
                (2, Some("line 2".to_string())),
                (3, Some("line 3".to_string())),
                (6, Some("line 6".to_string())),
                (7, None),
                (8, None),
                (9, None),
            ]),
        ];
        for (hashmap, expected_value) in zip(&mut hashmaps, &expected_values) {
            update_hashmap_by_lines(hashmap, lines.clone());
            assert_eq!(hashmap, expected_value);
        }
    }
}
