// immutable: variable will be created at runtime and is read only
// const: variables value will be computed at compile time (shortens runtime)
const MILLISECONDS_PER_SECOND: i32 = 1000;
const MICROSECONDS_PER_SECOND: i32 = MILLISECONDS_PER_SECOND * 1000;
const NANOSECONDS_PER_SECOND: i32 = MICROSECONDS_PER_SECOND * 1000;

fn main() {
    println!("{MILLISECONDS_PER_SECOND}");
    println!("{MICROSECONDS_PER_SECOND}");
    println!("{NANOSECONDS_PER_SECOND}");
}
