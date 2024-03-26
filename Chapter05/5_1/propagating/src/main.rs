use std::fs::File;
use std::io::{self, Read};

fn read_file_content(filepath: &str) -> Result<String, io::Error> {
    let file_result = File::open(filepath);

    let mut file = match file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut buffer = String::new();

    let read_result = match file.read_to_string(&mut buffer) {
        Ok(_) => Ok(buffer),
        Err(err) => Err(err),
    };

    return read_result;
}

fn main() {
    let read_result = read_file_content("cargo.toml");

    let read_filecontent = match read_result {
        Ok(file_content) => file_content,
        Err(_) => panic!("File is not utf8"),
    };

    println!("{read_filecontent}");
}
