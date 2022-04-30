use std::fmt::Debug;
use std::fmt::Display;

#[derive(Debug)]
struct Data<T> {
    value: T,
}

fn print_me<T: Debug>(item: &T) {
    println!("My value: {:?}", item);
}

fn main() {
    let char = Data { value: 'a' };
    let int = Data { value: 96 };
    let float = Data { value: 96.0 };

    print_me(&char);
    print_me(&int);
    print_me(&float);
}
