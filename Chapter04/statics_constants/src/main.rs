const THRESHOLD: i32 = 10;
// static THRESHOLD: i32 = 10; // 0xAA

fn is_above_threshold(num: i32) -> bool {
    num > THRESHOLD
}

fn is_below_threshold(num: i32) -> bool {
    num > THRESHOLD
}

fn main() {
    let value = 100;

    println!("{}", is_above_threshold(value));
    println!("{}", is_below_threshold(value));
}
