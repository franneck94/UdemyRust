// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

use std::fs::File;
use std::io::{Error, ErrorKind};

fn main() {
    let f1 = File::open("file.txt");
    let f2 = File::open("Cargo.toml");

    let f = match f2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("File not found!"),
            _ => panic!("Unknown error!"),
        },
    };
}
