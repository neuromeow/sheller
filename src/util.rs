use std::collections::HashMap;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{stdin, BufRead, BufReader, BufWriter, Write};
use std::ops::Range;
use std::os::unix::fs::OpenOptionsExt;

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
            let stdin = stdin();
            stdin.lock().lines().map(|line| line.unwrap()).collect()
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
        for number in range.clone() {
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
        let mut range_elements: Vec<u32> = range.clone().collect();
        if *reverse_inner_flag == true && range.end - range.start > 1 {
            range_elements.reverse();
        }
        for number in range_elements.clone() {
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
        update_script_file_bufwriter_header(&mut script_file_bufwriter, interpreter, description)?;
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
                update_script_file_bufwriter_header(
                    &mut script_file_bufwriter,
                    interpreter,
                    description,
                )?;
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
    ranges: &Vec<Range<u32>>,
    force_flag: &bool,
    reverse_flag: &bool,
    reverse_inner_flag: &bool,
) -> Result<(), Box<dyn Error>> {
    println!("History file: {:?}", file_pathname_or_none);
    println!("Output file: {:?}", script_file_pathname_or_none);
    println!("Interpreter: {}", interpreter);
    println!("Description: {}", description);
    println!("Line ranges: {:?}", ranges);
    println!("Force option: {}", force_flag);
    println!("Reverse option: {}", reverse_flag);
    println!("Reverse inner option: {}\n", reverse_inner_flag);
    Ok(())
}
