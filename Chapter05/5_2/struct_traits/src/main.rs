use std::fmt::Debug;
use std::fmt::Display;

#[derive(Debug)]
struct Data<T> {
    value: T,
}

impl<T> Data<T>
where
    T: Display + Debug,
{
    fn print_me(&self) {
        println!("My value: {}", self.value);
    }
}

fn main() {
    let c = Data { value: 'a' };
    let i = Data { value: 96 };
    let f = Data { value: 96.0 };

    c.print_me();
    i.print_me();
    f.print_me();
}
