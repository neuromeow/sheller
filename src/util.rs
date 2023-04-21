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
    file_path_or_none: &Option<OsString>,
) -> Result<Vec<String>, Box<dyn Error>> {
    let lines = match file_path_or_none {
        Some(file_path) => {
            let file = File::open(file_path)?;
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

fn create_script_file_bufwriter(
    script_file_pathname: &OsString,
) -> Result<BufWriter<File>, Box<dyn Error>> {
    let script_file_options = OpenOptions::new()
        .append(true)
        .create_new(true)
        .mode(0o744)
        .open(script_file_pathname)?;
    let script_file_bufwriter = BufWriter::new(script_file_options);
    Ok(script_file_bufwriter)
}

fn create_hashmap_from_range_vector(range_vector: &Vec<Range<u32>>) -> HashMap<u32, Option<String>> {
    let mut hashmap_from_range_vector = HashMap::new();
    for range in range_vector {
        for number in range.clone() {
            hashmap_from_range_vector.insert(number, None);
        }
    }
    hashmap_from_range_vector
}

fn update_hashmap_by_lines_vector(
    hashmap: &mut HashMap<u32, Option<String>>,
    lines_vector: Vec<String>,
) {
    for (index, line) in lines_vector.into_iter().enumerate() {
        let line_number = (index + 1) as u32;
        if hashmap.contains_key(&line_number) {
            hashmap.insert(line_number, Some(line));
        }
    }
}

fn update_script_file_bufwriter_header(
    script_file_bufwriter: &mut BufWriter<File>,
    interpreter: &Interpreter,
    description: &String,
) -> Result<(), Box<dyn Error>> {
    let mut header = format!("#!/bin/env {}\n#\n", interpreter);
    let header_description = format!("# {}\n#\n\n", description);
    header.push_str(&header_description);
    script_file_bufwriter.write_all(header.as_bytes())?;
    Ok(())
}

fn update_script_file_bufwriter_body_by_lines_vector(
    script_file_bufwriter: &mut BufWriter<File>,
    lines_vector: &mut Vec<String>,
    reverse_flag: &bool,
) -> Result<(), Box<dyn Error>> {
    if *reverse_flag == true {
        lines_vector.reverse();
    }
    for line in lines_vector {
        let body_line = line.to_owned() + "\n";
        script_file_bufwriter.write_all(body_line.as_bytes())?;
    }
    Ok(())
}

fn update_script_file_bufwriter_body_by_hashmap(
    file_bufwriter: &mut BufWriter<File>,
    hashmap: HashMap<u32, Option<String>>,
    range_vector: &Vec<Range<u32>>,
    reverse_flag: &bool,
    reverse_inner_flag: &bool,
) -> Result<(), Box<dyn Error>> {
    let mut reversed_range_vector_or_original = range_vector.clone();
    if *reverse_flag == true {
        reversed_range_vector_or_original.reverse();
    }
    for range in reversed_range_vector_or_original {
        let mut range_vec: Vec<u32> = range.clone().collect();
        if *reverse_inner_flag == true && range.end - range.start > 1 {
            range_vec.reverse();
        }
        for number in range_vec.clone() {
            match hashmap.get(&number).unwrap() {
                Some(v) => {
                    let body_line = v.clone() + "\n";
                    file_bufwriter.write_all(body_line.as_bytes())?;
                }
                _ => continue,
            }
        }
    }
    Ok(())
}

pub fn build_script_file(
    file_path: &Option<OsString>,
    output_file_pathname: &OsString,
    interpreter: &Interpreter,
    description: &String,
    range_vector: &Vec<Range<u32>>,
    force_flag: &bool,
    reverse_flag: &bool,
    reverse_inner_flag: &bool,
) -> Result<(), Box<dyn Error>> {
    let mut history_file_lines = read_lines_from_file_or_stdin(file_path)?;
    if history_file_lines.is_empty() {
        println!("The specified history file (or stdin) contains no content.");
        std::process::exit(1);
    }
    if range_vector.is_empty() {
        println!("No specified lines. All lines from the given file will be used.");
        let mut script_file_bufwriter = create_script_file_bufwriter(output_file_pathname)?;
        update_script_file_bufwriter_header(&mut script_file_bufwriter, interpreter, description)?;
        update_script_file_bufwriter_body_by_lines_vector(&mut script_file_bufwriter, &mut history_file_lines, reverse_flag)?;
        println!("{:?}", output_file_pathname);
    } else {
        let mut lines_hashmap = create_hashmap_from_range_vector(range_vector);
        update_hashmap_by_lines_vector(&mut lines_hashmap, history_file_lines);
        if lines_hashmap.values().any(|v| v.is_some()) {
            if *force_flag == true || lines_hashmap.values().all(|v| v.is_some()) {
                let mut script_file_bufwriter = create_script_file_bufwriter(output_file_pathname)?;
                update_script_file_bufwriter_header(&mut script_file_bufwriter, interpreter, description)?;
                update_script_file_bufwriter_body_by_hashmap(&mut script_file_bufwriter, lines_hashmap, range_vector, reverse_flag, reverse_inner_flag)?;
                println!("{:?}", output_file_pathname);
            } else {
                println!("The specified history file (or stdin) doesn't contain a command with the given number.");
                std::process::exit(1);
            }
        } else {
            println!("The specified history file doesn't contain any commands with such numbers.");
            std::process::exit(1);
        }
    }
    Ok(())
}

pub fn print_passed_parameters(
    file_path: &Option<OsString>,
    output_file_path: &OsString,
    interpreter: &Interpreter,
    description: &String,
    range_vector: &Vec<Range<u32>>,
    force_flag: &bool,
    reverse_flag: &bool,
    reverse_inner_flag: &bool,
) -> Result<(), Box<dyn Error>> {
    println!("The history file you passed: {:?}", file_path);
    println!("Output file: {:?}", output_file_path);
    println!("Interpreter: {}", interpreter);
    println!("Description: {}", description);
    println!("The line ranges you passed: {:?}", range_vector);
    println!("Force option: {:?}", force_flag);
    println!("Reverse option: {:?}", reverse_flag);
    println!("Reverse inner option: {:?}", reverse_inner_flag);
    Ok(())
}
