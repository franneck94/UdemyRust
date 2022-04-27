static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn main() {
    let n = 16;

    println!("This is: {}", LANGUAGE);
    println!("The threshold is: {}", THRESHOLD);
    println!("is big: {}", is_big(n));
}
