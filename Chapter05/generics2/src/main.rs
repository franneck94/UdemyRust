use std::fmt::Debug;
use std::fmt::Display;

#[derive(Debug)]
struct Data<T> {
    value: T,
}

impl<T> Data<T>
where
    T: Debug + Display,
{
    fn print_me(&self) {
        println!("My value: {}", self.value);
    }
}

fn main() {
    let char = Data { value: 'a' };
    let int = Data { value: 96 };
    let float = Data { value: 96.0 };

    char.print_me();
    int.print_me();
    float.print_me();
}
