use std::error::Error;
use std::fs::File;
use std::io::BufReader;

#[allow(dead_code)]
fn create_file_bufreader(file_path: &str) -> Result<BufReader<File>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let file_bufreader = BufReader::new(file);
    Ok(file_bufreader)
}
