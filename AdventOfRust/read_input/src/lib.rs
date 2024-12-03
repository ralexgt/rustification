use std::fs;
use std::error::Error;

pub fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let data_string = fs::read_to_string(file_path)?;
    Ok(data_string)
}
