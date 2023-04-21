use std::collections::HashMap;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::ops::Range;
use std::os::unix::fs::OpenOptionsExt;

use crate::cli::Interpreter;

fn create_file_bufreader(file_path: &OsString) -> Result<BufReader<File>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let file_bufreader = BufReader::new(file);
    Ok(file_bufreader)
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

fn update_hashmap_by_file_bufreader(
    hashmap: &mut HashMap<u32, Option<String>>,
    file_bufreader: BufReader<File>,
) {
    for (index, line) in file_bufreader.lines().enumerate() {
        let line_number = (index + 1) as u32;
        if hashmap.contains_key(&line_number) {
            hashmap.insert(line_number, Some(line.unwrap()));
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

fn update_script_file_bufwriter_body_by_file_bufreader(
    script_file_bufwriter: &mut BufWriter<File>,
    file_bufreader: BufReader<File>,
    reverse_flag: &bool,
) -> Result<(), Box<dyn Error>> {
    let mut lines: Vec<_> = file_bufreader.lines().map(|line| line.unwrap()).collect();
    if *reverse_flag == true {
        lines.reverse();
    }
    for line in lines {
        let body_line = line + "\n";
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
    file_path: &OsString,
    output_file_pathname: &OsString,
    interpreter: &Interpreter,
    description: &String,
    range_vector: &Vec<Range<u32>>,
    force_flag: &bool,
    reverse_flag: &bool,
    reverse_inner_flag: &bool,
) -> Result<(), Box<dyn Error>> {
    let history_file_bufreader = create_file_bufreader(file_path)?;
    if range_vector.is_empty() {
        println!("No specified lines. All lines from the given file will be used.");
        let mut script_file_bufwriter = create_script_file_bufwriter(output_file_pathname)?;
        update_script_file_bufwriter_header(&mut script_file_bufwriter, interpreter, description)?;
        update_script_file_bufwriter_body_by_file_bufreader(&mut script_file_bufwriter, history_file_bufreader, reverse_flag)?;
        println!("{:?}", output_file_pathname);
    } else {
        let mut lines_hashmap = create_hashmap_from_range_vector(range_vector);
        update_hashmap_by_file_bufreader(&mut lines_hashmap, history_file_bufreader);
        if lines_hashmap.values().any(|v| v.is_some()) {
            if *force_flag == true || lines_hashmap.values().all(|v| v.is_some()) {
                let mut script_file_bufwriter = create_script_file_bufwriter(output_file_pathname)?;
                update_script_file_bufwriter_header(&mut script_file_bufwriter, interpreter, description)?;
                update_script_file_bufwriter_body_by_hashmap(&mut script_file_bufwriter, lines_hashmap, range_vector, reverse_flag, reverse_inner_flag)?;
                println!("{:?}", output_file_pathname);
            } else {
                println!("The specified history file doesn't contain a command with the given number.");
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
    file_path: &OsString,
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
