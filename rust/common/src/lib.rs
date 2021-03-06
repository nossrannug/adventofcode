use std::fs::File;
use std::io::prelude::*;

pub fn get_file_content(path: String) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
