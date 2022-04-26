// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

use std::fs::File;
use std::io::{Error, ErrorKind};

fn main() {
    let f = File::open("file.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("Not found"),
            _ => panic!("Unknown error"),
        },
    };

    let _f2 = File::open("file.txt").unwrap();

    let _f3 = File::open("file.txt").expect("Failed to open file");

    let _f4 = open_file("file.txt");
}

fn open_file(filename: &str) -> Result<File, Error> {
    let f = File::open(filename)?;

    Ok(f)
}
