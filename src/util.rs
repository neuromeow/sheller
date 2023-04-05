use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::os::unix::fs::OpenOptionsExt;

fn create_file_bufreader(file_path: &OsString) -> Result<BufReader<File>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let file_bufreader = BufReader::new(file);
    Ok(file_bufreader)
}

fn find_line_in_file_bufreader(
    line_number: u32,
    file_bufreader: BufReader<File>,
) -> Option<String> {
    let mut found_line: Option<String> = None;
    let mut line_counter: u32 = 1;
    for line in file_bufreader.lines() {
        if line_counter == line_number {
            found_line = Some(line.unwrap());
            break;
        }
        line_counter += 1;
    }
    found_line
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

fn update_script_file_bufreader(
    body: String,
    file_bufwriter: &mut BufWriter<File>,
) -> Result<(), Box<dyn Error>> {
    let script_header = String::from("#!/bin/bash\n") + "#\n" + "# Script Description\n\n";
    let script_content = script_header + &body + "\n";
    file_bufwriter.write_all(script_content.as_bytes())?;
    Ok(())
}

pub fn build_script_file(
    line_number: u32,
    history_file_path: &OsString,
) -> Result<(), Box<dyn Error>> {
    let history_file_bufreader = create_file_bufreader(history_file_path)?;
    println!("Starting the script build process...");
    println!("The history file you passed: {:?}", history_file_path);
    println!("The line number you passed: {}", line_number);
    match find_line_in_file_bufreader(line_number, history_file_bufreader) {
        Some(command) => {
            let mut script_file_bufwriter = create_script_file_bufwriter()?;
            update_script_file_bufreader(command, &mut script_file_bufwriter)?;
            println!("Your script has been created!");
        }
        _ => {
            println!("The specified history file doesn't contain a command with the given number.");
            std::process::exit(1);
        }
    }
    Ok(())
}
