// - if the value is an Err, the Err will be returned from the whole function

use std::fs::File;
use std::io::{self, Read};

fn read_file_content(filepath: &str) -> Result<String, io::Error> {
    let file_result = File::open(filepath);

    let mut file = match file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    Ok(buffer)
}

fn read_file_content2(filepath: &str) -> Result<String, io::Error> {
    let mut file = File::open(filepath)?;

    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    Ok(buffer)
}

fn read_file_content3(filepath: &str) -> Result<String, io::Error> {
    let mut buffer = String::new();

    File::open(filepath)?.read_to_string(&mut buffer)?;

    Ok(buffer)
}

fn main() {
    let read_result = read_file_content3("cargo.toml");

    let read_filecontent = match read_result {
        Ok(file_content) => file_content,
        Err(_) => panic!("File is not utf8"),
    };

    println!("{read_filecontent}");
}
