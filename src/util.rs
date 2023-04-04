use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, BufWriter};
use std::os::unix::fs::OpenOptionsExt;

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

#[allow(dead_code)]
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
