// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

use std::fs::File;
use std::io::{Error, ErrorKind, Read};

fn main() {
    let _f1 = File::open("file.txt");
    let f2 = File::open("cargo.toml");

    let mut f = match f2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("File not found!"),
            _ => panic!("Unknown Error"),
        },
    };

    let mut file_content = String::new();
    let read_result = f.read_to_string(&mut file_content);

    match read_result {
        Ok(_) => {}
        Err(_) => panic!("File is not utf8"),
    }

    println!("{file_content}");
}
