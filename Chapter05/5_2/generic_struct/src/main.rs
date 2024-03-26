#[derive(Debug)]
struct Data<T> {
    value: T,
}

impl<T> Data<T> {
    fn print_me(&self) {
        println!("Hello its me!");
    }
}

impl Data<i32> {
    fn special_method(&self) {
        println!("Hello its me i32!");
    }
}

fn main() {
    let c = Data { value: 'a' };
    let i = Data { value: 96 };
    let f = Data { value: 96.0 };

    c.print_me();
    i.print_me();
    f.print_me();

    i.special_method();
    // c.special_method();
}
