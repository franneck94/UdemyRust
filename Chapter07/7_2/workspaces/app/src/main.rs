use lib1::add;
use lib2::sub;

fn main() {
    println!("Hello, world!");

    println!("{}", sub(add(2, 2), 4));
}
