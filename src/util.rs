use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
fn create_file_bufreader(file_path: &str) -> Result<BufReader<File>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let file_bufreader = BufReader::new(file);
    Ok(file_bufreader)
}

#[allow(dead_code)]
fn find_line_in_file_bufreader(line_number: u32, file_bufreader: BufReader<File>) -> Option<String> {
    let mut found_line: Option<String> = None;
    let mut line_counter: u32 = 1;
    for line in file_bufreader.lines() {
        if line_counter == line_number {
            found_line = Some(line.unwrap());
            break
        }
        line_counter += 1;
    }
    found_line
}
