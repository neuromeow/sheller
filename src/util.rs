use std::collections::HashMap;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::ops::Range;
use std::os::unix::fs::OpenOptionsExt;

fn create_file_bufreader(file_path: &OsString) -> Result<BufReader<File>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let file_bufreader = BufReader::new(file);
    Ok(file_bufreader)
}

fn create_script_file_bufwriter() -> Result<BufWriter<File>, Box<dyn Error>> {
    let script_file_name = "script_by_sheller.sh";
    let script_file_options = OpenOptions::new()
        .append(true)
        .create_new(true)
        .mode(0o744)
        .open(script_file_name)?;
    let script_file_bufwriter = BufWriter::new(script_file_options);
    Ok(script_file_bufwriter)
}

fn create_hashmap_from_ranges_vector(ranges_vector: &Vec<Range<u32>>) -> HashMap<u32, Option<String>> {
    let mut result = HashMap::new();
    for range in ranges_vector {
        for number in range.clone() {
            result.insert(number, None);
        }
    }
    result
}

fn update_lines_hashmap_by_file_bufreader_content(
    lines_hashmap: &mut HashMap<u32, Option<String>>,
    file_bufreader: BufReader<File>,
) {
    for (line_index, line_content) in file_bufreader.lines().enumerate() {
        let line_number = (line_index + 1) as u32;
        if lines_hashmap.contains_key(&line_number) {
            lines_hashmap.insert(line_number, Some(line_content.unwrap()));
        }
    }
}

fn update_script_file_bufwriter_header(
    file_bufwriter: &mut BufWriter<File>,
) -> Result<(), Box<dyn Error>> {
    let script_header = String::from("#!/bin/bash\n") + "#\n" + "# Script Description\n\n";
    file_bufwriter.write_all(script_header.as_bytes())?;
    Ok(())
}

fn update_script_file_bufwriter_body_by_file_bufreader_content(
    file_bufreader: BufReader<File>,
    file_bufwriter: &mut BufWriter<File>,
) -> Result<(), Box<dyn Error>> {
    for line in file_bufreader.lines() {
        let line_content = line? + "\n";
        file_bufwriter.write_all(line_content.as_bytes())?;
    }
    Ok(())
}

fn update_script_file_bufwriter_body_by_lines_hashmap(
    ranges_vector: &Vec<Range<u32>>,
    lines_hashmap: HashMap<u32, Option<String>>,
    file_bufwriter: &mut BufWriter<File>,
) -> Result<(), Box<dyn Error>> {
    for range in ranges_vector {
        for number in range.clone() {
            let command = lines_hashmap.get(&number).clone().unwrap();
            let command = command.clone().unwrap() + "\n";
            file_bufwriter.write_all(command.as_bytes())?;
        }
    }
    Ok(())
}

pub fn build_script_file_with_multiple_line_ranges(
    line_ranges: &Vec<Range<u32>>,
    history_file_path: &OsString,
) -> Result<(), Box<dyn Error>> {
    let history_file_bufreader = create_file_bufreader(history_file_path)?;
    if line_ranges.is_empty() {
        println!("No specified lines. All lines from the given file will be used.");
        let mut script_file_bufwriter = create_script_file_bufwriter()?;
        update_script_file_bufwriter_header(&mut script_file_bufwriter)?;
        update_script_file_bufwriter_body_by_file_bufreader_content(history_file_bufreader, &mut script_file_bufwriter)?;
    } else {
        let mut lines_hashmap = create_hashmap_from_ranges_vector(line_ranges);
        println!("{:?}", lines_hashmap);
        update_lines_hashmap_by_file_bufreader_content(&mut lines_hashmap, history_file_bufreader);
        println!("{:?}", lines_hashmap);
        match lines_hashmap.values().any(|line_content| line_content.is_none()) {
            true => {
                println!("The specified history file doesn't contain a command with the given number.");
                std::process::exit(1);
            }
            _ => {
                let mut script_file_bufwriter = create_script_file_bufwriter()?;
                update_script_file_bufwriter_header(&mut script_file_bufwriter)?;
                update_script_file_bufwriter_body_by_lines_hashmap(line_ranges, lines_hashmap, &mut script_file_bufwriter)?;
            }
        }
    }
    Ok(())
}

pub fn print_passed_parameters(
    line_ranges: &Vec<Range<u32>>,
    history_file_path: &OsString,
) -> Result<(), Box<dyn Error>> {
    println!("Starting the script build process...");
    println!("The history file you passed: {:?}", history_file_path);
    println!("The line ranges you passed: {:?}", line_ranges);
    Ok(())
}
